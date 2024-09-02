fn main() {
    let message = std::env::args()
        .nth(1) //needs expect since may call nonexistent index
        .expect("Missing the message for use in foxsay!");
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    -( I )-");
}
