
#[derive(Debug, Clone, Copy)] //makes an automatic copy 
enum Status {
    Pending,
    Declined,
    Successful
}


#[derive(Debug, Clone, Copy)]
struct Transaction {
    amount: u128,
    status: Status
}

fn print_enum(en: Status) {
    println!("{:?}", en);
}

fn main () {
    //
    let my_status: Status = Status::Successful;
    let tx: Transaction = Transaction {amount: 80, status: Status::Pending};
    println!("{:?}", my_status);
    println!("{:?}", tx);

    print_enum(tx.status);
    print_enum(tx.status);
}