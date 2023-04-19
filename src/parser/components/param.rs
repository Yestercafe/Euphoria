use crate::parser::components::desc::Desc;
use std::fmt::{Debug, Formatter, Write};

#[derive(Clone)]
pub struct Param {
    pub name: Option<String>,
    pub desc: Option<Desc>,
}

impl Debug for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(format!("@name:\n{}\n", name).as_str())?;
        } else {
            f.write_str("@name:\nMISSING_PARAM_NAME\n")?;
        }
        if let Some(desc) = &self.desc {
            f.write_str(format!("@desc:\n{:?}", desc).as_str())?;
        }

        Ok(())
    }
}

impl Param {
    pub fn new() -> Self {
        Self {
            name: None,
            desc: None,
        }
    }
}
