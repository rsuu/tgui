use crate::{items::*, View, ViewSet, *};

pub type Surface = WrapView<CreateSurfaceViewRequest, CreateSurfaceViewResponse>;

impl Activity {
    pub fn new_surface(&self, parent: i32, keyboard: bool, secure: bool) -> Res<Surface> {
        let mut res = Surface {
            req: CreateSurfaceViewRequest {
                data: Some(self.gen_create().unwrap().set_parent(parent)),
                keyboard,
                secure,
            },
            res: None,

            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::from(res.req.clone()))?);

        Ok(res)
    }
}

impl Surface {
    pub fn set_buffer(
        &self,
        v: &items::View,
        buffer: Option<&HwBufferRes>,
    ) -> Res<SurfaceViewSetBufferResponse> {
        self.act.sr(method::Method::SetSurfaceBuffer(
            SurfaceViewSetBufferRequest {
                v: Some(v.clone()),
                buffer: {
                    if let Some(buffer) = buffer {
                        buffer.bid
                    } else {
                        -1
                    }
                },
            },
        ))
    }
}

impl From<CreateSurfaceViewRequest> for method::Method {
    fn from(value: CreateSurfaceViewRequest) -> Self {
        Self::CreateSurfaceView(value)
    }
}
