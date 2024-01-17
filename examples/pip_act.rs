use std::thread::sleep_ms;
use tgui::{items, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;

    let act = Activity::new().set_ty(ActivityType::Pip).conn(&tgui)?;

    let data = items::Create::try_from(&act).unwrap();
    let img = Img::new().set_data(data).conn(&tgui)?;
    let ity = ImgTy::open_jpg("test.jpg").unwrap();
    img.update(&tgui, &ity, items::View::try_from(&act, &img))?;

    sleep_ms(10000);

    Ok(())
}
