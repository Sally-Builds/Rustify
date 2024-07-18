fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn main () {
    let sum = add(8, 9);
    let sub = |a: i32, b: i32| -> i32 {
        a + b
    };
    let mul = |a: i32, b: i32| a + b;
}