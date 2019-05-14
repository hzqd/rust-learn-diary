//rust--泛型trait和trait的区别?
// trait 关联类型  vs  trait 泛型

// 参考: https://stackoverflow.com/questions/41669634/implementing-a-generic-incrementable-trait-in-rust/41671697
// 参考: https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html
fn trait_generic() {

    trait Iterator<T> {                               // trait 泛型
        fn next(&mut self) -> Option<T>;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            self.count += 1;
            println!("u32");
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    impl Iterator<i32> for Counter {
        fn next(&mut self) -> Option<i32> {
            self.count += 1;
            println!("i32");
            if self.count < 6 {
                let value: i32 = self.count as i32;
                Some(value)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1u32));    // u32 进入 impl Iterator<u32> for Counter
    assert_eq!(counter.next(), Some(2u32));
    assert_eq!(counter.next(), Some(3));       // i32  进入 impl Iterator<i32> for Counter
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    // assert_eq!(counter.next(), None);       // TODO: 这里如果不注释掉, 会报错, 如何解决?

    // output:
    // u32
    // u32
    // i32
    // i32
    // i32

    // 总结:
    // 普通trait只允许定义一次方法名, 多次定义会报错.
    // 泛型trait允许为每个类型定义一次方法名, 也就是说可以定义多个impl for;
    // 泛型类型的trait, 提供了一种更灵活的写法, 但是要为每个类型都去封装相同的方法(做不同的事情).
    // 写一般的代码不需要这么灵活, 除非写核心代码, 或者写核心库才会用上把.
}


fn main() {
    trait_generic();
}