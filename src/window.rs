use winit::{dpi, event_loop, window};
use winit::event::Event;

use crate::timer;

pub fn run() {
    let event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Androxeda")
        .with_inner_size(dpi::LogicalSize::new(800, 600))
        // .with_decorations(false)
        .build(&event_loop)
        .expect("Window couldn't be built.");

    // TODO: splash screen

    let mut timer = timer::Timer::new(144.0, 144.0);
    // TODO: do extra stuff, then redraw timer

    event_loop.run(move |event, _, control_flow| {
        if event == Event::MainEventsCleared {
            timer.time_check();

            if timer.render_tick() {
                // TODO: game state update
            }

            if timer.draw_tick() {
                window.request_redraw();
            }
        }

        if event == Event::RedrawRequested(window.id()) {
            // TODO: render update
        }
    });
}