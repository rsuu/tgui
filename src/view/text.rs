use crate::{items::*, View, ViewSet, *};

pub type Text = WrapView<CreateTextViewRequest, CreateTextViewResponse>;

impl Activity {
    pub fn new_text(&self, parent: &impl View, text: String) -> Res<Text> {
        let mut res = Text {
            req: CreateTextViewRequest {
                data: Some(self.gen_create().unwrap().set_parent(parent.id()?)),
                text,
                selectable_text: true,
                clickable_links: true,
            },
            res: None,
            act: self.clone(),
        };

        res.res = Some(self.sr(method::Method::CreateTextView(res.req.clone()))?);

        Ok(res)
    }
}

impl Text {
    pub fn update(&self, text: String) -> Res<SetTextResponse> {
        self.act.sr(method::Method::SetText(SetTextRequest {
            v: self.act.gen_view(self),
            text,
        }))
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.req.text = text;

        self
    }

    pub fn set_selectable_text(mut self, flag: bool) -> Self {
        self.req.selectable_text = flag;

        self
    }

    pub fn set_clickable_links(mut self, flag: bool) -> Self {
        self.req.clickable_links = flag;

        self
    }
}

impl From<CreateTextViewRequest> for method::Method {
    fn from(value: CreateTextViewRequest) -> Self {
        Self::CreateTextView(value)
    }
}
