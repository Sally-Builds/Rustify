# Getting Started

## Installation

- To install rust, we have to install `rustup` a command line tool used for managing rust versions.

- `$ curl https://sh.rustup.rs -sSf | sh`

- Automatically rustup adds rust to our system path, which will take effect on next login, but if you want to add it manually you could use this command.<br />
  `$ source $HOME/.cargo/env`

- to update to latest rust version use this command `$ rustup update`

## Documentation

- Rust comes with a local documentation guide. To access the docs use this command `$ rustup doc`

## Writing and Running a Rust Program

- ### Custom method:

  - create a file with a .rs extension. _every rust file must end in `.rs`_
  - add the following lines of code: <br />

    ```rs
    fn main() {
        printlin!("Hello World!")
    }
    ```

  - The next step is to compile the program. `$ rustc file_name`
  - After compilation, a binary executable file is generated. The file will be the same as the rust file name you created earlier, but it wont have the `.rs` extension. If you use windows it would rather end with the `.exe`.
  - To run the compiled program we run it like this: `$ ./main`
  - We should see the text - "Hello World!" printed on the terminal

- ### Using Cargo

  Cargo is the official rust build system and package manager, just like we have different pkg manager is nodejs like npm, yarn, pnpm etc.

  #### Creating a Project

  - To create a new cargo project we use this command `$ cargo new hello_cargo --bin` where `hello_cargo` is our project name.
  - After running the command, cargo creates a folder called hello_cargo, inside the hello_cargo folder we will see src folder, cargo.toml file
  - The src folder is where our code lives, inside the src folder we will see a file called `main.rs` just like we created in the custom setup.
  - The cargo.toml file is the configuration file for our project, it will contain the dependencies or libraries used in our project.

  #### Building and Running Cargo Project

  - To compile a cargo project we use the command `$ cargo build`.
  - After the project is compiled, a folder called target is created and the we can run the executable file located at `/target/debug/hello_cargo`
  - To run the executable file we use `$ ./target/debug/hello_cargo`
  - To compile and run the project in one command we use `$ cargo run`
  - Cargo also provides another command `cargo check` which is used to compile the project but it doesnt generate the executable file. This command is faster than `$ cargo build` because it skips the step of generating the executable file. this is helpful when you just want to check that all your code is correct and devoid of any bugs.

  #### Building for Release(Production)

  - When our project is finally ready, we can use the command `$ cargo build --release`. We can then find the exectuable in the path `/target/release/hello_cargo` This command compiles the project with optimization.
  - Due to the optimization our code will run faster but during compilation it takes more time. This is the reason why we have the compilation for development(debug) and production(release).
