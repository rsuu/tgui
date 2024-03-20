use crate::{items::*, View, ViewSet, *};

pub type Space = WrapView<CreateSpaceRequest, CreateSpaceResponse>;

impl Activity {
    pub fn new_space(&self, parent: &impl View) -> Res<Space> {
        let mut res = Space {
            req: CreateSpaceRequest {
                data: Some(self.gen_create().unwrap().set_parent(parent.id()?)),
            },
            res: None,
            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::CreateSpace(res.req.clone()))?);

        Ok(res)
    }
}
