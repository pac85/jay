use crate::event_loop::{EventLoopDispatcher, EventLoopId, EventLoopRef};
use crate::EventLoopError;
use std::error::Error;
use std::rc::Rc;
use thiserror::Error;
use uapi::{c, Errno, OwnedFd};

#[derive(Debug, Error)]
pub enum SighandError {
    #[error("The signal fd is in an error state")]
    ErrorEvent,
    #[error("Could not read from the signal fd")]
    ReadFailed(#[source] std::io::Error),
    #[error("Could not block the signalfd signals")]
    BlockFailed(#[source] std::io::Error),
    #[error("Could not create a signalfd")]
    CreateFailed(#[source] std::io::Error),
    #[error("The event loop caused an error")]
    EventLoopError(#[from] EventLoopError),
}

pub fn install(el: &EventLoopRef) -> Result<(), SighandError> {
    let mut set: c::sigset_t = uapi::pod_zeroed();
    uapi::sigaddset(&mut set, c::SIGINT).unwrap();
    if let Err(e) = uapi::pthread_sigmask(c::SIG_BLOCK, Some(&set), None) {
        return Err(SighandError::BlockFailed(e.into()));
    }
    let fd = match uapi::signalfd_new(&set, c::SFD_CLOEXEC | c::SFD_NONBLOCK) {
        Ok(fd) => fd,
        Err(e) => return Err(SighandError::CreateFailed(e.into())),
    };
    let id = el.id()?;
    let sh = Rc::new(Sighand {
        fd,
        id,
        el: el.clone(),
    });
    el.insert(id, Some(sh.fd.raw()), c::EPOLLIN, sh)?;
    Ok(())
}

struct Sighand {
    fd: OwnedFd,
    id: EventLoopId,
    el: EventLoopRef,
}

impl EventLoopDispatcher for Sighand {
    fn dispatch(&self, events: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        if events & (c::EPOLLERR | c::EPOLLHUP) != 0 {
            return Err(Box::new(SighandError::ErrorEvent));
        }
        let mut sigfd: c::signalfd_siginfo = uapi::pod_zeroed();
        loop {
            if let Err(e) = uapi::read(self.fd.raw(), &mut sigfd) {
                match e {
                    Errno(c::EAGAIN) => break,
                    _ => return Err(Box::new(SighandError::ReadFailed(e.into()))),
                }
            }
            log::info!("Received signal {}", sigfd.ssi_signo);
            if sigfd.ssi_signo == c::SIGINT as _ {
                log::info!("Exiting");
                self.el.stop();
            }
        }
        Ok(())
    }
}

impl Drop for Sighand {
    fn drop(&mut self) {
        let _ = self.el.remove(self.id);
    }
}