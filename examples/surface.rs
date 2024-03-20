    // Surface
    let data = act.gen_create().unwrap().set_parent(-1).set_v(0);
    let surface_view = tgui.new_surface(Some(data), false, true)?;
    let mut event = Event::new(&act, surface_view.res())?;

    dbg!(&surface_view);
