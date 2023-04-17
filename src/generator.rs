use crate::doc::Doc;
use crate::sh_pair::SHPair;
use crate::source::Source;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    root: String,
    source_paths: Vec<String>,
}

impl Generator {
    pub fn new(root: String, source_paths: Vec<String>) -> Self {
        Self { root, source_paths }
    }

    pub fn generate(self) {
        println!("generate:");
        println!("root: {}", self.root);

        let prefix_length = if self.root != "." {
            self.root.len() + 1
        } else {
            0
        };

        let mut docs_map: HashMap<String, SHPair> = HashMap::new();

        for source_path in self.source_paths {
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
                    sh_pair.source =
                        Some(Source::new(source_filename, "cpp".to_string(), source_path));
                } else {
                    let mut new_sh_pair = SHPair::new();
                    new_sh_pair.source =
                        Some(Source::new(source_filename, "cpp".to_string(), source_path));
                    docs_map.insert(doc_name.clone(), new_sh_pair);
                }
            } else if file_ext == "h" {
                let mut doc_name = source_filename.clone();
                let _ = doc_name.split_off(doc_name.len() - "h".len() - 1);
                if let Some(sh_pair) = docs_map.get_mut(&doc_name) {
                    sh_pair.header =
                        Some(Source::new(source_filename, "h".to_string(), source_path));
                } else {
                    let mut new_sh_pair = SHPair::new();
                    new_sh_pair.header =
                        Some(Source::new(source_filename, "h".to_string(), source_path));
                    docs_map.insert(doc_name.clone(), new_sh_pair);
                }
            }
        }

        for (key, val) in docs_map.iter() {
            println!("{}, {:?}", key, val);
            let doc = Doc::new(key.clone(), val.clone());
            doc.generate();
        }
    }
}
