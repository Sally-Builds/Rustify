use std::{fs::File, io::Write};


fn main() {
    let f = File::create("hello_world.pdf");

    let mut f = match  f {
        Ok(file) => file,
        Err(_) => panic!("Error")
    };

    let _ = f.write(b"Hello World");

}
