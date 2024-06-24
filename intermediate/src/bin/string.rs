struct Person {
    name: String,
    age: i32,
    color: String,
}

impl Person {
    fn print(&self) {
        println!("Name is {:?} and favorite color is {:?}", self.name, self.color);
    }
}

fn main () {
    let persons = vec![
        Person{
            name: String::from("Joshua"),
            age: 19,
            color: "Blue".to_owned(),
        },
        Person{
            name: String::from("Chidimma"),
            age: 8,
            color: "White".to_owned(),
        },
        Person{
            name: String::from("Treasure"),
            age: 23,
            color: "Pink".to_owned(),
        },
    ];

    for person in &persons {
        if person.age <= 10 {
            person.print();
        }
    }
}