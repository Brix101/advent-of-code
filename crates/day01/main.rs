use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <day> <part>", args[0]);
        std::process::exit(1);
    }

    // Extract day and part from command-line arguments
    let day = &args[1];
    let part = &args[2];

    // Process and print the provided day and part
    println!("Day: {}", day);
    println!("Part: {}", part);
}
