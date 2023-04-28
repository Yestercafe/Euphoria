use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use crate::parser::lang::term_type::TermType;

pub struct Any {
    pub desc: Option<Desc>,
    pub returns: Option<Returns>,
    pub params: Vec<Param>,
    pub term_type: TermType,
}

impl Any {
    pub fn new() -> Self {
        Self {
            desc: None,
            returns: None,
            params: vec![],
            term_type: TermType::Null,
        }
    }
}
