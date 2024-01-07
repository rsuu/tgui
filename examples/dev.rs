use std::thread::sleep_ms;

use tgui::{
    items,
    view::{self, img::ImgTy, Img},
    Activity, Res, Tgui, View, ViewSet,
};

fn main() -> Res<()> {
    let _args = std::env::args().collect::<Vec<String>>();

    // success
    let tgui = Tgui::new()?.conn()?;

    let act = Activity::new();
    let r_act = tgui.new_activity(act)?;
    dbg!(&r_act);

    let aid = r_act.get_aid()?;

    // display img
    {
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

    // // create Layout and TextView and Widget
    // {
    //     unimplemented!();
    //     let layout = tgui.new_remote_layout()?;
    //     let rid = layout.rid;
    //
    //     // create RemoteTextView
    //     let req = view::RemoteText::new()
    //         .set_text("remote text".to_string())
    //         .set_parent(-1);
    //     let rtext_res = tgui.new_remote_text_view(&req)?;
    //
    //     let wid = args[1].clone();
    //     let req = view::Widget::new().set_wid(wid).set_rid(rid);
    //     let _ = tgui.new_widget_view(&req)?;
    //
    //     let _ = tgui.remote_text_update("bye".to_string(), rid, rtext_res.id)?;
    // }

    Ok(())
}
