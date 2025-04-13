#![allow(non_snake_case)]
use anyhow::Context;
use anyhow::Result;
use cli::SetLogLevel;
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

    let tags =
        tagedit::TagEditArgs::new("wav", "Elton John Hits", "Elton John", "filename", "Blues");
    // // // tagedit::EditTags("test/Ponveene-rtr.mp3", &tags).context("Error Editing Tags")?;

    // All files in the directory
    tags.tag_all("/data/UT/Music/1-LossLess/EltonJohn-wav")
        .context("Error updating tags")?;

    // Single File Edit
    // tagedit::EditTags(
    //     "/data/UT/Music/1-LossLess/EltonJohn-wav/03 Sacrifice.wav",
    //     &tags,
    // )
    // .context("Error Editing Tags")?;

    tagedit::print_tags("/data/UT/Music/1-LossLess/EltonJohn-wav/09 Blue Eyes.wav")
        .context("Error Printing tags")?;

    Ok(())
}
