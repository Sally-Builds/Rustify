# 15) Smart Pointers

- A pointer is a general concept for a variable that contains an address in memory. This address "points at" some other data.
- Smart Pointers are data structures that not only act like a pointer but also have additional metadata and capabilities.
- In Rust, the different smart pointers defined in the standard library provides functionality beyond that provided by references.
- An additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast in many cases smart pointers own the data the point to.

- `String` and `Vec<T>` are examples of smart pointers because they own some memory and allow you to manipulate it. they also have metadata.

- smart pointers are usually implemented using struct. the major difference with them and regular structs is that smart pointers struct implements the `Deref` and `Drop` traits.

- Deref traits allows the instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.

- Drop trait allows us to customize the code that is run when an instance of a smart pointer goes out of scope.

- The smart pointers in the standard library we will cover in this chapter includes

1. Box<T> for allocating values on the heap. **Allows immutable and mutable borrows checked at compile time**.
2. RC<T> a reference counting type that enables multiple ownership.
3. Ref<T> & RefMut<T> accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time.

## Using `Box<T>` to point to data on the heap.

- Box is the most straight forward smart pointer.
- Boxes allows us to store data on the heap rather than on the stack.
- What will remain on the stack is a pointer to the heap data.
- Boxes does not have performance overhead.

* Here are the situations we use them most of the time

1. When we have a type whose size cant be known at compile time and we want to use a value of that type in a context that requires an exact size.
2. When we have a large amount of data and you want to transfer ownership but ensure the data wont be copied when you do so.
3. When you want to own a value and you care only that its a type that implements a particular trait rather than being of a specific type.

## Using `Box<T>` to store data on the heap

```rs
    let b = Box::new(4);

    println!("b = {}", b);
```

- Just like any owned value, when a box goes out of scope, it will be deallocated.
- Putting a single value on a heap isnt very useful, so we wont use boxes by themselves in this way.

## Enabling Recursive Types with Boxes.

- At compile time, Rust needs to know how much space a type takes up.
- One type whose size cant be known at compile time is a recursive type where a value can have as part of itself another value of the same type.
- Lets explore the **Cons List**, which is a data type common in functional programming languages as an example of recursive type.

- A cons list is a data structure that comes from the lisp programming language and its dialect.

```rs
    enum List {
        Cons(i32, List),
        Nil
    }
```

- Each item in a cons list contains 2 elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item.
- A cons list is produced by recursively calling a cons function. The canonical name to denote the base case of the recursion is Nil.

```rs
    let list = Cons(1, Cons(2, Cons(3, Nil))); //wont compile
```

- when we try to compile the code we will get an error, with the message.

```rs
recursive type has infinite size
insert indirection(eg, a `Box`, `RC`, or `&`) at some point to make List representable
```

## Using Box<T> to get a recursive type with known size.

- Rust cannot figure out how much space to allocate for recursively defined types so the compiler give the error above.
- In the error message, `indirection` means that instead of storing the value directly.We will change the data structure to store the value indirectly by storing the pointer to the value instead.
- Because Box<T> is a pointer, Rust always knows how much space a Box<T> needs. A pointers size doesnt change based on the amount of data it is pointing to.
- This means we can put Box<T> inside a Cons variant instead of another list value directly. The Box<T> will point to the next list value that will be on the heap rather than inside the cons variant.

```rs
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))))
```

## Using smart pointers like regular references with the Deref Trait

- Implementing the Deref trait allows us to customize the behavior of the dereference operator(\*).
- By Implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on reference and then use the code with smart pointer too.

## Following the Pointer to the Value with Dereference Operator

- A regular reference is a type of pointer, and on way to think of a pointer is an arrow to a value stored somewhere else.

```rs
    let x = 5;
    let y = &x;

    assert_eq!(x, *y);
```

- If we want to use the value in y we have to use the \* operator to follow the reference to the value it is pointing to.

## Using Box<T> like a reference.

```rs
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, *y);
```

## Defining our own Smart Pointer.

- We'll build our custom Box<T> pointer like the one in the standard library so we can experience why smart pointers behave differently than references by default.

```rs
    struct MyBox<T> (T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox(x);

    assert_eq!(x, *y) //wont compile
```

- The MyBox type is a tuple struct with one element of type T.
- When we try to compile the code, we get a compilation error saying `type MyBox{integer}` cannot be dereferenced.
- This is because we have not yet implemented the `Deref` trait.

## Treating a type like a Reference by Implementing a Deref Trait

- The Deref trait by the standard library requires us to implement one method named `deref` that borrows self and returns a reference to the inner data.

```rs
    impl<T> Deref For MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0 //because MyBox is a tuple struct
        }
    }
```

- the type `Target = T` syntax defines an associated type for the Deref trait to use. Associated types are slightly different way of declaring a generic parameter.
- Without the Deref trait, the compiler can dereference only & references, behind the scene this is what it truly is `*(y.deref())`.

## Implicit Deref Coercions with functions and Methods

- **Deref Coercion** is a convenience that Rust performs on arguments to functions and methods.
- Deref coercion converts **a reference to a type that implements Deref** into **a type that Deref can convert the original type into**

```rs
    fn hello(name: &str) {
        println!("{}", name);
    }

    hello("Rust");
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
```

- Rust can turn `&MyBox<String>` into `&String` by calling deref.
- Rust calls deref again to turn `&String` into `&str` which matches the hello function definition.
- If Rust didnt implement DC, here is what our code will look like when we try to call hello function - `hello(&(*m)[..])`
- the `(*m)` dereferences the MyBox to a String, the `&` and `[..]` takes a string slice of the String that is equal to the whole String to match the signature of hello.

## How Deref Coercion interacts with Mutability

here are the 3 cases of deref coercion.

1. Immutable to Immutable - `&T to &U when T: Deref<Target = U>`.
2. Mutable to Mutable - `&mut T to &mut U when T: DerefMut<Target = U>`
3. Mutable to Immutable - `&mut T to &U when T: Deref<Target = U>`

## Running Code on Cleanup with the Drop Trait

- The second trait important to the smart pointer pattern is `Drop`, which lets us customize what happens when a value is about to go out of scope.
- The `Drop` trait requires you to implement one method named drop that takes a mutable reference to self.

```rs
    struct CustomSP {
        data: String
    }

    impl Drop for CustomSP {
        fn drop(&mut self) {
            println!("dropping value: {}", self.data);
        }
    }

    fn main () {
        let csp = CustomSP {String::new("Hello")}
    }
```

- We do no need to call the drop method explicitly, Rust does that for us when our instance went out of scope.

## Dropping a value entry with `std::mem::drop`

- We cannot call the drop method on an instance to drop early because Rust doesnt allow it and rightfully so, calling drop early would create a **double free error** because Rust will still try to drop the value after it goes out of scope.

- to drop early, we will use the `std::mem::drop` function provided by the standard library so we can force a value to be dropped before the end of its scope.

```rs
    let csp = CustomSP {data: String::new("hello")};
    drop(c);
```

- **Note - the `std::mem::drop` is different from the drop in the Drop trait**

## `RC<T>`, The Reference Counted Smart Pointer

- In majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners.
- To enable multiple ownership, Rust has a type called `RC<T>` which is an abbreviation for **Reference Counting**.
- The RC<T> type keeps track of the number of references to a value which determines whether or not a value is still in use.
- If there are zero(0) references to a value, the value can be cleared up without any reference becoming invalid.
- We use RC<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we cannot determine at compile time which part will finish using the data last.
- **Note: `RC<T>` is only for use in single threaded scenarios**

## Using `RC<T>` to share data.

- we will create two lists that shares ownership of a third list.

```rs
    let a = Cons(5, Box::new(10, Box::new(Nil)));
    let b = Cons(3, Box::new(a));
    let c = Const(8, Box::new(a));
```

- This wouldnt compile because Cons Variant own the data they hold, soo when we create the b list, a is moved into b and b now own a. then we try to use a again when creating c, we are not allowed to because a has already been moved.

- Well, we could change the definition of Cons to hold references instead but then we would have to specify lifetime parameters, but it will not work because the borrow checker wont let us compile `let a = Cons(10, &Nil)` because, the temporary Nil value would be dropped before a could take reference to it.
- We can change our definition to use `RC<T>` in place of `Box<T>`

```rs
    enum List {
        Cons(i32, RC<List>),
        Nil,
    }

    let a = RC::new(Cons(10, RC::new(Nil)));
    let b = Cons(12, RC::clone(&a));
    let c = Cons(4, RC::clone(&a));
```

- Each Cons variant now holds a value and RC<T> pointing to a List.
- When we create b, instead of taking ownership of a, we'll clone the RC<List> that a is holding, thereby increasing the number of reference from one to two and letting a and b share ownership of the data in the RC<List>.
- every time we call `RC::clone()`, the reference count to the data within the `RC<List>` is incremented and the data wont be cleared up unless there are zero(0) references to it.
- \*\*We could also have cloned using `a.clone()` method rather than `RC::clone(&a)` but Rust convention is to use the later in this case because the implementation of `RC::clone` doesnt make a deep copy of all the data as most implementation of clone do. Deep copies of data can take a lot of time.

## Cloning an `RC<T>` increases the reference count

- We can get the reference count by calling the `RC::strong_count` function. like this `RC::strong_count(&a);`.
- The function is named strong count rather than count because the RC<T> type also has a weak_count. we'll talk more on this later in the chapter.

## `RefCell<T>` and the Interior Mutability Pattern.

- **Interior Mutability** is a design pattern in Rust that allows us to mutate data even when there are immutable references to that data; normally this action isn't allowed by Rust borrowing rules.
- To mutate data, the pattern uses **unsafe** code inside a data structure to bend Rust usual rules that govern mutation and borrowing.
- We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler cannot guarantee that.

## Enforcing Borrowing Rules at Runtime with `RefCell<T>`

- Unlike `RC<T>`, the `RefCell<T>` type represents **Single Ownership** over the data it holds.
- The `RefCell<T>` is useful if you are sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
- With `RefCell<T>`, the invariants are enforced at runtime.
- With references, if you break the rules, you'll get a compiler error. With RefCell<T> if you break the rules, your program will panic and exit.
- Mutating the value inside an immutable value is the **interior mutability pattern**

## Interior Mutability: A mutable borrow to an immutable value

- When you have an immutable value, you cannot borrow it mutably.
- There are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.
- Using `RefCell<T>` is one way to get the ability to have interior mutability.

## A Use case for Interior Mutability: Mock Objects

- Mock objects are specific types of test doubles that record what happens during a test so we can assert that the correct actions took place.
- Rust doesn't have objects like other programming languages. However, we can create a struct that will serve the same purpose as a mock object.

```rs
    pub trait Messenger {
        send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: `a + Messenger> {
        messenger: &`a T,
        value: usize,
        max: usize
    }

    impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
        fn new(messenger: &Messenger, max: usize) -> LimitTracker {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        fn set_value(&mut self, value: usize) {
            let percentage = value / self.max;
            if percentage >= 0.5 && percentage < 0.75 {
                self.messenger.send("You are have used more than half of you plan");
            }
            if percentage >= 0.75 && percentage < 0.95 {
                self.messenger.send("You have used almost all you plan");
            }
            if percentage == 1 {
                self.messenger.send("You have used up all your plan");
            }

            self.value = value;
        }
    }
```

- One important part of the code is that the Messenger trait has one method called **send** that takes an immutable ref to self and the text of the message. This is the interface our mock object needs to have.
- We also want to test the behavior of our `set_value` method on the LimitTracker.
- set_value doesn't return anything for us to make assertions on.
- We want to be able to say that if we create LimitTracker with something theat implements the Messenger Trait and a particular value for max, when we pass different numbers for value, the messenger is told to send the appropriate message.

```rs
#[cfg(test)]
mod tests {
    struct MockMessenger {
            sent_messages: vec<String>
        }

        impl MockMessenger {
            fn new()  -> MockMessenger {
                MockMessenger {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            send(&self, message: &str) {
                self.sent_messages.push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();

            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.len(), 1);
        }
}

```

- This code above wont work, it gives us a compile time error. which says cannot borrow immutable field `self.sent_messages` as mutable.
- We cannot modify the MockMessenger to keep track of messages because the `send` method takes an immutable reference to self.
- We also cannot use `&mut self` because then the signature of send would'nt match the signature in the Messenger trait definition.

- This is a situation in which interior mutability can help.
- We will store the sent_messages withing a RefCell<T> and then the send message will be able to modify sent_messages to store the messages we've sent.
- here is what our MockMessenger will look like:

```rs
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();

            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }

```

- The sent_message field is now of type RefCell<Vec<String>> instead of Vec<String>.
- The send method, still has the first parameter as immutable borrow of self which matches the trait definition.
- we call the `borrow_mut` on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is a vector.
- We then call push on the mutable reference to the vector to keep track of the messages sent during the test.
- The last change we made is in the assertion, to see how many items are in the inner vector, we call the `borrow` on the RefCell<Vec<String>> to get an immutable reference to the vector.

## Keeping Track of Borrows at Runtime with RefCell<T>

- When creating an immutable and mutable reference, we use the `&` and `&mut` syntax respectfully.
- With RefCell<T>, we use the `borrow` and `borrow_mut` methods which are part of the safe API that belongs to RefCell<T>
- The **borrow** method returns the smart pointer type `Ref<T>`
- the **borrow_mut** method returns the smart pointer type `RefMut<T>`
- Both types implement Deref so we can treat them as regular references.
- The RefCell<T> keeps track of how many Ref & RefMut smart pointers are currently active.
- Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active, When Ref<T> value goes our of scope, the count goes down by one.
- RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

- If we try to violate these rules, our code will panic at runtime.
- Our code would also incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time.

- However, using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of messages it has sent while using it in a context where only immutable values are allowed.

## Having Multiple Owners of Mutable Data by Combining RC<T> and RefCell<T>

- RC<T> lets us have multiple owners of some data, but only gives immutable access to the data.
- If we have an RC<T> that holds a RefCell<T>, we can get a value that can have multiple owners and that we can mutate.

```rs
    enum List {
        Cons(RC<RefCell<i32>>, RC<List>),
        Nil
    }

    let value = RC::new(RefCell::new(5));
    let a = RC::new(Cons(RC::clone(&value), RC::new(Nil)));
    let b = Cons(RC::new(RefCell::new(8)), RC::clone(&a));
    let b = Cons(RC::new(RefCell::new(10)), RC::clone(&a));

    *value.borrow_mut() += 3;

    println!("a = {}", a);
    println!("b = {}", a);
    println!("c = {}", a);
```

- By using RefCell<T>, we have outwardly immutable List value, but we can use the methods on RefCell<T> that provides access to interior mutability so we can modify our data when we need to.

## Reference Cycles can Leak memory

- Rust memory safety guarantees makes it difficult but not impossible to accidentally create memory that is never cleaned up. This is know as **memory leaks**.
- As seen earlier, Rust allows memory leaks by using RC<T> and RefCell<T>
- it is possible to create a reference where items refer to each other in a cycle. This creates a memory leak because the reference count of each item in the cycle will never reach zero(0) and the values will never be dropped.

## Creating a Reference Cycle.

- Lets take a look at how reference cycle might happen and how to prevent it.

```rs
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<RC<Node>>>,
    }

    let a = RC::new(Node {value: 5, RefCell::new(None)});

    let b = RC::new(Node {value: 10, RefCell::new(RC::clone(&a))});

    *a.next.borrow_mut() = Some(RC::clone(&b));
```

- The example above creates a reference cycle because b points to a and we later change a to point to b.
- If you have RefCell<T> value that contains RC<T> values or similar nested combination of types with interior mutability and reference counting, we must be careful not to create cycles because we cannot rely on Rust to catch them.

## Preventing Reference Cycles: Turning RC<T> into Weak<T>

- We've shown that calling RC::clone increases the strong_count of an RC<T> instance and an RC<T> instance is only cleaned up if its strong_count is 0.
- we can also create **weak references** to the value within an RC<T> instance by calling **`RC::downgrade`** and passing a reference to the RC<T>
- When we call RC::downgrade, we get a smart pointer of type **`Weak<T>`**.
- Calling RC::downgrade increases the weak_count by 1 instead of increasing the strong_count.
- RC<T> uses a weak_count to keep track of how many Weak<T> references exist similar to strong_count.
- The only difference is that weak_count doesn't need to be 0 for RC<T> instance to be cleaned up.
- Strong references are how we can share ownership of an RC<T> instance.
- Weak references don't express ownership relationship.
- Weak reference wont cause reference cycle because any cycle involving some weak reference will be broken once the strong reference count of values involved is 0.
- Because the value that Weak<T> references might have been dropped, when we want to use the value, we must call **upgrade** method on the Weak<T> which returns an Option<RC<T>>

## Creating a Tree Data Structure: a Node with Child Nodes

```rs
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<RC<Node>>>,
    }

    let leaf = RC::new(Node {
        value: 2,
        children: RefCell::new(vec![])
    });

    let branch = RC::new(Node {
        value: 8,
        children: RefCell::new(vec![RC::clone(&leaf)])
    });
```

- Here we can get from the branch to the leaf through the branch but we cannot get from the leaf to the branch. This is because the leaf has no reference to branch and doesn't know they're related. We want the leaf to know that branch is its parent.

## Adding a Reference from Child to its Parent.

- To make child aware of parent, we need to add a parent field to our Node struct definition
- We know the parents type cant be RC<T> because that'll create a reference Cycle with leaf.parent point to branch and branch.children pointing to leaf which causes strong_count to never be 0.
- A parent node should own its children and if a parent node is dropped so should the children.
- However, a child should not own its parent. if a child node is dropped the parent should still exist.
- So instead of RC<T> we'll make the parent type Weak<T> specifically a `RefCell<Weak<Node>>`

```rs
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<RC<Node>>>
    }

    let leaf = RC::new(Node {
        value: 4,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    })

    let branch = RC::new(Node {
        value: 12,
        parent: RefCell::new(Weak::new()),
        children: vec![RC::clone(&leaf)],
    })

    *leaf.parent.borrow_mut = RC::downgrade(&branch);

    println!("leaf parent = {}", leaf.parent.borrow().upgrade);
```

## Visualizing changes to Strong and Weak counts

- By specifying that the relationship from a child to its parent should be a Weak<T> ref in the definition of Node, we are able to have parent node points to child nodes and vise versa without reference cycles and memory leaks
