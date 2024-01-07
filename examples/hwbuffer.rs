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
    let data = items::Create::new().set_aid(aid).set_v(100).set_parent(-1);
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

    //    let buffer = view::Buffer::new(&imgty)?;
    //    let mut buffer_res = tgui.new_buffer(&buffer)?;
    //    tgui.buffer_set(aid, &img_res, &buffer_res)?;
    //
    //    unsafe {
    //        buffer_res.mmap()?;
    //    }
    //
    //    {
    //        let buf = imgty.get_data()?;
    //        buffer_res.mmap_flush(buf)?;
    //
    //        tgui.buffer_blit(buffer_res.bid as i32)?;
    //        tgui.img_refresh(Some(items::View {
    //            aid,
    //            id: img_res.id,
    //        }))?;
    //    }

    sleep_ms(1000);

    unimplemented!();

    Ok(())
    // TODO:
}
