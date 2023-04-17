#[derive(Clone, Debug)]
pub struct Desc {
    pub description: String,
}

impl Desc {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
