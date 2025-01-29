# USING STRUCTS TO STRUCTURE RELATED DATA

Overview: In this chapter we: compare tuples and structs, how to use structs, how to define methods and associated function. Structs and enums are the building blocks for creating new types in our program domain.

- A `Struct`, or Structure is a custom data type that lets you name and package together multiple related values that make up a meaningful group.

### Defining and Instantiating Structs

- Structs are similar to tuples, like tuples, the pieces of a struct can be of different types.
- Unlike tuples, you can name each piece of data so it is clear what the values mean.
- To define a struct, we enter the keyword `struct` and name the entire struct

```rs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }
```

- To use a struct after we've defined it, we create an instance of that struct by specifying concrete values for each of the fields

```rs
    let user1 = User {
        email: String::from("johndoe@gmail.com"),
        username: String::from("Sally"),
        sign_in_count: 1,
        active: true,
    }
```

- To get a specific value from a struct, we can use the dot notation
- If the instance is mutable, we can change the value by using the dot notation and assigning into a particular field
- The entire instance must be mutable, Rust doesnt allow us to mark only certain fields as mutable
- we can also return structs from a function

```rs
    println!("email: {}", user1.email);

```

### Using the Field init shortcurt when Variables and Fields have the same name

```rs
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true
        }
    }
```

- Because the email field and the email parameter have the same name, we only need to write `email` rather than `email: email`.

### Creating Instances from Other Instances with Struct Update Syntax

- its often useful to crate a new instance of a struct that uses most of an old instance value but changes some.

```rs
let user1 = User {
        email: String::from("johndoe@gmail.com"),
        username: String::from("Sally"),
        sign_in_count: 1,
        active: true,
    }

let user2 = {
    email: String::from("janedoe@gmail.com"),
    username: String::from("Janey"),
    ...user1,
}
```

- The code creates an instance in user2 that has a different value for email and username but has the same values for the active and sign_in_count fields from user1.

### Using Tuples Struct without named fields to create different Types

- we can define structs that look similar to tuples called `tuple structs`
- Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type than other tuples, and naming each field as in a regular struct would be verbose or redundant.

```rs
    struct Color (i32, i32, i32);
    let black = Color(i32, i32, i32);
    struct Point (i32, i32, i32);
    let origin = Point(0, 0, 0);
```

- **NB**: Each struct you define is its own type, even though the fields within the struct have the same type. eg - a function that takes a parameter of type `Color` cannot take a `Point` as an argument, even though both types are made up of three i32 variables.
- we use dot notation followed by the index to access each field.

### Unit like Structs without any field

- we can also define structs that dont have any fields. These are called unit like structs because the behave similarly to `()`, the unit type.
- unit like structs can be useful in situations in which you need to implement a trait on some type but dont have any data that you want to store in the type itself.

```rs
    struct Human {}; //unit like struct
```

### Ownership of Struct Data

- in the examples above, we use the owned string type rather than the string slice `&str` type. This is was a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
- it is also possible for structs to store references to data owned by something else, but to do so we require the use of `lifetimes`.
- Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
- If we try to store a reference in a struct without specifying a lifetime, it wont work _more on lifetimes in chapter 10_.

### Adding useful Functionality with Derived traits

- it will be nice to be able to print an instance of a struct while we are debugging our program and see the values for all its fields.

```rs
  println!("user 1 is {}");
```

- The code above wont work. The `println!` macro can do many kinds of formatting and by default the curly brackets tell `printlin!` to use a formatting know as `Display` which is output intended for direct consumption.
- The primitive type we've seen so far implement the `Display` trait by default, because there is only one way to display a primitive type. But with structs, the way `println!` should format the output is less clear because there are more display possibilities.
- Due to this ambiguity, Rust doesnt try to guess what we want and so structs do not have a provided implementation of `Display`.
- Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
- The `Debug` Trait enables us to print out our struct in a way that is useful for developers so we can see its value while we are debugging.
- We also have to add the annotation `#[derive(Debug)]` just before the struct for this to work.

```rs
    #[derive(Debug)]
    struct Rectangle {
        ...
    }
```

- also to make the displayed content beautifully formatted we can use the specifier `{:#?}` instead of just `{:?}` in the println! string.

### Method Syntax

- methods are similar to function - they are both declared with the `fn` keyword.
- However, methods are different from function in that they are defined within the context of a struct(or enum or trait object), and their first parameter is always `self` which represents the calling struct instance.

### Defining methods

- To define the function within the context of the Struct, we start an `impl`(implementation) block. Then we add the area function within the `impl` scope

```rs
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {width: 10, height: 21};
    println!("area of the rect = {}", rect.area());
```

- A method must have the self as the first parameter.
- `&self` was chosen because we dont want to take ownership of the variable, we just want to read the data and not write to it.
- if we wanted to change the instance that we've called the method, we'd use `&mut self` as the first parameter.
- having a method take ownership of the instance by using using `self` is rare.
- **The main benefit of using methods instead of functions is for organization**

### Methods with more Parameters

```rs
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
```

### Associated Functions

- Another useful feature of `impl` blocks is that we are allowed to define function within the `impl` blocks that dont take `self` as parameter.
- These are called **Associated Functions** because they are associated with the struct itself.
- They are still functions but not methods because they dont have an instance of the struct to work with.
- **In practice, Associated Functions are often used for constructors that will return a new instance of the struct**

```rs
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {width: size, height: size}
        }
    }
    let square = Rectangle::square(12);
```

- The `::` syntax is used for both associated functions and namespaces created by modules.

### Multiple Impl blocks

- Each struct is allowed to have multiple `impl` blocks.
- We'll see a case in which multiple `impl` blocks are useful in chapterr 10 where we discuss generic types and Traits.

### Summary

- Structs lets us create custom types that are meaningful for our domain.
- By using structs we keep associated pieces of data connected to each other and name each piece to make our code clear.
- Methods lets us specify the behaviour that instances of our structs should have.
- Associated Functions lets us namespace functionality that is particular to our struct without having an instance available.
