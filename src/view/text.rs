use crate::{items::*, View, ViewSet, *};

pub type Text = WrapView<CreateTextViewRequest, CreateTextViewResponse>;

impl Text {
    pub fn update(
        &self,
        tgui: &Tgui,
        text: String,
        view: Option<items::View>,
    ) -> Res<SetTextResponse> {
        tgui.sr(method::Method::SetText(SetTextRequest { v: view, text }))
    }
}

impl Text {
    pub fn set_text(mut self, text: String) -> Self {
        self.req.text = text;

        self
    }

    pub fn flag_selectable_text(mut self) -> Self {
        self.req.selectable_text = true;

        self
    }

    pub fn flag_clickable_links(mut self) -> Self {
        self.req.clickable_links = true;

        self
    }
}

impl ViewSet for items::CreateTextViewRequest {
    fn set_data(mut self, data: items::Create) -> Self {
        self.data = Some(data);

        self
    }
}

impl View for items::CreateTextViewResponse {
    fn get_id(&self) -> Res<i32> {
        Ok(self.id)
    }
}

impl From<CreateTextViewRequest> for method::Method {
    fn from(value: CreateTextViewRequest) -> Self {
        Self::CreateTextView(value)
    }
}
