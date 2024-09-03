use clap::Parser;

#[derive(Parser)]
//[1] this is command-line args def'n
struct Options {
    message: String,
}

fn main() {
    //[2] uses the derived Parser
    let options = Options::parse(); //returns Options struct populated with parsed argument values

    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    -( I )-");
}
