
#[derive(Debug)]
pub struct BinaryTree {
    value: u32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>
}

impl BinaryTree {
    pub fn new(value: u32) -> BinaryTree {
        BinaryTree { value, left: None, right: None, }
    }

    pub fn insert_value(&mut self, value: u32) {
        if value > self.value {
            match self.right.as_mut() {
                Some(v) => {
                    v.insert_value(value);
                },
                None =>  {
                    let leaf = BinaryTree {value, left: None, right: None,};
                    self.right = Some(Box::new(leaf));
                }
            }
            
        }else if value < self.value {
            match self.left.as_mut() {
                Some(v) => {
                    v.insert_value(value);
                },
                None =>  {
                    let leaf = BinaryTree {value, left: None, right: None,};
                    self.left = Some(Box::new(leaf));
                }
            }
        }
    }  
}