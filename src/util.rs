use chrono;
use colored::*;
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
