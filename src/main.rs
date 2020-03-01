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

            let mut event_stream = EventStream::new(config);

            while let Some(event) = event_stream.next() {
                if !Event::confirm_start(&event) {
                    break;
                }

                event.run();

                event.send_notification();
            }

            let pomodoros = event_stream.message_count_pomodoro();

            println!("\nYou've done {}.\n", style(&pomodoros).cyan().bold());
        }
    }

    Ok(())
}
