use clap::Parser;

#[derive(Parser)]
//[1] this is command-line args def'n
struct Options {
    //sets default value for if no value entered by user
    #[clap(default_value = "Wa-pa-pa-pa-pa-pa-pow!")]
    /// What does the fox say?
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
