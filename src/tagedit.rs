use anyhow::{anyhow, Context, Ok, Result};
use id3::{Tag, TagLike, Version};
use std::fs::read_dir;
use std::fs::{copy, File};

pub fn GetTags(file_path: &str) -> Result<()> {
    let file_path = std::path::Path::new(file_path);
    let file_name = file_path
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("music.mp3"))
        .to_str()
        .unwrap_or("music.mp3");
    let temp_file = std::env::temp_dir().join(file_name);
    // let temp_file = std::path::Path::new("test/music.mp3");
    copy(file_path, &temp_file)?;

    let temp_file = temp_file.as_path();

    let mut tag = Tag::read_from_path(temp_file).context("Failed to read tags")?;

    println!("Before tag update:");
    tag.title().map(|t| println!("  Title: {}", t));
    tag.artist().map(|a| println!(" Artist: {}", a));
    tag.album().map(|a| println!("  Album: {}", a));
    tag.genre().map(|g| println!("  Genre: {}", g));

    let current_title = tag.title().unwrap_or("Unknown Title").to_string();
    tag.set_album("Tag updated Album Title-u");
    tag.set_artist("Tag updated Artist Name-u");
    tag.set_title(&current_title);
    tag.set_genre("Tag updated Genre-u");

    tag.write_to_path(temp_file, Version::Id3v24)?;

    copy(&temp_file, file_path)?;
    // delete temp file
    std::fs::remove_file(temp_file)?;

    println!("After tag Update:");
    let new_tag = Tag::read_from_path(file_path).context("Failed to read tags")?;
    new_tag.title().map(|t| println!("  Title: {}", t));
    new_tag.artist().map(|a| println!("  Artist: {}", a));
    new_tag.album().map(|a| println!("  Album: {}", a));
    new_tag.genre().map(|g| println!("  Genre: {}", g));
    Ok(())
}

use std::path::Path;

#[derive(Debug)]
pub struct TagEditArgs {
    pub dir: Option<String>,
    pub path: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub genre: Option<String>,
}

impl TagEditArgs {
    pub fn new(dir: &str, path: &str, album: &str, artist: &str, title: &str, genre: &str) -> Self {
        TagEditArgs {
            dir: Some(dir.to_string()),
            path: path.to_string(),
            album: Some(album.to_string()),
            artist: Some(artist.to_string()),
            title: Some(title.to_string()),
            genre: Some(genre.to_string()),
        }
    }

    pub fn print_tags(&self) -> Result<()> {
        // println!("After tag Update:");
        let new_tag = Tag::read_from_path(&self.path).context("Failed to read tags")?;
        println!("Tags: {}", &self.path);
        new_tag.title().map(|t| println!("  Title: {}", t));
        new_tag.artist().map(|a| println!("  Artist: {}", a));
        new_tag.album().map(|a| println!("  Album: {}", a));
        new_tag.genre().map(|g| println!("  Genre: {}", g));

        Ok(())
    }

    // pub fn tag_all(&self) -> Result<()> {
    //     if let Some(directory) = &self.dir {
    //         let dir = read_dir(directory).with_context(|| "Failed to read directory")?;
    //         for entry in dir {
    //             let dir_entry = entry.context("Files to read file")?;
    //             let file_path = dir_entry.path().to_str();
    //             if let Some(file_path) = file_path {
    //                 self.path = file_path
    //             }

    //     }

    //     Ok(())
    // }
}

pub fn EditTags(args: &TagEditArgs) -> Result<()> {
    let file_path = Path::new(&args.path);
    if !file_path.exists() {
        return Err(anyhow!("File does not exist"));
    }
    let file_name = file_path
        .file_name()
        .context("Error getting filename")?
        .to_str()
        .context("Error converting filename to string")?;

    let temp_file = std::env::temp_dir().join(file_name);
    copy(file_path, &temp_file)?;

    let mut tag = Tag::read_from_path(&temp_file).context("Failed to read tags")?;

    if let Some(album) = &args.album {
        tag.set_album(album);
    }
    if let Some(artist) = &args.artist {
        tag.set_artist(artist);
    }
    if let Some(title) = &args.title {
        tag.set_title(title);
    }
    if let Some(genre) = &args.genre {
        tag.set_genre(genre);
    }

    tag.write_to_path(&temp_file, Version::Id3v24)?;
    copy(&temp_file, file_path)?;
    std::fs::remove_file(temp_file)?;

    Ok(())
}

// // Add to cli.rs
// .arg(
//     Arg::new("album")
//         .long("album")
//         .value_name("ALBUM")
//         .help("Set album name")
// )
// .arg(
//     Arg::new("artist")
//         .long("artist")
//         .value_name("ARTIST")
//         .help("Set artist name")
// )
// .arg(
//     Arg::new("title")
//         .long("title")
//         .value_name("TITLE")
//         .help("Set song title")
// )
// .arg(
//     Arg::new("genre")
//         .long("genre")
//         .value_name("GENRE")
//         .help("Set genre")
// )
