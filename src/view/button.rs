use crate::{items::*, View, ViewSet, *};

pub type Button = WrapView<CreateButtonRequest, CreateButtonResponse>;

impl Tgui {
    pub fn new_button(&self, data: Create, allcaps: bool, text: String) -> Res<Button> {
        let mut res = Button {
            req: CreateButtonRequest {
                data: Some(data),
                allcaps,
                text,
            },
            res: None,
        };

        res.res = Some(self.sr(method::Method::CreateButton(res.req.clone()))?);

        Ok(res)
    }
}

impl ViewSet for items::CreateButtonRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateButtonResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl View for Button {
    fn get_id(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().id)
    }
}
