use std::{error::Error, fmt, fs::File, io::Read};


#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    EmptyString
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid number format"),
            ParseError::EmptyString => write!(f, "Empty input string"),
        }
    }
}

impl Error for ParseError {}

pub fn parse_number(s: &str) -> Result<i32, ParseError> {

    if s.is_empty() {
        return Err(ParseError::EmptyString)
    }

    s.parse().map_err(|_| ParseError::InvalidFormat)
}

pub fn read_file_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(path)?;  // Use the provided path
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // Handle read error with ?

    match contents.lines().next() {
        Some(line) => {
            let num = parse_number(line)?;  // Propagate ParseError
            Ok(num)
        },
        None => Err("File is empty or has no valid lines".into()),  // Convert &str to Box<dyn Error>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_ok() {
        let s = "30";

        assert_eq!(parse_number(s).unwrap(), 30);
    }

    #[test]
    fn should_fail_due_to_empty_string() {
        let s = "";

        match parse_number(s) {
            Err(e) => {
                assert_eq!(e, ParseError::EmptyString)
            },
            _ => ()
        }
    }

    #[test]
    fn should_fail_due_to_invalid_format() {
        let s = "hello world";

        match parse_number(s) {
            Err(e) => {
                assert_eq!(e, ParseError::InvalidFormat)
            },
            _ => ()
        }
    }
}