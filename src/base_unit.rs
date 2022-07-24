use crate::{
    ops::{Div as UnitDiv, Inv, Mul as UnitMul},
    Name, Prefix, Root,
};
use core::{
    i32,
    marker::PhantomData,
    ops::{Add, Neg, Sub},
};
use typenum::{Diff, Negate, Sum};

/// Prefixed unit
pub struct Pre<P, R>(PhantomData<(P, R)>);

pub trait Exponent {
    const EXP: i32;
}

impl Exponent for () {
    const EXP: i32 = 0;
}

impl<R: Root> Exponent for R {
    const EXP: i32 = 1;
}

impl<P, R: Root> Exponent for Pre<P, R> {
    const EXP: i32 = 1;
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

impl<U: BaseUnit, El: Add<Er>, Er> UnitMul<(U, Er)> for (U, El) {
    type Output = (U, Sum<El, Er>);
}

impl<U: BaseUnit, El: Sub<Er>, Er> UnitDiv<(U, Er)> for (U, El) {
    type Output = (U, Diff<El, Er>);
}

impl<U, E: Neg> Inv for (U, E) {
    type Output = (U, Negate<E>);
}

impl UnitMul for () {
    type Output = ();
}

impl UnitDiv for () {
    type Output = ();
}

impl<U, E> UnitMul<(U, E)> for () {
    type Output = (U, E);
}

impl<U, E> UnitMul<()> for (U, E) {
    type Output = (U, E);
}

impl<U, E> UnitDiv<()> for (U, E) {
    type Output = (U, E);
}

impl<U, E: Neg> UnitDiv<(U, E)> for () {
    type Output = (U, Negate<E>);
}

impl Inv for () {
    type Output = ();
}

pub trait ConvertFrom<U, V> {
    fn convert_from(value: V) -> V;
}

// impl<Pl: Prefix<V>, Rl, Pr: Prefix<V>, Rr, V> ConvertFrom<Pre<Pl, Rl>, V> for Pre<Pr, Rr> {
//     fn from(value: V) -> V {
//     }
// }
