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
    io::Cursor,
    os::{fd::AsRawFd, unix::io::RawFd},
    process::{Command, Stdio},
    thread::sleep_ms,
};

use crate::{items::NewActivityResponse, *};

#[derive(Debug)]
pub struct Tgui {
    pub main: RawFd,
    pub event: RawFd,
    pub pty: ProtocolType,

    activity: Activity,
}

// REFS: https://github.com/termux/termux-gui/blob/main/Protocol.md#protocol-negotiation
#[derive(Debug, Clone, Copy)]
pub enum ProtocolType {
    // protocol type:
    //   0: protobuf
    //   1: json
    //
    //     version_type
    Proto = 0b0000_0000,
    Json = 0b0000_0001,
}

impl Tgui {
    pub fn new() -> Self {
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

        Self {
            main,
            event,
            pty: ProtocolType::Proto,
            activity: Default::default(),
        }
    }

    pub fn with_json(mut self) -> Self {
        self.pty = ProtocolType::Json;

        self
    }

    pub fn conn(self) -> Self {
        let mut protocol = [self.pty as u8];
        while send(self.fd_main(), &protocol, MsgFlags::empty()).unwrap() == 0 {}

        protocol = [0u8];
        while recv(self.fd_main(), &mut protocol, MsgFlags::empty()).unwrap() == 0 {}

        self
    }

    pub fn send_msg(&self, msg: &[u8]) {
        send_all(self.fd_main(), msg);
    }

    pub fn recv_msg<T: Message + Default>(&self) -> Result<T, ()> {
        let size = recv_size(self.fd_main()).unwrap();
        let mut msg = vec![0_u8; size];
        let mut start = 0;
        while start < msg.len() {
            let ret = recv(self.fd_main(), &mut msg[start..], MsgFlags::empty()).unwrap();
            start += ret;
        }

        Ok(Message::decode(msg.as_slice()).unwrap())
    }

    pub fn fd_main(&self) -> RawFd {
        self.main
    }

    pub fn fd_event(&self) -> RawFd {
        self.event
    }

    pub fn activity(&self) -> &Activity {
        &self.activity
    }
}

fn send_all(fd: RawFd, msg: &[u8]) {
    let mut start = 0;
    while start < msg.len() {
        let ret = send(fd, &msg[start..], MsgFlags::empty()).unwrap();
        start += ret;
    }
}

// REFS: https://docs.rs/delimited-protobuf/latest/src/delimited_protobuf/lib.rs.html#5-14
fn recv_size(fd: RawFd) -> Option<usize> {
    let mut buf = [0_u8; 1];
    let mut num_bits_read = 0;
    let mut val: u32 = 0;
    let mut is_last: bool = false;
    let mut byte: u32;

    while !is_last {
        'l2: loop {
            if recv(fd, &mut buf, MsgFlags::empty()).is_ok() {
                break 'l2;
            }
        }

        byte = buf[0] as u32;
        is_last = byte >> 7 == 0;
        byte &= 0b0111_1111;

        byte = byte
            .checked_shl(num_bits_read)
            .expect("too many bytes for u32");
        val |= byte;
        num_bits_read += 7;
    }

    if val == 0 {
        None
    } else {
        Some(val as usize)
    }
}
