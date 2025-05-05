
pub struct MockFile {
    file_name: String,
}

impl MockFile {
    pub fn open(file_name: String) -> MockFile {
        println!("File initialized");
        MockFile { file_name }
    }
}

impl Drop for MockFile {
    fn drop(&mut self) {
        println!("Closing File with file name: {}", self.file_name);
    }
}


