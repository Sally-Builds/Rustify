# 17) Object-Oriented Programming Features of Rust

- OOP is a way of modeling Programs

### Characteristics of Object-Oriented Languages

- There is no consensus in the programming community about what features a language must have to be considered Object oriented.
- Rust is influenced by many programming paradigm including OOP.
- OOP languages share certain common characteristics namely: **Objects**, **Encapsulation** and **Inheritance**

### Objects Contain Data and Behavior

- According to the Authors of **Gang of Four** book, OOP is defined as programs made up of Objects. An Object packages both data and the procedures that operate on that data. The procedures are typically methods or operations.
- Using that definition, Rust is Object Oriented because structs and enums have data and impl block provides methods on structs and enums.

### Encapsulation that hides Implementation Details.

- Encapsulation means that implementation details of an object aren't accessible to the code using that Object. Therefore, the only way to interact with an Object is through a Public API.

### Inheritance as a Type System and as Code Sharing.

- Inheritance is a mechanism whereby an object can inherit from another object definition.
- Rust doesn't define a way in which structs can inherit parent struct.
- One of the reasons of using inheritance is for code reuse. we can share Rust code using default trait method implementation. This is similar to a parent class having an implementation of a method and a child class also having the implementation of that method.
- Another reason for using inheritance relates to Type System to enable a child type to be used in place of the parent type. This is called **Polymorphism**.
- Rust takes a different approach using trait objects instead of inheritance.

### Using Trait Objects that Allow for Values of Different Types

### Defining a Trait for Common Behavior

```rs
    pub trait Draw {
        draw(&self);
    }

    pub struct Screen {
        pub Component: Vec<<Box<Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
```

- This works differently than a struct that uses a generic type parameter with trait bounds.
- A generic type parameter can only be substituted with one concrete type at a time where as trait object allow for multiple concrete types to fill in for the trait object at runtime.

### Implementing The Trait

```rs
    pub struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing from Button...");
        }
    }

    pub struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Drawing from SelectBox...");
        }
    }

    fn main() {
        let screen = Screen {
            components: vec![
                Box::new(Button{}),
                Box::new(SelectBox{})
            ]
        }
    }
```

### Trait Objects Perform Dynamic Dispatch

- The code that results from **Monomorphization** is performing a **static dispatch** which is when the compiler knows what method you're calling at compile time.
- **Dynamic Dispatch** is when the compiler can't tell you at compile time which method you're calling.
- When we use trait Objects, Rust must use dynamic dispatch.
- The compiler doesn't know all the types that might be used with the code that is using trait objects, so it doesn't know which method implemented on which to call.
- Instead, at runtime, Rust uses the pointers inside the trait object to knwo which method to call.
- There is also a runtime cost when this lookup happens that doesnt occur which static dispatch.

### Object Safety is Required for trait Objects

- We can only make object-safe traits into trait objects.
- Some complex rules govern all the properties that make a trait object safe, but in practice only two rules are relevant.
- A trait is Object-safe if all the methods defined in the trait have the following properties:

1. The return type isn't `Self`.
2. They are no generic Type parameters.

- An example of a trait whose methods are not object safe is the standard library `Clone` trait.

```rs
    pub trait Clone {
        fn clone(&self) -> Self;
    }
```

- The compiler will indicate when you're trying to do something that violates the rules of Object safety in regard to trait objects.

### Implementing an Object-Oriented Design Pattern.

- A **State Pattern** is an Object Oriented Design Patterns.
- The Crux of the pattern is that a value has some internal state which is represented by a set of **State** objects and the values behavior changes based on internal state.
- Lets look at an example of the state design pattern and how to use it in Rust.
- We'll implement a blog post workflow in an incremental way.

```rs
    let mut post = Post::new();

    post.add_text("Hello, guys");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello, guys", post.content);
```

### Defining Post and Creating a New Instance in the Draft State

```rs
    trait State {}

    struct Draft {}

    impl State for Draft {}

    pub struct Post {
        state: Option<Box<State>>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
    }
```

### Storing the Text of the Post Content

```rs
    impl Post {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
    }
```

### Ensuring the Content of a Draft Post is Empty

- For now we'll make it return an empty string

```rs
    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            ""
        }
    }
```

### Requesting a Review of the Post Changes its State.

```rs
    trait State {
        fn request_review(self: Box<Self>) -> Box<State>
    }
    impl Post {
        ...
        pub fn request_review(&mut self) {
            if let some(s) = self.status.take() {
                self.state = Some(s.request_review())
            };
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(Self: Box<Self>) -> Box<State> {
            Box::new(PendingReview {})
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }
    }

```

- We give Post a public method named request_review that will take a mutable reference to self
- Then we call an internal request_review method on the current state of Post and this second request_method consumes the current state and returns a new state.
- We've added a request_review method on the State trait also.
- **NB** - Rather than having self, &self or &mut self as the first parameter of the method, we have Self: Box<Self> - This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
- We can now see the advantage of the state pattern, the request_review method or Post is the same no matter its state value. Each State is responsible for its own rules.

### Adding the `approve` method that changes the behavior of content

```rs
    impl Post {
        ...
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            };
        }
    }

    trait State {
        ...
        fn approve(self: Box<Self>) -> Box<State>
    }

    impl State for Draft {
        ...
        fn approve(self: Box<Self>) -> Box<State> {
            self
        }
    }

    impl State for PendingReview {
        fn approve(self: Box<Self>) -> Box<State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(Self: Box<Self>) -> Box<State> {
            Self
        }
    }
```

- We add the approve method to the state trait and a new struct that implements State, The Published State
- Similar to request_review method, if we call the approve method one Draft, it will have no effect because it will return Self.
- Next, we need to update the content method on Post for when the State is Published, we want to return a value in the posts content field, otherwise we return an empty string slice.

```rs
    impl Post {
        ...
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }
    }
```

- The goal is to keep all these rules inside the structs that implement State, we call the content method on the value in State and pass a post instance as an argument.
- We call `as_ref` method on the Option because we want a reference to the value inside the Option rather than taking ownership of the value.
- If we didn't call `as_ref`, we'll get an error because we cannot move State out of the borrowed &self of the function parameter.

- We then call unwrap method which we know will never fail because we are sure that State will always contain Some value.
- This is on of the cases where we have more info than the compiler.

```rs
    trait State {
        ...
        fn content<'a>(&self, post &'a Post) -> &'a str {
            ""
        }
    }

    impl State for Published {
        fn content<'a>(&self, post &'a Post) -> &'a str {
            &Post.content
        }
    }
```

- We've added a default implementation for the content method that returns an empty string slice.

### Trade-offs of the State Pattern.

- The State pattern has some trade offs which includes Tightly coupled code and a lot of Code Duplication.

### Encoding States and Behaviors as Types

- Rather than encapsulating the States and transitions completely so outside code has no knowledge of them. We could encode the States into different types.

### Implementing Transitions as Transformations into Different Types

- We want to enforce the rule that a draft post has to be reviewed and approved before it can be published.
- A Post in Pending review State should still not display any content

```rs
    impl DraftPost {
        pub fn request_review(self)-> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    pub struct PendingReviewPost {
        content: String
    }
    impl PendingReviewPost {
        pub fn approve(Self) -> Post {
            Post {
                content: self.content
            }
        }
    }
```

- The request_review and approve methods take ownership of Self, thus consuming the DraftPost and PendingReviewPost instances and transforming them into a PendingReviewPost and Publish Post respectively.
- This way we wont have any lingering DraftPost instances after we've called request_review on them and so forth.
- We've encoded the blog Post workflow into the types system.

```rs
    let mut post = Post::new();
    post.add_text("Hellow");
    let post = post.request_review();

    let post = post.approve();
```

- The request_review and approve methods return new instance rather than modifying the struct they're called on so we need to add more `let post =` shadowing assignments to save the returned instances
