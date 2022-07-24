use crate::{
    ops::{Div as UnitDiv, Inv, Mul as UnitMul},
    typenum::{Constant, ToConst, ToTypenum, Typenum},
    Const, Name, Prefix, Root,
};
use const_default::ConstDefault;
use core::{
    i32,
    marker::PhantomData,
    ops::{Add, Neg, Sub},
};
use typenum::{Diff, Negate, Sum};

/// Prefixed unit
pub struct Pre<P, R>(PhantomData<(P, R)>);

pub trait Exponent {
    const EXP: i8;
}

impl Exponent for () {
    const EXP: i8 = 0;
}

impl<R: Root> Exponent for R {
    const EXP: i8 = 1;
}

impl<P, R: Root> Exponent for Pre<P, R> {
    const EXP: i8 = 1;
}

impl<U, const E: i8> Exponent for Exp<U, E> {
    const EXP: i8 = E;
}

pub struct Exp<U, const N: i8>(PhantomData<U>);

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

impl<U: crate::name::Display, const E: i8> crate::name::Display for Exp<U, E> {
    fn display() -> String {
        U::display()
    }
}

impl<U: crate::name::Debug, const E: i8> crate::name::Debug for Exp<U, E> {
    fn debug() -> String {
        U::debug()
    }
}

impl<U, El: Add<Er>, Er> UnitMul<(U, Er)> for (U, El) {
    type Output = (U, Sum<El, Er>);
}

impl<U, El: Sub<Er>, Er> UnitDiv<(U, Er)> for (U, El) {
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

// -----------------------------------------

pub trait ToExp<U> {
    type Output;
}

impl<U, const N: i8> ToExp<U> for Const<N> {
    type Output = Exp<U, N>;
}

type ToExponent<U, E> = <E as ToExp<U>>::Output;

impl<U, const EL: i8, const ER: i8> UnitMul<Exp<U, ER>> for Exp<U, EL>
where
    Const<EL>: ToTypenum,
    Const<ER>: ToTypenum,
    Typenum<Const<EL>>: Add<Typenum<Const<ER>>>,
    Sum<Typenum<Const<EL>>, Typenum<Const<ER>>>: ToConst,
    Constant<Sum<Typenum<Const<EL>>, Typenum<Const<ER>>>>: ToExp<U> + ConstDefault,
{
    type Output = ToExponent<U, Sum<Const<EL>, Const<ER>>>;
}

impl<U, const EL: i8, const ER: i8> UnitDiv<Exp<U, ER>> for Exp<U, EL>
where
    Const<EL>: ToTypenum,
    Const<ER>: ToTypenum,
    Typenum<Const<EL>>: Sub<Typenum<Const<ER>>>,
    Diff<Typenum<Const<EL>>, Typenum<Const<ER>>>: ToConst,
    Constant<Diff<Typenum<Const<EL>>, Typenum<Const<ER>>>>: ToExp<U> + ConstDefault,
{
    type Output = ToExponent<U, Diff<Const<EL>, Const<ER>>>;
}

impl<U, const E: i8> Inv for Exp<U, E>
where
    Const<E>: ToTypenum,
    Typenum<Const<E>>: Neg,
    Negate<Typenum<Const<E>>>: ToConst,
    Constant<Negate<Typenum<Const<E>>>>: ToExp<U> + ConstDefault,
{
    type Output = ToExponent<U, Negate<Const<E>>>;
}

impl<U, const E: i8> UnitMul<Exp<U, E>> for () {
    type Output = Exp<U, E>;
}

impl<U, const E: i8> UnitMul<()> for Exp<U, E> {
    type Output = Exp<U, E>;
}

impl<U, const E: i8> UnitDiv<()> for Exp<U, E> {
    type Output = Exp<U, E>;
}

impl<U, const E: i8> UnitDiv<Exp<U, E>> for ()
where
    Const<E>: ToTypenum,
    Typenum<Const<E>>: Neg,
    Negate<Typenum<Const<E>>>: ToConst,
    Constant<Negate<Typenum<Const<E>>>>: ToExp<U> + ConstDefault,
{
    type Output = ToExponent<U, Negate<Const<E>>>;
}

pub trait ConvertFrom<U, V> {
    fn convert_from(value: V) -> V;
}

// impl<Pl: Prefix<V>, Rl, Pr: Prefix<V>, Rr, V> ConvertFrom<Pre<Pl, Rl>, V> for Pre<Pr, Rr> {
//     fn from(value: V) -> V {
//     }
// }
