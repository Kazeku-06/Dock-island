use anyhow::Result;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::config::Config;
use crate::ui::AppState;

pub struct TopDockApp {
    window: Option<Window>,
    state: Option<AppState>,
    config: Config,
}

impl TopDockApp {
    fn new(config: Config) -> Self {
        Self {
            window: None,
            state: None,
            config,
        }
    }
}

impl ApplicationHandler for TopDockApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            tracing::info!("Creating window...");
            
            let window_attrs = Window::default_attributes()
                .with_title("TopDock")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    self.config.compact_width,
                    self.config.compact_height,
                ))
                .with_decorations(false)
                .with_transparent(true)
                .with_resizable(false);

            match event_loop.create_window(window_attrs) {
                Ok(window) => {
                    // Position window at top of screen
                    if let Some(monitor) = window.current_monitor() {
                        let screen_size = monitor.size();
                        let window_size = window.outer_size();
                        
                        let x = (screen_size.width - window_size.width) / 2;
                        let y = self.config.top_margin;
                        
                        window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
                    }

                    tracing::info!("Window created successfully");
                    self.window = Some(window);
                    // State initialization will happen here once we implement rendering
                }
                Err(e) => {
                    tracing::error!("Failed to create window: {}", e);
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Rendering will be implemented in Phase 3
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // Handle keyboard shortcuts
                if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) {
                    tracing::info!("Escape pressed, exiting");
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

pub fn run() -> Result<()> {
    let config = Config::load()?;
    
    tracing::info!("Initializing event loop...");
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = TopDockApp::new(config);
    
    tracing::info!("Starting event loop...");
    event_loop.run_app(&mut app)?;

    Ok(())
}
