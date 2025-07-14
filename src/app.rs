use std::{io::stdout, time::Duration};

use crossterm::{event::{KeyCode, KeyEvent}, execute, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{component::{Bouncer, Component, Root}, events::{Event, EventManager}, renderer::Renderer};

pub struct App {
    component_tree: Box<dyn Component>,
    renderer: Renderer,
    event_manager: EventManager,
}

impl App {
    pub fn new() -> Self {
        let mut component_tree = Box::new(Root::new());
        let bouncer = Box::new(Bouncer::new());
        component_tree.set_child(bouncer);

        App {
            component_tree,
            renderer: Renderer::new(),
            event_manager: EventManager::new(Duration::from_millis(100)),
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
        
            self.component_tree.render(&mut self.renderer);
            self.renderer.render(&mut stdout)?;
        }

        execute!(stdout, LeaveAlternateScreen)?;
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

    fn update(&mut self) {
    }
}

