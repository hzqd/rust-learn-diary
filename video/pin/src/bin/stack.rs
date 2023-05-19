use std::{ptr::null, mem::swap, marker::PhantomPinned, pin::Pin};
use aoko::{structs_new_decl, no_std::functions::fun::s};

fn main() {
    let mut test1 = Test::new(s("test1"));
    let mut test1 = unsafe {
        Pin::new_unchecked(&mut test1)
    };
    test1.as_mut().init();

    let mut test2 = Test::new(s("test2"));
    let mut test2 = unsafe {
        Pin::new_unchecked(&mut test2)
    };
    test2.as_mut().init();

    swap(&mut test1, &mut test2);
    dbg!(test1.as_ref().a(), test1.as_ref().b());
    dbg!(test2.as_ref().a(), test2.as_ref().b());
}

structs_new_decl! {
    struct Test(a: String) {
        b: *const String = null(),
        _marker: PhantomPinned = PhantomPinned,
    }
}

impl Test {
    fn init(self: Pin<&mut Self>) {
        unsafe {
            self.get_unchecked_mut().b = &self.a
        }
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*self.get_ref().b }
    }
}