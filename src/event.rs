use crate::{items::*, View, *};

#[derive(Debug, Default)]
pub struct Event {
    aid: i32,
    id: i32,

    res_event: Option<items::event::Event>,
    intercept: InterceptEvent,
}

#[derive(Debug, Default)]
pub struct InterceptEvent {
    volume_up: bool,
    volume_down: bool,

    back_button: bool,
}

pub enum InterceptTy {
    VolumeUp,
    VolumeDown,

    BackButton,
}

impl Event {
    pub fn new(act: &Activity, view: &dyn View) -> Res<Self> {
        Ok(Self {
            aid: act.get_id()?,
            id: view.get_id()?,

            ..Default::default()
        })
    }

    pub fn set_intercept(&mut self, e: InterceptTy, flag: bool) {
        use InterceptTy::*;

        let i = &mut self.intercept;
        let v = match e {
            VolumeUp => &mut i.volume_up,
            VolumeDown => &mut i.volume_down,
            BackButton => &mut i.back_button,
        };

        *v = flag;
    }

    pub fn intercept_volume(
        &self,
        tgui: &Tgui,
        act: &Activity,
    ) -> Res<InterceptVolumeButtonResponse> {
        tgui.sr(method::Method::InterceptVolume(
            InterceptVolumeButtonRequest {
                aid: act.get_id()?,
                intercept_up: self.intercept.volume_up,
                intercept_down: self.intercept.volume_down,
            },
        ))
    }

    pub fn intercept_back(&self, tgui: &Tgui, act: &Activity) -> Res<InterceptBackButtonResponse> {
        tgui.sr(method::Method::InterceptBackButton(
            InterceptBackButtonRequest {
                aid: act.get_id()?,
                intercept: self.intercept.back_button,
            },
        ))
    }

    pub fn res(&self) -> Option<&items::event::Event> {
        self.res_event.as_ref()
    }

    pub fn get_event(&mut self, tgui: &Tgui) -> Res<()> {
        let items::Event { event: Some(e) } = tgui.recv_event()? else {
            return Err(MyErr::Todo);
        };

        self.res_event = Some(e);

        Ok(())
    }

    pub fn click_on(&self, tgui: &Tgui, view: Option<items::View>) -> Res<SendClickEventResponse> {
        tgui.sr(method::Method::SendClickEvent(SendClickEventRequest {
            v: view,
            send: true,
        }))
    }

    pub fn touch_on(&self, tgui: &Tgui, view: Option<items::View>) -> Res<SendClickEventResponse> {
        tgui.sr(method::Method::SendTouchEvent(SendTouchEventRequest {
            v: view,
            send: true,
        }))
    }
}
