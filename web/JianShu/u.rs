//rust--匿名函数FnOnce/FnMut/Fn
// 匿名函数中的FnOnce/FnMut/Fn


// 首先 FnOnce/FnMut/Fn 这三个东西被称为 Trait,
// 默认情况下它们是交给rust编译器去推理的, 大致的推理原则是:
//     FnOnce: 当指定这个Trait时, 匿名函数内访问的外部变量必须拥有所有权.
//     FnMut: 当指定这个Trait时, 匿名函数可以改变外部变量的值.
//     Fn: 当指定这个Trait时, 匿名函数只能读取(borrow value immutably)变量值.


// FnOnce inline way
// 以获取所有权的方式来获取其所在的环境的所有变量.
fn anonymous_fnonce() {
    let fn_name = "anonymous_fnonce";
    let mut b = String::from("hello");
    // 通过使用 move 的方式, 把所有权转移进来, rust 编译器
    // 会自动推理出这是一个 FnOnce Trait 匿名函数.
    let pushed_data = move || {
        // 由于所有权转移进来, 因此 b 已经被移除掉.
        // 因此这个匿名函数不可能在被执行第二遍.
        b.push_str(" world!");
        b
    };
    println!("{}: {}", fn_name, pushed_data());     // 这里只能运行一次.
    // println!("{}: {}", fn_name, pushed_data());     // 再次运行会报错.
}


// FnOnce callback way
fn anonymous_fnonce_callback() {
    let fn_name = "anonymous_fnonce_callback";
    fn calculate<T>(callback: T) -> i32
        where T: FnOnce() -> String
    {
        let data = callback();
        data.len() as i32
    }

    let x = " world!";
    let mut y = String::from("hello");
    let result = calculate(|| {
        y.push_str(x);
        y
    });
    println!("{}: {}", fn_name, result);
}


// FnMut inline way
// 以mutable的方式获取其所在的环境的所有变量.
fn anonymous_fnmut() {
    let fn_name = "anonymous_fnmut";
    let mut b = String::from("hello");

    // rust 自动检测到 pushed_data 这个匿名函数要修改其外部的环境变量.
    // 因此自动推理出 pushed_data 是一个 FnMut 匿名函数.
    let pushed_data = || {
        b.push_str(" world!");

        // 由于rust的 mutable 原则是, 只允许一个mut引用, 因此 变量 b 不能
        // 再被其他代码引用, 所以这里要返回更改后的结果.
        b
    };
    let c = pushed_data();
    println!("{}: {}", fn_name, c);
}


// FnMut callback way.
fn anonymous_fnmut_callback() {
    let fn_name = "anonymous_fnmut_callback";
    fn calculate<T>(mut callback: T)
        where T: FnMut(),
    {
        callback()
    }

    let mut b = String::from("hello");
    calculate(|| {
        b.push_str(" world!");
    });
    println!("{}: {}", fn_name, b);
}


// Fn inline way
// 以immutable的方式获取其所在的环境的所有变量.
fn anonymous_fn() {
    let fn_name = "anonymous_fn";
    let mut a = String::from("hello");
    let b = String::from(" world!");
    let pushed_data = |x: &mut String| {
        // b 再这里被引用, 但是最后还能被打印, 证明它是被immutable引用.
        x.push_str(&*b);
        println!("{}: {}", fn_name, x);
    };
    pushed_data(&mut a);
     println!("{}: {}", fn_name, b);
}


// Fn callback way
fn anonymous_fn_callback() {
    let fn_name = "anonymous_fn_callback";
    fn calculate<T>(callback: T)
        where T: Fn()
    {
        callback();
    }

    let a = String::from("hello");
    let b = String::from(" world!");
    calculate(|| {
        let joined = format!("{}{}", &*a, &*b);
        println!("{}: {}", fn_name, joined)
    })
}


fn main() {
    anonymous_fnonce();
    anonymous_fnonce_callback();
    anonymous_fnmut();
    anonymous_fnmut_callback();
    anonymous_fn();
    anonymous_fn_callback();
}