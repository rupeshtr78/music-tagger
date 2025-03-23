use anyhow::Result;
use log::debug;

mod cli;
use cli::cli_args;
mod file_renamer;
use file_renamer::rename_files;
mod tagedit;

fn main() -> Result<()> {
    // let args = cli_args()?;

    // let verbose = args.verbose == "false";

    // if verbose {
    //     env_logger::builder()
    //         .filter_level(log::LevelFilter::Debug)
    //         .init();
    // } else {
    //     env_logger::init();
    // }

    // debug!("Starting renaming process with args: {:?}", args);

    // rename_files(&args.path, &args.file_type, &args.current, &args.new)?;
    tagedit::read_tags("/Users/rupeshraghavan/apl/gits/gits-rupesh/rtr-rust-lab/multi-workspace/mp3-renamer/test/Ponveene.mp3")?;

    Ok(())
}
