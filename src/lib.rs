pub mod activity;
pub mod any;
pub mod event;
pub mod ffi;
pub mod impls;
pub mod layout;
pub mod remote;
pub mod res;
pub mod task;
pub mod toolkit;
pub mod utils;
pub mod view;
pub mod widget;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}

pub use {
    any::Any,
    // trait
    view::{Vi, View},
    // struct/enum
    {
        activity::*,
        ffi::android::LibAndroid,
        items::{
            set_gravity_request::Gravity, set_grid_layout_params_request::Alignment, size::Unit,
            view_size::Constant, Direction,
        },
        layout::*,
        remote::layout::RLayout,
        res::{MyErr, Res},
        task::Task,
        toolkit::*,
        utils::*,
        view::*,
        widget::Widget,
    },
};

// REFS: https://github.com/ArtemisX64/task-rs/
//       https://github.com/termux/termux-gui/
//       https://github.com/tareksander/termux-gui-c-bindings/
//       https://github.com/tareksander/termux-gui-python-bindings/
//       https://github.com/sjfricke/NDK-Socket-IPC/
//       https://github.com/rust-mobile/ndk
