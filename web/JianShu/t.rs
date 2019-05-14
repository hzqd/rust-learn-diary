//rust--ʲô����������?

// 1. �����������Ի�ȡ������������������б�����������.
// 2. ��Щ���ÿ����ǲ��ɱ�����Ҳ�����ǿɱ�����, ����ǿɱ���������Ҫ�ڱ�����������������ǰ������mut.


// ��ͨ�������ܻ�ȡ�����ڵ�scope���κα���.
// ��ע: �������, ��ע�͵������������������ļ�.
fn normal_function_can_not_outside_scope_variables() {
    let fn_name = "normal_function_can_not_outside_scope_variables";
    let b = "hello";
    fn plus_one(x: i32) -> i32 {
        println!("{}: {}", fn_name, b);    // �ں����ж�ȡ�ⲿ����, �ǲ����е�, ��˻ᱨ��.
        x + 1
    }

    println!("{}: {}", fn_name, plus_one(10));
}


// �������������ȡ�����ڵ�scope�����б���.
fn anonymous_function_can_get_outside_scope_variables() {
    let fn_name = "anonymous_function_can_get_outside_scope_variables";
    let b = String::from("hello");
    let plus_one = |x| {
        println!("{}: {}", fn_name, b);      // �����������ж�ȡ�ⲿ����, �ǿ��е�.
        x + 1
    };

    println!("{}: {}: {}", fn_name, plus_one(10), b);
}


fn anonymous_function_change_outside_scope_variables() {
    let fn_name = "anonymous_function_change_outside_scope_variables";
    let mut b = String::from("hello");
    let plus_one = || {
        b.push_str(" world!");      // �����������и����ⲿ����, �ǿ��е�.
        b
    };
    let c = plus_one();
    println!("{}: {}", fn_name, c);
}


fn main() {
    // normal_function_can_not_outside_scope_variables();
    anonymous_function_can_get_outside_scope_variables();
    anonymous_function_change_outside_scope_variables();
}