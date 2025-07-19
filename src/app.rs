use std::{io::{stdout, Stdout}, time::Duration};

use crossterm::{cursor::{Hide, Show}, event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, MouseEvent, MouseEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{component::{Component, ComponentEvent}, events::{Event, EventManager}, renderer::Renderer, state::StateContext};

pub fn run_app<F, P, C>(f: F)
where
    F: Fn(P) -> C + 'static,
    P: Default,
    C: Component + 'static,
{
    let root_fn = Box::new(move || Box::new(f(Default::default())) as Box<dyn Component>);

    App::new(root_fn)
        .run()
        .unwrap();
}


struct App {
    root_fn: Box<dyn Fn() -> Box<dyn Component + 'static>>,
    renderer: Renderer,
    event_manager: EventManager,
    _state_context: StateContext,
    stdout: Stdout,
}

impl App {
    pub fn new(root_fn: Box<dyn Fn() -> Box<dyn Component + 'static>>) -> Self {
        App {
            root_fn,
            renderer: Renderer::new(),
            event_manager: EventManager::new(Duration::from_millis(33)),
            _state_context: StateContext::new(),
            stdout: stdout()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen)?;
        execute!(self.stdout, EnableMouseCapture)?;

        let (width, height) = size()?;
        self.resize(width, height);
        execute!(self.stdout, Hide)?;

        let mut root = (self.root_fn)();

        loop {
            match self.event_manager.next()? {
                Event::Key(key) => self.handle_key(key),
                Event::Mouse(mouse) => self.handle_mouse(mouse),
                Event::Tick => {},
                Event::Resize(w, h) => self.resize(w, h),
                Event::Component(component_event) => root.propagate_event(&component_event),
                Event::Quit => break,
            }

            root.render(&mut self.renderer);
            self.renderer.render(&mut self.stdout)?;
        }

        execute!(self.stdout, Show)?;
        execute!(self.stdout, DisableMouseCapture)?;
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.renderer.resize(w as usize, h as usize);
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent { code: KeyCode::Char('q'), .. } => {
                self.event_manager.send(Event::Quit);
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent { kind: MouseEventKind::Down(_), row, column, .. } => {
                self.event_manager.send(Event::Component(
                    ComponentEvent::OnClick(column as usize, row as usize)
                ));
            },
            _ => {}
        }
    }

    // fn render(&mut self, root: &mut dyn Component) -> Result<(), Box<dyn std::error::Error>> {
    //     root.render(&mut self.renderer);
    //     self.renderer.render(&mut self.stdout)?;
    //     Ok(())
    // }
}

