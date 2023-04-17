#[derive(Clone)]
pub struct Returns {
    pub description: String,
}

impl Returns {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
