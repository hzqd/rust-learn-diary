//rust--reference和borrow的区别?

// reference和borrow有什么区别?
//
// reference和borrow其实是一个含义, 但是被拆分成两个单词,
// borrow是指函数运行必备的参数变量, 需要依赖外部提供, 这种参数声明就叫做borrow(借用).


fn reference_example() {
    let a = String::from("reference hello");
    let b = &a;                             // 这是引用: reference.
    println!("a: {}, b: {}", a, b)
}


fn borrow_example() {
    let a = String::from("borrow hello");

    fn print(msg: &String) {                              // 这是借用: borrow
        println!("{:?}", msg)
    }

    print(&a)
}


fn main() {
    reference_example();
    borrow_example();
}