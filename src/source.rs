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
}
