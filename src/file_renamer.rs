use anyhow::{Context, Result};
use log::{debug, info, warn};
use regex::Regex;
use std::fs::{read_dir, rename};
use std::path::Path;

pub struct FileRenamer {
    pub file_type: String,
    pub current: String,
    pub new: String,
}

impl FileRenamer {
    /// Rename a single file based on the new string
    pub fn rename_single_file(&self, file_path: &Path) -> Result<()> {
        let file_name = file_path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to get file name"))?;

        // Skip files that are not of the specified file type
        let file_extension = file_path.extension();
        if file_extension != Some(self.file_type.as_ref()) {
            debug!(
                "Skipping file: {} with {}",
                file_name,
                file_extension.unwrap().to_str().unwrap()
            );
            warn!("Skipping file: {}", file_name);
            return Ok(());
        }

        // Regex match a string and replace it with another string
        let re = Regex::new(&self.current).with_context(|| "Failed to create regex")?;

        if !re.is_match(file_name) {
            warn!(
                "File name does not match current {} string: {}",
                self.current, file_name
            );
            return Ok(());
        }
        let new_name = file_name.replace(&self.current, &self.new);
        info!("Renaming file: {} to {}", file_name, new_name);

        rename(file_path, file_path.with_file_name(new_name))
            .with_context(|| format!("Failed to rename file: {}", file_name))?;

        Ok(())
    }

    /// Rename files in a directory based on the new string
    pub fn rename_files_in_directory(&self, path: &str) -> Result<()> {
        let dir = read_dir(path).with_context(|| "Failed to read directory")?;
        for entry in dir {
            let entry = entry.context("Failed to read entry")?;
            let path = entry.path();

            self.rename_single_file(&path)?;
        }
        info!("Renaming completed successfully");
        Ok(())
    }
}

#[cfg(test)]
mod rename_tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn create_test_files(
        dir_path: &Path,
        file_names: &[&str],
    ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        let mut file_paths = Vec::new();
        for file_name in file_names {
            let file_path = dir_path.join(file_name);
            let mut file = File::create(&file_path)?;
            writeln!(file, "Hello, world!")?;
            file_paths.push(file_path);
        }
        Ok(file_paths)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;
        use tempfile::tempdir;

        fn create_test_files(
            dir_path: &Path,
            file_names: &[&str],
        ) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
            let mut file_paths = Vec::new();
            for file_name in file_names {
                let file_path = dir_path.join(file_name);
                let mut file = File::create(&file_path)?;
                writeln!(file, "Hello, world!")?;
                file_paths.push(file_path);
            }
            Ok(file_paths)
        }

        #[test]
        fn test_rename_all_files() -> Result<()> {
            let dir = tempdir()?;
            let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
            let created_files = create_test_files(dir.path(), &file_names)?;

            let file_renamer = FileRenamer {
                file_type: "mp3".to_string(),
                current: "file".to_string(),
                new: "new".to_string(),
            };
            file_renamer.rename_files_in_directory(dir.path().to_str().unwrap())?;

            for file in created_files {
                assert!(!(file).exists());
            }
            assert!(dir.path().join("new1.mp3").exists());
            assert!(dir.path().join("new2.mp3").exists());
            assert!(dir.path().join("new3.mp3").exists());

            dir.close()?;
            Ok(())
        }

        #[test]
        fn test_rename_files_with_invalid_file_type() -> Result<()> {
            let dir = tempdir()?;
            let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
            let created_files = create_test_files(dir.path(), &file_names)?;

            let file_renamer = FileRenamer {
                file_type: "txt".to_string(),
                current: "file".to_string(),
                new: "new".to_string(),
            };
            file_renamer.rename_files_in_directory(dir.path().to_str().unwrap())?;

            for file in created_files {
                assert!((file).exists());
            }

            dir.close()?;
            Ok(())
        }

        #[test]
        fn test_rename_some_files() -> Result<()> {
            let dir = tempdir()?;
            let file_names = ["no_rename1.mp3", "file2.mp3", "in_file_between.mp3"];
            let _created_files = create_test_files(dir.path(), &file_names)?;

            let file_renamer = FileRenamer {
                file_type: "mp3".to_string(),
                current: "file".to_string(),
                new: "new".to_string(),
            };
            file_renamer.rename_files_in_directory(dir.path().to_str().unwrap())?;

            assert!(dir.path().join("no_rename1.mp3").exists()); // should not be renamed
            assert!(dir.path().join("new2.mp3").exists());
            assert!(dir.path().join("in_new_between.mp3").exists()); // in between string should be renamed

            dir.close()?;
            Ok(())
        }

        #[test]
        fn test_with_invalid_file_name() -> Result<()> {
            let dir = tempdir()?;
            let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
            let created_files = create_test_files(dir.path(), &file_names)?;

            let file_renamer = FileRenamer {
                file_type: "mp3".to_string(),
                current: "new".to_string(),
                new: "new".to_string(),
            };
            file_renamer.rename_files_in_directory(dir.path().to_str().unwrap())?;

            for file in created_files {
                assert!((file).exists());
            }

            dir.close()?;
            Ok(())
        }

        #[test]
        fn test_with_invalid_path() -> Result<()> {
            let dir = tempdir()?;
            let file_names = ["file1.mp3", "file2.mp3", "file3.mp3"];
            let created_files = create_test_files(dir.path(), &file_names)?;

            let file_renamer = FileRenamer {
                file_type: "mp3".to_string(),
                current: "file".to_string(),
                new: "new".to_string(),
            };
            let res = file_renamer.rename_files_in_directory("invalid_path");
            assert!(res.is_err());

            for file in created_files {
                assert!((file).exists());
            }

            dir.close()?;
            Ok(())
        }
    }
}
