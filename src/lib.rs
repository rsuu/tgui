pub mod connection;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/tgui.proto0.rs"));
}
