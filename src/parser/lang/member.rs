use crate::parser::components::desc::Desc;
use std::fmt::{Debug, Formatter};
use crate::parser::lang::any::Any;

pub struct Member {
    pub desc: Option<Desc>,
    pub has_uproperty: bool,
    pub declare: Option<String>,
    pub name: Option<String>,
}

impl Member {
    pub fn new() -> Self {
        Self {
            desc: None,
            has_uproperty: false,
            declare: None,
            name: None,
        }
    }

    pub fn from_any(any: Any) -> Self {
        let mut this_member = Self::new();
        this_member.desc = any.desc;
        this_member
    }
}

impl Debug for Member {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(desc) = &self.desc {
            f.write_str(format!("desc: {:?}\n", desc).as_str())?;
        }
        f.write_str(format!("has_uproperty: {}\n", self.has_uproperty).as_str())?;
        if let Some(declare) = &self.declare {
            f.write_str(format!("declare: {:?}\n", declare).as_str())?;
        }

        Ok(())
    }
}
