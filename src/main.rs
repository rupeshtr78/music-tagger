#![allow(non_snake_case)]
use anyhow::Result;
use cli::{cli, SetLogLevel};
mod file_renamer;
use file_renamer::RenameFiles;
mod cli;
mod tagedit;

fn main() -> Result<()> {
    let args = cli()?;

    // let args = cli_dialog()?;

    SetLogLevel(&args.loglevel)?;

    RenameFiles(&args.path, &args.file_type, &args.current, &args.new)?;
    // tagedit::read_tags("test/Ponveene.mp3")?;

    Ok(())
}
