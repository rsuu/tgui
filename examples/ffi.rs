use nix::libc;
use std::ffi::*;
use std::ptr;

const LIBANDROID_SO: &str = "libandroid.so";

fn main() {
    let libandroid = LibAndroid::new();

    libandroid.destroy();
}

#[repr(C)]
pub struct TguiHardwareBuffer {
    id: i32,
    buffer: *mut AHardwareBuffer,
}

#[repr(C)]
pub struct AHardwareBuffer {
    _unused: [u8; 0],
}

#[link(name = "android")]
extern "C" {
    fn AHardwareBuffer_release(buffer: *mut AHardwareBuffer);
    fn AHardwareBuffer_recvHandleFromUnixSocket(
        socket: c_int,
        outBuffer: *mut *mut AHardwareBuffer,
    ) -> c_int;
    fn android_get_device_api_level() -> c_int;
}

pub struct LibAndroid {
    lib: *mut c_void,
    release: Option<unsafe extern "C" fn(*mut AHardwareBuffer)>,
    recv: Option<unsafe extern "C" fn(c_int, *mut *mut AHardwareBuffer) -> c_int>,
}

impl LibAndroid {
    pub fn new() -> LibAndroid {
        unsafe {
            let level = android_get_device_api_level();
            println!("api: {level}");

            if level < 26 {
                return LibAndroid {
                    lib: ptr::null_mut(),
                    release: None,
                    recv: None,
                };
            }

            let lib = libc::dlopen(
                CString::new(LIBANDROID_SO).unwrap().as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            if !lib.is_null() {
                let release = libc::dlsym(
                    lib,
                    CString::new("AHardwareBuffer_release").unwrap().as_ptr(),
                ) as *mut extern "C" fn(*mut AHardwareBuffer);
                let recv = libc::dlsym(
                    lib,
                    CString::new("AHardwareBuffer_recvHandleFromUnixSocket")
                        .unwrap()
                        .as_ptr(),
                )
                    as *mut extern "C" fn(c_int, *mut *mut AHardwareBuffer) -> c_int;

                println!("OK");
                println!("{:?}", lib);
                println!("{:?}", release);
                println!("{:?}", recv);

                LibAndroid {
                    lib,
                    release: Some(std::mem::transmute(release)),
                    recv: Some(std::mem::transmute(recv)),
                }
            } else {
                LibAndroid {
                    lib: ptr::null_mut(),
                    release: None,
                    recv: None,
                }
            }
        }
    }

    pub fn destroy(&self) {
        unsafe {
            if !self.lib.is_null() {
                libc::dlclose(self.lib);
            }
        }
    }
}
