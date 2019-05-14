//rust--宏

// 宏有两种形式:
// 声明式宏: vec!、println!、write! 这些都是声明式宏.
// 过程式宏: #[derive(Debug)]、#[derive(PartialEq)] 这些都是过程式宏.

// 声明式宏
// 可以定义一种符合当前场景的数据结构, 然后使用该宏来编写rust代码.

// 过程式宏
// 主要是为结构体、元祖等数据结构增加通用的trait公共接口和公共方法.


// 声明式宏语法
macro_rules! list {
    // $x 是变量
    // :expr 是关键字语法, 表示表达式
    // * 表示零次或多次表达式匹配
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(                          
                println!("{}", $x);
                temp_vec.push($x);
            )*                          // 多次匹配会多次运行这个代码块.
            temp_vec
        }
    }
}


// 过程式宏语法
// 暂无


fn main() {
    let x = list!(1,2,3);
    println!("{:?}", x)
}
// 参考: https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html
// Rust手册: https://doc.rust-lang.org/reference/macros.html
// macros book: https://danielkeep.github.io/tlborm/book/index.html