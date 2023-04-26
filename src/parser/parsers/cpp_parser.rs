use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use crate::parser::lang::member::Member;
use crate::parser::lang::method::Method;
use crate::parser::lang::r#enum::Enum;

pub struct CppParser {
    text: Vec<String>,
    source_path: String,
}

pub struct Parsed {
    pub members: Vec<Member>,
    pub methods: Vec<Method>,
    pub enums: Vec<Enum>,
}

impl Parsed {
    pub fn new() -> Self {
        Self {
            members: vec![],
            methods: vec![],
            enums: vec![],
        }
    }
}

impl CppParser {
    pub fn new(text: Vec<String>, source_path: String) -> Self {
        Self { text, source_path }
    }

    pub fn parse(&self) -> Parsed {
        let mut parsed = Parsed::new();
        let mut i = 0usize;

        while i < self.text.len() {
            // Find beginning of Euphoria docs
            if self.text[i] == "/**" {
                i += 1;

                // detect type
                if self.text[i].starts_with("* ") {
                    let mut line = self.text[i].clone().split_off(2);

                    // type identifier
                    if line.starts_with("%") {
                        match line.as_str() {
                            "%member" => {
                                let (next_i, new_member) = self.parse_member(i + 1);
                                parsed.members.push(new_member);
                                i = next_i - 1;
                            }
                            "%method" => {
                                let (next_i, new_method) = self.parse_method(i + 1);
                                parsed.methods.push(new_method);
                                i = next_i - 1;
                            }
                            // TODO: more, for example: enum
                            _ => {
                                // panic!("PARSER_ERROR: At line {} in source file {}, type `{}` is not supported.", i + 1, self.source_path, line);
                            }
                        }
                    } else {
                        // TODO: parse any, convert to specific type at last
                    }
                } else {
                    // TODO: curious part in docs, may do some process or panic?
                }
            }
            i += 1;
        }

        parsed
    }

    fn parse_member(&self, i: usize) -> (usize, Member) {
        let mut i = i;
        let mut this_member = Member::new();

        // Euphoria docs part
        while i < self.text.len() {
            // the end of docs
            if self.text[i] == "*/" {
                i += 1;
                break;
            }

            if self.text[i].starts_with("* ") {
                let line = self.text[i].clone().split_off(2);

                if line.starts_with("@") {
                    let sp: Vec<&str> = line.split_whitespace().collect();
                    let car = sp.first().unwrap();

                    match *car {
                        "@desc" | "@description" | "@brief" => {
                            let (next_i, desc) = self.get_desc(i + 1);
                            this_member.desc = Some(desc);
                            // next_i is the next line of the desc, -1 here, and +1 later,
                            // can be back to the right point
                            i = next_i - 1;
                        }
                        _ => {
                            self.panic_at_i(i);
                        }
                    }
                } else {
                    self.panic_at_i(i);
                }
            } else {
                // TODO: curious part in docs, may do some process or panic?
            }

            i += 1;
        }

        // UPROPERTY part
        self.assert_i(i);
        if self.text[i].trim().starts_with("UPROPERTY") {
            this_member.has_uproperty = true;
            // TODO: record it has which properties
            i += 1;
        }

        // declare part
        self.assert_i(i);
        let raw_declare = self.text[i].clone();
        this_member.declare = Some(raw_declare.clone());
        i += 1;
        // parse member declaration into member name
        let name = raw_declare
            .trim()
            .trim_end_matches(";")
            .trim_end_matches("{")
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .to_string();
        this_member.name = Some(name);

        (i, this_member)
    }

    fn parse_method(&self, i: usize) -> (usize, Method) {
        let mut i = i;
        let mut this_method = Method::new();

        while i < self.text.len() {
            // the end of docs
            if self.text[i] == "*/" {
                i += 1;
                break;
            }

            if self.text[i].starts_with("* ") {
                let line = self.text[i].clone().split_off(2);

                // component
                if line.starts_with("@") {
                    // sp is the argument list of component
                    let sp: Vec<&str> = line.split_whitespace().collect();
                    let car = sp.first().unwrap();

                    match *car {
                        "@desc" | "@description" | "@brief" => {
                            let (next_i, desc) = self.get_desc(i + 1);
                            this_method.desc = Some(desc);
                            i = next_i - 1;
                        }
                        "@returns" | "@return" => {
                            let (next_i, returns) = self.get_returns(i + 1);
                            this_method.returns = Some(returns);
                            i = next_i - 1;
                        }
                        "@param" | "@parameter" => {
                            let (next_i, mut param) = self.get_param(i + 1);
                            if sp.len() >= 2 {
                                param.name = Some(sp[1].to_string());
                            }
                            this_method.params.push(param);
                            i = next_i - 1;
                        }
                        _ => {
                            self.panic_at_i(i);
                        }
                    }
                } else {
                    self.panic_at_i(i);
                }
            } else {
                // TODO: curious part in docs, may do some process or panic?
            }

            i += 1;
        }

        // signature of the method
        self.assert_i(i);
        if self.text[i].trim().starts_with("UFUNCTION") {
            this_method.has_ufunction = true;
            // TODO: record it has which properties
            i += 1;
        }

        // declare part
        self.assert_i(i);
        let mut raw_signature = self.text[i].clone();
        this_method.signature = Some(raw_signature.clone());
        i += 1;

        // parse method signature into method name
        let name = raw_signature.trim().split("(").collect::<Vec<&str>>()[0]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        this_method.name = Some(name);

        (i, this_method)
    }

    fn assert_i(&self, i: usize) {
        if i >= self.text.len() {
            self.panic_at_i(i);
        }
    }

    fn panic_at_i(&self, i: usize) {
        let mut panic_info = format!(
            "PARSER_ERROR: At line {} in source file {}: \n",
            i + 1,
            self.source_path
        );
        let from = 2.max(i) - 2;
        let to = self.text.len().min(i + 3);
        panic_info.push_str("----------\n");
        for ii in from..i {
            panic_info.push_str(format!("    {}\n", self.text[ii]).as_str());
        }
        panic_info.push_str(format!(" => {}\n", self.text[i]).as_str());
        for ii in i + 1..to {
            panic_info.push_str(format!("    {}\n", self.text[ii]).as_str());
        }
        panic_info.push_str("----------\n");
        panic!("{}", panic_info);
    }

    fn get_desc(&self, i: usize) -> (usize, Desc) {
        let mut i = i;
        let mut this_desc = Desc::new(String::new());

        while i < self.text.len() {
            if self.text[i] == "*/" {
                break;
            }

            if self.text[i].starts_with("* ") {
                let mut line = self.text[i].clone().split_off(2);

                // if next components
                if line.starts_with("@") {
                    break;
                }

                // if be a line of the desc
                if !this_desc.description.is_empty() {
                    this_desc.description.push('\n');
                }
                // TODO: all line trimmed here, may support indents and formats
                this_desc.description.push_str(line.trim());
            } else {
                // TODO: curious part in docs, may do some process or panic?
            }

            i += 1;
        }

        this_desc.description = this_desc.description.trim().to_string();

        (i, this_desc)
    }

    fn get_returns(&self, i: usize) -> (usize, Returns) {
        let (next_i, desc) = self.get_desc(i);
        let mut this_returns = Returns::new();
        this_returns.desc = Some(desc);
        (next_i, this_returns)
    }

    fn get_param(&self, i: usize) -> (usize, Param) {
        let (next_i, desc) = self.get_desc(i);
        let mut this_param = Param::new();
        this_param.desc = Some(desc);
        (next_i, this_param)
    }
}
