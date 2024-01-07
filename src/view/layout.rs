use prost::Message;

use crate::{
    items::{
        method, CreateFrameLayoutResponse, CreateLinearLayoutRequest, CreateRemoteLayoutRequest,
        CreateRemoteLayoutResponse, Method,
    },
    *,
};

#[derive(Debug, Default)]
pub struct Layout {}

impl Layout {}

impl Tgui {
    pub fn new_remote_layout(&self) -> Res<CreateRemoteLayoutResponse> {
        unimplemented!("https://github.com/search?q=repo%3Atareksander%2Ftermux-gui-c-bindings%20CreateRemoteLayoutRequest&type=code");
        let req = CreateRemoteLayoutRequest::default();
        let method = Method {
            method: Some(method::Method::CreateRemoteLayout(req)),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        let res = self.recv_msg::<CreateRemoteLayoutResponse>()?;

        if res.rid == -1 {
            Err(MyErr::Todo)
        } else {
            Ok(res)
        }
    }

    pub fn new_layout_linear(&self) -> Res<CreateLinearLayoutRequest> {
        todo!()
    }

    pub fn new_layout_frame(&self) -> Res<CreateFrameLayoutResponse> {
        todo!()
    }
}
