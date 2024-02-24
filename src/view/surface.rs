use crate::{items::*, View, ViewSet, *};

pub type Surface = WrapView<CreateSurfaceViewRequest, CreateSurfaceViewResponse>;

impl Tgui {
    pub fn new_surface(&self, data: Option<Create>, keyboard: bool, secure: bool) -> Res<Surface> {
        let mut res = Surface {
            req: CreateSurfaceViewRequest {
                data,
                keyboard,
                secure,
            },
            res: None,
        };

        res.res = Some(self.sr(method::Method::from(res.req.clone()))?);

        Ok(res)
    }
}

impl Surface {
    pub fn set_buffer(
        &self,
        tgui: &Tgui,
        v: Option<items::View>,
        buffer: &BufferRes,
    ) -> Res<SurfaceViewSetBufferResponse> {
        tgui.sr(method::Method::SetSurfaceBuffer(
            SurfaceViewSetBufferRequest {
                v,
                buffer: buffer.bid,
            },
        ))
    }
}

impl ViewSet for items::CreateSurfaceViewRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateSurfaceViewResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl From<CreateSurfaceViewRequest> for method::Method {
    fn from(value: CreateSurfaceViewRequest) -> Self {
        Self::CreateSurfaceView(value)
    }
}
