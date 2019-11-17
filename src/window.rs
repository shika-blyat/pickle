use crate::events::{Events, WindowEvent};
use crate::grid::Grid;
use crate::shapes::ColorRGB;
use crate::shapes::Shape;
use crate::shapes::{Vertex, VertexSemantics};
use luminance::pipeline::TessGate;
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder, Tess, TessSliceIndex as _};
use luminance::texture::Dim2;
use luminance::texture::Flat;
use luminance_glutin::{GlutinSurface, Surface, WindowDim, WindowOpt};

const VS_STR: &str = include_str!("glsl/vs.glsl");
const FS_STR: &str = include_str!("glsl/fs.glsl");

#[allow(unused)]
pub struct Window<T: Shape> {
    width: u32,
    height: u32,
    events: Events,
    surface: GlutinSurface,
    back_buffer: Framebuffer<Flat, Dim2, (), ()>,
    background_color: ColorRGB,
    program: Program<VertexSemantics, (), ()>,
    pub grid: Grid<T>,
}

impl<T: Shape> Window<T> {
    pub fn new<'a>(size: (u32, u32), name: &'a str) -> Window<T> {
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
            width: size.0,
            height: size.1,
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
        let program = &self.program;
        let size = self.get_size();
        let mut tess: Vec<Tess> = vec![];
        for i in self.grid.get_queue() {
            tess.push(
                TessBuilder::new(&mut self.surface)
                    .add_vertices([
                        Vertex::from_point(i.get_points()[0], i.get_color(), size),
                        Vertex::from_point(i.get_points()[1], i.get_color(), size),
                        Vertex::from_point(i.get_points()[2],  i.get_color(), size),
                    ])
                    .set_mode(Mode::Triangle)
                    .build()
                    .unwrap(),
            )
        }
        self.surface.pipeline_builder().pipeline(
            &self.back_buffer,
            self.background_color.to_f32(),
            |_, mut shd_gate| {
                shd_gate.shade(&program, |_, mut rdr_gate| {
                    rdr_gate.render(RenderState::default(), |tess_gate| {
                        Window::<T>::render_all(tess, tess_gate);
                    });
                });
            },
        );
        self.grid.clear_queue();
        self.surface.swap_buffers();
    }
    fn render_all(shapes: Vec<Tess>, mut tess_gate: TessGate<'_, luminance_glutin::GlutinSurface>){
        for i in shapes{
            tess_gate.render(i.slice(..));
        }
    }
    pub fn get_size(&self) -> [u32; 2] {
        self.surface.size()
    }
}
