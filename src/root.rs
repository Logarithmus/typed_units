use crate::Name;

/// Base unit without prefix, e. g. meter, second
pub trait Root: Name {}

macro_rules! roots {
    ($(($full:ident, $full_str:literal, $short:ident, $short_str:literal)),+) => {
        $(#[allow(non_camel_case_types)]
        pub struct $short;

        #[allow(non_camel_case_types)]
        pub type $full = $short;

        impl $crate::Root for $full {}

        impl $crate::Name for $short {
            const SHORT: &'static str = $short_str;
            const FULL: &'static str = $full_str;
        }

        impl core::fmt::Display for $short {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::SHORT.fmt(f)
            }
        })+
    };
}

pub(crate) use roots;
