//rust--reference��borrow������?

// reference��borrow��ʲô����?
//
// reference��borrow��ʵ��һ������, ���Ǳ���ֳ���������,
// borrow��ָ�������бر��Ĳ�������, ��Ҫ�����ⲿ�ṩ, ���ֲ��������ͽ���borrow(����).


fn reference_example() {
    let a = String::from("reference hello");
    let b = &a;                             // ��������: reference.
    println!("a: {}, b: {}", a, b)
}


fn borrow_example() {
    let a = String::from("borrow hello");

    fn print(msg: &String) {                              // ���ǽ���: borrow
        println!("{:?}", msg)
    }

    print(&a)
}


fn main() {
    reference_example();
    borrow_example();
}