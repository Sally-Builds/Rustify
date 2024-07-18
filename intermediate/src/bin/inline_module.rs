mod greetings {
    fn good_morning () {
    println!("Good Morning");
}

pub fn good_night () {
   println!("Good Night");
}
}

mod math{
    fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn sub (a: i32, b: i32) -> i32 {
    a - b
}
}

fn main () {
    greetings::good_night();

}