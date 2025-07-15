use crossterm::event::{self, Event as CtEvent, KeyEvent, MouseEvent, MouseEventKind};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Resize(u16, u16),
    Quit,
}

pub struct EventManager {
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl EventManager {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        let tx_clone = tx.clone();

        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    match event::read().unwrap() {
                        CtEvent::Key(key) => tx_clone.send(Event::Key(key)).unwrap(),
                        CtEvent::Resize(w, h) => tx_clone.send(Event::Resize(w, h)).unwrap(),
                        CtEvent::Mouse(mouse) => tx_clone.send(Event::Mouse(mouse)).unwrap(),
                        _ => {}
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    tx_clone.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        Self { tx, rx }
    }

    pub fn send(&self, event: Event) {
        self.tx.send(event).unwrap();
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
