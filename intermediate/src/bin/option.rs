struct User {
    email: String,
    age: Option<i32>
}

fn main () {
    let customer = User {
        email: "uzoagulujoshua@yahoo.com".to_owned(),
        age: Some(80),
    };

    match customer.age {
        Some(data) => println!("yea {:?}", data),
        None => println!("brha")
    };
}