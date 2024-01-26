pub struct File {
    pub name: String,
    pub content: String,
}

impl File {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }

    pub fn read(&self) {
        println!("Reading file'{}': \n {}", self.name, self.content)
    }
}
