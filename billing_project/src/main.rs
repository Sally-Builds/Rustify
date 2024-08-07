// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io::{self, Write};

enum Menu {
    ADD,
    VIEW,
    EXIT,
}

impl Menu {
    fn print(&self) {
        match self {
            Self::ADD => println!("1) Add Bill"),
            Self::VIEW => println!("2) View Bills"),
            Self::EXIT => println!("3) Quit")
        }
    }
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: i32,
}


fn print_menu () {
    println!("Please choose an option below");
    println!("1) Add Bill");
    println!("2) View Bills");
    println!("3) Exit");
}

fn add(bills: &mut Vec<Bill>){
    print!("Enter Bill name: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error getting reading input");

    let name = buffer.trim().to_string();

    print!("Enter Bill amount: ");
    io::stdout().flush().unwrap();
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer2).expect("Error getting reading input");

    let amount: i32 = match buffer2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Provide a number");
            return
        }
    };

    let bill = Bill{name, amount};

    bills.push(bill);
    println!("bill added");
}


fn main() {


    let mut bills: Vec<Bill> = vec![];


    loop {
        print_menu();

        let mut buffer = String::new();    
        io::stdin().read_line(&mut buffer).expect("Error reading file");  

        let menu_option: i32 = match buffer.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid Option");
                continue;
            }
        };

        match menu_option {
            1 => add(&mut bills),
            2 => println!("{:?}", bills),
            3 => {
                println!("Application Shutting Down...");
                return;
            },
            _ => {
                println!("Invalid Option");
                continue;
            }
        }
    }

}
