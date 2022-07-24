use crate::{
    base_unit::Exponent,
    name::superscript,
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
use std::ops::Neg;

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
        (AstroUnit,  "astronomical unit", AU,   "AU"),
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

#[derive(Clone, Debug)]
struct ExpUnit {
    pub name: String,
    pub exp: i32,
}

impl ExpUnit {
    fn new(name: String, exp: i32) -> Self {
        Self { name, exp }
    }
}

impl Neg for ExpUnit {
    type Output = ExpUnit;

    fn neg(self) -> Self::Output {
        ExpUnit { exp: -self.exp, ..self }
    }
}

impl Display for ExpUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if self.exp.abs() != 1 {
            write!(f, "{}", superscript(self.exp))?;
        }
        Ok(())
    }
}

fn fmt_product_of_units(f: &mut Formatter<'_>, units: &[ExpUnit], sign: bool) -> fmt::Result {
    write!(f, "{}", if sign { units[0].clone() } else { -units[0].clone() })?;
    for unit in &units[1..] {
        write!(f, "⋅{}", if sign { unit.clone() } else { -unit.clone() })?;
    }
    Ok(())
}

macro_rules! impl_trait_for_unit {
    ($unit:ident<$($base_unit:ident),+>, $trait:ident, $fun:ident) => {
        impl<$($base_unit: crate::name::$trait + Exponent),+> $trait for $unit<$($base_unit),+> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let (numerator, denominator): (Vec<_>, Vec<_>) = [
                    $(ExpUnit::new($base_unit::$fun(), $base_unit::EXP)),+
                ].into_iter()
                    .filter(|unit| unit.exp != 0)
                    .partition(|unit| unit.exp > 0);

                match (numerator.len(), denominator.len()) {
                    (0, 0) => Ok(()),
                    (_, 0) => fmt_product_of_units(f, &numerator, true),
                    (0, _) => fmt_product_of_units(f, &denominator, true),
                    (1, 1) => {
                        fmt_product_of_units(f, &numerator, true)?;
                        write!(f, "/")?;
                        fmt_product_of_units(f, &denominator, false)

                    }
                    (1, _) => {
                        fmt_product_of_units(f, &numerator, true)?;
                        write!(f, "/(")?;
                        fmt_product_of_units(f, &denominator, false)?;
                        write!(f, ")")
                    }
                    (_, 1) => {
                        write!(f, "(")?;
                        fmt_product_of_units(f, &numerator, true)?;
                        write!(f, ")/")?;
                        fmt_product_of_units(f, &denominator, false)
                    }
                    _      => {
                        write!(f, "(")?;
                        fmt_product_of_units(f, &numerator, true)?;
                        write!(f, ")/(")?;
                        fmt_product_of_units(f, &denominator, false)?;
                        write!(f, ")")
                    }
                }
            }
        }
    };
}

impl_trait_for_unit!(Unit<L, M, Ti, I, Te, N, J>, Display, display);
impl_trait_for_unit!(Unit<L, M, Ti, I, Te, N, J>, Debug, debug);

pub mod unit {
    use super::{
        prefix::{k, kilo},
        root::{cd, g, gram, m, meter, mol, s, second, A, K},
        Unit,
    };
    use crate::{base_unit::Pre, typenum::C};

    macro_rules! unit_aliases {
        ($(($m:literal, $kg:literal, $s:literal, $A:literal, $K:literal, $mol:literal, $cd:literal) -> $alias:ident,)+) => {
            $(pub type $alias =
                Unit<(m, C<$m>), (Pre<k, g>, C<$kg>), (s, C<$s>), (A, C<$A>), (K, C<$K>), (mol, C<$mol>), (cd, C<$cd>)>;)+
        };
    }

    unit_aliases! {
        (0, 0, 0, 0, 0, 0, 0) -> Dimensionless,
        //(1, 0, 0, 0, 0, 0, 0) -> Meter,
        //(0, 1, 0, 0, 0, 0, 0) -> Kilogram,
        //(0, 0, 1, 0, 0, 0, 0) -> Second,
        (0, 0, 0, 1, 0, 0, 0) -> Ampere,
        (0, 0, 0, 0, 1, 0, 0) -> Kelvin,
        (0, 0, 0, 0, 0, 1, 0) -> Mole,
        (0, 0, 0, 0, 0, 0, 1) -> Candela,
        (1, 0,-1, 0, 0, 0, 0) -> MeterPerSecond,
        //(2, 0, 0, 0, 0, 0, 0) -> MeterSquared,
    }
    pub type Meter = Unit<(meter, C<1>)>;
    pub type Kilometer = Unit<(Pre<kilo, meter>, C<1>)>;
    pub type MeterSquared = Unit<(meter, C<2>)>;
    pub type Second = Unit<(), (), (second, C<1>)>;
    pub type Kilogram = Unit<(), (Pre<kilo, gram>, C<1>)>;
}

#[allow(non_upper_case_globals)]
pub mod consts {
    use super::{
        unit::{Ampere, Candela, Kelvin, Kilogram, Kilometer, Meter, MeterSquared, Mole, Second},
        Unit,
    };
    use const_default::ConstDefault;

    pub const m: Meter = Unit::DEFAULT;
    pub const km: Kilometer = Unit::DEFAULT;
    pub const m2: MeterSquared = Unit::DEFAULT;
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
    ($type:ident, $feat:literal) => {
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
    ($type:ident<$($generic:ident),+>, $feat:literal) => {
        // 10 * km
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J, $($generic),+> ::core::ops::Mul<Unit<L, M, Ti, I, Te, N, J>> for $type<$($generic),+> {
            type Output = $crate::Quantity<Unit<L, M, Ti, I, Te, N, J>, $type<$($generic),+>>;

            fn mul(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }

        // 10 / km = 10 * km^(-1)
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J, $($generic),+> ::core::ops::Div<Unit<L, M, Ti, I, Te, N, J>> for $type<$($generic),+>
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
                $crate::Quantity<$crate::ops::Inverse<Unit<L, M, Ti, I, Te, N, J>>, $type<$($generic),+>>;

            fn div(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
}

impl_mul_div_for_value_by_unit!(f32, "f32");
impl_mul_div_for_value_by_unit!(f64, "f64");
impl_mul_div_for_value_by_unit!(i8, "i8");
impl_mul_div_for_value_by_unit!(u8, "u8");
impl_mul_div_for_value_by_unit!(i16, "i16");
impl_mul_div_for_value_by_unit!(u16, "u16");
impl_mul_div_for_value_by_unit!(i32, "i32");
impl_mul_div_for_value_by_unit!(u32, "u32");
impl_mul_div_for_value_by_unit!(i64, "i64");
impl_mul_div_for_value_by_unit!(u64, "u64");
impl_mul_div_for_value_by_unit!(i128, "i128");
impl_mul_div_for_value_by_unit!(u128, "u128");

#[cfg(test)]
mod tests {
    use super::{
        consts::{m, m2, s},
        unit::MeterPerSecond,
    };
    use crate::{
        isq::{consts::kg, unit::Meter},
        Quantity,
    };
    use nalgebra::{RowVector3, Vector3};

    #[test]
    fn nalgebra_vec() {
        let l1 = 12_f32 * m;
        let l2 = 1_f32 * m2;
        let l3 = l1 + l2;
        let v1 = Quantity::<Meter, _>::new(RowVector3::new(1, 2, -1));
        let v2 = Quantity::<Meter, _>::new(RowVector3::new(-1, -2, 1));
        println!("{}\n{}\n{}", v1.clone(), v2.clone(), v1 + v2);
        println!("{}", RowVector3::<f32>::default())
    }
}
