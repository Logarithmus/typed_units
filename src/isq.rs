use crate::{
    base_unit::Exp,
    ops::{Div as UnitDiv, Inv as UnitInv, Mul as UnitMul},
    util::{impl_binary_op_for_type_array, impl_unary_op_for_type_array, type_array},
};
use const_default::ConstDefault;
use core::{
    fmt::{self, Debug, Display, Formatter},
    i32,
    marker::PhantomData,
    ops::{Div, Mul},
};

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
        (yotta, "yotta", Y,  "Y",  10,  24),
    }
}

/// Base units without prefix
pub mod root {
    use std::ops::Add;

    use crate::{
        base_unit::{ConvertFrom, Pre},
        kind,
        root::{roots, roots_with_alias},
        Root,
    };

    #[rustfmt::skip]
    roots_with_alias! {
        (meter,      "meter",             m,    "m"),
        (AstroUnit,  "astronomical unit", au,   "AU"),
        (gram,       "gram",              g,    "g"),
        (second,     "second",            s,    "s"),
        (ampere,     "Ampere",            A,    "A"),
        (Kelvin,     "Kelvin",            K,    "K"),
        (Celsius,    "degree Celsius",    degC, "°C"),
        (Fahrenheit, "degree Fahrenheit", degF, "°F"),
        (mole,       "mole",              mol,  "mol"),
        (candela,    "candela",           cd,   "cd"),
        (foot,       "foot",              ft,   "ft"),
        (yard,       "yard",              yd,   "yd"),
        (degree,     "degree",            deg,  "°"),
        (radian,     "radian",            rad,  "rad"),
    }

    #[rustfmt::skip]
    roots! {
        (inch, "inch", r#"""#),
    }

    impl<V: Add<Output = V> + From<f64>> ConvertFrom<Celsius, V> for Kelvin {
        fn convert_from(value: V) -> V {
            value + V::from(273.15)
        }
    }

    impl kind::Length for meter {}
    impl kind::Mass for gram {}
    impl kind::Time for second {}
    impl kind::Current for ampere {}
    impl kind::Temperature for Kelvin {}
    impl kind::AmountOfSubstance for mole {}
    impl kind::LuminousIntensity for candela {}

    impl<P, R: Root + kind::Length> kind::Length for Pre<P, R> {}
    impl<P, R: Root + kind::Mass> kind::Mass for Pre<P, R> {}
    impl<P, R: Root + kind::Time> kind::Time for Pre<P, R> {}
    impl<P, R: Root + kind::Current> kind::Current for Pre<P, R> {}
    impl<P, R: Root + kind::Temperature> kind::Temperature for Pre<P, R> {}
    impl<P, R: Root + kind::AmountOfSubstance> kind::AmountOfSubstance for Pre<P, R> {}
    impl<P, R: Root + kind::LuminousIntensity> kind::LuminousIntensity for Pre<P, R> {}
}

type_array!(Unit<L, M, Ti, I, Te, N, J>);
impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Mul, UnitMul);
impl_binary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, Div, UnitDiv);
impl_unary_op_for_type_array!(Unit<L, M, Ti, I, Te, N, J>, UnitInv, UnitInv);

impl<L, M, Ti, I, Te, N, J> ConstDefault for Unit<L, M, Ti, I, Te, N, J> {
    const DEFAULT: Self = Self(PhantomData);
}

macro_rules! impl_trait_for_unit {
    ($unit:ident<$($base_unit:ident),+>, $trait:ident, $fun:ident) => {
        impl<$($base_unit: crate::name::$trait + Exp),+> $trait for $unit<$($base_unit),+> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let mut numerator = vec![];
                let mut denominator = vec![];
                let mut numerator_count = 0;
                let mut denominator_count = 0;
                $(match $base_unit::EXP {
                    1 => {
                        numerator.push($base_unit::$fun());
                        numerator_count += 1;
                    }
                    2..=i32::MAX => {
                        numerator.push(format!("#{}{}", $base_unit::$fun(),  crate::name::superscript($base_unit::EXP)));
                        numerator_count += 1;
                    }
                    -1 => {
                        denominator.push($base_unit::$fun());
                        denominator_count += 1;
                    }
                    i32::MIN..=-2 => {
                        denominator.push(format!("{}{}", $base_unit::$fun(), crate::name::superscript(-$base_unit::EXP)));
                        denominator_count += 1;
                    }
                    0 => (),
                })+

                match (numerator_count, denominator_count) {
                    (0, 0) => (),
                    (_, 0) => write!(f, "{}", numerator.join("⋅"))?,
                    (0, _) => write!(f, "1/{}", denominator.join("⋅"))?,
                    (1, 1) => write!(f, "{}/{}", numerator[0], denominator[0])?,
                    (1, _) => write!(f, "{}/({})", numerator[0], denominator.join("⋅"))?,
                    (_, 1) => write!(f, "({})/{}", numerator.join("⋅"), denominator[0])?,
                    _      => write!(f, "({})/({})", numerator.join("⋅"), denominator.join("."))?,
                }

                Ok(())
            }
        }
    };
}

impl_trait_for_unit!(Unit<L, M, Ti, I, Te, N, J>, Display, display);
impl_trait_for_unit!(Unit<L, M, Ti, I, Te, N, J>, Debug, debug);

pub mod unit {
    use super::{
        prefix::k,
        root::{cd, g, m, mol, s, A, K},
        Unit,
    };
    use crate::base_unit::Pre;
    use typenum::{N1, P1, P2, Z0};

    macro_rules! unit_aliases {
        ($(($m:ident, $kg:ident, $s:ident, $A:ident, $K:ident, $mol:ident, $cd:ident) -> $alias:ident,)+) => {
            $(pub type $alias =
                Unit<(m, $m), (Pre<k, g>, $kg), (s, $s), (A, $A), (K, $K), (mol, $mol), (cd, $cd)>;)+
        };
    }

    unit_aliases! {
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
    use super::{
        unit::{Ampere, Candela, Kelvin, Kilogram, Meter, Mole, Second},
        Unit,
    };
    use const_default::ConstDefault;

    pub const m: Meter = Unit::DEFAULT;
    pub const kg: Kilogram = Unit::DEFAULT;
    pub const s: Second = Unit::DEFAULT;
    pub const A: Ampere = Unit::DEFAULT;
    pub const K: Kelvin = Unit::DEFAULT;
    pub const mol: Mole = Unit::DEFAULT;
    pub const cd: Candela = Unit::DEFAULT;
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
