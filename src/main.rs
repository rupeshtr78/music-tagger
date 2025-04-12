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

    // RenameFiles(&args.path, &args.file_type, &args.current, &args.new)?;
    // tagedit::GetTags("test/Ponveene.mp3")?;

    let tags = TagEditArgs::new(
        "",
        "test/Ponveene-rtr.mp3",
        "Nirakoottu",
        "Yesudas",
        "Ponveene",
        "Mal Film Song",
    );
    tagedit::EditTags(&tags).context("Error Editing Tags")?;

    tags.print_tags().context("Error Printing tags")?;

    Ok(())
}
