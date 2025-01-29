// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

fn main() {
    #[derive(Debug)]
    struct Order {
        price: u64,
        quantity: u64,
    }

    impl Order {
        fn new(price: u64, quantity: u64) -> Order {
            Order {price, quantity}
        }

        fn is_available(&self) -> bool {
            let result = if self.quantity > 0 {
                true
            }else {
                false
            };
            result
        }
    }


    let first_order = Order::new(100, 5);

    println!("Order price: {}, Order quantity: {}", first_order.price, first_order.quantity);
    println!("using Debug Trait - {:?}", first_order);
    println!("using Debug Trait with pretty formatter - {:#?}", first_order);
    println!("is order available? {}", first_order.is_available());
}
