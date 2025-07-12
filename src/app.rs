use std::io::stdout;

use crossterm::{event::{read, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::renderer::Renderer;

pub struct App {
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        App {
            renderer: Renderer::new()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        loop {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                _ => {}
            }
        }

        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

