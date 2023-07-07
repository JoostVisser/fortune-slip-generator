use std::path::PathBuf;

#[derive(Debug)]
pub struct WriteOptions {
    pub output_path: PathBuf,
    pub config_path: PathBuf,
}
