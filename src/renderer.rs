use crate::screen_buffer::ScreenBuffer;

pub struct Renderer {
    current_buffer: ScreenBuffer,
    previous_buffer: ScreenBuffer,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            current_buffer: ScreenBuffer::new(),
            previous_buffer: ScreenBuffer::new(),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.current_buffer.resize(width, height);
    }
}
