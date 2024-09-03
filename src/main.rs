use clap::Parser;

#[derive(Parser)]
//
struct Options {
    message: String,
}

fn main() {
    let options = Options::parse(); //
    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    -( I )-");
}
