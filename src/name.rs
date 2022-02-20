use crate::util::ConstStr;

/// Short & full names
pub trait Name {
    const SHORT: &'static str;
    const FULL: &'static str;
}

/// Buffer to store a name inside `ConstStr`
pub(crate) trait NameBuf<const S: usize, const F: usize> {
    const SHORT_BUF: ConstStr<S>;
    const FULL_BUF: ConstStr<F>;
}
