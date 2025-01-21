# Programming A Guessing Game

Overview: The guessing game introduces us to some important features and elements of the Rust programming language. As we go through the guessing game we will see what these features are and also how they are used. They will be a more detailed explanation of these concepts in the later chapters of the book, so hang on!

### How does the Guessing Game work?

A user is requested to guess a number, if their guess is smaller or greater than the secret number, they will be told until they get the correct number.

### Processing A Guess

The first thing we'll do is to ask the user for an input, process the input and make sure the input is in the expected form.

- To obtain a user input, we have to brin in the `io` library into scope. The `io` library comes from the standard library known as `std`.

* By default, Rust brings only a few types into scope of every program in the **prelude**. if a type you want isn't in the prelude, you must bring it explicitly with the `use` statement. in this case `use std::io;`

### Storing Values with Variables

```rs
let mut guess = String::new();
//let foo = 9;
```

- The `let` statement is used to create a **variable**.
- Variables by default are immutable, meaning cannot change.
- We use the keyword `mut` to tell rust that we want a variable to be mutable.
- `String` is a string type provided by the standard library that is a growable utf-8 encoded bit of text. The `::` syntax in the `::new()` indicates that `new` is an associated function of the type `String`.
- An associated function is implemented on a type itself, in this case `String` rather than on a particular instance of a String. _In some languages, this is called a <b>static method</b>_.
- The `new` function creates an empty string. Its a normal convention in rust for types to have `new` function associated with them. The `new` function makes a new value of some kind.

```rs
io::stdin().read_line(&mut guess)
           .expect("Failed to read line");
```

- The `stdin` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.
- The `read_line` method gets the input from the user and stores it into a string, so it takes a string as an argument.
- The string argument needs to be mutable, so that the method can change the string content by adding the user's input.
- the `&` indicates that this argument is a **reference**.
- **References gives us a way to let multiple parts of our code access one piece of data without needing to copy that data into memory multiple times**
- References like variables are immutable by default hence why we needed to write it as `&mut guess` instead of `&guess`;

### Handling Potential Errors with a Result type.

- `read_line` puts what the user types into the string we passed to it, but it also returns a value, in this case called `io::Result`.
- Rust has a number of types named `Result` in its std library: a generic Result as well as specific versions for sub domains such as the `io::Result`.
- The Result type is an **Enumeration**.
- An enumeration(_enum_) is a type that can have a fixed set of values, those values are called the enum's **variants**.
- for the `Result` type, the variants are `OK` or `Err`.
- The `OK` indicates the operation was successful, and inside the `OK` is the successfully generated value.
- The `Err` variant means the operation failed and it contains information about how and why the operation failed.
- **The purpose of these Result types is to encode error-handling information**
- if the instance of the `io::Result` is `Err` value, `expect` will cause the program to crash and display the message you passed as an argument to it.

### Printing values with the println! placeholders

```rs
println!("You guessed: {}", guess);
```

- the set of curly brackets {} is a placeholder that holds a value in place

### Using a Crate to Get more Functionality

- A `crate` is a package of Rust code.
- We've been building **binary crate**, which is an executable.
- The `rand` crate is a **library crate**, which contains code intended to be used in other programs.
- In the `cargo.toml` file, the `[dependencies]` section is where you tell cargo which external crate your project depends on and which version of theses crates you require.
- Cargo understands **Semantic Versioning(SemVer)** - which is a standard for writing version numbers.
- the number `0.3.14` is short hand for `^0.3.14` which means any version that has a public API compatible with version `0.3.14`.

- Cargo checks the `[dependecies]` section and downloads any crates we dont have yet.
- These libraries are downloaded from **Crates.io** - a rust ecosystem where people post their open source projects for others to use.
- Also, cargo also grabs a copy of libraries which the library can depend on. eg `rand` library can depend on `libc` library to work.

### Generating a Random Number

```rs
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..101);
```

- the `use rand::Rng;` line brings the Rng **`Trait`** into scope.
- The `Rng trait` defines methods that random number generator implements and the trait must be in scope for us to use those methods.
- The `rnd::thread_rng()` method will give use the particular random number generator that we are going to use: one that is specific to the current thread of execution and seeded by the operating system. <br>
  **NB: The number generator being specific to the current thread of execution is a design choice that offers safety. In a multithreaded program, multiple threads can run simultaneously. If all threads shared the same random number generator, there would be a risk of data races (where two or more threads access and modify the same data simultaneously). By providing each thread its own independent random number generator, `thread_rng` avoids this issue, ensuring each thread can safely generate random numbers without interfering with others.**

- Next, we call the `gen_range` method on the random generator. This method is defined by the `Rng` trait that we brought into scope with the `use rand::Rng;` statement.
- The `gen_range` method takes a range as argument and generates a random number between them. \*\*The lower bound is inclusive while the upper bound is exclusive, meaning if we have
  `1..5` as a range, it generates a random number from `1` to `4`.

**NB**: Instructions for using a crate are in each crate's documentation. A neat feature of cargo is that we can run the `cargo doc --open` command which will make you access the documentation provided by all your dependencies locally.

### Comparing the Guess to the Secret Number

```rs
    use std::cmp::Ordering;
    ...
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Hurray! You win!")
    }
```

- so we bring in a type into the scope called `std::cmp::Ordering`. Just like `Result`, `Ordering` is an `enum` and its variants are `Less`, `Greater` and `Equal`.
- the `cmp` method compares two methods and can be called on anything that can be compared. it takes a reference of what you want to compare it with. it returns a variant of the **Ordering enum** we brought into scope with the `use` statement.
- we use `match` expression to decide what to do next based on which variant was returned from the call to `cmp`.

- A `match` expression is made up of **arms**. An arm consist of `pattern` and a `code` that should run if the pattern is matched.
- The match constructs and patterns are powerful features in Rust that lets us express a variety of situations our code might encounter and makes sure we handle them all(exhaustive).
- Building the program, we run into a compilation error because `guess` and `secret_number` are not of the same type. `guess` is a `string` while `secret_number` is a number.
- In other for the `cmp` to work, the two values we compare must be of the same type. So we have to convert the string to a number.

`let guess: u32 = guess.trim().parse().expect("Not a number");`

- We create a variable called `guess` which already exist before, but this is not a problem because Rust allows us to **shadow** the previous value to guess with a new one.
- This **Shadowing** feature is often used in situations in which we want to convert a value from one type to another type.
- The `trim()` method removes any whitespace or new lines at the beginning and end of the value.
- The `parse()` method parses a string into some kind of number. because the `parse` method can parse a variety of number types, we need to tell rust the exact number type we want by explicitly setting the guess to `u32`.
- Also, the `u32` annotation means that Rust will now infer that the `secret_number` variable should be `u32` type as well, so the comparison will be between two values of the same type.
- The call to parse can cause an error, if the string contained something like `abc` there would be no way to convert a number. so because it might fail the parse method returns a `Result` type. For now we use the `expect` method to handle this situation which should crash the program or return the number parsed.

### Allowing multiple guess with looping.

```rs
loop {

}
```

- the `loop` keyword creates an infinite loop.

### Quitting After a Correct Guess

- Adding a break line in the `Ordering::Equal` arm will make the program exit the loop when the user guesses a secret number correctly.

### Handling invalid input

- instead of crashing the program when the user inputs a non-number using `expect` function. we can make the game ignore a non number so the user continue guessing. we can do that by refining the line where we parsed the guess to a number by using the `match` expression.
- **Switching from `expect` call to a `match` expression is how we generally move from crashing on an error to handling the error properly**
- Remember, `parse` returns a `Result` type and Result is an enum that has the variant `OK` and `Err`.

```rs
let guess: u32 = match guess.trim().cmp(&secret_number) {
    OK(num) => num,
    Err(_) => continue,
}
```

- if parse is able to successfully turn the string into a number, it will return `OK` value that contains the resulting number. the number then ends up being assigned to `guess` variable.
- if parse is not able to turn the string into a number, it will return an `Err` value that contains more information about the error, so the program will execute the `continue` statement, which tells the program to go to the next iteration of the loop.

### Conclusion

This chapter introduced us to some fundamental concepts in rust. This concepts will be explained in greater details in upcoming chapter. here are some of the concepts

- imports
- standard library
- io library
- external crates
- String type
- variable
- Associated functions(static methods)
- References
- Enums (`Result`, `Ordering`)
- expect method/ parse method
- Differences between Binary and Library crates
- Semantic Versioning(SemVer)
- Traits
- Threads
- Documentation
- match expressions
- Shadowing
- Loops
