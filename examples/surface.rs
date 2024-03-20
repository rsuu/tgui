// Layout:
//
// <FrameLayout>
//     <SrfaceView />
// </FrameLayout>

use std::{collections::HashMap, thread::sleep_ms};
use tgui::{items::Visibility, View, ViewSet, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;
    //tgui.config_set_overlay_touch_event(&act)?;

    // FrameLayout
    let data = act.gen_create().unwrap().set_parent(-1);
    let layout = tgui.new_layout_frame(data)?;

    // SurfaceView
    let data = act.gen_create().unwrap().set_parent(layout.id()?);
    let surface_view = tgui.new_surface(data, true, true)?;
    let asv = act.gen_view(&surface_view).unwrap();

    // Image
    let imgty = ImgTy::open_rgba8888("test.jpg")?;
    let hw1 = HwBuffer::new(&imgty)?;
    let hw2 = HwBuffer::new(&imgty)?;
    let mut hw1 = tgui.new_hwbuffer(&hw1)?;
    let mut hw2 = tgui.new_hwbuffer(&hw2)?;

    // loop {}
    let hw_buffer = {
        if true {
            &hw1
        } else {
            &hw2
        }
    };
    // FIXME: EGL
    let res = surface_view.set_buffer(&tgui, &asv, Some(hw_buffer))?;
    dbg!(res);

    //    unsafe {
    //        buffer_res.mmap()?;
    //    }
    //
    //    let buf = imgty.as_slice()?;
    //    buffer_res.mmap_flush(buf)?;
    //    buffer_res.blit(&tgui)?;

    sleep_ms(3000);

    Ok(())
}
