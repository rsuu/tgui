// TODO: Orientation

pub mod buffer;
pub mod hwbuffer;
pub mod img;
pub mod surface;
pub mod text;

use crate::{
    items::{set_gravity_request::Gravity, view_size::Constant, *},
    *,
};

pub use {
    buffer::{Buffer, BufferRes},
    hwbuffer::{HwBuffer, HwBufferRes},
    img::Img,
    surface::Surface,
    text::Text,
};

#[derive(Debug, Clone, Default)]
pub struct WrapView<Req, Res> {
    pub(crate) req: Req,
    pub(crate) res: Option<Res>,
    pub(crate) act: Activity,
}

impl<Req, Res> WrapView<Req, Res>
where
    Self: View,
    Req: Clone + Default + prost::Message,
    Res: Clone + Default + prost::Message,
{
    pub fn new(act: &Activity) -> Self {
        Self {
            act: act.clone(),
            ..Default::default()
        }
    }

    pub fn conn(mut self) -> crate::Res<Self>
    where
        items::method::Method: From<Req>,
    {
        self.res = Some(self.act.sr(items::method::Method::from(self.req.clone()))?);

        Ok(self)
    }

    pub fn req(&self) -> &Req {
        &self.req
    }

    pub fn mut_req(&mut self) -> &mut Req {
        &mut self.req
    }

    pub fn mut_res(&mut self) -> &mut Res {
        self.res.as_mut().unwrap()
    }
}

pub trait View: Sized {
    fn id(&self) -> Res<i32> {
        unreachable!()
    }

    fn parent(&self) -> Res<i32> {
        unreachable!()
    }

    fn visible(&self) -> Res<i32> {
        unreachable!()
    }

    fn act(&self) -> &Activity;

    fn aid(&self) -> Res<i32> {
        self.act().aid()
    }

    fn tid(&self) -> Res<i32> {
        self.act().tid()
    }

    fn view(&self) -> Option<items::View> {
        self.act().gen_view(self)
    }
}

pub trait ViewSet: Sized {
    fn set_id(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_aid(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_tid(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_parent(self, _: i32) -> Self {
        unreachable!()
    }

    fn set_v(self, _: items::Visibility) -> Self {
        unreachable!()
    }

    fn set_data(mut self, _: items::Create) -> Self {
        //self.req.data = Some(data);
        unreachable!()
    }
}

pub trait Vi: View {
    fn vi_clickable(&self, clickable: bool) -> Res<SetClickableResponse> {
        self.act()
            .sr(method::Method::SetClickable(SetClickableRequest {
                v: self.view(),
                clickable,
            }))
    }

    fn vi_visible(&self, vis: Visibility) -> Res<SetVisibilityResponse> {
        self.act()
            .sr(method::Method::SetVisibility(SetVisibilityRequest {
                v: self.view(),
                vis: vis.into(),
            }))
    }

    fn vi_click_event(&self, send: bool) -> Res<SetClickableResponse> {
        self.act()
            .sr(method::Method::SendClickEvent(SendClickEventRequest {
                v: self.view(),
                send,
            }))
    }

    fn vi_touch_event(&self, send: bool) -> Res<SetClickableResponse> {
        self.act()
            .sr(method::Method::SendTouchEvent(SendTouchEventRequest {
                v: self.view(),
                send,
            }))
    }

    fn vi_w(&self, w: f32, unit: Unit) -> Res<SetWidthResponse> {
        self.act().sr(method::Method::SetWidth(SetWidthRequest {
            v: self.view(),
            s: Some(ViewSize {
                value: Some(view_size::Value::Size(Size {
                    value: w,
                    unit: unit.into(),
                })),
            }),
        }))
    }

    fn vi_h(&self, h: f32, unit: Unit) -> Res<SetHeightResponse> {
        self.act().sr(method::Method::SetHeight(SetHeightRequest {
            v: self.view(),
            s: Some(ViewSize {
                value: Some(view_size::Value::Size(Size {
                    value: h,
                    unit: unit.into(),
                })),
            }),
        }))
    }

    fn vi_const_size(&self, size: Constant) -> Res<SetHeightResponse> {
        self.act().sr(method::Method::SetHeight(SetHeightRequest {
            v: self.view(),
            s: Some(ViewSize {
                value: Some(view_size::Value::Constant(size.into())),
            }),
        }))
    }

    fn vi_gravity(&self, horizontal: Gravity, vertical: Gravity) -> Res<SetGravityRequest> {
        self.act().sr(method::Method::SetGravity(SetGravityRequest {
            v: self.view(),
            horizontal: horizontal.into(),
            vertical: vertical.into(),
        }))
    }

    fn vi_padding(&self, size: Option<Size>, dire: Direction) -> Res<SetPaddingResponse> {
        self.act().sr(method::Method::SetPadding(SetPaddingRequest {
            v: self.view(),
            s: size,
            dir: dire.into(),
        }))
    }

    fn vi_bg(&self, color: u32) -> Res<SetBackgroundColorResponse> {
        self.act().sr(method::Method::SetBackgroundColor(
            SetBackgroundColorRequest {
                v: self.view(),
                color,
            },
        ))
    }

    fn vi_fg(&self, color: u32) -> Res<SetTextColorResponse> {
        self.act()
            .sr(method::Method::SetTextColor(SetTextColorRequest {
                v: self.view(),
                color,
            }))
    }

    fn config_keep_screen_on(&self) -> Res<SetClickableResponse> {
        self.act()
            .sr(method::Method::KeepScreenOn(KeepScreenOnRequest {
                aid: self.aid()?,
                on: true,
            }))
    }

    fn config_set_no_bar(&self) -> Res<ConfigureInsetsResponse> {
        self.act()
            .sr(method::Method::ConfigInsets(ConfigureInsetsRequest {
                aid: self.aid()?,
                shown: configure_insets_request::Bars::NoBar.into(),

                behaviour: configure_insets_request::BarBehaviour::Default.into(),
            }))
    }

    fn config_set_overlay_touch_event(&self) -> Res<SendOverlayTouchEventResponse> {
        self.act().sr(method::Method::SendOverlayTouch(
            SendOverlayTouchEventRequest {
                aid: self.aid()?,
                send: true,
            },
        ))
    }
}
impl<T: View> Vi for T {}

macro_rules! impl_view_for {
    ( $($t: tt),* ) => {
        $(
        impl View for $t {
            fn id(&self) -> Res<i32> {
                Ok(self.res.as_ref().unwrap().id)
            }

            fn act(&self) -> &Activity {
                &self.act
            }
        }

        impl ViewSet for $t {
            fn set_data(mut self, data: items::Create) -> Self {

                self.req.data = Some(data);

                self
            }
        }
        )*
    };
}

impl_view_for! {
    LinearLayout, GridLayout, FrameLayout,

    Button, ProgressBar, Space,

    Img, Text, Surface
}
