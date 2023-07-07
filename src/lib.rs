//! A dead simple ANSI terminal color painting library.
//!
//! # Features
//!
//! Several terminal coloring libraries exist ([`ansi_term`], [`colored`],
//! [`term_painter`], to name a few), begging the question: why yet another?
//! Here are a few reasons:
//!
//!   * This library makes simple things _simple_: `use` [`Paint`] and go!
//!   * Zero dependencies by default. It really is simple.
//!   * [Automatic Windows support] for the vast majority (95%+) of Windows
//!     users.
//!   * Featureful `no_std` support with `default-features = false`.
//!   * Style constructors are `const`: store styles statically, even with
//!     dynamic conditions!
//!   * _Any_ type implementing a formatting trait can be stylized, not just
//!     strings.
//!   * Formatting specifiers like `{:x}` and `{:08b}` are supported and
//!     preserved!
//!   * Styling can be [enabled] and [disabled] globally and [dynamically], on the fly.
//!   * Styles themselves can be made [conditional](Style::whenever()).
//!   * Built-in (optional) conditions for [TTY detection] and [common
//!     environment variables].
//!   * Arbitrary items can be [_masked_] for selective disabling.
//!   * Styling can [_wrap_] to preserve styling across resets.
//!   * The name `yansi` is pretty cool 😎.
//!
//! All that said, `yansi` borrows API ideas from older libraries as well as
//! implementation details from [`ansi_term`].
//!
//! [`ansi_term`]: https://crates.io/crates/ansi_term
//! [`colored`]: https://crates.io/crates/colored
//! [`term_painter`]: https://crates.io/crates/term-painter
//! [_masked_]: #masking
//! [_wrap_]: #wrapping
//! [enabled]: crate::enable
//! [disabled]: crate::disable
//! [dynamically]: crate::enable_when
//! [enabled conditionally]: Condition
//! [TTY detection]: Condition#tty
//! [common environment variables]: Condition#environment-variables
//! [Automatic Windows support]: #windows
//!
//! # Usage
//!
//! The [`Paint`] trait is implemented for every type. Import it and call
//! chainable builder methods:
//!
//! ```rust
//! use yansi::Paint;
//!
//! println!("Testing, {}, {}, {}!",
//!     "Ready".bold(),
//!     "Set".yellow().italic(),
//!     "STOP".white().on_red().bright().underline().bold());
//! ```
//!
//! `>` Testing,
//! <b>Ready</b>,
//! <span style="color: yellow;"><i><b>Set</b></i></span>,
//! <span style="color: white; background: red;"><u><b>STOP</b></u></span>!
//!
//! The methods return a [`Painted`] type which consists of a [`Style`] and a
//! reference to the receiver. Displaying a [`Painted`] (via `print!()`,
//! `format!()`, etc) results in emitting ANSI escape codes that effectuate the
//! style.
//!
//! ## Uniform `const` Builders
//!
//! The builder methods are uniformly available for [`Style`], [`Color`], and
//! [`Painted`], which means you can chain calls across library types. They are
//! also `const`, allowing creations of `const` or `static` styles which can be
//! directly applied via the [`paint()`](Paint::paint()) method, also from the
//! [`Paint`] trait and thus available for every type:
//!
//! ```rust
//! use yansi::{Paint, Style, Color::*};
//!
//! // `const` constructors allow static `Style` for easy reuse.
//! static ALERT: Style = White.bright().underline().italic().on_red();
//!
//! println!("Testing, {}, {}, {}!",
//!     "Ready".bold(),
//!     "Set".yellow().bold(),
//!     "STOP".paint(ALERT));
//! ```
//!
//! `>` Testing,
//! <b>Ready</b>,
//! <span style="color: yellow;"><b>Set</b></span>,
//! <span style="color: white; background: red;"><u><em>STOP</em></u></span>!
//!
//! ## Conditional Styling
//!
//! ### Globally
//!
//! Styling is enabled by default but can be enabled and disabled globally via
//! [`enable()`] and [`disable()`]. When styling is disabled, no ANSI escape
//! codes are emitted, and [_masked_] values are omitted.
//!
//! Stying can also be _conditionally_ globally enabled via [`enable_when()`]
//! based on an arbitrary and dynamic [`Condition`]: a function that returns
//! `true` to enable styling and `false` to disable it.
//!
//! ### Per-`Style`
//!
//! A specific `Style` can also itself be conditionally applied by using
//! [`whenever()`](Style::whenever()):
//!
//! ```rust
//! # #[cfg(feature = "detect-tty")] {
//! use yansi::{Paint, Style, Color::*, Condition};
//!
//! static WARNING: Style = Black.bold().on_yellow().whenever(Condition::STDERR_IS_TTY);
//!
//! eprintln!("{}", "Bees can sting!".paint(WARNING));
//! # }
//! ```
//!
//! With the above, if `stderr` is a TTY, then:
//! `>` <span style="background: yellow; color: black;"><b>Bees can sting!</b></span>
//!
//! If it is not a TTY, styling is not emitted:
//! `>` Bees can sting!
//!
//! See [`Condition`] for a list of built-in conditions which require enabling
//! crate features.
//!
//! # Quirks
//!
//! TODO: Describe quirks.
//!
//! ## Masking
//!
//! Items can be arbitrarily _masked_ with the [`mask()`](Paint::mask()) builder
//! method. Masked values are not emitted when styling is disabled, globally or
//! for a given style. This allows selective output based on whether styling is
//! enabled.
//!
//! One use for this feature is to print certain characters only when styling is
//! enabled. For instance, you might wish to emit the 🎨 emoji when coloring is
//! enabled but not otherwise. This can be accomplished by masking the emoji:
//!
//! ```rust
//! use yansi::Paint;
//!
//! println!("I like colors!{}", " 🎨".mask());
//! ```
//!
//! When styling is enabled, this prints: `>` I like colors! 🎨
//!
//! With styling disabled, this prints: `>` I like colors!
//!
//! ## Wrapping
//!
//! Styling can _wrap_ via [`Quirk::Wrap`] or the equivalent
//! [`wrap()`](Painted::wrap()) constructor. A wrapping style modifies any
//! styling resets emitted by the internal value so that they correspond to the
//! wrapping style. In other words, the "reset" style of the wrapped item is
//! modified to be the wrapping style.
//!
//! Wrapping incurs a performance cost and is needed only in situations when
//! styling opaque items or items that may already be styled, such as when
//! implementing a generic logger. It exists to enable consistent styling across
//! such items:
//!
//! ```rust
//! use yansi::Paint;
//!
//! // Imagine that `inner` is opaque and we don't know it's styling.
//! let inner = format!("{} and {}", "Stop".red(), "Go".green());
//!
//! // We can use `wrap` to ensure anything in `inner` not styled is blue.
//! println!("Hey! {}", inner.blue().wrap());
//! ```
//!
//! Thanks to wrapping, this prints:
//! `>` Hey! <span style="color: blue">
//! <span style="color: red">Stop</span> and
//! <span style="color: green">Go</span>
//! </span>
//!
//! Without wrapping, the `red()` reset after "Stop" is not overwritten:
//! `>` Hey! <span style="color: red">Stop</span> and <span style="color: green">Go</span>
//!
//! # Windows
//!
//! Styling is supported and enabled automatically on Windows beginning with
//! the Windows 10 Anniversary Update, or about [95% of all Windows machines
//! worldwide](https://gs.statcounter.com/os-version-market-share/windows/desktop/worldwide),
//! and likely closer to 100% of developer machines (e.g., 99% of visitors to
//! [rocket.rs](https://rocket.rs) on Windows are on Windows 10+).
//!
//! Yansi enables styling support on Windows by querying the Windows API on the
//! first attempt to color. If support is available, it is enabled. If support
//! is not available, styling is disabled and no styling is emitted.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "_nightly", feature(doc_cfg))]
// #![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[macro_use]
mod macros;
mod windows;
mod attribute;
mod style;
mod fmt;
mod color;
mod paint;
mod global;
mod condition;
mod set;

// TODO: WIP.
// pub mod hyperlink;

pub use paint::{Painted, Paint};
pub use attribute::{Attribute, Quirk};
pub use style::Style;
pub use color::Color;
pub use condition::Condition;
pub use global::{enable, enable_when, disable, is_enabled};
