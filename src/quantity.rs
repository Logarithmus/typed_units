use crate::base_unit::ConvertFrom;
use const_default::ConstDefault;
use core::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::op;

pub struct Quantity<U, V> {
    pub value: V,
    phantom: PhantomData<U>,
}

impl<U, V: Clone> Clone for Quantity<U, V> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<U, V: Copy> Copy for Quantity<U, V> {}

impl<U: Debug + ConstDefault, V: Debug> Debug for Quantity<U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.value, U::DEFAULT)
    }
}

impl<U, V: PartialEq> PartialEq for Quantity<U, V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U, V: Eq> Eq for Quantity<U, V> {}

impl<U, V: PartialOrd> PartialOrd for Quantity<U, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U, V: Ord> Ord for Quantity<U, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<U, V> Quantity<U, V> {
    #[must_use]
    pub const fn new(value: V) -> Self {
        Self { value, phantom: PhantomData }
    }

    pub fn from<Uother>(other: Quantity<Uother, V>) -> Self
    where
        U: ConvertFrom<Uother, V>,
    {
        Self::new(U::convert_from(other.value))
    }

    pub fn into<Uother: ConvertFrom<U, V>>(self) -> Quantity<Uother, V> {
        Quantity::new(Uother::convert_from(self.value))
    }
}

impl<U, V: Default> Default for Quantity<U, V> {
    fn default() -> Self {
        Self::new(Default::default())
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

impl<U: Display + ConstDefault, V: Display> Display for Quantity<U, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::DEFAULT)
    }
}

#[cfg(test)]
mod tests {
    use crate::isq::consts::*;

    #[test]
    fn add_quantity_to_quantity() {
        let v1 = 10_f32 * (m / s);
        let v2 = 3_f32 * (m / s);
        let volume = 100_i32 * (m * m * m);
        let density = 1000_i32 * (kg / (m * m * m));
        let destiny = 1000_i32 * (kg / (m * m * m) / kg);
        let destiny2 = 1000_i32 * (kg / (m * m * m) / s / s);
        println!("{}\n{}\n{}\n{}\n{}\n{}", v1, v2, volume, density, destiny, destiny2);
        println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", v1, v2, volume, density, destiny, destiny2);
        assert_eq!(v1 + v2, 13_f32 * (m / s));
    }

    #[test]
    fn sub_quantity_from_quantity() {
        let v1 = 10_f32 * (m / s);
        let v2 = 3_f32 * (m / s);
        assert_eq!(v1 - v2, 7_f32 * (m / s));
    }

    #[test]
    fn mul_quantity_by_quantity() {
        let speed = 10_f32 * (m / s);
        let time = 3_f32 * s;
        assert_eq!(speed * time, 30_f32 * m);
    }

    #[test]
    fn div_quantity_by_quantity() {
        let distance = 21_f32 * m;
        let time = 3_f32 * s;
        assert_eq!(distance / time, 7_f32 * (m / s));
    }
}
