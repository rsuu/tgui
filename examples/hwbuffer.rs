use tgui::{
    ffi, items,
    view::{self, img::ImgTy, Img},
    *,
};

use std::thread::sleep_ms;

fn main() -> Res<()> {
    let libandroid = ffi::LibAndroid::new();

    // success
    let tgui = Tgui::new()?.conn()?;

    let act = Activity::new();
    let r_act = tgui.new_activity(act)?;
    dbg!(&r_act);

    let aid = r_act.get_aid()?;

    let imgty = ImgTy::open_rgba8888("test.jpg")?;
    let data = items::Create::new().set_aid(aid).set_v(2).set_parent(-1);
    let img = Img::new().set_data(data);
    let img_res = tgui.new_img_view(img)?;

    let hwbuffer = view::HwBuffer::new(&imgty)?;
    let hwbuffer_res = tgui.new_hwbuffer(&hwbuffer, &libandroid)?;
    tgui.hwbuffer_set(aid, img_res.id, hwbuffer_res.bid as i32)?;

    dbg!(&hwbuffer_res);

    {
        let surface_res = tgui.new_surface_view(aid)?;

        dbg!(tgui.hwbuffer_view_config(aid, surface_res.id)?);
    }

    unimplemented!();

    sleep_ms(10000);

    Ok(())
    // TODO:
}
