use super::util;
use anyhow::Error;
use std::{path::PathBuf, str::FromStr};

pub fn run(cli: &clap::ArgMatches) -> Result<(), Error> {
    let is_create = cli.contains_id("create");
    let is_commit = cli.contains_id("commit");
    let images_source = PathBuf::from_str(
        cli.try_get_one::<String>("images")?
            .ok_or(Error::msg("Error reading image sources value."))?
            .as_str(),
    )?;

    if is_create {
        let machines = util::collect_values::<String>(cli, "create")?;
        let is_all = cli.contains_id("create-all");
        let image_paths = util::collect_image_paths(machines, images_source, is_all, true)?;

        return create(image_paths);
    }

    if is_commit {
        let machines = util::collect_values::<String>(cli, "commit")?;
        let is_all = cli.contains_id("commit-all");
        let image_paths = util::collect_image_paths(machines, images_source, is_all, true)?;

        return commit(image_paths);
    }

    panic!("No valid argument provided to the snapshot subcommand.")
}

pub fn create(image_paths: Vec<PathBuf>) -> Result<(), Error> {
    todo!()
}

pub fn commit(image_paths: Vec<PathBuf>) -> Result<(), Error> {
    todo!()
}

pub fn new(image_path: PathBuf) -> Result<(), Error> {
    todo!()
}
