# Common Programming Concept

Overview: Specifically in this chapter we'll learn about variables, basic types, functions, comments and control flow



### Variables and Mutability
- variables by default in Rust are **immutable**. However we still have the option to make a variable mutable.
- To make a variable mutable, we simply add the `mut` keyword before the variable name.
```rs
    let y = 20; //immutable
    let mut x = 12; //mutable

    x = 4;
```

### Differences between Variables and Constants
- Like immutable variable, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variable.
- Firstly, we are not allowed to use `mut` with constants. Constants are always immutable.
- to declare a constant we use the `const` keyword and the type of the value **must** be annotated
- Constants can be declared in any scope, including global scope, this makes them useful for values that many parts of our code need to know about.
- Lastly, constant may be set only to a constant expression and not to the result of a function or any other value computed at runtime.
```rs
    const MAX_POINTS: u32 = 100_000;
```

- Rust naming conventions for constants is to use uppercase with underscore between words.
- Constants are valid for the entire time a  program runs, within the scope they are declared in.

### Shadowing
- You can declare a new variable with the same name as the previous variable and the new variable is said to shadow the previous variable.
- We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword.

```rs
    let x = 5;
    let x = x * 2; //shadows the previous variable
```
- **A significant difference between `mut` and shadowing is that because we are effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.**

### Data Types
- Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with the data.
- We look at two data type subsets: Scalar and Compound
- *NB*: Rust is a statically typed language, which means that it must know the types of all the variables at compile time. the compiler can usually infer what type we want to use based on the value and how we use it. **In the case where multiple types are possible, we must add a type annotation**

1) #### Scalar Types
- A scalar type represents a single value. Rust has four(4) primary scalar types - **integers, floating-point numbers, Booleans and Characters**
    * **Integer Types**
        - unsigned - u8, u16, u32, u64
        - signed - i8, i16, i32, i64
        - architecture - isize, usize
        * the `isize` and `usize` depends on the kind of computer your program is running on. 64 bit if you are on a 64 bit architecture and 32 bit if you are on a 32 bit architecture.
        * i32 is generally the fastest, even on 64 bit systems and this is Rust's default for integer.
        **NB** The primary situation in which we'd use the usize or isize is when indexing some sort of collection.

    * **Floating Point Types**
        - this are numbers with decimal points - `f32` and `f64`
        - `f32` type is single precision float
        - `f64` type is double precision float

    * **Boolean Types**
        - `true` or `false`

    * **Character Types**
        - This is Rust's must primitive alphabetic type
        `let c = 'A'`

2) #### Compound Types
- These types can group multiple values into one type. Rust has two primitive compound types - `tuples` and `arrays`

    * **Tuples**
        - This is a way of grouping together some number of other values with a variety of types into one compound type.
        ```rs
            let tup: (u32, f32) = (65, 4.2);

            let (x, y) = tup; //destructing a tuple

            let first = tup.0; //accessing based on index
        ```

    * **Arrays**
        - unlike tuples, every elements of an array must have the same type.
        - Arrays in Rust have a **fixed** length, once declared, they cannot grow or shrink in size;
        `let arr = [1, 2, 3]; //array definition`
        - Arrays are useful when we want our data allocated on the stack rather than the heap or when we wan to ensure we always have a fixed number of elements.
        - Arrays isnt as flexible as a `vector` type. - 
        - A vector is a similar collection type provided by the `std` library that is allowed to grow or shrink in size.
        - use array when you are sure the size doesnt need to change. eg. list of all month of the year
        * If we try to access an invalid array element we get an **index our of bound** runtime error.


### Functions
- Rust codes uses snake case as the convention style for functions and variable names
- we can define functions anywhere is our Rust code
* In function signatures, we must declare the type of each parameter

#### Statements and Expressions in Function bodies
- Statement are instructions that perform some actions and do not return a value
- Expressions evaluate to a resulting value
```rs
    let y = 8; // this is a statement
    x + 1 // this is an expression
```
- An Expression can be part of a statement
- *calling a function is an expression*
- *Expressions do not include a semicolon: if you add a semicolon it becomes a statement.
```rs
    let y = {
        let x = 3; //this is a statment
        x + 1 // this is an expression - it assigns the value 4 to the  variable y
    }
```

#### Function with  return values
```rs
    fn func(a: i32, b: i32) -> i32 {
        a + b //notice there is no semicolon after the expression
    }
```

### Control Flow
- `if` expression allows us to branch our code depending on a condition
- the condition in the expression must evaluate to a `bool`, if not we get an error

```rs
    let num = 5;

    if num > 5 {
        //do sth
    }else if num == 5 {
        //do sth
    }else {
        //do sth
    }
```

**Using `if` in a let expression**
- because if is an expression we can use it on the right side of the let statement
```rs
    let condition = true;

    let num = if condition {
        1 //num is set to the value 1
    } else {
        0 //num is set to the value 0
    }
```

#### Repetitions with `loop`
```rs
    loop {
        //to break out of the loop we use the break keyword
    }
```
**Conditional loop with `while`**
```rs
    let mut num = 5;
    
    while num != 0 {
        println!("{}", num);
        num = num - 1;
    }
```

**Looping through a collection with `for`**
- The while loop is slow and error prone
- for a more concise alternative we use the `for` loop

```rs
    let arr = [1, 2, 3];

    for num in arr {
        println!("{}", num);
    }
```