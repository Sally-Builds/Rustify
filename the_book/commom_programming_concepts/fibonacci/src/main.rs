use std::io;

fn main() {
    loop {
        println!("Enter the number: ");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Error Reading user input");

        let user_input: u32 =  match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ans = fibonacci_calc(user_input);

         println!("The {}th fibonacci number is {} ", user_input, ans);  

         break;

    }

}


fn fibonacci_calc (n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fibonacci_calc(n - 1) + fibonacci_calc(n - 2)
}
