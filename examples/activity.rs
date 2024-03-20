use std::thread::sleep_ms;
use tgui::{View, ViewSet, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;

    // Threre has two ways to create a new request.
    // 1. with default config
    // 2. with custom config
    //

    let act = tgui.new_activity()?;
    //let act = Activity::new(&tgui)
    //    .set_ty(ActivityType::Normal)
    //    .set_intercept_back_button(false)
    //    .conn()?;

    sleep_ms(3000);

    tgui.close();

    Ok(())
}
