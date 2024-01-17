// WIP

use std::thread::sleep_ms;
use tgui::*;

fn main() -> Res<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let tgui = Tgui::new()?.conn()?;

    // Create a remote layout
    let res = tgui.new_r_layout()?;

    let wid = args[1].clone();
    let req = RLayout { wid, parent: -1 };

    // Add a TextView
    let atv = tgui.r_layout_add_text_view(&req, &res)?;
    // Add a ImageView
    let aiv = tgui.r_layout_add_img_view(&req, &res)?;

    // 0 = gone, 1 = hidden, 2 = visible
    let vis = 2;
    tgui.r_layout_vis(&res, atv.id, vis)?;
    tgui.r_layout_vis(&res, aiv.id, vis)?;

    let text = "abcde
fghjk
waefwiowa
weofniof
waewefawef
weofniof
wefnwaf";

    let ity = ImgTy::open_rgba8888("test.jpg")?;

    loop {
        {
            // Set the text
            let st = tgui.r_layout_set_text(&res, atv.id, text.to_string())?;
            dbg!(&st);

            let stc = tgui.r_layout_set_text_color(&res, atv.id, 0xff1234)?;
            dbg!(&stc);

            let sb = tgui.r_layout_set_bg(&req, &res, atv.id, 0xffff_ffff)?;
            dbg!(&atv);
        }

        // Set the image
        {
            let st = tgui.r_layout_set_img(&res, aiv.id, &ity)?;
        }

        // update the widget
        let f = tgui.r_layout_flush(&req, &res)?;
        dbg!(&f);

        sleep_ms(1000);
    }
}
