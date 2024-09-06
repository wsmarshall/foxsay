use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
//[1] this is command-line args def'n
struct Options {
    //sets default value for if no value entered by user
    #[clap(default_value = "Wa-pa-pa-pa-pa-pa-pow!")]
    /// What does the fox say?
    message: String,
    //adds an option for making the fox appear dead
    #[clap(short = 'd', long = "dead")]
    /// Make the fox appear dead
    dead: bool,
    //adds a simplified, didactic option for loading pic from a file
    #[clap(short = 'f', long = "file")]
    ///Load the fox picture from the specified file
    foxfile: Option<std::path::PathBuf>,
}

fn main() {
    //[2] uses the derived Parser
    let options = Options::parse(); //returns Options struct populated with parsed argument values
    let message = options.message;

    //provides an error print
    if message.to_lowercase() == "woof" {
        eprintln!("ERROR: A fox shouldn't bark like a dog.")
    }

    //set options for eyes
    let eye = if options.dead { "x" } else { "o" };

    match &options.foxfile {
        Some(path) => {
            let fox_template =
                std::fs::read_to_string(path).expect(&format!("could not read file {:?}", path));
            let eye = format!("{}", eye.green().bold());
            //format!() would need formatting string at compile time, but fox_template is loaded at runtime
            //so use String.replace()
            let fox_picture = fox_template.replace("{eye}", &eye);
            println!("{}", message.bright_red().on_bright_yellow());
            println!("{}", &fox_picture);
        }
        None => {
            println!("{}", message.bright_red().underline().on_bright_yellow());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye.green().bold());
            println!("    -( I )-");
        }
    }

    //printing priority for 1>, 2> commands: stdout, stderr
}
