use std::thread::sleep_ms;
use tgui::{View, ViewSet, *};

fn main() {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;

    let data = act.gen_create().unwrap().set_parent(-1);
    let btn = tgui.new_button(Some(data), false, "btn".to_string())?;
    let mut event = Event::new(&act, btn.res())?;
}
