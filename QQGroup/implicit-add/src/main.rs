use f4::Add;

fn main() {}

mod f4 {
    pub trait Add<Rhs = Self> {
        type Output;
        fn add(self, other: Rhs) -> Self::Output;
    }

    macro_rules! add_impl {
        ($lhs:ty, $rhs:ty) => {
            impl Add<$rhs> for $lhs {
                type Output = $lhs;
                fn add(self, other: $rhs) -> $lhs {
                    self + (other as $lhs)
                }
            }
        };
        ($lhs:ty) => {
            impl Add for $lhs {
                type Output = $lhs;
                fn add(self, other: $lhs) -> $lhs {
                    self + other
                }
            }
        };
    }

    macro_rules! pairing {
        ($t: ty, $($ts: ty),*) => {
            add_impl!{$t}
            $(add_impl!{$t, $ts})*
            pairing!{$($ts),*}
        };
        ($t: ty) => {add_impl!{$t}};
    }

    pairing! {u128, i128, u64, i64, u32, i32, u16, i16, u8, i8}
}

#[test]
fn t() {
    dbg!(255u8.add(1i8));
}
