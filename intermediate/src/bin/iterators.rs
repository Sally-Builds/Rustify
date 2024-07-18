fn main () {
    let data: Vec<i32> = vec![1, 2, 3, 4, 5].iter().map(|num| num * 3).filter(|num| num > &10).collect();

    for number in data.iter() {
        println!("{:?}", *number)
    }
}