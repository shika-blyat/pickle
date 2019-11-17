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
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

impl Vertex {
    fn translate(win_size: u32, point: u32) -> f32 {
        -1.0 + 2.0 / win_size as f32 * point as f32
    }
    pub fn from_point(point: (u32, u32), color: ColorRGB, win_size: (u32, u32)) -> Vertex {
        // translate from window size to (-1;1) coordinate
        let width = Vertex::translate(win_size.1, point.1);
        let height = -(Vertex::translate(win_size.0, point.0));
        Vertex {
            position: VertexPosition::new([width, height]),
            color: VertexRGB::new(color.0),
        }
    }
}
pub trait Shape {
    fn get_color(&self) -> ColorRGB;
    fn get_points(&self) -> Vec<(u32, u32)>;
}

pub struct Triangle {
    points: ((u32, u32), (u32, u32), (u32, u32)),
    color: ColorRGB,
}
impl Shape for Triangle {
    fn get_color(&self) -> ColorRGB {
        self.color
    }
    fn get_points(&self) -> Vec<(u32, u32)> {
        [self.points.0,self.points.1,self.points.2].to_vec()
    }
}
impl Triangle {
    pub fn new(p1: (u32, u32), p2: (u32, u32), p3: (u32, u32), color: ColorRGB) -> Triangle {
        Triangle {
            points: (p1, p2, p3),
            color,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct ColorRGB([u8; 3]);

trait Color{}
impl Color for ColorRGB{}
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
