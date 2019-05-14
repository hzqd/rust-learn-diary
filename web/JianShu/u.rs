//rust--��������FnOnce/FnMut/Fn
// ���������е�FnOnce/FnMut/Fn


// ���� FnOnce/FnMut/Fn ��������������Ϊ Trait,
// Ĭ������������ǽ���rust������ȥ�����, ���µ�����ԭ����:
//     FnOnce: ��ָ�����Traitʱ, ���������ڷ��ʵ��ⲿ��������ӵ������Ȩ.
//     FnMut: ��ָ�����Traitʱ, �����������Ըı��ⲿ������ֵ.
//     Fn: ��ָ�����Traitʱ, ��������ֻ�ܶ�ȡ(borrow value immutably)����ֵ.


// FnOnce inline way
// �Ի�ȡ����Ȩ�ķ�ʽ����ȡ�����ڵĻ��������б���.
fn anonymous_fnonce() {
    let fn_name = "anonymous_fnonce";
    let mut b = String::from("hello");
    // ͨ��ʹ�� move �ķ�ʽ, ������Ȩת�ƽ���, rust ������
    // ���Զ����������һ�� FnOnce Trait ��������.
    let pushed_data = move || {
        // ��������Ȩת�ƽ���, ��� b �Ѿ����Ƴ���.
        // ���������������������ڱ�ִ�еڶ���.
        b.push_str(" world!");
        b
    };
    println!("{}: {}", fn_name, pushed_data());     // ����ֻ������һ��.
    // println!("{}: {}", fn_name, pushed_data());     // �ٴ����лᱨ��.
}


// FnOnce callback way
fn anonymous_fnonce_callback() {
    let fn_name = "anonymous_fnonce_callback";
    fn calculate<T>(callback: T) -> i32
        where T: FnOnce() -> String
    {
        let data = callback();
        data.len() as i32
    }

    let x = " world!";
    let mut y = String::from("hello");
    let result = calculate(|| {
        y.push_str(x);
        y
    });
    println!("{}: {}", fn_name, result);
}


// FnMut inline way
// ��mutable�ķ�ʽ��ȡ�����ڵĻ��������б���.
fn anonymous_fnmut() {
    let fn_name = "anonymous_fnmut";
    let mut b = String::from("hello");

    // rust �Զ���⵽ pushed_data �����������Ҫ�޸����ⲿ�Ļ�������.
    // ����Զ������ pushed_data ��һ�� FnMut ��������.
    let pushed_data = || {
        b.push_str(" world!");

        // ����rust�� mutable ԭ����, ֻ����һ��mut����, ��� ���� b ����
        // �ٱ�������������, ��������Ҫ���ظ��ĺ�Ľ��.
        b
    };
    let c = pushed_data();
    println!("{}: {}", fn_name, c);
}


// FnMut callback way.
fn anonymous_fnmut_callback() {
    let fn_name = "anonymous_fnmut_callback";
    fn calculate<T>(mut callback: T)
        where T: FnMut(),
    {
        callback()
    }

    let mut b = String::from("hello");
    calculate(|| {
        b.push_str(" world!");
    });
    println!("{}: {}", fn_name, b);
}


// Fn inline way
// ��immutable�ķ�ʽ��ȡ�����ڵĻ��������б���.
fn anonymous_fn() {
    let fn_name = "anonymous_fn";
    let mut a = String::from("hello");
    let b = String::from(" world!");
    let pushed_data = |x: &mut String| {
        // b �����ﱻ����, ��������ܱ���ӡ, ֤�����Ǳ�immutable����.
        x.push_str(&*b);
        println!("{}: {}", fn_name, x);
    };
    pushed_data(&mut a);
     println!("{}: {}", fn_name, b);
}


// Fn callback way
fn anonymous_fn_callback() {
    let fn_name = "anonymous_fn_callback";
    fn calculate<T>(callback: T)
        where T: Fn()
    {
        callback();
    }

    let a = String::from("hello");
    let b = String::from(" world!");
    calculate(|| {
        let joined = format!("{}{}", &*a, &*b);
        println!("{}: {}", fn_name, joined)
    })
}


fn main() {
    anonymous_fnonce();
    anonymous_fnonce_callback();
    anonymous_fnmut();
    anonymous_fnmut_callback();
    anonymous_fn();
    anonymous_fn_callback();
}