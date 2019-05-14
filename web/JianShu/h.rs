//rust--什么是ownership?
// 什么是所有权?
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn print(msg: String) {
    println!("{}", msg)
}


// 数据赋值 和 所有权转移
fn create_a_variable() {
    // system 是变量名, "hello" 是数据
    // 数据"hello"存储在内存中.
    // 变量名 system  拥有数据 "hello" 的所有权.
    let system = String::from("windows");

    // 变量当作参数传递, 默认情况下会转移所有权,
    // 并且变量名会被回收掉, 后续程序不能再调用这个变量名.
    print(system);
    // print(system);      // 这里会报错: value used here after move

    {
        let a = "hello";
    }

    println!("{}", a)     // 这里会报错: cannot find value `a` in this scope

    // 备注:
    // 这个函数中的代码, 展示了所有权的所有规则.
    // let system = String::from("windows") 对应的是第一条规则.
    //
    // print(system) 对应的是第二条规则, 数据同一时间只能拥有一个所有权,
    // 也就是说变量当作参数传递进去之后, system就不再拥有 "windows" 这个数据的所有权,
    // 当一个变量名没有所有权时, 编译器会自动回收该变量名.
    //
    // println!("{}", a) 对应的是第三条规则, 它引用了一个子作用域中的变量名,
    // 子作用域中的变量名并没有传递出来, 因此编译器会drop掉这个变量名.
}


fn main() {
    create_a_variable()
}