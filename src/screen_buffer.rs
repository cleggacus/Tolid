#[derive(Clone, Copy)]
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
}
