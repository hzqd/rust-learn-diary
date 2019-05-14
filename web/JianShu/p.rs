//rust--����trait��trait������?
// trait ��������  vs  trait ����

// �ο�: https://stackoverflow.com/questions/41669634/implementing-a-generic-incrementable-trait-in-rust/41671697
// �ο�: https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html
fn trait_generic() {

    trait Iterator<T> {                               // trait ����
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
    assert_eq!(counter.next(), Some(1u32));    // u32 ���� impl Iterator<u32> for Counter
    assert_eq!(counter.next(), Some(2u32));
    assert_eq!(counter.next(), Some(3));       // i32  ���� impl Iterator<i32> for Counter
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    // assert_eq!(counter.next(), None);       // TODO: ���������ע�͵�, �ᱨ��, ��ν��?

    // output:
    // u32
    // u32
    // i32
    // i32
    // i32

    // �ܽ�:
    // ��ͨtraitֻ������һ�η�����, ��ζ���ᱨ��.
    // ����trait����Ϊÿ�����Ͷ���һ�η�����, Ҳ����˵���Զ�����impl for;
    // �������͵�trait, �ṩ��һ�ָ�����д��, ����ҪΪÿ�����Ͷ�ȥ��װ��ͬ�ķ���(����ͬ������).
    // дһ��Ĵ��벻��Ҫ��ô���, ����д���Ĵ���, ����д���Ŀ�Ż����ϰ�.
}


fn main() {
    trait_generic();
}