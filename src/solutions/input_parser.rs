pub struct InputParser {
    content: String,
}

impl InputParser {
    pub fn new(filename: &str) -> Self {
        Self {
            content: std::fs::read_to_string(filename).expect("can't find file"),
        }
    }

    pub fn get_string_vec(&self) -> Vec<&str> {
        self.content.split("\r\n").collect()
    }
}
