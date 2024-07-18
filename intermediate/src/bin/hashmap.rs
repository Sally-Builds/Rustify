use std::collections::HashMap;


struct Contents {
    content: String
}
fn main () {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents{content: "me".to_owned()});
    lockers.insert(2, Contents{content: "you".to_owned()});

    for(locker_no, content) in lockers.iter() {
        println!("{}: {}", locker_no, content.content)
    }

    lockers.insert(5, Contents{content: "Bro".to_owned()});

    let mut stock: HashMap<String, i32> = HashMap::new();
    let mut total_stock: i32 = 0;

    stock.insert("Chairs".to_owned(), 5);
    stock.insert("Beds".to_owned(),3);
    stock.insert("Tables".to_owned(), 2);
    stock.insert("Couches".to_owned(), 0);

    for (furniture, stock) in stock.iter() {
        if *stock == 0 {
            println!("furniture name {}, stock amount - out of stock", furniture)
        }else {
            println!("furniture name {}, stock amount - {}", furniture, stock)
        }
        total_stock += *stock;
    }

    println!("total amount in stock {:?}", total_stock)
}