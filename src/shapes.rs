use luminance_derive::{Semantics, Vertex};
const SLOPE_TO_LUMCOLOR: f32 = 1. / 256.0;

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}
#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
#[allow(unused)]
pub struct Vertex {
    pub position: VertexPosition,
    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

impl Vertex {
    fn translate(_win_size: u32, point: u32, init_size: u32) -> f32 {
        // translate from window size to (-1;1) coordinate
        (-1.0 + 2.0 / init_size as f32 * point as f32)
    }
    pub fn from_point(point: (u32, u32), color: ColorRGB, win_size: [u32; 2], init_size: [u32;2]) -> Vertex {

        let width = Vertex::translate(win_size[1], point.1, init_size[1]);
        let height = -(Vertex::translate(win_size[0], point.0 , init_size[0]));
        Vertex {
            position: VertexPosition::new([width, height]),
            color: VertexRGB::new(color.0),
        }
    }
    /*pub fn from_point(
        point: (u32, u32),
        color: ColorRGB,
        win_size: [u32; 2],
        _init_size: [u32; 2],
    ) -> Vertex {
        let fpixelw: f32 = 1.0 / win_size[1] as f32;
        let fpixelh: f32 = 1.0 / win_size[0] as f32;
        let width = -1.0 + (fpixelw * point.1 as f32);
        let height = 1.0 - (fpixelh * point.0 as f32);
        Vertex {
            position: VertexPosition::new([width, height]),
            color: VertexRGB::new(color.0),
        }
    }*/
}

pub enum Shapes {
    Triangle {
        points: ((u32, u32), (u32, u32), (u32, u32)),
        color: ColorRGB,
    },
    Rectangle {
        points: ((u32, u32), (u32, u32), (u32, u32), (u32, u32)),
        color: ColorRGB,
    },
    Line {
        points: ((u32, u32), (u32, u32)),
        color: ColorRGB,
    },
}
impl Shape for Shapes {
    fn get_color(&self) -> ColorRGB {
        match self {
            Shapes::Line { points: _, color } => *color,
            Shapes::Rectangle { points: _, color } => *color,
            Shapes::Triangle { points: _, color } => *color,
        }
    }
    fn get_points(&self) -> Vec<(u32, u32)> {
        match self {
            Shapes::Line { points, color: _ } => vec![(*points).0, (*points).1],
            Shapes::Rectangle { points, color: _ } => {
                vec![(*points).0, (*points).1, (*points).2, (*points).3]
            }
            Shapes::Triangle { points, color: _ } => vec![(*points).0, (*points).1, (*points).2],
        }
    }
    fn get_vertex(&self, size: [u32; 2], init_size: [u32; 2]) -> Vec<Vertex> {
        match self {
            Shapes::Line {
                points: _,
                color: _,
            } => vec![
                Vertex::from_point(
                    self.get_points()[0],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[1],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
            ],
            Shapes::Rectangle {
                points: _,
                color: _,
            } => vec![
                Vertex::from_point(
                    self.get_points()[0],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[1],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[2],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[3],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
            ],
            Shapes::Triangle {
                points: _,
                color: _,
            } => vec![
                Vertex::from_point(
                    self.get_points()[0],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[1],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
                Vertex::from_point(
                    self.get_points()[2],
                    self.get_color(),
                    size,
                    [init_size[0], init_size[1]],
                ),
            ],
        }
    }
}
pub trait Shape {
    fn get_color(&self) -> ColorRGB;
    fn get_points(&self) -> Vec<(u32, u32)>;
    fn get_vertex(&self, size: [u32; 2], init_size: [u32; 2]) -> Vec<Vertex>;
}
#[derive(Debug, Copy, Clone)]
pub struct ColorRGB([u8; 3]);

trait Color {}
impl Color for ColorRGB {}
impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> ColorRGB {
        ColorRGB([r, g, b])
    }
    pub fn to_f32(&self) -> [f32; 4] {
        [
            SLOPE_TO_LUMCOLOR * self.0[0] as f32,
            SLOPE_TO_LUMCOLOR * self.0[1] as f32,
            SLOPE_TO_LUMCOLOR * self.0[2] as f32,
            1.0,
        ]
    }
}
