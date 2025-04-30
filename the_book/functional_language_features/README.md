# Functional Language Features: Iterators & Closures

- Programming in functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution and so on.
- This chapter specifically covers **closures** - A function-like construct you can store in a variable; **iterators** - A way of processing a series of elements.

## Closures: Anonymous Functions that can cover their environments

- Rusts closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- Unlike functions, closures can capture values from the scope in which they're called.

### Creating an abstraction of Behavior with closures.

- We want to define code in one place in our program but only execute that code where we actually need the result. This is the use of closures!
- To define a closures, we start with a pair of vertical pipes(|), inside which they specify the parameters to the closures.
- After the parameters, we place a curly brackets that holds the body of the closure - these are optional if the body of the closure is a single expression.
- The end of a closure after the curly brackets needs a semicolon.

```rs
    let expensive_closure = |num| {
        println!("This is a closure");
        thread::sleep(Duration::from_sec(2));
        num * 2
    };
```

- **NB - The `let` statement means expensive_closure contains the definition of an anonymous function, and not the resulting value of calling the anonymous function.**

### Closure Type inference and annotation.

- Closures dont require us to annotate the types of the parameters or the return value like `fn` function do.
- As with variables we can add annotations if we want.

```rs
    let age = |num: u32| -> u32 {num * 4};
```

### Storing Closures Using Generic Parameters and the `fn` Traits

- We can create a struct that will hold a closure and the resulting value of calling the closure.
- The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of the code doesnt have to be responsible for saving and reusing the result. This pattern is known as **`Memoization`** or **`Lazy evaluation`**.
- To make a struct that holds a closure, we need to specify the type of the closure because a struct definition needs to know the types of each of its field.
- Each closure instance has its own unique anonymous type i.e even if two closures have the same signatures their types are still considered different.
- To define structs or function parameters that use closures, we use generics and trait bounds.
- All Closures implement at least one of these traits - `Fn`, `FnMut`, `FnOnce`. These traits are provided by the standard library.
- We add types to the `Fn` trait bounds to represent the types of the parameters and return values of the closures to match a trait bound.

```rs
    struct Cacher<T> where T: Fn(u32) -> u32 {
        Calculation: T,
        value: Option(u32)
    };
```

- The Cacher has a calculation field of the generic type `T`. The trait bound on `T` specifies that it is a closure by using the `Fn` trait.
- **NB - Function implement all three of the Fn traits**

```rs
    impl<T> Cacher<T>
        where T: Fn(u32) -> u32 {
            fn new(Calculation: T) -> Cacher<T> {
                Cacher {
                    Calculation,
                    None
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) -> v,
                    None -> {
                        let v = (self.Calculation)(arg);
                        self.value = v;
                        v
                    }
                }
            }
        }

        let mut expensive_result = Cacher::new(|v| => {
            v * 2
        })

        expensive_result.value(18);
```

- We want Cacher to handle the struct fields values rather than letting the calling code potentially change the values in these fields directly, so these fields are private.
- When the calling code needs the result of evaluating the closure, instead of calling the closure directly, it will call the value method. The method checks whether we already have a resulting value in self.value in a Some; if we do, it returns the value within the Some without having to run the closure again.

### Limitations of the Cacher Implementation

- our cacher struct has two limitations

1. The first limitation is that a Cacher instance assumes it will always get the same value for the parameter arg to the value method.

```rs
    let mut c = Cacher::new(|x| => x);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2) // this test will fail
```

- the problem is that the first time we called c.value with 1, the cacher instance saved Some(1) in self.value. So no matter what we pass into the value method, it will always be 1.

#### Solution

- Try modifying cacher to hold a hashmap rather than a single value.
- The keys of the hashmap will be the arg values that are passed in
- The value of the hashmap will be the result of calling the closure on that key.
- Instead of looking at whether self.value directly has a Some or None value, the value function will look up the arg in the hashmap and return the value if present. if it is not present, the Cacher will call the closure and save the resulting value in the hashmap associated with its arg value.

2. The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32.

#### Solution

- Try introducing more generic parameters to increase the flexibility of the Cacher Functionality.

## Capturing the Environment with Closures

- closures have a capability that functions do not have: they can capture their environment and access variables from the scope in which they are defined.

```rs
    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));
```

- when a closure captures a value from its environment it uses memory to store the value for use in the closure body.
- This use of memory causes an overhead that we dont want to pay in most common cases.
- Closures can capture values from their environment in 3 ways, which directly maps to three ways a function can take a parameter.

1. taking ownership
2. borrowing mutably
3. borrowing immutably

- These are encoded in the three `Fn` traits

1. FnOnce - consumes the variable it captures from its enclosing environment
2. FnMut - can change the environment because it mutably borrows values
3. Fn - borrows values from the environment immutably

- When we create a closure, Rust infers which trait to use based on how the closures uses the values from the env.
- All closures implement the FnOnce trait because they can all be called at least once.

- **if we want to force the closure to take ownership of the values it uses in the env we can use the `move` keyword before the parameter list**

- This technique is mostly used when passing a closure to a new thread to move the data so it is owned by the new thread.

```rs
    let s = String::new("Hello");
    let is_empty = move || -> s.is_empty()

    println!("{}", s) //this code will not compile
```

### Processing a series of items with Iterators

- The iterator pattern allows you to perform some task on a sequence of items in turn.
- An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
- In Rust, iterators are lazy, meaning they have no effect until you call methods that consumes the iterator to use it up.

- The example below creates an iterator over the items in the vector v1 by calling the `method` defined on vec<T>

```rs
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for v in v1_iter {
        //
    }
```

- when the for loop is called using the iterator in v1_iter each element in the iterator is used in one iteration of the loop, which prints each value

#### The Iterator Trait and the next method

- All iterators implement a trait named **Iterator** that is defined in the std library

```rs
    pub trait Iterator {
        type Item,

        fn next(&mut self) -> Option<Self::Item>,
        //other methods
    }
```

- There is a new definition we come across in this trait called `Item` and `Self::Item` which are defining an associated type with this trait.
- The code above says that to implement the Iterator trait requires we define the Item type and this Item type is used in the return type of the `next` method.
- The Iterator traits only requires us to implement the `next` method which returns one item of the iterator at a time wrapped in an Option Enum.

```rs
    let v1 = vec![1, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
```

- we needed to make `v1_iter` mutable because calling the next method on an iterator changes the internal state that the Iterator uses to keep track of the sequence. In other words, the code consumes or uses up the Iterator.
- We didnt need to make the v1_iter mutable when we used the for loop because the loop took ownership of the variable and made it mutable behind the scenes.
- The values we get from the call to next method are immutable references to the values in the vector.

1. `iter` method produces an iterator over immutable references.
2. `into_iter` method produces an iterator that takes ownership over `v1` and returns owned values.
3. `iter_mut` produces an iterator over mutable references.

#### Methods That consumes the Iterator

- Some of the Iterator trait default implementation call the `next` method which is why it is required to implement it.
- Methods that call next are called `Consuming Adaptors` because calling them uses up the iterator.
- On of such methods is the `sum` method, it takes ownership of the iterator and iterates through the items by repeatedly calling next.

```rs
    let sum: i32 = v1_iter.sum();

    assert_eq!(sum, 4);
```

- After the call to `sum`, we cant use v1_iter anymore because it has been dropped by `sum` after it took ownership of the iterator.

#### Methods that produce other Iterators

- Other methods defined on the Iterator traits, know as `Iterator Adaptors` allow you to change iterators into different kinds of Iterators.
- We can chain multiple calls to Iterator adaptors to perform complex actions in a readable way.
- Because all iterators are lazy, we have to call one of the consuming adaptor methods to get results from call to iterator adaptors.

```rs
    let v1: vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| -> x * 2);
```

- We get warning from our compiler saying iterator adaptors are lazy and do nothing unless consumed.
- to get rid of this waning - we'll use the `collect` method
- as the code above, the closure we've specified never gets called so using collect consumes the iterator and collects the resulting values into a collection data type.

```rs
    let v2: vec<_> = v1.iter().map(|x| -> x * 2).collect();
```

#### Using closures that capture their environment

```rs
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe.size).collect()
    }
```

- In the shoe_in_my_size function, we use into_iter to create an iterator that takes ownership of the vector.
- We then call `filter` to adapt that iterator into a new iterator and then collect the values of the iterator into a collection.

#### Creating our own Iterators with the Iterator Trait

- we can create iterators that do anything we want by implementing the Iterator trait on our own types.
- The only method we need to implement is the `next` method.

```rs
    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            }else {
                None
            }
        }
    }
    impl Counter {
        fn new() -> Counter {
            Counter {count: 0}
        }
    }
```

- we set the associated type for our iterator to u32, meaning the iterator will return u32 values.

##### Using our counter Iterator's next method

```rs
    let mut counter = Counter::new();

    assert_eq!(counter.next(),  1);
```

##### Using other Iterator traits Methods

```rs
    let sum: u32 = Counter::new()
                    .zip(Counter::new().skip(1))
                    .map(|a, b| a * b)
                    .filter(|x| x % 3 == 0)
                    .sum();
    assert_eq!(sum, 18);
```

- all of these method calls are possible because we specified how the next method works and the std lib provides default implementation for other methods that call next.

#### Improving our I/O Project

- we can improve the new method in the Config implementation by passing in Iterator to the methods as parameter instead of borrowed String.
- So since the `env::args()` is an Iterator we pass it directly as the argument to the new method and also we have to updated the new method definition to accepts an Iterator.

```rs
    let config = Config::new(env::args());
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {}
```

##### Using Iterator trait methods instead of indexing

```rs
    ...
    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didnt get a query string");
    };
```

##### Making code cleaner with Iterator Adaptors

- we can take advantage of Iterators in the search function of our IO project.
- here is what the previous implementation looked like.

```rs
    ...
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
```

- here is how we can use Iterator adaptors. Doing so helps us minimize the amount of mutable state and makes our code a lot cleaner.

```rs
    contains.lines().filter(|line| line.contains(query)).collect();
```
