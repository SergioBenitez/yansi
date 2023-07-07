use core::fmt::*;

use crate::*;

pub struct PaintedLink<T> {
    painted: Painted<T>,
    link: String,
}

pub trait HyperlinkExt {
    fn link(&self, url: impl ToString) -> PaintedLink<&Self>;
}

impl<T: Display> Display for PaintedLink<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.painted.style.fmt_prefix(f)?;
        write!(f, "\x1B]8;;{}\x1B\\", self.link)?;
        self.painted.value.fmt(f)?;
        write!(f, "\x1B]8;;\x1B\\")?;
        self.painted.style.fmt_suffix(f)
    }
}

impl<T> HyperlinkExt for T {
    fn link(&self, url: impl ToString) -> PaintedLink<&Self> {
        PaintedLink { painted: Painted::new(self), link: url.to_string() }
    }
}

impl<T> Painted<T> {
    pub fn link(&self, url: impl ToString) -> PaintedLink<&Self> {
        PaintedLink { painted: Painted::new(self), link: url.to_string() }
    }
}

impl<T> PaintedLink<T> {
    #[inline(always)]
    const fn apply(mut self, a: crate::style::Application) -> Self {
        self.painted.style = self.painted.style.apply(a);
        self
    }

    properties!([pub const] constructor(Self) -> Self);
}
