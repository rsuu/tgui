use prost::Message;

use crate::{
    items::{
        self, method, CreateImageViewRequest, CreateImageViewResponse, Method, SetImageRequest,
        SetImageResponse,
    },
    *,
};

#[derive(Debug, Default)]
pub struct Img {
    data: Option<items::Create>,
    keyboard: bool,
}

impl Img {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn default_data(mut self) -> Self {
        self.data = Some(items::Create {
            parent: -1,
            ..Default::default()
        });

        self
    }

    pub fn set_parent(mut self, parent: i32) -> Self {
        if let Some(data) = self.data.as_mut() {
            data.parent = parent;
        }

        self
    }

    pub fn set_aid(mut self, aid: i32) -> Self {
        if let Some(data) = self.data.as_mut() {
            data.aid = aid;
        }

        self
    }
}

impl Tgui {
    pub fn new_img_view(&self, req: Img) -> Result<CreateImageViewResponse, ()> {
        let a = CreateImageViewRequest {
            data: req.data,
            keyboard: req.keyboard,
        };
        let method = Method {
            method: Some(method::Method::CreateImageView(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice());

        self.recv_msg()
    }

    pub fn img_update(
        &self,
        v: Option<items::View>,
        image: Vec<u8>,
    ) -> Result<SetImageResponse, ()> {
        let a = SetImageRequest { v, image };
        let method = Method {
            method: Some(method::Method::SetImage(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice());

        self.recv_msg()
    }
}
