# 18) Pattern and Matching

- Patterns are special syntax in Rust for matching against the structure of types both complex and simple

## All the Places Patterns can be used

### Match Arms

- We use patterns in the arms of match expressions

```rs
    match value {
        pattern => expression,
    }
```

### Conditional if let expression

- if let is a shorter way to write the equivalent expression that only matches one case.
- Optionally, if let can also have else containing code to run if the pattern in the if let doesn't match.
- We can also mix and match `if let`, `else if` and `else if let` expression

```rs
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("Your favorite color is {color}")
    }else if is_tuesday {
        println!("its green Tuesday")
    }else if let Ok(age) = age {
        if age > 30 {

        }else {

        }
    }else {
        ..
    }
```

### While let Conditional Loop

- The `while let` conditional loop allows a while loop to run for as long as a pattern continues to match.

```rs
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
```

### for Loops

- `for x in y` - the pattern in this case is x

### let Statements

```rs
    let x = 5;
    let pattern = expression;
```

- in the `let x = 5;` example, x is the pattern which means "bind what matches here top the variable x"

### Function Parameters

```rs
    fn foo(x:i32) {

    }
```

- x parameter is a pattern.
- We can also use patterns in closure parameter list in the same way as in function parameter list, because closures are similar to functions.

- Patterns don't work the same in every place we use them. In some places, the patterns must be irrefutable, in other circumstances, they can be refutable.

### Refutability: Whether a Pattern might fail to match

- Patterns comes in 2 forms - **refutable** and **irrefutable**.
- Patterns that will match for any possible value passed are irrefutable. eg - `let x = 4;` because x matches anything and therefore cannot fail to match.
- Patterns that can fail to match for some reason are refutable. eg - `Some(x)` in the expression `if let Some(x) = a_value {}` because if the value of a_variable is `None` the pattern will not match.

### Pattern Syntax

#### Matching Literals

```rs
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("others"),
    }
```

#### Matching named variables

- There is a complication when we use named variables in match expressions because match starts a new scope and variables declared as part of a pattern inside the match expression will shadow those with same name outside the match construct.
- To create a match expression that compares the values of the outer variable rather than introducing a shadowed variable, we need to use match guard conditionals instead.

#### Multiple Patterns

- In match expressions we can match multiple patterns using the **Pipe Syntax(|)** which means "OR".

```rs
    let x = 1;

    match x {
        1 | 2 | 3 => println!("first three"),
        4 => ..,
        _ => ..
    }
```

#### Matching Ranges of values with the `...` Syntax

- The `...` syntax allows us to match to an inclusive range of values.

```rs
    let x = 5;
    match x {
        1...5 => ..,
        _ => ..,
    }
```

- **NB: ranges are only allowed with numeric values or char values**

### Destructuring to Break Apart Values

- we can use patterns to destructure structs, enums, tuples and references.

#### Destructuring structs

```rs
    struct Point {
        x: i32,
        y: i32,
    }

    fn main () {
        let point = Point {x: 10, y: -4};

        let Point {x: a, y: a} = point;
        let Point {x, y} = point

        match point {
            Point {x, y: 0} => //matches if y = 0,
            Point {x: 10, y} => //Matches if x = 10,
            Point {x, y} => //matches all the Point struct
        }
    }
```

#### Destructuring Enums

```rs
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main () {
        let msg = Message::ChangeColor(0, 160, 255)

        match msg {
            Message::Quit => //do sth,
            Message::Move{x, y} => //do sth with x and y,
            Message::Write(text) => //do sth with text,
            Message::ChangeColor(r, g, b) => //do sth with r, g, b variables
        }
    }
```

#### Destructuring References

- When the value we're matching to our pattern contains a reference we need to destructure the reference from the value by specifying a `&` in the pattern.
- Doing so lets us get a variable holding the value that the reference points to rather than a variable that holds the reference.

```rs
    let points = vec![Point {x: 10, y: 4}, Point {x: 8, y: 19}];

    let sum_of_squares = points.iter().map(|&Point {x, y}| x*x + y*y).sum();
```

- If we had not included the `&` in the `&Point {x, y}`, we'd have gotten a type mismatch error.

### Destructuring structs and tuples

- We can mix match and nest destructuring patterns in even more complex ways.

```rs
    let ((feet, inches), Point{x, y}) = ((5, 10), Point {x: 12, y: 8});
```

- This code lets us break complex types into their component parts so we can use the values we're interested in separately.

### Ignoring Values in a Pattern

- There are a few ways to ignore entire values or parts of values in a pattern - using the `_` pattern and using the `_` pattern within another pattern.

#### Ignoring an Entire value with `_`

```rs
    fn foo(_: i32, y: i32) {
        println!("{y}");
    }
```

#### Ignoring Parts of a value with a nested `_`

```rs
    let mut set_value = Some(5);
    let new_set_value = Some(8);

    match (set_value, new_set_value) {
        (Some(_), Some(_)) => //do sth,
        _ => {
            set_value = new_set_value
        }
    }
```

- We can also use underscores in multiple places withing one pattern to ignore particular values.

```rs
    let nums = (2, 3, 4, 5,7);

    match nums {
        (first, _, third, _, fifth) => //do sth with first, third and fifth
    }
```

#### Ignoring a unused variable by starting its name with `_`

- If we create a variable but don't use it anywhere, Rust will usually issue a warning because that could be a bug.
- Sometimes its useful to create a variable you wont use yet, in this case we can tell Rust no to warn us about the unused variable by starting the name of the variable with `_`.

```rs
    let _x = 5;
    let y = 10;
```

**NB - There is a subtle difference between using just `_` and using a name that starts with underscore - The syntax -x still binds the value to a variable, where as `_` doesn't bind at all.**

```rs
    let s = Some(String::from("Hello"));

    if let Some(_msg) = s {
        //
    }
    println!("{s}");
```

- This code wont compile because s value will still be moved into \_s which prevents us from using the variable s again.
- However, using the underscore alone doesn't ever bind to a value

#### Ignoring remaining parts of a value with `..`

```rs
    struct Point {x: i32, y: i32, z: i32};

    let origin = Point {x: 10, y: 0, z: 8};

    match origin {
        Point {x, ..} => println!("{x}")
    }
```

- The syntax `..` will expand to as many values as it needs to. for eg

```rs
    let num = (2, 4, 5, 6, 7, 8);

    match num {
        (first,.., last) => println!("{first} {last}")
    }
```

- The `..` will match and ignore everything in the middle.
- However, using `..` must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error. for eg

```rs
    match num {
        (.., second,..) => //do sth
    }
```

- Using `..` in two places like this in ambiguous.

### Creating References in Patterns with `ref` and `ref mut`

- Lets look at using `ref` to make references so ownership of the values isn't moved to variables in the pattern.

```rs
    let my_name = Some(String::from("Joshua"));

    match my_name {
        Some(name) => //do sth,
        None => //do sth
    }

    println!("{my_name}");
```

- Because ownership of my_name has been moved to name, we can no longer use my_name variable in the println! after the match because my_name no longer has ownership.
- To fix this code, we want to make the `Some(name)` pattern borrow that part of my_name rather than taking ownership.
- We cannot use `&` to create references in a pattern. Instead we use the `ref` keyword

```rs
    match my_name {
        Some(ref name) => //do sth with name,
        None => //do nth i guess
    }
    println!("{my_name}")
```

- To create a mutable reference, so we can mutate the value match in a pattern we use the `ref mut` instead of `&mut`

```rs
    let mut my_name = Some(String::from("Joshua"));

    match my_name {
        Some(ref mut name) => *name = String::from("Sally Nwamama"),
        None => //do sth
    }

    println!("{my_name}"); //prints "Sally Nwamama"
```

### Extra Conditionals with Match Guards

- A match guard is an additional if condition specified after the pattern in a match arm that must also match along with the pattern matching for that arm to be chosen.

```rs
    let num = Some(9);

    match num {
        Some(x) if x < 7 => //do sth,
        Some(x) => //do sth,
        None => // do nth i guess-
    }
```

- We can also use the OR operator `|` in a match guard to specify multiple patterns: the match guard condition will apply to all the patterns.

```rs
    let x = 4;
    let y = false;

    match x {
        4 | 7 | 9 if y => //do sth,
        _ => //do another thing
    }
```

- The precedence of a match guard in relation to a pattern behaves like this `(4 | 7 | 9) if y => ..` rather than `4 | 7 | (9 if y) => ..`

### @ Bindings

- The `@` operator lets us create a variable that holds a value at the same time we're testing that value to see whether it matches a pattern.

```rs
    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello {id: 5};

    match msg {
        Message::Hello{id: id_variable@ 3...7} => println!("{id_variable}"),
        Message::Hello(id: 10...12) => println!("another range"),
        Message::Hello{id} => //do sth with id
    }
```

- By Specifying `id_variable@` before `3...7`, we're capturing whatever value matched the range while also testing that the value matched the range pattern.
- In the second arm, we only specified range in the pattern, the code associated with the arm doesn't have a variable that contains the actual value of the id field. The pattern code isn't able to use the value from the id field because we haven't saved the id value in a variable.
- Using @ lets us test a value and save it in a variable withing one pattern.
