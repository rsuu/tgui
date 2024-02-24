use crate::{items::*, View, *};

impl Tgui {
    pub fn get_event(&self) -> Res<Option<items::event::Event>> {
        Ok(
            if let items::Event { event: Some(e) } = self.recv_event()? {
                Some(e)
            } else {
                None
            },
        )
    }

    pub fn event_intercept_volume(
        &self,
        act: &Activity,
        intercept_up: bool,
        intercept_down: bool,
    ) -> Res<InterceptVolumeButtonResponse> {
        self.sr(method::Method::InterceptVolume(
            InterceptVolumeButtonRequest {
                aid: act.get_aid()?,
                intercept_up,
                intercept_down,
            },
        ))
    }

    pub fn event_intercept_back(&self, act: &Activity) -> Res<InterceptBackButtonResponse> {
        self.sr(method::Method::InterceptBackButton(
            InterceptBackButtonRequest {
                aid: act.get_aid()?,
                intercept: true,
            },
        ))
    }
}
