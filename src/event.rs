use crate::{items::*, View, *};

impl Activity {
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
        intercept_up: bool,
        intercept_down: bool,
    ) -> Res<InterceptVolumeButtonResponse> {
        self.sr(method::Method::InterceptVolume(
            InterceptVolumeButtonRequest {
                aid: self.aid()?,
                intercept_up,
                intercept_down,
            },
        ))
    }

    pub fn event_intercept_back(&self) -> Res<InterceptBackButtonResponse> {
        self.sr(method::Method::InterceptBackButton(
            InterceptBackButtonRequest {
                aid: self.aid()?,
                intercept: true,
            },
        ))
    }
}
