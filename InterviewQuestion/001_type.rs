// 面试：在 main 函数中的 test 函数内的闭包参数 s 是什么类型？
trait Ext: Sized {
    fn test(self: &Self, f: impl Fn(Self)) {}
}

impl<T> Ext for &T {}

fn main() {
    (&&&mut String::new()).test(|s| ());
}
