# Generic Types, Traits & Lifetimes

- Generics are abstract stand-ins for concrete types or other properties
- In this chapter, we'll explore how to define our own types, functions and methods with generics.

## Generic Data types.

- we can use generics to create definition of items like function signatures or structs which can then use with many different concrete data types.

#### In Function Definition

```rs
    fn largest<T>(list: &[T]) -> T {}
```

- we read this definition as the function `largest` is generic over some types T.
- The function has one parameter named `list`, which is a slice of values of type `T`. The largest function will return a value of the same type `T`

#### In Struct Definition

```rs
    struct Point<T> {
        x: T,
        y: T,
    }
```

- also if we wanted to define a struct in which x andy y are both generics but could have different types, we can use multiple generic type parameters

```rs
    struct Point<T, U> {
        x: T,
        y: U,
    }
```

### In Enum Definition

```rs
    Option<T> {
        Some(T),
        None
    }

    Result<T, E> {
        Ok(T),
        Err(E)
    }
```

#### In Method Definitions

```rs
    struct Point<T> {
        x: T,
        y: T,
    }

    Impl<T> Point<T> {
        fn new(x: T, y: T) -> T {
            Point(x, y)
        }
    }
```

```rs
    Impl Point<f32> {
        fn distance(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
```

- This code means that type Point<f32> will have a method named distance and any other instance of Point<T> where T is not of type f32 will not have this method defined.

```rs
    struct Point<T, U> {
        x: T,
        y: U
    }
    Impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: Other.y
            }
        }
    }
```

The purpose of the example is to show a situation in which some generic parameters are declared with `impl` and some are declared with the method definition.

- Here, the generic parameters T & U are declared after impl, because they go with the struct definition. The generic parameters V & W are declared after `fn mixup` because they are relevant to the method.

## Performance of Code Using

- Rust implements generics in such a way that our code doesnt run any slower using generic types than it would with concrete types.
- Rust accomplishes this by performing **Monomorphization** o the code at compile time.
- Monomorphization is the process of turning generic code into specific code by filing in the concrete type that are used when compiled.

## Traits: Defining Shared Behavior

- A trait tells the Rust compiler about functionality a particular type has and can share with other types.
- We can use traits to define shared behavior in an abstract way.
- We can use traits bounds to specify that a generic can be any type that has certain behavior.
- _traits are similar to interfaces in other languages, but with some differences_

### Defining a Trait

- A type's behavior consists of the methods we can call on that type.
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

```rs
   pub trait Summary {
       fn summarize(&self) -> String
   }
```

- A trait can have multiple methods in its body.
- each type implementing this trait must provide its own custom behavior of the body of the method.

### Implementing a Trait on a Type

```rs
   pub struct Tweet {
       pub username: String,
       pub content: String,
       pub reply: bool,
       pub retweet: bool
   }
   impl Summary for Tweet {
       fn summarize(&self) -> String {
           String::from("This is a summary implementation")
       }
   }

   let tweet = Tweet{...};
   tweet.summarize();
```

- One restriction to note with trait implementation is that we can implement a trait on a type only if either the trait or the type is local to our crate

```rs
   impl Display for Tweet {} // the Tweet struct is local ie we defined it so this will work
   impl Summary for Vec<T> {} // The Summary trait is local ie we defined it so this will work but the Vec<T> is defined by the std library
   impl Display for Vec<T> {} // this wont work because none a defined local by our crate
```

- This restriction is part of a property of programs called **Coherence** and more specifically the **Orphan Rule**
- These rules ensures that other peoples code cant break our code and vice versa.
- Without the rule, two crates would implement the same Trait for the same type and Rust wouldnt know which implementation to use.

### Default Implementation

- sometime, its useful to have default behavior for some or all of the methods in a trait instead of requiring implementation for all methods on every type.

```rs
   pub trait Summary {
       fn summarize(&self) -> String {
           String::new("Read More")
       }
   }
```

### Trait Bounds

- we can use traits bounds to constrain generic types to ensure the type will be limited to those that implement a particular trait and behavior.

```rs
    pub fn notify(T: Summary)(item: T) {
        println!("Breaking news: {}", item.summarize());
    }
```

- we can also specify multiple trait bounds on a generic type using the `+` syntax or we can use the `where` clause to make the function more readable.

```rs
    pub fn some_fn<T: display + clone, U: clone + Debug>(t: T, u: U) -> i32 {}

    fn some_fn<T, U>(t: T, u: U) ->
    where T: Display + Clone, U: Clone + Debug {}
```

### Using Trait Bounds to Conditionally Implement Methods

- By using a trait bound with `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

```rs
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new (x: T, y: T) -> Pair {
            Pair {x, y}
        }
    }

    impl<T: Display + PartialOd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("x is greater than y: {x}");
            }else {
                println!("x is lesser than y: {y}");
            }
        }
    }
```

- implementation of a trait on any type that satisfies the trait bounds are called **`blanket implementation`**

## Validating References with Lifetimes

- lifetimes ensures that references are valid as long as we need them to be.
- Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
- Most of the time, lifetimes are implicit and inferred, just like most of the time a type is inferred.
- The same way we annotate types when multiple types are possible, we also annotate lifetimes when the lifetimes of references could be related in a few different ways.

### Preventing Dangling References with lifetimes

- The main aim of lifetimes is to prevent dangling references, which causes the program to reference data other than the data it is intended to reference.

```rs
    {
        let r = 8;
        let x = &r;
    }
    println!("{x}"); // this wont compile
```

- This would not compile because x is no longer in scope which means there is no reference to it. Rust is able to know that at compile time using the **`borrow checker`**

### The Borrow Checker

- This is a feature in Rust compiler that compares scopes to determine whether all borrows are valid.

### Generic Lifetimes in Functions

```rs
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        }else {
            y
        }
    }
```

- This code will not compile, the error message it will display indicates that the return type need a generic lifetime parameter because Rust cant tell if the reference being returned refers to x or y.
- The borrow checker is not able to infer the lifetimes of x and y and how it relates to the return type.
- To fix this issue, we will add a **`generic lifetime`** parameter that define the relationship between the references so the borrow checker can have what to work with.

### Lifetime Annotation Syntax

- Lifetime annotations dont change how long any of the references live.
- Lifetime annotations describes the relationships of the lifetimes of multiple references to each other without affecting lifetimes.

```rs
    &i32 // a reference
    &'a i32 // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime Annotations in Function Signatures

```rs
    fn longest<'a>(x: &'a str, y: &'a str) {}
```

- The function signature is read as - for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as the lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a
- These constraints are what we want Rust to enforce.
- This also means all references in the parameter and return type must have the same lifetime.
- When annotating Lifetimes in Functions, the annotation goes in the function signature and not in the function body.

- The generic lifetime will get the concrete lifetime that is equal to the shortest/smallest lifetime of x and y argument.
- Also, because we've annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the smaller of the lifetimes of x and y.

### Thinking in terms of Lifetimes

- the way we specify lifetime paramters depends on what our function is doing.
- eg, if we changed the longest function to always return the first paramter, we dont need to specify the lifetime of the y paramter.

```rs
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
```

- we've not specified a lifetime parameter for y because y does not have any relationship with x or the return value of the function.
- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the arguments.
- Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. With this Rust has enough information to allow memory safe operations and disallow operations that would create dangling pointers.

### Lifetime Annotations in Struct Definitions

- Structs can also hold references, but for this to happen, we need to add lifetime annotations on every reference in the struct definition.

```rs
    struct ImportantExcerpt<'a> {
        part: &'a str
    }
```

- This annotation means that an instance of ImportantExcerpt struct cannot outlive the reference it holds in the part field.

### Lifetime Elison

- They were examples in which a function had a parameter which was a reference and a return type which was a reference but we did not have lifetime annotation on these function. eg

```rs
    fn first_word(x: &str) -> &str {}
```

- The rust compiler has a set of rules it follows when it encounters situations like this we call this rules the **`Lifetime elison rules`**
- These rules dont provide full inference. If Rust applies the whole rules and there is still ambiguity, as to the lifetime of reference, the compiler will throw error.
- Lifetimes on function or methods parameters are called **`input lifetimes`** and lifetimes on return values are called **`output lifetimes`**.
- The compiler uses three(3) rules to figure out what lifetime references have if there arent explicitly annotated.

1. Each Parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameter.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because its a method, the lifetime of self is assigned to all output lifetime parameters

### Lifetime Annotations in Method Definition

```rs
    impl<'a> ImportantExcerpt<'a> { //lifetime parameter is required here
        fn level(&self) -> i32 { //lifetime parameter is not required here
            1
        }

        fn another(&self, anno: &str) -> &str {
            //
        }
    }

```

- The third lifetime elison rule applies to the another method, there are multiple reference parameter but also a &self parameter, which means the output lifetime will be the lifetime of the &self parameter.

### The Static Lifetime

- Rust has a specified lifetime called `'static` which denotes the entire duration of the program. -**All string literals have a 'static lifetime**
- The text of these strings are stored directly in the binary of our program
- Some suggest we use 'static lifetimes in error messages

### Generic Type Parameters, Trait Bounds and Lifetime together

```rs
    use std::fmt::Display;

    fn longest_with_annotation<'a T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display {
        println!("Annotation {}", ann);
        if x.lent() > y.len() {
            x
        }else {
            y
        }
    }
```
