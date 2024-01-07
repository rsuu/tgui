use crate::{
    items::{CreateEvent},
    *,
};

#[derive(Debug)]
struct Event {}

impl Event {
    fn new() {
        let _event = CreateEvent {
            ..Default::default()
        };
    }
}

impl Tgui {
    pub fn event_auto_click(&self) {}
}
