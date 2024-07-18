// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp_lower(){
        let result = clamp(3, 5, 10);
        assert_eq!(result, 5, "Should return lower value")
    }

    #[test]
    fn test_clamp_upper(){
        let result = clamp(12, 5, 10);
        assert_eq!(result, 10, "Should return upper value")
    }

    #[test]
    fn test_clamp_n(){
        let result = clamp(12, 12, 12);
        assert_eq!(result, 12, "Should return n value")
    }

    #[test]
    fn test_div(){
        let result = div(10, 5);
        assert_eq!(result, Some(2), "Should return 2 value")
    }

    #[test]
    fn test_div_0(){
        let result = div(10, 100);
        assert_eq!(result, Some(0), "Should return 0 value")
    }

    #[test]
    fn test_concat () {
        let result = concat("a", "b");
        assert_eq!(result, String::from("ab"), "should come after string")
    }
}