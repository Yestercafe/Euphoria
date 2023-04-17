use crate::parser::components::desc::Desc;

pub struct Enum {
    pub desc: Option<Desc>,
    pub name: Option<String>,
}

impl Enum {
    pub fn new() -> Self {
        Self {
            desc: None,
            name: None,
        }
    }
}
