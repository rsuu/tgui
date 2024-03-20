use crate::{
    items::{CreateFrameLayoutRequest, CreateFrameLayoutResponse},
    *,
};

pub type FrameLayout = WrapView<CreateFrameLayoutRequest, CreateFrameLayoutResponse>;

impl Activity {
    pub fn new_layout_frame(&self, parent: &impl View) -> Res<FrameLayout> {
        FrameLayout::new(self)
            .set_data(self.gen_create().unwrap().set_parent(parent.id()?))
            .conn()
    }

    pub fn new_top_layout_frame(&self) -> Res<FrameLayout> {
        FrameLayout::new(self)
            .set_data(self.gen_create().unwrap().set_parent(-1))
            .conn()
    }
}

impl From<CreateFrameLayoutRequest> for items::method::Method {
    fn from(value: items::CreateFrameLayoutRequest) -> Self {
        Self::CreateFrameLayout(value)
    }
}
