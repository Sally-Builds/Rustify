fn main () {
    let numbers = vec![10, 20, 30, 40];

    for number in &numbers {
        match number {
            30 => println!("Thirty "),
            _ => println!("{:?} ", number)
        }
    }

    println!("{:?}", numbers.len())
}
