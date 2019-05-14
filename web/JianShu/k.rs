//rust--��Ϊʲômatch�Ͳ��ܱȽ�str��String?

// Ϊʲômatch���ܱȽ� str �� String ?

// �����������, �����Ҫע�͵���������.
fn can_not_match_str_and_string() {
    let a = String::from("hello");

    match a {
        "hello" => println!("match true."),
        _ => println!("match false")
    }
    // error output:  expected struct `std::string::String`, found type `&'static str`.
    // ������Ϊmatch�ȱȽ�����, Ȼ��deref֮��, �ٱȽ�ֵ, һ����������.
}

// �����������, �����Ҫע�͵���������.
fn match_must_be_variant_of_tuple_or_struct() {
    let a = String::from("hello");

    match a {
        String::from("hello") => println!("match."),
        _ => println!("nothing")
    }

    // error_output: expected tuple struct/variant, found method `<String>::from`
    // ������Ϊmatch��pattern matchʱ, Ҫ��������ʱ��Ա����.
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

// ��ô, ���׸���αȽ� str �� String?
// ����: ʹ�� if �ж��﷨�����!
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
        // ����������õ���ջ������, ����������Ȩת������, ��Ϊ���ڲ�������copy/clone.
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_str_and_string_using_if_statement4() {
    let a = Some(String::from("hello"));
    match a {
        // ����������õ��Ƕ�������, ����Ҫ��������Ȩת������, ��˲�����ref.
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

// �ο�: https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match