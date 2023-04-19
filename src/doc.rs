use crate::parser::lang::method::Method;
use crate::parser::parsers::cpp_parser::{CppParser, Parsed};
use crate::sh_pair::SHPair;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::html_helper::HtmlHelper;

pub struct Doc {
    doc_name: String,
    dest_path: String,
    sh_pair: SHPair,
}

impl Doc {
    pub fn new(doc_name: String, dest_path: String, sh_pair: SHPair) -> Self {
        Self {
            doc_name,
            dest_path,
            sh_pair,
        }
    }

    pub fn generate(self) {
        // parse .cpp file
        // deprecate: parse .cpp file, because there is always no doc comments in UE C++

        // parse .h file
        if let Some(header_file) = &self.sh_pair.header {
            let file = File::open(header_file.path()).unwrap();
            let buffered = BufReader::new(file);

            let mut text: Vec<String> = vec![];
            for line in buffered.lines() {
                text.push(line.unwrap().trim().to_string());
            }

            let cpp_parser = CppParser::new(text, header_file.path().to_string());

            let result = cpp_parser.parse();

            self.write_out_to_html(result);
        }
    }

    fn write_out_to_html(self, parsed: Parsed) {
        let doc_file_path = self.dest_path + "//" + self.doc_name.as_str() + ".html";
        let doc_file = Path::new(doc_file_path.as_str());
        if doc_file.exists() {
            std::fs::remove_file(doc_file).unwrap();
        }

        let doc_file = File::create(doc_file_path.as_str());
        if let Err(_) = doc_file {
            panic!("FS_ERROR: cannot create file {}", doc_file_path);
        } else if let Ok(mut doc_file) = doc_file {
            // TODO: handle exceptions
            doc_file.write(HtmlHelper::str_header().as_bytes());

            doc_file.write(HtmlHelper::gen_heading(1, &self.doc_name).as_bytes());

            doc_file.write(HtmlHelper::str_member_list().as_bytes());
            for member in &parsed.members {
                // TODO: use id to generate toc
                let (member_str, _) = HtmlHelper::gen_member(&member);
                doc_file.write(member_str.as_bytes());
            }

            doc_file.write(HtmlHelper::str_method_list().as_bytes());
            for method in &parsed.methods {
                doc_file.write(HtmlHelper::gen_method(&method).as_bytes());
            }

            doc_file.write(HtmlHelper::str_footer().as_bytes());
        }
    }
}
