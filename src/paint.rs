use core::fmt;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::{String, ToString};

use crate::{Color, Attribute, Quirk, Style, Condition};

/// An arbitrary value with a [`Style`] applied to it.
#[derive(Copy, Clone)]
pub struct Painted<T> {
    /// The value to be styled.
    pub value: T,
    /// The style to apply.
    pub style: Style,
}

/// A trait to apply styling to any value, implemented for all types.
///
/// See the [crate level docs](crate#usage) for usage details.
pub trait Paint {
    /// Create a new [`Painted`] with a default [`Style`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// let painted = Paint::new("hello");
    /// ```
    #[inline(always)]
    fn new(self) -> Painted<Self> where Self: Sized {
        Painted::new(self)
    }

    #[doc(hidden)]
    #[inline(always)]
    fn apply(&self, a: crate::style::Application) -> Painted<&Self> {
        Painted::new(self).apply(a)
    }

    /// Apply a style wholesale to `self`. Any previous style is replaced.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::{Paint, Style, Color::*};
    ///
    /// static DEBUG: Style = Black.bold().on_yellow();
    ///
    /// let painted = "hello".paint(DEBUG);
    /// ```
    #[inline(always)]
    fn paint<S: Into<Style>>(&self, style: S) -> Painted<&Self> {
        Painted { value: self, style: style.into() }
    }

    properties!(signature(&Self) -> Painted<&Self>);
}

impl<T: ?Sized> Paint for T {
    properties!(constructor(&Self) -> Painted<&Self>);
}

impl<T> Painted<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Painted<T> {
        Painted { value, style: Style::new() }
    }

    #[inline(always)]
    const fn apply(mut self, a: crate::style::Application) -> Self {
        self.style = self.style.apply(a);
        self
    }

    properties!([pub const] constructor(Self) -> Self);
}

impl<T> Painted<T> {
    pub(crate) fn color_fmt_value(
        &self,
        fmt: &dyn Fn(&T, &mut fmt::Formatter) -> fmt::Result,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        self.style.fmt_prefix(f)?;
        fmt(&self.value, f)?;
        self.style.fmt_suffix(f)
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn clear_fmt_args(
        &self,
        fmt: &dyn Fn(&T, &mut fmt::Formatter) -> fmt::Result,
        f: &mut fmt::Formatter,
        args: &fmt::Arguments<'_>,
    ) -> fmt::Result {
        // A tiny state machine to find escape sequences.
        enum State { Searching, Open, }
        let mut state = State::Searching;
        let escape = |c: char| {
            match state {
                State::Searching if c == '\x1B' => { state = State::Open; true }
                State::Open if c == 'm' => { state = State::Searching; true }
                State::Searching => false,
                State::Open => true,
            }
        };

        // Only replace when the string contains styling.
        let string = args.to_string();
        if string.contains('\x1B') {
            f.write_str(&string.replace(escape, ""))
        } else {
            fmt(&self.value, f)
        }
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn color_wrap_fmt_args(
        &self,
        fmt: &dyn Fn(&T, &mut fmt::Formatter) -> fmt::Result,
        f: &mut fmt::Formatter,
        args: &fmt::Arguments<'_>,
    ) -> fmt::Result {
        // Only replace when the string contains styling.
        let string = args.to_string();
        if !string.contains('\x1B') {
            return self.color_fmt_value(fmt, f);
        }

        // Compute the prefix for the style with a reset in front.
        let mut prefix = String::new();
        prefix.push_str("\x1B[0m");
        self.style.fmt_prefix(&mut prefix)?;

        // Write out formatted string, replacing resets with computed prefix.
        self.style.fmt_prefix(f)?;
        write!(f, "{}", args.to_string().replace("\x1B[0m", &prefix))?;
        self.style.fmt_suffix(f)
    }

    pub(crate) fn fmt_args(
        &self,
        fmt: &dyn Fn(&T, &mut fmt::Formatter) -> fmt::Result,
        f: &mut fmt::Formatter,
        _args: fmt::Arguments<'_>,
    ) -> fmt::Result {
        let masked = self.style.quirks.contains(Quirk::Mask);
        let enabled = crate::is_enabled()
            && (self.style.condition)()
            && crate::windows::cache_enable();

        #[cfg(not(feature = "alloc"))]
        match (enabled, masked) {
            (true, _) => self.color_fmt_value(fmt, f),
            (false, false) => fmt(&self.value, f),
            (false, true) => Ok(()),
        };

        #[cfg(feature = "alloc")]
        match (enabled, masked, self.style.quirks.contains(Quirk::Wrap)) {
            (true, _, true) => self.color_wrap_fmt_args(fmt, f, &_args),
            (true, _, false) => self.color_fmt_value(fmt, f),
            (false, false, true) => self.clear_fmt_args(fmt, f, &_args),
            (false, false, false) => fmt(&self.value, f),
            (false, true, _) => Ok(()),
        }
    }
}

impl<T> From<Painted<T>> for Style {
    fn from(painted: Painted<T>) -> Self {
        painted.style
    }
}
