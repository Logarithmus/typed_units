

/// Short & full names
pub trait Name {
    /// Short name
    const SHORT: &'static str;
    /// Full name
    const FULL: &'static str;
}

pub trait Display {
    fn display() -> String;
}

impl<N: Name> Display for N {
    fn display() -> String {
        N::SHORT.to_string()
    }
}

pub trait Debug {
    fn debug() -> String;
}

impl<N: Name> Debug for N {
    fn debug() -> String {
        N::FULL.to_string()
    }
}

pub fn superscript(num: i32) -> String {
    let s = num.to_string();
    s.bytes()
        .map(|c| match c {
            b'0' => '⁰',
            b'1' => '¹',
            b'2' => '²',
            b'3' => '³',
            b'4' => '⁴',
            b'5' => '⁵',
            b'6' => '⁶',
            b'7' => '⁷',
            b'8' => '⁸',
            b'9' => '⁹',
            b'-' => '⁻',
            _ => unreachable!(),
        })
        .collect()
}
