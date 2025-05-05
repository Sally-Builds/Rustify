use std::rc::Rc;

pub struct Graph {
    value: u32,
    children: Vec<Rc<Graph>>
}

impl Graph {
    pub fn new(value: u32) -> Graph {
        Graph { value, children: vec![] }
    }



}

// fn main_rc() {
//     let child = Graph::new(10);
//     let graph1 = Graph {value: 12, children: }

//     let 
// }