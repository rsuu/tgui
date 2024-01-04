use nix::sys::socket::{recv, send, MsgFlags};
use prost::Message;
use std::thread::sleep_ms;

use tgui::{
    connection::Tgui,
    items::{
        self, method, new_activity_request::ActivityType, AddBufferRequest, CreateTextViewRequest,
        CreateTextViewResponse, Method, NewActivityRequest, NewActivityResponse, SetTextResponse,
        View,
    },
};

fn main() {
    // success
    //new_json();
    //sleep_ms(3000);

    // failed
    new_proto();
    sleep_ms(3000);
}

fn new_proto() {
    // success
    let tgui = Tgui::new(tgui::connection::ProtocolType::Proto);

    let msg = {
        let activity = NewActivityRequest {
            tid: -1,
            r#type: ActivityType::Normal.into(),
            intercept_back_button: false,
        };
        let method = Method {
            method: Some(method::Method::NewActivity(activity)),
        };

        method.encode_to_vec()
    };

    // msg:
    // [
    // 130, 1, 11, 8,
    // 255, 255, 255, 255,
    // 255, 255, 255, 255,
    // 255, 1
    // ]
    //
    dbg!(&msg);
    // let len = (msg.len() as u32).to_be_bytes();
    // tgui.send_msg(&len[..]);

    // failed: nothing happened :(
    tgui.send_msg(msg.as_slice());

    // https://github.com/termux/termux-gui/blob/4cf436042f3f1cdf7ecf7f1b344ee9f70dc01733/app/src/main/java/com/termux/gui/protocol/protobuf/v0/HandleActivity.kt#L35
    // https://github.com/tareksander/termux-gui-c-bindings/blob/ee285bca72a3102fd02a187f19f854cef68bf43f/src/activity.cpp#L6
}

fn new_json() {
    let tgui = Tgui::new(tgui::connection::ProtocolType::Json);

    let msg = r#"{
        "method": "newActivity",
        "params": {}
    }"#
    .as_bytes();
    // REFS: For the JSON Protocol, each message must be preceded by the length of the message (without this length value) as a 4 byte unsigned integer, the same with the return messages from the plugin. This integer is send big-endian.
    let len = (msg.len() as u32).to_be_bytes();
    tgui.send_msg(len.as_slice());
    tgui.send_msg(msg);
}
