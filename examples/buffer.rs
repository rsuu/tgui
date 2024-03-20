use std::thread::sleep_ms;
use tgui::{Img, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;

    let imgty = ImgTy::open_rgba8888("test.jpg")?;
    let buffer = Buffer::new(&imgty)?;
    let mut buffer_res = tgui.new_buffer(&buffer)?;

    let data = act.gen_create().unwrap().set_parent(-1);
    let img = Img::new().set_data(data).conn(&tgui)?;
    let view = act.gen_view(img.res()).unwrap();
    buffer_res.set(&tgui, &act, &img)?;

    unsafe {
        buffer_res.mmap()?;
    }

    // frame 1
    {
        let buf = imgty.as_slice()?;
        buffer_res.mmap_flush(buf)?;
        buffer_res.blit(&tgui)?;

        let view = act.gen_view(img.res()).unwrap();
        img.refresh(&tgui, view.clone())?;

        sleep_ms(1000);
    }

    // frame 2
    let imgty = ImgTy::open_rgba8888("test2.jpg")?;
    let buf = imgty.as_slice()?;
    buffer_res.mmap_flush(buf)?;
    buffer_res.blit(&tgui)?;

    img.refresh(&tgui, view.clone())?;

    sleep_ms(3000);

    tgui.close();

    Ok(())
}
