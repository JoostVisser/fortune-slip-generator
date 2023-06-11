use std::{
    fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

pub struct TempFile {
    pub dir: TempDir,
    pub path: PathBuf,
}

pub fn create_temp_file<P: AsRef<Path>>(file_path: P, file_contents: &str) -> TempFile {
    let temp_dir = TempDir::new().unwrap();
    let temp_file_path = temp_dir.path().join(file_path);

    fs::write(&temp_file_path, file_contents).unwrap();

    TempFile {
        dir: temp_dir,
        path: temp_file_path,
    }
}
