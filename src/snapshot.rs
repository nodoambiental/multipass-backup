use super::util;
use anyhow::Error;

pub fn run(cli: &clap::ArgMatches) -> Result<(), Error> {
    let create = cli.contains_id("create");
    let generate = cli.contains_id("generate");

    if create {
        todo!();
        //return create();
    }

    if generate {
        todo!();
        //return generate();
    }

    panic!("No valid argument provided to the config subcommand.")
}

pub fn create() -> Result<(), Error> {
    todo!();
}

pub fn generate() -> Result<(), Error> {
    todo!();
}
