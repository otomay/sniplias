use crossterm::event::{self, KeyEvent, KeyEventKind};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next(&self) -> Result<Event, std::io::Error> {
        let poll_duration = self.tick_rate;

        if event::poll(poll_duration)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    return Ok(Event::Key(key));
                }
            }
        }

        Ok(Event::Tick)
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}
