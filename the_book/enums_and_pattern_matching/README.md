# Enums & Pattern Matching

overview: enums allow us to define a type by enumerating its possible values.

- first we'll define and use an enum and also show how to encode meaning along with data.
- we'll look at the `Option` enum.
- we'll also look at pattern matching.
- we'll also cover the `if let` construct.

### Defining an Enum

```rs
    enum IpAddrKind {
        V4,
        V6,
    }
```

### Enum Values

- we can create instances of each of the two variants of `IpAddrKind` like this.

```rs
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

- note, the variants of the enum are namespaced under its identifier and we use a double colon `::` to separate the two.
- both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type `IpAddrKind`
- we can then for instance define a function that takes any `IpAddrKind`

```rs
    fn route(ip_type: IpAddrKind) {}
```

- we can call this function with either variant.

-At the moment we dont have a way to store the actual ip address data; we only know what kind it is.

- we could put data directly into each enum variant.

```rs
    enum IpAddr {
        V6(String),
        V4(String)
    }
```

- This new `IpAddr` enum says that `V6` and `V4` variants will have associated String values.

```rs
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
```

- here is another example of an enum with wide variety of types embedded in its variants

```rs
    enum Message {
        Quit,
        Move {x: i32, y: i32} //anonymous struct,
        Write(String),
        ChangeColor(i32, i32, i32),
    }
```

- Just as we are able to define methods on structs using `Impl`, we are also able to define methods on enums.

```rs
    Impl Message {
        fn call(&self) {
            // do sth
        }
    }
    let m = Message::Write(String::from("Hello World"));
    m.call();
```

### The Option Enum and Its Advantage over Null Values

- The `Option` enum is an enum defined by the standard library.
- The `Option` type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing.
- Expressing this concept in terms of the type system means the compiler can check whether you've handled all the cases you should be handling.
- This Functionality can prevent bugs that are extremely common in other programming languages.

**NB: It is also important to note that Rust does not have a Null feature that other programming languages have**

- `Null` is a value that means there is no value there.
- In Languages with null, variables can always be in one of two states null or not null.
- The problem with null values is that if you try to use a null value as a not-null value, you'll get an error of some kind. It is extremely easy to make this kind of error.
- Rust does not have null, but it does have an enum that can encode the concept of a value being present or absent, this is using the `Option<T>` enum.

```rs
    enum Option<T> {
        Some(T),
        None
    }
```

- The `Option<T>` enum is so useful that it is included in the prelude for us which means we dont have to bring it into scope explicitly.
- The `<T>` syntax is a feature of Rust. it is a generic type parameter. We'll see more on Generics in Chapter 10.

```rs
    let some_number = Some(5);
    let some_string = Some(String::from("Hello"));
    let absent_number: Option<i32> = None;
```

- When we use `None` rather than `Some`, we need to explicitly tell Rust what type of `Option<T>` we have, because the compiler cannot infer the type that the Some variant will hold by looking only at a None value.

```rs
    let x: i8 = 5;
    let y: Option<i8> = Some(12);

    let sum = x + y; //this code wont compile because we try to add variables of different types
```

- We have to convert an `Option<T>` to a `T` before we can perform any operation on `T`.

* _Not having to worry about incorrectly assuming a not-null value helps us be more confident in our code. If we are expected to have a value that is possibly null, we must **explicitly** opt to making the type of the value `Option<T>`._

- Then when we want to use the value, we are required to explicitly handle the case when the value is absent.
- In general, in order to use an `Option<T>` value, you want to have a code that will handle each variant.
- We want to run some code that will run when we have `Some(T)` value and the code is allowed to use the inner `T`. we also want some other code to run if the value is `None`.
- The **match** expression is a control flow construct that does just this when used with enums; it will run different code depending on which variant of the enum it has.

### The match control flow Operator

- Rust has an extremely powerful control flow operator called **match** that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
- Patterns can be made up of literal values, variable names, wildcards and many other things.

```rs
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
```

- This seems very similar to an expression used with `if`, but there is a big difference: with `if`, the expression needs to return a Boolean, but with match, it can be any type.
- When the `match` expression executes, it compares the resulting value against the pattern of each arm, in order.
- The code associated with each arm is an expression and the resulting value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

### Patterns that Bind to Values

- match arms can also bind to the parts of the values that match the pattern.
- This is how we can extract values out of enum variants

```rs
    enum Message {
        Greeting(String),
    }
    fn value_in_message(message: Message) -> String {
        match message {
            Message::Greeting(msg) => {
                println!("{}", msg);
                msg
            }
        }
    }

    value_in_message(Message::Greeting("Hello my people!"));
```

### Matching with `Option<T>`

```rs
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None,
            Some(num) => Some(num + 1),
        }
    }

    let five = plus_one(Some(4));
    let six = plus_one(five);
```

- Combining match and enums is useful in many situations. You'll see this patterns a lot in Rust Code.
  - Matching against an enum
  - Binding a variable to a data inside it
  - execute the code based on it

**Matches are exhaustive in Rust, we must handle every single possibility in order for the code to compile or work**

### The \_ Placeholder

- Rust also has a pattern we can use when we dont want to list all possible values. for eg, a `u8` can only hold values from `0-255`, if we only cared about `1, 2,4`, then trying to handle all the other values will be cumbersome. Rust gives us a catch all in the underscore(\_) placeholder. we use this to match any remaining value which wasnt specified.

```rs
    fn match_num(x: i8) {
        match x {
            1 => println!("One!!!"),
            2 => println!("Two!!!"),
            4 => println!("Four!!!"),
            _ => ()
        }
    }
```

**NB: It is also important to note that the \_ must come last**

### Concise Control Flow with if let

- the if let syntax lets us combine `if let` into a less verbose way to handle values that match one pattern while ignoring the rest.
- Using `if let` means less typing, less indentation and less boilerplate code. However we loose the exhaustive checking that match enforces.
- Choosing between `match` and `if let` depends on what you are doing in your particular situation.
- `if let` is seen as syntactical sugar of a match that runs code when the value matches one pattern and then ignores all other values.

```rs
    //using match
    let some_u8_value = Some(10);
    match some_u8_value {
        None => (),
        Some(10) => println!("value is Ten!!!")
    }

    //using the if let construct
    if let Some(10) = some_u8_value {
        println!("value is Ten!!!");
    }
```

- The `if let` takes a pattern and an expression separated by an equal sign respectfully.
- We can also include `else` within an `if let`. The else catches the other cases.

```rs
if let Some(10) = some_u8_value {
        println!("value is Ten!!!");
    }else {
        println!("Gotcha!!!");
    }
```
