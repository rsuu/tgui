use prost::Message;

use crate::{
    items::{
        method, AddRemoteTextViewRequest, AddRemoteTextViewResponse, CreateTextViewRequest, CreateTextViewResponse, Method,
        SetRemoteTextRequest, SetRemoteTextResponse, SetTextRequest, SetTextResponse,
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

#[derive(Debug, Default)]
pub struct RemoteText {
    // req
    rid: i32,
    parent: i32,
    text: String,
}

impl RemoteText {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;

        self
    }

    pub fn set_parent(mut self, parent: i32) -> Self {
        self.parent = parent;

        self
    }
}

impl Text {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;

        self
    }

    pub fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

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
}

impl Tgui {
    pub fn new_text_view(&self, req: Text) -> Res<CreateTextViewResponse> {
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

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    pub fn text_update(&self, v: Option<items::View>, text: String) -> Res<SetTextResponse> {
        let a = SetTextRequest { v, text };
        let method = Method {
            method: Some(method::Method::SetText(a)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}

impl Tgui {
    pub fn new_remote_text_view(&self, req: &RemoteText) -> Res<AddRemoteTextViewResponse> {
        let req = AddRemoteTextViewRequest {
            rid: req.rid,
            parent: req.parent,
        };
        let method = Method {
            method: Some(method::Method::AddRemoteTextView(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    pub fn remote_text_update(
        &self,
        text: String,
        rid: i32,
        id: i32,
    ) -> Res<SetRemoteTextResponse> {
        let req = SetRemoteTextRequest { text, rid, id };
        let method = Method {
            method: Some(method::Method::SetRemoteText(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
