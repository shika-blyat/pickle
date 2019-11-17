use pickle::events::WindowEvent;
use pickle::shapes::Triangle;
use pickle::shapes::{ColorRGB};
use pickle::window::Window;
use std::time::Instant;

fn main(){
	let mut window = Window::new((400, 400), "A window");
    let start_t = Instant::now();
    'app: loop {
        for event in window.get_events() {
            match event {
                WindowEvent::Close => break 'app,
                _ => (),
            }
        }
        window.set_background(ColorRGB::new(125, 25, 212));
        let t = start_t.elapsed().as_secs();
        if t < 2 {
            let triangle = Triangle::new(
                (350, 350),
                (100, 100),
                (100, 350),
                ColorRGB::new(25, 25, 25),
            );
            window.grid.add_shape(triangle);
        }
        window.display();
    }
}