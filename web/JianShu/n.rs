//rust--ʲô����������?
// 
// �ο�: https://doc.rust-lang.org/book/second-edition/ch10-03-lifetime-syntax.html
// �ο�: https://stackoverflow.com/questions/31609137/why-are-explicit-lifetimes-needed-in-rust


// ����: ��������ֻ������/�����й�, �����������/����, ��ô�Ͳ������������ڵ�˵��,
//       ��Ϊ������/���ö���Ȼ���������Ȩת��, ����Ȩת�ƻ�����scope������.


// ��������������function��method��trait �� struct ��,
// ������Ĭ������»�Ϊÿ���������͵Ĳ����Զ����䲻ͬ(Ϊÿ�����õ�������һ��'a/'b/'c)����������,
// ֻ�г������޷�ʶ������ʱ, �Żᱨ��Ҫ�������Լ�����д��������.


// ʲô����±��������Զ�������������?
// 1. ���������ж������, �ҷ���ֵ���Ͳ�������ʱ, ���������Զ�Ϊÿ���������һ����ͬ����������.
//    fn print(status: &i32, msg: &str) -> Message {};
//    �������Զ�������������
//    fn print<'a, 'b>(status: &'a i32, msg: &'b i32) -> Message {};
fn multi_reference_parameters_and_return_value_is_not_reference_type() {

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Message {
        status: i32,
        msg: String
    }

    // ��д��������
    #[allow(unused_variables)]
    fn print1(status: &i32, msg: &str) -> Message {
        if status == &200 {
            Message {status: 200, msg: String::from("ok!")}
        } else {
            Message {status: 404, msg: String::from("page not found!")}
        }
    };

    // д��������
    #[allow(unused_variables)]
    fn print2<'a, 'b>(status: &'a i32, msg: &'b str) -> Message {
        if status == &200 {
            Message {status: 200, msg: String::from("ok!")}
        } else {
            Message {status: 404, msg: String::from("page not found!")}
        }
    }

    print1(&200, "ok!");
    print1(&404, "page not found!");

    print2(&200, "ok!");
    print2(&404, "page not found!");
}


// 2. ������ֻ����һ����������, ��ô���������Զ�������������, �������ֵҲ��һ����������, ��ô������Ҳ���Զ����������͵ķ���ֵ�����������.
//    fn print(msg: &str) -> &str {}
//    �������Զ�������������
//    fn print<'a>(msg: &'a str) -> &'a str {};
fn only_one_reference_parameter_and_return_value_type_is_reference_too() {

    // ��д��������
    #[allow(unused_variables)]
    fn print1(status: i32, msg: &str) -> &str {
        if status == 200 {
            println!("{}", msg);
            msg
        } else {
            let s = "custom message";
            println!("{}", s);
            s
        }
    };

    // д��������
    #[allow(unused_variables)]
    fn print2<'a>(status: i32, msg: &'a str) -> &'a str {
        if status == 200 {
            println!("{}", msg);
            msg
        } else {
            let s = "custom message";
            println!("{}", s);
            s
        }
    }

    print1(200, "ok!");
    print1(404, "page not found!");

    print2(200, "ok!");
    print2(404, "page not found!");
}


// 3. �������к��� &self �� &mut self ʱ, �������ֵҲ��һ����������, ��ô���������Զ����������͵ķ���ֵ�����������.
//    struct ImportantExcerpt<'a> {
//        part: &'a str,
//    }
//    impl<'a> ImportantExcerpt<'a> {
//        fn announce_and_return_part(&self, announcement: &str) -> &str {
//            println!("Attention please: {}", announcement);
//            self.part
//        }
//    }
//    �������Զ�������������
//    struct ImportantExcerpt<'a> {
//        part: &'a str,
//    }
//    impl<'a> ImportantExcerpt<'a> {
//        fn announce_and_return_part(&'a self, announcement: &'a str) -> &'a str {
//            println!("Attention please: {}", announcement);
//            self.part
//        }
//    }
fn lifetime_on_method() {
    struct Message<'a> {
        id: &'a str,
        msg: &'a str,
    }

    // ��д��������
    impl<'a> Message<'a> {
        fn get_msg_by_id1(&self, id: &str) -> &str {
            if self.id == id {
                println!("lifetime_on_method: {} {}", self.id, self.msg);
                self.msg
            } else {
                println!("lifetime_on_method: {} {}", id, self.msg);
                self.msg
            }
        }
    }
    let m = Message {id: "200", msg: "ok!"};
    m.get_msg_by_id1("200");
    m.get_msg_by_id1("404");

    // д��������
    impl<'a> Message<'a> {
        fn get_msg_by_id2(&'a self, id: &'a str) -> &'a str {
            if self.id == id {
                println!("lifetime_on_method: {} {}", self.id, self.msg);
                self.msg
            } else {
                println!("lifetime_on_method: {} {}", id, self.msg);
                self.msg
            }
        }
    }

    let m = Message {id: "200", msg: "ok!"};
    m.get_msg_by_id2("200");
    m.get_msg_by_id2("404");
}


// ʲô����±����������Զ�������������?
// ���������д���һ����������, ���ҷ���ֵ����Ҳ������ʱ, ��������Ҫ�������д��������������.
fn error_example_that_compile_not_fill_lifetime() {        // �������, ��ע�͵���������
    fn print(status: &str, msg: &str) -> &str {          // ����������� fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &str {}
        if status == "200" {
            msg                                            // msg == &'b str        ��   ָ���� &str ��һ��
        } else {
            status                                         // status == &'a str     ��   ָ���� &str ��һ��
        }
    }

    print("200", "ok!");
    // error output: missing lifetime specifier, expected lifetime parameter.
    // ��ע: ������Ӹ����Ͳ��ǱȽ��������ڵĳ�������,
    //       ����ʵ�ʷ���ֵ��ָ������ֵ�������ڲ�һ�µ�����.
    // ����:
    //       1. ��֪���������Զ�������������:
    //          fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &str {}
    //          ���ǻᱨ��, ��Ϊ������û�и�������÷���ֵ���Ͳ�����������,
    //          ������Ϊ������ֵ��һ������ʱ, ���������������.
    //
    //       2. ����ֶ�������������:
    //          fn print<'a, 'b>(status: &'a str, msg: &'b str) -> &'a str {};
    //          ����Ҳ�ᱨ��, ������Ϊ�߼���������治�����Ƿ��� 'a ���������������� status ���ñ���,
    //          Ҳ�п��ܻ᷵�� 'b ������������� msg ���ñ���, �����ָ���ķ���ֵ -> &'a str �����.
    //
    //       3. Ψһ�Ľ���취��, ��Ϊ���������õ���������, ����һ��������������.
    //          fn print<'a>(status: &'a str, msg: &'a str) -> &'a str {};
    //          ͨ�����ַ�ʽ, ���۷��ص���status����msg�������� 'a ��������,
    //          ���ֱ�﷽ʽ�����������, 1. ���ǵ��������ڳ���һ��, 2. ����ֵ��������ָ��ֵ������һ��.
}


fn fix_error_example_that_compile_not_fill_lifetime() {
    fn print<'a>(status: &'a str, msg: &'a str) -> &'a str {
        if status == "200" {
            println!("{}: {}", status, msg);
            msg
        } else {
            println!("{}: {}", status, msg);
            status
        }
    }

    print("200", "ok!");
    print("404", "page not found!");
}


fn main() {
    multi_reference_parameters_and_return_value_is_not_reference_type();
    only_one_reference_parameter_and_return_value_type_is_reference_too();
    lifetime_on_method();
    // error_example_that_compile_not_fill_lifetime();
    fix_error_example_that_compile_not_fill_lifetime();
}