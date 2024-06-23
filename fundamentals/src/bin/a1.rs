enum Flavours {
    Strawberry,
    Lemon
}

struct Drink {
    flavour: Flavours,
    ounce: i32,
}

fn print_me (drink: Drink) {
    let flavour: String;
    match drink.flavour {
        Flavours::Lemon => flavour = "Lemon Flavour".to_string(),
        Flavours::Strawberry => flavour = "Strawberry Flavour".to_string()
    }

    println!("The Flavour is {} and it weighs {}", flavour, drink.ounce)
}

fn main () {
    let flavour = Flavours::Lemon;
    match flavour {
        Flavours::Lemon => println!("Lemon Flavour"),
        Flavours::Strawberry => println!("Strawberry Flavour")
    }

    let drink =  Drink{
        ounce: 23,
        flavour: Flavours::Strawberry
    };

    print_me(drink)
}