//rust--用if判断str和String为什么返回true?
// 为什么断言str和String是True?
fn asserteq_str_and_string_is_true_problem() {
    let a = "hello";
    let b = String::from("hello");
    assert_eq!(a, b, "assert error.")
}

// 为什么if判断str和Stringi是True?
fn if_statement_compare_str_and_string_is_true_problem() {
    let a = "hello";
    let b = String::from("hello");
    if a == b {
        println!("compare true.")
    } else {
        println!("compare false")
    }
    // output: compare true.
}

// 这两个问题本质上是一个问题, 因为所有的assert、while内部也是if来验证逻辑是否成立.
// if条件句利用了PartialEq trait 通用接口来进行判断, 恰巧String 实现了 PartialEq<str>
// 和 PartialEq<&str>, 所以用 &str 和 String 来比较是能够完成常规判断操作的.

fn main() {
    asserteq_str_and_string_is_true_problem();
    if_statement_compare_str_and_string_is_true_problem();
}

// 参考: https://github.com/rust-lang/rust/issues/41254