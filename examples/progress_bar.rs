// layout:
//
// <FrameLayout>
//   <LinearLayout>
//     <Text />
//     <Space />
//     <Text />
//   </LinearLayout>
//
//   <LinearLayout>
//     <ProgressBar />
//   </LinearLayout>
// </FrameLayout>

use std::thread::sleep_ms;
use tgui::{items::Visibility, Vi, View, ViewSet, *};

fn main() -> Res<()> {
    let task = Task::new()?.conn()?;
    let act = task.new_activity(-1)?;

    // FrameLayout
    let layout_frame = act.new_top_layout_frame()?;

    // LinearLayout
    let layout_linear = act.new_layout_linear(&layout_frame, true)?;

    // TextView
    let left_text_view = act.new_text(&layout_linear, "00:00:00".to_string())?;

    // SpaceView
    let space = act.new_space(&layout_linear)?;

    // TextView
    let right_text_view = act.new_text(&layout_linear, "10:00:00".to_string())?;

    // LinearLayout
    let layout_linear2 = act.new_layout_linear(&layout_frame, true)?;

    // ProgressBar
    let pb = act.new_progress_bar(&layout_linear2)?;

    for n in 0..100 {
        pb.set(n)?;

        if n % 2 == 0 {
            pb.vi_visible(Visibility::Visible)?;
        } else {
            pb.vi_visible(Visibility::Gone)?;
        }
        left_text_view.update(format!("00:00:{n}"))?;

        sleep_ms(100);
    }

    act.close();

    Ok(())
}
