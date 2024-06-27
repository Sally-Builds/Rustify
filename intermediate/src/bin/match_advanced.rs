enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Test {
    event: String,
    amount: i32,
}

#[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

struct Ticket {
    price: i32,
    ticket: TicketType,
}

fn main() {
    let discount = Discount::Percent(20);

    match discount {
        Discount::Percent(amount) => println!("The percentage discount is {:?}", amount),
        _others => println!("We dont care"), // instead of using _ to catch others we can make the name descriptive
                                             // _ => ()
    };

    let concert: Test = Test {
        event: "Concert".to_owned(),
        amount: 800,
    };

    match concert {
        Test { event, amount: 80 } => println!("event is {:?} and price is", event),
        Test { ref event, .. } if event == "Concerts" => println!("{:?}", event),
        Test { event, amount } => println!("event is {:?} and price is {:?}", event, amount),
    }

    let ticket1 = Ticket {
        price: 400,
        ticket: TicketType::Backstage("Joshua".to_owned()),
    };

    let arr: Vec<Ticket> = vec![
        ticket1,
        Ticket {
            price: 900,
            ticket: TicketType::Vip("Jane".to_owned()),
        },
        Ticket {
            price: 200,
            ticket: TicketType::Standard,
        },
    ];

    for ticket in &arr {
        match ticket {
            Ticket { price, ref ticket } => match ticket {
                TicketType::Backstage(data) => {
                    println!("Backstage: Name - {:?}, price - {:?}", data, price)
                }
                TicketType::Vip(data) => println!("Vip: Name - {:?}, price - {:?}", data, price),
                _ => println!("Standard: price {:?}", price),
            },
        }
    }
}
