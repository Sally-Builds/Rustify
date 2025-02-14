use std::collections::HashMap;

use collections;

fn main() {
    // let mut v = vec![6, 10, 12, 15, 13, 3, 7, 9, 9, 9, 4, 8, 9, 15];


    // let median = collections::stats::calc_median(&mut v);
    // println!("Vector median = {}", median);

    // let mean = collections::stats::calc_mean(&v);

    // println!("vector mean = {}", mean);

    // let mode = collections::stats::calc_mode(&v);
    // println!("vector mode = {}", mode);

    // let mut s = String::from("Airst");
    // println!("string before transformer - {}", s);
    // collections::pig_latin::transformer(&mut s);
    // println!("string after transformer - {}", s);

    // println!("{}", collections::pig_latin::optimized_transformer("yellow"));

    let mut map = HashMap::new();
    collections::employee_system::add(&mut map);
    collections::employee_system::print_dept_employee(&map);
}