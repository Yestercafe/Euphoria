mod generator;
mod source;
mod sh_pair;
mod doc;
mod parser;

use clap::Parser;
use glob::{glob, GlobResult};
use crate::generator::Generator;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Euphoria Docs Generator", long_about = None)]
struct Args {
    #[arg(short, long)]
    language: String,

    #[arg(short, long)]
    path: String,
}

fn main() {
    let args: Args = Args::parse();

    println!("lang = {}, path = {}", args.language, args.path);

    let root = glob(args.path.as_str()).expect("Failed to read glob pattern");
    let root: Vec<GlobResult> = root.collect();
    if root.len() == 0 {
        println!("Dir doesn\'t exist.");
        return;
    }
    let root = root[0].as_ref().unwrap().display().to_string();

    let mut matches = args.path;
    matches.push_str("/**/*");

    let mut files : Vec<String> = vec![];

    for e in glob(matches.as_str()).expect("Failed to read glob pattern") {
        let filename: String = e.as_ref().unwrap().display().to_string();
        if filename.ends_with(".cpp") || filename.ends_with(".h") {
            files.push(filename);
        }
    }

    let generator = Generator::new(root, files);
    generator.generate();
}
