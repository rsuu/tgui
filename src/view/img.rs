use crate::{items::*, View, ViewSet, *};

pub type Img = WrapView<CreateImageViewRequest, CreateImageViewResponse>;

impl Img {
    pub fn update(&self, tgui: &Tgui, ity: &ImgTy, view: items::View) -> Res<SetImageResponse> {
        tgui.sr(method::Method::SetImage(SetImageRequest {
            v: Some(view),
            image: ity.to_vec()?,
        }))
    }

    pub fn refresh(&self, tgui: &Tgui, view: items::View) -> Res<RefreshImageViewResponse> {
        tgui.sr(method::Method::RefreshImageView(RefreshImageViewRequest {
            v: Some(view),
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

impl ViewSet for items::CreateImageViewRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl ViewSet for Img {
    fn set_data(mut self, data: items::Create) -> Self {
        self.req.data = Some(data);

        self
    }
}

impl View for items::CreateImageViewResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl View for Img {
    fn get_id(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().id)
    }
}

impl TguiDrop for items::View {
    fn drop(&mut self, tgui: &Tgui) -> Res<()> {
        let _: DeleteViewResponse =
            tgui.sr(method::Method::DeleteView(items::DeleteViewRequest {
                v: Some(self.clone()),
            }))?;

        Ok(())
    }
}
