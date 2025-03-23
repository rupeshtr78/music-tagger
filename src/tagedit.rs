use anyhow::{Context, Ok, Result};
use id3::{Tag, TagLike, Version};
use std::fs::{copy, File};

pub fn GetTags(file_path: &str) -> Result<()> {
    let temp_file_path = std::env::temp_dir().join("music.mp3");
    let file_path = std::path::Path::new(file_path);
    let temp_file = std::path::Path::new("test/music.mp3");
    copy(file_path, temp_file)?;

    let mut tag = Tag::read_from_path(temp_file).context("Failed to read tags")?;

    println!("Before tag update:");
    tag.title().map(|t| println!("  Title: {}", t));
    tag.artist().map(|a| println!(" Artist: {}", a));
    tag.album().map(|a| println!("  Album: {}", a));
    tag.genre().map(|g| println!("  Genre: {}", g));

    tag.set_album("Tag updated Album Title2");
    tag.set_artist("Tag updated Artist Name2");
    // tag.set_title("Tag updated Title");
    tag.set_genre("Tag updated Genre2");

    tag.write_to_path(temp_file, Version::Id3v24)?;

    println!("After tag Update:");
    tag.title().map(|t| println!("  Title: {}", t));
    tag.artist().map(|a| println!(" Artist: {}", a));
    tag.album().map(|a| println!("  Album: {}", a));
    tag.genre().map(|g| println!("  Genre: {}", g));

    copy(&temp_file, file_path)?;
    // delete temp file
    std::fs::remove_file(temp_file)?;

    Ok(())
}
