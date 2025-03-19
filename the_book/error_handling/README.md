# Error Handling

- Rust groups errors into two(2) major categories: **recoverable** and **unrecoverable** errors
- Recoverable errors, such as file not found error is reasonable for us to report the problem to the user and retry the operation.
- Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.
- Most languages does'nt distinguish between the two(2) errors, they handle them the same way using mechanism.
- Rust does'nt have exceptions, instead it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

## Unrecoverable Errors with `panic!`

- when the `panic!` macro executes your program will print a failure message, unwind and clean up the stack, and then quit
- we use this macro when bugs of some kind has been detected and is not clear to us how to handle the error.

### Unwinding the stack or aborting in response to a `panic~`

- By default , when a panic occurs, the program starts unwinding, meaning it walks back up the stack and clears up the memory. This can be a very intensive process
- There is another alternative to immediately abort, which means the program doesn't have to clean up anything. The cleaning up in this case is left to the Operating System.
- If we wanted to make the resulting binary in our project as small as possible, we can switch from unwinding to abortion upon a panic by adding a `panic = 'abort'` to the appropraite `[profile]` section in our `cargo.toml` file.

```rs
    [profile.release]
    panic = 'abort'
```

### Using a `panic!` Backtrace

```rs
    let v = [1, 2, 4];
    v[99];
```

- In the example above, we try to access an element that doesnt exist on the vector, because of this, Rust will **panic**.
- Other languages like `C` will attempt to give you what exactly you asked for in this situation, even though it isnt what you want. You will get whatever is at the location in memory that would correspond to that element in the vector, even though the memory doesn't belong to the vector.
- This is called **buffer Overload** and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldnt be allowed to.
- This is the reason Rust stops execution and refuse to continue
- In order to get the full backtrace log. use this command below.

```rs
$ RUST_BACKTRACE=1 cargo run
```

### Recoverable Errors with `Result`

- most errors aren't serious enough to require the program to stop entirely.
- example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.
- We use the `Result` enum to handle recoverable errors. It is defined as having two(2) variants, `OK` and `Err`

```rs
    enum Result<T, E> {
        OK(T),
        Err(E)
    }
```

- The `T` and `E` are generic type parameters.
- `T` represents the type of the value that will be returned in a success case within the `OK` variant.
- `E` represents the type of the error that will be returned in failure case within the `Err` variant.

```rs
    use std::fs::File;
    let file = File::Open("hello.txt");
    let file = match(file) {
        OK(data) => data,
        Err(_) => panic!("File doesnt exist")
    };

```

- The `Result` enum just like the `Option` Enum is already imported in the prelude, hence why we do not need to explicitly use `Result::OK`.

### Matching on Different Errors

- we can take different actions for different failure reasons. here is what i mean bro

```rs
    use std::fs::File;
    let file = File::Open("hello.txt");
    let file = match(file) {
        OK(data) => data,
        Err(ref error) => if error.kind == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                OK(f) => f,
                Err(e) => panic!("Sth went wrong bro")
            };
        },
        Err(error) => panic!("Failed to open file: {:?}", error),
    };
```

- The type of value that `File::Open` returns inside the `Err` variant is the `io::Error` which is a struct provided by the `std` library.
- The struct has a method kind that we can get an `io::ErrorKind` value.
- The enum `io::ErrorKind` is provided by the `std` library and has variants representing the different kinds of errors that might result from an `io` operation.
- The condition `if error.kind() == ErrorKind::NotFound` is called a **`match guard`**; this is an extra condition on a match arm that further refines the arms pattern. This condition must be true for that arms code to be run, otherwise, the pattern matching will move to the next arm.
- The `ref` in the pattern is needed so error is not moved into the guard condition but is merely referenced by it.
- In the Context of a pattern, **\*`&` matches a reference and gives you its value, but ref matches a value and gives you a reference**

### Shortcuts for Panic on Error: `unwrap` and `expect`.

- using `match` works well enough, but it can be verbose and doesn't always communicate intent well. The `Result<T, E>` type has many helper methods defined on it to do various tasks.
- one of these methods, is called `unwrap`: If the `Result` value is `OK` variant, `unwrap` will return the value inside the `OK`. If the Result is the `Err` variant, unwrap will call the `panic!` macro on our behalf.
- The `expect` method is similar to unwrap, but it lets us choose the `panic!` error message.
- Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rs
    let f = File::Open("hello.txt").expect("Failed to open hello.txt");
```

### Propagating Errors

- When you're writing a function whose implementation calls something that might fail, instead of handling the error within the function, we can return the error to the calling code so that it can decide what to do. This is known as **Propagating** the error. It gives more control to the calling code.

```rs
    fn read_username_from_file () -> Result<String, io::Error> {
        ...
    }
```

- we chose the `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we call in the function body that might fail.
- This pattern of propagating error is very common in Rust that Rust provides the `?` operator to make it easier.

### A shortcut for Propagating Errors: the `?` Operator.

- The `?` operator is a shortcut is Rust used for error handling that lets us propagate error without having to write boilerplate code.
- **It is used with functions that return `Result` or `Option` enum**
- if the call gives an `OK` variant, `?` unwraps or extracts the value from the enum and lets us continue in our code.
- if the call gives an `Err` variant, `?` return the error immediately
