use crate::html_helper::HtmlHelper;
use crate::parser::lang::method::Method;
use crate::parser::parsers::cpp_parser::{CppParser, Parsed};
use crate::sh_pair::SHPair;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

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
            let header = HtmlHelper::str_header()
                + r#"<p><a href="./index.html">Back to index</a></p>"#
                + HtmlHelper::gen_heading(1, &self.doc_name).as_str();
            let mut toc = r#"<h2 class="heading2">TOC</h2>"#.to_string();
            let mut content = String::new();
            let footer = HtmlHelper::str_footer();

            content += HtmlHelper::str_member_list(&mut toc).as_str();
            for member in &parsed.members {
                // TODO: use id to generate toc
                let (member_str, _) = HtmlHelper::gen_member(&member, &mut toc);
                content += member_str.as_str();
            }

            content += HtmlHelper::str_method_list(&mut toc).as_str();
            for method in &parsed.methods {
                // TODO: use id to generate toc
                let (method_str, _) = HtmlHelper::gen_method(&method, &mut toc);
                content += method_str.as_str();
            }

            // TODO: handle exceptions
            doc_file.write(header.as_bytes()).unwrap();
            doc_file.write(toc.as_bytes()).unwrap();
            doc_file.write(content.as_bytes()).unwrap();
            doc_file.write(footer.as_bytes()).unwrap();
        }
    }
}
