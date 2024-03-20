// TODO: Orientation

use crate::{items::*, View, *};

use self::{set_gravity_request::Gravity, view_size::Constant};

// global
impl Tgui {
    pub fn config_keep_screen_on(&self, act: &Activity) -> Res<SetClickableResponse> {
        self.sr(method::Method::KeepScreenOn(KeepScreenOnRequest {
            aid: act.get_aid()?,
            on: true,
        }))
    }

    pub fn config_set_no_bar(&self, act: &Activity) -> Res<ConfigureInsetsResponse> {
        self.sr(method::Method::ConfigInsets(ConfigureInsetsRequest {
            aid: act.get_aid()?,
            shown: configure_insets_request::Bars::NoBar as i32,

            behaviour: configure_insets_request::BarBehaviour::Default as i32,
        }))
    }

    pub fn config_set_overlay_touch_event(
        &self,
        act: &Activity,
    ) -> Res<SendOverlayTouchEventResponse> {
        self.sr(method::Method::SendOverlayTouch(
            SendOverlayTouchEventRequest {
                aid: act.get_aid()?,

                send: true,
            },
        ))
    }
}

// view
impl Tgui {
    pub fn view_set_clickable(
        &self,
        view: items::View,
        clickable: bool,
    ) -> Res<SetClickableResponse> {
        self.sr(method::Method::SetClickable(SetClickableRequest {
            v: Some(view),
            clickable,
        }))
    }

    pub fn view_set_click_event(&self, view: items::View, send: bool) -> Res<SetClickableResponse> {
        self.sr(method::Method::SendClickEvent(SendClickEventRequest {
            v: Some(view),
            send,
        }))
    }

    pub fn view_set_touch_event(&self, view: items::View, send: bool) -> Res<SetClickableResponse> {
        self.sr(method::Method::SendTouchEvent(SendTouchEventRequest {
            v: Some(view),
            send,
        }))
    }

    pub fn view_set_w(&self, view: items::View, w: f32, unit: Unit) -> Res<SetWidthResponse> {
        self.sr(method::Method::SetWidth(SetWidthRequest {
            v: Some(view),
            s: Some(ViewSize {
                value: Some(view_size::Value::Size(Size {
                    value: w,
                    unit: unit as i32,
                })),
            }),
        }))
    }

    pub fn view_set_h(&self, view: items::View, h: f32, unit: Unit) -> Res<SetHeightResponse> {
        self.sr(method::Method::SetHeight(SetHeightRequest {
            v: Some(view),
            s: Some(ViewSize {
                value: Some(view_size::Value::Size(Size {
                    value: h,
                    unit: unit as i32,
                })),
            }),
        }))
    }

    pub fn view_set_const_size(&self, view: items::View, size: Constant) -> Res<SetHeightResponse> {
        self.sr(method::Method::SetHeight(SetHeightRequest {
            v: Some(view),
            s: Some(ViewSize {
                value: Some(view_size::Value::Constant(size as i32)),
            }),
        }))
    }

    pub fn view_set_gravity(
        &self,
        view: items::View,
        horizontal: Gravity,
        vertical: Gravity,
    ) -> Res<SetGravityRequest> {
        self.sr(method::Method::SetGravity(SetGravityRequest {
            v: Some(view),
            horizontal: horizontal as i32,
            vertical: vertical as i32,
        }))
    }

    pub fn view_set_padding(
        &self,
        view: items::View,
        size: Option<Size>,
        dire: Direction,
    ) -> Res<SetPaddingResponse> {
        self.sr(method::Method::SetPadding(SetPaddingRequest {
            v: Some(view),
            s: size,
            dir: dire as i32,
        }))
    }

    pub fn view_set_bg(&self, view: items::View, color: u32) -> Res<SetBackgroundColorResponse> {
        self.sr(method::Method::SetBackgroundColor(
            SetBackgroundColorRequest {
                v: Some(view),
                color,
            },
        ))
    }

    pub fn view_set_fg(&self, view: items::View, color: u32) -> Res<SetTextColorResponse> {
        self.sr(method::Method::SetTextColor(SetTextColorRequest {
            v: Some(view),
            color,
        }))
    }
}
