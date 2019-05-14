//rust--ʲô��trait bound?
// ʲô��trait bound ?


// Ĭ�������, �����Ǳ�ʾ��������, ���Ͱ�ָ����: �޶��������͵ķ�Χ.
// ���Ͱ�������д��.


fn example1() {
    // ��һ��д������T����ָ������trait.
    // T: Fn(u32) -> u32 ��ʾָ���������ֻ������������trait.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    // ����ֻҪimpl����дһ�ΰ�, ��������Ҫ���ظ�д.
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
    // �ڶ���д����ʹ��where�ؼ���.
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