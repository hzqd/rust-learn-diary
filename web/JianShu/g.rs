//rust--ʲô��match?
// match��ʲô?
// match��������˼��������ƥ��, Ҫôƥ��Ҫô��ƥ��.
// ƥ���м�����Ϊ��
// 1. �л�û��.
// 2. �ڲ��������Χ��.  in
// 3. �Ƿ����.          equal
// 4. �Ƿ�Ϊtrue         bool
//
// match pattern ������һ��Ԫ����߽ṹ��, ������Ϊ���Ƕ���һ������, ��Ա(Variant).
// Ԫ��: (1,2,3,4,5,6)   �����ÿ��Ԫ�ض���һ����Ա(Variant).
// �ṹ��: Point {x: 0, y: 10} ���� x �� y ��Point����ĳ�Ա(Variant).
// Option: Some �� None ������Ա�������� Option.
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

// match pattern ����֧���ж�����, ��������²�Ҫ��ÿ����������ǳ�Ա(Variant).
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