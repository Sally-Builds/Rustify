

pub mod generics {
    pub struct Pair<T: Copy> {
        first: T, 
        second: T,
    }
    
    impl<T: Copy> Pair<T> {
        pub fn new(first: T, second: T) -> Pair<T> {
            Pair {
                first,
                second,
            }
        }
    
        pub fn first(&self) -> T {
            self.first
        }
    
        pub fn second(&self) -> T {
            self.second
        }
    
        pub fn swap(&mut self) {
            let temp = self.first;
            self.first = self.second;
            self.second = temp;
        }
    }
       
}

pub mod trait_example {
    use std::f64::consts::PI;
    pub trait Area {
        fn area(&self) -> f64;
    }
    
    pub struct Circle {
        pub radius: f64,
    }
    
    impl Area for Circle {
        fn area(&self) -> f64 {
            PI * self.radius.powi(2)
        }
    }
    
    pub struct Square {
        side: f64,
    }
    
    impl Area for Square {
        fn area(&self) -> f64 {
            self.side.powi(2)
        }
    }
    
    pub struct Rectangle {
        width: f64,
        length: f64,
    }
    
    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.width
        }
    }
    
    pub fn print_area<T: Area>(shape: T) {
        println!("Area of shape: {}", shape.area())
    }
    
}

pub mod lifetimes_example {
    pub fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            return s1;
        }

        s2
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{lifetimes_example::longest_string, trait_example::Area};

    use super::*;

    #[test]
    fn verify_pair_was_created_successfully() {
        let first = 18;
        let second = 23;

        let pair = generics::Pair::new(first, second);

        assert_eq!(pair.first(), first);
        assert_eq!(pair.second(), second);
    }

    #[test]
    fn verify_swap_was_successful() {
        let first = 18;
        let second = 23;

        let mut pair = generics::Pair::new(first, second);

        pair.swap();

        assert_eq!(pair.first(), second);
        assert_eq!(pair.second(), first);
    }

    #[test]
    fn verify_area() {
        let radius :f64 = 12.00;
        let area: f64 = (PI * radius * radius).into();
        let circle = trait_example::Circle{radius};
        assert_eq!(circle.area(), area)
    }

    #[test]
    fn should_return_longest_string() {
        let s1 = "Hello World";
        let s2 = String::from("My Name is Joshua Uzoagulu");

        assert_eq!(longest_string(s1, s2.as_str()), s2)
    }

}
