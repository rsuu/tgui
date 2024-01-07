pub mod buffer;
pub mod hwbuffer;
pub mod img;
pub mod impls;
pub mod layout;
pub mod surface;
pub mod text;
pub mod widget;

pub use {
    buffer::{Buffer, BufferRes},
    hwbuffer::{HwBuffer, HwBufferRes},
    img::{Img, ImgRes},
    surface::Surface,
    text::{RemoteText, Text},
    widget::Widget,
};

use crate::Res;

pub trait View {
    fn new() -> Self
    where
        Self: Sized,
    {
        unreachable!()
    }

    fn get_id(&self) -> Res<i32> {
        unreachable!()
    }

    fn get_aid(&self) -> Res<i32> {
        unreachable!()
    }

    fn get_tid(&self) -> Res<i32> {
        unreachable!()
    }

    fn get_parent(&self) -> Res<i32> {
        unreachable!()
    }

    fn get_v(&self) -> Res<i32> {
        unreachable!()
    }
}

pub trait ViewSet: Sized {
    fn set_id(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_aid(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_tid(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_parent(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_v(self, _v: i32) -> Self {
        unreachable!()
    }
}
