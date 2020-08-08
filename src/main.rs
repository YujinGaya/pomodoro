use console::style;
use structopt::StructOpt;

mod config;
mod event;

use config::Config;
use event::{Event, EventStream};

// @TODO: Show stats even with SIGINT, SIGTERM
// @TODO: graceful fail (parsing config.. etc)

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
            println!("On task {}\n", style(&task).green().bold());

            let config_file = Config::load().unwrap();
            let config = config_opt | config_file | Default::default();

            let pomodoros = EventStream::new(config)
                .take_while(|event| {
                    if Event::confirm_start(&event) {
                        event.run();
                        event.send_notification();
                        
                        true
                    } else {
                        false
                    }
                })
                .filter(|event| {
                    if let Event::Pomodoro(_) = event {
                        true
                    } else {
                        false
                    }
                })
                .count();

            let pomodoros = EventStream::message_count_pomodoro(pomodoros);

            println!("\nYou've done {}.\n", style(pomodoros).cyan().bold());
        }
    }

    Ok(())
}
