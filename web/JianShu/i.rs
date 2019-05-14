//rust--const和static变量的区别?
// 静态变量于常量变量的区别.
// 参考: https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable
// 参考: https://doc.rust-lang.org/book/first-edition/const-and-static.html

// 摘要
// Constants and immutable static variables might seem similar,
// but a subtle difference is that values in a static variable have a fixed address in memory.
// Using the value will always access the same data. Constants,
// on the other hand, are allowed to duplicate their data whenever they’re used.

// 静态变量是固定的内存地址, 因此它可以在unsafe代码块里面被改变.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 常量没有固定的内存地址,
// 编译器并不是在读取到定义常量时创建内存来存放, 而是读取到被调用时才会创建.
// 参考: https://stackoverflow.com/questions/40148175/what-does-it-mean-for-a-const-type-in-rust-to-be-inlined