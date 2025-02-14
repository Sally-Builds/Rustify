use std::collections::HashMap;

// •Using a hash map and vectors, create a text interface to allow a user to add 
//employee names to a department in a company. 
//For example, “Add Sally to Engineering” or
//  “Add Amir to Sales.” Then let the user retrieve a list of all people in a 
//department or all people in the company by department, sorted alphabetically. 
// The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!

pub fn add(hash_map: &mut HashMap<String, Vec<String>>) {
    loop {
        let mut input = String::new();

        println!("Add Employee to Department: ");
        std::io::stdin().read_line(&mut input).unwrap();

        let s = input.trim();

        if s == "exit" {
            break
        }

        let str_arr = s.split_whitespace();
        let mut key = String::new();
        let mut val = String::new();

        for (index, str) in str_arr.enumerate() {
            if index == 1 {
                val = str.to_owned();
            }

            if index == 3 {
                key = str.to_owned();
            }
        }
        
        if !key.is_empty() {
            let vect = hash_map.entry(key).or_insert(Vec::new());
            vect.push(val);
    }

    }
}

pub fn print_dept_employee(hash_map: &HashMap<String, Vec<String>>) {
    let mut s = String::new();

    println!("Get Department Members: ");
    std::io::stdin().read_line(&mut s).unwrap();

    let input = s.trim();

    let x = hash_map.get(input);

    if let Some(vect) = x {
        println!("Here is a list of employees in the {} department", input);
        let mut cloned_vect = vect.clone();
        cloned_vect.sort();
        for employee in cloned_vect {
            println!("{}", employee);
        }
    };
}