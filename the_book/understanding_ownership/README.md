# Understanding Ownership

**Overview:** Ownership enables Rust to make memory safety guarantees without needing a garbage collector. In this chapter, we discuss - ownership, borrowing and slices, and how Rust lays data out in memory.

### What is Ownership?

- All programs have to manage the way they use a computers memory while running.
- With Rust, Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
- We will learn ownership by working through some examples that focus on a very common data structure `String`.

### The Stack and the Heap

- Both the stack and the heap are parts of memory that is available to our code to use at runtime, but they are structured in different ways.
- The **STACK** stores values in the order it gets them and removes the values in the opposite order - referred to as **Last In, First Out (LIFO)**
- The Stack is fast because of the way it access the data; it never has to search for a place to put a new data or a place to get data from because that place is always at the top!
- Another property that makes the stack fast is that all data on the stack must take up a known **Fixed Size**.

* Data with a size unknown at compile time or a size that might change will be stored in the **Heap**

- The **HEAP** is less organized. When you put data on the heap, you ask for some amount of space, The Operating System finds an empty spot some where in the heap that is big enough and then marks that spot as being used and then it returns a **`pointer`** address of that location.
- Because a `pointer` is a known fixed size, it is stored in the stack but when we want the actual data, we have to follow the pointer.
- Accessing data in the heap is slower than accessing data in the stack because to get the data, you have to follow the pointer to get there.

### Ownership Rules

- Each Value in Rust has a variable that is called its **owner**.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.

### Variable Scope

- A scope is the range within a program for which an item is valid.

```rs
    {
        let s = "hello"; //s in in scope
        //do sth with s
    }// scope is now over, s is no longer valid
```

### The String Type

- To illustrate the rules of ownership properly, we need a data type that is more complex.
- The primitive types we covered earlier are all stored on the stack and popped off the stack when their scope is over, but we want to look at data store on the heap and explore how Rust knows when to clean up the data.
- The **String** type is allocated on the heap and is able to store an amount of text that is unknown to us at compile time.

```rs
    let mut s = String::from("Hello");
    s.push_str(" World!");
```

### Memory Allocation

- With the String type, in order to support mutable, growable piece of text, we need to allocate an amount of memory on the heap unknown at compile to build the contents. This means that:
  - Memory must be first be requested from the operating system at runtime.
  - We need a way to return this memory to the operating system when we are done with our String.
- The first part is done when we call the `String::from`, it requests memory it needs.
- It is our responsibility to identify when memory is no longer being used, and then call code to explicitly return the memory, just as we requested it. **Doing this will be really tedious and difficult**
- With Rust, the memory is automatically returned once the variable that owns it goes out of scope!
- When a variable goes out of scope, Rust calls a `drop` function that cleans up the heap memory for that variable.

### Ways That Variable And Data Interact: MOVE

- Multiple variable can interact with the same data in different ways in Rust.

```rs
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x , y);
```

- Here we bind `x` to 5, and then make a copy of `x` and bind it to `y`. we now have two variables `x` and `y` which are both equal to 5.

```rs
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2) // This wont compile
```

- This doesnt work as the previous example with the integer, in that the second line will be a copy of the value `s1` and bind to `s2`

- A `String` is made up of three(3) parts:

  - A pointer to the memory that holds the contents of the `String`.
  - The length (how much memory in bytes, the contents of the string is currently using.)
  - The capacity (total amount of memory in bytes that the string has received from the operating system)
    **NB: There is difference between length and capacity**

- When we assign `s1` to `s2`, the string data is copied, meaning we copy the pointer, length and capacity that are on the stack. **We do not copy the actual data on the heap that the pointer refers to**.

* Earlier, we said that when a variable goes out of scope, Rust automatically calls a drop function that cleans up the heap memory for the variable. but with the string example both pointers of the variable point to the same location.**This is a problem because when s1 and s2 goes out of scope they will try to free same memory**

- This is an error known as **Double Free Error** and this is a memory safety bug. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

- The way Rust ensures memory safety for this scenario is that instead of trying to copy the allocated memory, Rust considers `s1` to no longer be valid and therefore Rust does no longer need to free anything when `s1` goes out of scope.
- Rust invalidates the first variable, this is known as a **move** in Rust. In this example we say that `s1` was moved to `s2`. That solves our problem with only `s2` valid, so that when it goes out of scope, it alone will free the memory and we are done.

### Ways That Variable And Data Interact: CLONE

- If we want to deeply copy the String data in the heap and not just the stack, we use a common method called **clone**

```rs
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2)
```

### Stack-Only Data: Copy

```rs
    let x = 10;
    let y = x;

    println!("x = {}, y = {}", x , y);
```

- This code just contradicts what we just learned about the clone method.
- We do not have a call to `clone` but `x` is still valid and was'nt moved into `y`.
- The reason is that types like integers that have fixed sizes at compile time and are store entirely on the stack are quick to copy.
- If a type implements a `Copy` trait, older variables will still remain usable after assignment.
- Rust wont let us annotate a type with `Copy` trait, if the type or any part of the type has implemented a `Drop` trait.
- Here are some of the types that implement the `Copy` Trait
  - integers
  - bool
  - character
  - floating points
  - tuples **NB: Only if they contain types that are also `Copy`**

### Ownership And Functions

- The semantics for passing value to a function are similar to those for assigning a value to a variable.
- Passing a variable to a function will move or copy, just as assignment does.

### Return Values and Scope

```rs
    fn main() {
        let s = new();

        println!("s = {}", s);
    }

    fn new() -> String {
        let s = String::from(" world");
        s
    }
```

- Returning values can also transfer ownership.
- The ownership of a variable follows the same pattern every time:
  - Assigning a value to another variable **moves** it.
  - When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop`, unless the data has been moved to be owned by another variable.
  - Taking ownership with every function and returning ownership with every function is a bit tedious. what if we want to let a function use a variable but not take ownership?

### References And Borrowing

- Here is how we could define a function that has a reference to an object as a parameter instead of taking ownership of the variable.

```rs
    fn main () {
        let s = String::from("Hello");
        let len = calc_len(&s);
    }

    fn calc_len(s: &String) -> usize {
        s.len()
    }
```

- notice that we pass `&s` into the fn and we take `&String` rather than `String` as the parameter
- These ampersands are called `references`, and they allow us to refer to some value without taking ownership of it.
- the opposite of referencing by usng `&` is called `dereferencing`, which is accomplished with the dereference operator (`*`)
- The `&s` syntax lets us create a reference that refers to the value of `s` but does not own it. Because it does not own it, the value it points to will not be dropped when the reference is out of scope.
- We do not drop what the reference points to when it is out of scope because we do not own it in the first place.
- When functions have references as parameters instead of actual values, we do not need to return the values in order to give back ownership, because we never had ownership!
- We call have references as function parameter **`Borrowing`**
- Just as variables are immutable by default, so are references. we are not allowed to modify sth we have a reference to.

### Mutable Reference

- To make a reference mutable we add the `mut` keyword before the `&`
- But mutable references have one big restriction: **You can only have one mutable reference to a particular piece of data in a particular scope**

```rs
    let mut s = String::from("Bro");
    let r1 = &mut s;
    let r2 = &mut s; //wont compile
```

- The code above will fail!
  = This restriction allows for a controlled mutation.
- The benefit of this restriction is that Rust prevents **Data Races** at compile time.
  **Data Races is similar to race conditions** - it happens when these 3 behavior occur. 1) Two or more pointers access the same data at the same time. 2) At least one of the pointer is being used to write to the data. 3) There is no mechanism being used to synchronize access to the data.

* Data races are hard to diagnose and fix at run time, Luckily Rust handles this at compile time. here is a code that works

```rs
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; //since r1 is out of scope and has been dropped this work perfectly
```

- A similar rule exists for combining mutable and immutable references

```rs
    let mut s = String::from("Hello");

    let r1 = &s; //ok
    let r2 = &s; //ok

    let r3 = &mut s; //error here
```

- we cannot have a mutable reference to a variable while we have an immutable one in the same scope.
- multiple immutable references are okay since the only read the data and dont write.

### Dangling References

- Dangling references or pointers is a pointer that references a location in memory that has been freed up.
- In Rust, the compiler guarantees that such wouldnt happen.

```rs
    fn main () {
        let s = dangle();
    }

    fn dangle () -> &String {
        let s = String::from("dangle");
        &s
    }
```

- The code wont compile because `s` in the dangle will be dropped when it goes our of scope at the end of the function but we are trying to return a reference to `s`. This means the reference will be pointing to an invalid string.

### The Slice Type

- Slices lets us reference a contiguous sequence of element in a collection rather than the whole collection
- This data type does not have ownership.

### String Slices

- A String slice is a reference to part of a String.

```rs
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
```

- So rather than a reference to the entire string, its a reference to a portion of the string.
- The `start..end` syntax is a range that begins at `start` and continuous up to but not including the `end`.

### Rusts Range Syntax

```rs
    &s[0..2];
    &s[..2];

    &s[3..len];
    &s[3..];

    &[0..len];
    &[..];
```

### String Literals are Slices

```rs
let s = "Hello World";
```

- Remember that string literals are stored inside the binary.
- The type here is `&str` - it is a slice pointing to the specific point of the binary. This why string literals are immutable. because they have fixed size.
- `&str` is an immutable reference

### String Slices as Parameters

`fn first_word(s: &str) -> &str {}`

- If we have string slices we can pass that directly
- If we have `String`, we can pass a slice of the entire string

```rs
    let my_string = String::from("Hello World");
    let my_string_first_word = first_word(&my_string[..]);
    let literal = "Hello There";
    let literal_first_word = first_word(&literal[..]);
    let literal_first_word = first_word(&literal);
    let literal_first_word = first_word(literal); // because string literals are string slices so this works
```

### Other Slices

- String slices are specific to Strings, there are more general slice types too

```rs
    let a [1,2,3,4,5];
    let slice = &a[0..2];
```

- The slice has a type of `&a[i32]`. it also works as the same way a String slice do by storing a reference to the first element and length.

### Ownership Exercises

- Write a function that takes two mutable references to integers and swaps their value
- Write a function that takes a slice of integers and returns the average of the numbers
- Write a function that concatenates two string slices and returns a new string
- Write a function that takes a slice of integers and returns the largest value
