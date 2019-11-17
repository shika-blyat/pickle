use luminance_glutin::{Event, GlutinSurface, WindowEvent as GlutinEvent};
use luminance_windowing::Surface;

pub enum WindowEvent {
    KeyPressed,
    MouseButton,
    MouseMovement,
    Close,
}

pub struct Events;

impl Events {
    pub fn new() -> Events {
        Events
    }
    pub fn get_events(&self, surface: &mut GlutinSurface) -> Vec<WindowEvent> {
        let mut event_list = vec![];
        for event in surface.poll_events() {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    // If we close the window or press escape, quit the main loop (i.e. quit the application).
                    GlutinEvent::CloseRequested | GlutinEvent::Destroyed => {
                        event_list.push(WindowEvent::Close)
                    }
                    _ => (),
                }
            }
        }
        event_list
    }
}
