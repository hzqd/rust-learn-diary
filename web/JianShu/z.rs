//rust--as_ref��borrow������

// as_ref ��ת���ú���, ����������Ȩ����ת�������ö���,
// ���ı䱻ת������Ļ����ϲ���һ�����ö���.

// as_ref �������������Ͷ�Ĭ��֧��, �ܶ�ʱ����Ҫ�Լ�ȥ����.
// as_ref ��AsRef trait �Ĺ����ӿڷ���.
// ֻ����Щʵ���� as_ref �����ӿڷ��������Ͳ���ʹ��as_ref.
// Ŀǰ: Option, Box, Result ����������Ĭ���ṩ֧��as_ref.

// as_ref �� Borrow ��������:
// ����������������:
//     Borrow ����ֱ���� int, &str, String, vec, [], struct, enum ����������ֱ��ָ��&������.
//     as_ref ����, ����Ҫ��������T: AsRef<int>, T: AsRef<str>, T: AsRef<struct name> ��֧��.
// Ƕ��������������: Some(&int) , Box(&int) ,
//     Borrow �����ڶ���ṹʱ���� Some<&int> , Box<&int> ��������.
//     as_ref ��ֱ�ӿ�������ЩǶ�׽ṹ��ʹ��as_ref.
// ���õ�����
//     Borrow ���õ����õı�����ʽ��:   &str -> &&str
//     as_ref ���õ����õı�����ʽ��:   &str -> &str


fn borrow_example() {
    let s = 1;
    let x = &s;                                                                // ֱ������
    println!("s:{}; x: {}", s, x);
}


fn borrow_nest_example() {

    fn hello(x: Option<&i32>) -> Option<&i32> {                                          // ָ��Some<&i32>
        match x {
            Some(_item) => x,
            None => None
        }
    }

    let s = 1234;
    let z = hello(Some(&s));                                                   // ����֮ǰҪ�Ȱ�����������.
    println!("s: {};  z: {:?}", s, z);
}


#[allow(dead_code)]
#[allow(unused_variables)]
fn borrow_reference_to_reference() {
    let a: &str = "str";
    let b: &&str = &a;
}


fn as_ref_example() {

    // as_ref �����ֳ�����ʹ���ϲ��� borrow,
    // ������Ϊ����д��Ҫ�������Ȩת�ƽ���.
    // �ֲ��ܰ�&str ���ػ�ȥ, ��Ϊ�������ڻ��ͻ.
    // ����as_ref�����������ֳ�����ʹ��.

    fn hello<T: AsRef<str>>(x: T) {
        let xx = x.as_ref();
        println!("xx: {}", xx);
    }
    let s = String::from("hello");
    hello(s);
}


fn as_ref_nest_example() {

    // as_ref �ǳ��ʺ����ֳ���, �򵥿��.

    fn hello(x: Option<i32>) -> Option<i32> {
        x.as_ref();                                                            // Option<i32>  to  Option<&i32> �ܷ����������ı�д.
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
        let z: &str = y.as_ref();                   // ������������, ��Զֻ��һ������.
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