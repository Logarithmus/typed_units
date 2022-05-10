use crate::{
    name::NameBuf,
    ops::{Div as UnitDiv, Downcast, Downcasted, Inv, Inverse, Mul as UnitMul, Upcast, Upcasted},
    typenum::Positive,
    util::{concat_const_str, ConstStr},
    Name, Prefix, Root,
};
use core::ops::{Add, Neg, Sub};
use std::{
    marker::PhantomData,
    ops::{Div, Mul},
};
use typenum::{
    private::PrivateIntegerAdd, Cmp, Compare, Diff, NInt, Negate, PInt, Prod, Quot, Sum, Z0,
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

/// Implements operators (Mul & Div) for (U, E),
/// where U -- base unit with or without prefix, E -- exponent
macro_rules! impl_ops_for_base_unit_0 {
    ($($op:ident: $op_with:ident: $out:ident,)+) => {
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

impl_ops_for_base_unit_0! {
    UnitMul: Add: Sum,
    UnitDiv: Sub: Diff,
}

impl<U: BaseUnit, El: Add<Er> + Positive, Er: Positive> UnitMul<(U, PInt<Er>)> for (U, PInt<El>)
where
    Sum<El, Er>: Positive,
{
    type Output = (U, PInt<Sum<El, Er>>);
}

impl<U: BaseUnit, El: Sub<Er> + Positive, Er: Positive> UnitDiv<(U, PInt<Er>)> for (U, PInt<El>)
where
    Diff<El, Er>: Positive,
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

pub struct SimpleUnit<U>(PhantomData<U>);

impl<U> SimpleUnit<U> {
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<U: Upcast> Upcast for SimpleUnit<U> {
    type Output = Upcasted<U>;
}

impl<Ul: Upcast, Ur: Upcast> Mul<SimpleUnit<Ur>> for SimpleUnit<Ul>
where
    Upcasted<Ul>: Mul<Upcasted<Ur>>,
    Prod<Upcasted<Ul>, Upcasted<Ur>>: Downcast,
{
    type Output = SimpleUnit<Downcasted<Prod<Upcasted<Ul>, Upcasted<Ur>>>>;

    fn mul(self, _: SimpleUnit<Ur>) -> Self::Output {
        Self::Output::new()
    }
}

impl<Ul: Upcast, Ur: Upcast> Div<SimpleUnit<Ur>> for SimpleUnit<Ul>
where
    Upcasted<Ul>: Div<Upcasted<Ur>>,
    Quot<Upcasted<Ul>, Upcasted<Ur>>: Downcast,
{
    type Output = SimpleUnit<Downcasted<Quot<Upcasted<Ul>, Upcasted<Ur>>>>;

    fn div(self, _: SimpleUnit<Ur>) -> Self::Output {
        Self::Output::new()
    }
}

/// Implement `Mul<SimpleUnit<...>>` & `Div<SimpleUnit<...>>` operators for $type (e. g. `f32`)
/// `32_f32 * SimpleUnit<...>`, `32_f32 / SimpleUnit<...>`
macro_rules! impl_mul_div_for_value_by_simple_unit {
    ($type:ty, $feat:literal) => {
        // 10 * km
        //#[cfg(feature = $feat)]
        impl<U> ::core::ops::Mul<SimpleUnit<U>> for $type {
            type Output = $crate::Quantity<SimpleUnit<U>, $type>;

            fn mul(self, _: SimpleUnit<U>) -> Self::Output {
                Self::Output::new(self)
            }
        }

        // 10 / km = 10 * km^(-1)
        //#[cfg(feature = $feat)]
        impl<U: Inv> ::core::ops::Div<SimpleUnit<U>> for $type
        where
            Inverse<U>: Downcast,
        {
            type Output = $crate::Quantity<SimpleUnit<Downcasted<Inverse<U>>>, $type>;

            fn div(self, _: SimpleUnit<U>) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
}

impl_mul_div_for_value_by_simple_unit!(f32, "f32");
impl_mul_div_for_value_by_simple_unit!(f64, "f64");
impl_mul_div_for_value_by_simple_unit!(i8, "18");
impl_mul_div_for_value_by_simple_unit!(u8, "u8");
impl_mul_div_for_value_by_simple_unit!(i16, "i16");
impl_mul_div_for_value_by_simple_unit!(u16, "u16");
impl_mul_div_for_value_by_simple_unit!(i32, "i32");
impl_mul_div_for_value_by_simple_unit!(u32, "u32");
impl_mul_div_for_value_by_simple_unit!(i64, "i64");
impl_mul_div_for_value_by_simple_unit!(u64, "u64");
impl_mul_div_for_value_by_simple_unit!(i128, "i128");
impl_mul_div_for_value_by_simple_unit!(u128, "u128");
