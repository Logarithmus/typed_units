use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use derivative::Derivative;
use typenum::op;

#[derive(Derivative)]
#[derivative(Debug, PartialEq, PartialOrd, Ord)]
pub struct Quantity<U, V> {
    pub value: V,

    #[derivative(
        Debug = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    phantom: PhantomData<U>,
}

impl<U, V: Eq> Eq for Quantity<U, V> {}

impl<U, V> Quantity<U, V> {
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Ul: Mul<Ur>, Ur, V: Mul<Output = V>> Mul<Quantity<Ur, V>> for Quantity<Ul, V> {
    type Output = Quantity<op!(Ul * Ur), V>;

    fn mul(self, rhs: Quantity<Ur, V>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<Ul: Div<Ur>, Ur, V: Div<Output = V>> Div<Quantity<Ur, V>> for Quantity<Ul, V> {
    type Output = Quantity<op!(Ul / Ur), V>;

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

#[cfg(test)]
mod tests {
    use crate::isq::consts::*;

    #[test]
    fn mul_quantity_by_quantity() {
        let speed = 10_f32 * (m / s);
        let time = 3_f32 * s;
        let distance = speed * time;
        assert_eq!(distance, 30_f32 * m);
    }

    #[test]
    fn div_quantity_by_quantity() {
        let distance = 21_f32 * m;
        let time = 3_f32 * s;
        let speed = distance / time;
        assert_eq!(speed, 7_f32 * (m / s));
    }
}
