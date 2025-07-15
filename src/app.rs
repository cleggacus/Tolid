use std::{io::{stdout, Stdout}, time::Duration};

use crossterm::{cursor::{Hide, Show}, event::{KeyCode, KeyEvent}, execute, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{component::Component, events::{Event, EventManager}, renderer::Renderer};

pub struct App {
    root: Box<dyn Component>,
    renderer: Renderer,
    event_manager: EventManager,
    stdout: Stdout,
}

impl App {
    pub fn new<T: Component + 'static>(root: T) -> Self {
        App {
            root: Box::new(root),
            renderer: Renderer::new(),
            event_manager: EventManager::new(Duration::from_millis(33)),
            stdout: stdout()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen)?;

        let (width, height) = size()?;
        self.resize(width, height);
        execute!(self.stdout, Hide)?;

        loop {
            // let mut root = (self.root_fn)();

            match self.event_manager.next()? {
                Event::Input(key) => self.handle_input(key),
                Event::Tick => {},
                Event::Resize(w, h) => self.resize(w, h),
                Event::Quit => break,
            }

            self.root.render(&mut self.renderer);
            self.renderer.render(&mut self.stdout);
        }

        execute!(self.stdout, Show)?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.renderer.resize(w as usize, h as usize);
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent { code: KeyCode::Char('q'), .. } => {
                self.event_manager.send(Event::Quit);
            }
            _ => {}
        }
    }

    // fn render(&mut self, root: &mut dyn Component) -> Result<(), Box<dyn std::error::Error>> {
    //     root.render(&mut self.renderer);
    //     self.renderer.render(&mut self.stdout)?;
    //     Ok(())
    // }
}

