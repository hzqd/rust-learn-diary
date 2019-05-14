//rust--什么是match?
// match是什么?
// match从字面意思来理解就是匹配, 要么匹配要么不匹配.
// 匹配有几种行为：
// 1. 有或没有.
// 2. 在不在这个范围内.  in
// 3. 是否相等.          equal
// 4. 是否为true         bool
//
// match pattern 必须是一个元祖或者结构体, 这是因为他们都有一个特性, 成员(Variant).
// 元祖: (1,2,3,4,5,6)   里面的每个元素都是一个成员(Variant).
// 结构体: Point {x: 0, y: 10} 其中 x 和 y 是Point里面的成员(Variant).
// Option: Some 和 None 两个成员都隶属于 Option.
fn raise_error_if_match_patterns_not_a_variant() {
    let a = String::from("hello");
    match a {
        String::from("hello") => println!("match true"),
        _ => println!("nothing")
    }

    // error output: not a tuple variant or struct
}

fn tuple_match_is_ok() {
    let a = 3;
    match a {
        1...5 => println!("tuple_match_is_ok: hit"),
        _ => println!("nothing")
    }
}

fn struct_match_is_ok() {
    struct Point {x: i32, y: i32}
    let p = Point {x: 10, y: 20};
    match p {
        Point {x: 10, ..} => println!("struct_match_is_ok: x: hit"),
        Point {y: 20, ..} => println!("struct_match_is_ok: y: hit"),
        _ => println!("nothing")
    }
}

fn option_match_is_ok() {
    let a = Some(String::from("hello"));
    match a {
        Some(ssss) => println!("option_match_is_ok: some: {}", ssss),
        None => println!("nothing")
    }
}

// match pattern 额外支持判断条件, 这种情况下不要求每个对象必须是成员(Variant).
#[allow(dead_code)]
#[allow(unused_variables)]
fn raise_error_in_match_patterns_using_outer_variable() {
    let a = "hello";
    let b = Some(String::from("hello"));
    match b {
        Some(a) => println!("raise_error_using_outer_variable: a: {}", a),
        None => println!("nothing")
    }
    // error output: let a = "hello" never used, consider using `_a` instead.
}

fn match_patterns_using_outer_variable_with_if_statement() {
    let a = "hello";
    let b = String::from("hello");
    match b {
        _ if a == b => println!("using_outer_variable: {}", b),
        _ => println!("nothing")
    }
}


fn main() {
    // raise_error_if_match_patterns_not_a_variant();
    tuple_match_is_ok();
    struct_match_is_ok();
    option_match_is_ok();
    // raise_error_in_match_patterns_using_outer_variable();
    match_patterns_using_outer_variable_with_if_statement()
}