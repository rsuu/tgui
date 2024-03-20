use crate::{items::*, View, ViewSet, *};

pub type Img = WrapView<CreateImageViewRequest, CreateImageViewResponse>;

impl Img {
    pub fn update(&self, ity: &ImgTy, view: items::View) -> Res<SetImageResponse> {
        self.act.sr(method::Method::SetImage(SetImageRequest {
            v: Some(view),
            image: ity.to_vec()?,
        }))
    }

    pub fn refresh(&self, v: &impl View) -> Res<RefreshImageViewResponse> {
        self.act
            .sr(method::Method::RefreshImageView(RefreshImageViewRequest {
                v: self.act().gen_view(v),
            }))
    }

    pub fn use_keyboard(mut self) -> Self {
        self.req.keyboard = true;

        self
    }
}

impl From<CreateImageViewRequest> for items::method::Method {
    fn from(value: items::CreateImageViewRequest) -> Self {
        Self::CreateImageView(value)
    }
}
