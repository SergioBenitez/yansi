#![doc(html_root_url = "https://docs.rs/yansi/0.3")]

//! A dead simple ANSI terminal color painting library.
//!
//! # Usage
//!
//! Usage is best illustrated via a quick example:
//!
//! ```rust
//! use yansi::{Paint, Color};
//!
//! println!("Testing, {}, {}, {}!",
//!     Paint::red(1),
//!     Paint::green(2).bold().underline(),
//!     Paint::blue("3").bg(Color::White).italic());
//! ```
//!
//! ## Paint
//!
//! The main entry point into this library is the [`Paint`] type. `Paint`
//! encapsulates a value of any type that implements the [`Display`] or
//! [`Debug`] trait. When a `Paint` is `Display`ed or `Debug`ed, the appropriate
//! ANSI escape characters are emitted before and after the wrapped type's `fmt`
//! implementation.
//!
//! `Paint` can be constructed via [a myriad of methods]. In addition to these
//! constructors, you can also use the [`paint`](Color::paint()) method on a
//! given [`Color`] value to construct a `Paint` type. Both of these approaches
//! are shown below:
//!
//! ```rust
//! use yansi::Paint;
//! use yansi::Color::Red;
//!
//! println!("I'm {}!", Paint::red("red").bold());
//! println!("I'm also {}!", Red.paint("red").bold());
//! ```
//! [`Paint`]: Paint
//! [`Display`]: ::std::fmt::Display
//! [`Debug`]: ::std::fmt::Debug
//! [a myriad of methods]: struct.Paint.html#unstyled-constructors
//! [`Color`]: Color
//!
//! ## Styling
//!
//! Modifications to the styling of an item can be made via [a number of
//! chainable methods] on `Paint`.
//!
//! ```rust
//! use yansi::Paint;
//!
//! Paint::new("hi").underline().invert().italic().dimmed().bold();
//! ```
//!
//! Styling can also be created independently from a `Paint` structure via the
//! [`Style`] structure. This allows common styling to be stored and reused. A
//! `Style` can be applied via the [`style.paint()`] method or the
//! [`paint.with_style()`] method:
//!
//! ```rust
//! use yansi::{Paint, Style};
//!
//! // A bold, itatlic style with red foreground.
//! let alert = Style::red().bold().italic();
//!
//! // Using `style.paint()`; this is preferred.
//! println!("Alert! {}", alert.paint("This is serious business!"));
//! println!("Hi! {}", alert.underline().paint("Super serious!"));
//!
//! // Using `paint.with_style()`.
//! println!("Alert! {}", Paint::new("Yet another.").with_style(alert));
//! ```
//!
//! [a number of chainable methods]: struct.Paint.html#setters
//! [`Style`]: Style
//! [`style.paint()`]: Style::paint()
//! [`paint.with_style()`]: Paint::with_style()
//!
//! # Disabling
//!
//! Painting can be disabled globally via the [`Paint::disable()`] method. When
//! painting is disabled, the `Display` and `Debug` implementations for `Paint`
//! will emit the `Display` or `Debug` of the contained object and nothing else.
//! Painting can be reenabled via the [`Paint::enable()`] method.
//!
//! One potential use of this feature is to allow users to control color ouput
//! via an environment variable. For instance, to disable coloring if the
//! `CLICOLOR` variable is set to `0`, you might write:
//!
//! ```rust
//! # { if false { // we don't actually want to disable coloring
//! use yansi::Paint;
//!
//! if let Ok(true) = std::env::var("CLICOLOR").map(|v| v == "0") {
//!     Paint::disable();
//! }
//! # } }
//! ```
//!
//! [`Paint::disable()`]: Paint::disable()
//! [`Paint::enable()`]: Paint::enable()
//!
//! ## Masking
//!
//! Items can be arbitrarily _masked_. When an item is masked and painting is
//! disabled, the `Display` and `Debug` implementations of `Paint` write
//! nothing. This allows you to selectively omit output when painting is
//! disabled. Values can be masked using the [`Paint::masked()`] and
//! [`Style::masked()`]constructors or [`paint.mask()`] and [`style.mask()`]
//! style setters.
//!
//! [`Paint::masked()`]: Paint::masked()
//! [`paint.mask()`]: Paint::mask()
//! [`Style::masked()`]: Style::masked()
//! [`style.mask()`]: Style::mask()
//!
//! One use for this feature is to print certain characters only when painting
//! is enabled. For instance, you might wish to emit the 🎨 emoji when
//! coloring is enabled but not otherwise. This can be accomplished by masking
//! the emoji:
//!
//! ```rust
//! use yansi::Paint;
//!
//! println!("I like colors!{}", Paint::masked(" 🎨"));
//! ```
//!
//! This will print "I like colors! 🎨" when painting is enabled and "I like
//! colors!" when painting is disabled.
//!
//! # Windows
//!
//! Coloring is supported on Windows beginning with the Windows 10 anniversary
//! update. Since this update, Windows consoles support ANSI escape sequences.
//! This support, however, must be explicitly enabled. `yansi` provides the
//! [`Paint::enable_windows_ascii()`] method to enable ASCII support on Windows
//! consoles when available.
//!
//! ```rust
//! use yansi::Paint;
//!
//! // Enable ASCII escape sequence support on Windows consoles.
//! Paint::enable_windows_ascii();
//! ```
//!
//!
//! You may wish to disable coloring on unsupported Windows consoles to avoid
//! emitting unrecognized ASCII escape sequences:
//!
//! ```rust
//! use yansi::Paint;
//!
//! if cfg!(windows) && !Paint::enable_windows_ascii() {
//!     Paint::disable();
//! }
//! ```
//!
//! [`Paint::enable_windows_ascii()`]: Paint::enable_windows_ascii()
//!
//! # Why?
//!
//! Several terminal coloring libraries exist ([`ansi_term`], [`colored`],
//! [`term_painter`], to name a few), begging the question: why yet another?
//! Here are a few reasons:
//!
//!   * This library is _much_ simpler: there are three types!
//!   * Like [`term_painter`] but unlike [`ansi_term`], _any_ type implementing
//!     `Display` or `Debug` can be stylized.
//!   * Styling can be enabled and disabled globally, on the fly.
//!   * Arbitrary items can be _masked_ for selective disabling.
//!   * Typically only one type needs to be imported: [`Paint`].
//!   * Zero dependencies. It really is simple.
//!   * The name `yansi` is pretty short.
//!
//! All that being said, this library borrows API ideas from the three libraries
//! as well as implementation details from [`ansi_term`].
//!
//! [`ansi_term`]: https://crates.io/crates/ansi_term
//! [`colored`]: https://crates.io/crates/colored
//! [`term_painter`]: https://crates.io/crates/term-painter

#[macro_use] mod docify;
#[macro_use] mod macros;

#[cfg(test)] mod tests;
mod windows;
mod paint;
mod style;
mod color;

pub use color::Color;
pub use style::Style;
pub use paint::Paint;
