fn test (a: User) {
    println!("{}", a.age);
}

struct User {
    age: i32,
}

struct GroceryItem {
    qty: i32,
    id: i32,
}

fn display_qty (qty: i32) {
    println!("{}", qty);
}

fn display_id (grocery: &GroceryItem) {
    println!("{}", grocery.id);
}

fn main() {
    //Ownership
    let user: User = User {
        age: 32
    };

    test(user);
    // test(user); // wont compile due to variable moving

    let grocery = GroceryItem{
        id: 1,
        qty: 12,
    };

    display_qty(grocery.qty);
    display_id(&grocery);


    //Implementation Impl
}
