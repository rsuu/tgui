use tgui::{
    items,
    view::{self, img::ImgTy, Img},
    Activity, Res, Tgui, View, ViewSet,
};

use std::thread::sleep_ms;

fn main() -> Res<()> {
    // success
    let tgui = Tgui::new()?.conn()?;

    let act = Activity::new();
    let r_act = tgui.new_activity(act)?;
    dbg!(&r_act);

    let aid = r_act.get_aid()?;

    let imgty = ImgTy::open_rgba8888("test.jpg")?;
    let buffer = view::Buffer::new(&imgty)?;
    let mut buffer_res = tgui.new_buffer(&buffer)?;

    let data = items::Create::new().set_aid(aid).set_v(100).set_parent(-1);
    let img = Img::new().set_data(data);
    let img_res = tgui.new_img_view(img)?;
    tgui.buffer_set(aid, &img_res, &buffer_res)?;

    unsafe {
        buffer_res.mmap()?;
    }

    // frame 1
    {
        let buf = imgty.get_data()?;
        buffer_res.mmap_flush(buf)?;

        tgui.buffer_blit(buffer_res.bid as i32)?;
        tgui.img_refresh(Some(items::View {
            aid,
            id: img_res.id,
        }))?;
        sleep_ms(1000);
    }

    // frame 2
    let imgty = ImgTy::open_argb8888("test2.jpg")?;
    loop {
        let buf = imgty.get_data()?;
        buffer_res.mmap_flush(buf)?;

        tgui.buffer_blit(buffer_res.bid as i32)?;
        tgui.img_refresh(Some(items::View {
            aid,
            id: img_res.id,
        }))?;

        sleep_ms(100);
    }
}
