use crate::{items::*, View, *};

pub use items::new_activity_request::ActivityType;

pub type Activity = WrapView<NewActivityRequest, NewActivityResponse>;

impl Activity {
    pub fn conn(mut self, tgui: &Tgui) -> Res<Self> {
        let res = tgui.sr(method::Method::NewActivity(self.req.clone()))?;

        self.res = Some(res);

        if self.res.as_ref().unwrap().tid == -1 && self.res.as_ref().unwrap().aid == -1 {
            return Err(MyErr::Todo);
        }

        Ok(self)
    }

    pub fn set_ty(mut self, ty: ActivityType) -> Self {
        self.req.r#type = ty.into();

        self
    }

    pub fn flag_intercept_back_button(mut self) -> Self {
        self.req.intercept_back_button = true;

        self
    }

    pub fn gen_create(&self) -> Option<items::Create> {
        let Ok(aid) = self.get_id() else {
            return None;
        };

        Some(items::Create::default().set_aid(aid))
    }

    pub fn gen_view(&self, res: &dyn View) -> Option<items::View> {
        let (Ok(aid), Ok(id)) = (self.get_id(), res.get_id()) else {
            return None;
        };

        Some(items::View { aid, id })
    }
}

impl View for Activity {
    fn new() -> Self {
        Self {
            req: NewActivityRequest {
                tid: -1,
                r#type: ActivityType::Normal.into(),
                intercept_back_button: false,
            },
            ..Default::default()
        }
    }

    fn get_id(&self) -> Res<i32> {
        self.get_aid()
    }

    fn get_aid(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().aid)
    }

    fn get_tid(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().tid)
    }
}

impl ViewSet for Activity {
    fn set_tid(mut self, tid: i32) -> Self {
        self.req.tid = tid;

        self
    }
}
