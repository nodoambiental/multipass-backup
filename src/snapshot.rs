use super::util;
use anyhow::Error;

pub fn run(cli: &clap::ArgMatches) -> Result<(), Error> {
    let create = cli.contains_id("create");
    let commit = cli.contains_id("commit");

    if create {
        todo!();
        //return create();
    }

    if commit {
        todo!();
        //return commit();
    }

    panic!("No valid argument provided to the config subcommand.")
}

pub fn create() -> Result<(), Error> {
    todo!();
}

pub fn commit() -> Result<(), Error> {
    todo!();
}
