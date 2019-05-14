//rust--如何获取Some里面的值?
fn example1() {
    let a = Some("hello");
    match a {
        Some(b) => println!("show variable a: {}", b),
        _ => println!("nothing")
    }
}


fn example2() {
    let a = Some("hello");
    println!("show variable a: {}", a.unwrap())
}


fn main() {
    example1();
    example2();
}