use eqx_app::prelude::Module;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub use winit::window::Window as WInitWindow;

pub struct Window {}

impl Default for Window {
    fn default() -> Self {
        Window {}
    }
}

impl Module for Window {
    fn setup(&mut self) {}

    fn init(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop
            .run(move |event, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    _ => {}
                },
                _ => {}
            })
            .unwrap();
    }
}
