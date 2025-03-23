#![allow(non_snake_case)]
use anyhow::Result;
use log::debug;

mod cli;
use cli::{cli, SetLogLevel};
mod file_renamer;
use file_renamer::RenameFiles;
mod tagedit;

fn main() -> Result<()> {
    let args = cli()?;

    SetLogLevel();

    RenameFiles(&args.path, &args.file_type, &args.current, &args.new)?;
    // tagedit::read_tags("test/Ponveene.mp3")?;

    Ok(())
}
