fn main () {
    let isUser = Some("user");

    match isUser {
        Some(user) => println!("{:?}", user),
        None => println!("none")
    }

    if let Some(user) = isUser {
        println!("user");
    }else {
        println!("NOne")
    }
}