use std::time::Instant;

use log::{info, warn};
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

    let mut last_frame = Instant::now();
    let mut skipped_frames = 0;
    let mut about_to_wait_count = 0;

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
                    // handle_key_event(&mut state, event);
                }
                WindowEvent::Resized(physical_size) => {
                    info!("Resized to {:?}", physical_size);
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    if last_frame.elapsed().as_millis() > 1000 / 300 {
                        last_frame = Instant::now();
                        let render_start = Instant::now();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                warn!("Lost surface");
                                state.resize(state.size)
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory"),
                            Err(e) => eprintln!("{:?}", e),
                        }
                        println!(
                            "Rendered in {}ms",
                            render_start.elapsed().as_micros() as f32 / 1000.0
                        );
                    } else {
                        skipped_frames += 1;
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                // print!(
                //     "\rAbout to wait for {} events, skipped frames: {}",
                //     about_to_wait_count, skipped_frames
                // );
                // if last_frame.elapsed().as_millis() > 1000 / 300 {
                state.window().request_redraw();
                //     last_frame = Instant::now();
                // } else {
                //     skipped_frames += 1;
                // }
            }
            _ => {
                // println!("{:?}", event);
            }
        })
        .unwrap();
}
