/// Event is one of pomodoro, short break, and long break.
/// EventStream is something that sequentially generates event according to config.
/// EventStream is constructed by a Config.
use std::thread;
use std::time::{Duration, Instant};

use console::Term;
use dialoguer::Confirmation;
use indicatif::{ProgressBar, ProgressStyle};

#[cfg(target_os = "macos")]
use mac_notification_sys;

use crate::config::Config;

pub struct EventStream {
    duration_pomodoro: u64,
    duration_short_break: u64,
    duration_long_break: u64,
    repetition: i32,
    count: i32,
}

impl EventStream {
    pub fn new(config: Config) -> Self {
        EventStream {
            duration_pomodoro: config.duration_pomodoro.unwrap(),
            duration_short_break: config.duration_short_break.unwrap(),
            duration_long_break: config.duration_long_break.unwrap(),
            repetition: config.repetition.unwrap(),
            count: 0,
        }
    }

    pub fn message_count_pomodoro(n: usize) -> String {
        if n == 0 {
            "nothing".to_string()
        } else if n < 3 {
            format!("1 pomodoro")
        } else {
            format!("{} pomodoros", (n + 1) / 2)
        }
    }
}

impl Iterator for EventStream {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.count;
        let set = self.repetition * 2;

        let event = if count % set == set - 1 {
            Event::LongBreak(self.duration_long_break)
        } else if count % 2 == 1 {
            Event::ShortBreak(self.duration_short_break)
        } else {
            Event::Pomodoro(self.duration_pomodoro)
        };

        self.count += 1;

        Some(event)
    }
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Pomodoro(u64),
    ShortBreak(u64),
    LongBreak(u64),
}

impl Event {
    /// Duration of event in **seconds**
    pub fn duration(&self) -> u64 {
        *match self {
            Event::Pomodoro(d) => d,
            Event::ShortBreak(d) => d,
            Event::LongBreak(d) => d,
        } * 60
    }

    pub fn message_name(&self) -> &str {
        match self {
            Event::Pomodoro(_) => "Pomodoro",
            Event::ShortBreak(_) => "Short break",
            Event::LongBreak(_) => "Long break",
        }
    }

    pub fn message_start_confirmation(&self) -> &str {
        match self {
            Event::Pomodoro(_) => "Ready to start a pomodoro?",
            Event::ShortBreak(_) => "Start a short break?",
            Event::LongBreak(_) => "Start a long break?",
        }
    }

    pub fn confirm_start(&self) -> bool {
        let confirm_start = Confirmation::new()
            .with_text(self.message_start_confirmation())
            .interact()
            .unwrap();

        Term::stderr().clear_last_lines(1).unwrap();

        confirm_start
    }

    pub fn run(&self) {
        let start = Instant::now();

        let bar = ProgressBar::new(self.duration());
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\t {elapsed_precise} [{bar:25.cyan/blue}]")
                .progress_chars("#>-"),
        );
        bar.set_message(&format!("  {}", self.message_name()));

        loop {
            let time_elapsed = Instant::now().duration_since(start).as_secs();
            bar.set_position(time_elapsed);

            if time_elapsed >= self.duration() {
                bar.finish_with_message(&format!("✓ {}", self.message_name()));
                break;
            }

            thread::sleep(Duration::from_millis(500));
        }
    }

    /// Send desktop notification. Currently only works on macOS.
    pub fn send_notification(&self) {
        #[cfg(target_os = "macos")]
        mac_notification_sys::send_notification(
            &format!("{} finished", self.message_name()),
            &None,
            match self {
                Event::Pomodoro(_) => "Take a break",
                Event::ShortBreak(_) => "Ready for another pomodoro?",
                Event::LongBreak(_) => "Ready for another pomodoro?",
            },
            &Some("Ping"),
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_config() {
        let mut es = EventStream::new(Default::default());

        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::ShortBreak(5)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::ShortBreak(5)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::ShortBreak(5)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(30)));
    }

    #[test]
    fn no_short_break_config() {
        let mut es = EventStream::new(Config {
            repetition: Some(1),
            ..Default::default()
        });

        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(30)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(30)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(30)));
    }

    #[test]
    fn alternating_config() {
        let mut es = EventStream::new(Config {
            duration_long_break: Some(15),
            repetition: Some(2),
            ..Default::default()
        });

        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::ShortBreak(5)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(15)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::ShortBreak(5)));
        assert_eq!(es.next(), Some(Event::Pomodoro(25)));
        assert_eq!(es.next(), Some(Event::LongBreak(15)));
    }
}
