use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use crate::parser::lang::method::Method;
use crate::parser::parsers::cpp_parser::Status::Whatever;

pub struct CppParser {
    text: Vec<String>,
}

#[derive(Eq, PartialEq, Debug)]
enum Status {
    Desc,
    ReturnsDesc,
    ParamDesc,
    Signature,
    Comment,
    Whatever,
}

impl CppParser {
    pub fn new(text: Vec<String>) -> Self {
        Self {
            text,
        }
    }

    pub fn parse(self) -> Vec<Method> {
        let mut methods: Vec<Method> = vec![];
        let mut this_method = Method::new();
        let mut desc: String = String::new();
        let mut status = Status::Whatever;

        for line in self.text {
            // println!("parsing: {}, status: {:?}", line, status);
            match status {
                Status::Signature => {
                    this_method.signature = Some(line);
                    methods.push(this_method);
                    this_method = Method::new();
                    status = Whatever;
                    continue;
                }
                _ => {}
            }

            if line == "/**" {
                status = Status::Comment;
            } else if line.trim() == "*/" {
                status = Status::Signature;
            } else if line.starts_with(" *") {
                let mut line = line.clone().split_off(3.min(line.len()));
                if line.starts_with("@") {
                    if line.starts_with("@desc") || line.starts_with("@description") {
                        status = Status::Desc;
                    } else if line.starts_with("@param") || line.starts_with("@parameter") {
                        let mut param = Param::new();
                        let sp: Vec<&str> = line.split_whitespace().collect();
                        if sp.len() == 2 {
                            param.name = Some(sp[1].to_string());
                        }
                        this_method.params.push(param);
                        status = Status::ParamDesc;
                    } else if line.starts_with("@return") || line.starts_with("@returns") {
                        status = Status::ReturnsDesc;
                    }
                } else if line.trim().len() == 0 {
                    continue;
                } else {
                    match status {
                        Status::Desc => {
                            if let Some(desc) = this_method.desc {
                                let mut desc = desc.clone();
                                desc.description.push('\n');
                                desc.description.push_str(line.as_str());
                                this_method.desc = Some(desc);
                            } else {
                                this_method.desc = Some(Desc::new(line));
                            }
                        }
                        Status::ReturnsDesc => {
                            if let Some(returns) = this_method.returns {
                                let mut returns = returns.clone();
                                returns.description.push('\n');
                                returns.description.push_str(line.as_str());
                                this_method.returns = Some(returns);
                            } else {
                                this_method.returns = Some(Returns::new(line));
                            }
                        }
                        Status::ParamDesc => {
                            let mut last_param = this_method.params.pop().unwrap();
                            if let Some(desc) = last_param.description {
                                let mut desc = desc.clone();
                                desc.push('\n');
                                desc.push_str(line.as_str());
                                last_param.description = Some(desc);
                            } else {
                                last_param.description = Some(line);
                            }
                            this_method.params.push(last_param);
                        }
                        _ => {}
                    }
                }
            }
        }

        println!("methods length: {}", methods.len());

        methods
    }
}
