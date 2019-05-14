//rust--什么是匿名函数?

// 1. 匿名函数可以获取到其所在作用域的所有变量名的引用.
// 2. 这些引用可以是不可变引用也可以是可变引用, 如果是可变引用则需要在变量名和匿名函数名前面声明mut.


// 普通函数不能获取其所在的scope的任何变量.
// 备注: 错误代码, 需注释掉才能运行整个代码文件.
fn normal_function_can_not_outside_scope_variables() {
    let fn_name = "normal_function_can_not_outside_scope_variables";
    let b = "hello";
    fn plus_one(x: i32) -> i32 {
        println!("{}: {}", fn_name, b);    // 在函数中读取外部变量, 是不可行的, 因此会报错.
        x + 1
    }

    println!("{}: {}", fn_name, plus_one(10));
}


// 匿名函数允许获取其所在的scope的所有变量.
fn anonymous_function_can_get_outside_scope_variables() {
    let fn_name = "anonymous_function_can_get_outside_scope_variables";
    let b = String::from("hello");
    let plus_one = |x| {
        println!("{}: {}", fn_name, b);      // 在匿名函数中读取外部变量, 是可行的.
        x + 1
    };

    println!("{}: {}: {}", fn_name, plus_one(10), b);
}


fn anonymous_function_change_outside_scope_variables() {
    let fn_name = "anonymous_function_change_outside_scope_variables";
    let mut b = String::from("hello");
    let plus_one = || {
        b.push_str(" world!");      // 在匿名函数中更改外部变量, 是可行的.
        b
    };
    let c = plus_one();
    println!("{}: {}", fn_name, c);
}


fn main() {
    // normal_function_can_not_outside_scope_variables();
    anonymous_function_can_get_outside_scope_variables();
    anonymous_function_change_outside_scope_variables();
}