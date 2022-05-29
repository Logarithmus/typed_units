use crate::Name;

pub trait Prefix<V>: Name {
    const BASE: V;
    const EXP: V;
}

macro_rules! impl_prefix {
    ($prefix:ident, $base:literal, $exp:literal, $type:ident) => {
        impl $crate::Prefix<$type> for $prefix {
            const BASE: $type = $base;
            const EXP: $type = $exp;
        }
    };
}

pub(crate) use impl_prefix;

macro_rules! prefixes {
    ($(($full:ident, $full_str:literal, $short:ident, $short_str:literal, $base:literal, $exp:literal),)+) => {
        $(//#[cfg(feature = $full_str)]
        #[allow(non_camel_case_types)]
        pub struct $full;

        //#[cfg(feature = $full_str)]
        #[allow(non_camel_case_types)]
        pub type $short = $full;

        //#[cfg(feature = $full_str)]
        impl $crate::Name for $full {
            const SHORT: &'static str = $short_str;
            const FULL: &'static str = $full_str;
        }

        // #[cfg(feature = $full_str)]
        // crate::prefix::impl_prefix!($full, $base, $exp, i8);
        // crate::prefix::impl_prefix!($full, $base, $exp, u8);
        crate::prefix::impl_prefix!($full, $base, $exp, i16);
        // crate::prefix::impl_prefix!($full, $base, $exp, u16);
        crate::prefix::impl_prefix!($full, $base, $exp, i32);
        // crate::prefix::impl_prefix!($full, $base, $exp, u32);
        crate::prefix::impl_prefix!($full, $base, $exp, i64);
        // crate::prefix::impl_prefix!($full, $base, $exp, u64);
        crate::prefix::impl_prefix!($full, $base, $exp, i128);
        // crate::prefix::impl_prefix!($full, $base, $exp, u128);
        crate::prefix::impl_prefix!($full, $base, $exp, isize);
        // crate::prefix::impl_prefix!($full, $base, $exp, usize);

        impl const_default::ConstDefault for $full {
            const DEFAULT: Self = Self;
        }

        //#[cfg(feature = $full_str)]
        impl ::core::fmt::Display for $full {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::SHORT.fmt(f)
            }
        }

        impl ::core::fmt::Debug for $full {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::FULL.fmt(f)
            }
        })+
    };
}

pub(crate) use prefixes;
