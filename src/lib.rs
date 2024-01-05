pub mod activity;
pub mod connection;
pub mod event;
pub mod view;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}

pub use activity::Activity;
pub use connection::Tgui;
