use std::thread;
use std::time::{Duration, Instant};

use console::{style, Term};
use dialoguer::Confirmation;
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

mod config;
mod notification;

use config::Config;

// @TODO: Turn off mac notification via config
// @TODO: Show stats even with SIGINT, SIGTERM

#[derive(Debug, StructOpt)]
#[structopt(name = "pomodoro", about = "A command line pomodoro timer.")]
enum Opt {
    /// Start pomodoro timer with given task name
    Do {
        task: String,

        #[structopt(flatten)]
        config: Config,
    },
}

fn main() -> std::io::Result<()> {
    match Opt::from_args() {
        Opt::Do {
            task,
            config: config_opt,
        } => {
            let config_path = format!(
                "{}/{}",
                dirs::home_dir().unwrap().display(),
                ".config/pomodoro/config.toml"
            );
            let config_file = Config::load(&config_path).unwrap();

            let _ = config_opt | config_file | Default::default();

            println!("On task {}\n", style(&task).green().bold());

            for i in 0.. {
                let event = if i % 8 == 7 {
                    Event::LongBreak
                } else if i % 2 == 1 {
                    Event::ShortBreak
                } else {
                    Event::Pomodoro(&task)
                };

                if i != 0 {
                    let will_continue = Confirmation::new()
                        .with_text(event.start_comfirmation())
                        .interact()?;
                    Term::stderr().clear_last_lines(1)?;

                    if !will_continue {
                        let pomodoros = if i == 0 {
                            "none".to_string()
                        } else if i == 1 || i == 2 {
                            format!("1 pomodoro")
                        } else {
                            format!("{} pomodoros", (i + 1) / 2)
                        };

                        println!("\nYou've done {}.\n", style(&pomodoros).cyan().bold());

                        break;
                    }
                }

                event.run();
            }
        }
    }

    Ok(())
}

// @TODO: EventStream<Item=Event>::with(config)

#[derive(Debug)]
enum Event<'a> {
    Pomodoro(&'a str),
    ShortBreak,
    LongBreak,
}

const DURATION_POMODORO: u64 = 25 * 60;
const DURATION_SHORT_BREAK: u64 = 5 * 60;
const DURATION_LONG_BREAK: u64 = 30 * 60;

impl Event<'_> {
    fn name(&self) -> &str {
        match self {
            Event::Pomodoro(_) => "Pomodoro",
            Event::ShortBreak => "Short break",
            Event::LongBreak => "Long break",
        }
    }

    fn start_comfirmation(&self) -> &str {
        match self {
            Event::Pomodoro(_) => "Ready for another pomodoro?",
            Event::ShortBreak => "Start a short break?",
            Event::LongBreak => "Start a long break?",
        }
    }

    fn end_notification_body(&self) -> &str {
        match self {
            Event::Pomodoro(_) => "Take a break",
            Event::ShortBreak => "Ready for another pomodoro?",
            Event::LongBreak => "Ready for another pomodoro?",
        }
    }

    fn duration(&self) -> u64 {
        match self {
            Event::Pomodoro(_) => DURATION_POMODORO,
            Event::ShortBreak => DURATION_SHORT_BREAK,
            Event::LongBreak => DURATION_LONG_BREAK,
        }
    }

    fn run(&self) {
        let start = Instant::now();

        let bar = ProgressBar::new(self.duration());
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\t {elapsed_precise} [{bar:25.cyan/blue}]")
                .progress_chars("#>-"),
        );

        bar.set_message(&format!("  {}", self.name()));

        loop {
            let time_elapsed = Instant::now().duration_since(start).as_secs();
            bar.set_position(time_elapsed);

            if time_elapsed >= self.duration() {
                bar.finish_with_message(&format!("✓ {}", self.name()));
                break;
            }

            thread::sleep(Duration::from_millis(500));
        }

        let message = format!("{} finished", self.name());
        notification::send(&message, self.end_notification_body());
    }
}
