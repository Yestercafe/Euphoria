use clap::{Arg, Command, Parser};

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
}
