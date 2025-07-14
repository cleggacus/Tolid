use std::{io::{Stdout, Write}, ops::Range, mem};

use crossterm::{cursor::MoveTo, queue, style::{Attribute, Attributes, Color, ContentStyle, Print, PrintStyledContent, SetAttribute, SetAttributes, SetBackgroundColor, SetForegroundColor, SetUnderlineColor, StyledContent, Stylize}, QueueableCommand};

use crate::screen_buffer::{BufferCell, ScreenBuffer};

type BoxCharLayout = u8;

const BOX_UP: BoxCharLayout     = 1 << 3;
const BOX_RIGHT: BoxCharLayout  = 1 << 2;
const BOX_DOWN: BoxCharLayout   = 1 << 1;
const BOX_LEFT: BoxCharLayout   = 1 << 0;

const BOX_CHAR_LAYOUT_COUNT: usize = 1 << 4;

// ----------------------- UNICODE BOX CHAR TABLE -----------------------
//          0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
// U+250x   ─   ━   │   ┃   ┄   ┅   ┆   ┇   ┈   ┉   ┊   ┋   ┌	┍   ┎	┏
// U+251x   ┐   ┑   ┒   ┓   └   ┕   ┖   ┗   ┘   ┙   ┚   ┛   ├	┝   ┞	┟
// U+252x   ┠   ┡   ┢   ┣   ┤   ┥   ┦   ┧   ┨   ┩   ┪   ┫   ┬	┭   ┮	┯
// U+253x   ┰   ┱   ┲   ┳   ┴   ┵   ┶   ┷   ┸   ┹   ┺   ┻   ┼	┽   ┾	┿
// U+254x   ╀   ╁   ╂   ╃   ╄   ╅   ╆   ╇   ╈   ╉   ╊   ╋   ╌	╍   ╎	╏
// U+255x   ═   ║   ╒   ╓   ╔   ╕   ╖   ╗   ╘   ╙   ╚   ╛   ╜	╝   ╞	╟
// U+256x   ╠   ╡   ╢   ╣   ╤   ╥   ╦   ╧   ╨   ╩   ╪   ╫   ╬	╭   ╮	╯
// U+257x   ╰   ╱   ╲   ╳   ╴   ╵   ╶   ╷   ╸   ╹   ╺   ╻   ╼	╽   ╾	╿

const BOX_CHAR_OFFSET: usize = '─' as usize;
const BOX_CHAR_COUNT: usize = 128;

struct BoxCharMap {
    to_char: [char; BOX_CHAR_LAYOUT_COUNT],
    to_layout: [u8; BOX_CHAR_COUNT],
}

impl BoxCharMap {
    pub const fn new() -> Self {
        let mut to_char = [' '; BOX_CHAR_LAYOUT_COUNT];
        let mut to_layout = [0u8; BOX_CHAR_COUNT];

        macro_rules! set {
            ($layout:expr, $ch:literal) => {
                to_char[$layout as usize] = $ch;
                to_layout[$ch as usize - BOX_CHAR_OFFSET] = $layout;
            };
        }

        // Half lines
        set!(BOX_UP, '╵');
        set!(BOX_RIGHT, '╶');
        set!(BOX_DOWN, '╷');
        set!(BOX_LEFT, '╴');

        // Full lines
        set!(BOX_UP   | BOX_DOWN, '│');
        set!(BOX_LEFT | BOX_RIGHT, '─');

        // Corners
        set!(BOX_DOWN | BOX_RIGHT, '╭');
        set!(BOX_DOWN | BOX_LEFT,  '╮');
        set!(BOX_UP   | BOX_RIGHT, '╰');
        set!(BOX_UP   | BOX_LEFT,  '╯');

        // T shapes
        set!(BOX_DOWN | BOX_LEFT | BOX_RIGHT, '┬');
        set!(BOX_UP   | BOX_LEFT | BOX_RIGHT, '┴');
        set!(BOX_UP   | BOX_DOWN | BOX_RIGHT, '├');
        set!(BOX_UP   | BOX_DOWN | BOX_LEFT,  '┤');

        // Cross
        set!(BOX_UP | BOX_RIGHT | BOX_DOWN | BOX_LEFT, '┼');

        Self {
            to_char,
            to_layout
        }
    }

    pub fn char_to_layout(&self, current_char: char) -> BoxCharLayout {
        if (current_char as usize) < BOX_CHAR_OFFSET || (current_char as usize) >= BOX_CHAR_OFFSET + BOX_CHAR_COUNT {
            return 0;
        }

        self.to_layout[current_char as usize - BOX_CHAR_OFFSET]
    }

    pub fn layout_to_char(&self, layout: BoxCharLayout) -> char {
        self.to_char[layout as usize]
    }

    pub fn overlap_char(&self, c: char, layout: BoxCharLayout) -> char {
        let old_layout = self.char_to_layout(c);
        let new_layout = old_layout | layout;
        self.layout_to_char(new_layout)
    }
}

const BOX_CHAR_LAYOUT_MAP: BoxCharMap = BoxCharMap::new();

#[derive(Clone, Copy)]
pub struct RenderContext {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Renderer {
    current_buffer: ScreenBuffer,
    previous_buffer: ScreenBuffer,
    width: usize,
    height: usize,

    current_foreground_color: Color,
    current_background_color: Color,
    current_underline_color: Color,
    current_attributes: Attributes,

    render_context_global: RenderContext,
    render_context_stack: Vec<RenderContext>,
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

         render_context_global: RenderContext { x: 0, y: 0, width: 0, height: 0 },
         render_context_stack: vec![],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.current_buffer.resize(width, height);
        self.previous_buffer.resize(width, height);
        self.current_buffer.clear();
        self.previous_buffer.clear();
        self.render_context_global.width = width;
        self.render_context_global.height = height;
    }

    pub fn pop_render_context(&mut self) {
        self.render_context_stack.pop();
    }

    pub fn push_render_context(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.render_context_stack.push(RenderContext { x, y, width, height });
    }

    pub fn current_render_context(&self) -> &RenderContext {
        self.render_context_stack.last()
            .unwrap_or(&self.render_context_global)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&BufferCell> {
        let render_context = self.current_render_context();

        if x >= render_context.width || y >= render_context.height {
            return None;
        }

        let x = x + render_context.x;
        let y = y + render_context.y;

        self.current_buffer.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, cell: BufferCell) {
        let render_context = self.current_render_context();

        if x >= render_context.width || y >= render_context.height {
            return;
        }

        let x = x + render_context.x;
        let y = y + render_context.y;

        self.current_buffer.set(x, y, cell);
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.draw_v_capped_line(x, y, height);
        self.draw_v_capped_line(x+width-1, y, height);
        self.draw_h_capped_line(x, y, width);
        self.draw_h_capped_line(x, y+height-1, width);
    }

    pub fn draw_v_capped_line(&mut self, x: usize, y: usize, length: usize) {
        if length <= 1 {
            return;
        }

        self.draw_box_char(x, y, BOX_DOWN);
        self.draw_box_char(x, y+length-1, BOX_UP);

        for y in y+1..y+length-1 {
            self.draw_box_char(x, y, BOX_DOWN | BOX_UP);
        }
    }

    pub fn draw_h_capped_line(&mut self, x: usize, y: usize, length: usize) {
        if length <= 1 {
            return;
        }

        self.draw_box_char(x, y, BOX_RIGHT);
        self.draw_box_char(x+length-1, y, BOX_LEFT);

        for x in x+1..x+length-1 {
            self.draw_box_char(x, y, BOX_RIGHT | BOX_LEFT);
        }
    }

    pub fn draw_box_char(&mut self, x: usize, y: usize, box_char: BoxCharLayout) {
        if let Some(c) = self.get(x, y) {
            let new_char = BOX_CHAR_LAYOUT_MAP.overlap_char(*c.content(), box_char);
            self.set(x, y, BufferCell::new(*c.style(), new_char));
        }
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

        mem::swap(&mut self.previous_buffer, &mut self.current_buffer);
        self.current_buffer.clear();

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
