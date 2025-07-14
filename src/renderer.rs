use std::{io::{Stdout, Write}, ops::Range};

use crossterm::{cursor::MoveTo, queue, style::{Attribute, Attributes, Color, ContentStyle, Print, PrintStyledContent, SetAttribute, SetAttributes, SetBackgroundColor, SetForegroundColor, SetUnderlineColor, StyledContent, Stylize}, QueueableCommand};

use crate::screen_buffer::{BufferCell, ScreenBuffer};

pub struct Renderer {
    current_buffer: ScreenBuffer,
    previous_buffer: ScreenBuffer,
    width: usize,
    height: usize,

    current_foreground_color: Color,
    current_background_color: Color,
    current_underline_color: Color,
    current_attributes: Attributes,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            current_buffer: ScreenBuffer::new(),
            previous_buffer: ScreenBuffer::new(),
            width: 0,
            height: 0,

            current_foreground_color: Color::Reset,
            current_background_color: Color::Reset,
            current_underline_color: Color::Reset,
            current_attributes: Attributes::default(),
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

    pub fn set(&mut self, x: usize, y: usize, cell: BufferCell) {
        self.current_buffer.set(x, y, cell);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {

    }

    pub fn draw_v_line(&mut self, x: usize, y: usize, length: usize) {

    }

    pub fn draw_h_line(&mut self, x: usize, y: usize, length: usize) {

    }

    pub fn render_renderable<T: Renderable>(&mut self, renderable: &T) {
        renderable.render(self);
    }

    pub fn render(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        for y in 0..self.height {
            let mut start: Option<(usize, BufferCell)> = None;

            for x in 0..self.width {
                let current_cell = self.current_buffer.get(x, y).copied();
                let previous_cell = self.previous_buffer.get(x, y).copied();

                if let (Some(current_cell), Some(previous_cell)) = (current_cell, previous_cell) {
                    if current_cell == previous_cell {
                        if let Some((start_index, start_cell)) = start {
                            let style = start_cell.style();
                            let content = self.slice_string(start_index..x, y);
                            self.queue_styled_string(stdout, start_index, y, style, &content)?;

                            start = None;
                        }
                    } else {
                        if let Some((start_index, start_cell)) = start {
                            if start_cell.style() != current_cell.style() {
                                let style = start_cell.style();
                                let content = self.slice_string(start_index..x, y);
                                self.queue_styled_string(stdout, start_index, y, style, &content)?;

                                start = Some((x, current_cell));
                            }
                        } else {
                            start = Some((x, current_cell));
                        }
                    }
                }
            }

            if let Some((start_index, start_cell)) = start {
                let style = start_cell.style();
                let content = self.slice_string(start_index..self.width, y);
                self.queue_styled_string(stdout, start_index, y, style, &content)?;
            }
        }

        stdout.flush()?;

        Ok(())
    }

    fn queue_styled_string(&mut self, stdout: &mut Stdout, x: usize, y: usize, style: &ContentStyle, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let foreground_color = style.foreground_color.unwrap_or(Color::Reset);
        let background_color = style.background_color.unwrap_or(Color::Reset);
        let underline_color = style.underline_color.unwrap_or(Color::Reset);
        let attributes = style.attributes;

        stdout.queue(MoveTo(x as u16, y as u16))?;

        if foreground_color != self.current_foreground_color {
            stdout.queue(SetForegroundColor(foreground_color))?;
            self.current_foreground_color = foreground_color;
        }

        if background_color != self.current_background_color {
            stdout.queue(SetBackgroundColor(background_color))?;
            self.current_background_color = background_color;
        }

        if underline_color != self.current_underline_color {
            stdout.queue(SetUnderlineColor(underline_color))?;
            self.current_underline_color = underline_color;
        }

        if attributes != self.current_attributes {
            stdout.queue(SetAttributes(attributes))?;
            self.current_attributes = attributes;
        }

        stdout.queue(Print(content))?;

        Ok(())
    }

    fn slice_string(&self, x_range: Range<usize>, y: usize) -> String {
        if let Some(row) = self.current_buffer.row(y) {
            row[x_range].iter().map(|cell| cell.content()).collect()
        } else {
            String::new()
        }
    }
}

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}
