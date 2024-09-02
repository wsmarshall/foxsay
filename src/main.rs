fn main() {
    let message = std::env::args()
        .nth(1)
        .expect("Missing the message. Usage: foxsay <message>");
    println!(("{}"), message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("     ( I )");
}
