use std::fmt::{self, Display};
use std::ops::BitOr;

use {Paint, Color};

#[derive(Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Properties(u8);

impl Properties {
    pub const BOLD: Self = Properties(1 << 0);
    pub const DIMMED: Self = Properties(1 << 1);
    pub const ITALIC: Self = Properties(1 << 2);
    pub const UNDERLINE: Self = Properties(1 << 3);
    pub const BLINK: Self = Properties(1 << 4);
    pub const INVERT: Self = Properties(1 << 5);
    pub const HIDDEN: Self = Properties(1 << 6);
    pub const STRIKETHROUGH: Self = Properties(1 << 7);

    #[inline(always)]
    pub fn contains(self, other: Properties) -> bool {
        (other.0 & self.0) == other.0
    }

    #[inline(always)]
    pub fn set(&mut self, other: Properties) {
        self.0 |= other.0;
    }

    #[inline(always)]
    pub fn iter(self) -> Iter {
        Iter { index: 0, properties: self }
    }
}

impl BitOr for Properties {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Properties(self.0 | rhs.0)
    }
}

pub struct Iter {
    index: u8,
    properties: Properties,
}

impl Iterator for Iter {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 8 {
            let index = self.index;
            self.index += 1;

            if self.properties.contains(Properties(1 << index)) {
                return Some(index as usize);
            }
        }

        None
    }
}

/// Represents a set of styling options.
///
/// See the [crate level documentation](./) for usage information.
///
/// # Method Glossary
///
/// The `Style` structure exposes many methods for convenience. The majority of
/// these methods are shared with [`Paint`](Paint).
///
/// ### Unstyled Constructors
///
/// Return a new `Style` structure with no styling applied.
///
///   * [`Style::new()`](Style::new())
///   * [`Style::masked()`](Style::masked())
///
/// ### Foreground Color Constructors
///
/// Return a new `Style` structure with a foreground color applied.
///
///   * [`Style::black()`](Style::black())
///   * [`Style::red()`](Style::red())
///   * [`Style::green()`](Style::green())
///   * [`Style::yellow()`](Style::yellow())
///   * [`Style::blue()`](Style::blue())
///   * [`Style::purple()`](Style::purple())
///   * [`Style::cyan()`](Style::cyan())
///   * [`Style::white()`](Style::white())
///
/// ### Getters
///
/// Return information about a `Style` structure.
///
///   * [`style.fg_color()`](Style::fg_color())
///   * [`style.bg_color()`](Style::bg_color())
///   * [`style.is_masked()`](Style::is_masked())
///   * [`style.is_bold()`](Style::is_bold())
///   * [`style.is_dimmed()`](Style::is_dimmed())
///   * [`style.is_italic()`](Style::is_italic())
///   * [`style.is_underline()`](Style::is_underline())
///   * [`style.is_blink()`](Style::is_blink())
///   * [`style.is_invert()`](Style::is_invert())
///   * [`style.is_hidden()`](Style::is_hidden())
///   * [`style.is_strikethrough()`](Style::is_strikethrough())
///
/// ### Setters
///
/// Set a style property on a given `Style` structure.
///
///   * [`style.fg(color: Color)`](Style::fg())
///   * [`style.bg(color: Color)`](Style::bg())
///   * [`style.mask()`](Style::mask())
///   * [`style.bold()`](Style::bold())
///   * [`style.dimmed()`](Style::dimmed())
///   * [`style.italic()`](Style::italic())
///   * [`style.underline()`](Style::underline())
///   * [`style.blink()`](Style::blink())
///   * [`style.invert()`](Style::invert())
///   * [`style.hidden()`](Style::hidden())
///   * [`style.strikethrough()`](Style::strikethrough())
///
/// These methods can be chained:
///
/// ```rust
/// use yansi::Style;
///
/// Style::red().underline().invert().italic().dimmed().bold();
/// ```
///
/// ### Converters
///
/// Convert a `Style` into another structure.
///
///   * [`style.paint<T>(item: T) -> Paint<T>`](Style::paint())
///
/// ### Raw Formatters
///
/// Write the raw ANSI codes for a given `Style` to a `fmt::Formatter`.
///
///   * [`style.fmt_prefix(f: &mut fmt::Formatter)`](Style::fmt_prefix())
///   * [`style.fmt_suffix(f: &mut fmt::Formatter)`](Style::fmt_suffix())
#[repr(packed)]
#[derive(Default, Debug, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Style {
    pub(crate) foreground: Color,
    pub(crate) background: Color,
    pub(crate) properties: Properties,
    pub(crate) masked: bool,
}

impl PartialEq for Style {
    fn eq(&self, other: &Style) -> bool {
        self.foreground == other.foreground
            && self.background == other.background
            && self.properties == other.properties
    }
}

macro_rules! constructors_for {
    ($($name:ident: $color:ident),*) => ($(
    docify!([
        Constructs a new @code{Style} structure with the foreground color set to
        $name.

        @fence @rust
        use yansi::Style; @nl @nl

        let $name = @{"Style::"} @[$name] @{"();"} @nl
        @{r#"println!("This is going to be "#} @[$name] @{r#": {}", "#}
            @[$name] @{r#".paint("yay!"));"#}
        @fence
    ];
        #[inline]
        pub fn $name() -> Style {
            Style::new().fg(Color::$color)
        }
    );)*)
}

macro_rules! checker_for {
    ($($name:ident ($fn_name:ident): $property:ident),*) => ($(
    docify!([
        Returns @code{true} if the @[_$name]_ property is set on @code{self}.

        @fence @rust
        use yansi::Style; @nl @nl

        let plain = @{"Style::new();"} @nl
        @{"assert!(!"} @[plain.$fn_name] @{"());"} @nl @nl

        let styled = @[plain.$name] @{"()"}; @nl
        @{"assert!("} @[styled.$fn_name] @{"());"}
        @fence
    ];
        #[inline]
        pub fn $fn_name(&self) -> bool {
            self.properties.contains(Properties::$property)
        }
    );)*)
}

#[inline(always)]
fn write_spliced<T: Display>(c: &mut bool, f: &mut fmt::Formatter, t: T) -> fmt::Result {
    if *c {
        write!(f, ";{}", t)
    } else {
        *c = true;
        write!(f, "{}", t)
    }
}

impl Style {
    /// Default, unstylized `Style`. This is identical to `Style::default()`.
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// let plain = Style::new();
    /// assert_eq!(plain, Style::default());
    /// ```
    #[inline]
    pub fn new() -> Style {
        Self::default()
    }

    /// Default, unstylized but _masked_ `Style`. Aside from masking, this is
    /// identical to `Style::default()`.
    ///
    /// An item with _masked_ styling is not written out when painting is
    /// disabled during `Display` or `Debug` invocations. When painting is
    /// enabled, masking has no effect.
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// let plain = Style::masked();
    /// assert_eq!(plain, Style::default());
    /// ```
    #[inline]
    pub fn masked() -> Style {
        Self::default().mask()
    }

    constructors_for!(black: Black, red: Red, green: Green, yellow: Yellow,
                         blue: Blue, purple: Purple, cyan: Cyan, white: White);

    /// Returns the foreground color of `self`.
    ///
    /// ```rust
    /// use yansi::{Style, Color};
    ///
    /// let plain = Style::new();
    /// assert_eq!(plain.fg_color(), Color::Unset);
    ///
    /// let red = plain.fg(Color::Red);
    /// assert_eq!(red.fg_color(), Color::Red);
    /// ```
    #[inline]
    pub fn fg_color(&self) -> Color {
        self.foreground
    }

    /// Returns the foreground color of `self`.
    ///
    /// ```rust
    /// use yansi::{Style, Color};
    ///
    /// let plain = Style::new();
    /// assert_eq!(plain.bg_color(), Color::Unset);
    ///
    /// let white = plain.bg(Color::White);
    /// assert_eq!(white.bg_color(), Color::White);
    /// ```
    #[inline]
    pub fn bg_color(&self) -> Color {
        self.background
    }

    /// Returns `true` if `self` is masked.
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// let plain = Style::new();
    /// assert!(!plain.is_masked());
    ///
    /// let masked = plain.mask();
    /// assert!(masked.is_masked());
    /// ```
    #[inline]
    pub fn is_masked(&self) -> bool {
        self.masked
    }

    checker_for!(bold (is_bold): BOLD, dimmed (is_dimmed): DIMMED,
                    italic (is_italic): ITALIC, underline (is_underline): UNDERLINE,
                    blink (is_blink): BLINK, invert (is_invert): INVERT,
                    hidden (is_hidden): HIDDEN,
                    strikethrough (is_strikethrough): STRIKETHROUGH);

    /// Sets the foreground to `color`.
    ///
    /// ```rust
    /// use yansi::{Color, Style};
    ///
    /// let red_fg = Style::new().fg(Color::Red);
    /// ```
    #[inline]
    pub fn fg(mut self, color: Color) -> Style {
        self.foreground = color;
        self
    }

    /// Sets the background to `color`.
    ///
    /// ```rust
    /// use yansi::{Color, Style};
    ///
    /// let red_bg = Style::new().bg(Color::Red);
    /// ```
    #[inline]
    pub fn bg(mut self, color: Color) -> Style {
        self.background = color;
        self
    }

    /// Sets `self` to be masked.
    ///
    /// An item with _masked_ styling is not written out when painting is
    /// disabled during `Display` or `Debug` invocations. When painting is
    /// enabled, masking has no effect.
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// let masked = Style::new().mask();
    ///
    /// // "Whoops! " will only print when coloring is enabled.
    /// println!("{}Something happened.", masked.paint("Whoops! "));
    /// ```
    #[inline]
    pub fn mask(mut self) -> Style {
        self.masked = true;
        self
    }

    style_builder_for!(Style, |style| style.properties,
                       bold: BOLD, dimmed: DIMMED, italic: ITALIC,
                       underline: UNDERLINE, blink: BLINK, invert: INVERT,
                       hidden: HIDDEN, strikethrough: STRIKETHROUGH);

    /// Constructs a new `Paint` structure that encapsulates `item` with the
    /// style set to `self`.
    ///
    /// ```rust
    /// use yansi::{Style, Color};
    ///
    /// let alert = Style::red().bold().underline();
    /// println!("Alert: {}", alert.paint("This thing happened!"));
    /// ```
    #[inline]
    pub fn paint<T>(self, item: T) -> Paint<T> {
        Paint::new(item).with_style(self)
    }

    #[inline(always)]
    fn is_plain(&self) -> bool {
        self == &Style::default()
    }

    /// Writes the ANSI code prefix for the currently set styles.
    ///
    /// This method is intended to be used inside of [`fmt::Display`] and
    /// [`fmt::Debug`] implementations for custom or specialized use-cases. Most
    /// users should use [`Paint`] for all painting needs.
    ///
    /// [`fmt::Display`]: fmt::Display
    /// [`fmt::Debug`]: fmt::Debug
    /// [`Paint`]: Paint
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::fmt;
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
    pub fn fmt_prefix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // A user may just want a code-free string when no styles are applied.
        if self.is_plain() {
            return Ok(());
        }

        let mut splice = false;
        write!(f, "\x1B[")?;

        for i in self.properties.iter() {
            let k = if i >= 5 { i + 2 } else { i + 1 };
            write_spliced(&mut splice, f, k)?;
        }

        if self.background != Color::Unset {
            write_spliced(&mut splice, f, "4")?;
            self.background.ascii_fmt(f)?;
        }

        if self.foreground != Color::Unset {
            write_spliced(&mut splice, f, "3")?;
            self.foreground.ascii_fmt(f)?;
        }

        // All the codes end with an `m`.
        write!(f, "m")
    }

    /// Writes the ANSI code suffix for the currently set styles.
    ///
    /// This method is intended to be used inside of [`fmt::Display`] and
    /// [`fmt::Debug`] implementations for custom or specialized use-cases. Most
    /// users should use [`Paint`] for all painting needs.
    ///
    /// [`fmt::Display`]: fmt::Display
    /// [`fmt::Debug`]: fmt::Debug
    /// [`Paint`]: Paint
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::fmt;
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
    pub fn fmt_suffix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_plain() {
            return Ok(());
        }

        write!(f, "\x1B[0m")
    }
}
