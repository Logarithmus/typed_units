use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use derivative::Derivative;
use typenum::{op, Prod, Quot};

use crate::{
    ops::{Downcast, Downcasted, Upcast, Upcasted},
    unit::SimpleUnit,
};

#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantity<U, V> {
    pub value: V,

    #[derivative(
        Debug = "ignore",
        PartialEq = "ignore",
        // `Eq` is a marker trait, it doesn't care about struct contents, so no "ignore" needed
        Eq,
        PartialOrd = "ignore",
        Ord = "ignore"
    )]
    phantom: PhantomData<U>,
}

impl<U, V> Quantity<U, V> {
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Ul, Ur, V> Mul<Quantity<Ur, V>> for Quantity<Ul, V>
where
    Ul: Upcast,
    Ur: Upcast,
    V: Mul<Output = V>,
    Upcasted<Ul>: Mul<Upcasted<Ur>>,
    Prod<Upcasted<Ul>, Upcasted<Ur>>: Downcast,
{
    type Output = Quantity<Downcasted<Prod<Upcasted<Ul>, Upcasted<Ur>>>, V>;

    fn mul(self, rhs: Quantity<Ur, V>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<Ul, Ur, V> Div<Quantity<Ur, V>> for Quantity<Ul, V>
where
    Ul: Upcast,
    Ur: Upcast,
    V: Div<Output = V>,
    Upcasted<Ul>: Div<Upcasted<Ur>>,
    Quot<Upcasted<Ul>, Upcasted<Ur>>: Downcast,
{
    type Output = Quantity<Downcasted<Quot<Upcasted<Ul>, Upcasted<Ur>>>, V>;

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
        let time = 3_f32 * (s * s / s);
        let distance = speed * time;
        assert_eq!(distance, 30_f32 * m);
    }
}
