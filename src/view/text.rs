use prost::Message;

use crate::{
    items::{
        method, new_activity_request::ActivityType, CreateTextViewRequest, CreateTextViewResponse,
        Method, NewActivityRequest, NewActivityResponse, SetTextRequest, SetTextResponse,
    },
    *,
};

#[derive(Debug, Default)]
pub struct Text {
    data: Option<items::Create>,
    text: String,
    selectable_text: bool,
    clickable_links: bool,
}

impl Text {
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

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;

        self
    }

    pub fn flag_selectable_text(mut self) -> Self {
        self.selectable_text = true;

        self
    }

    pub fn flag_clickable_links(mut self) -> Self {
        self.clickable_links = true;

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

    pub fn set_v(mut self, v: i32) -> Self {
        if let Some(data) = self.data.as_mut() {
            data.v = v;
        }

        self
    }
}

impl Tgui {
    pub fn new_text_view(&self, req: Text) -> Result<CreateTextViewResponse, ()> {
        let req = CreateTextViewRequest {
            data: req.data,
            text: req.text,
            selectable_text: req.selectable_text,
            clickable_links: req.clickable_links,
        };
        let method = Method {
            method: Some(method::Method::CreateTextView(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice());

        self.recv_msg()
    }

    pub fn text_update(&self, v: Option<items::View>, text: String) -> Result<SetTextResponse, ()> {
        let a = SetTextRequest { v, text };
        let method = Method {
            method: Some(method::Method::SetText(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice());

        self.recv_msg()
    }
}
