use std::io;

fn get_input() -> io::Result<String> {
    println!("Enter Input: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn main () {
    let mut all_input = vec![];
    let mut count = 0;

    while count < 2 {
        match get_input() {
            Ok(input) => {
                all_input.push(input);
                count += 1;
            },
            Err(e) => println!("Error: {:?}", e)
        }
    }

    for input in all_input {
        println!("user input: {:?}, altered input: {:?}", input, input.to_uppercase())
    }
}