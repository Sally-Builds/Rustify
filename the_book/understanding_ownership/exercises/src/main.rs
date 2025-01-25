fn main() {
    let mut a = 10;
    let mut b = 12;
    let arr = [1, 2, 10, 3, 5,6];

    let mut s = String::from("Hello");

    {
        let r3 = &mut s;
    println!("r3 = {}", r3);
}
    
    let r1 = &s; //ok
    let r2 = &s; //ok

    let mut _bs = "okay";
    _bs = "bro";

    println!("_bs: {}", _bs);

    println!("r1 = {}, r2 = {}", r1, r2);

    //Swap
    swap(&mut a, &mut b);

    println!("After swap: a = {}, b = {}", a, b);
    //End of Swap

    //slice Average
   let arr_avg =  slice_average(&arr);

   println!("Arr Avg is: {:.2}", arr_avg);
   //end of slice average

   //max value
    let max_value = max_value(&arr);
    println!("Maximum value in array = {}", max_value);

   //end of max value


   // two string concat
   let concatenated_string = str_concat("hello", " world");

   println!("Concatenated String - {}", concatenated_string);
   //end of two string concat
}


/*
Write a function that takes two mutable references to integers and swaps their value
*/
fn swap (a: &mut i32, b: &mut i32) {
    let temp_a = *a;

    *a = *b;
    *b = temp_a;
}

/**
 *  Write a function that takes a slice of integers and returns the average of the numbers
 */
fn slice_average (slice: &[i32]) -> f64 {
    let mut total = 0;
    let len = slice.len() as f64;
    
    for num in slice.iter() {
        total = total + *num;
    }

    f64::from(total) / len
}


fn max_value (numbers: &[i32]) -> i32 {
    let mut max = numbers[0];

    for (index, num) in numbers.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if *num > max {
            max = *num;
        }
    }

    max
}

/*
Write a function that concatenates two string slices and returns a new string
*/
fn str_concat (a: &str, b: &str) -> String {

    let mut s = String::from(a);
    s.push_str(b);

    s
}