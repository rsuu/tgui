use std::thread::sleep_ms;
use tgui::{View, ViewSet, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;

    // TextView
    let data = act.gen_create().unwrap().set_parent(-1);
    let text_view = Text::new()
        .set_data(data)
        .set_text("hi".to_string())
        .conn(&tgui)?;

    let mut event = Event::new(&act, text_view.res())?;
    let view = act.gen_view(text_view.res());

    'l1: loop {
        event.get_event(&tgui)?;

        let Some(e) = event.res() else {
            continue;
        };

        match e {
            items::event::Event::Start(items::StartEvent { .. }) => {
                dbg!("start");

                event.set_intercept(InterceptTy::VolumeUp, true);
                event.set_intercept(InterceptTy::VolumeDown, true);
                let res = event.intercept_volume(&tgui, &act)?;
                dbg!(res);

                event.set_intercept(InterceptTy::BackButton, true);
                let res = event.intercept_back(&tgui, &act)?;
                dbg!(res);
            }

            items::event::Event::Volume(items::VolumeKeyEvent { key, .. }) => {
                dbg!(items::volume_key_event::VolumeKey::from_i32(*key));
            }

            items::event::Event::Touch(items::TouchEvent {
                touches,
                v,
                action,
                index,
                ..
            }) => {
                dbg!(touches, v, action, index);
            }

            items::event::Event::Back(items::BackButtonEvent { .. }) => {
                dbg!("Back");
                break 'l1;
            }

            _ => {}
        }
    }

    Ok(())
}
