pub mod helpers;
fn main() {
    println!("Hello, world!");

    let name = helpers::get_name( "Joshua",  "Uzoagulu");
    println!("{}", name);

    helpers::child_mod::print_msg();
}
