//rust--什么是生命周期?
// 
// 参考: https://doc.rust-lang.org/book/second-edition/ch10-03-lifetime-syntax.html
// 参考: https://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust


// 核心: 生命周期只跟引用/借用有关, 如果不是引用/借用, 那么就不存在生命周期的说法,
//       因为非引用/借用都必然会产生所有权转移, 所有权转移会跳出scope的限制.


// 生命周期作用在function、method、trait 和 struct 中,
// 编译器默认情况下会为每个引用类型的参数自动补充不同(为每个引用单独增加一个'a/'b/'c)的生命周期,
// 只有出现它无法识别的情况时, 才会报错并要求让你自己来填写生命周期.


// 什么情况下编译器会自动补充生命周期?
// 1. 当参数含有多个引用, 且返回值类型不是引用时, 编译器会自动为每个引用添加一个不同的声明周期.
//    fn print(status: &i32, msg: &str) -> Message {};
//    编译器自动补充生命周期
//    fn print<'a, 'b>(status: &'a i32, msg: &'b i32) -> Message {};
fn multi_reference_parameters_and_return_value_is_not_reference_type() {

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Message {
        status: i32,
        msg: String
    }

    // 不写生命周期
    #[allow(unused_variables)]
    fn print1(status: &i32, msg: &str) -> Message {
        if status == &200 {
            Message {status: 200, msg: String::from("ok!")}
        } else {
            Message {status: 404, msg: String::from("page not found!")}
        }
    };

    // 写生命周期
    #[allow(unused_variables)]
    fn print2<'a, 'b>(status: &'a i32, msg: &'b str) -> Message {
        if status == &200 {
            Message {status: 200, msg: String::from("ok!")}
        } else {
            Message {status: 404, msg: String::from("page not found!")}
        }
    }

    print1(&200, "ok!");
    print1(&404, "page not found!");

    print2(&200, "ok!");
    print2(&404, "page not found!");
}


// 2. 当参数只含有一个引用类型, 那么编译器会自动补充生命周期, 如果返回值也是一个引用类型, 那么编译器也会自动给引用类型的返回值添加生命周期.
//    fn print(msg: &str) -> &str {}
//    编译器自动补充生命周期
//    fn print<'a>(msg: &'a str) -> &'a str {};
fn only_one_reference_parameter_and_return_value_type_is_reference_too() {

    // 不写生命周期
    #[allow(unused_variables)]
    fn print1(status: i32, msg: &str) -> &str {
        if status == 200 {
            println!("{}", msg);
            msg
        } else {
            let s = "custom message";
            println!("{}", s);
            s
        }
    };

    // 写生命周期
    #[allow(unused_variables)]
    fn print2<'a>(status: i32, msg: &'a str) -> &'a str {
        if status == 200 {
            println!("{}", msg);
            msg
        } else {
            let s = "custom message";
            println!("{}", s);
            s
        }
    }

    print1(200, "ok!");
    print1(404, "page not found!");

    print2(200, "ok!");
    print2(404, "page not found!");
}


// 3. 当参数中含有 &self 或 &mut self 时, 如果返回值也是一个引用类型, 那么编译器会自动给引用类型的返回值添加生命周期.
//    struct ImportantExcerpt<'a> {
//        part: &'a str,
//    }
//    impl<'a> ImportantExcerpt<'a> {
//        fn announce_and_return_part(&self, announcement: &str) -> &str {
//            println!("Attention please: {}", announcement);
//            self.part
//        }
//    }
//    编译器自动补充生命周期
//    struct ImportantExcerpt<'a> {
//        part: &'a str,
//    }
//    impl<'a> ImportantExcerpt<'a> {
//        fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
//            println!("Attention please: {}", announcement);
//            self.part
//        }
//    }
fn lifetime_on_method() {
    struct Message<'a> {
        id: &'a str,
        msg: &'a str,
    }

    // 不写生命周期
    impl<'a> Message<'a> {
        fn get_msg_by_id1(&self, id: &str) -> &str {
            if self.id == id {
                println!("lifetime_on_method: {} {}", self.id, self.msg);
                self.msg
            } else {
                println!("lifetime_on_method: {} {}", id, self.msg);
                self.msg
            }
        }
    }
    let m = Message {id: "200", msg: "ok!"};
    m.get_msg_by_id1("200");
    m.get_msg_by_id1("404");

    // 写生命周期
    impl<'a> Message<'a> {
        fn get_msg_by_id2(&'a self, id: &'a str) -> &'a str {
            if self.id == id {
                println!("lifetime_on_method: {} {}", self.id, self.msg);
                self.msg
            } else {
                println!("lifetime_on_method: {} {}", id, self.msg);
                self.msg
            }
        }
    }

    let m = Message {id: "200", msg: "ok!"};
    m.get_msg_by_id2("200");
    m.get_msg_by_id2("404");
}


// 什么情况下编译器不会自动补充生命周期?
// 当参数含有大于一个引用类型, 并且返回值类型也是引用时, 编译器就要求必须填写完整的生命周期.
fn error_example_that_compile_not_fill_lifetime() {        // 问题代码, 需注释掉才能运行
    fn print(status: &str, msg: &str) -> &str {          // 编译器翻译成 fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &str {}
        if status == "200" {
            msg                                            // msg == &'b str        与   指定的 &str 不一致
        } else {
            status                                         // status == &'a str     与   指定的 &str 不一致
        }
    }

    print("200", "ok!");
    // error output: missing lifetime specifier, expected lifetime parameter.
    // 备注: 这个例子根本就不是比较生命周期的长短问题,
    //       而是实际返回值与指定返回值生命周期不一致的问题.
    // 推理:
    //       1. 已知编译器会自动补充生命周期:
    //          fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &str {}
    //          这是会报错, 因为编译器没有给这个引用返回值类型补充生命周期,
    //          这是因为当返回值是一个引用时, 必须包含生命周期.
    //
    //       2. 如果手动生命生命周期:
    //          fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &'a str {};
    //          这样也会报错, 这是因为逻辑代码块里面不仅仅是返回 'a 这个生命周期里面的 status 引用变量,
    //          也有可能会返回 'b 生命周期里面的 msg 引用变量, 因此与指定的返回值 -> &'a str 不相符.
    //
    //       3. 唯一的解决办法是, 认为的锁定引用的生命周期, 都在一个生命周期里面.
    //          fn print<'a>(status: &'a str, msg: &'a str) -> &'a str {};
    //          通过这种方式, 不论返回的是status还是msg都是属于 'a 生命周期,
    //          这种表达方式满足两种情况, 1. 他们的生命周期长短一致, 2. 返回值的类型与指定值的类型一致.
}


fn fix_error_example_that_compile_not_fill_lifetime() {
    fn print<'a>(status: &'a str, msg: &'a str) -> &'a str {
        if status == "200" {
            println!("{}: {}", status, msg);
            msg
        } else {
            println!("{}: {}", status, msg);
            status
        }
    }

    print("200", "ok!");
    print("404", "page not found!");
}


fn main() {
    multi_reference_parameters_and_return_value_is_not_reference_type();
    only_one_reference_parameter_and_return_value_type_is_reference_too();
    lifetime_on_method();
    // error_example_that_compile_not_fill_lifetime();
    fix_error_example_that_compile_not_fill_lifetime();
}