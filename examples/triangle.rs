use pickle::events::WindowEvent;
use pickle::shapes::Triangle;
use pickle::shapes::{ColorRGB};
use pickle::window::Window;

fn main(){
	println!("b");
	let mut window = Window::new((400, 400), "A window");
    'app: loop {
        for event in window.get_events() {
            match event {
                WindowEvent::Close => break 'app,
                _ => (),
            }
        }
        window.set_background(ColorRGB::new(255, 0, 0));
        let triangle = Triangle::new(
            (350, 350),
            (100, 100),
            (100, 350),
            ColorRGB::new(25, 25, 25),
        );
        window.grid.add_shape(triangle);
        window.display();
    }
}