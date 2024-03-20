pub use items::new_activity_request::ActivityType;

use crate::{
    items::{NewActivityRequest, NewActivityResponse},
    View, *,
};

use nix::sys::socket::{recv, send, MsgFlags};
use prost::Message;
use std::os::unix::io::RawFd;

#[derive(Debug, Clone, Default)]
pub struct Activity {
    pub(crate) req: NewActivityRequest,
    pub(crate) res: Option<NewActivityResponse>,
    pub(crate) task: Task,
}

impl Activity {
    pub fn set_tid(mut self, tid: i32) -> Self {
        self.req.tid = tid;

        self
    }

    pub fn set_ty(mut self, ty: ActivityType) -> Self {
        self.req.r#type = ty.into();

        self
    }

    pub fn set_intercept_back_button(mut self, flag: bool) -> Self {
        self.req.intercept_back_button = flag;

        self
    }

    pub fn gen_create(&self) -> Option<items::Create> {
        let Ok(aid) = self.id() else {
            return None;
        };

        Some(items::Create::default().set_aid(aid))
    }

    pub fn gen_view(&self, other: &impl View) -> Option<items::View> {
        let (Ok(aid), Ok(id)) = (self.aid(), other.id()) else {
            return None;
        };

        Some(items::View { aid, id })
    }

    pub fn close(&self) {
        self.sr::<items::FinishTaskResponse>(items::method::Method::FinishTask(
            items::FinishTaskRequest {
                tid: self.tid().unwrap(),
            },
        ))
        .unwrap();
    }
}

impl Activity {
    // send and recv Method msg
    pub fn sr<T: Message + Default>(&self, msg: items::method::Method) -> Res<T> {
        let msg = items::Method { method: Some(msg) }.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    // send and recv msg
    pub fn sr_msg<T: Message + Default>(&self, msg: &[u8]) -> Res<T> {
        self.send_msg(msg)?;

        self.recv_msg()
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
        todo!()
    }

    pub fn sr_event(&self, msg: items::method::Method) -> Res<items::Event> {
        let msg = items::Method { method: Some(msg) }.encode_length_delimited_to_vec();
        self.send_event(msg.as_slice())?;

        self.recv_event()
    }

    pub fn send_event(&self, msg: &[u8]) -> Res<()> {
        send_all(self.fd_event(), msg)?;

        Ok(())
    }

    pub fn recv_event(&self) -> Res<items::Event> {
        let size = recv_size(self.fd_event())?;
        let mut msg = vec![0_u8; size];
        let mut start = 0;
        while start < msg.len() {
            let ret = recv(self.fd_event(), &mut msg[start..], MsgFlags::empty())?;
            start += ret;
        }

        Ok(Message::decode(msg.as_slice())?)
    }

    pub fn fd_main(&self) -> RawFd {
        self.task.main
    }

    pub fn fd_event(&self) -> RawFd {
        self.task.event
    }
}

impl View for Activity {
    fn id(&self) -> Res<i32> {
        self.aid()
    }

    fn aid(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().aid)
    }

    fn tid(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().tid)
    }

    fn act(&self) -> &Activity {
        &self
    }
}

impl ViewSet for Activity {
    fn set_tid(mut self, tid: i32) -> Self {
        self.req.tid = tid;

        self
    }
}

impl From<NewActivityRequest> for items::method::Method {
    fn from(value: items::NewActivityRequest) -> Self {
        Self::NewActivity(value)
    }
}

#[inline]
pub fn send_all(fd: RawFd, msg: &[u8]) -> Res<()> {
    let mut start = 0;
    while start < msg.len() {
        let ret = send(fd, &msg[start..], MsgFlags::empty())?;
        start += ret;
    }

    Ok(())
}

// REFS: https://docs.rs/delimited-protobuf/latest/src/delimited_protobuf/lib.rs.html#5-14
#[inline]
pub fn recv_size(fd: RawFd) -> Res<usize> {
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
