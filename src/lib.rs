// REFS: https://github.com/ArtemisX64/tgui-rs/
//       https://github.com/termux/termux-gui/
//       https://github.com/tareksander/termux-gui-c-bindings/
//       https://github.com/sjfricke/NDK-Socket-IPC/

pub mod activity;
pub mod connection;
pub mod event;
pub mod ffi;
pub mod res;
pub mod view;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}

pub use view::{View, ViewSet};
pub use {
    activity::Activity,
    connection::Tgui,
    res::{MyErr, Res},
    view::img::ImgTy,
};
