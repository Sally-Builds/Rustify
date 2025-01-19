pub mod iter_example {
    pub fn simple_iterator () {
        let fruits = vec!["Banana", "Mango", "Orange", "Blueberry"];


        for fruit in fruits.iter() {
            println!("My {} fruit", fruit)
        }
    }

    pub fn combining_iterators () {
        let fruits = vec!["Banana", "Mango", "Orange", "Blueberry"];
        let foods = vec!["Beans", "Rice"];


        let new_vec = fruits.iter().chain(foods.iter());

        //combine the iterators into a data type of collections
        let all: Vec<&&str> = new_vec.clone().collect();

        for food in new_vec {
            println!("{}", food)
        }

    }

    pub fn transforming_iterators () {
        let fruits = vec!["Banana", "Mango", "Orange", "Blueberry"];

        let  altered_fruits = fruits.iter().map(|s| String::from(*s));

        let new_fruits = altered_fruits.map(|mut s| {s.push_str(" fruit"); return s});

        new_fruits.for_each(|s| println!("{}", s));
    }

    pub fn zipping () {
        let first_names = vec!["Sally", "Jon", "Jane"];
        let last_names = vec!["Nwamama", "Jones", "Doe"];

        let full_names = first_names.iter().clone().zip(last_names.iter().clone());

        full_names.for_each(|s| println!("{} {}", *s.0, *s.1));
    }
}
