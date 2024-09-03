use clap::Parser;

#[derive(Parser)]
//this is command-line args def'n
struct Options {
    message: String,
}

fn main() {
    //uses the derived Parser
    let options = Options::parse(); //returns Options struct
                                    //populated with parsed argument values

    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    -( I )-");
}
