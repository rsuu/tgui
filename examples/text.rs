use std::thread::sleep_ms;
use tgui::{items, view::text::Text, View, ViewSet, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;

    // create
    let data = act.gen_create().unwrap().set_parent(-1);
    let text = Text::new()
        .set_data(data)
        .set_text("hi".to_string())
        .conn(&tgui)?;
    sleep_ms(1000);

    // update
    text.update(
        &tgui,
        "bye".to_string(),
        Some(
            items::View::new()
                .set_aid(act.get_id()?)
                .set_id(text.get_id()?),
        ),
    )?;
    sleep_ms(1000);

    Ok(())
}
