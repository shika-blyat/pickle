use crate::shapes::Shapes;
use std::collections::{vec_deque, VecDeque};

#[allow(unused)]
pub struct Grid {
    width: u32,
    height: u32,
    shape_queue: VecDeque<Shapes>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let shape_queue = VecDeque::new();
        Grid {
            width,
            height,
            shape_queue,
        }
    }
    pub fn add_shape(&mut self, shape: Shapes) {
        self.shape_queue.insert(0, shape);
    }
    pub fn get_queue(&self) -> vec_deque::Iter<'_, Shapes> {
        self.shape_queue.iter()
    }
    pub fn clear_queue(&mut self) {
        self.shape_queue.clear();
    }
}
