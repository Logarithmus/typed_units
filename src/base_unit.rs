use crate::{
    ops::{Div as UnitDiv, Inv, Mul as UnitMul},
    typenum::Positive,
    Name, Root,
};
use core::{
    i32,
    marker::PhantomData,
    ops::{Add, Neg, Sub},
};
use typenum::{
    private::PrivateIntegerAdd, Cmp, Compare, Diff, Integer, NInt, Negate, PInt, Sum, Z0,
};

/// Prefixed unit
pub struct Pre<P, R>(PhantomData<(P, R)>);

pub trait Exp {
    const EXP: i32;
}

impl<R: Root> Exp for R {
    const EXP: i32 = 1;
}

impl<P, R: Root> Exp for Pre<P, R> {
    const EXP: i32 = 1;
}

impl<U, E: Integer> Exp for (U, E) {
    const EXP: i32 = E::I32;
}

/// Base unit for system of units
pub trait BaseUnit {}

impl<P, R> BaseUnit for Pre<P, R> {}

impl<R: Root> BaseUnit for R {}

impl<P: Name, R: Name> crate::name::Display for Pre<P, R> {
    fn display() -> String {
        format!("{}{}", P::SHORT, R::SHORT)
    }
}

impl<P: Name, R: Name> crate::name::Debug for Pre<P, R> {
    fn debug() -> String {
        format!("{}{}", P::FULL, R::FULL)
    }
}

impl<U: crate::name::Display, E> crate::name::Display for (U, E) {
    fn display() -> String {
        U::display()
    }
}

impl<U: crate::name::Debug, E> crate::name::Debug for (U, E) {
    fn debug() -> String {
        U::debug()
    }
}

/// Implements operators (`Mul` & `Div`) for `(U, E)`,
/// where `U` -- base unit with or without prefix, `E` -- exponent
macro_rules! impl_ops_for_exp_unit_0 {
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

impl_ops_for_exp_unit_0! {
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

pub trait ConvertFrom<U, V> {
    fn convert_from(value: V) -> V;
}

// impl<Pl: Prefix<V>, Rl, Pr: Prefix<V>, Rr, V> ConvertFrom<Pre<Pl, Rl>, V> for Pre<Pr, Rr> {
//     fn from(value: V) -> V {
//     }
// }
