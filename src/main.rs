#![forbid(unsafe_code)]

use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod analyzer;
mod bundle;
mod bundle_set;
mod nodes;
mod target_database;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Bundles to operate on
    bundles: Vec<PathBuf>,

    /// The path to which to save the stitched bundle
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let output_file = File::create(cli.output)?;
    let output_writer = BufWriter::new(output_file);
    let output_archive = zip::ZipWriter::new(output_writer);

    let mut bundles = vec![];
    let mut metadatas = vec![];
    for path in cli.bundles {
        let bundle = bundle::Bundle::open(&path)?;

        let metadata = bundle.metadata.clone();
        bundles.push(bundle);
        metadatas.push(metadata);
    }

    let mut bundles = bundle_set::BundleSet::new(bundles.into_iter());

    let stitched_metadata = bundle::StitchedMetadata::new(metadatas);
    bundles.link()?;
    bundles.splice(&stitched_metadata, output_archive)?;

    Ok(())
}
