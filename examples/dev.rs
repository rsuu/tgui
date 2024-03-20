use std::thread::sleep_ms;
use tgui::*;

fn main() -> Res<()> {
    let _args = std::env::args().collect::<Vec<String>>();

    // success
    let tgui = Tgui::new()?.conn()?;

    let act = Activity::new();
    let r_act = tgui.new_activity(act)?;
    dbg!(&r_act);

    let aid = r_act.id()?;

    // display img
    {
        // high-level:
        // let req = ImgReq::new()
        //   .set_xxx()
        //   .set_yyy();
        //
        // tgui.exec(req);
        //
        // ?loop

        let data = items::Create::new().set_aid(aid).set_v(100).set_parent(-1);
        let img = Img::new().set_data(data);
        let res = tgui.new_img_view(img)?;
        let _ = tgui.img_update(ImgTy::open_jpg("test.jpg")?, Some(aid), Some(res.id))?;
        sleep_ms(3000);
    }

    // display text
    {
        let data = items::Create::new().set_aid(aid).set_parent(-1).set_v(100);
        let text = "hi".to_string();
        let req = view::Text::new().set_data(data).set_text(text);
        let res = tgui.new_text_view(req)?;
        sleep_ms(1000);

        // reset text
        {
            let text = "bye".to_string();
            let _r_text_up = tgui.text_update(Some(items::View { aid, id: res.id }), text)?;
            sleep_ms(1000);
        }
    }

    Ok(())
}
