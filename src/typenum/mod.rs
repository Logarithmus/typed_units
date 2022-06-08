//! This module should be upstreamed to <https://lib.rs/typenum>

pub mod frac;

use crate::util::{binary_ops_out_aliases, trait_alias};
use core::ops::{Div, Mul};
use typenum::{op, Abs, Gcd, Gcf, NonZero, Prod, Unsigned};

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
    Prod<L, R>: Abs,
    op!(abs(L * R)): Div<Gcf<L, R>>,
{
    type Output = op!(abs(L * R) / gcd(L, R));
}

pub trait Alias {
    type Alias;
}

pub struct P1;

impl Alias for typenum::P1 {
    type Alias = P1;
}
