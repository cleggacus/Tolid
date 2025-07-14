use crossterm::style::{ContentStyle, StyledContent};

pub type BufferCell = StyledContent<char>;
const EMPTY_CHAR: char = ' ';

pub fn empty_cell() -> BufferCell {
    ContentStyle::new().apply(EMPTY_CHAR)
}

pub struct ScreenBuffer {
    buffer: Vec<BufferCell>,
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
        self.buffer.resize(width*height, empty_cell());
        self.width = width;
        self.height = height;
    }

    pub fn clear(&mut self) {
        self.buffer.fill(empty_cell());
    }

    pub fn get_by_index(&self, i: usize) -> Option<&BufferCell> {
        self.buffer.get(i)
    }

    pub fn set_by_index(&mut self, i: usize, cell: BufferCell) {
        if i < self.buffer.len() {
            self.buffer[i] = cell;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&BufferCell> {
        if x < self.width && y < self.height {
            self.buffer.get(y * self.width + x)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: BufferCell) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = cell;
        }
    }

    pub fn row(&self, y: usize) -> Option<&[BufferCell]> {
        if y < self.height {
            let start = y * self.width;
            let end = start + self.width;
            Some(&self.buffer[start..end])
        } else {
            None
        }
    }
}
