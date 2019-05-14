//rust--问号操作符
// 什么是问号操作符?
// 参考: https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html
// 参考: https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about


// 由于Rust中没有Exception异常处理的语法,
// Rust只有panic报错, 并且panic不允许被保护, 因为没有提供 try 这种语法.
// Rust的异常处理是通过 Result 的 Ok 和 Err 成员来传递和包裹错误信息.
// 然而错误信息的处理一般都是要通过match来对类型进行比较, 所以很多时候
// 代码比较冗余, 通过?符号来简化Ok和Err的判断.


// 下面的例子提供了一个不使用?符号 以及 一个使用?符号的样例代码.
fn halves_if_even<'a >(i: i32) -> Result<i32, &'a str> {                       // 取数值的二分之一.
    if i % 2 == 0 {
        Ok(i/2)
    } else {
        Err("error")
    }
}

fn not_use_question_mark() {
    let a = 10;                                                                // 把这里改成 9 就会报错.
    let half = halves_if_even(a);
    let half = match half {
        Ok(item) => item,
        Err(e) => panic!(e),
    };
    assert_eq!(half, 5);
}


fn use_question_mark<'a >() -> Result<i32, &'a str> {                          // 这里必须要返回Result
    let a = 10;
    let half = halves_if_even(a)?;                                             // 因为?要求其所在的函数必须要返回Result
    assert_eq!(half, 5);
    Ok(half)                                                                   
}


fn main() {
    not_use_question_mark();
    let _ = use_question_mark();
}