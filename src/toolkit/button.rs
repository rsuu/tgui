use crate::{items::*, ViewSet, *};

pub type Button = WrapView<CreateButtonRequest, CreateButtonResponse>;

impl Activity {
    pub fn new_button(&self, parent: i32, allcaps: bool, text: String) -> Res<Button> {
        let mut res = Button {
            req: CreateButtonRequest {
                data: Some(self.gen_create().unwrap().set_parent(parent)),
                allcaps,
                text,
            },
            res: None,
            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::CreateButton(res.req.clone()))?);

        Ok(res)
    }
}
