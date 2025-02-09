# Common Collections

- Rust standard library includes a number of very useful data structures called **collections**.
- unlike arrays and tuples the data these collections points to is stored on the heap which means the amount of data does not need to be known during compile time and can grow or shrink as the program grows.
- Each of the collections has different capabilities and costs, so choosing an appropriate one for your current situation is a skill you'll develop over time.
- In this chapter, we discuss the three(3) most used collections namely:
  - Vectors
  - Strings
  - Hashmaps

### Storing values with values

- `Vec<T>` also known as vector allows us to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type

#### Creating a Vector

```rs
    let v:Vec<i32> = Vec::new(); //we must annotate the type here because the compiler cannot infer from the new method
```

- Vectors are implemented using **Generics**
- We can also create a vector by using the `vec!` macro for convenience

```rs
    let v = vec![1, 2, 3]; //here rust can infer that our vector is <i32>
```

#### Updating a vector

```rs
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
```

- as with a variable, if we wanted to be able to change its value, we add the `mut` keyword after the `let` keyword.
- we didnt annotate because the data we push in the next line helps Rust compiler infer that the vector is of type `Vec<i32>`.

#### Dropping a vector drops its element

- like any other struct, a vector is freed when it goes out of scope
- When a vector gets dropped, so are all its contents.

#### Reading Elements of a vector

- There are two (2) ways to reference a value stored in a vector
- we can use either indexing syntax or with `get` method

```rs
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    let second: Option<i32> = v.get(1);
```

- in the first option, if we try to reference a non existent element the program will panic.
- in the second method, it is safer and will return an `Option` enum. This is a safer way to read elements of a vector.
- When a program has a valid reference the borrow checker enforces the ownership and borrowing rules to ensure that the reference and any other reference to the contents of the vector remains valid.

```rs
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(8);
```

- This code wont compile because of an ownership rule that says we cannot have a mutable and immutable reference to a value in the same scope.
- mere looking at the code, it should work because appending to the vector should not change the other elements in their positions
- With Rust, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space. This happens if there isnt enough room to put all the elements next to each other where the vector currently is.
- **In this case, the reference to the first element would be pointing to a de-allocated memory**

#### Iterating over values in a vector

- if we want to access each element in a vector, we can iterate through all of the elements rather than using indexes.

```rs
    let v = vec![1, 2, 3, 4, 5];
    for num in &v {
        println!("{}", num);
    }
```

- we can also iterate over mutable references to each elements in a mutable vector in order to make changes to all the elements.

```rs
    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 50;
    }
```

we use the dereference operator(\*) to get the values in `i`.

#### using an Enum to store multiple types

- we said earlier that vectors can only store values that are the same type. This can be inconvenient; there are use cases for needing to store a list of items of different types.
- we can do this using enums, since the type of the variant of an enum is the enum itself, we can have enum variants hold different value types.

```rs
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![SpreadSheet::Int(3), SpreadSheet::Text(String::from("Hellow"))];
```

- Here are some of the reasons why a vector must have elements of the same type.
  - Rust needs to know what type will be in a vector at compile time so it knows exactly, how much memory on the heap will be needed to store each element.
- If Rust allowed for a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. So us using an enum plus match expression means that Rust will ensure at compile time that every possible case is handled.
- **Vector type also has lots of method associated with it, will see docs to know them**

### Storing Utf-8 Encoded Text with Strings

- We will discuss Strings in the context of collections because **Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text**.

#### What is a String

- Firstly, lets defined what we mean by the term string. Rust has one string type in the core language which is the string slice `str` usually seen in its borrowed form `&str`. String slices are references to some utf-8 encoded string data stored elsewhere.
- **String type is provided by the Rust Standard Library rather than the coded into the core language, it is a growable, mutable, owned, utf-8 encoded string type**
- when we use the term "strings" in rust we mean both the `String` and `&str` string slices type.

#### Creating a New String

```rs
    let mut s = String::new();
```

- This creates a new empty string called `s`, which we can load data into
- Also most of the vector methods are also defined in the String type.

- if we had some initial data we wanted to start the string with, we can use the `to_string` method which is available on any type that implements the `Display` trait, as string literals do.
- we can also use the `String::from()` to create a string from a string literal.

```rs
    let initial_data = "Hello World";
    let s = initial_data.to_string();
    let s2 = "Hello World again".to_string();
    let s3 = String::from("Last example");
```

- Also remember that Strings are utf-8 encoded, so we can include any properly encoded data in them.

#### Updating a String

- A string can grow in size and its content can change, if you push more data into it.
- we can also use the `+` operator or the `format!` macro to concatenate String values

##### Appending to a String with `push_str` and `push`

```rs
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // the push_str does not take ownership of the s2
    println!("s1 = {}, s2 = {}", s1, s2);

    let mut s3 = String::from("Hell");
    s3.push('o'); //the push takes a single character as a parameter and adds it to the string
```

##### Concatenating with the `+` operator or the `format!` macro.

```rs
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; //s1 has been moved here and can longer be used
```

- The reason `s1` is no longer valid and the reason we used a reference to `s2` has to do with the signature of the method that gets called when we use the `+` operator.
- The `+` operator uses the add method, whose signature looks like this

```rs
    fn add(self, s:&str) -> String
```

- so this means we can only add a `&str` to a string, we cannot add two String values together.
- Hold on o!, but the type of `&s2` in the example is `&String` and not `&str`, as specified in the `add` parameter. So why does our code compile?
- The reason we are able to use `&s2` in the call to add is that the compiler can **Coerce** the `&String` argument to `&str`.
- When we call the `add` method, Rust uses a **deref coercion** which turns `&s2` into `&s2[..]`

```rs
    let s1 = String::from("Tic");
    let s1 = String::from("Tac");
    let s1 = String::from("Toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    let sFormat = format!("{}-{}-{}", s1, s2, s3);
```

- for a more complicated String combination, we use the `format!` macro.
- the `format!` macro works in the same way as the `println!` macro, but instead of printing its output, it returns a String with the contents.
- The `format!` macro is much easier to read and doesnt take ownership of any of its parameters.

##### Indexing into Strings

- if we tried to access parts of a String using the indexing syntax in Rust, you'll get an error.
- Rust Strings doesnt support indexing. To understand why, lets discuss how Rust stores strings in memory.

##### Internal Representation

- A String is a wrapper over a `Vec<u8>`
- Rust uses UTF-8 encoding which means
  - some characters take 1 byte(like English letters: A, B, C)
  - Some characters take 2, 3, or even 4 bytes(like emojis or certain non-latin letters) so when we try to access using index eg s[0] to get a character the character might be 2 bytes which is the 0 and 1 index. This is the reason why indexing doesnt work in Rust.

##### Bytes, Scalar Values and Grapheme Clusters! Omo!

- Rust treats strings in three(3) different ways, depending on how you want to interpret the text
  - Bytes(u8 value) - raw storage form
  - Scalar(char value) - individual characters but some symbols need multiple characters
  - Grapheme(Actual Letters) What humans perceive as characters (letters, symbols, etc.).

##### Slicing Strings

- Indexing into a string is often a bad idea because its not clear what type the return type of the string indexing operation should be: byte, char, grapheme or string slice.
- to be more specific in our indexing and indicate that we want a string slice rather than indexing using [] with a single number, we use [] with a range to create a string slice containing particular bytes

##### Methods of Iterating over Strings

- if you need to perform operations on individual unicode scalar values, the best way to do so is to use `chars` method

```rs
    for c in "Hello".chars() {
        //print c
    }
```

- The `bytes` method returns each raw byte, which might be appropriate for your usecase.

```rs
    for b in "Hello".bytes() {
        //print b
    }
```

### Storing Keys with Associated Values in Hashmaps

- The type `HashMap<k, v>` stores a mapping of keys of type `k` to value type `v`. it does this via a hashing function which determines how it places these keys and values into memory.
- Hashmaps are useful when you want to look up data not by using an index as you can with vectors, but by using a key that can be of any type.

##### Creating a new HashMap

- we can create an empty HashMap with `new` and add elements with `insert` method.

```rs
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::("blue"), 10);
    scores.insert(String::("red"), 30);
```

- we need to first bring in HashMap from the collections portion of the standard library.
- of the three common collections, this is the least used so its not brought automatically into the prelude.
- Another way of constructing a hashmap is by using the `collect` method on a vector of tuple, where each tuple consists of a key and a value.
- the `collect` method gathers data into a number of collection types, including HashMap.
- for example if we had the team names and initial scores in two separate vectors we could use `zip` method to create a vector of tuples from both vectors.

```rs
    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 30];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
```

- The type annotation HashMap<_,_> is needed here because its possible to collect into many different data structures and Rust doesnt know which you want unless you specify
- For the parameters for the key and value types, we use underscore because Rust can infer types that the hashmap contains based on the types of data in the vectors.

#### HashMaps and Ownership

- for types that implement the `Copy` trait, such as `i32`, the values are copied into the hashmap.
- for owned values such as `String`, the values will be moved and the hashmap will be the owner of those values.

```rs
    let field_key = String::from("name");
    let field_value = String::from("Joshua Uzoagulu");

    let mut users = HashMap::new();
    users.insert(field_key, field_value);

```

- we wont be able to use the field_key and field_value after they have been moved into the hashmap with the call to `insert`.
- if we insert references to values into the hashmap, the values wont be moved to the hashmap. However, the values must be valid for at least as long as the hashmap is valid.

#### Accessing Values in Hashmap

- we access a value out of a hashmap by providing its key to `get` method.
- the result of the `get` method is on `Option<&v>` enum.
- we can also iterate over each key/value pair in a hashmap in a similar manner as we do with vectors, using `for` loop.

```rs
    for (key, value) in &scores {
        // print key and value
    }
```

- This code will print each pair in an arbitrary order.

#### Updating a HashMap

- each key can only have one value associated with it at a time.
- when we want to change the data in a hashmap, we have to declare how to handle the case when a key already has a value assigned to it.
  - we could replace the old value with a new value
  - we could keep the old value and ignore the new value, only adding new value if the key doesnt already have a value.
  - we could combine the old value and the new value.

##### Overwriting a value

```rs
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 80);
```

##### Only Inserting a value if a key has no value

- Hashmaps have a special API called `entry` that takes the key you want to check as a parameter. The return value of the entry method is an enum called `Entry` that represents a value that might or might not exist.

```rs
    scores.entry(String::from("red")).or_insert(90);
```

- **the `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding Entry key if that key exist, and if not inserts the parameter as the new value for this key and returns a mutable reference to the new value**

##### Updating a value based on the old value

```rs
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = map.entry(word).insert(0);

        *count += 1;
    }
```

- The `or_insert` method returns a mutable reference(&mut v) to the value for this key.
- in order to assign to that value we must first dereference using the (\*)

### Hashing Functions

- By default, HashMap uses a cryptographically secure hashing function that can provide resistance to **Denial of Service(DOS)** attacks.
- You can decide to use another hashing function if you dont like the default for some reason by specifying a different **`hasher`**.
- A hasher is a type that implements the **`BuilderHasher`** trait.
- You dont necessarily have to implement your own hasher from scratch _crates.io_ has libraries shared by other Rust users that provide hashers implementing many comming hashing algorithms.
