use std::fmt;
use std::ops::BitOr;

use Paint;

/// An enum representing an ANSI color code.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum Color {
    /// No color has been set. Nothing is changed when applied.
    Unset,

    /// Black #0 (foreground code `30`, background code `40`).
    Black,

    /// Red: #1 (foreground code `31`, background code `41`).
    Red,

    /// Green: #2 (foreground code `32`, background code `42`).
    Green,

    /// Yellow: #3 (foreground code `33`, background code `43`).
    Yellow,

    /// Blue: #4 (foreground code `34`, background code `44`).
    Blue,

    /// Purple: #5 (foreground code `35`, background code `45`).
    Purple,

    /// Cyan: #6 (foreground code `36`, background code `46`).
    Cyan,

    /// White: #7 (foreground code `37`, background code `47`).
    White,

    /// A color number from 0 to 255, for use in 256-color terminals.
    Fixed(u8),

    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

impl Color {
    /// Constructs a new `Paint` structure that encapsulates `item` with the
    /// foreground color set to the color `self`.
    ///
    /// ```rust
    /// use yansi::Color::Blue;
    ///
    /// println!("This is going to be blue: {}", Blue.paint("yay!"));
    /// ```
    #[inline(always)]
    pub fn paint<T>(self, item: T) -> Paint<T> {
        Paint::new(item).fg(self)
    }

    /// Constructs a new `Style` structure with the foreground color set to the
    /// color `self`.
    ///
    /// ```rust
    /// use yansi::Color::Green;
    ///
    /// let success = Green.style().bold();
    /// println!("Hey! {}", success.paint("Success!"));
    /// ```
    #[inline(always)]
    pub fn style(self) -> Style {
        Style::new().fg(self)
    }
}

#[doc(hidden)]
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Unset => Ok(()),
            Color::Black => write!(f, "0"),
            Color::Red => write!(f, "1"),
            Color::Green => write!(f, "2"),
            Color::Yellow => write!(f, "3"),
            Color::Blue => write!(f, "4"),
            Color::Purple => write!(f, "5"),
            Color::Cyan => write!(f, "6"),
            Color::White => write!(f, "7"),
            Color::Fixed(num) => write!(f, "8;5;{}", num),
            Color::RGB(r, g, b) => write!(f, "8;2;{};{};{}", r, g, b)
        }
    }
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Self { Color::Unset }
}

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

impl Style {
    /// Default, unstylized `Style`. This is identical to `Style::default()`.
    ///
    /// ```rust
    /// use yansi::Style;
    ///
    /// let plain = Style::new();
    /// assert_eq!(plain, Style::default());
    /// ```
    #[inline(always)]
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
    #[inline(always)]
    pub fn masked() -> Style {
        Self::default().mask()
    }

    constructors_for!(black: Black, red: Red, green: Green, yellow: Yellow,
                         blue: Blue, purple: Purple, cyan: Cyan, white: White);

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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn mask(mut self) -> Style {
        self.masked = true;
        self
    }

    fn properties(&mut self) -> &mut Properties { &mut self.properties }
    style_builder_for!(Style, bold: BOLD, dimmed: DIMMED, italic: ITALIC,
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
    #[inline(always)]
    pub fn paint<T>(self, item: T) -> Paint<T> {
        Paint::new(item).with_style(self)
    }
}
