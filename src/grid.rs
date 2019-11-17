use crate::shapes::Shape;
use std::collections::VecDeque;

pub struct Grid<T: Shape> {
    width: u32,
    height: u32,
    pub shape_queue: VecDeque<T>,
}

impl<T: Shape> Grid<T> {
    pub fn new(width: u32, height: u32) -> Grid<T> {
        let shape_queue = VecDeque::new();
        Grid {
            width,
            height,
            shape_queue,
        }
    }
    pub fn add_shape(&mut self, shape: T) {
        self.shape_queue.insert(0, shape);
    }
}
