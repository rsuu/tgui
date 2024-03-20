use crate::{items::*, View, ViewSet, *};

use self::items::SetLinearLayoutParamsResponse;

pub type LinearLayout = WrapView<CreateLinearLayoutRequest, CreateLinearLayoutResponse>;

impl Activity {
    pub fn new_layout_linear(
        &self,
        parent: &impl View,
        flag_horizontal: bool,
    ) -> Res<LinearLayout> {
        let mut res = LinearLayout {
            req: CreateLinearLayoutRequest {
                data: Some(self.gen_create().unwrap().set_parent(parent.id()?)),
                horizontal: flag_horizontal,
            },
            res: None,
            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::CreateLinearLayout(res.req.clone()))?);

        Ok(res)
    }

    pub fn new_top_layout_linear(&self, flag_horizontal: bool) -> Res<LinearLayout> {
        let mut res = LinearLayout {
            req: CreateLinearLayoutRequest {
                data: Some(self.gen_create().unwrap().set_parent(-1)),
                horizontal: flag_horizontal,
            },
            res: None,
            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::CreateLinearLayout(res.req.clone()))?);

        Ok(res)
    }

    pub fn set_layout_linear(
        &self,
        v: &impl View,
        weight: f32,
        position: i32,
    ) -> Res<SetLinearLayoutParamsResponse> {
        self.sr(method::Method::SetLinearLayout(
            items::SetLinearLayoutParamsRequest {
                v: self.gen_view(v),
                weight,
                position,
            },
        ))
    }
}

// REFS: https://developer.android.com/reference/android/widget/LinearLayout
