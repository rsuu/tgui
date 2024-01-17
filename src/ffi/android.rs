// Needed Termux TARGET_SDK >= 29

use nix::libc;
use std::ffi::*;

const LIBANDROID_SO: &str = "libandroid.so";

fn main() {
    unsafe {
        start();
    }
}

unsafe fn start() {
    let libandroid = LibAndroid::new();

    // // Configure and create AHardwareBuffer object
    // AHardwareBuffer* ahwb = nullptr;
    // AHardwareBuffer_Desc desc = ...
    // AHardwareBuffer_allocate(&desc, &ahwb);

    let (stride, height, width, layers) = (0, 0, 0, 1);
    let desc = AHardwareBuffer_Desc {
        stride,
        height,
        width,
        layers,
        //format: AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM,
        format: 0,
        // TODO:
        usage: 0,
        rfu0: 0,
        rfu1: 0,
    };

    let mut awb = AHardwareBuffer { _unused: [] };
    // TARGET_SDK >= 29
    {

        // dbg!(AHardwareBuffer_isSupported(&desc as *const _));
        // return 0 on success
        //let res = AHardwareBuffer_allocate(&desc as *const _, &mut awb as *mut _ as *mut _);
        //dbg!(res);
    }
}

pub struct LibAndroid {
    lib: *mut c_void,

    pub fn_release: Option<fn_AHardwareBuffer_release>,
    pub fn_recv: Option<fn_AHardwareBuffer_recvHandleFromUnixSocket>,
    pub fn_allocate: Option<fn_AHardwareBuffer_allocate>,
}

#[repr(C)]
pub struct TguiHardwareBuffer {
    id: i32,
    buffer: *mut AHardwareBuffer,
}

#[repr(C)]
pub struct AHardwareBuffer {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct AHardwareBuffer_Desc {
    pub width: u32,
    pub height: u32,
    pub layers: u32,
    pub format: u32,
    pub usage: u64,
    pub stride: u32,
    pub rfu0: u32,
    pub rfu1: u64,
}

type fn_AHardwareBuffer_release = unsafe extern "C" fn(buffer: *mut AHardwareBuffer);

type fn_AHardwareBuffer_recvHandleFromUnixSocket =
    unsafe extern "C" fn(socketFd: c_int, outBuffer: *mut *mut AHardwareBuffer) -> c_int;

type fn_AHardwareBuffer_allocate = unsafe extern "C" fn(
    desc: *const AHardwareBuffer_Desc,
    outBuffer: *mut *mut AHardwareBuffer,
) -> c_int;
type fn_AHardwareBuffer_isSupported =
    unsafe extern "C" fn(desc: *const AHardwareBuffer_Desc) -> c_int;

#[link(name = "android")]
extern "C" {
    fn android_get_device_api_level() -> c_int;

    fn AHardwareBuffer_release(buffer: *mut AHardwareBuffer);
    fn AHardwareBuffer_recvHandleFromUnixSocket(
        socket: c_int,
        outBuffer: *mut *mut AHardwareBuffer,
    ) -> c_int;
    fn AHardwareBuffer_allocate(
        desc: *const AHardwareBuffer_Desc,
        outBuffer: *mut *mut AHardwareBuffer,
    ) -> c_int;
    fn AHardwareBuffer_isSupported(desc: *const AHardwareBuffer_Desc) -> c_int;

}

impl LibAndroid {
    pub unsafe fn new() -> LibAndroid {
        let level = android_get_device_api_level();

        if level < 26 {
            panic!("Unsupported api level: {level}")
        }

        let s = CString::new(LIBANDROID_SO).unwrap();
        let lib = libc::dlopen(s.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);

        if lib.is_null() {
            panic!()
        }

        let s = CString::new("AHardwareBuffer_release").unwrap();
        let fn_release = libc::dlsym(lib, s.as_ptr()) as *mut fn_AHardwareBuffer_release;
        let fn_release = Some(std::mem::transmute(fn_release));

        let s = CString::new("AHardwareBuffer_recvHandleFromUnixSocket").unwrap();
        let fn_recv =
            libc::dlsym(lib, s.as_ptr()) as *mut fn_AHardwareBuffer_recvHandleFromUnixSocket;
        let fn_recv = Some(std::mem::transmute(fn_recv));

        let s = CString::new("AHardwareBuffer_isSupported").unwrap();
        let fn_allocate = libc::dlsym(lib, s.as_ptr()) as *mut fn_AHardwareBuffer_isSupported;
        let fn_allocate: Option<fn_AHardwareBuffer_isSupported> =
            Some(std::mem::transmute(fn_allocate));

        //        let s = CString::new("AHardwareBuffer_allocate").unwrap();
        //        let fn_allocate = libc::dlsym(lib, s.as_ptr()) as *mut fn_AHardwareBuffer_allocate;
        //        let fn_allocate = Some(std::mem::transmute(fn_allocate));
        // ERROR: undefined symbol: AHardwareBuffer_allocate
        let fn_allocate = None;

        LibAndroid {
            lib,
            fn_release,
            fn_recv,
            fn_allocate,
        }
    }

    pub unsafe fn destroy(&self) {
        if !self.lib.is_null() {
            libc::dlclose(self.lib);
        }
    }
}

impl Drop for LibAndroid {
    fn drop(&mut self) {
        unsafe {
            self.destroy();
        }
    }
}

// REFS: https://developer.android.com/ndk/reference/group/a-hardware-buffer#ahardwarebuffer
