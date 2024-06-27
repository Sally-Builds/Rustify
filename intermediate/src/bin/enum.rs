enum Mouse {
    LeftClick,
    RightClick,
    Scroll(i32),
    Move(i32),
}


fn main () {
    let mouse: Mouse = Mouse::Scroll(10);
    
}


