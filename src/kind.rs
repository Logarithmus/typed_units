use crate::unit::BaseUnit;

pub trait Length: BaseUnit {}
pub trait Mass: BaseUnit {}
pub trait Time: BaseUnit {}
pub trait Current: BaseUnit {}
pub trait Temperature: BaseUnit {}
pub trait AmountOfSubstance: BaseUnit {}
pub trait LuminousIntensity: BaseUnit {}
pub trait Angle: BaseUnit {}
pub trait Ratio: BaseUnit {}

// impl kind::Length for root::meter {}
// impl kind::Mass for root::gram {}
// impl kind::Time for root::second {}
// impl kind::Current for root::ampere {}
// impl kind::Temperature for root::kelvin {}
// impl kind::AmountOfSubstance for root::mole {}
// impl kind::LuminousIntensity for root::candela {}

// impl<P: Prefix, R: Root + kind::Length> kind::Length for (P, R) {}
// impl<P: Prefix, R: Root + kind::Mass> kind::Mass for (P, R) {}
// impl<P: Prefix, R: Root + kind::Time> kind::Time for (P, R) {}
// impl<P: Prefix, R: Root + kind::Current> kind::Current for (P, R) {}
// impl<P: Prefix, R: Root + kind::Temperature> kind::Temperature for (P, R) {}
// impl<P: Prefix, R: Root + kind::AmountOfSubstance> kind::AmountOfSubstance for (P, R) {}
// impl<P: Prefix, R: Root + kind::LuminousIntensity> kind::LuminousIntensity for (P, R) {}
