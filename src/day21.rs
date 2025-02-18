use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...

       let mut file_path = PathBuf::new();
       file_path.push(temp_dir());
       let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        file_path.push(format!("{}", time_stamp));
        let file = File::create(&file_path)?;
        Ok(Self{file_path, file})
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        // Your code here...
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

impl Drop for TempFile{
    fn drop(&mut self){
         let _ = std::fs::remove_file(&self.file_path);
    }
}


