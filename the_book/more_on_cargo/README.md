# 14) More About Cargo and Crate.io

- In this chapter we'll show how to customize our build through release profiles, publish libraries on crates.io, Organize large projects with workspaces, install binaries from crates.io and extend cargo using

## Customizing Builds with Release profiles

- In Rust, `release` profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
- Cargo has two main profiles, the `dev` profile cargo uses when we run `cargo run build` and the `release` profile cargo uses when we run `cargo build --release`.
- The dev profile is used in development while the release is used in release builds.
  -we can customize these profiles by add a `[profile.*]` section in our cargo.toml file

```toml
    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 1
```

- `opt-level` is a setting which controls the number of optimization Rust will apply to our code with range 0 - 3
- we can override any setting by adding a different value.

### Publishing a Crate to Crate.io

- we can share our code with other people by publishing our own package.

#### Making Useful Documentation Comments

- Accurately documenting our packages will help others know how to use and when to use them, so its worth investing time to write documentation.
- Rust has a particular kind of comment for documentation, known as **documentation comment**, that generates html documentation.
- Documentation comments uses 3 slashes instead of 2. it also supports markdown notation for formatting a text.
- We place documentation comments just before the item they're documenting.
- we use `cargo doc` to put generated html doc in the target/doc directory
- we use `cargo doc --open` to open the result in a web browser.

###### Commonly Used Sections

- we use the `#` to create a section. here are commonly used
- Panics - a function could panic
- Errors - a function returns a Result
- Safety - a function is unsafe to call

##### Documentation Comments as Tests

- Adding example code blocks to our documentation comment can help demonstrate how to use or library, but also running cargo test will run the code examples in our documentation as test

#### Commenting Contained Items

- `//!` is another style for documentation comment. it adds documentation to the Item that contains the comment rather than adding documentation to the items following the comments.
- we typically use these documentation comments inside crate root file or inside modules to document the crate or the modules as a whole.

````rs
// This is a regular comment explaining the code
let x = 5; // You can also put them after code

/** The next is documentation comment - used to document the item that follows them (struct, functions, enums) */

/// Adds two numbers together
///
/// # Examples
/// ```
/// let sum = add(2, 3);
/// assert_eq!(sum, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/**Used to document the item that contains them (typically used at the beginning of modules or crates) */

//! # My Crate
//!
//! This is a library that provides awesome functionality.
//! It's documented using inner doc comments.
````

- documentation comments within items are useful for describing crates and modules. we use them to explain the overall purpose of the container to help users understand crates organization.

### Exporting a convenient public API with `pub use`

- The structure of our public API is a major consideration when publishing a crate.
- If the structure isnt convenient for others to use from another library, we dont have to rearrange our internal organization. we can re-export items to make a public structure thats different from our private structure by using `pub use`.
- re-exporting takes a public item in one location and makes it public in another location as if it were in the other location instead.
- In cases where there are many nested modules, re-exporting the types at the top level with `pub use` can make a significant difference in the experience of people who use our crate.

```rs
    pub mod kinds {
        pub enum PrimaryColor {}
        pub enum SecondaryColor {}
    }

    // top module
    use arts::kinds::PrimaryColor;

    pub use kinds::PrimaryColor;

    //usage in another library
    use art::PrimaryColor;
```

### Publishing on cargo.io

- To publish on cargo.io, we first need to create an account. After creating an account we then get our API token from cargo.io/me and then with the token we login using `cargo login our-token`. The token is then stored in `~/cargo/credentials` in our computer.
- we can add metadata to our cargo.toml file like version number, description, license, author etc.
- we can then publish using `cargo publish`.
- When we make changes to our code we can republish by changing the version number in the cargo.toml file
- If we have a version we dont want users to use anymore, we can **yank** its version using `cargo yank --vers 1.0.1`. what this means is that when users want to install our crate, they wouldnt be able to install the yanked version but this would not affect users who are already using the version who have the cargo.lock already.

### Cargo Workspaces

- as our projects develops, we might want to split our packages further into multiple library crates. Cargo offers a feature called **Workspaces** that can help manage related packages developed in tandem.

##### Creating a Workspace

- A workspace is a set of packages that share the same `cargo.lock` and output directory.
- To create a workspace, we first create a directory. Inside the newly created directory, we then create a cargo.toml file to configure the entire workspace
- The file wont have a [package] section or metadata as seen in other cargo.toml
- it will start with a [workspace] section that allows us to add members to the workspace.

```rs
    [workspace]
    members = [
        "adder"
    ]
```

- we can then create the "adder" binary by running `cargo new --bin adder`
- The members of the workspace dont have their own target directory.
- We could also add another member to the workspace and then run the `cargo new --lib new_lib`
- here is what the workspace directory will look like

```sh
    - cargo.lock
    - cargo.toml
    - adder
        - cargo.toml
        - src
            - main.rs
    - new_lib
        - cargo.toml
        - src
            - lib.rs
    - target
```

- If we wanted the members of the workspace to interact we first have to add path dependency

```rs
    // in the adder cargo.toml
    [dependencies]
    new_lib = {path: "../new_lib"}

    //main.rs
    extern crate new_lib;

    main() {
        new_lib::add_one(5);
    }
```

- To run the binary crate we have to specify the name of the member to run with the `-p` flag. eg `cargo run -p adder`.

##### Depending on an External Crate in a Workspace.

- We have only one cargo.lock file at the top level rather than in each individual members. This helps to make all the dependencies compatible with all the members of the workspace.

```rs
        //new_lib/cargo.toml
        [dependencies]
        rand = "0.3.14"
```

- when we run `cargo build` it updates the cargo.lock file and gives it information about the rand package of new_lib.
- if we want to use the rand in adder member, it wont work until we add it to the adder/cargo.toml file as dependency exactly as we did in new_lib/cargo.toml

##### Adding a test to a Workspace

- when we add test to the library crate in the workspace and run `cargo test` it checks the whole workspace and run each test in its members.
- We can be more specific as well by running `cargo test -p new_lib`
