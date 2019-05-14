//rust--trait继承另外一个trait
fn main() {

    use std::fmt;

    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    // OutlinePrint 这个trait 继承fmt::Display, 
    // 并提供一个额外的 outline_print 功能.
    trait OutlinePrint: fmt::Display {
        // outline_print是一个默认实现功能函数.
        // 利用self数据进行二次加工.
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // 由于OutlinePrint已经默认实现了outline_print功能, 
    // 因此这里只要声明一下Point拥有这个trait即可.
    impl OutlinePrint for Point {}
    
    // 这里仍然需要声明Point拥有fmt::Display, 这是因为Rust的语法要求.
    impl fmt::Display for Point {
        
        // 为什么这里要写这个方法?
        // 那是因为fmt::Display只是定义了一个接口, 并没有实现这个方法.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 通过三面这几种定义、组合、声明, 最终完成了打印效果改装.
    let p = Point {x: 1, y: 2};
    p.outline_print();

    // output:
    // **********
    // *        *
    // * (1, 2) *
    // *        *
    // **********
}