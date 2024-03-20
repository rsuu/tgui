use crate::{items::*, *};

#[derive(Debug)]
pub struct RLayout {
    pub wid: String,
    pub parent: i32,
    // req: Option<RLayoutReq>
    // res: Option<RLayoutRes>
}

#[derive(Debug)]
pub struct RLayoutReq {
    pub wid: String, // Option
    pub parent: i32, // Option
}

#[derive(Debug)]
pub struct RLayoutRes {
    pub rid: i32,
}

// RLayout::new().set_xxx().send_req();
// RLayout::new().id();
impl RLayout {
    pub fn new() -> Self {
        todo!()
    }

    pub fn id(&self) -> Option<i32> {
        todo!()
    }
}

impl Activity {
    pub fn new_rlayout(&self) -> Res<RLayoutRes> {
        let CreateRemoteLayoutResponse { rid, code } = self.sr(
            method::Method::CreateRemoteLayout(CreateRemoteLayoutRequest {}),
        )?;

        Ok(RLayoutRes { rid })
    }

    pub fn rlayout_flush(&self, req: &RLayout, res: &RLayoutRes) -> Res<SetWidgetLayoutResponse> {
        self.sr(method::Method::SetWidgetLayout(SetWidgetLayoutRequest {
            rid: res.rid,
            wid: req.wid.clone(),
        }))
    }
}

// View
impl Activity {
    pub fn rlayout_set_bg(
        &self,
        req: &RLayout,
        res: &RLayoutRes,
        view_id: i32,
        color: u32,
    ) -> Res<SetRemoteBackgroundColorResponse> {
        self.sr(method::Method::SetRemoteBackgroundColor(
            SetRemoteBackgroundColorRequest {
                rid: res.rid,
                id: view_id,
                color,
            },
        ))
    }

    pub fn rlayout_vis(
        &self,
        res: &RLayoutRes,
        view_id: i32,
        vis: i32,
    ) -> Res<SetRemoteVisibilityResponse> {
        self.sr(method::Method::SetRemoteVisibility(
            SetRemoteVisibilityRequest {
                rid: res.rid,
                id: view_id,
                v: vis,
            },
        ))
    }
}

// TextView
impl Activity {
    pub fn rlayout_add_text_view(
        &self,
        req: &RLayout,
        res: &RLayoutRes,
    ) -> Res<AddRemoteTextViewResponse> {
        self.sr(method::Method::AddRemoteTextView(
            AddRemoteTextViewRequest {
                rid: res.rid,
                parent: req.parent,
            },
        ))
    }

    pub fn rlayout_set_text(
        &self,
        res: &RLayoutRes,
        text_view_id: i32,
        text: String,
    ) -> Res<SetRemoteTextResponse> {
        self.sr(method::Method::SetRemoteText(SetRemoteTextRequest {
            rid: res.rid,
            id: text_view_id,
            text,
        }))
    }

    pub fn rlayout_set_text_color(
        &self,
        res: &RLayoutRes,
        text_view_id: i32,
        color: u32,
    ) -> Res<SetRemoteTextColorResponse> {
        self.sr(method::Method::SetRemoteTextColor(
            SetRemoteTextColorRequest {
                rid: res.rid,
                id: text_view_id,
                color,
            },
        ))
    }
}

// ImageView
impl Activity {
    pub fn rlayout_add_img_view(
        &self,
        req: &RLayout,
        res: &RLayoutRes,
    ) -> Res<AddRemoteImageViewResponse> {
        self.sr(method::Method::AddRemoteImageView(
            AddRemoteImageViewRequest {
                rid: res.rid,
                parent: req.parent,
            },
        ))
    }

    pub fn rlayout_set_img(
        &self,
        res: &RLayoutRes,
        img_view_id: i32,
        img: &ImgTy,
    ) -> Res<SetRemoteImageResponse> {
        self.sr(method::Method::SetRemoteImage(SetRemoteImageRequest {
            rid: res.rid,
            id: img_view_id,
            image: img.to_vec()?,
        }))
    }
}
