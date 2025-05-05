# 16) Fearless Concurrency

- Concurrent Programming is when different parts of a program execute independently.
- Parallel programming is when different parts of a program execute at the same time.
- These concepts are becoming more prevalent as more computers take advantage of their multiple processors.

## Using Threads to Run Code Simultaneously

- Executed program code runs in a **process** and the Operating system manages multiple processes at once.
- within our program, we can have independent parts that run simultaneously. The features that run these independent parts are called threads.
- having multiple threads can improve performance but it also adds complexity.

- The order in which the threads run cannot be known. This leads to a problem such as

  - **Race Condition** in which threads access data or resources in an inconsistent order.
  - **Deadlocks** where two threads are waiting on each other to finish using a resource the other thread has, preventing both threads from continuing.

- Rust attempts to mitigate these negative effects of using threads but programming in multithreaded context will take careful thoughts and different code structure from a single threaded context.

- Programming languages implement threads in a few different ways; Many OS provides an API for creating new threads.
- The model where a language calls the OS API to created a thread is sometimes called **1:1** meaning one OS thread per on language thread.

- Some Programming languages provide their own special implementation of threads. These threads are known as **Green threads** and languages that use them will execute them in the context of a different number of OS Threads.
- The **Green Threaded model** is called **M:N** model; where **M** green threads per **N** OS threads, where M and N are not necessarily the same number.
- Each model has its own advantage and trade offs, and the trade off most important in Rust is **Runtime support**.
- In this context **runtime** means code that is included by the language in every binary.
- The green threaded model(M:N) requires a larger language runtime to manage threads.
- Hence why Rust standard library implement 1:1 threading model.
- They are some crates that implement M:N threading.

## Creating a New Thread with Spawn

- To create a new thread, we call the `thread::spawn`` function and pass it a closure containing the code we want to run in the new thread.

```rs
    use std::thread;
    use std::time::Duration;

    fn main () {
        thread::spawn(|| {
            for i in 1..10 {
                println!("{i} from spawned thread");
                thread::sleep(Duration::from_millis(1));
            }
        })

        for i in 1..10 {
            println!("{i} from main thread")
                thread::sleep(Duration::from_millis(1));
        }
    }
```

- The call to `thread::sleep` forces a thread to stop its execution for a short duration allowing a different thread to run.

## Waiting for All threads to finish using Join handles

- the code above stops the spawned thread prematurely most of the time due to the main thread ending, but we also cannot guarantee that the spawned thread will get to run at all.
- The reason is that there is no guarantee on the order in which threads run.

- We can fix the problem of the spawned thread not getting to run or not running completely by saving the return value of thread::spawn in a variable.
- The return type of thread::spawn is `JoinHandle`. This is an owned value we can call `join` method on.
- Calling the `join` method on JoinHandle type will wait for its thread to finish.

```rs
    fn main {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                ...
            }
        })

        for i in 1..5 {
            ...
        }

        handle.join().unwrap();
    }
```

- Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
- **Blocking** a thread means that thread is prevented from performing work or exiting.
- small details such as where `join` is called can affect whether or not our threads run at the same time.

## Using `move` closures with thread.

- The move closure is often used along side thread::spawn because it allows us to use data from one thread in another thread.

```rs
    let v = vec![1, 3];

    let handle = thread::spawn(|| {
        println!("vec is {v}");
    })
```

- The code above wont compile.
- Rust infers how to capture `v` and because `println!` only needs a reference to v, the closure tries to borrow `v`.
- However, Rust cannot tell how long the spawned thread will run, so it doesn't know whether the reference to `v` will always be valid.
- By adding `move` keyword before the closure, we force the closure to take ownership of the values its using rather than allowing Rust to infer that it should borrow the values.

```rs
    let v = vec![1, 3];

    let handle = thread::spawn(move || {
        println!("vec is {v}");
    })
    //we cannot use v again because it has been moved.
```

## Using Message Passing to Transfer Data between threads.

- Message passing is a concept where threads or actors communicate by sending each other messages containing data. This enures safe concurrency.
- there is popular Go slogan that say "Do not communicate by sharing memory rather share memory by communicating."
- A major tool Rust has for accomplishing message sending concurrency is the **Channel**.
- A Channel in programming has two(2) halves: a **transmitter** and a **receiver**.
- One part of our code calls methods on the transmitter with data you want to send and another part checks the receiving end for arriving messages.

- A channel is said to be closed, if either the transmitter or receiver half is dropped.

```rs
    use std::sync::mpsc;

    fn main () {
        let (tx, rx) = mpsc::channel();
    }
```

- **`mpsc`** stands for multiple Sender, Single Consumer.
- Rust standard library ensures we can have multiple sending ends that produces values but only one receiving end that consumes those values.

```rs
    use std::sync::mpsc;

    fn main () {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let value = String::from("Hello");

            tx.send(value).unwrap();
        })

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

- The transmitting end has a `send` method that takes the value we want to send.
- The send method returns a `Result<T, E>` type.
- If the receiving end has been dropped, and there is nowhere to send a value, the send operation will return an error.

- The receiving end of a channel has two useful methods `recv` and `try_recv`.
- `recv` will block all the main threads execution and wait until values is sent down the channel.
- Once a value is sent `recv` returns a `Result<T, E>` when the sending of a channel closes, `recv` will return an error to signal that no more values will be coming in.
- `try_recv` method doesn't block but will instead return Result<T, E> immediately with an OK value holding a message or Err if no message.
- try_recv is useful if the thread has other work to do while waiting for messages. it is normally used in a loop.

## Channels and Ownership Transference.

- The ownership rules play a vital role in message sending because they help you write safe concurrent code.
- If we try to use a variable after we've sent it as a message using `tx.send(val)` we would get an ownership compilation error.
- the `send` method takes ownership of the value.

## Sending Multiple Values and Seeing the receiver waiting.

- we will use an example where the spawned thread sends a series of messages stored in a vector and then wait one second interval between the messages.

```rs
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let messages = vec![String::from("Hello"), String::from("My Friend")];

        for message in messages {
            tx.send(message);
            thread::sleep(Duration::from_secs());
        }
    })

    for received in rx {
        println!("value = {}", received);
    }
```

## Creating Multiple Producers by Cloning the Transmitter.

```rs
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::sender::clone(&tx);

    thread::spawn(move || {
        tx.send("from thread tx");
    })

    thread::spawn(move || {
        tx1.send("from thread tx1");
    })
```

- Calling clone on the sending end of the channel gives us a new sending handle.
- The message will arrive in a non deterministic order.

## Shared-State Concurrency

- Message passing is not th only way of handling concurrency.
- From the Go slogan "Communicate by Sharing Memory", what exactly does it mean?
- In a way, channels in any programming language are similar to single ownership, because once you transfer the value down a channel, you should no longer use it.
- Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.
- As seen earlier with smart pointers that allows multiple ownership, it adds some complexity because different owners need managing.

## Using Mutexes to Allow Access to Data from One thread at a time.

- Mutex is an abbreviation for mutual exclusion, ie a mutex allows only one thread to access some data at any given time.
- To access data in a mutex, the thread must first signal that it wants access by asking to acquire the mutex's **`Lock`**.
- The **Lock** is a data structure that is part of the mutex that keeps tracks of who currently has exclusive access to the data.
- The Mutex is guarding the data it holds through a **Locking System**.
- Mutex has 2 rules:

  1. You must attempt to acquire the lock before using the data.
  2. When done with the data, you must unlock the data so other threads can acquire the lock.

- Management of mutexes can be tricky to get right, hence why so many people prefer channels.
- With Rust type systems and ownership rules, you cannot get locking and unlocking wrong.

## The API of Mutex<T>

```rs
    use std::sync::Mutex;

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num =6;
        }

        println!("m = {m}"); //single threaded example
    }
```

- To access the data inside a mutex, we use the lock method to acquire the lock.
- The call to lock will block the current thread so it cant do any work until its its turn to have the lock.
- The call to lock will fail if another thread holding the lock panicked, which means no one would ever be able to get the lock.
- Mutex is a smart pointer, the call to lock also returns a smart pointer called **`MutexGuard`**.
- The smart pointer implements Deref and Drop traits.

## Sharing a Mutex<T> between Multiple threads.

```rs
    use std::sync::Mutex;
    use std::thread;

    fn main () {
        let counter = Mutex::new(0);

        let mut handles = vec![];

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
        handles.push(handle);

         let handle2 = thread::spawn(move || {
            let mut num2 = counter.lock().unwrap();
            *num2 += 1;
        })
        handles.push(handle2);

        for handle in handles {
            handle.join().unwrap();
        }
    }
```

- The code above wont work, we get an error indicating that counter is moved into the closure for the thread associated with handle. The move is preventing us from capturing counter when we try to call lock on it and store the result in num2 in the second thread.

- Rust is saying we cant move ownership of counter into multiple threads.

## Multiple Ownership with Multiple threads

- In the previous chapter 15 (Smart Pointers), we could have multiple owners by using the RC<T> smart pointer to create a reference counted value.
- We can wrap Mutex<T> in an RC<T> and clone the RC<T> before moving ownership to the thread.
- This wont work unfortunately, RC<T> is not safe to share across threads.

## Atomic Reference Counting with Arc<T>

- Arc<T> is a type like RC<T> that is safe to use in concurrent situations.
- the 'A' stands for **atomic** meaning it is an **Atomic Reference Counted** type.
- Atomic works like primitive types but safe to share across threads.
- Not all primitive types are atomic because thread safety comes with a performance penalty that you only want to pay when you really need to.

```rs
    fn main () {
        let handles = vec![];

        let counter = Arc::new(Mutex::new(0));

        for i in 1..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            })

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
```

## Similarities between `RefCell<T>`/`RC<T>` and `Mutex<T>`/`Arc<T>`

- notice that in the example above, the counter is immutable, but we could get a mutable reference to the value inside it. This means Mutex<T> provides interior mutability as the `Cell` family does.
- Same way we use RefCell<T> to allow us to mutate contents inside an RC<T> we use Mutex<T> to mutate contents inside Arc<T>.

- Rust can't protect us from all kinds of logical errors while using Mutex<T>.
- Just as RC<T> creates reference cycles causing **memory leaks**, Mutex<T> comes with risks of creating **deadlocks** - this occurs when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

## Extensible Concurrency with Sync and Send traits

- Rust language has very few concurrency features.
- Almost every concurrency feature talked about so far has been part of the standard library not the language.
- We can also write our own concurrency features or use those written by others.
- Two(2) concurrency concepts are embedded into the language which are `std::marker` traits `Send` and `Sync`.

## Allowing Transference of Ownership between threads with send

- The `Send` marker trait indicates that ownership of the type implementing Send can be transferred between threads.
- Almost every Rust type is `Send`, with a few exceptions like RC<T>
- **RC<T> cannot be Send because if you cloned RC<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. For this reason we use RC<T> only in single-threaded situations**

## Allowing Access from Multiple threads with Sync

- The `Sync` marker traits indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
- any type T is Sync if &T(a reference to T) is Send.
- RC<T> and RefCell<T> are not Sync for the same reason they are not Send.

## Implementing Send and Sync manually is Unsafe

- Manually implementing these traits involves implementing unsafe Rust code.
