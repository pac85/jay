mod handler;

use crate::backend::{KeyboardId, MouseId};
use crate::config::handler::ConfigProxyHandler;
use crate::ifs::wl_seat::SeatId;
use crate::utils::ptr_ext::PtrExt;
use crate::{NumCell, State};
use i4config::_private::ipc::{InitMessage, ServerMessage, V1InitMessage};
use i4config::_private::{bincode_ops, ConfigEntry, VERSION};
use i4config::keyboard::ModifiedKeySym;
use i4config::{InputDevice, Keyboard, Mouse, Seat};
use libloading::Library;
use std::cell::Cell;
use std::ptr;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Could not load the config library")]
    CouldNotLoadLibrary(#[source] libloading::Error),
    #[error("Config library does not contain the entry symbol")]
    LibraryDoesNotContainEntry(#[source] libloading::Error),
}

pub struct ConfigProxy {
    handler: Rc<ConfigProxyHandler>,
}

impl ConfigProxy {
    pub fn invoke_shortcut(&self, seat: SeatId, modsym: &ModifiedKeySym) {
        self.handler.send(&ServerMessage::InvokeShortcut {
            seat: Seat(seat.raw() as _),
            mods: modsym.mods,
            sym: modsym.sym,
        });
    }

    pub fn new_keyboard(&self, kb: KeyboardId) {
        self.handler.send(&ServerMessage::NewInputDevice {
            device: InputDevice::Keyboard(Keyboard(kb.raw() as _)),
        });
    }

    pub fn new_mouse(&self, mouse: MouseId) {
        self.handler.send(&ServerMessage::NewInputDevice {
            device: InputDevice::Mouse(Mouse(mouse.raw() as _)),
        });
    }

    pub fn del_keyboard(&self, kb: KeyboardId) {
        self.handler.send(&ServerMessage::DelInputDevice {
            device: InputDevice::Keyboard(Keyboard(kb.raw() as _)),
        });
    }

    pub fn del_mouse(&self, mouse: MouseId) {
        self.handler.send(&ServerMessage::DelInputDevice {
            device: InputDevice::Mouse(Mouse(mouse.raw() as _)),
        });
    }
}

impl Drop for ConfigProxy {
    fn drop(&mut self) {
        unsafe {
            self.handler.dropped.set(true);
            (self.handler.unref)(self.handler.client_data.get());
        }
    }
}

unsafe extern "C" fn default_client_init(
    srv_data: *const u8,
    srv_unref: unsafe extern "C" fn(data: *const u8),
    srv_handler: unsafe extern "C" fn(data: *const u8, msg: *const u8, size: usize),
    msg: *const u8,
    size: usize,
) -> *const u8 {
    extern "C" fn configure() {
        default_config::configure();
    }
    i4config::_private::client::init(srv_data, srv_unref, srv_handler, msg, size, configure)
}

impl ConfigProxy {
    fn new(lib: Option<Library>, entry: &ConfigEntry, state: &Rc<State>) -> Self {
        let version = entry.version.min(VERSION);
        let data = Rc::new(ConfigProxyHandler {
            client_data: Cell::new(ptr::null()),
            dropped: Cell::new(false),
            _lib: lib,
            _version: version,
            unref: entry.unref,
            handle_msg: entry.handle_msg,
            state: state.clone(),
            next_id: NumCell::new(1),
            keymaps: Default::default(),
            bufs: Default::default(),
        });
        let init_msg =
            bincode::encode_to_vec(&InitMessage::V1(V1InitMessage {}), bincode_ops()).unwrap();
        unsafe {
            let client_data = (entry.init)(
                Rc::into_raw(data.clone()) as _,
                unref,
                handle_msg,
                init_msg.as_ptr(),
                init_msg.len(),
            );
            data.client_data.set(client_data);
        }
        data.send(&ServerMessage::Configure);
        Self { handler: data }
    }

    pub fn default(state: &Rc<State>) -> Self {
        let entry = ConfigEntry {
            version: VERSION,
            init: default_client_init,
            unref: i4config::_private::client::unref,
            handle_msg: i4config::_private::client::handle_msg,
        };
        Self::new(None, &entry, state)
    }

    #[allow(dead_code)]
    pub unsafe fn from_file(path: &str, state: &Rc<State>) -> Result<Self, ConfigError> {
        let lib = match Library::new(path) {
            Ok(l) => l,
            Err(e) => return Err(ConfigError::CouldNotLoadLibrary(e)),
        };
        let entry = lib.get::<&'static ConfigEntry>(b"I4_CONFIG_ENTRY\0");
        let entry = match entry {
            Ok(e) => *e,
            Err(e) => return Err(ConfigError::LibraryDoesNotContainEntry(e)),
        };
        Ok(Self::new(Some(lib), entry, state))
    }
}

unsafe extern "C" fn unref(data: *const u8) {
    let server = data as *const ConfigProxyHandler;
    drop(Rc::from_raw(server));
}

unsafe extern "C" fn handle_msg(data: *const u8, msg: *const u8, size: usize) {
    let server = (data as *const ConfigProxyHandler).deref();
    if server.dropped.get() {
        return;
    }
    let msg = std::slice::from_raw_parts(msg, size);
    server.handle_request(msg);
}