/// Enum representing text attributes, largely for text formatting.
///
/// Text attributes are typically applied to a [`Style`], [`Color`], or
/// [`Painted`] struct via the corresponding chainable builder methods such as
/// [`bold()`] or [`italic()`]. The returned value will apply the attribute(s)
/// when rendered or printed.
///
/// Attributes are idempotent, so applying an attribute more than once has no
/// more affect than applying it once.
///
/// # Terminal Support
///
/// Whether an applied attribute actually has an effect on how text is rendered
/// in a terminal depends on the terminal's support for the attribute as well as
/// the terminal's configuration. Common attributes, such as `bold`, `dim`,
/// `italic`, `underline`, and `strike` typically have good support and are
/// largely reliable. Less commonly supported attributes like `conceal` and
/// `invert` will _usually_ be supported by "modern" terminals. Rarely supprted
/// attributes, like  `blink` and `rapid blink` will usually have no effect when
/// applied.
///
/// # Example
///
/// ```rust
/// use yansi::{Style, Color::Red};
///
/// /// A style with red foreground and every "common" attribute applied.
/// static MAD: Style = Red.bold().dim().italic().underline().strike();
/// ```
///
/// [`Style`]: crate::Style
/// [`Painted`]: crate::Painted
/// [`Color`]: crate::Painted
/// [`bold()`]: crate::Style::bold()
/// [`italic()`]: crate::Style::italic()
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Attribute {
    /// Makes text <b>bold</b>.
    ///
    /// Typically used via the [`bold()`](crate::Style::bold()) builder method.
    Bold,
    /// Makes text <span style="opacity: 50%">dim</span>.
    ///
    /// Typically used via the [`dim()`](crate::Style::dim()) builder method.
    Dim,
    /// Display text in <i>italics</i>.
    ///
    /// Typically used via the [`italic()`](crate::Style::italic()) builder
    /// method.
    Italic,
    /// <u>Underline</u> text.
    ///
    /// Typically used via the [`underline()`](crate::Style::underline())
    /// builder method.
    Underline,
    /// <style>@keyframes blinker { 50% { opacity: 0; } }</style>
    /// <span style="animation: blinker 1s linear infinite;">Blink.</span>
    ///
    /// Typically used via the [`blink()`](crate::Style::blink()) builder
    /// method.
    Blink,
    /// <style>@keyframes blinker { 50% { opacity: 0; } }</style>
    /// <span style="animation: blinker 0.5s linear infinite;">Blink rapidly.</span>
    ///
    /// Typically used via the [`rapid_blink()`](crate::Style::rapid_blink())
    /// builder method.
    RapidBlink,
    /// <span style="background: black; color: white;">Invert</span>
    /// (flip) the foreground and background colors.
    ///
    /// Typically used via the [`invert()`](crate::Style::invert()) builder
    /// method.
    Invert,
    /// <span style="color: #333; background: #000;">Conceal</span> text.
    ///
    /// Typically used via the [`conceal()`](crate::Style::conceal()) builder
    /// method.
    Conceal,
    /// Display text with a <s>strike</s> through it.
    ///
    /// Typically used via the [`strike()`](crate::Style::strike()) builder
    /// method.
    Strike,
}

/// Enum representing a quirk mode of yansi.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Quirk {
    Mask,
    Wrap,
    Bright,
    OnBright
}

set_enum! {
    Attribute { Bold, Dim, Italic, Underline, Blink, RapidBlink, Invert, Conceal, Strike }
}

set_enum! {
    Quirk { Mask, Wrap, Bright, OnBright }
}

impl Attribute {
    pub(crate) fn fmt(&self, f: &mut dyn core::fmt::Write) -> core::fmt::Result {
        write!(f, "{}", match self {
            Attribute::Bold => 1,
            Attribute::Dim => 2,
            Attribute::Italic => 3,
            Attribute::Underline => 4,
            Attribute::Blink => 5,
            Attribute::RapidBlink => 6,
            Attribute::Invert => 7,
            Attribute::Conceal => 8,
            Attribute::Strike => 9,
        })
    }
}

impl From<Attribute> for crate::Style {
    fn from(attr: Attribute) -> Self {
        crate::Style::new().attr(attr)
    }
}

impl From<Quirk> for crate::Style {
    fn from(quirk: Quirk) -> Self {
        crate::Style::new().quirk(quirk)
    }
}
