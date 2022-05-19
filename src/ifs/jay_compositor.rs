use {
    crate::{
        cli::CliLogLevel,
        client::{Client, ClientError},
        globals::{Global, GlobalName},
        ifs::{jay_idle::JayIdle, jay_log_file::JayLogFile, jay_screenshot::JayScreenshot},
        leaks::Tracker,
        object::Object,
        screenshoter::take_screenshot,
        utils::{
            buffd::{MsgParser, MsgParserError},
            errorfmt::ErrorFmt,
        },
        wire::{jay_compositor::*, JayCompositorId},
    },
    bstr::ByteSlice,
    log::Level,
    std::{ops::Deref, rc::Rc},
    thiserror::Error,
};

pub struct JayCompositorGlobal {
    name: GlobalName,
}

impl JayCompositorGlobal {
    pub fn new(name: GlobalName) -> Self {
        Self { name }
    }

    fn bind_(
        self: Rc<Self>,
        id: JayCompositorId,
        client: &Rc<Client>,
        _version: u32,
    ) -> Result<(), JayCompositorError> {
        let obj = Rc::new(JayCompositor {
            id,
            client: client.clone(),
            tracker: Default::default(),
        });
        track!(client, obj);
        client.add_client_obj(&obj)?;
        Ok(())
    }
}

global_base!(JayCompositorGlobal, JayCompositor, JayCompositorError);

impl Global for JayCompositorGlobal {
    fn singleton(&self) -> bool {
        true
    }

    fn version(&self) -> u32 {
        1
    }

    fn secure(&self) -> bool {
        true
    }
}

simple_add_global!(JayCompositorGlobal);

pub struct JayCompositor {
    id: JayCompositorId,
    client: Rc<Client>,
    tracker: Tracker<Self>,
}

impl JayCompositor {
    fn destroy(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let _req: Destroy = self.client.parse(self, parser)?;
        self.client.remove_obj(self)?;
        Ok(())
    }

    fn get_log_file(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let req: GetLogFile = self.client.parse(self, parser)?;
        let log_file = Rc::new(JayLogFile::new(req.id, &self.client));
        track!(self.client, log_file);
        self.client.add_client_obj(&log_file)?;
        let path = match &self.client.state.logger {
            Some(logger) => logger.path(),
            _ => "".as_bytes().as_bstr(),
        };
        log_file.send_path(path);
        Ok(())
    }

    fn quit(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let _req: Quit = self.client.parse(self, parser)?;
        log::info!("Quitting");
        self.client.state.ring.stop();
        Ok(())
    }

    fn set_log_level(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let req: SetLogLevel = self.client.parse(self, parser)?;
        const ERROR: u32 = CliLogLevel::Error as u32;
        const WARN: u32 = CliLogLevel::Warn as u32;
        const INFO: u32 = CliLogLevel::Info as u32;
        const DEBUG: u32 = CliLogLevel::Debug as u32;
        const TRACE: u32 = CliLogLevel::Trace as u32;
        let level = match req.level {
            ERROR => Level::Error,
            WARN => Level::Warn,
            INFO => Level::Info,
            DEBUG => Level::Debug,
            TRACE => Level::Trace,
            _ => return Err(JayCompositorError::UnknownLogLevel(req.level)),
        };
        if let Some(logger) = &self.client.state.logger {
            logger.set_level(level);
        }
        Ok(())
    }

    fn take_screenshot(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let req: TakeScreenshot = self.client.parse(self, parser)?;
        let ss = Rc::new(JayScreenshot {
            id: req.id,
            client: self.client.clone(),
            tracker: Default::default(),
        });
        track!(self.client, ss);
        self.client.add_client_obj(&ss)?;
        match take_screenshot(&self.client.state) {
            Ok(s) => {
                let dmabuf = s.bo.dmabuf();
                let plane = &dmabuf.planes[0];
                ss.send_dmabuf(
                    &s.drm,
                    &plane.fd,
                    dmabuf.width,
                    dmabuf.height,
                    plane.offset,
                    plane.stride,
                );
            }
            Err(e) => {
                let msg = ErrorFmt(e).to_string();
                ss.send_error(&msg);
            }
        }
        self.client.remove_obj(ss.deref())?;
        Ok(())
    }

    fn get_idle(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let req: GetIdle = self.client.parse(self, parser)?;
        let idle = Rc::new(JayIdle {
            id: req.id,
            client: self.client.clone(),
            tracker: Default::default(),
        });
        track!(self.client, idle);
        self.client.add_client_obj(&idle)?;
        Ok(())
    }

    fn get_client_id(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let _req: GetClientId = self.client.parse(self, parser)?;
        self.client.event(ClientId {
            self_id: self.id,
            client_id: self.client.id.raw(),
        });
        Ok(())
    }

    fn enable_symmetric_delete(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let _req: EnableSymmetricDelete = self.client.parse(self, parser)?;
        self.client.symmetric_delete.set(true);
        Ok(())
    }

    fn unlock(&self, parser: MsgParser<'_, '_>) -> Result<(), JayCompositorError> {
        let _req: Unlock = self.client.parse(self, parser)?;
        let state = &self.client.state;
        if state.lock.locked.replace(false) {
            if let Some(lock) = state.lock.lock.take() {
                lock.finish();
            }
            for output in state.outputs.lock().values() {
                if let Some(surface) = output.node.lock_surface.take() {
                    surface.destroy_node();
                }
            }
            state.tree_changed();
            state.damage();
        }
        self.client.symmetric_delete.set(true);
        Ok(())
    }
}

object_base! {
    JayCompositor;

    DESTROY => destroy,
    GET_LOG_FILE => get_log_file,
    QUIT => quit,
    SET_LOG_LEVEL => set_log_level,
    TAKE_SCREENSHOT => take_screenshot,
    GET_IDLE => get_idle,
    GET_CLIENT_ID => get_client_id,
    ENABLE_SYMMETRIC_DELETE => enable_symmetric_delete,
    UNLOCK => unlock,
}

impl Object for JayCompositor {
    fn num_requests(&self) -> u32 {
        UNLOCK + 1
    }
}

simple_add_obj!(JayCompositor);

#[derive(Debug, Error)]
pub enum JayCompositorError {
    #[error("Parsing failed")]
    MsgParserError(#[source] Box<MsgParserError>),
    #[error(transparent)]
    ClientError(Box<ClientError>),
    #[error("Unknown log level {0}")]
    UnknownLogLevel(u32),
}
efrom!(JayCompositorError, ClientError);
efrom!(JayCompositorError, MsgParserError);
