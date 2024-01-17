use crate::{items::*, View, ViewSet, *};

pub type Img = WrapView<CreateImageViewRequest, CreateImageViewResponse>;

impl Img {
    pub fn update(
        &self,
        tgui: &Tgui,
        ity: &ImgTy,
        view: Option<items::View>,
    ) -> Res<SetImageResponse> {
        tgui.sr(method::Method::SetImage(SetImageRequest {
            v: view,
            image: ity.to_vec()?,
        }))
    }

    pub fn refresh(&self, tgui: &Tgui, view: Option<items::View>) -> Res<RefreshImageViewResponse> {
        tgui.sr(method::Method::RefreshImageView(RefreshImageViewRequest {
            v: view,
        }))
    }
}

impl ViewSet for items::CreateImageViewRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateImageViewResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl From<CreateImageViewRequest> for method::Method {
    fn from(value: CreateImageViewRequest) -> Self {
        Self::CreateImageView(value)
    }
}
