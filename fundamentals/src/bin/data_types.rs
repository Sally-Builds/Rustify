fn add (a: i32, b: i32) -> i32 {
    //function declaration
    a + b
}


//enumerations
enum Status {
    PENDING,
    CANCELLED,
    SUCCESSFUL,
    DECLINED,
}


//Structs
struct User {
    status: Status,
    age: i8,
    salary: u64,
}

fn main() {
    //data types and variables
    let name = "Joshua";
    let age = 26;
    let grade = 'A';

    //integer literal
    let salary: u64 = 800_000;


    let hourly_rate = 42.5;

    //function call
    let sum: i32 = add(8, 9);

    //println macro
    println!("{:?}", sum);
    println!("{}", name);
    println!("My Salary is ${salary}");

    //control flow

    // if else if else
    if grade == 'A' {
        println!("Excellent");
    }else if grade == 'B' {
        println!("Good");
    }else {
        println!("Iti boribor");
    }

    //switch or match

    match grade {
        'A' => println!("Excelent"),
        'B' => println!("Good"),
        _ => println!("Iti boribor")
    }


    //loops

    //infinte loop
    let mut i = 1;
    let mut j = 5;
    println!("===== infinte loop =====");
    loop {
        if i > 5 {
            break;
        };

        println!("{i}");
        i = i + 1;
    }

    println!("===== while loop =====");
    while j >= 0 {
        println!("{j}");

        j = j - 1;
    }


    //enumeration
    let status: Status = Status::PENDING;
    let msg;

    match status {
        Status::PENDING => msg = "pending",
        Status::CANCELLED => msg = "cancelled",
        Status::SUCCESSFUL => msg = "successful",
        Status::DECLINED => msg = "declined"
    }

    println!("{msg}");


    //struct usage
    let user: User = User{
        status: Status::SUCCESSFUL,
        age: 19,
        salary: salary,
    };

    println!("{}", user.salary);

    //Tuples
    let (a, b) = (4, 8);
    let tup = (4, 8);

    println!("{}, {}", a, b);
    println!("{}, {}", tup.1, tup.0);


    //Expressions

    let new_status: Status = Status::SUCCESSFUL;
    let status_msg = match new_status {
        Status::PENDING => "pending",
        Status::CANCELLED => "cancelled",
        Status::SUCCESSFUL => "successful",
        Status::DECLINED => "declined"
    };
    println!("expression status msg = {}", status_msg);

    let is_of_age = user.age >= 18;

    println!("{}", is_of_age);
}