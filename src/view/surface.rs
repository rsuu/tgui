use crate::{
    items::{
        method, CreateSurfaceViewResponse, Method, SurfaceViewConfigRequest,
        SurfaceViewConfigResponse,
    },
    *,
};

use prost::Message;

#[derive(Debug)]
pub struct Surface {}

impl Tgui {
    pub fn new_surface_view(&self, aid: i32) -> Res<CreateSurfaceViewResponse> {
        let method = Method {
            method: Some(method::Method::CreateSurfaceView(
                items::CreateSurfaceViewRequest {
                    data: Some(items::Create {
                        aid,
                        parent: -1,
                        v: 100,
                    }),
                    keyboard: false,
                    secure: false,
                },
            )),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }

    pub fn hwbuffer_view_config(&self, aid: i32, id: i32) -> Res<SurfaceViewConfigResponse> {
        let method = Method {
            method: Some(method::Method::SurfaceConfig(SurfaceViewConfigRequest {
                v: Some(items::View { aid, id }),
                ..Default::default()
            })),
        };
        let msg = method.encode_length_delimited_to_vec();

        self.send_msg(msg.as_slice())?;

        self.recv_msg()
    }
}
