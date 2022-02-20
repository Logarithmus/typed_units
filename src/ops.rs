use crate::util::{binary_ops_out_aliases, unary_ops_out_aliases};
use core::ops::{Add, Div, Mul, Sub};
use num_traits::Inv;
use typenum::Cmp;

unary_ops_out_aliases!(Inv);
binary_ops_out_aliases!(Add, Sub, Mul, Div, Cmp);
