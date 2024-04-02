use crate::*;

use nix::sys::socket::{
    accept, bind, listen, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, UnixAddr,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use std::{
    os::{fd::AsRawFd, unix::io::RawFd},
    process::{Command, Stdio},
    thread::sleep_ms,
};

#[derive(Debug, Clone, Default)]
pub struct Task {
    pub main: RawFd,
    pub event: RawFd,
    pub pty: ProtocolType,
    tid: i32,
}

// REFS: https://github.com/termux/termux-gui/blob/main/Protocol.md#protocol-negotiation
#[derive(Debug, Clone, Copy, Default)]
pub enum ProtocolType {
    // protocol type:
    //   0: protobuf
    //   1: json
    //
    //     version_type
    #[default]
    Proto = 0b0000_0000,
    Json = 0b0000_0001,
}

impl Task {
    pub fn new_activity(&self, tid: i32) -> Res<Activity> {
        let mut res = Activity {
            req: items::NewActivityRequest {
                tid,
                r#type: ActivityType::Normal.into(),
                intercept_back_button: false,
            },
            res: None,
            task: self.clone(),
        };

        res.res = Some(res.sr(items::method::Method::NewActivity(res.req.clone()))?);
        res.task.tid = res.res.as_ref().unwrap().tid;

        Ok(res)
    }
}

impl Task {
    pub fn new() -> Res<Self> {
        // random string
        let main_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);
        let event_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);

        let main_sock_addr = UnixAddr::new_abstract(main_addr.as_bytes())?;
        let event_sock_addr = UnixAddr::new_abstract(event_addr.as_bytes())?;

        let main_sock = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )?;
        let event_sock = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )?;

        sleep_ms(10);
        bind(main_sock.as_raw_fd(), &main_sock_addr)?;

        sleep_ms(10);
        bind(event_sock.as_raw_fd(), &event_sock_addr)?;

        listen(&main_sock, 1)?;
        listen(&event_sock, 1)?;

        Command::new("am")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args([
                "broadcast",
                "-n",
                "com.termux.gui/.GUIReceiver",
                "--es",
                "mainSocket",
                &main_addr,
                "--es",
                "eventSocket",
                &event_addr,
            ])
            .output()?;

        let main = accept(main_sock.as_raw_fd())?;
        let event = accept(event_sock.as_raw_fd())?;

        Ok(Self {
            main,
            event,
            pty: ProtocolType::Proto,
            tid: -1,
        })
    }

    pub fn conn(self) -> Res<Self> {
        let mut protocol = [self.pty as u8];
        while send(self.main, &protocol, MsgFlags::empty())? == 0 {}

        protocol = [0u8];
        while recv(self.main, &mut protocol, MsgFlags::empty())? == 0 {}

        Ok(self)
    }
}

impl From<items::FinishTaskRequest> for items::method::Method {
    fn from(value: items::FinishTaskRequest) -> Self {
        Self::FinishTask(value)
    }
}
