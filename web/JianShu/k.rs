//rust--那为什么match就不能比较str和String?

// 为什么match不能比较 str 和 String ?

// 这是问题代码, 因此需要注释掉才能运行.
fn can_not_match_str_and_string() {
    let a = String::from("hello");

    match a {
        "hello" => println!("match true."),
        _ => println!("match false")
    }
    // error output:  expected struct `std::string::String`, found type `&'static str`.
    // 这是因为match先比较类型, 然后deref之后, 再比较值, 一共三个步骤.
}

// 这是问题代码, 因此需要注释掉才能运行.
fn match_must_be_variant_of_tuple_or_struct() {
    let a = String::from("hello");

    match a {
        String::from("hello") => println!("match."),
        _ => println!("nothing")
    }

    // error_output: expected tuple struct/variant, found method `<String>::from`
    // 这是因为match做pattern match时, 要求对象必须时成员对象.
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

// 那么, 到底该如何比较 str 和 String?
// 答案是: 使用 if 判断语法来完成!
fn match_str_and_string_using_if_statement() {
    let a = String::from("hello");
    match a == "hello" {                // if statement
        true => println!("match."),
        _ => println!("nothing")
    }
    // output: match.
}

fn match_str_and_string_using_if_statement2() {
    let a = String::from("hello");
    match a {
        _ if a == "hello" => println!("match."),
        _ => println!("nothing")
    }
}

fn match_str_and_string_using_if_statement3() {
    let num = Some(4);
    match num {
        // 由于这里采用的是栈上数据, 不存在所有权转移问题, 因为它内部是做了copy/clone.
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_str_and_string_using_if_statement4() {
    let a = Some(String::from("hello"));
    match a {
        // 由于这里采用的是堆上数据, 所以要考虑所有权转移问题, 因此采用了ref.
        Some(ref x) if x == "hello" => println!("matched: {}", x),
        Some(_x) => println!("unkown"),
        None => println!("nothing")
    }
}


fn main() {
    // can_not_match_str_and_string();
    // match_must_be_variant_of_tuple_or_struct();
    tuple_match_is_ok();
    struct_match_is_ok();
    match_str_and_string_using_if_statement();
    match_str_and_string_using_if_statement2();
    match_str_and_string_using_if_statement3();
    match_str_and_string_using_if_statement4();
    match_str_and_string_using_if_statement5();
}

// 参考: https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match