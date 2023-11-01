mod input;
mod command;

fn main() {
    loop {
        let input = input::get_user_input("> ");
        command::handle_command(&input);
    }
}
