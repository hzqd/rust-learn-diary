//rust--什么是trait?
// 
// trait 对应面向对象中的<接口>, 通过定义一个公共接口,
// struct 的 method 可以自行实现自己的方法, 最终实现整体多态.


// 下面的例子是采用了标准库中的Iterator来完成了.zip().map().filter().sum()操作.
// 同时也通过自己实现next来完成迭代过程.
fn use_standard_library_trait() {

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}


fn implement_self_trait() {

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 由于是自己实现的Iterator, 这将会覆盖掉标准库中的Iterator.
    // 这会导致原本标准库的Iterator提供的所有其他功能都不可用.
    // 因此下面的这些代码是无法运行的.
    // let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    //                              .map(|(a, b)| a * b)
    //                              .filter(|x| x % 3 == 0)
    //                              .sum();
    // assert_eq!(18, sum);
}


fn main() {
    use_standard_library_trait();
    implement_self_trait();
}