fn main () {
    let mut data = Some(5);

    while let Some(i) = data {
        println!("{:?}", i);

            data = Some(i - 1);
            if i == 0 {
            data = None;
        }
    }

    let vect = vec![1, 2, 3, 4,5,6];
    let mut num_iter = vect.iter();

    while let Some(num) = num_iter.next() {
        println!("num = {:?}", num)
    }
}