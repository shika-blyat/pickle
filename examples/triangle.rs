use pickle::events::WindowEvent;
use pickle::shapes::ColorRGB;
use pickle::shapes::Shapes;
use pickle::window::Window;
use std::time::Instant;

fn main() {
    let mut window = Window::new((400, 400), "A window");
    let start_t = Instant::now();
    'app: loop {
        for event in window.get_events() {
            match event {
                WindowEvent::Close => break 'app,
                WindowEvent::Resize => eprintln!("Resize !"),
                _ => (),
            }
        }
        window.set_background(ColorRGB::new(125, 25, 212));
        let _t = start_t.elapsed().as_secs();
        let rectangle = Shapes::Rectangle {
            points: ((100, 200), (200, 200), (200, 100), (100, 100)),
            color: ColorRGB::new(25, 25, 25),
        };
        let line = Shapes::Line {
            points: ((100, 350), (350, 350)),
            color: ColorRGB::new(255, 0, 145),
        };
        let triangle = Shapes::Triangle {
            points: ((100, 300), (200, 300), (200, 200)),
            color: ColorRGB::new(255, 85, 145),
        };
        let circle = Shapes::Circle {
            points: (300, 50),
            radius: 50,
            color: ColorRGB::new(255, 85, 145),
        };
        window.grid.add_shape(line);
        window.grid.add_shape(rectangle);
        window.grid.add_shape(triangle);
        window.grid.add_shape(circle);
        window.display();
    }
}
