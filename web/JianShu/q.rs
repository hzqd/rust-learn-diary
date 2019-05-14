//rust--trait的默认泛型参数
//
// 语法形式
// trait FlyInterface<RHS=Self> {                   // 这里定义了一个默认泛型类型
//     fn flap_wings(&self, rhs; RHS) -> &str;      // 这里使用了默认泛型类型
// }
//
// 既然说是泛型, 那么它的本质就是可以接受任意类型, 只不过默认是Self自身.
// 额外补充: 虽然flap_wings方法顶了一个rhs参数名, 但是在做实现的时候,
//           并不是必须得叫rhs参数名, rust只要求数量一致就行了.


fn example1() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    // Add是标准库中的trait, 由于 Add 的 <RHS=Self> 是泛型参数, 因此
    // 在这里做具体得实现时, 可以填写具体的类型来覆盖掉默认类型参数.
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        // 只需要确认这里的类型参数与覆盖参数一致即可
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let mm = Millimeters(15);
    let m = Meters(20);
    println!("{:?}", mm + m);
}


fn example2() {

    trait ABC<RHS=Self> {
        type Item;
        fn next(&self, rhs: RHS) -> String;
    }

    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32
    }

    // 由于 ABC 的 <RHS=Self>是泛型参数, 因此
    // 在这里做具体得实现时, 可以填写具体的类型来覆盖掉默认类型参数.
    impl ABC<u32> for Point {
        type Item = i32;

        // 只需要确认这里的类型参数与覆盖参数一致即可
        fn next(&self, sd: u32) -> String {
            {sd};  // 这行代码没有任何意义.
            String::from("ok")
        }
    }


    let u = Point {x: 1, y: 2};

    println!("{:?}", u.next(10));
}


fn main() {
    example1();
    example2();
}