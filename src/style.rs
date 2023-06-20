use core::fmt::{self, Write};

use crate::color::{Color, Variant};
use crate::attribute::{Attribute, Quirk};
use crate::condition::Condition;
use crate::set::Set;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::String, borrow::Cow};

#[cfg(feature = "std")]
use std::borrow::Cow;

/// A set of styling options.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style {
    pub(crate) foreground: Option<Color>,
    pub(crate) background: Option<Color>,
    pub(crate) attributes: Set<Attribute>,
    pub(crate) quirks: Set<Quirk>,
    pub(crate) condition: Condition,
}

struct AnsiSplicer<'a> {
    f: &'a mut dyn fmt::Write,
    splice: bool,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Application {
    fg(Color),
    bg(Color),
    attr(Attribute),
    quirk(Quirk),
    whenever(Condition),
}

impl Style {
    const DEFAULT: Style = Style {
        foreground: None,
        background: None,
        attributes: Set::EMPTY,
        quirks: Set::EMPTY,
        condition: Condition::ALWAYS,
    };

    /// Returns a new style with no foreground or background, no attributes
    /// or quirks, and an [`ALWAYS`](Condition::ALWAYS) condition.
    ///
    /// This is the default.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// assert_eq!(Style::new(), Style::default());
    /// ```
    #[inline]
    pub const fn new() -> Style {
        Style::DEFAULT
    }

    #[inline(always)]
    pub(crate) const fn apply(mut self, a: Application) -> Style {
        match a {
            Application::fg(color) => self.foreground = Some(color),
            Application::bg(color) => self.background = Some(color),
            Application::whenever(cond) => self.condition = cond,
            Application::attr(attr) => self.attributes = self.attributes.union(attr),
            Application::quirk(quirk) => self.quirks = self.quirks.union(quirk),
        }

        self
    }

    /// Writes the ANSI code prefix for the currently set styles.
    ///
    /// This method is intended to be used inside of [`fmt::Display`] and
    /// [`fmt::Debug`] implementations for custom or specialized use-cases. Most
    /// users should use [`Painted`] for all painting needs.
    ///
    /// This method writes the ANSI code prefix irrespective of whether painting
    /// is currently enabled or disabled. To write the prefix only if painting
    /// is enabled, condition a call to this method on [`is_enabled()`].
    ///
    /// [`fmt::Display`]: fmt::Display
    /// [`fmt::Debug`]: fmt::Debug
    /// [`Painted`]: crate::Painted
    /// [`is_enabled()`]: crate::is_enabled()
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::fmt;
    /// use yansi::Style;
    ///
    /// struct CustomItem {
    ///     item: u32,
    ///     style: Style
    /// }
    ///
    /// impl fmt::Display for CustomItem {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         self.style.fmt_prefix(f)?;
    ///         write!(f, "number: {}", self.item)?;
    ///         self.style.fmt_suffix(f)
    ///     }
    /// }
    /// ```
    pub fn fmt_prefix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        // Give a sequence-free string when no styles are applied.
        if self == &Style::DEFAULT {
            return Ok(());
        }

        let brighten = |color: Option<Color>, bright: bool| match (color, bright) {
            (Some(color), true) => Some(color.to_bright()),
            _ => color
        };

        let mut f = AnsiSplicer { f, splice: false };
        f.write_str("\x1B[")?;

        for attr in self.attributes.iter() {
            f.splice()?;
            attr.fmt(&mut f)?;
        }

        if let Some(color) = brighten(self.background, self.quirks.contains(Quirk::OnBright)) {
            f.splice()?;
            color.fmt(&mut f, Variant::Bg)?;
        }

        if let Some(color) = brighten(self.foreground, self.quirks.contains(Quirk::Bright)) {
            f.splice()?;
            color.fmt(&mut f, Variant::Fg)?;
        }

        // All of the sequences end with an `m`.
        f.write_char('m')
    }

    /// Returns the ANSI code sequence prefix for the style as a string.
    ///
    /// This returns a string with the exact same sequence written by
    /// [`fmt_prefix()`](Self::fmt_prefix()). See that method for details.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "_nightly", doc(cfg(feature = "alloc")))]
    pub fn prefix_seq(&self) -> Cow<'static, str> {
        let mut prefix = String::new();
        let _ = self.fmt_prefix(&mut prefix);
        prefix.into()
    }

    /// Writes the ANSI code sequence suffix for the style.
    ///
    /// This method is intended to be used inside of [`fmt::Display`] and
    /// [`fmt::Debug`] implementations for custom or specialized use-cases. Most
    /// users should use [`Painted`] for all painting needs.
    ///
    /// This method writes the ANSI code suffix irrespective of whether painting
    /// is currently enabled or disabled. To write the suffix only if painting
    /// is enabled, condition a call to this method on [`is_enabled()`].
    ///
    /// [`fmt::Display`]: fmt::Display
    /// [`fmt::Debug`]: fmt::Debug
    /// [`Painted`]: crate::Painted
    /// [`is_enabled()`]: crate::is_enabled()
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::fmt;
    /// use yansi::Style;
    ///
    /// struct CustomItem {
    ///     item: u32,
    ///     style: Style
    /// }
    ///
    /// impl fmt::Display for CustomItem {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         self.style.fmt_prefix(f)?;
    ///         write!(f, "number: {}", self.item)?;
    ///         self.style.fmt_suffix(f)
    ///     }
    /// }
    /// ```
    pub fn fmt_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        if self == &Style::DEFAULT {
            return Ok(());
        }

        f.write_str("\x1B[0m")
    }

    /// Returns the ANSI code sequence suffix for the style as a string.
    ///
    /// This returns a string with the exact same sequence written by
    /// [`fmt_suffix()`](Self::fmt_suffix()). See that method for details.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "_nightly", doc(cfg(feature = "alloc")))]
    pub fn suffix_seq(&self) -> Cow<'static, str> {
        if self == &Style::DEFAULT {
            return Cow::from("");
        }

        Cow::from("\x1B[0m")
    }

    properties!([pub const] constructor(Self) -> Self);
}

impl AnsiSplicer<'_> {
    fn splice(&mut self) -> fmt::Result {
        if self.splice { self.f.write_char(';')?; }
        self.splice = true;
        Ok(())
    }
}

impl fmt::Write for AnsiSplicer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.f.write_str(s)
    }
}
