use crate::{
    name::NameBuf,
    ops::{AddOut, CmpOut, SubOut},
    util::{binary_ops_out_aliases, concat_const_str, trait_alias, ConstStr},
    Name, Prefix, Root,
};
use core::ops::{Add, Neg, Sub};
use typenum::{
    private::PrivateIntegerAdd, Abs, AbsVal, Cmp, Gcd, Gcf, NInt, Negate, NonZero, PInt, Unsigned,
    Z0,
};

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

pub trait UnitAdd<Rhs = Self> {
    type Output;
}
pub trait UnitSub<Rhs = Self> {
    type Output;
}

pub trait UnitMul<Rhs = Self> {
    type Output;
}

pub trait UnitDiv<Rhs = Self> {
    type Output;
}

binary_ops_out_aliases!(UnitAdd, UnitSub, UnitMul, UnitDiv);

trait_alias!(Unsigned, NonZero -> Positive);

/// Implements operators (Mul & Div) for (U, E),
/// where U -- base unit with or without prefix, E -- exponent
macro_rules! impl_ops_for_base_unit_0 {
    ($($op:ident: $op_with:ident: $out:ident),+) => {
        $(impl<Ul: BaseUnit, Ur: BaseUnit> $op<(Ur, Z0)> for (Ul, Z0) {
            type Output = (Ul, Z0);
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, El: Positive> $op<(Ur, Z0)> for (Ul, PInt<El>) {
            type Output = Self;
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, El: Positive> $op<(Ur, Z0)> for (Ul, NInt<El>) {
            type Output = Self;
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, Er: Positive> $op<(Ur, PInt<Er>)> for (Ul, Z0) {
            type Output = (Ur, $out<Z0, PInt<Er>>);
        }

        impl<Ul: BaseUnit, Ur: BaseUnit, Er: Positive> $op<(Ur, NInt<Er>)> for (Ul, Z0) {
            type Output = (Ur, $out<Z0, NInt<Er>>);
        })+
    };
}

impl_ops_for_base_unit_0!(UnitMul: Add: AddOut, UnitDiv: Sub: SubOut);

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitMul<(U, PInt<Er>)> for (U, PInt<El>)
where
    AddOut<El, Er>: Positive,
{
    type Output = (U, PInt<AddOut<El, Er>>);
}

impl<U: BaseUnit, El: Sub<Er> + Positive, Er: Positive> UnitDiv<(U, PInt<Er>)> for (U, PInt<El>)
where
    SubOut<El, Er>: Positive,
    El: Cmp<Er> + PrivateIntegerAdd<CmpOut<El, Er>, Er>,
{
    type Output = (U, SubOut<PInt<El>, PInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er> UnitMul<(U, PInt<Er>)> for (U, NInt<El>)
where
    Er: Positive + Cmp<El> + PrivateIntegerAdd<CmpOut<Er, El>, El>,
{
    type Output = (U, AddOut<NInt<El>, PInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, PInt<Er>)> for (U, NInt<El>)
where
    AddOut<El, Er>: Positive,
{
    type Output = (U, NInt<AddOut<El, Er>>);
}

impl<U, El, Er> UnitMul<(U, NInt<Er>)> for (U, PInt<El>)
where
    U: BaseUnit,
    El: Add<Er> + Positive + Cmp<Er> + PrivateIntegerAdd<CmpOut<El, Er>, Er>,
    Er: Positive,
{
    type Output = (U, AddOut<PInt<El>, NInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, NInt<Er>)> for (U, PInt<El>)
where
    AddOut<El, Er>: Positive,
{
    type Output = (U, PInt<AddOut<El, Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitMul<(U, NInt<Er>)> for (U, NInt<El>)
where
    AddOut<El, Er>: Positive,
{
    type Output = (U, AddOut<NInt<El>, NInt<Er>>);
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitDiv<(U, NInt<Er>)> for (U, NInt<El>)
where
    AddOut<El, Er>: Positive,
{
    type Output = (U, NInt<AddOut<El, Er>>);
}

/// Inversion (reciprocal) operator for (Unit, Exponent)
pub trait UnitInv {
    type Output;
}

impl<U, E: Neg> UnitInv for (U, E) {
    type Output = (U, Negate<E>);
}
