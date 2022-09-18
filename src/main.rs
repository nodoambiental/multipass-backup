use anyhow::Error;
use clap::{Arg, ArgGroup, Command};
use colored::*;
use std::process;

mod backup;
mod snapshot;
mod util;

fn main() {
    let matches = Command::new("multipass-images")
        .version("0.2.0")
        .author(util::string_to_sstr(format!("{}", "Agata Ordano - aordano@protonmail.com".bright_cyan())))
        .about("Utility to easily create and backup snapshots for Canonical's Multipass virtual machines.")
        .long_about(concat! ("This utility uses QEMU to create and manage snapshots for Multipass image files, and Rsync to backup/update to a given path.\n\n"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("snapshot")
            .about("Creates and commits snapshots for Multipass virtual machine image files.")
            .long_about(util::string_to_sstr(
                format!("{}\n {}",
                    "This command eases using QEMU to manage snapshots to the source image files. ".yellow(),
                    concat!("You can create new snapshots, commit them, or both. ")
                )
            ))
            .arg_required_else_help(true)
            .arg(
                Arg::new("images")
                .alias("sources")
                .short('i')
                .long("images")
                .multiple_values(false)
                .takes_value(true)
                .value_name("MULTIPASS IMAGES PARENT DIR")
                .default_value("/var/snap/multipass/common/data/multipassd/vault/instances")
                .help("Path pointing to the parent directory where Multipass images reside. This directory should only contain folders named after the machines, which each folder containing the image file. You can find it on the default ubuntu installation with \'sudo find /var/ -name \"instances\" -type d\'")
            )
            .arg(
                Arg::new("create")
                .alias("new")
                .short('n')
                .long("create")
                .takes_value(true)
                .multiple_values(true)
                .value_name("MACHINE")
                .help("Creates a snapshot for the given machines.")
                .conflicts_with("commit")
            )
            .arg(
                Arg::new("commit")
                .alias("apply")
                .short('c')
                .long("commit")
                .takes_value(true)
                .multiple_values(true)
                .value_name("MACHINE")
                .conflicts_with("create")
                .help("Commits snapshots for the given machines.")
            )
            .arg(
                Arg::new("commit-all")
                .long("commit-all")
                .takes_value(false)
                .conflicts_with_all(&["commit", "create"])
                .help("Commits snapshots for all machines.")
            )
            .arg(
                Arg::new("create-all")
                .long("create-all")
                .takes_value(false)
                .conflicts_with_all(&["commit", "create"])
                .help("Creates a snapshot for all machines.")
            )
        )
        .subcommand(
            Command::new("backup")
            .about("Backs up Multipass Image files to a selected directory.")
            .arg_required_else_help(true)
            .arg(
                Arg::new("images")
                .alias("sources")
                .short('i')
                .long("images")
                .multiple_values(false)
                .takes_value(true)
                .value_name("MULTIPASS IMAGES PARENT DIR")
                .default_value("/var/snap/multipass/common/data/multipassd/vault/instances")
                .help("Path pointing to the parent directory where Multipass images reside. This directory should only contain folders named after the machines, which each folder containing the image file. You can find it on the default ubuntu installation with \'sudo find /var/ -name \"instances\" -type d\'")
            )
            .arg(
                Arg::new("machine")
                .short('m')
                .long("machine")
                .multiple_values(true)
                .takes_value(true)
                .value_name("MACHINE NAME")
                .help("Name(s) of the machine(s) to be backed up.")
                .conflicts_with("all")
            )
            .arg(
                Arg::new("sync")
                .alias("snapshot")
                .short('s')
                .long("sync")
                .takes_value(false)
                .help("Creates and commits a snapshot before backup.")
            )
            .arg(
                Arg::new("output")
                .alias("out")
                .short('o')
                .long("out")
                .takes_value(true)
                .required(true)
                .help("Sets the output directory.")
            )
            .arg(
                Arg::new("all")
                .short('a')
                .long("all")
                .takes_value(false)
                .conflicts_with("machine")
                .help("Backs up all machines.")
            )
        )
        .get_matches();
    let runtime = run(matches);
    match runtime {
        Ok(()) => {}
        Err(error) => util::stdout("fatal", &error.to_string()),
    }
}

fn run(cli: clap::ArgMatches) -> Result<(), Error> {
    return match cli.subcommand() {
        Some(("snapshot", sub_m)) => snapshot::run(sub_m),
        Some(("backup", sub_m)) => backup::run(sub_m),
        _ => Ok(()),
    };
}
