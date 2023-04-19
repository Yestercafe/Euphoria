use crate::parser::components::desc::Desc;

#[derive(Clone)]
pub struct Returns {
    pub desc: Option<Desc>,
}

impl Returns {
    pub fn new() -> Self {
        Self { desc: None }
    }
}
