use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style::Print};

use crate::screen_buffer::{Cell, ScreenBuffer};

pub struct Renderer {
    current_buffer: ScreenBuffer,
    previous_buffer: ScreenBuffer,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            current_buffer: ScreenBuffer::new(),
            previous_buffer: ScreenBuffer::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.current_buffer.resize(width, height);
        self.previous_buffer.resize(width, height);
        self.current_buffer.clear();
        self.previous_buffer.clear();
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.current_buffer.set(x, y, cell);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn render_renderable<T: Renderable>(&mut self, renderable: &T) {
        renderable.render(self);
    }

    pub fn render(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for y in 0..self.height {
            for x in 0..self.width {
                let current_cell = self.current_buffer.get(x, y);
                let previous_cell = self.previous_buffer.get(x, y);

                if let (Some(current_cell), Some(previous_cell)) = (current_cell, previous_cell) {
                    if current_cell != previous_cell {
                        Renderer::render_cell(stdout, x, y, current_cell)?;
                    }
                }
            }
        }

        stdout.flush()?;

        Ok(())
    }

    fn render_cell(stdout: &mut Stdout, x: usize, y: usize, cell: Cell) -> Result<(), Box<dyn std::error::Error>> {
        queue!(stdout, MoveTo(x as u16, y as u16))?;

        match cell {
            Cell::Char(char) => queue!(stdout, Print(char)),
            Cell::Blank => queue!(stdout, Print(' '))
        }?;

        Ok(())
    }
}

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}
