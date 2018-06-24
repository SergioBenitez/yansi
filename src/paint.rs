use std::fmt;

use style::{Style, Properties};
use color::Color;

/// A structure encapsulating an item and styling.
///
/// See the [crate level documentation](./) for usage information.
///
/// # Method Glossary
///
/// The `Paint` structure exposes many methods for convenience.
///
/// ### Unstyled Constructors
///
/// Return a new `Paint` structure with no styling applied.
///
///   * [`Paint::new(item: T)`](Paint::new())
///   * [`Paint::masked(item: T)`](Paint::masked())
///
/// ### Foreground Color Constructors
///
/// Return a new `Paint` structure with a foreground color applied.
///
///   * [`Paint::rgb(r: u8, g: u8, b: u8, item: T)`](Paint::rgb())
///   * [`Paint::fixed(color: u8, item: T)`](Paint::fixed())
///   * [`Paint::black(item: T)`](Paint::black())
///   * [`Paint::red(item: T)`](Paint::red())
///   * [`Paint::green(item: T)`](Paint::green())
///   * [`Paint::yellow(item: T)`](Paint::yellow())
///   * [`Paint::blue(item: T)`](Paint::blue())
///   * [`Paint::magenta(item: T)`](Paint::magenta())
///   * [`Paint::cyan(item: T)`](Paint::cyan())
///   * [`Paint::white(item: T)`](Paint::white())
///
/// ### Getters
///
/// Return information about the `Paint` structure.
///
///   * [`paint.style()`](Paint::style())
///
/// ### Setters
///
/// Set a style property on a given `Paint` structure.
///
///   * [`paint.with_style(style: Style)`](Paint::with_style())
///   * [`paint.mask()`](Paint::mask())
///   * [`paint.fg(color: Color)`](Paint::fg())
///   * [`paint.bg(color: Color)`](Paint::bg())
///   * [`paint.bold()`](Paint::bold())
///   * [`paint.dimmed()`](Paint::dimmed())
///   * [`paint.italic()`](Paint::italic())
///   * [`paint.underline()`](Paint::underline())
///   * [`paint.blink()`](Paint::blink())
///   * [`paint.invert()`](Paint::invert())
///   * [`paint.hidden()`](Paint::hidden())
///   * [`paint.strikethrough()`](Paint::strikethrough())
///
/// These methods can be chained:
///
/// ```rust
/// use yansi::Paint;
///
/// Paint::new("hi").underline().invert().italic().dimmed().bold();
/// ```
///
/// ### Global Methods
///
/// Modify or observe the global behavior of painting.
///
///   * [`Paint::enable()`](Paint::enable())
///   * [`Paint::disable()`](Paint::disable())
///   * [`Paint::is_enabled()`](Paint::is_enabled())
///   * [`Paint::enable_windows_ascii()`](Paint::enable_windows_ascii())
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Paint<T> {
    item: T,
    style: Style,
}

macro_rules! constructors_for {
    ($T:ty, $($name:ident: $color:ident),*) => ($(
    docify!([
        Constructs a new @code{Paint} structure encapsulating @code{item} with
        the foreground color set to $name.

        @fence @rust
        use yansi::Paint; @nl @nl

        @{r#"println!("This is going to be "#} @[$name] @{r#": {}", "#}
            @[Paint::$name] @{r#"("yay!"));"#}
        @fence
    ];
        #[inline]
        pub fn $name(item: $T) -> Paint<$T> {
            Paint::new(item).fg(Color::$color)
        }
    );)*)
}

impl<T> Paint<T> {
    /// Constructs a new `Paint` structure encapsulating `item` with no styling.
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// assert_eq!(Paint::new("hello!").to_string(), "hello!".to_string());
    /// ```
    #[inline]
    pub fn new(item: T) -> Paint<T> {
        Paint { item, style: Style::default() }
    }

    /// Constructs a new _masked_ `Paint` structure encapsulating `item`.
    ///
    /// A masked `Paint` is not written out when painting is disabled during
    /// `Display` or `Debug` invocations. When painting is enabled, masking has
    /// no effect.
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // The emoji won't be printed when coloring is disabled.
    /// println!("{}Sprout!", Paint::masked("🌱 "));
    /// ```
    #[inline]
    pub fn masked(item: T) -> Paint<T> {
        Paint::new(item).mask()
    }

    /// Constructs a new `Paint` structure encapsulating `item` with the
    /// foreground color set to the RGB color `r`, `g`, `b`.
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// println!("This is going to be funky: {}", Paint::rgb(70, 130, 122, "hi!"));
    /// ```
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8, item: T) -> Paint<T> {
        Paint::new(item).fg(Color::RGB(r, g, b))
    }

    /// Constructs a new `Paint` structure encapsulating `item` with the
    /// foreground color set to the fixed 256-bit color `color`.
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// println!("This is going to be funky: {}", Paint::fixed(100, "hi!"));
    /// ```
    #[inline]
    pub fn fixed(color: u8, item: T) -> Paint<T> {
        Paint::new(item).fg(Color::Fixed(color))
    }

    constructors_for!(T, black: Black, red: Red, green: Green, yellow: Yellow,
                         blue: Blue, magenta: Magenta, cyan: Cyan, white: White);

    /// Retrieves the style currently set on `self`.
    ///
    /// ```rust
    /// use yansi::{Style, Paint};
    ///
    /// let alert = Style::red().bold().underline();
    /// let painted = Paint::red("hi").bold().underline();
    ///
    /// assert_eq!(alert, painted.style());
    /// ```
    #[inline]
    pub fn style(&self) -> Style {
        self.style
    }

    /// Sets the style of `self` to `style`.
    ///
    /// Any styling currently set on `self` is lost. Prefer to use the
    /// [`style.paint()`](Style::paint()) method to create a `Paint` struct from
    /// `Style`.
    ///
    /// ```rust
    /// use yansi::{Paint, Style};
    ///
    /// let s = Style::red().bold().underline();
    ///
    /// // Using this method.
    /// println!("Alert: {}", Paint::new("This thing happened!").with_style(s));
    ///
    /// // Using the `style.paint()` method.
    /// println!("Alert: {}", s.paint("This thing happened!"));
    /// ```
    #[inline]
    pub fn with_style(mut self, style: Style) -> Paint<T> {
        self.style = style;
        self
    }

    /// Masks `self`.
    ///
    /// A masked `Paint` is not written out when painting is disabled during
    /// `Display` or `Debug` invocations. When painting is enabled, masking has
    /// no effect.
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // "Whoops! " will only print when coloring is enabled.
    /// println!("{}Something happened.", Paint::red("Whoops! ").mask());
    /// ```
    #[inline]
    pub fn mask(mut self) -> Paint<T> {
        self.style.masked = true;
        self
    }

    /// Sets the foreground to `color`.
    ///
    /// ```rust
    /// use yansi::Paint;
    /// use yansi::Color::Red;
    ///
    /// println!("Red foreground: {}", Paint::new("hi!").fg(Red));
    /// ```
    #[inline]
    pub fn fg(mut self, color: Color) -> Paint<T> {
        self.style.foreground = color;
        self
    }

    /// Sets the background to `color`.
    ///
    /// ```rust
    /// use yansi::Paint;
    /// use yansi::Color::Yellow;
    ///
    /// println!("Yellow background: {}", Paint::new("hi!").bg(Yellow));
    /// ```
    #[inline]
    pub fn bg(mut self, color: Color) -> Paint<T> {
        self.style.background = color;
        self
    }

    style_builder_for!(Paint<T>, |paint| paint.style.properties,
                       bold: BOLD, dimmed: DIMMED, italic: ITALIC,
                       underline: UNDERLINE, blink: BLINK, invert: INVERT,
                       hidden: HIDDEN, strikethrough: STRIKETHROUGH);
}

macro_rules! impl_fmt_trait {
    ($trait:ident) => (
        impl<T: fmt::$trait> fmt::$trait for Paint<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if Paint::is_enabled() {
                    self.style.fmt_prefix(f)?;
                    fmt::$trait::fmt(&self.item, f)?;
                    self.style.fmt_suffix(f)
                } else if !self.style.masked {
                    fmt::$trait::fmt(&self.item, f)
                } else {
                    Ok(())
                }
            }
        }
    )
}

impl_fmt_trait!(Display);
impl_fmt_trait!(Debug);

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

static ENABLED: AtomicBool = AtomicBool::new(true);

impl Paint<()> {
    /// Disables coloring globally.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // With coloring enabled, ANSI color codes are emitted.
    /// assert_ne!(Paint::green("go").to_string(), "go".to_string());
    ///
    /// // With coloring disabled, ANSI color codes are _not_ emitted.
    /// Paint::disable();
    /// assert_eq!(Paint::green("go").to_string(), "go".to_string());
    /// ```
    pub fn disable() {
        ENABLED.store(false, Ordering::Release);
    }

    /// Enables coloring globally. Coloring is enabled by default, so this
    /// method should only be called to _re_ enable coloring.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // With coloring disabled, ANSI color codes are _not_ emitted.
    /// Paint::disable();
    /// assert_eq!(Paint::green("go").to_string(), "go".to_string());
    ///
    /// // Reenabling causes color code to be emitted.
    /// Paint::enable();
    /// assert_ne!(Paint::green("go").to_string(), "go".to_string());
    /// ```
    pub fn enable() {
        ENABLED.store(true, Ordering::Release);
    }

    /// Returns `true` if coloring is enabled and `false` otherwise. Coloring is
    /// enabled by default but can be enabled and disabled on-the-fly with the
    /// [`Paint::enable()`] and [`Paint::disable()`] methods.
    ///
    /// [`Paint::disable()`]: struct.Paint.html#method.disable
    /// [`Paint::enable()`]: struct.Paint.html#method.disable
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // Coloring is enabled by default.
    /// assert!(Paint::is_enabled());
    ///
    /// // Disable it with `Paint::disable()`.
    /// Paint::disable();
    /// assert!(!Paint::is_enabled());
    ///
    /// // Reenable with `Paint::enable()`.
    /// Paint::enable();
    /// assert!(Paint::is_enabled());
    /// ```
    pub fn is_enabled() -> bool {
        ENABLED.load(Ordering::Acquire)
    }

    /// Enables ASCII terminal escape sequences on Windows consoles when
    /// possible. Returns `true` if escape sequence support was successfully
    /// enabled and `false` otherwise. On non-Windows targets, this method
    /// always returns `true`.
    ///
    /// Support for escape sequences in Windows consoles was added in the
    /// Windows 10 anniversary update. For targets with older Windows
    /// installations, this method is expected to return `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yansi::Paint;
    ///
    /// // A best-effort Windows ASCII terminal support enabling.
    /// Paint::enable_windows_ascii();
    /// ```
    #[inline]
    pub fn enable_windows_ascii() -> bool {
        ::windows::enable_ascii_colors()
    }
}
