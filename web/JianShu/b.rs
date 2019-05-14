//rust所有权
//
//所有权指涵盖的范围有: 变量作用域、垃圾处理机制、引用，围绕着几种行为来阐述所有权概念。
//
//变量作用域(Scope)
//任何编程语言中都存在变量作用域的概念, Rust中的变量作用域与其他编程语言也基本一致；作用域大致上会分为: 全局作用域, 函数内作用域.
//
//全局作用域
//在下面这段代码中, PROGRAM常量和main函数就是一个全局作用域; 反过来讲就是当前程序代码中，全局作用域拥有两个对象, 第一个对象是 PROGRAM 常量， 第二个对象是 main 函数。

const PROGRAM: &'static str = "Rust";

fn main() {
    println!("{}", PROGRAM);
}


//# 作用域的形式可以这么理解, global_scope中有两个对象.
global_scope = {
"const PROGRAM": "Rust"
"fn main": "println!({}, PROGRAM)"
}
//函数内作用域
//变量不能let(定义)在全局作用域中, 所以全局作用域能做的事情并不多: 定义常量、定义函数、引用其他模块文件; 更多的合法操作实在函数作用域中完成: 定义变量、打印变量、逻辑计算、逻辑判断、函数调用和执行等.

const PROGRAM: &'static str = "Rust";

fn main() {
    let s = "abc";                      // 定义变量
    let sf = simple_function();         // 执行函数
    let sum = 10 + 15;                  // 逻辑计算
    if sum > 0 {                        // 逻辑判断
        println!("{} {} {}", PROGRAM, s, sum);  // 打印常量和变量
    }
}

fn simple_function() {
    let sf = "Simple Function";         // 定义变量
    sf                                  // return 变量
}


//# 作用域的形式可以这么理解, global_scope中有三个对象, 其中
//# main对象中又有一个二级作用域, simple_function对象中也有一个
//# 二级作用域.
global_scope = {                        // 作用域

"const PROGRAM": "Rust",

"main": {                           // 作用域
"let s": "abc",
"let sf": "Simple Function",
"let sum": 25,
},

"simple_function": {                // 作用域
"let sf": "Simple Function"
}
}



//垃圾处理机制
//像其他具备GC回收机制编程语言一样，当程序执行完成并跳出某个作用域(通常指的是一个函数)时，该作用域中的所有变量将会失效(被回收)；除此之外, Rust对垃圾回收这件事情上还具备其他的能力和行为, 例如: 当需要对存储在堆(Heap)中的数据进行复制时, 被赋值对象将具备赋值对象的数据，而赋值对象将会被回收。

//离开作用域时回收数据
fn simple_function() {
    let sf = "Simple Function";    // sf变量开始生效
    println!("{}", sf);            // sf变量仍然生效
}                                  // sf变量不再生效

fn main() {
    simple_function();             // 当该函数执行完成后, sf变量就会被回收
    println!("{}", sf);            // 报错: sf变量不存在.
}
//变量传递后即刻失效

fn simple_function(sf: String) {
    println!("simple_function: {}", sf);
}

fn main() {
    let s = String::from("hello");
    simple_function(s);
    println!("main: {}", s);
}



//# 报错
//Compiling ownership v0.1.0 (file:///opt/learn_rust/ownership)
//error[E0382]: use of moved value: `s`
//--> src/main.rs:10:26
//|
//9  |     simple_function(s);
//|                     - value moved here
//10 |     println!("main: {}", s);
//|                          ^ value used here after move
//|
//= note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait

//通过return让它重回原作用域

fn simple_function(sf: String) -> String {  // -> String 声明返回值类型
    println!("simple_function: {}", sf);
    sf
}

fn main() {
    let s = String::from("hello");
    let s = simple_function(s);     // 利用返回值, 重新定义变量.
    println!("main: {:?}", s)
}

//数据存储在栈上时, 变量再赋值不会回收原变量
fn main() {
    let a = 10;                 // rust会将固定不变的值存储在栈(Stack)上.
    let b = a;                  // rust对栈上的数据默认采取深复制.
    println!("{} {}", a, b);
}

fn main() {
    let a = "abc";              // 与上面是一样的.
    let b = a;
    println!("{} {}", a, b);
}
//当变量存储再堆上时, 变量再赋值就会回收原变量(所有权转移)
fn main() {

    // rust会将这种未知大小的数据存储在堆(Heap)上.
    // 因为String::from这种结构的数据支持push_str
    // 来扩充它的大小, 因此从本质上来讲是未知大小.
    let a = String::from("hello");

    // rust对堆上的数据默认采取移除上一个变量创建
    // 新变量的机制, 这种做法在术语上叫做所有权转移.
    let b = a;

    // 这里会报错, a变量已被移除
    println!("{} {}", a, b)
}


//# 报错信息
//Compiling ownership v0.1.0 (file:///opt/learn_rust/ownership)
//error[E0382]: use of moved value: `a`
//--> src/main.rs:10:23
//|
//7  |     let b = a;
//|         - value moved here
//...
//10 |     println!("{} {}", a, b)
//|                       ^ value used here after move
//|
//= note: move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait




//引用(reference)
//不可变更引用(默认)
fn main() {
    let a = 10;

    // b变量是一个指针, 它并没有实际数据的所有权.
    let b = &a;

    // 引用是在栈上创建一个指针指向栈数据.
    // 它比在栈上深复制更轻量.
    println!("{} {}", a, b)
}
//可变引用
fn main() {
    let mut a = "hello";
    a + "b"
    println!("{}", a)
}
//引用的注意事项
//可变对象不能被多次引用, 这会导致数据竞争.
fn main() {
    let mut a = 10;
    let b = &mut a;
    let c = &mut a;
}

//# 报错
//Compiling ownership v0.1.0 (file:///opt/learn_rust/ownership)
//error[E0499]: cannot borrow `a` as mutable more than once at a time
//--> src/main.rs:18:18
//|
//17 |     let b = &mut a;
//|                  - first mutable borrow occurs here
//18 |     let c = &mut a;
//|                  ^ second mutable borrow occurs here
//19 | }

//可变对象被可变引用之后, 再次引用会导致数据不一致.
fn main() {
    let mut a = 10;
    // 可变对象被可变引用走了
    let b = &mut a;
    // 这里会报错, 因为数据状态任何时刻都可能会改变,
    // 这是不可预期的, 所以rust不允许这种情况的出现.
    println!("{} {}", a, b)
}

//可变对象被可变引用之后，数据不一致的解决办法
fn simple_function(sf: &mut String) -> String {
    sf.push_str(" world!");
    let new_sf = sf.clone();
    new_sf
}

fn main() {
    let mut a = String::from("hello");
    // 站在变量的角度来讲: &mut a 的专业术语为<可变引用>
    // &mut a 当做参数传递给函数时, 专业术语为<可变借用>
    let b = simple_function(&mut a);
    println!("a: {}\nb: {}", a, b)
}