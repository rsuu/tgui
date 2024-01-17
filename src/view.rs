pub mod activity;
pub mod buffer;
pub mod hwbuffer;
pub mod img;
pub mod layout;
pub mod surface;
pub mod text;
pub mod views;
pub mod widget;

use crate::*;

#[derive(Debug, Default)]
pub struct WrapView<Req, Res> {
    req: Req,
    res: Option<Res>,
}

impl<Req, Res> WrapView<Req, Res>
where
    Req: ViewSet + Clone + prost::Message + Default,
    Res: View + Clone + prost::Message + Default,
    items::method::Method: From<Req>,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn conn(mut self, tgui: &Tgui) -> crate::Res<Self>
where {
        let res = tgui.sr(items::method::Method::from(self.req.clone()))?;

        self.res = Some(res);

        Ok(self)
    }

    pub fn req(&self) -> &Req {
        &self.req
    }

    pub fn res(&self) -> &Res {
        self.res.as_ref().unwrap()
    }

    pub fn mut_req(&mut self) -> &mut Req {
        &mut self.req
    }

    pub fn mut_res(&mut self) -> &mut Res {
        self.res.as_mut().unwrap()
    }

    pub fn get_req(self) -> Req {
        self.req
    }

    pub fn get_res(self) -> Res {
        self.res.unwrap()
    }

    pub fn set_data(mut self, data: items::Create) -> Self {
        self.req = ViewSet::set_data(self.req, data);

        self
    }

    pub fn get_id(&self) -> crate::Res<i32> {
        View::get_id(self.res())
    }
}

pub trait View {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
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

    fn set_v(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_data(self, _: items::Create) -> Self {
        unreachable!()
    }
}
