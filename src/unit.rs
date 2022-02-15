use crate::{prefix::dec::kilo, Prefix, Quantity, Root};
use core::marker::PhantomData;
use paste::paste;
use seq_macro::seq;
use std::ops::{Add, Div, Mul, Sub};
use typenum::{
    private::{PrivateDivInt, PrivateIntegerAdd},
    Abs, AbsVal, Cmp, Gcd, Gcf, Integer, NInt, Negate, NonZero, PInt, Unsigned, P1, P3, Z0,
};

pub mod root {
    /// Unit without prefix
    pub trait Root {
        const SHORT: &'static str;
        const LONG: &'static str;
    }

    macro_rules! roots {
        ($(($long:ident, $long_str:literal, $short:ident, $short_str:literal)),+) => {
            $(#[allow(non_camel_case_types)]
            pub struct $short;

            pub type $long = $short;

            impl $crate::Root for $long {
                const SHORT: &'static str = $short_str;
                const LONG: &'static str = $long_str;
            }

            impl core::fmt::Display for $long {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    use $crate::Root;

                    Self::SHORT.fmt(f)
                }
            })+
        };
    }

    #[rustfmt::skip]
    roots! {
        (meter,   "meter",   m,   "m"),
        (gram,    "gram",    g,   "g"),
        (second,  "second",  s,   "s"),
        (ampere,  "ampere",  A,   "A"),
        (kelvin,  "kelvin",  K,   "K"),
        (mole,    "mole",    mol, "mol"),
        (candela, "candela", cd,  "cd")
    }
}

#[rustfmt::skip]
pub mod kind {
    pub trait Length:            super::BaseUnit {}
    pub trait Mass:              super::BaseUnit {}
    pub trait Time:              super::BaseUnit {}
    pub trait Current:           super::BaseUnit {}
    pub trait Temperature:       super::BaseUnit {}
    pub trait AmountOfSubstance: super::BaseUnit {}
    pub trait LuminousIntensity: super::BaseUnit {}
    pub trait Angle:             super::BaseUnit {}
    pub trait Ratio:             super::BaseUnit {}
}

impl kind::Length for root::meter {}
impl kind::Mass for root::gram {}
impl kind::Time for root::second {}
impl kind::Current for root::ampere {}
impl kind::Temperature for root::kelvin {}
impl kind::AmountOfSubstance for root::mole {}
impl kind::LuminousIntensity for root::candela {}

impl<P: Prefix, R: Root + kind::Length> kind::Length for (P, R) {}
impl<P: Prefix, R: Root + kind::Mass> kind::Mass for (P, R) {}
impl<P: Prefix, R: Root + kind::Time> kind::Time for (P, R) {}
impl<P: Prefix, R: Root + kind::Current> kind::Current for (P, R) {}
impl<P: Prefix, R: Root + kind::Temperature> kind::Temperature for (P, R) {}
impl<P: Prefix, R: Root + kind::AmountOfSubstance> kind::AmountOfSubstance for (P, R) {}
impl<P: Prefix, R: Root + kind::LuminousIntensity> kind::LuminousIntensity for (P, R) {}

/// Base unit for system of units
pub trait BaseUnit {}

impl<P: Prefix, R: Root> BaseUnit for (P, R) {}

impl<R: Root> BaseUnit for R {}

/// Repeat $tokens as many times as $_count repeats
macro_rules! repeat {
    ($_count:ident, $($tokens:tt)+) => {
        $($tokens)+
    };
}

macro_rules! count_idents {
    ($($identifier:ident),*) => {<[()]>::len(&[$(repeat!($identifier, ())),*])};
}

#[test]
fn test_count_idents() {
    assert_eq!(count_idents!(A, B, C, D), 4);
}

macro_rules! type_array {
    ($name:ident<$($param:ident),+>) => {
        pub struct $name<$($param = ()),+>(::core::marker::PhantomData<($($param),+)>);

        impl<$($param),+> $name<$($param),+> {
            const LEN: usize = count_idents!($($param),+);

            pub const fn new() -> Self {
                Self(::core::marker::PhantomData)
            }

            pub const fn len() -> usize {
                Self::LEN
            }
        }
    };
}

#[test]
fn type_array_len() {
    type_array!(Test<A, B, C, D, E, F>);
    const len: usize = <Test>::len();
    assert_eq!(len, 6);
}

macro_rules! impl_binary_op_for_type_array {
    ($name:ident<$($param:ident),+>, $op:ident, $op_bound:path) => {
        paste::paste! {
            impl<$( [<$param 1>] : $op_bound< [<$param 2>] >,)+ $( [<$param 2>] ),+> ::core::ops::$op<$name<$( [<$param 2>] ),+>> for $name<$( [<$param 1>] ),+> {
                type Output = $name<$(<[<$param 1>] as $op_bound<[<$param 2>]>>::Output,)+>;

                fn [<$op:lower>](self, _: $name<$( [<$param 2>] ),+>) -> Self::Output {
                    Self::Output::new()
                }
            }
        }
    };
}

macro_rules! impl_unary_op_for_type_array {
    ($name:ident<$($param:ident),+>, $op:ident, $op_bound:path) => {
        impl<$($param: $op_bound,)+> ::core::ops::$op for $name<$($param),+> {
            type Output = $name<$(<$param as $op_bound>::Output,)+>;

            paste::paste! {
                fn [<$op:lower>](self) -> Self::Output {
                    Self::Output::new()
                }
            }
        }
    };
}

/// International System of Quantities
pub trait ISQ {
    type L: BaseUnit;
    type M: BaseUnit;
    type Ti: BaseUnit;
    type I: BaseUnit;
    type Te: BaseUnit;
    type N: BaseUnit;
    type J: BaseUnit;
}

pub struct Si;

impl ISQ for Si {
    type L = root::meter;
    type M = (kilo, root::gram);
    type Ti = root::second;
    type I = root::ampere;
    type Te = root::kelvin;
    type N = root::mole;
    type J = root::candela;
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

/// Generate aliases for operators' output type
/// E. g. `type AddOut<L, R> = <L as Add<R>>::Output`
macro_rules! ops_out_aliases {
    ($($op:ident),+) => {
        paste! {
            $(pub type [<$op Out>]<L, R> = <L as $op<R>>::Output;)+
        }
    };
}

ops_out_aliases!(Add, Sub, Mul, Div, UnitMul, UnitDiv, Cmp);

macro_rules! trait_alias {
    // single alias
    ($first_trait:ident, $($trait:ident),* -> $alias:ident) => {
        pub trait $alias: $first_trait $(+ $trait),* {}
        impl<T: $first_trait $(+ $trait),*> $alias for T {}
    };

    // multiple aliases
    ($($first_trait:ident, $($trait:ident),* -> $alias:ident;),+) => {
        $(trait_alias!($first_trait:ident, $($trait:ident),* -> $alias:ident)),+
    }
}

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

impl<
        U: BaseUnit,
        El: Add<Er> + Positive + Cmp<Er> + PrivateIntegerAdd<CmpOut<El, Er>, Er>,
        Er: Positive,
    > UnitMul<(U, NInt<Er>)> for (U, PInt<El>)
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

pub trait UnitNeg {
    type Output;
}

impl<U, E: core::ops::Neg> UnitNeg for (U, E) {
    type Output = (U, Negate<E>);
}

pub trait LcmOp<Rhs>: Gcd<Rhs> {
    type Output;
}

impl<L: Gcd<R> + Mul<R>, R> LcmOp<R> for L
where
    MulOut<L, R>: Abs,
    AbsVal<MulOut<L, R>>: Div<Gcf<L, R>>,
{
    type Output = DivOut<AbsVal<MulOut<L, R>>, Gcf<L, R>>;
}

pub type Lcm<L, R> = <L as LcmOp<R>>::Output;

trait FracAddOp<Fr> {
    type Lcm;
    type NOut;
    type Output;
}

impl<Nl, Nr, Dl: Mul<Dr> + LcmOp<Dr>, Dr> FracAddOp<(Nr, Dr)> for (Nl, Dl)
where
    Lcm<Dl, Dr>: Div<Dl> + Div<Dr>,
    Nl: Mul<DivOut<Lcm<Dl, Dr>, Dl>>,
    Nr: Mul<DivOut<Lcm<Dl, Dr>, Dr>>,
    MulOut<Nl, DivOut<Lcm<Dl, Dr>, Dl>>: Add<MulOut<Nr, DivOut<Lcm<Dl, Dr>, Dr>>>,
    AddOut<MulOut<Nl, DivOut<Lcm<Dl, Dr>, Dl>>, MulOut<Nr, DivOut<Lcm<Dl, Dr>, Dr>>>:
        Div<Lcm<Dl, Dr>>,
{
    type Lcm = Lcm<Dl, Dr>;
    type NOut = AddOut<MulOut<Nl, DivOut<Self::Lcm, Dl>>, MulOut<Nr, DivOut<Self::Lcm, Dr>>>;
    type Output = DivOut<Self::NOut, Self::Lcm>;
}

type FracAdd<L, R> = <L as FracAddOp<R>>::Output;

pub mod isq {
    use super::{UnitDiv, UnitMul, UnitNeg};

    type_array!(Unit<L, M, Ti, I, Te, N, J>);
    impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Mul, UnitMul);
    impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Div, UnitDiv);
    impl_unary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Neg, UnitNeg);

    mod alias {
        use crate::prefix::dec::*;
        use crate::unit::isq::Unit;
        use crate::unit::root::{cd, g, meter, mol, s, A, K};
        use typenum::{P1, Z0};

        pub type Dimensionless =
            Unit<(meter, P1), ((k, g), Z0), (s, Z0), (A, Z0), (K, Z0), (mol, Z0), (cd, Z0)>;
        pub type Meter =
            Unit<(meter, P1), ((k, g), Z0), (s, Z0), (A, Z0), (K, Z0), (mol, Z0), (cd, Z0)>;
        pub type Kilogram =
            Unit<(meter, Z0), ((k, g), P1), (s, Z0), (A, Z0), (K, Z0), (mol, Z0), (cd, Z0)>;
        pub type Second =
            Unit<(meter, Z0), ((k, g), Z0), (s, P1), (A, Z0), (K, Z0), (mol, Z0), (cd, Z0)>;
        pub type Ampere =
            Unit<(meter, Z0), ((k, g), Z0), (s, Z0), (A, P1), (K, Z0), (mol, Z0), (cd, Z0)>;
        pub type Kelvin =
            Unit<(meter, Z0), ((k, g), Z0), (s, Z0), (A, Z0), (K, P1), (mol, Z0), (cd, Z0)>;
        pub type Mole =
            Unit<(meter, Z0), ((k, g), Z0), (s, Z0), (A, Z0), (K, Z0), (mol, P1), (cd, Z0)>;
        pub type Candela =
            Unit<(meter, Z0), ((k, g), Z0), (s, Z0), (A, Z0), (K, Z0), (mol, Z0), (cd, P1)>;

        pub type Kilometer =
            Unit<((k, meter), P1), ((k, g), Z0), (s, Z0), (A, Z0), (K, Z0), (mol, Z0), (cd, Z0)>;
    }
    pub use alias::*;
}

pub const km: isq::Kilometer = isq::Kilometer::new();
pub const s: isq::Second = isq::Second::new();

#[test]
fn test_units() {
    let km_per_s = km / s;
    let dist = 10_f64 * km;
    let time = 2_f64 * s;
    let speed = dist / time;
    //let km_per_s = km / s;
}
