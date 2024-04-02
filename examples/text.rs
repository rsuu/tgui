// layout:
//
// <LinearLayout>
//   <Text />
// </LinearLayout>

use std::thread::sleep_ms;
use tgui::*;

fn main() -> Res<()> {
    let task = Task::new()?.conn()?;
    let act = task.new_activity(-1)?;

    let layout_linear = act.new_top_layout_linear(true)?;

    let text = act.new_text(&layout_linear, "hi".to_string())?;
    // // longer version
    // let text = Text::new(&act)
    //     .set_data(act.gen_create().unwrap().set_parent(layout_linear.id()?))
    //     .set_selectable_text(true)
    //     .set_clickable_links(true)
    //     .set_text("hi".to_string())
    //     .conn()?;
    sleep_ms(1000);

    text.update("bye".to_string())?;
    sleep_ms(1000);

    act.close();

    Ok(())
}
