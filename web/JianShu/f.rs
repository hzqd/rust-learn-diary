//rust--��δ�ӡstructʵ������?
fn example1() {
    // ��һ�ַ����Ǹ�Struct����һ��derive(Debug).
    #[derive(Debug)]
    struct MyStruct {x: i32, y: i32}
    let ms = MyStruct {x: 0, y: 10};
    println!("{:?}", ms)
}


fn example2() {
    // �ڶ��ַ������Լ�ȥʵ��һ��Display.
    // Refer: https://stackoverflow.com/questions/30253422/how-to-print-structs-and-arrays
    struct MyStruct {a: i32, b: i32}

    impl std::fmt::Display for MyStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(value a: {}, value b: {})", self.a, self.b)
        }
    }

    let ms = MyStruct { a: 0, b: 15 };
    println!("Used Display: {}", ms);
}


fn main() {
    example1();                // output: MyStruct { x: 0, y: 10 }
    example2();       // output: Used Display: (value a: 0, value b: 15)
}