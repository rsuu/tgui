use std::{io::Cursor, thread::sleep_ms};

use tgui::{
    activity::Activity,
    connection::Tgui,
    items,
    view::{self, Img},
};

fn main() {
    // success
    let tgui = Tgui::new().conn();

    let act = Activity::new();
    let r_act = tgui.new_activity(act).unwrap();
    dbg!(&r_act);

    let img = view::Img::new()
        .default_data()
        .set_aid(r_act.aid)
        .set_parent(-1);
    let r_img = tgui.new_img_view(img).unwrap();
    let img_path = "test.jpg";
    let base_img = image::open(img_path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    base_img
        .write_to(
            &mut Cursor::new(&mut buf),
            image::ImageOutputFormat::Jpeg(90),
        )
        .unwrap();
    let r_img_up = tgui
        .img_update(
            Some(items::View {
                aid: r_act.aid,
                id: r_img.id,
            }),
            buf,
        )
        .unwrap();
    sleep_ms(3000);

    let text = "hi".to_string();
    let text = view::Text::new()
        .default_data()
        .set_text(text)
        .set_aid(r_act.aid)
        .set_v(100);
    let r_text = tgui.new_text_view(text).unwrap();
    sleep_ms(1000);

    let text = "bye".to_string();
    let r_text_up = tgui
        .text_update(
            Some(items::View {
                aid: r_act.aid,
                id: r_text.id,
            }),
            text,
        )
        .unwrap();
    sleep_ms(1000);
}
