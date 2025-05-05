pub mod binary_tree;
pub mod custom_sp;
pub mod dropping;
pub mod reference_counting;
fn main() {
    println!("Hello, world!");


    let mut tree = binary_tree::BinaryTree::new(1);

    tree.insert_value(10);
    tree.insert_value(5);
    tree.insert_value(6);
    tree.insert_value(3);
    tree.insert_value(60);
    tree.insert_value(25);
    tree.insert_value(18);


    println!("{:?}", tree);
}
