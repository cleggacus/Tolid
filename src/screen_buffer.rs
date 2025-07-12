use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Blank,
    Char(char)
}

pub struct ScreenBuffer {
    buffer: Vec<Cell>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            width: 0,
            height: 0
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.buffer.resize(width*height, Cell::Blank);
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Cell::Blank);
    }

    pub fn get_by_index(&self, i: usize) -> Option<Cell> {
        self.buffer.get(i).copied()
    }

    pub fn set_by_index(&mut self, i: usize, cell: Cell) {
        if i < self.buffer.len() {
            self.buffer[i] = cell;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = cell;
        }
    }
}
