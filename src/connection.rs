use nix::sys::socket::{
    accept, bind, listen, recv, recvmsg, send, socket, AddressFamily, MsgFlags, RecvMsg, SockFlag,
    SockType, SockaddrStorage, UnixAddr,
};
use prost::Message;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use std::{
    os::{fd::AsRawFd, unix::io::RawFd},
    process::{Command, Stdio},
    thread::sleep_ms,
};

use crate::items::NewActivityResponse;

#[derive(Debug)]
pub struct Tgui {
    pub main: RawFd,
    pub event: RawFd,
}

#[derive(Debug)]
pub enum ProtocolType {
    // protocol type:
    //   0: protobuf
    //   1: json
    //
    // REFS: https://github.com/termux/termux-gui/blob/main/Protocol.md#protocol-negotiation
    //     version_type
    Proto = 0b0000_0000,
    Json = 0b0000_0001,
}

impl Tgui {
    pub fn new(ty: ProtocolType) -> Self {
        // gen random string
        let main_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);
        let event_addr = Alphanumeric.sample_string(&mut thread_rng(), 50);

        let main_sock_addr = UnixAddr::new_abstract(main_addr.as_bytes()).unwrap();
        let event_sock_addr = UnixAddr::new_abstract(event_addr.as_bytes()).unwrap();

        let main_sock = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .unwrap();
        let event_sock = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .unwrap();

        sleep_ms(10);
        bind(main_sock.as_raw_fd(), &main_sock_addr).unwrap();

        sleep_ms(10);
        bind(event_sock.as_raw_fd(), &event_sock_addr).unwrap();

        listen(&main_sock, 1).unwrap();
        listen(&event_sock, 1).unwrap();

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
            .output()
            .unwrap();

        let main = accept(main_sock.as_raw_fd()).unwrap();
        let event = accept(event_sock.as_raw_fd()).unwrap();

        let mut protocol = [ty as u8];
        while send(main, &protocol, MsgFlags::empty()).unwrap() == 0 {}

        protocol = [0u8];
        while recv(main, &mut protocol, MsgFlags::empty()).unwrap() == 0 {}

        Self { main, event }
    }

    pub fn send_msg(&self, msg: &[u8]) {
        send_all(self.main, msg);
    }

    pub fn recv_msg<T: Message + Default>(&self) -> T {
        todo!()
    }
}

fn send_all(fd: RawFd, msg: &[u8]) {
    let mut start = 0;
    while start < msg.len() {
        let ret = send(fd, &msg[start..], MsgFlags::empty()).unwrap();
        start += ret;
    }
}
