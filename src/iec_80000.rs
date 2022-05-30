/// Binary prefixes
pub mod prefix {
    use crate::prefix::prefixes;

    #[rustfmt::skip]
    prefixes! {
        (kibi, "kibi", Ki, "Ki", 1024, 1),
        (mebi, "mebi", Mi, "Mi", 1024, 2),
        (gibi, "gibi", Gi, "Gi", 1024, 3),
        (tebi, "tebi", Ti, "Ti", 1024, 4),
        (pebi, "pebi", Pi, "Pi", 1024, 5),
        (exbi, "exbi", Ei, "Ei", 1024, 6),
        (zebi, "zebi", Zi, "Zi", 1024, 7),
        (yobi, "yobi", Yi, "Yi", 1024, 8),
    }
}

/// Base units without prefix
pub mod root {
    use crate::root::roots_with_alias;

    #[rustfmt::skip]
    roots_with_alias! {
        (Byte, "byte", B, "B"),
        (Bit,  "bit", bit, "bit"),
    }
}
