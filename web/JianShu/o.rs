//rust--ʲô��trait?
// 
// trait ��Ӧ��������е�<�ӿ�>, ͨ������һ�������ӿ�,
// struct �� method ��������ʵ���Լ��ķ���, ����ʵ�������̬.


// ����������ǲ����˱�׼���е�Iterator�������.zip().map().filter().sum()����.
// ͬʱҲͨ���Լ�ʵ��next����ɵ�������.
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

    // �������Լ�ʵ�ֵ�Iterator, �⽫�Ḳ�ǵ���׼���е�Iterator.
    // ��ᵼ��ԭ����׼���Iterator�ṩ�������������ܶ�������.
    // ����������Щ�������޷����е�.
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