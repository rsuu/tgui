use nix::{
    cmsg_space,
    libc::{self, MAP_SHARED, PROT_READ, PROT_WRITE},
    sys::socket::{recv, recvmsg, ControlMessageOwned, MsgFlags, RecvMsg, SockaddrStorage},
};
use prost::Message;
use std::{
    os::fd::{IntoRawFd, RawFd},
    ptr,
};

use crate::{
    items::{
        method, AddBufferRequest, BlitBufferRequest, BlitBufferResponse, Method, SetBufferRequest,
        SetBufferResponse,
    },
    view::ImgRes,
    ImgTy, *,
};

#[derive(Debug)]
pub struct Buffer {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct BufferRes {
    pub fd: RawFd,
    pub bid: usize,

    len: usize,
    ptr: *mut u8,
}

impl Buffer {
    pub fn new(imgty: &ImgTy) -> Res<Self> {
        let (width, height) = imgty.get_wh()?;

        Ok(Self { width, height })
    }

    pub fn size(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

impl BufferRes {
    pub unsafe fn mmap(&mut self) -> Res<()> {
        // REFS: https://www.ibm.com/docs/en/zos/2.4.0?topic=functions-mmap-map-pages-memory
        let addr = libc::mmap(
            ptr::null_mut(),
            self.len,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            self.fd,
            0,
        );

        if addr.is_null() {
            return Err(MyErr::Todo);
        }

        self.ptr = addr as *mut _ as *mut u8;

        Ok(())
    }

    // # Safety
    // We will panic if ptr is null OR buf.len != self.len
    pub fn mmap_flush(&mut self, buf: &[u8]) -> Res<()> {
        if self.ptr.is_null() || buf.len() != self.len {
            return Err(MyErr::Todo);
        }

        // for (i, b) in buf.iter().enumerate() {
        //     unsafe {
        //         self.ptr.add(i).write(*b);
        //     }
        // }

        let mem = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };
        mem.copy_from_slice(buf);

        Ok(())
    }
}

impl Tgui {
    pub fn new_buffer(&self, req: &Buffer) -> Res<BufferRes> {
        let Buffer { width, height } = *req;
        let method = Method {
            method: Some(method::Method::AddBuffer(AddBufferRequest {
                // BUG: it's rgba and not argb
                f: items::add_buffer_request::Format::Argb8888.into(),
                width,
                height,
            })),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        // i32
        let mut buf = [0u8; 4];
        let mut start = 0;
        while start < 4 {
            let ret = recv(self.main, &mut buf[start..], MsgFlags::empty()).unwrap();
            start += ret;
        }

        // the value can be discarded
        let id = i32::from_be_bytes(buf);

        let mut tmp = [0_u8; 1];
        let mut fds = cmsg_space!([RawFd; 2]);
        let mut fd = None;
        'l1: loop {
            let mut io_mut_buff = [std::io::IoSliceMut::new(&mut tmp)];
            let ret: RecvMsg<SockaddrStorage> = recvmsg(
                self.fd_main(),
                &mut io_mut_buff,
                Some(&mut fds),
                MsgFlags::MSG_CMSG_CLOEXEC,
            )
            .unwrap();

            for f in ret.cmsgs().into_iter() {
                //dbg!(&f);
                if let ControlMessageOwned::ScmRights(f) = f {
                    fd = Some(*f.first().unwrap());
                    break 'l1;
                }
            }
        }

        //dbg!(fds, id, fd);

        if id < 0 {
            Err(MyErr::Todo)
        } else {
            Ok(BufferRes {
                fd: fd.unwrap().into_raw_fd(),
                bid: id as usize,

                // TODO: enum rgb size
                len: req.size() * 4,
                ptr: std::ptr::null_mut(),
            })
        }
    }

    pub fn buffer_blit(&self, buffer: i32) -> Res<BlitBufferResponse> {
        let req = BlitBufferRequest { buffer };
        let method = Method {
            method: Some(method::Method::BlitBuffer(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    pub fn buffer_set(
        &self,
        aid: i32,
        img_res: &ImgRes,
        buffer_res: &BufferRes,
    ) -> Res<SetBufferResponse> {
        let method = Method {
            method: Some(method::Method::SetBuffer(SetBufferRequest {
                v: Some(items::View {
                    aid,
                    id: img_res.id,
                }),
                buffer: buffer_res.bid as i32,
            })),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
