mod doc;
mod generator;
mod html_helper;
mod md_helper;
mod parser;
mod sh_pair;
mod source;

use crate::generator::Generator;
use clap::Parser;
use glob::{glob, GlobResult};
use std::alloc::dealloc;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Euphoria Docs Generator", long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    destination: String,
}

fn main() {
    let args: Args = Args::parse();

    println!(
        "source = {}, destination = {}",
        args.source, args.destination
    );

    let root = glob(args.source.as_str()).expect("Failed to read glob pattern");
    let root: Vec<GlobResult> = root.collect();
    if root.len() == 0 {
        println!("Source dir doesn\'t exist.");
        return;
    }
    let root = root[0].as_ref().unwrap().display().to_string();

    let mut matches = args.source;
    matches.push_str("/**/*");

    let mut files: Vec<String> = vec![];

    for e in glob(matches.as_str()).expect("Failed to read glob pattern") {
        let filename: String = e.as_ref().unwrap().display().to_string();
        if filename.ends_with(".cpp") || filename.ends_with(".h") {
            files.push(filename);
        }
    }

    let generator = Generator::new(root, args.destination, files);
    generator.generate();
}
