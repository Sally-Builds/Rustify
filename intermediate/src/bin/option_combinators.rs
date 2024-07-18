fn main () {
    let a: Option<i32> = Some(78);
    dbg!(a);

    let is_some = a.is_some();
    dbg!(is_some);
    let is_none = a.is_none();
    dbg!(is_none);

    let a_map = a.map(|a| a);
    dbg!(a_map);
    let a_filter = a.filter(|a| *a >= 50);
    dbg!(a_filter);

    let or_else = a.or_else(|| Some(9));
    dbg!(or_else);
    let unwrapped_or_else = a.unwrap_or_else(|| 9);
    dbg!(unwrapped_or_else);

}