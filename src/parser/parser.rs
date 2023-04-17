pub struct Parser {
    text: String,
    ext: String,
}

impl Parser {
    pub fn new(text: String, ext: String) -> Self {
        Self {
            text,
            ext,
        }
    }
}
