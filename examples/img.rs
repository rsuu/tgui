// TODO:
//   [ ] A: remove shadow of Button
//       B: ImageButton
//   [ ] LinearLayout

// Layout:
//
// <FrameLayout>
//   <LinearLayout orientation="horizontal">
//     <ImageButton left />
//     <Img center />
//     <ImageButton right />
//   </LinearLayout>
// </FrameLayout>

use std::{collections::HashMap, thread::sleep_ms};
use tgui::{items::Visibility, View, ViewSet, *};

fn main() -> Res<()> {
    let mut view_map = HashMap::<(i32, i32), &'static str>::new();

    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;
    //tgui.config_set_overlay_touch_event(&act)?;

    // GridLayout
    let data = act.gen_create().unwrap().set_parent(-1);
    let l_view = tgui.new_layout_grid(data, 1, 3)?;

    // Left Button
    let data = act.gen_create().unwrap().set_parent(l_view.get_id()?);
    let l_btn_view = tgui.new_button(data, false, "lbtn".to_string())?;
    let l_abv = act.gen_view(&l_btn_view).unwrap();
    //tgui.view_set_clickable(l_abv.clone(), false)?;
    tgui.view_set_w(l_abv.clone(), 160.0, Unit::Dp)?;
    tgui.view_set_const_size(l_abv.clone(), Constant::MatchParent)?;
    tgui.set_layout_grid(l_abv.clone(), 1, 1, 1, 1, Alignment::Left, Alignment::Left)?;
    //tgui.view_set_padding(l_abv.clone(), None, Direction::Right)?;
    view_map.insert((l_abv.aid, l_abv.id), "lbtn");

    // Right Button
    let data = act.gen_create().unwrap().set_parent(l_view.get_id()?);
    //.set_v(Visibility::Hidden);
    let r_btn_view = tgui.new_button(data, false, "rbtn".to_string())?;
    let r_abv = act.gen_view(&r_btn_view).unwrap();
    tgui.view_set_clickable(r_abv.clone(), false)?;
    tgui.view_set_click_event(r_abv.clone(), true)?;
    tgui.view_set_bg(r_abv.clone(), 0x00000000)?;
    tgui.view_set_fg(r_abv.clone(), 0x00000000)?;
    tgui.view_set_w(r_abv.clone(), 160.0, Unit::Dp)?;
    tgui.view_set_const_size(r_abv.clone(), Constant::MatchParent)?;
    tgui.set_layout_grid(
        r_abv.clone(),
        1,
        3,
        1,
        1,
        Alignment::Center,
        Alignment::Right,
    )?;
    //tgui.view_set_padding(r_abv.clone(), None, Direction::Left)?;
    view_map.insert((r_abv.aid, r_abv.id), "rbtn");

    // Image Buffer
    let data = act.gen_create().unwrap().set_parent(l_view.get_id()?);
    let img_view = Img::new().set_data(data).conn(&tgui)?;
    let ity = ImgTy::open_jpg("test.jpg").unwrap();
    let aiv = act.gen_view(&img_view).unwrap();
    tgui.view_set_touch_event(aiv.clone(), true)?;
    tgui.view_set_const_size(aiv.clone(), Constant::MatchParent)?;
    tgui.set_layout_grid(
        aiv.clone(),
        1,
        1,
        1,
        3,
        Alignment::Center,
        Alignment::Center,
    )?;
    view_map.insert((aiv.aid, aiv.id), "img");

    'l1: loop {
        let Some(e) = tgui.get_event()? else {
            continue;
        };

        match e {
            // REFS: https://stackoverflow.com/questions/5283293/ontouch-works-but-onclicklistener-doesnt
            //items::event::Event::Touch(items::TouchEvent { v, time, .. }) => 's: {
            items::event::Event::Click(items::ClickEvent { v, .. }) => 's: {
                let Some(v) = v else {
                    break 's;
                };

                let Some(v) = view_map.get(&(v.aid, v.id)) else {
                    break 's;
                };

                if v == &"rbtn" {
                    println!("rbtn");

                    return Ok(());
                }
            }

            items::event::Event::OverlayScale(items::OverlayScaleEvent { aid, span, .. }) => {
                dbg!(aid, span);
            }

            items::event::Event::Back(..) => {
                dbg!("Back");
                break 'l1;
            }

            items::event::Event::Start(items::StartEvent { .. }) => {
                dbg!("start");
                tgui.event_intercept_volume(&act, true, true)?;
                tgui.event_intercept_back(&act)?;
            }

            _ => {}
        }

        img_view.update(&tgui, &ity, aiv.clone())?;
        sleep_ms(100);
    }

    Ok(())
}

// REFS: https://learn.microsoft.com/en-us/xamarin/xamarin-forms/user-interface/controls/layouts
// REFS: https://github.com/anilbeesetti/nextplayer/blob/8e2adee003d4c1772be6d05bb9f3838bc13e5efe/feature/player/src/main/res/layout/exo_player_control_view.xml#L234
