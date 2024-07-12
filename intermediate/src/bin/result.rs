#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: String, age: i32) -> Result<Self, String> {
        match age >= 21 {
            true => return Ok(Self {name, age}),
            false => return Err("User Not an Adult".to_owned())
        };
    }
}

// fn print_msg (person: &)

fn main () {
    let adultOne = Adult::new("Sally".to_owned(), 21);
    let adultTwo = Adult::new("Iyida".to_owned(), 17);

    println!("{:?}", adultOne);
    println!("{:?}", adultTwo);
}