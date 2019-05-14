//rust--as_ref和borrow的区别

// as_ref 是转引用函数, 将具有所有权对象转换成引用对象,
// 不改变被转换对象的基础上产生一个引用对象.

// as_ref 并不是所有类型都默认支持, 很多时候都需要自己去声明.
// as_ref 是AsRef trait 的公共接口方法.
// 只有那些实现了 as_ref 公共接口方法的类型才能使用as_ref.
// 目前: Option, Box, Result 这三种类型默认提供支持as_ref.

// as_ref 和 Borrow 的区别是:
// 基础数据类型引用:
//     Borrow 可以直接在 int, &str, String, vec, [], struct, enum 这种类型上直接指定&来引用.
//     as_ref 则不行, 它需要声明泛型T: AsRef<int>, T: AsRef<str>, T: AsRef<struct name> 来支持.
// 嵌套数据类型引用: Some(&int) , Box(&int) ,
//     Borrow 必须在定义结构时声明 Some<&int> , Box<&int> 才是引用.
//     as_ref 则直接可以在这些嵌套结构上使用as_ref.
// 引用的引用
//     Borrow 引用的引用的表现形式是:   &str -> &&str
//     as_ref 引用的引用的表现形式是:   &str -> &str


fn borrow_example() {
    let s = 1;
    let x = &s;                                                                // 直接引用
    println!("s:{}; x: {}", s, x);
}


fn borrow_nest_example() {

    fn hello(x: Option<&i32>) -> Option<&i32> {                                          // 指定Some<&i32>
        match x {
            Some(_item) => x,
            None => None
        }
    }

    let s = 1234;
    let z = hello(Some(&s));                                                   // 传入之前要先把引用声明好.
    println!("s: {};  z: {:?}", s, z);
}


#[allow(dead_code)]
#[allow(unused_variables)]
fn borrow_reference_to_reference() {
    let a: &str = "str";
    let b: &&str = &a;
}


fn as_ref_example() {

    // as_ref 在这种场景的使用上不如 borrow,
    // 这是因为这种写法要求把所有权转移进来.
    // 又不能把&str 返回回去, 因为生命周期会冲突.
    // 所以as_ref不建议在这种场景下使用.

    fn hello<T: AsRef<str>>(x: T) {
        let xx = x.as_ref();
        println!("xx: {}", xx);
    }
    let s = String::from("hello");
    hello(s);
}


fn as_ref_nest_example() {

    // as_ref 非常适合这种场景, 简单快捷.

    fn hello(x: Option<i32>) -> Option<i32> {
        x.as_ref();                                                            // Option<i32>  to  Option<&i32> 很方便后续代码的编写.
        x
    }

    let s = 1234;
    let z = hello(Some(s));
    println!("s: {}; z: {:?}", s, z);
}


fn as_ref_reference_to_reference() {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn hello<T: AsRef<str>>(x: T) {
        let y: &str = x.as_ref();
        let z: &str = y.as_ref();                   // 引用上再引用, 永远只有一层引用.
    }

    let s = "hello";
    hello(s);
}


fn main() {
    borrow_example();
    borrow_nest_example();
    borrow_reference_to_reference();
    as_ref_example();
    as_ref_nest_example();
    as_ref_reference_to_reference();
}