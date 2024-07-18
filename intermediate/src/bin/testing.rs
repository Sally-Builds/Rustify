fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main () {}

#[cfg(test)]
mod test {

    #[test]
    fn check_all_caps () {
        use crate::*;

        let result = all_caps("Hello");

        assert_eq!(result, String::from("HELLO"), "string should be all uppercase")
    }
}