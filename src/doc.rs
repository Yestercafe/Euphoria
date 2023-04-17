use crate::parser::lang::method::Method;
use crate::parser::parsers::cpp_parser::CppParser;
use crate::sh_pair::SHPair;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Doc {
    doc_name: String,
    sh_pair: SHPair,
}

impl Doc {
    pub fn new(doc_name: String, sh_pair: SHPair) -> Self {
        Self { doc_name, sh_pair }
    }

    pub fn generate(self) {
        if let Some(source_file) = self.sh_pair.source {
            let source_file = File::open(source_file.path()).unwrap();
            let buffered = BufReader::new(source_file);

            let mut text: Vec<String> = vec![];
            for line in buffered.lines() {
                text.push(line.unwrap());
            }

            let cpp_parser = CppParser::new(text);
            let methods = cpp_parser.parse();

            for method in methods {
                println!("{:?}", method);
            }
        }
    }

    fn write_out_methods(methods: Vec<Method>) {
        for method in methods {}
    }
}
