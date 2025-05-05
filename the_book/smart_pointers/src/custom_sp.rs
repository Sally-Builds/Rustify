use std::ops::{Deref, DerefMut};


pub struct CustomSP<T> {
    data: Vec<T>
}


impl<T> CustomSP<T> {
    pub fn new() -> CustomSP<T> {
        CustomSP { data: vec![] }
    }
}

impl<T> Deref for CustomSP<T>  {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CustomSP<T>  {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
    }
}