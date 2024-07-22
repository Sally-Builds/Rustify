// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<Self> {
        match state.trim().to_lowercase().as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}

fn print_msg (state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Turning Off"),
        Sleep => println!("Sleeping in..."),
        Reboot => println!("Rebooting in 60 seconds"),
        Shutdown => println!("Shutting down..."),
        Hibernate => println!("Hibernating...")
    }
}

fn get_input () -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}

fn main() {
    println!("Enter Powerstate: ");
    let input = get_input();

    match input {
        Ok(state) => {
            match PowerState::new(&state) {
                Some(data) => print_msg(data),
                None => println!("Invalid Keyword")
            }
        },
        Err(e) => println!("Error: {:?}", e)
    }
}
