//rust--const��static����������?
// ��̬�����ڳ�������������.
// �ο�: https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable
// �ο�: https://doc.rust-lang.org/book/first-edition/const-and-static.html

// ժҪ
// Constants and immutable static variables might seem similar,
// but a subtle difference is that values in a static variable have a fixed address in memory.
// Using the value will always access the same data. Constants,
// on the other hand, are allowed to duplicate their data whenever they��re used.

// ��̬�����ǹ̶����ڴ��ַ, �����������unsafe��������汻�ı�.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// ����û�й̶����ڴ��ַ,
// �������������ڶ�ȡ�����峣��ʱ�����ڴ������, ���Ƕ�ȡ��������ʱ�Żᴴ��.
// �ο�: https://stackoverflow.com/questions/40148175/what-does-it-mean-for-a-const-type-in-rust-to-be-inlined