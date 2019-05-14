//rust--trait��Ĭ�Ϸ��Ͳ���
//
// �﷨��ʽ
// trait FlyInterface<RHS=Self> {                   // ���ﶨ����һ��Ĭ�Ϸ�������
//     fn flap_wings(&self, rhs; RHS) -> &str;      // ����ʹ����Ĭ�Ϸ�������
// }
//
// ��Ȼ˵�Ƿ���, ��ô���ı��ʾ��ǿ��Խ�����������, ֻ����Ĭ����Self����.
// ���ⲹ��: ��Ȼflap_wings��������һ��rhs������, ��������ʵ�ֵ�ʱ��,
//           �����Ǳ���ý�rhs������, rustֻҪ������һ�¾�����.


fn example1() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    // Add�Ǳ�׼���е�trait, ���� Add �� <RHS=Self> �Ƿ��Ͳ���, ���
    // �������������ʵ��ʱ, ������д��������������ǵ�Ĭ�����Ͳ���.
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        // ֻ��Ҫȷ����������Ͳ����븲�ǲ���һ�¼���
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

    // ���� ABC �� <RHS=Self>�Ƿ��Ͳ���, ���
    // �������������ʵ��ʱ, ������д��������������ǵ�Ĭ�����Ͳ���.
    impl ABC<u32> for Point {
        type Item = i32;

        // ֻ��Ҫȷ����������Ͳ����븲�ǲ���һ�¼���
        fn next(&self, sd: u32) -> String {
            {sd};  // ���д���û���κ�����.
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