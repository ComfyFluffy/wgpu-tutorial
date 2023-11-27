use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

mod state;

fn handle_key_event(state: &mut State, event: KeyEvent) {
    if event.state != ElementState::Pressed {
        return;
    }
    match event.logical_key {
        Key::Named(NamedKey::Space) => {}
        Key::Character(c) => match c.to_ascii_lowercase().as_str() {
            "c" => state
                .window()
                .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
            "f" => state.window().set_fullscreen(None),
            _ => {}
        },
        _ => {}
    }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    handle_key_event(&mut state, event);
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                _ => {}
            },
            Event::AboutToWait => {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
                    Err(e) => eprintln!("{:?}", e),
                }
                state.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
