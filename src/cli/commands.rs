use crate::common::models;
use crate::{cli::manifests, cli::utils};
use clap::ArgMatches;

pub fn check(sub_matches: &ArgMatches) -> models::Manifest {
    let filepath = sub_matches.get_one::<String>("filepath").unwrap();
    let raw_manifest = utils::load_file(filepath);
    manifests::parse_manifest_file(raw_manifest)
}
