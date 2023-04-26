use crate::doc::Doc;
use crate::html_helper::HtmlHelper;
use crate::sh_pair::SHPair;
use crate::source::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Generator {
    root: String,
    dest_path: String,
    source_paths: Vec<String>,
}

impl Generator {
    pub fn new(root: String, dest_path: String, source_paths: Vec<String>) -> Self {
        Self {
            root,
            dest_path,
            source_paths,
        }
    }

    pub fn generate(&self) {
        println!("generate:");
        println!("root: {}", self.root);

        let prefix_length = if self.root != "." {
            self.root.len() + 1
        } else {
            0
        };

        let mut docs_map: HashMap<String, SHPair> = HashMap::new();

        for source_path in &self.source_paths {
            let mut split = source_path.clone();
            let (_, source_filename) = split.split_at_mut(prefix_length);
            let path_from_root_to_file: Vec<&str> = source_filename
                .split(|c: char| c == '/' || c == '\\')
                .collect();
            let source_filename = path_from_root_to_file.last().unwrap().to_string();
            let file_ext = source_filename
                .split(".")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string();
            if file_ext == "cpp" {
                let mut doc_name = source_filename.clone();
                let _ = doc_name.split_off(doc_name.len() - "cpp".len() - 1);
                if let Some(sh_pair) = docs_map.get_mut(&doc_name) {
                    sh_pair.source = Some(Source::new(
                        source_filename,
                        "cpp".to_string(),
                        source_path.clone(),
                    ));
                } else {
                    let mut new_sh_pair = SHPair::new();
                    new_sh_pair.source = Some(Source::new(
                        source_filename,
                        "cpp".to_string(),
                        source_path.clone(),
                    ));
                    docs_map.insert(doc_name.clone(), new_sh_pair);
                }
            } else if file_ext == "h" {
                let mut doc_name = source_filename.clone();
                let _ = doc_name.split_off(doc_name.len() - "h".len() - 1);
                if let Some(sh_pair) = docs_map.get_mut(&doc_name) {
                    sh_pair.header = Some(Source::new(
                        source_filename,
                        "h".to_string(),
                        source_path.clone(),
                    ));
                } else {
                    let mut new_sh_pair = SHPair::new();
                    new_sh_pair.header = Some(Source::new(
                        source_filename,
                        "h".to_string(),
                        source_path.clone(),
                    ));
                    docs_map.insert(doc_name.clone(), new_sh_pair);
                }
            }
        }

        // generate index page
        self.generate_index(&docs_map);
        self.generate_css();

        // generate subpages
        for (key, val) in docs_map.iter() {
            println!("{}, {:?}", key, val);
            let doc = Doc::new(key.clone(), self.dest_path.clone(), val.clone());
            doc.generate();
        }
    }

    fn generate_index(&self, docs_map: &HashMap<String, SHPair>) {
        let dest_path = Path::new(self.dest_path.as_str());
        if dest_path.exists() && dest_path.is_file() {
            panic!(
                "FS_ERROR: {} is a file, not a directory or doesn't exist.",
                self.dest_path
            );
        } else if !dest_path.exists() {
            if let Err(_) = std::fs::create_dir_all(dest_path) {
                panic!("FS_ERROR: can't create the directory {}", self.dest_path);
            }
        }

        let index_file_path = self.dest_path.clone() + "//index.html";
        let index_file = Path::new(index_file_path.as_str());
        if index_file.exists() {
            std::fs::remove_file(index_file).unwrap();
        }

        let index_file = File::create(index_file_path.as_str());
        if let Err(_) = index_file {
            panic!("FS_ERROR: cannot create file {}", index_file_path);
        } else if let Ok(mut index_file) = index_file {
            // TODO: handle exceptions
            index_file.write(HtmlHelper::str_header().as_bytes());

            let mut sorted_docs_map: Vec<(&String, &SHPair)> = vec![];

            for (k, v) in docs_map.iter() {
                sorted_docs_map.push((&k, &v));
            }

            sorted_docs_map.sort_by(|a, b| a.0.cmp(b.0));
            for (k, v) in sorted_docs_map.iter() {
                index_file.write(
                    HtmlHelper::gen_url((k.to_string() + ".html").as_str(), k.as_str()).as_bytes(),
                );
            }

            index_file.write(HtmlHelper::str_footer().as_bytes());
        }
    }

    fn generate_css(&self) {
        // TODO: remove unwrap, handle exception
        let dest_css = self.dest_path.clone() + "//euphoria.css";
        // std::fs::remove_file(dest_css.as_str()).unwrap();
        std::fs::copy("./styles/euphoria.css", dest_css.as_str()).unwrap();
    }
}
