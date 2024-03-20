use crate::{items::*, View, ViewSet, *};

pub type Space = WrapView<CreateSpaceRequest, CreateSpaceResponse>;

impl Tgui {
    pub fn new_space(&self, data: Create) -> Res<Space> {
        let mut res = Space {
            req: CreateSpaceRequest { data: Some(data) },
            res: None,
        };

        res.res = Some(self.sr(method::Method::CreateSpace(res.req.clone()))?);

        Ok(res)
    }
}

impl ViewSet for items::CreateSpaceRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateSpaceResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl View for Space {
    fn get_id(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().id)
    }
}
