use super::file::File;
pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn list_files(&self) {
        println!("Files in directory '{}'", self.name);
        for file in &self.files {
            println!(" - '{}'", file.name);
        }
    }
}
