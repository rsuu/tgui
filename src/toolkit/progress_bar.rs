use crate::{items::*, View, ViewSet, *};

pub type ProgressBar = WrapView<CreateProgressBarRequest, CreateProgressBarResponse>;

impl Activity {
    pub fn new_progress_bar(&self, parent: &impl View) -> Res<ProgressBar> {
        ProgressBar::new(self)
            .set_data(self.gen_create().unwrap().set_parent(parent.id()?))
            .conn()
    }
}

impl ProgressBar {
    pub fn set(&self, progress: u32) -> Res<SetProgressResponse> {
        self.act.sr(method::Method::SetProgress(SetProgressRequest {
            v: Some(items::View {
                aid: self.act.aid()?,
                id: self.id()?,
            }),
            progress,
        }))
    }
}

impl From<CreateProgressBarRequest> for items::method::Method {
    fn from(value: items::CreateProgressBarRequest) -> Self {
        Self::CreateProgressBar(value)
    }
}
