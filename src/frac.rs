use crate::ops::{Sum, Quotient, Product};
use core::ops::{Add, Div, Mul};
use typenum::{Abs, AbsVal, Gcd, Gcf};

pub trait LcmOp<Rhs>: Gcd<Rhs> {
    type Output;
}

pub type Lcm<L, R> = <L as LcmOp<R>>::Output;

impl<L: Gcd<R> + Mul<R>, R> LcmOp<R> for L
where
    Product<L, R>: Abs,
    AbsVal<Product<L, R>>: Div<Gcf<L, R>>,
{
    type Output = Quotient<AbsVal<Product<L, R>>, Gcf<L, R>>;
}

pub trait FracAddOp<Fr> {
    type Lcm;
    type NOut;
    type Output;
}

impl<Nl, Nr, Dl: Mul<Dr> + LcmOp<Dr>, Dr> FracAddOp<(Nr, Dr)> for (Nl, Dl)
where
    Lcm<Dl, Dr>: Div<Dl> + Div<Dr>,
    Nl: Mul<Quotient<Lcm<Dl, Dr>, Dl>>,
    Nr: Mul<Quotient<Lcm<Dl, Dr>, Dr>>,
    Product<Nl, Quotient<Lcm<Dl, Dr>, Dl>>: Add<Product<Nr, Quotient<Lcm<Dl, Dr>, Dr>>>,
    Sum<Product<Nl, Quotient<Lcm<Dl, Dr>, Dl>>, Product<Nr, Quotient<Lcm<Dl, Dr>, Dr>>>:
        Div<Lcm<Dl, Dr>>,
{
    type Lcm = Lcm<Dl, Dr>;
    type NOut = Sum<Product<Nl, Quotient<Self::Lcm, Dl>>, Product<Nr, Quotient<Self::Lcm, Dr>>>;
    type Output = Quotient<Self::NOut, Self::Lcm>;
}

type FracAdd<L, R> = <L as FracAddOp<R>>::Output;
