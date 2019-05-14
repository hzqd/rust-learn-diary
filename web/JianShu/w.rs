//rust--��

// ����������ʽ:
// ����ʽ��: vec!��println!��write! ��Щ��������ʽ��.
// ����ʽ��: #[derive(Debug)]��#[derive(PartialEq)] ��Щ���ǹ���ʽ��.

// ����ʽ��
// ���Զ���һ�ַ��ϵ�ǰ���������ݽṹ, Ȼ��ʹ�øú�����дrust����.

// ����ʽ��
// ��Ҫ��Ϊ�ṹ�塢Ԫ������ݽṹ����ͨ�õ�trait�����ӿں͹�������.


// ����ʽ���﷨
macro_rules! list {
    // $x �Ǳ���
    // :expr �ǹؼ����﷨, ��ʾ���ʽ
    // * ��ʾ��λ��α��ʽƥ��
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(                          
                println!("{}", $x);
                temp_vec.push($x);
            )*                          // ���ƥ�����������������.
            temp_vec
        }
    }
}


// ����ʽ���﷨
// ����


fn main() {
    let x = list!(1,2,3);
    println!("{:?}", x)
}
// �ο�: https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html
// Rust�ֲ�: https://doc.rust-lang.org/reference/macros.html
// macros book: https://danielkeep.github.io/tlborm/book/index.html