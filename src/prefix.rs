pub trait Prefix {
    const SHORT: &'static str;
    const LONG: &'static str;
    const BASE: u32;
    const EXP: i32;
}

pub struct NoPrefix;

impl Prefix for NoPrefix {
    const SHORT: &'static str = "";
    const LONG: &'static str = "";
    const BASE: u32 = 1;
    const EXP: i32 = 0;
}

macro_rules! prefixes {
    ($(($long:ident, $long_str:literal, $short:ident, $short_str:literal, $base:literal, $exp:literal)),+) => {
        $(//#[cfg(feature = $long_str)]
        #[allow(non_camel_case_types)]
        pub struct $short;

        //#[cfg(feature = $long_str)]
        #[allow(non_camel_case_types)]
        pub type $long = $short;

        //#[cfg(feature = $long_str)]
        impl $crate::Prefix for $short {
            const SHORT: &'static str = $short_str;
            const LONG: &'static str = $long_str;
            const BASE: u32 = $base;
            const EXP: i32 = $exp;
        }

        //#[cfg(feature = $long_str)]
        impl core::fmt::Display for $short {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::Prefix;

                Self::SHORT.fmt(formatter)
            }
        })+
    };
}

/// Decimal (metric) prefixes
pub mod dec {
    #[rustfmt::skip]
    prefixes! {
        (yocto, "yocto", y,  "y",  10, -24),
        (zepto, "zepto", z,  "z",  10, -21),
        (atto,  "atto",  a,  "a",  10, -18),
        (femto, "femto", f,  "f",  10, -15),
        (pico,  "pico",  p,  "p",  10, -12),
        (nano,  "nano",  n,  "n",  10,  -9),
        (micro, "micro", mu, "μ",  10,  -6),
        (milli, "milli", m,  "m",  10,  -3),
        (centi, "centi", c,  "c",  10,  -2),
        (deci,  "deci",  d,  "d",  10,  -1),
        (deca,  "deca",  da, "da", 10,   1),
        (hecto, "hecto", h,  "h",  10,   2),
        (kilo,  "kilo",  k,  "k",  10,   3),
        (mega,  "mega",  M,  "M",  10,   6),
        (giga,  "giga",  G,  "G",  10,   9),
        (tera,  "tera",  T,  "T",  10,  12),
        (peta,  "peta",  P,  "P",  10,  15),
        (exa,   "exa",   E,  "E",  10,  18),
        (zetta, "zetta", Z,  "Z",  10,  21),
        (yotta, "yotta", Y,  "Y",  10,  24)
    }
}

/// Binary prefixes
pub mod bin {
    #[rustfmt::skip]
    prefixes! {
        (kibi, "kibi", Ki, "Ki", 1024, 1),
        (mebi, "mebi", Mi, "Mi", 1024, 2),
        (gibi, "gibi", Gi, "Gi", 1024, 3),
        (tebi, "tebi", Ti, "Ti", 1024, 4),
        (pebi, "pebi", Pi, "Pi", 1024, 5),
        (exbi, "exbi", Ei, "Ei", 1024, 6),
        (zebi, "zebi", Zi, "Zi", 1024, 7),
        (yobi, "yobi", Yi, "Yi", 1024, 8)
    }
}
