use crate::{items::*, View, ViewSet, *};

use self::items::SetLinearLayoutParamsResponse;

pub type LinearLayout = WrapView<CreateLinearLayoutRequest, CreateLinearLayoutResponse>;

impl Tgui {
    pub fn new_layout_linear(
        &self,
        data: items::Create,
        flag_horizontal: bool,
    ) -> Res<LinearLayout> {
        let mut res = LinearLayout {
            req: CreateLinearLayoutRequest {
                data: Some(data),
                horizontal: flag_horizontal,
            },
            res: None,
        };

        res.res = Some(self.sr(method::Method::CreateLinearLayout(res.req.clone()))?);

        Ok(res)
    }

    pub fn set_layout_linear(
        &self,
        v: items::View,
        weight: f32,
        position: i32,
    ) -> Res<SetLinearLayoutParamsResponse> {
        self.sr(method::Method::SetLinearLayout(
            items::SetLinearLayoutParamsRequest {
                v: Some(v),
                weight,
                position,
            },
        ))
    }
}

impl ViewSet for items::CreateLinearLayoutRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateLinearLayoutResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl View for LinearLayout {
    fn get_id(&self) -> Res<i32> {
        Ok(self.res.as_ref().unwrap().id)
    }
}

// REFS: https://developer.android.com/reference/android/widget/LinearLayout
