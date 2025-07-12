use std::{io::stdout, time::Duration};

use crossterm::{cursor::MoveTo, event::{KeyCode, KeyEvent}, execute, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{events::{Event, EventManager}, renderer::Renderer, screen_buffer::Cell};

pub struct App {
    renderer: Renderer,
    event_manager: EventManager,
    pos_x: usize,
    pos_y: usize,
    input: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            renderer: Renderer::new(),
            event_manager: EventManager::new(Duration::from_millis(4000)),
            pos_x: 0,
            pos_y: 0,
            input: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let (width, height) = size()?;
        self.resize(width, height);

        loop {
            match self.event_manager.next()? {
                Event::Input(key) => self.handle_input(key),
                Event::Tick => self.update(),
                Event::Resize(w, h) => self.resize(w, h),
                Event::Quit => break,
            }
        
            self.renderer.render(&mut stdout)?;
            execute!(stdout, MoveTo(self.pos_x as u16, self.pos_y as u16))?;
        }

        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.renderer.resize(w as usize, h as usize);
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        if self.input {
            match key_event {
                KeyEvent { code: KeyCode::Esc, .. } => {
                    self.input = false;
                }
                KeyEvent { code: KeyCode::Char(char), .. } => {
                    let (width, height) = self.renderer.size();

                    self.renderer.set(self.pos_x, self.pos_y, Cell::Char(char));

                    if self.pos_x <= width-1 {
                        self.pos_x += 1;
                    } else if self.pos_y <= height-1 {
                        self.pos_x = 0;
                        self.pos_y += 1;
                    } else {
                        self.pos_x = 0;
                        self.pos_y = 0;
                    }
                }
                _ => {}
            }
        } else {
            match key_event {
                KeyEvent { code: KeyCode::Char('q'), .. } => {
                    self.event_manager.send(Event::Quit);
                }
                KeyEvent { code: KeyCode::Char('h'), .. } => {
                    if self.pos_x > 0 {
                        self.pos_x -= 1;
                    }
                }
                KeyEvent { code: KeyCode::Char('j'), .. } => {
                    let (_, height) = self.renderer.size();

                    if self.pos_y < height-1 {
                        self.pos_y += 1;
                    }
                }
                KeyEvent { code: KeyCode::Char('k'), .. } => {
                    if self.pos_y > 0 {
                        self.pos_y -= 1;
                    }
                }
                KeyEvent { code: KeyCode::Char('l'), .. } => {
                    let (width, _) = self.renderer.size();

                    if self.pos_x < width-1 {
                        self.pos_x += 1;
                    }
                }
                KeyEvent { code: KeyCode::Char('i'), .. } => {
                    self.input = true;
                } 
                _ => {}
            }
        }
    }

    fn update(&mut self) {
    }
}

