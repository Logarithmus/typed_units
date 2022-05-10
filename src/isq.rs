use crate::{
    ops::{Div as UnitDiv, Inv as UnitInv, Mul as UnitMul},
    util::{impl_binary_op_for_type_array, impl_unary_op_for_type_array, type_array},
};
use core::ops::{Div, Mul};

/// Metric prefixes
pub mod prefix {
    use crate::prefix::prefixes;

    #[rustfmt::skip]
    prefixes! {
        (yocto, "yocto", y,  "y",  10, -24),
        (zepto, "zepto", z,  "z",  10, -21),
        (atto,  "atto",  a,  "a",  10, -18),
        (femto, "femto", f,  "f",  10, -15),
        (pico,  "pico",  p,  "p",  10, -12),
        (nano,  "nano",  n,  "n",  10,  -9),
        (micro, "micro", mu, "μ",  10,  -6),
        (milli, "milli", m,  "m",  10,  -3),
        (centi, "centi", c,  "c",  10,  -2),
        (deci,  "deci",  d,  "d",  10,  -1),
        (deca,  "deca",  da, "da", 10,   1),
        (hecto, "hecto", h,  "h",  10,   2),
        (kilo,  "kilo",  k,  "k",  10,   3),
        (mega,  "mega",  M,  "M",  10,   6),
        (giga,  "giga",  G,  "G",  10,   9),
        (tera,  "tera",  T,  "T",  10,  12),
        (peta,  "peta",  P,  "P",  10,  15),
        (exa,   "exa",   E,  "E",  10,  18),
        (zetta, "zetta", Z,  "Z",  10,  21),
        (yotta, "yotta", Y,  "Y",  10,  24)
    }
}

/// Base units without prefix
pub mod root {
    use crate::root::roots;

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

type_array!(Unit<L, M, Ti, I, Te, N, J>);
impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Mul, UnitMul);
impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Div, UnitDiv);
impl_unary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, UnitInv, UnitInv);

pub mod unit {
    use super::{
        prefix::k,
        root::{cd, g, m, mol, s, A, K},
        Unit,
    };
    use crate::{
        ops::{Downcast, Upcast, Upcasted},
        unit::SimpleUnit,
    };
    use std::{
        marker::PhantomData,
        ops::{Div, Mul},
    };
    use typenum::{Prod, Quot, N1, P1, P2, Z0};

    macro_rules! impl_unit_upcast_downcast {
        ($(($m:ident, $kg:ident, $s:ident, $A:ident, $K:ident, $mol:ident, $cd:ident) -> $alias:ident,)+) => {
            $(pub struct $alias;

            impl $alias {
                #[must_use]
                pub const fn new() -> Self {
                    Self
                }
            }

            impl Upcast for $alias {
                type Output =
                    Unit<(m, $m), ((k, g), $kg), (s, $s), (A, $A), (K, $K), (mol, $mol), (cd, $cd)>;
            }

            impl Downcast
                for Unit<(m, $m), ((k, g), $kg), (s, $s), (A, $A), (K, $K), (mol, $mol), (cd, $cd)>
            {
                type Output = $alias;
            })+
        };
    }

    impl_unit_upcast_downcast! {
        (Z0, Z0, Z0, Z0, Z0, Z0, Z0) -> Dimensionless,
        (P1, Z0, Z0, Z0, Z0, Z0, Z0) -> Meter,
        (Z0, P1, Z0, Z0, Z0, Z0, Z0) -> Kilogram,
        (Z0, Z0, P1, Z0, Z0, Z0, Z0) -> Second,
        (Z0, Z0, Z0, P1, Z0, Z0, Z0) -> Ampere,
        (Z0, Z0, Z0, Z0, P1, Z0, Z0) -> Kelvin,
        (Z0, Z0, Z0, Z0, Z0, P1, Z0) -> Mole,
        (Z0, Z0, Z0, Z0, Z0, Z0, P1) -> Candela,
        (P1, Z0, N1, Z0, Z0, Z0, Z0) -> MeterPerSecond,
        (P2, Z0, Z0, Z0, Z0, Z0, Z0) -> MeterSquared,
    }
}

#[allow(non_upper_case_globals)]
pub mod consts {
    use crate::unit::SimpleUnit;

    use super::unit::*;

    pub const m: SimpleUnit<Meter> = SimpleUnit::<Meter>::new();
    pub const kg: SimpleUnit<Kilogram> = SimpleUnit::<Kilogram>::new();
    pub const s: SimpleUnit<Second> = SimpleUnit::<Second>::new();
    pub const A: SimpleUnit<Ampere> = SimpleUnit::<Ampere>::new();
    pub const K: SimpleUnit<Kelvin> = SimpleUnit::<Kelvin>::new();
    pub const mol: SimpleUnit<Mole> = SimpleUnit::<Mole>::new();
    pub const cd: SimpleUnit<Candela> = SimpleUnit::<Candela>::new();
}

#[test]
fn test_units() {
    use consts::*;

    let dist = 10_f64 * m;
    let time = 2_f64 * s;
    let speed = dist / time;
}

/// Implement `Mul<Unit<...>>` & `Div<Unit<...>>` operators for $type (e. g. `f32`)
/// `32_f32 * Unit<...>`, `32_f32 / Unit<...>`
macro_rules! impl_mul_div_for_value_by_unit {
    ($type:ty, $feat:literal) => {
        // 10 * km
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J> ::core::ops::Mul<Unit<L, M, Ti, I, Te, N, J>> for $type {
            type Output = $crate::Quantity<Unit<L, M, Ti, I, Te, N, J>, $type>;

            fn mul(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }

        // 10 / km = 10 * km^(-1)
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J> ::core::ops::Div<Unit<L, M, Ti, I, Te, N, J>> for $type
        where
            L: UnitInv,
            M: UnitInv,
            Ti: UnitInv,
            I: UnitInv,
            Te: UnitInv,
            N: UnitInv,
            J: UnitInv,
        {
            type Output =
                $crate::Quantity<$crate::ops::Inverse<Unit<L, M, Ti, I, Te, N, J>>, $type>;

            fn div(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
}

impl_mul_div_for_value_by_unit!(f32, "f32");
impl_mul_div_for_value_by_unit!(f64, "f64");
impl_mul_div_for_value_by_unit!(i8, "18");
impl_mul_div_for_value_by_unit!(u8, "u8");
impl_mul_div_for_value_by_unit!(i16, "i16");
impl_mul_div_for_value_by_unit!(u16, "u16");
impl_mul_div_for_value_by_unit!(i32, "i32");
impl_mul_div_for_value_by_unit!(u32, "u32");
impl_mul_div_for_value_by_unit!(i64, "i64");
impl_mul_div_for_value_by_unit!(u64, "u64");
impl_mul_div_for_value_by_unit!(i128, "i128");
impl_mul_div_for_value_by_unit!(u128, "u128");
