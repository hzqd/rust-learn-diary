//rust--��if�ж�str��StringΪʲô����true?
// Ϊʲô����str��String��True?
fn asserteq_str_and_string_is_true_problem() {
    let a = "hello";
    let b = String::from("hello");
    assert_eq!(a, b, "assert error.")
}

// Ϊʲôif�ж�str��Stringi��True?
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

// ���������Ȿ������һ������, ��Ϊ���е�assert��while�ڲ�Ҳ��if����֤�߼��Ƿ����.
// if������������PartialEq trait ͨ�ýӿ��������ж�, ǡ��String ʵ���� PartialEq<str>
// �� PartialEq<&str>, ������ &str �� String ���Ƚ����ܹ���ɳ����жϲ�����.

fn main() {
    asserteq_str_and_string_is_true_problem();
    if_statement_compare_str_and_string_is_true_problem();
}

// �ο�: https://github.com/rust-lang/rust/issues/41254