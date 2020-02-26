use std::fs;
use std::io::Error as IoError;
use std::ops::BitOr;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;
use structopt::StructOpt;
use toml;
use toml::de::Error as DeError;

/// All durations are in minutes.
/// Priority is command line argument, configuration file, and then default value.
#[derive(Copy, Clone, Debug, Deserialize, StructOpt, PartialEq)]
pub struct Config {
    #[structopt(short = "p", long = "pomodoro")]
    duration_pomodoro: Option<u64>,

    #[structopt(short = "s", long = "short")]
    duration_short_break: Option<u64>,

    #[structopt(short = "l", long = "long")]
    duration_long_break: Option<u64>,

    #[structopt(short = "r", long = "repetition")]
    repetition: Option<i32>,
}


#[derive(Debug)]
pub enum Error {
    Io(IoError),
    De(DeError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<DeError> for Error {
    fn from(err: DeError) -> Error {
        Error::De(err)
    }
}


impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let s = fs::read_to_string(&path)?;
        Ok(Config::from_str(&s)?)
    }
}

impl FromStr for Config {
    type Err = DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}


const DEFAULT_DURATION_POMODORO: u64 = 25;
const DEFAULT_DURATION_SHORT_BREAK: u64 = 5;
const DEFAULT_DURATION_LONG_BREAK: u64 = 30;
const DEFAULT_REPETITION: i32 = 4;

impl Default for Config {
    fn default() -> Config {
        Config {
            duration_pomodoro: Some(DEFAULT_DURATION_POMODORO),
            duration_short_break: Some(DEFAULT_DURATION_SHORT_BREAK),
            duration_long_break: Some(DEFAULT_DURATION_LONG_BREAK),
            repetition: Some(DEFAULT_REPETITION),
        }
    }
}


impl BitOr for Config {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Config {
            duration_pomodoro: self.duration_pomodoro.or(rhs.duration_pomodoro),
            duration_short_break: self.duration_short_break.or(rhs.duration_short_break),
            duration_long_break: self.duration_long_break.or(rhs.duration_long_break),
            repetition: self.repetition.or(rhs.repetition),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Config;

    const EMPTY_CONFIG: Config = Config {
        duration_pomodoro: None,
        duration_short_break: None,
        duration_long_break: None,
        repetition: None,
    };

    #[test]
    fn parse_empty() {
        let config = Config::from_str(r#""#);

        assert!(config.is_ok());
        assert_eq!(config.unwrap(), EMPTY_CONFIG);
    }

    #[test]
    fn parse_partial() {
        let config = Config::from_str(r#"duration_pomodoro = 25"#);

        assert!(config.is_ok());
        assert_eq!(
            config.unwrap(),
            Config {
                duration_pomodoro: Some(25),
                ..EMPTY_CONFIG
            }
        );
    }

    #[test]
    fn parse_full() {
        let config = Config::from_str(
            r#"
            duration_pomodoro = 25
            duration_short_break = 5
            duration_long_break = 30
            repetition = 4
        "#,
        );

        assert!(config.is_ok());
        assert_eq!(config.unwrap(), Default::default());
    }

    #[test]
    fn parse_excess() {
        let config = Config::from_str(
            r#"
            duration_pomodoro = 25
            duration_short_break = 5
            duration_long_break = 30
            repetition = 4
            
            [hhgttg]
            meaning-of-life = 42
        "#,
        );

        assert!(config.is_ok());
        assert_eq!(config.unwrap(), Default::default());
    }

    #[test]
    fn parse_illformed() {
        let config = Config::from_str(r#"duration: 25"#);
        assert!(config.is_err());
    }

    #[test]
    fn or() {
        let a = Config {
            duration_pomodoro: Some(20),
            duration_short_break: Some(5),
            ..EMPTY_CONFIG
        };

        let b = Config {
            duration_pomodoro: Some(25),
            duration_long_break: Some(25),
            ..EMPTY_CONFIG
        };

        assert_eq!(
            a | b,
            Config {
                duration_pomodoro: Some(20),
                duration_short_break: Some(5),
                duration_long_break: Some(25),
                repetition: None
            }
        );
    }

    #[test]
    fn or_multiple() {
        let a = Config {
            duration_pomodoro: Some(20),
            duration_short_break: Some(5),
            ..EMPTY_CONFIG
        };

        let b = Config {
            duration_pomodoro: Some(25),
            duration_long_break: Some(25),
            ..EMPTY_CONFIG
        };

        assert_eq!(
            a | b | Default::default(),
            Config {
                duration_pomodoro: Some(20),
                duration_short_break: Some(5),
                duration_long_break: Some(25),
                repetition: Some(4)
            }
        );
    }
}
