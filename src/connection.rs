use nix::{
    sys::socket::{
        accept, bind, listen, recv, send, socket, AddressFamily, MsgFlags,
        SockFlag, SockType, UnixAddr,
    },
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

use crate::{res::MyErr, *};

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
    pub fn new() -> Res<Self> {
        // gen random string
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
            activity: Default::default(),
        })
    }

    pub fn with_json(mut self) -> Self {
        self.pty = ProtocolType::Json;

        self
    }

    pub fn conn(self) -> Res<Self> {
        let mut protocol = [self.pty as u8];
        while send(self.fd_main(), &protocol, MsgFlags::empty())? == 0 {}

        protocol = [0u8];
        while recv(self.fd_main(), &mut protocol, MsgFlags::empty())? == 0 {}

        Ok(self)
    }

    pub fn send_msg(&self, msg: &[u8]) -> Res<()> {
        send_all(self.fd_main(), msg)?;

        Ok(())
    }

    pub fn recv_msg<T: Message + Default>(&self) -> Res<T> {
        let size = recv_size(self.fd_main())?;
        let mut msg = vec![0_u8; size];
        let mut start = 0;
        while start < msg.len() {
            let ret = recv(self.fd_main(), &mut msg[start..], MsgFlags::empty())?;
            start += ret;
        }

        Ok(Message::decode(msg.as_slice())?)
    }

    pub fn recv_msg_fd(&self) -> Res<()> {
        // TODO:
        Ok(())
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

fn send_all(fd: RawFd, msg: &[u8]) -> Res<()> {
    let mut start = 0;
    while start < msg.len() {
        let ret = send(fd, &msg[start..], MsgFlags::empty())?;
        start += ret;
    }

    Ok(())
}

// REFS: https://docs.rs/delimited-protobuf/latest/src/delimited_protobuf/lib.rs.html#5-14
fn recv_size(fd: RawFd) -> Res<usize> {
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
        Err(MyErr::ProtoZeroLen)
    } else {
        Ok(val as usize)
    }
}
