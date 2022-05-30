use crate::Name;

/// Base unit without prefix, e. g. meter, second
pub trait Root: Name {}

macro_rules! roots {
    ($(($full:ident, $full_str:literal, $short_str:literal),)+) => {
        $(#[allow(non_camel_case_types)]
        pub struct $full;

        impl $crate::Root for $full {}

        impl $crate::Name for $full {
            const SHORT: &'static str = $short_str;
            const FULL: &'static str = $full_str;
        }

        impl const_default::ConstDefault for $full {
            const DEFAULT: Self = Self;
        }

        impl core::fmt::Display for $full {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::SHORT.fmt(f)
            }
        }

        impl core::fmt::Debug for $full {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::FULL.fmt(f)
            }
        })+
    };
}

pub(crate) use roots;

macro_rules! roots_with_alias {
    ($(($full:ident, $full_str:literal, $short:ident, $short_str:literal),)+) => {
        crate::root::roots! {
            $(($full, $full_str, $short_str),)+
        }

        $(#[allow(non_camel_case_types)]
        pub type $short = $full;)+

    };
}

pub(crate) use roots_with_alias;
