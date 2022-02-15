use crate::unit::isq::{Dimensionless, Unit};
use crate::unit::{DivOut, MulOut, UnitDiv, UnitNeg};
use core::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use typenum::operator_aliases::Negate;

pub struct Quantity<U, V> {
    value: V,
    phantom: PhantomData<U>,
}

impl<U, V> Quantity<U, V> {
    pub const fn new(value: V) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

macro_rules! impl_ops_for_unit {
    ($type:ty, $feat:literal) => {
        // 10 * km
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J> ::core::ops::Mul<Unit<L, M, Ti, I, Te, N, J>> for $type {
            type Output = Quantity<Unit<L, M, Ti, I, Te, N, J>, $type>;

            fn mul(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }

        // 10 / km = 10 * km^(-1)
        //#[cfg(feature = $feat)]
        impl<L, M, Ti, I, Te, N, J> ::core::ops::Div<Unit<L, M, Ti, I, Te, N, J>> for $type
        where
            L: UnitNeg,
            M: UnitNeg,
            Ti: UnitNeg,
            I: UnitNeg,
            Te: UnitNeg,
            N: UnitNeg,
            J: UnitNeg,
        {
            type Output = Quantity<Negate<Unit<L, M, Ti, I, Te, N, J>>, $type>;

            fn div(self, _: Unit<L, M, Ti, I, Te, N, J>) -> Self::Output {
                Self::Output::new(self)
            }
        }
    };
}

impl_ops_for_unit!(f32, "f32");
impl_ops_for_unit!(f64, "f64");
impl_ops_for_unit!(i8, "18");
impl_ops_for_unit!(u8, "u8");
impl_ops_for_unit!(i16, "i16");
impl_ops_for_unit!(u16, "u16");
impl_ops_for_unit!(i32, "i32");
impl_ops_for_unit!(u32, "u32");
impl_ops_for_unit!(i64, "i64");
impl_ops_for_unit!(u64, "u64");
impl_ops_for_unit!(i128, "i128");
impl_ops_for_unit!(u128, "u128");

impl<Ul: Mul<Ur>, Ur, V: Mul<Output = V>> Mul<Quantity<Ur, V>> for Quantity<Ul, V> {
    type Output = Quantity<MulOut<Ul, Ur>, V>;

    fn mul(self, rhs: Quantity<Ur, V>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<Ul: Div<Ur>, Ur, V: Div<Output = V>> Div<Quantity<Ur, V>> for Quantity<Ul, V> {
    type Output = Quantity<DivOut<Ul, Ur>, V>;

    fn div(self, rhs: Quantity<Ur, V>) -> Self::Output {
        Self::Output::new(self.value / rhs.value)
    }
}

impl<U, V: Add<Output = V>> Add for Quantity<U, V> {
    type Output = Self;

    fn add(self, rhs: Quantity<U, V>) -> Self::Output {
        Self::Output::new(self.value + rhs.value)
    }
}

impl<U, V: Sub<Output = V>> Sub for Quantity<U, V> {
    type Output = Self;

    fn sub(self, rhs: Quantity<U, V>) -> Self::Output {
        Self::Output::new(self.value - rhs.value)
    }
}
