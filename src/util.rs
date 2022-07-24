/// Repeat $tokens as many times as $_count repeats
macro_rules! repeat {
    ($_count:ident, $($tokens:tt)+) => {
        $($tokens)+
    };
}

pub(crate) use repeat;

macro_rules! count_idents {
    ($($identifier:ident),*) => {<[()]>::len(&[$($crate::util::repeat!($identifier, ())),*])};
}

pub(crate) use count_idents;

macro_rules! type_array {
    ($name:ident<$($param:ident = $default:ident),+>) => {
        pub struct $name<$($param = ($default, crate::typenum::Const<0>)),+>(::core::marker::PhantomData<($($param),+)>);

        impl<$($param),+> $name<$($param),+> {
            const LEN: usize = $crate::util::count_idents!($($param),+);

            #[allow(dead_code)]
            #[must_use]
            pub const fn new() -> Self {
                Self(::core::marker::PhantomData)
            }

            #[allow(dead_code)]
            #[must_use]
            pub const fn new_ref() -> &'static Self {
                &Self(::core::marker::PhantomData)
            }

            #[must_use]
            pub const fn len() -> usize {
                Self::LEN
            }
        }
    };
}

pub(crate) use type_array;

macro_rules! impl_unary_op_for_type_array {
    ($name:ident<$($type_param:ident),+>, $op:ident, $op_bound:ident) => {
        impl<$($type_param: $op_bound,)+> $op for $name<$($type_param),+> {
            type Output = $name<$(<$type_param as $op_bound>::Output,)+>;
        }
    };
}

pub(crate) use impl_unary_op_for_type_array;

macro_rules! impl_binary_op_for_type_array {
    ($name:ident<$($param:ident),+>, $op:ident, $op_bound:ident) => {
        paste::paste! {
            impl<$( [<$param 1>] : $op_bound< [<$param 2>] >,)+ $( [<$param 2>] ),+> $op<$name<$( [<$param 2>] ),+>> for $name<$( [<$param 1>] ),+> {
                type Output = $name<$(<[<$param 1>] as $op_bound<[<$param 2>]>>::Output,)+>;

                fn [<$op:lower>](self, _: $name<$( [<$param 2>] ),+>) -> Self::Output {
                    Self::Output::new()
                }
            }
        }
    };
}

pub(crate) use impl_binary_op_for_type_array;

/// Generate aliases for unary operators' output type
/// E. g. `type NegOut<L> = <L as Neg>::Output`
macro_rules! unary_ops_out_aliases {
    ($($op:ident -> $out:ident,)+) => {
        $(pub type $out<L> = <L as $op>::Output;)+
    };
}

pub(crate) use unary_ops_out_aliases;

/// Generate aliases for binary operators' output type
/// E. g. `type Sum<L, R> = <L as Add<R>>::Output`
macro_rules! binary_ops_out_aliases {
    ($($op:ident -> $out:ident,)+) => {
        $(pub type $out<L, R> = <L as $op<R>>::Output;)+
    };
}

pub(crate) use binary_ops_out_aliases;

macro_rules! trait_alias {
    // single alias
    (($first_trait:ident, $($trait:ident),*) -> $alias:ident) => {
        pub trait $alias: $first_trait $(+ $trait),* {}
        impl<T: $first_trait $(+ $trait),*> $alias for T {}
    };

    // multiple aliases
    ($(($first_trait:ident, $($trait:ident),*) -> $alias:ident;),+) => {
        $(trait_alias!($first_trait:ident, $($trait:ident),* -> $alias:ident)),+
    }
}

pub(crate) use trait_alias;

use crate::{
    base_unit::Pre,
    isq::{prefix::kilo, root::gram},
};

#[cfg(test)]
mod tests {
    #[test]
    fn count_idents() {
        assert_eq!(count_idents!(A, B, C, D), 4);
    }
}
