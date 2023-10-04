use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Mini Haskell Compiler", long_about = None)]
struct Args {
    /// AST Dump
    #[arg(long, action = clap::ArgAction::SetTrue)]
    ast: bool,
}

fn main() {
    let args = Args::parse();

    if args.ast {
        println!("TODO: AST dump");
    }
}
