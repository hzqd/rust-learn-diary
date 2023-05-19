use std::{ptr::null, mem::swap, marker::PhantomPinned, pin::Pin};
use aoko::no_std::functions::fun::s;

fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    swap(&mut test1, &mut test2);

    dbg!(test1.as_ref().a(), test1.as_ref().b());
    dbg!(test2.as_ref().a(), test2.as_ref().b());
}

struct Test {
    a: String,
    b: *const String,
    _maker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: s(txt),
            b: null(),
            _maker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        unsafe {
            boxed.as_mut().get_unchecked_mut().b = &boxed.as_ref().a
        }
        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*self.get_ref().b }
    }
}