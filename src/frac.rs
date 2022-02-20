use crate::ops::{AddOut, DivOut, MulOut};
use core::ops::{Add, Div, Mul};
use typenum::{Abs, AbsVal, Gcd, Gcf};

pub trait LcmOp<Rhs>: Gcd<Rhs> {
    type Output;
}

pub type Lcm<L, R> = <L as LcmOp<R>>::Output;

impl<L: Gcd<R> + Mul<R>, R> LcmOp<R> for L
where
    MulOut<L, R>: Abs,
    AbsVal<MulOut<L, R>>: Div<Gcf<L, R>>,
{
    type Output = DivOut<AbsVal<MulOut<L, R>>, Gcf<L, R>>;
}

pub trait FracAddOp<Fr> {
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
