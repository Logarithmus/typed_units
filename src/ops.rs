use crate::util::{binary_ops_out_aliases, unary_ops_out_aliases};

unary_ops_out_aliases! {
    Inv -> Inverse,
}

macro_rules! reexport_core_ops {
    ($(($op:ident, $sign:literal),)+) => {
        $(#[doc = concat!(
            "`", $sign, "` operator, used when `std::ops::", stringify!($op),
            "` can't be used due to orphan rules"
        )]
        pub trait $op<Rhs = Self> {
            type Output;
        })+
    };
}

reexport_core_ops! {
    (Add, "+"),
    (Sub, "-"),
    (Mul, "*"),
    (Div, "/"),
}

pub trait Inv {
    type Output;
}

pub trait One {
    const ONE: Self;
}

binary_ops_out_aliases! {
    Add -> Sum,
    Sub -> Diff,
    Mul -> Prod,
    Div -> Quot,
}
