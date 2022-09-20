use super::snapshot;
use super::util;
use anyhow::Error;
use std::{path::PathBuf, str::FromStr};

pub fn run(cli: &clap::ArgMatches) -> Result<(), Error> {
    let is_all = cli.contains_id("all");
    let is_sync = cli.contains_id("sync");
    let destination = cli
        .try_get_one::<String>("output")?
        .ok_or(Error::msg("No output destination found."))?;
    let images_source = PathBuf::from_str(
        cli.try_get_one::<String>("images")?
            .ok_or(Error::msg("Error reading image sources value."))?
            .as_str(),
    )?;

    let machines = util::collect_values::<String>(cli, "machine")?;

    let image_paths = util::collect_image_paths(machines, images_source, is_all, true)?;

    if is_sync {
        sync(image_paths.clone())?;
    }

    copy(image_paths, PathBuf::from(destination))
}

pub fn copy(image_paths: Vec<PathBuf>, destination: PathBuf) -> Result<(), Error> {
    todo!()
}

pub fn sync(image_paths: Vec<PathBuf>) -> Result<(), Error> {
    snapshot::create(image_paths.clone())?;
    snapshot::commit(image_paths)
}
