# 19) Advanced Features

### Unsafe Rust

- Rust has a second language hidden inside that doesn't enforce memory safety guarantees.
- If you use unsafe code incorrectly, problems due to memory safety such as **`null pointer dereferencing`** can occur.
- Another reason Rust has unsafe "alter ego" is that the underlying computer hardware is inherently unsafe. If Rust didn't let us do unsafe operations, we couldn't do certain tasks.
- Rust needs to allow us to do low-level system programming such as directly interacting with the Operating System or even writing our own OS.

### Unsafe Superpowers

- To switch to unsafe Rust, we use the `unsafe` keyword and then start a new block that holds the unsafe code.
- You can take four(4) actions in unsafe Rust called the **unsafe superpowers** and they include:

1. Dereference a raw pointer.
2. Call an unsafe function or method.
3. Access or modify a mutable static variable.
4. Implement an unsafe trait.

- It's also important to note that unsafe doesn't turn off the borrow checker or disable any other of Rust safety checks. The unsafe keyword only gives us access to these four features that are then not checked by the compiler for memory safety.
- Its also essential to keep unsafe blocks small so we could catch memory bugs early.

- It is best to enclose unsafe code within a safe abstraction and provide a safe API.

### Dereferencing a Raw Pointer

- Unsafe Rust has two(2) new types called **`Raw Pointers`** that are similar to references.
- Raw Pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively.
- In The context of raw pointers, immutable means that the pointer cant be directly assigned to after being dereferenced.
- Raw pointers are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location.
- Raw pointers are not guaranteed to point to a valid location.
- Raw pointers are allowed to be **null**.
- Raw Pointers don't implement any automatic cleanup.

```rs
    let mut num = 5;
    let r1 = &num as *const i32; //immutable raw pointer from a reference
    let r2 = &mut num as *mut i32; //mutable raw pointer from a reference
```

- Notice that we didn't include the unsafe keyword in this code.
- We can create a raw pointer in safe code but we cannot dereference raw pointers outside an unsafe block.
- We know that this particular raw pointers are valid because we created them directly from references guaranteed to be valid.
- Here is an example of a raw pointer to an arbitrary location whose validity isn't certain

```rs
    let address = 0x12345usize;
    let r = address as *const i32;
```

- To dereference a raw pointer we use the dereference operator `*` in an unsafe block

```rs
    let mut num = 5;
    let r1 = &num as *const i32; //immutable raw pointer from a reference
    let r2 = &mut num as *mut i32; //mutable raw pointer from a reference

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
```

- Creating a raw pointer does no harm, its only when we try to access the value that it points at that we might end up dealing with invalid values.
- With Raw pointers we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a **data race**.
- So with the risk of creating **data race** why should we ever use raw pointers?

1. We could use them when interfacing with a C code.
2. To build a safe abstraction that the borrow checker doesn't understand.

### Calling an Unsafe Function or Method.

```rs
    unsafe fn dangerous() {
        //do sth
    }

    unsafe {
        dangerous();
    }
```

- We must call the dangerous function within a separate unsafe block. if we try to call `dangerous` without the unsafe block our code wont compile.
- Bodies of unsafe function are effectively unsafe blocks so to perform other unsafe operation within an unsafe function we don't need to add another unsafe block.

### Creating a Safe Abstraction over unsafe code

- Just because a function contain unsafe code doesn't mean we have to mark the function as unsafe.
- As an example lets study the standard library `split_at_mut` method that requires some unsafe code and explore how we might implement it.
- This safe method is defined on a mutable slice. it takes one slice and makes it two by splitting the slice at the index given as argument.

```rs
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut[1, 2, 3])
    assert_eq!(b, &mut[4, 5, 6, 7])
```

- If we attempt to use only safe Rust to implement this method it wont compile. Here is what an attempt to use safe Rust looks lik.

```rs
    fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
        let len = slice.len();

        assert!(mid <= len);

        (&mut slice[..mid], &mut slice[mid..])
    }
```

- This code will fail with the reason "cannot borrow \*slice as mutable more than once at a time".
- Rust's borrow checker cannot understand that we're borrowing different parts of the slice. It only knows that we're borrowing from the same slice twice.
- Borrowing different parts of a slice is fundamentally okay because the two slices aren't overlapping, but Rust isn't smart enough to know this.
- When we know our code is okay but Rust doesn't, its time to reach for **unsafe code**.
- The code below shows how to use unsafe block, a raw pointer and some calls to unsafe functions to make the implementation of `split_at_mut` work.

```rs
    use std::Slice;

    fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
        let len = slice.len();

        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (Slice::from_raw_parts_mut(ptr, mid), Slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }
```

- The function `from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid.
- The `offset` method or raw pointer is also unsafe because it must trust that the offset location is also a valid pointer.
- Hence, we have to put an unsafe block around them. This is an acceptable and appropriate use of unsafe.
- We created a safe abstraction to the unsafe code within the implementation of the function that uses unsafe code in a safe way because it creates only valid pointers from the data this function has access to.
- Here is a code that will likely crash using the `slice::from_raw_parts_mut` method. The code takes an arbitrary memory location and creates a slice 10,000 items long.

```rs
    use std::Slice;

    let address = 0x012345usize;
    let rp = address as *mut i32;

    let slice = unsafe {
        Slice::from_raw_parts_mt(rp, 10000)
    }
```

- We don't own the memory at this arbitrary location and there is no guarantee that the slice this code creates contains valid i32 values.
- Attempting to use slice as though its a valid slice results in an undefined behavior.

### Using extern functions to call external code

- Sometimes your Rust code might need to interact with code written in another language.
- For this, Rust has a keyword, **`extern`** that facilitates the creation and use of a foreign function interface(FFI)

- An FFI is a way for programming languages to define functions and enable a different programming language to call those functions.
- The example below shows how to setup an integration with the `abs` function from the c standard library.
- Functions declared withing `extern` block are always unsafe to call from Rust code because other languages don't enforce Rust rules and guarantees so the responsibility falls on the programmer to ensure safety.

```rs
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 is {}", abs(-3));
        }
    }
```

- Within the `extern "C"` block, we list the names and signatures of external functions from another language we want to call.
- The "C" part defines the AbI the external function uses.
- The ABI defines how to call the function at the assembly level.
- The "C" ABI is the most common and follows the C programming language ABI.

### Calling Rust Functions from other languages

- We can also use `extern` to create an interface that allows other languages to call Rust functions.
- Instead of an extern block, we add the extern keyword and specify the ABI to use just before the `fn` keyword
- We also add a `#[no_mangle]` annotation to tell Rust not to mangle the name of the function.
- **Mangling** is where the compiler changes the name of a function to a different name that contains more information for other parts of the compilation process to consume - but it is less human readable.

```rs
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function!!!");
    }
```

- This usage of extern does nto require unsafe.

### Accessing or Modifying a Mutable Static Variable

- In Rust, Global variables are called **static variables**.

```rs
    static HELLO_WORLD: &str = "Hello World!!!";

    fn main () {
        println!("{}, HELLO_WORLD");
    }
```

- static variables are similar to constants
- The names of static variables are in SCREAMING_SNAKE_CASE by convention and we must annotate the variable type which in this example is `'&static str`.
- Static variables can only store references with the `'static` lifetime, so there is no need to annotate the lifetime explicitly.
- Accessing an immutable static variable is safe.
- Constants and Immutable static variables are similar but have a subtle difference.
- Static variables have fixed address in memory. Using the value will always access the same data.
- Constants on the other hand, are allowed to duplicate their data whenever they are used.
- Another difference between constants and static variables is that static variables can be mutable.
- Accessing and modifying mutable static variables is unsafe.

```rs
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    fn main() {
        add_to_count(5);

        unsafe {
            println!("COUNTER = {}", COUNTER);
        }
    }
```

- Any code that reads or writes from COUNTER must be within an unsafe block.
- With mutable data, that is globally accessible, it is difficult to ensure there are no data races, which is why Rust considers mutable static variable to be unsafe.

### Implementing an Unsafe Trait

- A trait is unsafe when at least one of its methods has some invariant that the compiler cannot verify.
- We can declare that a trait is unsafe by adding the unsafe keyword before the trait and marking the implementation of the trait as unsafe.

```rs
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
```

- By using unsafe impl, we're promising that we'll upload the invariants that the compiler can't verify.

### When to use Unsafe code.

- Using unsafe for any of the 4 superpowers discussed earlier.
- It is trickier to get unsafe code correct because the compiler cant help uphold memory safety.

### Advanced Lifetimes

- We'll look at three advanced lifetime features that we haven't covered yet:

1. Lifetime subtyping - Ensures that one lifetime outlives another lifetime.
2. Lifetime Bounds - Specifies a lifetime for a reference to a generic type.
3. Inference of Trait Object Lifetimes - Allows the compiler to infer trait object lifetimes and when they need to be specified.

#### Ensuring One Lifetime Outlives Another with Lifetime Subtyping.

- Lifetime Subtyping specifies that one lifetime should outlive another lifetime.

```rs
     struct context(&str);
     struct Parser {
        context: &Context,
     }
     impl Parser {
        fn parse(&self) -> Result<(), &str> {
            Err(&self.context.0[1..])
        }
     }
```

- Compiling this code results in errors because Rust expects lifetime parameters on the string slice in Context and the reference to Context in Parse.
- So lets rectify this issue.

```rs
struct context<'a>(&'a str);
     struct Parser<'a> {
        context: &'a Context<'a>,
     }
     impl Parser {
        fn parse(&self) -> Result<(), &str> {
            Err(&self.context.0[1..])
        }
     }
```

- This code compiles fine.
- It tells Rust that Parser holds a reference to Context with a lifetime of 'a and Context holds a string slice that also lives long as a reference to the context in Parser.
- The Rust compiler error messages earlier stated that lifetime parameters were required for these references and now we'e added the lifetime parameter.

- Next we'll add a function that takes an instance of context, uses a Parser to Parse that context and returns what parse returns

```rs
    struct context<'a>(&'a str);
    struct Parser<'a> {
        context: &'a Context<'a>,
     }
    impl Parser {
        fn parse(&self) -> Result<(), &str> {
            Err(&self.context.0[1..])
        }

        fn parse_context(context: Context) -> Result<(), &str> {
            Parser {context: &context}.parse()
        }
     }
```

- when we try to compile this code we get a compiler error in summary saying - borrowed value does not live long enough, `context` does not live long enough.
- These errors states that the Parser instance that is created and the context parameter live only until the end of the parse_context function.
- In other words, Parser and context needs to outlive the entire parse_context function and be valid before the function starts as well as after it ends for all the references in this code to always be valid.
- Parser takes a reference to Context and Context is owned by the `parse_content`. context also has a reference to a &str which means context doesn't own the string slice and it should live more than the parser amd context.
- Rust thinks we are trying to return a reference to a value that goes out of scope at the end of of the function because we annotated all the lifetimes with the same lifetime parameters.
- The parse_context function cannot see that within the parse function, the string slice returned will outlive Context and Parser and that the reference parse_context returns refers to the string slice and not context or Parser.
- We need to tell Rust that the string slice in context and the reference to the context in Parser have different lifetimes and the return values of parse_context is tied to the lifetime of the string slice in context.

```rs
struct context<'s>(&'s str);
    struct Parser<'c, 's> {
        context: &'c Context<'s>,
     }
    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }

        fn parse_context(context: Context) -> Result<(), &str> {
            Parser {context: &context}.parse()
        }
     }
```

- This code wont still compile because Rust doesn't know of any relationship between 'c and 's.
- To be valid, the referenced data in Context with lifetime 's needs to be constrained to guarantee that it lives longer than the reference with lifetime 'c.
- Lifetime subtyping specifies that one lifetime parameters lives at least as long as another one.
- We can declare a lifetime 'b that lives at least as lon as a lifetime 'a by using the syntax `'b: 'a`

```rs
    struct Parse <'c, 's: c'> {
    ...
    }
```

#### Lifetime Bounds on References to Generic Types.

- We can add lifetime parameters as constraints on generic types just like we add trait bounds on generic types. This is called **lifetime bounds**.
- Lifetime bounds helps Rust verify that references in generic types wont outlive the data they're referencing.
- As an example, consider the type that is a wrapper over references. recall the `RefCell<T>`.
- These types are wrappers over references that keep track of the borrowing rules at runtime.
- Here is the definition of Ref struct without the lifetime bounds for now: `Ref<'a, T>(&'a T);`
- Rust will throw an error because it doesn't know how long the generic type T will live.
- Because T can be of any type, T could be a reference or a type that holds one or more references, each of which could have their own lifetimes. Rust can't be sure T will live as long as 'a. - `struct Ref<'a, T: 'a> (&'a T);`.
- This code compiles because the `T: 'a` syntax specifies that T can be any types but if it contains any reference the reference must live at least as long as the 'a.
- We could also solve this problem in a different way. ```struct 'static Ref<T: 'static>(&'static T);`
- Because 'static means the reference must live as long as the entire program, a type that contains no reference meets the criteria of all references living as long as the entire program.

### Inference of Trait Object Lifetimes

```rs
    trait Red {}

    struct Ball<'a> {
        diameter: &'a i32,
    }

    impl<'a> Red for Ball<'a> {
        fn main () {
            let num = 5;
            let obj = Box::new(Ball {diameter: &num}) as Box<Red>;
        }
    }
```

- This code will compile successfully even though we haven't explicitly annotated the lifetimes involved in obj.
- This code works because there are rules for working with lifetimes and trait objects which are -

1. The default lifetime of a trait object is 'static.
2. With &'a trait or &'a mut trait, the default lifetime of a trait object is 'a.
3. With single T: 'a clause, the default lifetime of a trait object is 'a.
4. With multiple T: 'a clause, there is no default lifetime, we must explicitly define a lifetime.

- When we must be explicit we add a lifetime bound on a trait object like Box<Red> using the the syntax

```rs
    Box<Red + 'static> or Box<Red + 'a>
```

depending on whether the reference lives for the entire program or not.

### Advanced Traits

#### Specifying Placeholders Types in Trait Definitions with Associated Types

- Associated types connect a type placeholder with a trait such that the trait method definition can use these placeholder types in their signature.
- The implementor of the trait will specify the concrete type to be used in this types place.
- That way, we can define a trait that uses some type without needing to know exactly what those types are until the trait is implemented.

```rs
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {}
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            // do sth
        }
    }
```

- the `type = Item` is a placeholder type.
- Implementor of the Iterator will specify the concrete type for Item.
- Associated types seems similar to generics, so when do we know which to use?
- The syntax seems comparable to that of generics so why not just define the Iterator trait with generics like this -

```rs
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }
```

- The difference is that when using generics, we mst annotate the types in each implementation because we can have multiple implementation for Iterator for Counter.
- With associated types, we don't need to annotate the type because we cannot implement a trait on a type multiple times.

#### Default Generic Type Parameter and Operator Overloading

- When we use generic type parameters, we can specify a default concrete type for the generic type.
- This eliminates the need for implementor of the trait to specify a concrete type if the default type works.
- The syntax for specifying a default type for a generic type is `<PlaceholderType = ConcreteType>`
- An example where this concept is useful is with **Operator Overloading**.
- Operator Overloading is customizing the behavior of an operator(such as + operator) in particular situations
- Rust doesn't allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed by `std::ops` by implementing the trait associated with the operator.

```rs
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main () {
        assert_eq!(Point {x: 1, y: 0} + Point {x: 2, y: 3}, Point {x: 3, y: 3});
    }
```

- The add method adds the x values of two point instances and y values to create a new Point.
- The Add trait has an associated type named Output that determines the type returned from the add method.
- The default generic type in this code is within the Add trait

```rs
    type Add<RHS = Self> {
        type Output;
        fn add(Self, rhs: RHS) -> Self::Output;
    }
```

- The part `<RHS = Self>` is a syntax called **`Default type Parameter`**.
- The RHS generic type parameter defines the type of the rhs parameter in the add method.
- If we do not specify a concrete type for RHS when we implement the Add trait, the type of RHS will default to Self.
- Here is an example where we customize the RHS type rather than using the default

```rs
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(Self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 100));
        }
    }
```

- We use default type parameters in 2 ways

1. To extend a type without breaking existing code.
2. to allow customization in specific cases most users wont need.

#### Fully Qualified Syntax for Disambiguation: Calling Methods with the same name

- Nothing in Rust prevents a trait from having a method with the same name as another trait method, nor does Rust prevents you from implementing both trait on one type.
- It is also possible to implement a method directly on the type with the same name as a method from a trait.

- When calling methods with the same name, we'll have to specify which one we want.

```rs
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("Flying from pilot");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Flying with a broom");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("I am just a human");
        }
    }
```

- When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type.

```rs
    fn main () {
        let person = Human;
        person.fly; //I am just human
    }
```

- To call the fly method from either the Pilot trait or Wizard trait we need to use more explicit syntax to specify.

```rs
    fn main() {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);

        person.fly();
    }
```

- Because the fly method takes a self parameter, Rust could figure out which implementation of a trait to use based on the type of Self.
- However, associated functions that are part of traits don't have a Self parameter and Rust can't figure out which type you mean unless we use **Fully Qualified Syntax**

```rs
    trait Animal {
        fn baby_animal() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_animal() -> String {
            String::from("spot")
        }
    }

    impl Animal for Dog {
        fn baby_animal() -> String {
            String::from("puppy")
        }
    }

    fn main() {
        println!("{}", Dog::baby_animal()) //spot
        println!("{}", <Dog as Animal>::baby_animal()) //puppy

    }
```

- In general, Fully Qualified Syntax is defined as follows

```rs
    <Type as Trait>::function(receiver_if_method, next_arg, ..);
```

- For associated functions, there would not be a receiver, there would only be a list of arguments.

### Using Super traits to Require One Traits Functionality within Another Trait.

- Sometimes, we might need one trait to use another trait functionality.
- In this case, we need to rely on the dependant trait being implemented.
- The trait you rely on is a **`Supertrait`** of the trait you're implementing.

```rs
    use std::fmt

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            ...
        }
    }
```

- Because we've specified that OutlinePrint trait requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display.

```rs
    struct Point {
        x: i32,
        y: i32,
    }
    impl OutputPrint for Point {}
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})", self.x, self.y)
        }
    }
```

### Using the NewType Pattern to Implement External Traits on External Types.

- Earlier we mentioned the orphan rule that states we're allowed to implement a trait on a type as long as either the trait or the type are local to our crate.
- We could get around this restriction using the **newType pattern** which involves creating a new type in a tuple struct.
- The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. Then the wrapper type becomes local to our crate and we can implement the trait on the wrapper.
- As an example, we want to implement Display trait on a Vec<T> type, which isn't possible given the orphan rule because Display trait and Vec<T> type are defined outside our crate.
- We can make a wrapper struct that holds an instance of Vec<T>, then we can implement Display on Wrapper and use the Vec<T> value.

```rs
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write(f, "[{}]", self.0.join(", "))
        }
    }

    fn main () {
        let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
        println!("w = {}", w);
    }
```

- The downside of using this technique is that Wrapper is a new type so it doesn't have the methods of the value its holding. We would have to implement all the methods of the value its holding.

### Advanced Types

#### Using the Newtype Pattern for Type Safety and Abstraction

- The newType pattern is also useful for more tasks including statically enforcing that values are never confused and indicating the units of a value.
- Another use of the newtype pattern is in abstracting away some implementation details of a type - the newtype can expose a public API that is different from the API of the primitive inner type if we used new type directly to restrict the available functionality.

- NewTypes Can also hide internal implementation
- it is a lightweight to achieve encapsulation to hide implementation detail.

#### Creating Type Synonyms with Type Aliases

- Rust provides the ability to declare a type alias to give an existing type another name.

```rs
    type Kilometer = i32;

    let x: i32 = 5;
    let y: Kilometer = 8;

    println!("{}", x + y);
```

- The main usecase of type synonym is to reduce repetition. we might have a length type like this `Box<fn() + Send + 'static>`.
- Writing this lengthy type in function signatures and type annotations over the code can be error prone.

```rs
    type Thunk = Box<fn() + Send + 'static>;
```

- Type aliases are also commonly used with the `Result<T, E>` type for reducing repetition.
- The Result<.., Error> is repeated alot.
- the `std::io` has this type of alias declaration

```rs
    type Result<T> = Result<T, std::io::Error>;
```

- Because this declaration is in the `std::io` module, we can use the fully qualified alias `Result<T>`

#### The Never type that Never Returns

- Rust has a special type named `!` that's known as in type theory lingo as the **empty type** because it has no value.
- It is called the **never type** because it stands in the place of the return type when a function will never return.

```rs
    fn bar () -> ! {

    }
```

- The code is read as "the function bar returns never";
- Functions that returns never are called **Diverging Functions**

```rs
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }
```

- Rust requires that guess has only one type which is u32. So what does continue return? -
- `continue` returns a `!` value
- When Rust computes the type of guess, it looks at both arms, the former with a value of `u32` and the latter with with a value of `!`. Because `!` can never have a value, Rust decodes that the type of guess is u32.
- `panic` `loop` `print` all have type of never.

#### Dynamically Sized Types and the Sized trait

- Due to Rust need to know certain details, such as the how much space to allocate for a value of a particular type there is a corner of its type system that can be confusing.
- The concept of dynamically sized types are some times referred to as **DST** or **unsized types**
- We'll use the `str` which is a DST as an example.
- The `str` not `&str` is a DST, we can't know how long the string is until runtime, meaning we can't create a type `str` nor can we take an argument of `str`.

```rs
    let s1: str = "Hello"; //cannot compile
    let s2: str = "Bro"; //cannot compile
```

- This code wont compile, Rust needs to know how much memory to allocate to any value of a particular type and all values of a type must use the same amount of memory.
- If Rust allowed this code, the two str values would need to take up the same amount of space, but they have different lenghts.
  -s1 needs 5 bytes and s2 needs 3 bytes.
- This is why its not possible to create a variable holding a DST.
- we rather make the type of s1 and s2 a &str rather than str.
- String slices data structure stores the starting position and the length of the slice. ie - &str contains 2 values.
- With this we can know the size of a &str value at compile time. it is twice the length of a `usize`
- In general this is the way in which dynamically sized types are used in Rust. They have extra bit of metadata that stores the size of the dynamic information.
- The golden rule of DST is that we must put values of DST behind a pointer of some kind.
- We can combine str with all kinds of pointer, eg - `Box<str>`, `RC<str>`
- We've encountered this before but with a different DST called Traits.
- Every trait is a DST we can refer by using the name of the trait.
- Previously we stated that to use traits as trait objects we must put them behind a pointer such as `&trait`, `Box<trait>`, `RC<trait` etc
- To work with DST, Rust has a particular trait called **Sized trait** that determines whether or not the type size is known at compile time.
- This trait is automatically implemented for everything whose size is known at compile time.
- Also, Rust implicitly adds a bound on Sized to every generic function.

```rs
    fn generic<T>(t: T) {

    }

    fn generic<T: Sized>(t: T) { //behind the scenes

    }
```

- By default, generic functions will work only on types that have a known size at compile time.
- We can use a special syntax to relax on this restriction

```rs
fn generic<T: ?Sized>(t: &T) { //behind the scenes

    }
```

- A Trait bound on ?Sized is the opposite of a trait bound on Sized - it would be read as "T may or may not be Sized"
- This `?Sized` syntax is only available for Sized not any other traits.
- Also notice we switched the type of t parameter from T to &T because the type might not be Sized, we need to use it behind some kind of pointer: in this case we chose a reference.

### Advanced Functions and Closures

#### Function Pointers

- We've talked about how to pass closures to functions, we can also pass regular functions to functions
- Doing this with function pointers will allow us to use functions as arguments to other functions.
- Function coerce to the type `fn` not to be confused with the `Fn` closure trait.
- the `fn` type is a function pointer.

```rs
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn main () {
        let ans = do_twice(add_one, 5);

        println!("{}", ans);
    }
```

- Unlike closures, the `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.
- Function pointers implement all three of the closure traits - (Fn, FnOnce, FnMut)
- So we can always pass a function pointer as an argument for a function that expects a closure.
- An example of where we would want to only accept functions and not closures is when interfering with external code that doesn't have closure - C functions can accept functions as arguments but C doesn't have closures.

```rs
    let list_of_num = vec![1, 2, 3];
    let list_of_strings = list_of_num.iter().map(|i| i.to_string()).collect();
    let list_of_strings2 = list_of_num.iter().map(ToString::string).collect();
```

#### Returning Closures

- Closures are represented by traits which means you cannot return closures directly

```rs
fn returns_closure () -> Fn(i32) -> i32 { // wont compile
    |x| x * 2
}
```

- To make it work we need to wrap the closure in a pointer

```rs
fn returns_closure () -> Box<Fn(i32) -> i32> { // wont compile
    Box::new(|x| x * 2)
}
```
