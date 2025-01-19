pub mod testing_if {
    pub fn can_get_pregnant () {
        let gender_to_get_pregnant = String::from("female");

        let mut user_gender = String::from("");

        println!("What is your gender - male or female?");
        std::io::stdin().read_line(&mut user_gender).unwrap();

        user_gender = user_gender.replace('\n', "");
        if user_gender == gender_to_get_pregnant {
            println!("Hurray!!! You are a woman and you can get pregnant");
        }else {
            println!("You are a man!");
        }
    }

    pub fn test_is_off_age () {
        let adult = 18u8;

        let mut user_input = String::from("");
        std::io::stdin().read_line(&mut user_input).unwrap();

        let user_age = user_input.replace('\n', "").parse::<u8>().unwrap();

        if user_age >= adult {
            println!("Congratulations!! You are an adult. Do whatever you want.")
        }else {
            println!("Oops!!! You are still a baby.")
        }


    }
}

pub mod testing_iterations {
    pub fn test_loop () {
        let mut counter = 1;

        loop {
            println!("{}", counter);

            if counter == 10 {break}

            counter += 1;
        }
    }
}