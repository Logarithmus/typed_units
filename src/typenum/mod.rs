//! This module should be upstreamed to <https://lib.rs/typenum>
pub mod frac;

use crate::util::{binary_ops_out_aliases, trait_alias};
use const_default::ConstDefault;
use core::ops::{Add, Div, Mul, Sub};
use std::ops::Neg;
use typenum::{
    op, Abs, Diff, Gcd, Gcf, Integer, Negate, NonZero, Prod, Quot, Sum, Unsigned, N1, N2, N3, N4,
    N5, N6, N7, N8, P1, P2, P3, P4, P5, P6, P7, P8, Z0,
};

trait_alias!((Unsigned, NonZero) -> Positive);

/// Least common multiple for `typenum`
pub trait LcmOp<Rhs>: Gcd<Rhs> {
    type Output;
}

binary_ops_out_aliases! {
    LcmOp -> Lcm,
}

impl<L: Gcd<R> + Mul<R>, R> LcmOp<R> for L
where
    op!(L * R): Abs,
    op!(abs(L * R)): Div<Gcf<L, R>>,
{
    type Output = op!(abs(L * R) / gcd(L, R));
}

pub trait ToTypenum {
    type Typenum: Integer;
}

pub type Typenum<N> = <N as ToTypenum>::Typenum;

pub trait ToConst {
    type Const;
}

pub type Const<T> = <T as ToConst>::Const;

#[derive(Clone, Copy)]
pub struct Num<const N: i8>;

macro_rules! num_to_typenum_and_back {
    ($($const:literal <-> $typenum:ident,)+) => {
        $(impl ToTypenum for Num<$const> {
            type Typenum = $typenum;
        }

        impl ToConst for $typenum {
            type Const = Num<$const>;
        })+
    };
}

num_to_typenum_and_back! {
   -8 <-> N8,
   -7 <-> N7,
   -6 <-> N6,
   -5 <-> N5,
   -4 <-> N4,
   -3 <-> N3,
   -2 <-> N2,
   -1 <-> N1,
    0 <-> Z0,
    1 <-> P1,
    2 <-> P2,
    3 <-> P3,
    4 <-> P4,
    5 <-> P5,
    6 <-> P6,
    7 <-> P7,
    8 <-> P8,
}

macro_rules! impl_binary_ops_for_num {
    ($(($op:ident, $fun:ident, $out:ident),)+) => {
        $(impl<const L: i8, const R: i8> $op<Num<R>> for Num<L>
        where
            Num<L>: ToTypenum,
            Num<R>: ToTypenum,
            Typenum<Num<L>>: $op<Typenum<Num<R>>>,
            $out<Typenum<Num<L>>, Typenum<Num<R>>>: ToConst,
            Const<$out<Typenum<Num<L>>, Typenum<Num<R>>>>: ConstDefault,
        {
            type Output = Const<$out<Typenum<Num<L>>, Typenum<Num<R>>>>;

            fn $fun(self, _: Num<R>) -> Self::Output {
                Self::Output::DEFAULT
            }
        })+
    };
}

impl_binary_ops_for_num! {
    (Add, add, Sum),
    (Sub, sub, Diff),
    (Mul, mul, Prod),
    (Div, div, Quot),
}

impl<const N: i8> Neg for Num<N>
where
    Num<N>: ToTypenum,
    Typenum<Num<N>>: Neg,
    Negate<Typenum<Num<N>>>: ToConst,
    Const<Negate<Typenum<Num<N>>>>: ConstDefault,
{
    type Output = Const<Negate<Typenum<Num<N>>>>;

    fn neg(self) -> Self::Output {
        Self::Output::DEFAULT
    }
}
