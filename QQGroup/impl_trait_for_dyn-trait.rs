trait A {
    fn foo(&self);
}

trait Z {}

impl A for dyn Z {
    fn foo(&self) {}
}

impl Z for i32 {}   impl Z for bool {}

fn main() {         Z::foo(&true as &dyn Z);
    let x = 1;
    let dyn_x = &x as &dyn Z;
    dyn_x.foo();    Z::foo(&1 as &dyn Z);   <dyn Z as A>::foo(&1 as &dyn Z);

    Z::bar();       <dyn Z as B>::bar();
}

trait B {
    fn bar();
}

impl B for dyn Z {
    fn bar() {}
}
