use crate::{
    items::{method, CreateFrameLayoutRequest, CreateFrameLayoutResponse},
    *,
};

pub type FrameLayout = WrapView<CreateFrameLayoutRequest, CreateFrameLayoutResponse>;

impl Tgui {
    pub fn new_layout_frame(&self, data: items::Create) -> Res<CreateFrameLayoutResponse> {
        self.sr(method::Method::CreateFrameLayout(
            items::CreateFrameLayoutRequest { data: Some(data) },
        ))
    }
}

impl ViewSet for items::CreateFrameLayoutRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateFrameLayoutResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl View for FrameLayout {
    fn get_id(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().id)
    }
}
