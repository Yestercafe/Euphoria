use clap::arg;
use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use crate::parser::lang::any::Any;
use crate::parser::lang::member::Member;
use crate::parser::lang::method::Method;
use crate::parser::lang::r#enum::Enum;
use crate::parser::lang::term_type::TermType;

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
                let (next_i, new_any) = self.parse_any(i + 1);
                i = next_i;
                match new_any.term_type {
                    TermType::Member => parsed = self.attach_member(parsed, new_any, i),
                    TermType::Method => parsed = self.attach_method(parsed, new_any, i),
                    TermType::Null => panic!("TermType::Null"),
                    TermType::Undecidable => panic!("TermType::Undecidable"),
                    _ => todo!(),
                }
            }
            i += 1;
        }

        parsed
    }

    fn parse_any(&self, i: usize) -> (usize, Any) {
        let mut i = i;
        let mut this_any = Any::new();

        while i < self.text.len() {
            // the end of docs
            if self.text[i] == "*/" {
                i += 1;
                break;
            }

            if self.text[i].starts_with("* ") {
                let line = self.text[i].clone().split_off(2);
                if line.starts_with("@") {
                    let sp = line.split_once(" ");
                    let mut instruction: Option<&str> = None;
                    let mut arguments: Option<&str> = None;

                    if let Some((car, cdr)) = sp {
                        instruction = Some(car);
                        arguments = Some(cdr);
                    } else {
                        instruction = Some(&line);
                    }

                    if let Some(instruction) = instruction {
                        match instruction {
                            "@brief" | "@desc" | "@description" => {
                                let (next_i, mut desc) = self.get_desc(i + 1);
                                if let Some(arguments) = arguments {
                                    if desc.description.len() == 0 {
                                        desc.description = arguments.trim().to_string();
                                    } else {
                                        desc.description = arguments.trim().to_string() + "\n" + desc.description.as_str();
                                    }
                                }
                                this_any.desc = Some(desc);
                                i = next_i - 1;
                            }
                            "@returns" | "@return" => {
                                let (next_i, mut returns) = self.get_returns(i + 1);
                                if let Some(arguments) = arguments {
                                    if let Some(desc) = returns.desc {
                                        if desc.description.len() == 0 {
                                            returns.desc = Some(Desc::new(arguments.trim().to_string()));
                                        } else {
                                            let mut new_desc = Desc::new(arguments.trim().to_string() + "\n" + desc.description.as_str());
                                            new_desc.description = new_desc.description.trim().to_string();
                                            returns.desc = Some(new_desc);
                                        }
                                    } else {
                                        returns.desc = Some(Desc::new(arguments.trim().to_string()));
                                    }
                                }
                                this_any.returns = Some(returns);
                                i = next_i - 1;
                            }
                            "@param" | "@parameter" => {
                                let (next_i, mut param) = self.get_param(i + 1);
                                if let Some(arguments) = arguments {
                                    let arg0s = arguments.split_once(" ");
                                    if let Some((arg0, arg1)) = arg0s {
                                        param.name = Some(arg0.trim().to_string());
                                        if let Some(desc) = param.desc {
                                            if desc.description.len() == 0 {
                                                param.desc = Some(Desc::new(arg1.trim().to_string()));
                                            } else {
                                                let mut new_desc = Desc::new(arg1.trim().to_string() + "\n" + desc.description.as_str());
                                                new_desc.description = new_desc.description.trim().to_string();
                                                param.desc = Some(new_desc);
                                            }
                                        } else {
                                            param.desc = Some(Desc::new(arg1.trim().to_string()));
                                        }
                                    } else {
                                        param.name = Some(arguments.trim().to_string());
                                    }
                                }
                                this_any.params.push(param);
                                i = next_i - 1;
                            }
                            _ => self.panic_at_i(i),
                        }
                    } else {
                        panic!("Never reach")
                    }
                }
            } else {
                // TODO: curious part in docs, may do some process or panic?
            }

            i += 1;
        }

        // Detect term types
        self.assert_i(i);
        if self.text[i].trim().starts_with("UFUNCTION") {
            this_any.term_type = TermType::Method;
        } else if self.text[i].trim().starts_with("UPROPERTY") {
            this_any.term_type = TermType::Member;
        } else {
            if self.text[i].trim().trim_end_matches(";").trim().trim_end_matches("const").trim().chars().last().unwrap() == ')' {
                this_any.term_type = TermType::Method;
            } else {
                this_any.term_type = TermType::Member;
            }
        }
        // Do not increase `i`, postprocess in attach_*

        (i, this_any)
    }

    fn attach_member(&self, parsed: Parsed, this_any: Any, i: usize) -> Parsed {
        let mut parsed = parsed;
        let mut i = i;
        let mut this_member = Member::from_any(this_any);

        self.assert_i(i);
        if self.text[i].trim().starts_with("UPROPERTY") {
            this_member.has_uproperty = true;
            i += 1;
        }
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

        parsed.members.push(this_member);
        parsed
    }

    fn attach_method(&self, parsed: Parsed, this_any: Any, i: usize) -> Parsed {
        let mut parsed = parsed;
        let mut i = i;
        let mut this_method = Method::from_any(this_any);

        self.assert_i(i);
        if self.text[i].trim().starts_with("UFUNCTION") {
            this_method.has_ufunction = true;
            i += 1;
        }
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

        parsed.methods.push(this_method);
        parsed
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
