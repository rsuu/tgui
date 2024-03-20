use prost::Message;

use crate::{
    items::{method, Method, SetWidgetLayoutRequest, SetWidgetLayoutResponse},
    *,
};

#[derive(Debug, Default)]
pub struct Widget {
    rid: i32,
    wid: String,
}

impl Widget {
    pub fn new() -> Self {
        Self {
            rid: -1,
            ..Default::default()
        }
    }

    pub fn set_rid(mut self, rid: i32) -> Self {
        self.rid = rid;

        self
    }

    pub fn set_wid(mut self, wid: String) -> Self {
        self.wid = wid;

        self
    }
}

impl Activity {
    pub fn new_widget_view(&self, req: &Widget) -> Res<SetWidgetLayoutResponse> {
        let req = SetWidgetLayoutRequest {
            rid: req.rid,
            wid: req.wid.clone(),
        };
        let method = Method {
            method: Some(method::Method::SetWidgetLayout(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
