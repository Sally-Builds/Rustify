use std::collections::HashMap;

fn database () {


#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

    
#[derive(Debug)]
struct User {
    full_name: String,
    age: u32,
    gender: Gender,
}

impl User {
     fn new(full_name: String, age: u32, gender: Gender) -> User {

        User { full_name, age, gender }
    }
}

    let mut db: HashMap<u32, User> = HashMap::new();

    let user = User::new(String::from("Joshua Uzoagulu"), 26, Gender::Male);

    db.insert(1, user);
}
