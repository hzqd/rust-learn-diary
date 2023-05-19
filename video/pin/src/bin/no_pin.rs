use std::{ptr::null, mem::swap};
use aoko::{structs_new_decl, no_std::functions::fun::s};

fn main() {
    let mut test1 = Test::new(s("test1"));
    test1.init();

    let mut test2 = Test::new(s("test2"));
    test2.init();

    swap(&mut test1, &mut test2);
    dbg!(test1.a(), test1.b());
    dbg!(test2.a(), test2.b());
}

structs_new_decl! {
    struct Test(a: String) {
        b: *const String = null(),
    }
}

impl Test {
    fn init(self: &mut Self) {
        self.b = &self.a
    }

    fn a(self: &Self) -> &str {
        &self.a
    }

    fn b(self: &Self) -> &String {
        unsafe { &*self.b }
    }
}