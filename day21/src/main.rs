use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::{File, OpenOptions, remove_file};
use std::env::temp_dir;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 1];
        rng.read_exact(&mut buffer)?;
        let file_dir = temp_dir();
        let file_path = format!("{}/temp_file_{}", file_dir.to_str().unwrap(), buffer[0]);
        let file_path = PathBuf::from(file_path);
        let file = File::create(&file_path)?;
        Ok(
            Self {
                file_path,
                file,
            }
        )
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write(data)?;
        Ok(())
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

fn main() {
    println!("No test for this day.");
}
