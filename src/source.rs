#[derive(Debug, Clone)]
pub struct Source {
    filename: String,
    ext: String,
    path: String,
}

impl Source {
    pub fn new(filename: String, ext: String, path: String) -> Self {
        Self {
            filename,
            ext,
            path,
        }
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn ext(&self) -> &str {
        &self.ext
    }
    pub fn path(&self) -> &str {
        &self.path
    }
}
