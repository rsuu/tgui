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

    tgui.buffer_set(act.get_id()?, &img, &buffer_res)?;
    unsafe {
        buffer_res.mmap()?;
    }

    // frame 1
    {
        let buf = imgty.as_slice()?;
        buffer_res.mmap_flush(buf)?;

        tgui.buffer_blit(buffer_res.bid as i32)?;
        let view = act.gen_view(img.res());
        img.refresh(&tgui, view)?;

        sleep_ms(1000);
    }

    // frame 2
    let imgty = ImgTy::open_rgba8888("test2.jpg")?;
    loop {
        let buf = imgty.as_slice()?;
        buffer_res.mmap_flush(buf)?;

        tgui.buffer_blit(buffer_res.bid as i32)?;
        let view = act.gen_view(img.res());
        img.refresh(&tgui, view)?;

        sleep_ms(100);
    }
}
