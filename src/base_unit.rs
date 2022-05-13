use crate::{
    name::NameBuf,
    ops::{Div as UnitDiv, Inv, Mul as UnitMul},
    typenum::Positive,
    util::{concat_const_str, ConstStr},
    Name, Prefix, Root,
};
use core::ops::{Add, Neg, Sub};
use typenum::{private::PrivateIntegerAdd, Cmp, Compare, Diff, NInt, Negate, PInt, Sum, Z0};

/// Base unit for system of units
pub trait BaseUnit {}

impl<P: Prefix, R: Root> BaseUnit for (P, R) {}

impl<R: Root> BaseUnit for R {}

impl<P: Prefix, R: Root> NameBuf<16, 64> for (P, R) {
    const SHORT_BUF: ConstStr<16> = concat_const_str!(P::SHORT, R::SHORT);
    const FULL_BUF: ConstStr<64> = concat_const_str!(P::FULL, R::FULL);
}

impl<P: Prefix, R: Root> Name for (P, R) {
    const SHORT: &'static str = Self::SHORT_BUF.as_str();
    const FULL: &'static str = Self::FULL_BUF.as_str();
}

/// Implements operators (`Mul` & `Div`) for `(U, E)`,
/// where `U` -- base unit with or without prefix, `E` -- exponent
macro_rules! impl_ops_for_base_unit_0 {
    ($($op:ident: $out:ident,)+) => {
        $(impl<Ul: BaseUnit, Ur: BaseUnit> $op<(Ur, Z0)> for (Ul, Z0) {
            type Output = (Ul, Z0);
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, El: Positive> $op<(Ur, Z0)> for (Ul, PInt<El>) {
            type Output = Self;
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, El: Positive> $op<(Ur, Z0)> for (Ul, NInt<El>) {
            type Output = Self;
        }

        impl<Ul, Ur, Er: Positive> $op<(Ur, PInt<Er>)> for (Ul, Z0) {
            type Output = (Ur, $out<Z0, PInt<Er>>);
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, Er: Positive> $op<(Ur, NInt<Er>)> for (Ul, Z0) {
            type Output = (Ur, $out<Z0, NInt<Er>>);
        })+
    };
}

impl_ops_for_base_unit_0! {
    UnitMul: Sum,
    UnitDiv: Diff,
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitMul<(U, PInt<Er>)> for (U, PInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, PInt<Sum<El, Er>>);
}

impl<U: BaseUnit, El: Sub<Er> + Positive, Er: Positive> UnitDiv<(U, PInt<Er>)> for (U, PInt<El>)
where
    El: Cmp<Er> + PrivateIntegerAdd<Compare<El, Er>, Er>,
{
    type Output = (U, Diff<PInt<El>, PInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er> UnitMul<(U, PInt<Er>)> for (U, NInt<El>)
where
    Er: Positive + Cmp<El> + PrivateIntegerAdd<Compare<Er, El>, El>,
{
    type Output = (U, Sum<NInt<El>, PInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, PInt<Er>)> for (U, NInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, NInt<Sum<El, Er>>);
}

impl<U, El, Er> UnitMul<(U, NInt<Er>)> for (U, PInt<El>)
where
    U: BaseUnit,
    El: Add<Er> + Positive + Cmp<Er> + PrivateIntegerAdd<Compare<El, Er>, Er>,
    Er: Positive,
{
    type Output = (U, Sum<PInt<El>, NInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, NInt<Er>)> for (U, PInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, PInt<Sum<El, Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitMul<(U, NInt<Er>)> for (U, NInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, Sum<NInt<El>, NInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, NInt<Er>)> for (U, NInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, NInt<Sum<El, Er>>);
}

impl<U, E: Neg> Inv for (U, E) {
    type Output = (U, Negate<E>);
}
