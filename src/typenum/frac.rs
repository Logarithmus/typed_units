//! This module should be upstreamed to <https://lib.rs/typenum>

use crate::{
    typenum::{Lcm, LcmOp},
    util::binary_ops_out_aliases,
};
use core::ops::{Add, Div, Mul};
use typenum::{Prod, Quot, Sum};

/// Operator to add 2 fractions of type-level numbers
///
/// We can't use `core::ops::Add` because both `core::ops::Add` & `(N, D)` are foreign to our crate.
///
/// We could wrap `(N, D)` into newtype struct like `struct Frac(N, D)`, but it would make
/// `uom-ng` compile errors more verbose.
pub trait FracAdd<Rhs> {
    type Output;
}

binary_ops_out_aliases! {
    FracAdd -> FracSum,
}

/// **TLDR;** `Sum<(Nl, Dl), (Nr, Dr)> == (Nl / Dl) + (Nr / Dr)`
///
/// * `N` -- numerator
/// * `D` -- denominator
/// * `l` -- left-hand side
/// * `r` -- right-hand-side
impl<Nl, Nr, Dl: Mul<Dr> + LcmOp<Dr>, Dr> FracAdd<(Nr, Dr)> for (Nl, Dl)
where
    Lcm<Dl, Dr>: Div<Dl> + Div<Dr>,
    Nl: Mul<Quot<Lcm<Dl, Dr>, Dl>>,
    Nr: Mul<Quot<Lcm<Dl, Dr>, Dr>>,
    Prod<Nl, Quot<Lcm<Dl, Dr>, Dl>>: Add<Prod<Nr, Quot<Lcm<Dl, Dr>, Dr>>>,
    Sum<Prod<Nl, Quot<Lcm<Dl, Dr>, Dl>>, Prod<Nr, Quot<Lcm<Dl, Dr>, Dr>>>: Div<Lcm<Dl, Dr>>,
{
    /// Sum of 2 fractions
    type Output = <(Nl, Dl) as hidden::FracAddImpl<(Nr, Dr)>>::Output;
}

mod hidden {
    use crate::typenum::{Lcm, LcmOp};
    use core::ops::{Add, Div, Mul};
    use typenum::{Prod, Quot, Sum};

    pub trait FracAddImpl<Rhs> {
        /// Least common denominator
        type Lcd;
        /// Numerator of the sum of 2 fractions
        type NOut;
        /// Sum of 2 fractions
        type Output;
    }

    impl<Nl, Nr, Dl: Mul<Dr> + LcmOp<Dr>, Dr> FracAddImpl<(Nr, Dr)> for (Nl, Dl)
    where
        Lcm<Dl, Dr>: Div<Dl> + Div<Dr>,
        Nl: Mul<Quot<Lcm<Dl, Dr>, Dl>>,
        Nr: Mul<Quot<Lcm<Dl, Dr>, Dr>>,
        Prod<Nl, Quot<Lcm<Dl, Dr>, Dl>>: Add<Prod<Nr, Quot<Lcm<Dl, Dr>, Dr>>>,
        Sum<Prod<Nl, Quot<Lcm<Dl, Dr>, Dl>>, Prod<Nr, Quot<Lcm<Dl, Dr>, Dr>>>: Div<Lcm<Dl, Dr>>,
    {
        type Lcd = Lcm<Dl, Dr>;
        type NOut = Sum<Prod<Nl, Quot<Self::Lcd, Dl>>, Prod<Nr, Quot<Self::Lcd, Dr>>>;
        type Output = Quot<Self::NOut, Self::Lcd>;
    }
}
