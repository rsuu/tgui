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
        method, AddBufferRequest, BlitBufferRequest, BlitBufferResponse, DeleteBufferResponse,
        Method, SetBufferRequest, SetBufferResponse,
    },
    Img, ImgTy, *,
};

impl Activity {
    pub fn new_buffer(&self, req: &Buffer) -> Res<BufferRes> {
        let Buffer { width, height, .. } = *req;
        let method = Method {
            method: Some(method::Method::AddBuffer(AddBufferRequest {
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
            let ret = recv(self.fd_main(), &mut buf[start..], MsgFlags::empty()).unwrap();
            start += ret;
        }

        let bid = i32::from_be_bytes(buf);
        if bid < 0 {
            return Err(MyErr::Todo);
        }

        let mut tmp = [0_u8; 1];
        let mut fds = cmsg_space!([RawFd; 2]);
        let mut fd = Option::<i32>::None;
        'l1: loop {
            let mut io_mut_buff = [std::io::IoSliceMut::new(&mut tmp)];
            let ret: RecvMsg<SockaddrStorage> = recvmsg(
                self.fd_main(),
                &mut io_mut_buff,
                Some(&mut fds),
                MsgFlags::MSG_CMSG_CLOEXEC,
            )
            .unwrap();

            if ret.bytes != 1 {
                return Err(MyErr::Todo);
            }

            for f in ret.cmsgs().into_iter() {
                if let ControlMessageOwned::ScmRights(v) = f {
                    fd = v.first().copied();

                    break 'l1;
                }
            }
        }

        //dbg!(fds, id, fd);

        Ok(BufferRes {
            fd: fd.unwrap().into_raw_fd(),
            bid,

            // TODO: enum rgb size
            len: req.size() * 4,
            ptr: std::ptr::null_mut(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: u32,
    pub height: u32,

    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct BufferRes {
    pub fd: RawFd,
    pub bid: i32,

    len: usize,
    ptr: *mut u8, // Vec<u8>
}

impl Buffer {
    pub fn new(imgty: &ImgTy) -> Res<Self> {
        let (width, height) = imgty.size()?;

        Ok(Self {
            width,
            height,
            data: imgty.as_slice()?.to_vec(),
        })
    }

    pub fn zero(width: u32, height: u32) -> Res<Self> {
        Ok(Self {
            width,
            height,
            data: vec![0; width as usize * height as usize * 4],
        })
    }

    pub fn size(&self) -> usize {
        self.width as usize * self.height as usize
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}

impl BufferRes {
    pub fn blit(&self, act: &Activity) -> Res<()> {
        act.sr(method::Method::BlitBuffer(BlitBufferRequest {
            buffer: self.bid,
        }))
    }

    pub fn set(&self, act: &Activity, img: &Img) -> Res<()> {
        act.sr(method::Method::SetBuffer(SetBufferRequest {
            v: Some(items::View {
                aid: act.aid()?,
                id: img.id()?,
            }),
            buffer: self.bid,
        }))
    }

    // unsafe: `fd` is created in cpp.
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

    pub fn mmap_flush(&mut self, buf: &[u8]) -> Res<()> {
        // for (i, b) in buf.iter().enumerate() {
        //     unsafe {
        //         self.ptr.add(i).write(*b);
        //     }
        // }

        let mem = self.mmap_from_ptr(buf.len())?;
        mem.copy_from_slice(buf);

        Ok(())
    }

    pub fn mmap_flush_with_swap(&mut self, buf: &mut [u8]) -> Res<()> {
        let mem = self.mmap_from_ptr(buf.len())?;
        mem.swap_with_slice(buf);

        Ok(())
    }

    // # Safety
    // Make sure `buf.len == self.len`
    fn mmap_from_ptr(&mut self, len: usize) -> Res<&mut [u8]> {
        debug_assert_eq!(self.len, len);

        if self.ptr.is_null() {
            return Err(MyErr::Todo);
        }

        let mem = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };

        Ok(mem)
    }

    //    pub fn delete(&mut self, act: &Activity) -> Res<DeleteBufferResponse> {
    //        unsafe {
    //            if nix::libc::close(self.fd) != 0 {
    //                return Err(MyErr::Todo);
    //            }
    //
    //            nix::libc::munmap(self.ptr as *mut _, self.len);
    //        }
    //
    //        act.sr(method::Method::DeleteBuffer(items::DeleteBufferRequest {
    //            buffer: self.bid,
    //        }))
    //    }
}
