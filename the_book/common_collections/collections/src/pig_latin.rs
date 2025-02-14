// •	Convert strings to pig latin. The first consonant of each word is moved to the end of the word 
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to 
// the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding! 


pub fn transformer (str: &mut String)  {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut is_vowel = false;
    let mut s = String::new();
    let mut first_letter: Option<char> = None;

    for val in str.chars().enumerate() {
        if val.0 == 0 {
            let ch = vowels.iter().find(|&&c| c == val.1.to_ascii_lowercase());
            match ch {
                Some(_) => {
                    is_vowel = true;
                    s.push(val.1);
                },
                None => {
                    first_letter = Some(val.1);
                }
            }
        }else {
            s.push(val.1);
        }
    }

    if is_vowel {
        // println!("{}-hay", s);
        *str = format!("{}-hay", s)
    }else {
        if let Some(c) = first_letter {
            *str = format!("{}-{}ay", s, c); 
        };
    }
}

pub fn optimized_transformer (str: &str) -> String {
    if str.is_empty() {
        return String::new()
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let (first, rest) = str.split_at(1);

    if vowels.contains(&first.chars().next().unwrap().to_ascii_lowercase()) {
        format!("{}{}-hay", first, rest)
    }else {
        format!("{}-{}ay", rest, first)
    }
}