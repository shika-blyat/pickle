use crate::events::{Events, WindowEvent};
use crate::grid::Grid;
use crate::shapes::ColorRGB;
use crate::shapes::{Shape, Shapes, VertexSemantics};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::TessGate;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder, TessSliceIndex as _};
use luminance::texture::Dim2;
use luminance::texture::Flat;
use luminance_glutin::{GlutinSurface, Surface, WindowDim, WindowOpt};

const VS_STR: &str = include_str!("glsl/vs.glsl");
const FS_STR: &str = include_str!("glsl/fs.glsl");

#[allow(unused)]
pub struct Window {
    init_width: u32,
    init_height: u32,
    events: Events,
    surface: GlutinSurface,
    back_buffer: Framebuffer<Flat, Dim2, (), ()>,
    background_color: ColorRGB,
    program: Program<VertexSemantics, (), ()>,
    pub grid: Grid,
}

impl Window {
    pub fn new<'a>(size: (u32, u32), name: &'a str) -> Window {
        let surface = GlutinSurface::new(
            WindowDim::Windowed(size.0, size.1),
            name,
            WindowOpt::default(),
        );
        if let Err(e) = surface {
            panic!("An error occured while creating the surface:\n {:?}", e);
        }
        let mut surface = surface.unwrap();
        let events = Events::new();
        let grid = Grid::new(size.0, size.1);
        let back_buffer = surface.back_buffer().unwrap();
        let background_color = ColorRGB::new(0, 0, 0);
        let program: Program<VertexSemantics, (), ()> =
            Program::from_strings(None, VS_STR, None, FS_STR)
                .unwrap()
                .ignore_warnings();
        Window {
            init_width: size.0,
            init_height: size.1,
            events,
            surface,
            grid,
            back_buffer,
            background_color,
            program,
        }
    }
    pub fn get_events(&mut self) -> Vec<WindowEvent> {
        self.events.get_events(&mut self.surface)
    }
    pub fn set_background(&mut self, color: ColorRGB) {
        self.background_color = color;
    }
    pub fn display(&mut self) {
        self.back_buffer = self.surface.back_buffer().unwrap();
        let mut tess: Vec<Tess> = vec![];
        let size = self.get_size();
        for shape in self.grid.get_queue() {
            match shape {
                Shapes::Triangle {
                    points: _,
                    color: _,
                } => tess.push(
                    TessBuilder::new(&mut self.surface)
                        .add_vertices(shape.get_vertex(size, [self.init_width, self.init_height]))
                        .set_mode(Mode::Triangle)
                        .build()
                        .unwrap(),
                ),
                Shapes::Rectangle {
                    points: _,
                    color: _,
                } => tess.push(
                    TessBuilder::new(&mut self.surface)
                        .set_vertex_nb(4)
                        .add_vertices(shape.get_vertex(size, [self.init_width, self.init_height]))
                        .set_mode(Mode::TriangleFan)
                        .build()
                        .unwrap(),
                ),
                Shapes::Line {
                    points: _,
                    color: _,
                } => tess.push(
                    TessBuilder::new(&mut self.surface)
                        .add_vertices(shape.get_vertex(size, [self.init_width, self.init_height]))
                        .set_mode(Mode::Line)
                        .build()
                        .unwrap(),
                ),
            }
        }
        let program = &self.program;
        self.surface.pipeline_builder().pipeline(
            &self.back_buffer,
            self.background_color.to_f32(),
            |_, mut shd_gate|{
                shd_gate.shade(&program, |_, mut rdr_gate| {
                    rdr_gate.render(RenderState::default(), |tess_gate| {
                        Window::render_all(tess, tess_gate);
                    });
                });
            },
        );
        self.grid.clear_queue();
        self.surface.swap_buffers();
    }
    fn render_all(shapes: Vec<Tess>, mut tess_gate: TessGate<'_, luminance_glutin::GlutinSurface>) {
        for i in shapes {
            tess_gate.render(i.slice(..));
        }
    }
    pub fn get_size(&self) -> [u32; 2] {
        self.surface.size()
    }
}
