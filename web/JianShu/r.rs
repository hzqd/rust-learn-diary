//rust--trait�̳�����һ��trait
fn main() {

    use std::fmt;

    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    // OutlinePrint ���trait �̳�fmt::Display, 
    // ���ṩһ������� outline_print ����.
    trait OutlinePrint: fmt::Display {
        // outline_print��һ��Ĭ��ʵ�ֹ��ܺ���.
        // ����self���ݽ��ж��μӹ�.
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // ����OutlinePrint�Ѿ�Ĭ��ʵ����outline_print����, 
    // �������ֻҪ����һ��Pointӵ�����trait����.
    impl OutlinePrint for Point {}
    
    // ������Ȼ��Ҫ����Pointӵ��fmt::Display, ������ΪRust���﷨Ҫ��.
    impl fmt::Display for Point {
        
        // Ϊʲô����Ҫд�������?
        // ������Ϊfmt::Displayֻ�Ƕ�����һ���ӿ�, ��û��ʵ���������.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // ͨ�������⼸�ֶ��塢��ϡ�����, ��������˴�ӡЧ����װ.
    let p = Point {x: 1, y: 2};
    p.outline_print();

    // output:
    // **********
    // *        *
    // * (1, 2) *
    // *        *
    // **********
}