pub mod match_pattern {
    const AGE: i32 = 80;

    pub fn match_this () -> String {
        match AGE {
            18 => "You are just coming of age".to_string(),
            1..=17 => "You are a minor".to_string(),
            _ => "I really dont care".to_string(),
        }   
    }
}