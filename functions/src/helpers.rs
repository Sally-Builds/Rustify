pub fn get_name (first_name: &str, last_name: &str) -> String {
    let name = format!("{} {}", first_name, last_name);

    return name;
}

pub mod child_mod {
    pub fn print_msg () {
        println!("Hello There!!!!")
    }
}