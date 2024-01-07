use prost::Message;

use crate::{
    items::{
        method, new_activity_request::ActivityType, Method, NewActivityRequest, NewActivityResponse,
    },
    *,
};

#[derive(Debug, Default)]
pub struct Activity {
    pub(crate) tid: i32,
    ty: ActivityType,
    intercept_back_button: bool,
}

impl Activity {
    pub fn new() -> Self {
        Self {
            tid: -1,
            ..Default::default()
        }
    }

    pub fn set_tid(mut self, tid: i32) -> Self {
        self.tid = tid;

        self
    }

    pub fn set_ty(mut self, ty: ActivityType) -> Self {
        self.ty = ty;

        self
    }

    pub fn falg_intercept_back_button(mut self) -> Self {
        self.intercept_back_button = true;

        self
    }
}

impl Tgui {
    // https://github.com/termux/termux-gui/blob/4cf436042f3f1cdf7ecf7f1b344ee9f70dc01733/app/src/main/java/com/termux/gui/protocol/protobuf/v0/HandleActivity.kt#L35
    // https://github.com/tareksander/termux-gui-c-bindings/blob/ee285bca72a3102fd02a187f19f854cef68bf43f/src/activity.cpp#L6
    pub fn new_activity(&self, act: Activity) -> Res<NewActivityResponse> {
        let req = NewActivityRequest {
            tid: act.tid,
            r#type: act.ty.into(),
            intercept_back_button: act.intercept_back_button,
        };

        let method = Method {
            method: Some(method::Method::NewActivity(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
