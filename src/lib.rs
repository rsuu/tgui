pub mod connection;
pub mod event;
pub mod ffi;
pub mod impls;
pub mod remote;
pub mod res;
pub mod utils;
pub mod view;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}

pub use {
    // trait
    view::{View, ViewSet},
    // struct/enum
    {
        connection::Tgui,
        event::{Event, InterceptTy},
        ffi::android::LibAndroid,
        remote::layout::RLayout,
        res::{MyErr, Res},
        utils::*,
        view::{
            activity::{Activity, ActivityType},
            buffer::{Buffer, BufferRes},
            hwbuffer::{HwBuffer, HwBufferRes},
            img::Img,
            surface::Surface,
            text::Text,
            widget::Widget,
            WrapView,
        },
    },
};

// REFS: https://github.com/ArtemisX64/tgui-rs/
//       https://github.com/termux/termux-gui/
//       https://github.com/tareksander/termux-gui-c-bindings/
//       https://github.com/tareksander/termux-gui-python-bindings/
//       https://github.com/sjfricke/NDK-Socket-IPC/
//       https://github.com/rust-mobile/ndk
