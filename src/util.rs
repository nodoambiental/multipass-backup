use anyhow::Error;
use chrono;
use colored::*;
use std::any::Any;
use std::path::PathBuf;
use std::process;

// From https://stackoverflow.com/a/52367953/16134348
pub fn string_to_sstr(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn stdout(selector: &str, message: &str) {
    let time = chrono::offset::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    // TODO implement IO error handling
    match selector {
        "info" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[INFO]".bright_blue().bold(),
                message.bright_blue().italic()
            );
        }
        "debug" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[DEBUG]".yellow().bold(),
                message.yellow().italic()
            );
        }
        "fatal" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[FATAL]".bright_purple().bold(),
                message.bright_red().bold()
            );
            process::exit(1);
        }
        "error" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[ERROR]".bright_red().bold(),
                message.bright_red().italic()
            );
        }
        "warning" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[WARN]".yellow().bold(),
                message.yellow().italic()
            );
        }
        "success" => {
            println!(
                "{} {} {}",
                format!("[{}]", time).as_str().magenta(),
                "[SUCCESS]".bright_green().bold(),
                message.bright_green().italic()
            );
        }
        _ => {
            println!(
                "{} {}",
                format!("[{}]", time).as_str().magenta(),
                message.normal().italic()
            );
        }
    }
}

pub fn collect_values<T: Any + Clone + Send + Sync>(
    matches: &clap::ArgMatches,
    argument: &str,
) -> Result<Vec<T>, Error> {
    let is_argument = matches.contains_id(argument);

    let values: Vec<T> = match is_argument {
        true => match matches.try_get_many::<T>(argument)? {
            None => Vec::new(),
            Some(values) => {
                let mut list = Vec::new();
                for value in values {
                    list.push(value.clone())
                }
                list
            }
        },
        false => Vec::new(),
    };
    Ok(values)
}

pub fn collect_image_paths(
    image_names: Vec<String>,
    images_source: PathBuf,
    all: bool,
    base: bool,
) -> Result<Vec<PathBuf>, Error> {
    todo!()
}

pub fn base_image_exists(image_paths: Vec<PathBuf>) -> Result<bool, Error> {
    todo!()
}
