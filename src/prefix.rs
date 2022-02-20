use crate::Name;

pub trait Prefix: Name {
    const BASE: u32;
    const EXP: i32;
}

macro_rules! prefixes {
    ($(($full:ident, $full_str:literal, $short:ident, $short_str:literal, $base:literal, $exp:literal)),+) => {
        $(//#[cfg(feature = $full_str)]
        #[allow(non_camel_case_types)]
        pub struct $short;

        //#[cfg(feature = $full_str)]
        #[allow(non_camel_case_types)]
        pub type $full = $short;

        //#[cfg(feature = $full_str)]
        impl $crate::Name for $short {
            const SHORT: &'static str = $short_str;
            const FULL: &'static str = $full_str;
        }

        //#[cfg(feature = $full_str)]
        impl $crate::Prefix for $short {
            const BASE: u32 = $base;
            const EXP: i32 = $exp;
        }

        //#[cfg(feature = $full_str)]
        impl ::core::fmt::Display for $short {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::Name>::SHORT.fmt(formatter)
            }
        })+
    };
}

pub(crate) use prefixes;
