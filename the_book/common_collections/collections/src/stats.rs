use std::collections::HashMap;


pub fn calc_mean(vect: &[i32]) -> f64 {
    let sum:i32 = vect.iter().sum();

    if sum == 0 {
        return 0.0;
    }

    let total = vect.len() as f64;

    sum as f64 / total
}


pub fn calc_median(vect: &mut [i32]) -> f64 {
    vect.sort();
    println!("sorted vector - {:?}", vect);
    let even_or_odd = vect.len() as u32 % 2;
    match even_or_odd {
        0 => {
            let mid = vect.len() / 2;
            let sum = vect[mid - 1] + vect[mid];
            return sum as f64 / 2  as f64;
        },
        _ => {
            let mid = vect.len() / 2;
            return vect[mid - 1] as f64;
        },
    }
}

pub fn calc_mode(vect: &[i32]) -> i32 {
    let mut mode: HashMap<i32,i32> = HashMap::new();
    
    for v in vect {
        let count = mode.entry(*v).or_insert(0);
        *count += 1;

    }

    let mut max: (i32, i32) = (0, 0);
    for (k, v) in mode {
        if v > max.1 {
            max = (k, v)
        }
    }

    max.0
}

// Given a list of integers, 
// use a vector and return the mean (the average value), 
// median (when sorted, the value in the middle position), 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list. 
// •	Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding! 