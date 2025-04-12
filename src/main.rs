#![allow(non_snake_case)]
use ::anyhow::Context;
use anyhow::Result;
use cli::{cli, cli_dialog, SetLogLevel};
use tagedit::TagEditArgs;
mod cli;
mod file_renamer;
mod tagedit;

fn main() -> Result<()> {
    // let args = cli()?;

    // let args = cli_dialog()?;

    // SetLogLevel(&args.loglevel)?;
    SetLogLevel("debug")?;

    // RenameFiles(&args.path, &args.file_type, &args.current, &args.new)?;
    // tagedit::GetTags("test/Ponveene.mp3")?;

    let tags = TagEditArgs::new("wav", "wholedir", "Yesudas", "Ponveene", "Mal Film Song");
    // tagedit::EditTags("test/Ponveene-rtr.mp3", &tags).context("Error Editing Tags")?;

    tags.tag_all("test").context("Error updating tags")?;
    tagedit::print_tags("test/ThalaivarTheme.wav").context("Error Printing tags")?;

    Ok(())
}
