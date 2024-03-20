// WIP

use crate::{items::*, *};

use nix::sys::socket::{recv, MsgFlags};
use prost::Message;

#[derive(Debug)]
pub struct HwBuffer {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct HwBufferRes {
    //pub fd: RawFd,
    pub bid: i32,

    pub len: usize,
    //pub ptr: *mut u8,
}

impl Activity {
    //pub fn new_hwbuffer(&self, req: &HwBuffer, libandroid: &LibAndroid) -> Res<HwBufferRes> {
    pub fn new_hwbuffer(&self, buffer: &HwBuffer) -> Res<HwBufferRes> {
        let HwBuffer { width, height } = *buffer;
        let msg = Method {
            method: Some(method::Method::CreateHardwareBuffer(
                CreateHardwareBufferRequest {
                    width: width as i32,
                    height: height as i32,
                    format: items::create_hardware_buffer_request::Format::Rgba8888.into(),
                    cpu_read: items::create_hardware_buffer_request::CpuUsage::Rarely.into(),
                    cpu_write: items::create_hardware_buffer_request::CpuUsage::Rarely.into(),
                },
            )),
        };
        let msg = msg.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        // i32
        let mut buf = [0u8; 4];
        let mut start = 0;
        while start < 4 {
            let ret = recv(self.fd_main(), &mut buf[start..], MsgFlags::empty()).unwrap();
            start += ret;
        }

        // the value can be discarded
        let bid = i32::from_be_bytes(buf);
        dbg!(&bid);

        //let mut a_hwb = crate::ffi::android::AHardwareBuffer { _unused: [] };

        //unsafe {
        //    let res = libandroid.fn_recv.unwrap()(self.main, &mut a_hwb as *mut _ as *mut _);
        //
        //    if res != 0 {
        //        panic!()
        //    }
        //}

        if bid > 0 {
            Ok(HwBufferRes {
                //fd: fd.unwrap().into_raw_fd(),
                //ptr: std::ptr::null_mut(),
                bid,
                len: buffer.len(),
            })
        } else {
            Err(MyErr::Msg("new_hwbuffer"))
        }
    }

    //    pub fn hwbuffer_set(&self, aid: i32, id: i32, bid: i32) -> Res<SurfaceViewSetBufferResponse> {
    //        let method = Method {
    //            method: Some(method::Method::SetSurfaceBuffer(
    //                SurfaceViewSetBufferRequest {
    //                    v: Some(items::View { aid, id }),
    //                    buffer: bid,
    //                },
    //            )),
    //        };
    //        let msg = method.encode_length_delimited_to_vec();
    //
    //        self.send_msg(msg.as_slice())?;
    //
    //        {
    //            let method = Method {
    //                method: Some(method::Method::SetBuffer(SetBufferRequest {
    //                    v: None,
    //                    buffer: bid,
    //                })),
    //            };
    //            let msg = method.encode_length_delimited_to_vec();
    //
    //            self.send_msg(msg.as_slice())?;
    //            self.recv_msg::<SetBufferResponse>()?;
    //        }
    //
    //        self.recv_msg()
    //    }
}

impl HwBuffer {
    pub fn new(imgty: &ImgTy) -> Res<Self> {
        let (width, height) = imgty.size()?;

        Ok(Self { width, height })
    }

    pub fn len(&self) -> usize {
        self.width as usize * 4 * self.height as usize
    }
}
