use std::ops::Add;
use aoko::{no_std::functions::fun::s, assert_eqs};

#[derive(Debug)]
struct Expr;

macro_rules! impl_add_expr {
    ($($rhs:ty, $ext:ty);+ $(;)?) => {
        $(
            impl Add<$rhs> for $ext {
                type Output = String;
            
                fn add(self, other: $rhs) -> Self::Output {
                    format!("{:?} + {:?}", self, other)
                }
            }
        )+
    };
}

impl_add_expr!(
    Expr, Expr;
    u8, Expr; Expr, u8;         i8, Expr; Expr, i8;
    u16, Expr; Expr, u16;       i16, Expr; Expr, i16;
    u32, Expr; Expr, u32;       i32, Expr; Expr, i32;       f32, Expr; Expr, f32;
    u64, Expr; Expr, u64;       i64, Expr; Expr, i64;       f64, Expr; Expr, f64;
    usize, Expr; Expr, usize;   isize, Expr; Expr, isize;
    u128, Expr; Expr, u128;     i128, Expr; Expr, i128;
);

#[test]
fn test() {
    assert_eqs! {
        1 + Expr, s("1 + Expr");
        Expr + 1, s("Expr + 1");
        Expr + Expr, s("Expr + Expr");
    }
}