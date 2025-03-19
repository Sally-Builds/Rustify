use std::collections::btree_map::Range;

fn main() {
    let day_actions = [
        "A partridge in a pear tree", 
        "Two turtle doves", 
        "Three French hens", 
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];

    for i in (0..12) {
        println!("On the {} day of Christmas,", days[i]);
        println!("my true love gave to me");
        for j in (0..i+1).rev() {
            if j != 0 {
                println!("{},", day_actions[j]);
            }else {
                println!("{}.", day_actions[j]);
            }
        }
        println!("");
    }

    let x: (i32, i32, i32) = (10, 11, 12);
    let (y, z, m) = x;
    println!("{}", m);
}
