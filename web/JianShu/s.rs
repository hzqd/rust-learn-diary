//rust--什么是trait bound?
// 什么是trait bound ?


// 默认情况下, 泛型是表示所有类型, 泛型绑定指的是: 限定泛型类型的范围.
// 泛型绑定有两种写法.


fn example1() {
    // 第一种写法是再T后面指定限制trait.
    // T: Fn(u32) -> u32 表示指定这个泛型只接受匿名函数trait.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    // 这里只要impl后面写一次绑定, 后面则不需要再重复写.
    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(15));
}


fn example2() {
    // 第而种写法是使用where关键字.
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    println!("{}", cacher.value(20));
    println!("{}", cacher.value(25));
}



fn main() {
    example1();
    example2();
}