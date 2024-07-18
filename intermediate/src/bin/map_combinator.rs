fn optional_number () -> Option<i32> {
    Some(18)
}

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();

    match  name.as_str() {
        "sam" => Some(1),
        "matt" => Some(6),
        "katie" => Some(9),
        _ => None,
    }
}

fn main () {
    let value = optional_number().map(|num| num * 2).map(|square| square + 1);

    match value {
        Some(num) => println!("{}", num),
        None => println!("nth")
    }

    let name = "katie";
    let user = find_user(name).map(|user_id| User {user_id, name: name.to_owned()});

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("User not found")
    }
}


