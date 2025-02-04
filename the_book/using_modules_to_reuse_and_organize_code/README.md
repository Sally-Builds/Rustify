# Using Modules to Reuse and Organize Code

As our code grows, we'll eventually move functionality into other functions for reuse and better organization. By splitting our code into smaller chunks, we'll make each chunk easier to understand on it's own. Rust has a module system that enables the reuse of code in organized fashion.

- In the same way we extract lines of code into a function, we can extract functions (also structs and enums) into different **modules**
- A **Module** is a namespace that contains definitions of functions or types and you can choose whether those definitions are visible outside their module(public) or not(private).
- The **`mod`** keyword is used to declare a new module.
- By default, functions, types, constants and modules are private. the **`pub`** keyword makes an item public and therefore visible outside its namespace.
- The `use` keyword brings modules or their definitions inside modules into scope so its easier to refer to them.

### Mod & The Filesystem

- We'll start by creating a new crate, but instead of a binary crate, we will create a library crate.
- **A library Crate** is a project that other people can pull into their project as dependency.

```rs
    $ cargo new communicator --lib
```

- here cargo created an `src/lib.rs` file instead of `src/main.rs`
- inside the lib.rs file, cargo creates an example test to get our library started.
- Because we dont have a src/main.rs file, there is nothing for cargo to execute with the `cargo run` command. Therefore, we'll use the `cargo build` command to compile our library crate code.

### Module Definitions

- every module definition in Rust starts with the `mod` keyword.

```rs
    mod network {
        fn connect() {
            // do sth will you!
        }
    }
```

- every thing inside the mod block is namespaced.
- in this case, we have a single function, `connect`. if we wanted to call this function from code outside the network module, we would need to specify the module and use the `::` namespace syntax like so `network::connect()`

- we can also have modules inside other modules

```rs
    mod network {
        fn connect () {

        }

        mod client {
            fn connect () {

            }
        }
    }
```

- Modules form a hierarchy, the contents of `src/lib.rs` are at the top most level and the submodules are at lower levels. Here is what our previous example hierarchy looks like

```bash
    communicator
    |_ network
       |_ client
```

- `client` module is a child of the `network` module rather than a sibiling
- if we wanted client to be a sibling, then we move it out of the network module

```bash
    communicator
    |_ network
    |_ client
```

### Moving Modules to Other Files

- modules form a herarchichial structure, much like the computing filesystem.
- we can use Rust module system along with multiple files to split up Rust projects so not everything lives in `src/lib.rs` or `src/main.rs`
- lets say our `src/lib.rs` has this module hierarchy

```rs
    mod client {
        fn connect () {

        }
    }
    mod network {
        fn connect () {

        }

        mod server {

        }
    }
```

- if our modules had many functions and those functions become much, it would be hard to maintain. so we'll separate the client, network and server modules from `src/lib.rs` file and place into their own files.

- first, we replace the `client` module code with only the declaration of the client module.

```rs
    mod client;
    ...
```

- we are still declaring the `client` module here but by replacing the block with a semicolon, we are telling Rust to look in another location for the code defined within the scope of the client module.
- we then create an external file with that module name inside the `src` directory.
- we then enter the connect function in the `client.rs` file.

```rs
    //client.rs
    fn connect () {

    }
```

- we do not need a mod declaration in this file because we already declared the client module with **mod** in the `src/lib.rs`
- This file just provides the content of the client module. if we put a mod client here, we'd be giving the client module its own submodule named `client`.
- \*\*Rust only knows to look in the `src/lib.rs` file by default. if we wanted to add more files we need to tell Rust in `src/lib.rs` to look in other files, this is why mod client needs to be defined in `src/lib.rs` and cannot be defined in `src/client.rs`.

- Next we extract the remaining modules to their respective files.

```rs
    // src/lib.rs
    mod client;
    mod network;

    // src/network.rs
    fn connect () {

    }
    mod server {
        fn connect () {

        }
    }
```

- This works perfectly, but what if we want to move the mod server into its own separate file, will it still work?

- if we create a `src/server.rs` file and enter the contents of the server module that we extracted, when we try to build, we'll get an error - **"cannot declare a new module at this location"**

- To make this work here is how our project hierarchy would look

```sh
    communicator
    |_ client.rs
    |_ lib.rs
    |_ network
        |_ mod.rs
        |_ server.rs
```

- The reason for this is to avoid naming clashes so we create a directory for a module with nested modules and move the contents to `mod.rs` and then the contents of the submodule to its own file in the module directory

### Rules of Module Filesystem

- If a module named `foo` has no submodules, you should put the declarations for `foo` in a file named `foo.rs`

- if a module named foo does have submodules, you should put the declaration for foo in the file named `foo/mod.rs`.
- These rules apply recursively.
- The modules should be declared in their parent modules file using `mod` keyword.

### Controlling Visibility with `pub`

- we start by trying to use the communicator library from another project calling it externally. se we create a binary crate in the same directory as our library crate by making a `src/main.rs` file.

```rs
    // src/main.rs
    extern crate communicator;

    fn main () {
        communicator::client::connect();
    }
```

- we use the `extern crate` command to bring the communicator library crate into scope.
- Our package now contains two crates.
- Cargo treats the `src/main.rs` as the root file for binary crate which is separate from the existing library crate whose file is `src/lib.rs`.
- This pattern is quite common for executable projects - most functionality is in a library crate, and the binary crate uses the library crate.
- other programs can also use the library crate.
- From the POV of a crate outside the communicator library looking in, all the modules we've been creating are within a module that has the same name as the crate `communicator`.

- **NB**: if we are using an external crate within a submodule of our project, the `extern crate` should go in our root module. Then in the submodule, we can refer to items from external crates as if the items are top-level modules.

- The default state of all code in Rust is private: no one else is allowed to use the code.
- Marking a function or module as `public` lets Rust know that the function will be used by code outside your program
- **When a function is marked as public, Rust will not require that it be used in our program and will stop warning that the function is unused**

### Making a Function Public

- To tell Rust to make a function public, we add the `pub` keyword to the start of the declaration.

```rs
    // src/lib.rs
    pub mod client;

    // src/client.rs
    pub fn connect () {

    }
```

#### Privacy Rules

- if an item is public, it can be accessed through any of its parent modules.
- if an item is private, it can be accessed only by its immediate parent module and any of the parents child module.

### Bringing Names into scope with the `use` Keyword.

- Rust `use` keywords shortens lengthy function call by bringing the modules of the function you want to call into scope.

```rs
    pub mod a {
        pub mod series {
            pub mod of {
                pub fn nested_modules () {

                }
            }
        }
    }

    //main.rs
    use a::series:of;
    fn main () {
        of::nested_modules();
    }
```

- The line `use a::series::of;` means that rather than using the full path wherever we want to refer to the module, we can use use `of`.
- the `use` keyword brings only what we've specified into scope; it doesnt bring children of modules into scope. Hence why we still use `of::nested_modules()` when we wanted to call the function.

- if we wanted to bring the function into scope directly we could do this.

```rs
    use a::series::of::nested_modules;
    nested_modules();
```

- Enums also form a sort of namespace like modules, we can also bring an enum variant into scope with `use` as we can also bring an enum variant into scope with `use` as well.

- For any kind of `use` statement, if you are bringing multiple items from one namespace into scope, you can list them using curly brackets and commas. like below

```rs
    // another file
    enum TrafficLight {
            Red,
            Yellow,
            Green
    }

    //main.rs
    use TrafficLight::{Red, Yellow};

    fn main () {
        let red = Red;
        let yellow = Yellow;
        let green = TrafficLight::Green;
    }
```

- we are still specifying the TrafficLight namespace for the `Green` variant because we did not include in the use statement.

### Bringing All Names into scope with a Glob.

- To bring all the items(visible) items in a namespace into scope at once we use the (_) syntax, which is called the `glob operator`. `use TrafficLight::_;`
- We should use the glob operator sparingly, it could also pull in more items than you expected and cause naming conflicts.

### Using Super to access a Parent Module

- Paths are always relative to the current module, the only exception is in the use statement, where paths are relative to the crate root by default.
- So to use a module from within another module, we have to go one module hierarchy. We can either use leading colons to let Rust know that we want to start from the root and list the whole path like this `::client::connect();` or we can use `super` to move up one module in the hierarchy from our current module, like this `super::client::connect();`

- The `super` functionality changes the path you give to use so it is relative to the parent module instead of the root modules.
