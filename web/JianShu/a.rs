//rust语法
//单引号和双引号

//错误的写法
fn main() {
    println!('hello world');
}
//# 错误信息
//error: character literal may only contain one codepoint: ')
// --> src/main.rs:2:26
//  |
//2 |     println!('hello world')
                             ^^
//正确的写法
fn main() {
    println!("hello world");
}

//小结
//变量定义全部采用双引号, 不能采用单引号。

//行结束符 ;
//错误的写法
fn main() {
    println!("hello world")
    println!("sssssssssss")
}
//# 错误信息
//error: expected one of `.`, `;`, `?`, `}`, or an operator, found `println`
// --> src/main.rs:3:5
//  |
//2 |     println!("hello world")
//  |                            - expected one of `.`, `;`, `?`, `}`, or an operator here

//正确的写法
fn main() {
    println!("hello world");
    println!("sssssssssss");
}

//小结
//每行执行代码后面最好带上一个行结束符;。

//缩进
//无缩进(支持)
fn main() {
println!("hello world"); 
}
//空格缩进(支持)
fn main() {
    println!("hello world");
}
//tab缩进(支持)
fn main() {
        println!("hello world");
}
//小结
//rust 对缩进没有要求, 但是鼓励采用4个空格来对代码块进行缩进.

//main特殊函数
//直接将代码暴露在全局作用域中, 也就是说不写main.
println!("hello world");
# 报错信息
//error: macros that expand to items must either be surrounded with braces or followed by a semicolon
// --> <println macros>:2:9
//  |
//2 | print ! ( concat ! ( $ fmt , "\n" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *
//  |         ^^^^^^^^^^^^^^^^^^^

//将代码封装在函数中, 不写main.
fn simple() {
    println!("hello world");
}
//# 报错信息
//error: main function not found

//正确的写法
fn simple() {
    println!("hello world");
}
fn main() {
    simple();
}

//main函数不能定义参数
fn main(first: i32, second: f64) {     // 不能定义参数
    println!("{} {}", first, second)
}

//main函数不能定义返回值
fn main() -> String {                   // 不能定义返回值
    let s = String::from("hello");
    s
}

//普通函数使用return返回值
fn simple_function() -> i32 {
    let s = 10;
    return s
}
fn main() {
    let s = simple_function();
    println!("{}", s)
}

//普通函数不用return返回值(官方推荐、源码规范)
fn simple_function() -> i32 {
    let s = 10;
    s
}
fn main() {
    let s = simple_function();
    println!("{}", s)
}

//小结
//在全局作用域中不能直接撰写执行类型的语句。
//main特殊函数, 是所有程序的入口, 所以封装好所有代码之后, 必须放入到main中被rust识别和运行。

 
//变量
//变量不能定义在函数外
let s = "a";
fn main() {
    println!("hello world! {}", s)
}
//# 报错信息
//error: expected item, found `let`
// --> main.rs:1:1
//  |
//1 | let s = "a";
//  | ^^^
  
//# 正确的写法
fn main() {
    let s = "a";   // 变量定义写在函数内
    println!("hello world! {}", s);
}

//变量不能直接被打印
fn main() {
    let s = "a";
    println!(s);
}
//# 报错信息
//error: expected a literal
// --> main.rs:3:14
//  |
//3 |     println!(s);
//  |              ^
  
//# 正确的写法
fn main() {
    let s = "a";
    let x = "xxx";
    println!("{} {}", s, x);
}

//变量默认不能被修改
fn main() {
    let s = 1;
    s += 2;
    println!("{}", s)
}
//# 报错信息
//error[E0384]: re-assignment of immutable variable `s`
// --> main.rs:3:5
//  |
//2 |     let s = 1;
//  |         - first assignment to `s`
//3 |     s += 2;
//  |     ^^^^^^ re-assignment of immutable variable

//# 正确的写法
fn main() {
    let mut s = 1;
    s += 2;
    println!("{}", s);
}

//可更改变量必须类型相同
fn main() {
    let mut s = 10;
    s += 'abc';
}
//# 报错
//error: character literal may only contain one codepoint: 'abc'
//  --> main.rs:25:10
//   |
//25 |     s += 'abc';
//   |          ^^^^^
//error: aborting due to previous error(s)


//变量遮盖(Shadowing)
fn main() {
    let s = 1;
    let s = 2 + 10;
    println!("{}", s);
}

//# 编译时报警告, 运行时正常; 编译器认为这种写法并不符合规范
//# 但也是可以运行的, 因为第一个`let s = 1`没有被使用就被
//# 覆盖掉了, 这种情况完全可以不用定义这行代码.
//   Compiling datatype v0.1.0 (file:///opt/learn_rust/datatype)
//warning: unused variable: `s`
// --> src/main.rs:2:9
//  |
//2 |     let s = 1;
//  |         ^
//  |
//  = note: #[warn(unused_variables)] on by default

//    Finished dev [unoptimized + debuginfo] target(s) in 0.46 secs
//     Running `target/debug/datatype`
//12

//# 正确的Shadowing写法
fn main() {
    let s = 1;
    let s = s + 10;
    println!("{}", s);
}

//变量赋值
//整数类型变量
fn main() {
    // "i32"   
    // 公式: -(2 ** (32 - 1))  ~  (2 ** (32 - 1)) -1
    // 结果: -2147483648     ~   2147483647
    let s = 150;          
    println!("{}", s);    
}
fn main() {
    // "u32"   
    // 公式: 0 ~ (2 ** (32 - 1)) -1
    // 结果: 0 ~  4294967295
    let s: u32 = 4294967295;          
    println!("{}", s);    
}

//浮点类型变量
fn main() {
    let s = 3.14;       // "f64"
    println!("{}", s);
}

//字符串变量
fn main() {
    let s = "hello";      // "&str"
    println!("{}", s);
}

//数组数字变量
fn main() {
    let s = [1, 2, 3, 4];   // "[i32; 4]"
    println!("{}", s);
}

//# 编译时报错,
//   Compiling datatype v0.1.0 (file:///opt/learn_rust/datatype)
//error[E0277]: the trait bound `[{integer}; 4]: std::fmt::Display` is not satisfied
// --> src/main.rs:3:20
//  |
//3 |     println!("{}", s);
//  |                    ^ the trait `std::fmt::Display` is not implemented for `[{integer}; 4]`
//  |
//  = note: `[{integer}; 4]` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
//  = note: required by `std::fmt::Display::fmt`

//error: aborting due to previous error

//error: Could not compile `datatype`.

//To learn more, run the command again with --verbose.

//# 正确的写法
fn main() {
    let s = [1, 2, 3, 4];     // "[i32; 4]"
    println!("{:?}", s);
}

//数组字符串变量
fn main() {
    let s = ["a", "b", "c"];     // "[&str; 3]"
    println!("{:?}", s);
}

//数组混搭元素
//# 数组中的变量只能是相同类型的, 下面这个例子会报错, 详细的介绍请参考这里:
//# https://rustbyexample.com/primitives/array.html  

fn main() {
    let s = ["a", 1, 2, "b"];     
    println!("{:?}", s);
}

//error[E0308]: mismatched types
//  --> main.rs:12:19
//   |
//12 |     let s = ["a", 1, 2, "b"];
//   |                   ^ expected &str, found integral variable
//   |
//   = note: expected type `&str`
//              found type `{integer}`

//error: aborting due to previous error(s)

//# 数组中的变量只能是相同类型, 并且元素内的结构也要保持一直.
//# 错误的写法
fn main() {
    let s = [
        ["a", "b", "c", "d"],
        ["e", "f", "g", "h", "i"],
    ];   
    println!("{:?}", s);
}

//# 正确的写法
fn main() {
    let s = [                               // "[[&str; 4]; 6]"
        ["a", "b", "c", "d"],
        ["e", "f", "g", "h"],
        ["i", "j", "k", "l"],
        ["m", "n", "o", "p"],
        ["q", "r", "s", "t"],
        ["q", "r", "s", "t"],
    ];  
    println!("{:?}", s);
}

//# 正确的写法
fn main() {
    let s = [                               // "[(&str, i32); 6]"
        ("a", 1), ("b", 2), ("c", 3),
        ("d", 4), ("e", 5), ("f", 6),
    ];   
    println!("{:?}", s);
}

//元祖数字变量
fn main() {
    let s = (1, 2, 3, 4, 5);                // "(i32, i32, i32, i32, i32)"
    println!("{:?}", s);
}

//元祖字符串变量
fn main() {
    let s = ("a", "b", "c");                // "(&str, &str, &str)"
    println!("{:?}", s);
}

//元祖混搭变量
fn main() {
    let s = (                               
        "a", 1, 2, "b",                     // "(&str, i32, i32, &str, 
        [3, 4], ["c", "d"],                 //   [i32; 2], [&str; 2], 
        [[4, 5], [6, 7]],                   //   [[i32; 2]; 2], 
        [["e", "f"], ["g", "h"]]            //   [[&str; 2]; 2])"
    );
    println!("{:?}", s);
}

 
//我是如何打印这些变量类型的?
// 使用rust-nightly版本.
// 代码参考: https://users.rust-lang.org/t/get-the-type-of-a-var/3618/2

#![feature(core_intrinsics)]

use std::intrinsics::type_name;

fn test_type<T>(_: T) {
    println!("{:?}", unsafe { type_name::<T>() });
}

fn main() {
    let s = (
        "a", 1, 2, "b",
        [3, 4], ["c", "d"],
        [[4, 5], [6, 7]],
        [["e", "f"], ["g", "h"]]
    );
    test_type(s);
}

//常量
//字符串常量
const program_language: &'static str = "Rust";

fn main() {
    let hello = "Hello";
    println!("{} {}!", hello, program_language);
}

//# 编译时报警告, 运行时正常;
//# 编译器认为常量的变量名应该是由大写字母构成的;
//   Compiling datatype v0.1.0 (file:///opt/learn_rust/datatype)
//warning: constant `program_language` should have an upper case name such as `PROGRAM_LANGUAGE`
// --> src/main.rs:1:1
//  |
//1 | const program_language: &'static str = "Rust";
//  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//  |
//  = note: #[warn(non_upper_case_globals)] on by default

//    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
//     Running `target/debug/datatype`
//Hello Rust!

//# 正确的写法
const PROGRAM_LANGUAGE: &'static str = "Rust";
fn main() {
    let hello = "Hello";
    println!("{} {}!", hello, PROGRAM_LANGUAGE);
}

//浮点常量
const PROGRAM_VERSION: f64 = 1.18;
const PROGRAM_LANGUAGE: &'static str = "Rust";
fn main() {
    let hello = "Hello";
    println!("{} {}: {}!", hello, PROGRAM_LANGUAGE, PROGRAM_VERSION);
}

//整数常量
//# 数字可以用下划线来标识, 不影响实际的数字数值, 便于阅读.
const NUMBER: i32 = 100_000_000;
fn main() {
    println!("{}", NUMBER);     // 输出: 100000000 
}

//# 有符号整数: "i32"     i == sign == 有符号整数(-表示负数)
//# 公式: -(2 ** (32 - 1))  ~  (2 ** (32 - 1)) -1
//# 结果: -2147483648     ~   2147483647
const ROLLBACK_VERSION: i32 = 2;
const PROGRAM_LANGUAGE: &'static str = "Rust";
fn main() {
    let hello = "Hello";
    println!("{} {}: {}!", hello, PROGRAM_LANGUAGE, ROLLBACK_VERSION);
}


//# 无符号整数: "u32"     u == unsign == 无符号整数(必须是正整数)
//# 公式: 0 ~ (2 ** (32 - 1)) -1
//# 结果: 0 ~  4294967295
const ROLLBACK_VERSION: i32 = 2;
const PROGRAM_LANGUAGE: &'static str = "Rust";
fn main() {
    let hello = "Hello";
    println!("{} {}: {}!", hello, PROGRAM_LANGUAGE, ROLLBACK_VERSION);
}

//数组数字常量
const DATA_TYPE_ARRAY: &'static [i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//# 严谨的写法
const DATA_TYPE_ARRAY: &'static [i32; 10] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//数组字符串常量
const DATA_TYPE_ARRAY: &'static [&'static str] = &["a", "b"];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//# 严谨的写法
const DATA_TYPE_ARRAY: &'static [&'static str; 2] = &["a", "b"];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//数组数组常量
//# 数组中嵌套数组
const DATA_TYPE_ARRAY: [[i32; 2]; 2] = [[1, 2], [3, 4]];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//# 数组中嵌套元祖
const DATA_TYPE_ARRAY: [(&str, i32); 2] = [("a", 1), ("b", 2)];
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//元祖字符串常量
const DATA_TYPE_ARRAY: (&str, &str, &str, i32, i32, i32) = ("a", "b", "c", 1, 2, 3);
fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}

//# 严谨的写法
const DATA_TYPE_ARRAY: (
    &'static str, &'static str, &'static str, i32, i32, i32
) = ("a", "b", "c", 1, 2, 3);

fn main() {
    let hello = "Hello";
    println!("{} {:?}", hello, DATA_TYPE_ARRAY);
}
 
//循环
//for循环数组(迭代)
//# 错误的写法
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in a {
        println!("{}", i)
    }
}

//# 正确的写法
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in a.iter() {
        println!("{}", i)
    }
}

//for循环迭代对象(迭代)

//# 1..9 == [1,2,3,4,5,6,7,8,9].iter()
//# 1..9 == Iter(1,2,3,4,5,6,7,8,9)
//# 1..9是一个迭代对象
fn main() {
    for i in 1..9 {
        println!("{}", i)
    }
}

//# 反转数组迭代对象排列顺序
fn main() {
    for i in (1..9).rev() {
        println!("{:?}", i)
    }
}

//# 排列顺序等同于下面的代码
fn main() {
    let a = [1,2,3,4,5,6,7,8,9];
    for i in a.iter().rev() {
        println!("{:?}", i)
    }
}

//for循环continue关键字
//# 从30开始打印
fn main() {
    let a = 1..100;
    for i in a {
        if i < 30 {
            continue
        }
        println!("{:?}", i)
    }
}

//for循环break关键字
//# 当大于30时，就退出循环
fn main() {
    let a = 1..100;
    for i in a {
        println!("{:?}", i);
        if i > 30 {
            break
        }
    }
}

//while循环
//# 当小于30时, 持续循环
fn main() {
    let mut a = 1;
    while a < 30 {
        println!("{}", a);
        a += 1
    }
}

//# 当小于30时, 持续循环
fn main() {
    let mut a = 1;
    while true {
        if a < 30 {
            println!("{}", a);
        } else {
            break
        }
        a += 1
    }
}

//loop循环
fn main() {
    let mut a = 1;
    loop {
        if a < 30 {
            println!("{}", a)
        }
        a += 1
    }
}

//while true 和 loop 有什么区别?
//从循环的角度看它们没有区别.
//从编译器的角度看, while 后面时一个条件句的解析, 而loop则没有条件句的解析, 因此loop理论上要比while更快.
//从程序员的角度看, while是用来做单个具体业务逻辑判断; 而loop则是用来做更宏观的模块代码设计使用, 把逻辑判断依托于抽象层去完成, 例如read_line获取用户输入来判断是否要继续或退出.

//枚举
//枚举是用来框定一个对象的所有属性定义, 确保程序在运行过程中涉及到该对象时, 输入输出是在定义范围内而不是超出预期之外的值.
//枚举成员是特殊的类型值
//枚举允许不它的成员不是一个int、str、String、Array或Tuple, 而可以是一个特殊的像是变量名称一样的标识.

#[derive(Debug)]        // trait
enum UserRole {         // 枚举对象
Admin,              // 枚举成员
Manager,
    Employee
}
fn main() {
    let admin = UserRole::Admin;        // 枚举可以直接赋值
    let manager = UserRole::Manager;
    let employee = UserRole::Employee;
    println!("{:?} {:?} {:?}", admin, manager, employee);
    println!("{:?}", UserRole::Admin);  // 枚举没有具体值，所以不存在所有权。
}

//枚举匹配
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Match Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    println!("{:?}", value_in_cents(Coin::Penny))
}

//枚举嵌套
#[derive(Debug)]      // trait
enum UsState {        // 枚举对象
Alabama,          // 枚举成员
Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),     // 枚举嵌套, 为Quarter这个枚举成员限定一个范围.
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 这里 state 可以随便定义，例如 abc
        Coin::Quarter(state) => {
            // 如果state变量发生变化, 引用时也要跟着对应起来.
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", value_in_cents(Coin::Nickel));
    println!("{:?}", value_in_cents(Coin::Dime));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alaska)))
}

//Some特殊枚举成员
//标准的match写法
fn main() {
    let integer_ = Some(4u8);    // Some是核心库中的范型枚举
    match integer_ {
        Some(4) => println!("four"),
        _ => println!("unknown"),     // _也是核心库中的特殊枚举(意指: 其他)
    }
}

//if let 表达式是一个match的一个缩写语法糖
fn main() {
    let integer_ = Some(4u8);
    if let Some(4) = integer_ {
        println!("four");
    }
}

//if let ... else ... 表达式
fn main() {
    let integer_ = Some(4u8);
    if let Some(6) = integer_ {    // 这种语法糖简化了代码却可能会带来一定的风险.
        println!("four")                // 需开发人员自己权衡.
    } else {
        println!("unknown")
    }
}