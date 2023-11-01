pub fn handle_command(command: &str) {
    match command {
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
