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
        // parse .cpp file
        // deprecate: parse .cpp file, because there is always no doc comments in UE C++

        // parse .h file
        if let Some(header_file) = self.sh_pair.header {
            let file = File::open(header_file.path()).unwrap();
            let buffered = BufReader::new(file);

            let mut text: Vec<String> = vec![];
            for line in buffered.lines() {
                text.push(line.unwrap().trim().to_string());
            }

            let cpp_parser = CppParser::new(text, header_file.path().to_string());

            let result = cpp_parser.parse();

            println!("result.members length: {}", result.members.len());
            for member in result.members {
                println!("%member\n{:?}", member);
            }

            println!("result.methods length: {}", result.methods.len());
            for method in result.methods {
                println!("%method\n{:?}", method);
            }
        }
    }

    fn write_out_methods(methods: Vec<Method>) {
        for method in methods {}
    }
}
