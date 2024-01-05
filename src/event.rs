use crate::{
    items::{self, CreateEvent},
    *,
};

#[derive(Debug)]
struct Event {}

fn new() {
    let event = CreateEvent {
        ..Default::default()
    };
}

impl Tgui {
    pub fn event_auto_click(&self) {}
}
