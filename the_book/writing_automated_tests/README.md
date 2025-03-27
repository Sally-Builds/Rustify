# Writing Automated Tests

### How To Write Tests

- Tests are Rust Functions that verify that non-test code is functioning in the expected manner. Its typical to have our tests bodies perform these 3 actions

1. Set up an data or state needed
2. Run the code you want to test
3. Assert the results are what you expect

### The anatomy of test function

- A test in Rust is a function that is annotated with the **test** attribute.
- Attributes are metadata about pieces of Rust code; an example we've seen previously is the **derive** attribute used with structs and enums.
- To change a function into test function we add the `#[test]` attribute on the line before the `fn`.
- We run our test with the command `cargo test`, Rust then builds a test runner binary that runs the functions annotated with the test attributes and report on whether each test function passes or fails.

```rs

    fn two_plus_two() -> u8 {
        2 + 2
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn check_two_plus_two() {
            assert_eq!(2, two_plus_two());
        }
    }
```

#### Checking Results with the **`assert`** macro.

- The `assert!` macro, provided by the std library is useful when you want to ensure that some condition in a test evaluates to true.
- We give the `assert!` macro a condition that evaluates to a boolean. if the value is true assert! does nothing, else it calls the `panic!` macro.

#### Testing equality with the **`assert_eq!`** and **`assert_ne!`** macros

- These macros compare two arguments for equality or inequality respectively.
- If the condition fails, the assert_eq! and assert_ne! macro will print the two values you gave as argument making it easier to see why the test failed.
- The `assert_eq!` uses the operator `==` and the `assert_ne!` uses the `!=` behind the scene so this means that the arguments we give to them **MUST** implement the `PartialEq` trait.
- Also because it prints the arguments when it fails, we also have to make sure the arguments implements the `Debug` trait.
- All primitive types and most of the std library implements these traits.
- For Structs and Enum that we define, we'll need to implement the required traits.
- Both `PartialEq` and `Debug` are derivable traits, so it is as straight forward as adding `#[derive(PartialEq, Debug)]` annotation to our struct and enums.

#### Adding Custom Failure Messages

- We can also add custom message to be printed with the failure message as optional arguments to `assert!`, `assert_eq!` and `assert_ne!` macros

```rs

    assert!(is_user(), "user doesnt exist")
```

#### Checking for panics with `should_panic`

- Its important to check that our code handles error conditions as we expect.
- We can write tests that ensure that a function panics if some condition are met.
- The `should_panic` attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesnt panic.
- We place the `#[should_panic]` attribute after the `#[test]` attribute and before the test function it applies to. for e.g.

```rs
    #[tests]
    #[should_panic]
    fn greater_than_two() {
        Guess:new(200);
    }
```

- If our code panicked for a different reason we did not expect, the test will pass. To make sure it panics for the exact reason we expected we cam add am expected parameter to the `should_panic` attribute.

```rs
    #[should_panic(expected="Guess value must be less than or equal to 100")]
```

### Controlling how Tests are run

#### Running Tests in Parallel or Consecutively

- When we run multiple tests, by default they run in parallel using threads.
- Due to this, we have to make sure our tests dont depend on each other or any shared state, including a shared environment such as the current working directory or environmental variables.
- If you do not want to run the tests in parallel or if you want more fine grained control over the number of threads used, we use the `--test` thread flag.

```rs
    $ cargo test -- --test-threads=1
```

- This sets the number of test threads to 1, telling the program not to use any parallelism. This will obviously take longer but the tests wont interfere with each other if they share a state.

#### Showing function outputs

- By default, if a test passes Rusts test library captures anything printed to the standard output. eg if we call the println! macro in a test and the test passes, we wont see the output on the terminal, we will see only the line that indicates the test passed.
- if a test fails, we'll see whatever was printed to standard output with the rest of the failure message.
- If we want to see printed values for passing tests as well, we can disable the output capture behavior by using the `--nocapture` flag

```rs
    $ cargo test -- --nocapture
```

### Running a subset of tests by name

#### Running Single tests

```rs
    $ cargo test one_hundred // will only run the test for one_hundred
```

#### Filtering to run multiple tests

```rs
    cargo test add // this command will run all test with add in the name
```

#### Ignoring some tests unless specifically requested

- Sometimes a few specific tests can be very time-consuming to execute, so we might want to exclude them during most runs of cargo test

```rs
    #[test]
    #[ignore]
    fn expensive_test() {
        // this test is ignored when we run cargo test
    }
```

- if we decide to run ignored test we use use this command

```rs
    $ cargo test -- --ignored
```

## Test Organization

- Rust community thinks about tests in terms of two main categories:

1. Unit Tests
2. Integration Tests

#### Unit Tests

- The purpose of unit test is to test each unit of code in isolation from the rest of the code to quickly pin point where code is and isnt working as expected.
- We put unit tests in the `src` directory in each file with the code they're testing.
- The convention is to create a module named `Tests` in each file to contain the test functions and to annotate the module with `cfg(test)`

#### The Tests Module and the `#[cfg(test)]`

- the `#[cfg(test)]` attribute on the test module tells Rust to compile and run the test code only when we run `cargo test`, not run `cargo build`

```rs
    #[cfg(test)]
    mod tests {
        #[test]
        fn expensive_test() {

        }
    }
```

- The attribute `cfg` stands for configuration and tells Rust that the following item should only be included given certain configuration options. In this case the configuration option is `test`.

#### Testing Private Functions

```rs
    fn internal_address() -> i32 {
        3
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn internal_test() {
            ...
            assert_eq!(3, internal_address())
        }
    }
```

- we just import and call the function even though it wasnt marked as public.

#### Integration Test

- In Rust, Integration Tests are entirely external to our library. They use our library in the same way any other code would, which means they can only call functions that re part of our library's public API.
- To create an integration test, we need to create a `test` directory

##### The `tests` directory

- we create a `tests` directory at the top level of our project directory next to `src` directory

```rs
    - src
        . lib.rs
    - tests
        . integration_test.rs
    . cargo.toml

    //integration_test.rs
    extern crate adder:

    #[test]
    fn it_adds_two() {
        assert_eq!(4, adder::add_two(2));
    }
```

- In integration test we do not need to annotate with the #[cfg(test)]
- Cargo treats the tests directory specially and compiles files in this directory only when we run `cargo test`
- running `cargo test` runs the unit test, integration test and doc test
- if we wanted to run only a particular integration test file we use the command

```rs
    $ cargo test --test test_filename
```

##### Submodules in Integration Tests

- As our integration tests grow we might want to add more than one file in the tests directory to help organize our test better.
- Since Rust treats each integration test file as its own crate, this means they work differently as how `src` does.
- This behavior is more noticeable when we want to create helper function which our test functions will want to use.
- if we create a `tests/helper.rs` file and place a function named `setup` that we can call from multiple test functions in multiple test files, when we run the tests, we'll see a new section in the test output for `helper.rs` file even though the file doesnt contain test function
- To avoid having `helper.rs` appear in the test output, instead of creating a `tests/helper.rs` file, we'll create a `tests/helper/mod.rs` file
- After creating the module we can then use it from the integration test file as a module.

```rs
    extern crate addr;
    mod helper;

    #[test]
    fn it_adds_two() {
        helper::setup();
        assert_eq!(4, addr::add_two(2));
    }
```

#### Integration Tests for Binary crates

- If our project is a binary crate that only contains src/main.rs file and doesnt have a src/lib.rs file, we cannot create integration tests in the tests directory and use extern crate to import functions defined in the src/main.rs
- Only library crates expose functions that other crates can call and use
- **Binary crates are meant to be run on their own, independently**
- This is one of the reasons why Rust projects that provide a binary crate have very straight forward src/main.rs file that calls the logic which lives in the src/lib.rs file.
- Using that structure, we can perform integration tests by using the `extern crate` to test the important functionality.
