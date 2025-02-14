enum Menu {
    Add,
    View
}

struct Bill {
    name: String,
    amount: u32,
}

impl  Bill {
    fn new(name: String, amount: u32) -> Bill {
        Bill { name, amount }
    }
}

fn print_menu() {
    println!("");
    println!("== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("");
    println!("Enter Selection: ")
}

fn add_bill(vect: &mut Vec<Bill>) {
    let mut bill_name = String::new();
    let mut bill_amount = String::new();

    loop {
        println!("Enter Bill name: ");
        let _ = std::io::stdin().read_line(&mut bill_name);
    
        println!("Enter Bill amount: ");
        let _ = std::io::stdin().read_line(&mut bill_amount);
    
        let bill_name = bill_name.trim();
        let bill_amount = match bill_amount.trim().parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid amount");
                continue;
            }
        };
        let bill = Bill::new(bill_name.to_owned(), bill_amount);
        vect.push(bill);
        println!("Bill Added Successfully");
        break;
    }

}


fn view_bill(vect: &Vec<Bill>) {
    if vect.is_empty() {
       return println!("Bills is empty")
    }

    println!("== Bill List ==");
    for v in vect {
        println!("Name: {}, Amount: {}", v.name, v.amount);
    }
}

pub fn entry_point() {
    let mut bills:Vec<Bill> = Vec::new();
    
    loop {
        print_menu();

        let mut s = String::new();

        let _ = std::io::stdin().read_line(&mut s);

        let num = s.trim().parse::<u32>();

        let num = match num {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a number menu");
                // panic!("Enter a valid bill menu");
                // return entry_point();
                continue;
            }
        };

        let menu_item = match num {
            1 => Menu::Add,
            2 => Menu::View,
            _ => {
                println!("Menu not found");
                // return entry_point();
                continue;
            }
        };


        match menu_item {
            Menu::Add => add_bill(&mut bills),
            Menu::View => view_bill(&bills),
        }
    }

}