mod buffer;
mod camera;
mod render;
mod state;
mod texture;

use state::*;

use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow::Poll, EventLoop},
    keyboard::{PhysicalKey, KeyCode},
    window::{Window, WindowId},
};

pub fn run(config: Config) -> Result<(), EventLoopError> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(Poll);
    let mut app = App::new(config);

    log::info!("Running app");
    event_loop.run_app(&mut app)
}

pub struct Config {
    pub initial_window_size: PhysicalSize<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            initial_window_size: PhysicalSize { width: 1600, height: 1200 },
        }
    }
}

struct App {
    state: Option<State>,
    config: Config,
    start_instant: Instant,
}

impl App {
    fn new(config: Config) -> Self {
        Self {
            state: None,
            config,
            start_instant: Instant::now(),
        }
    }

    fn uptime(&self) -> Duration {
        self.start_instant.elapsed()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Resumed");
        let window_attributes = Window::default_attributes()
            .with_title("Hello Land!")
            .with_inner_size(self.config.initial_window_size);
        let window = event_loop.create_window(window_attributes).unwrap();
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        if window_id != state.window().id() { return; }
        if state.input(&event) { return; }

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close Requested. Exiting now...");
                event_loop.exit();
            },
            WindowEvent::Resized(new_size) => {
                log::info!("Resized to {:?}", new_size);
                state.resize(new_size);
            },
            WindowEvent::RedrawRequested => {
                state.window().request_redraw();
                state.update();
                state.render();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                match keycode {
                    KeyCode::Escape => {
                        log::info!("Close requested by escape key. Exiting now...");
                        event_loop.exit();
                    },
                    KeyCode::KeyT => {
                        log::info!("Time since startup: {:?}", self.uptime());
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
