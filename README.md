# `yansi`

[![Build Status](https://github.com/SergioBenitez/yansi/workflows/CI/badge.svg)](https://github.com/SergioBenitez/yansi/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/yansi.svg)](https://crates.io/crates/yansi)
[![Documentation](https://docs.rs/yansi/badge.svg)](https://docs.rs/yansi)

A dead simple ANSI terminal color painting library.

```rust
use yansi::Paint;

println!("Testing, {}, {}, {}!",
    "Ready".bold(),
    "Set".black().on_yellow().invert().italic(),
    "STOP".white().on_red().bright().underline().bold());
```

`>` Testing,
  <b>Ready</b>,
  <span style="color: yellow; background: black;"><i><b>Set</b></i></span>,
  <span style="color: white; background: red;"><u><b>STOP</b></u></span>!

## Features

Why *y*et another *ANSI* terminal coloring library? Here are some reasons:

  * This library makes simple things _simple_: `use` [`Paint`] and go!
  * Zero dependencies by default. It really is simple.
  * Zero allocations except as needed by opt-in [wrapping].
  * [Automatic Windows support] for the vast majority (95%+) of Windows users.
  * [Featureful `no_std`], no-`alloc`, support with `default-features = false`.
  * [`Style` constructors are `const`]: store styles statically, even with
    dynamic conditions!
  * _Any_ type implementing a formatting trait can be styled, not just strings.
  * Styling can be [enabled] and [disabled] globally and [dynamically], on the
    fly.
  * A `Style` can be predicated on arbitrary [conditions].
  * Formatting specifiers like `{:x}` and `{:08b}` are supported and preserved!
  * [Built-in (optional) conditions] for [TTY detection] and [common environment
    variables].
  * Arbitrary items can be [_masked_] for selective disabling.
  * Styling can [_wrap_] to preserve styling across resets.
  * Styling can [_linger_] beyond a single value.
  * Experimental support for [hyperlinking] is included.
  * The name `yansi` is pretty cool 😎.

[`Paint`]: https://docs.rs/yansi/1.0.0-rc/yansi/trait.Paint.html
[`ansi_term`]: https://crates.io/crates/ansi_term
[`colored`]: https://crates.io/crates/colored
[`term_painter`]: https://crates.io/crates/term-painter
[_masked_]: https://docs.rs/yansi/1.0.0-rc/yansi/#masking
[wrapping]: https://docs.rs/yansi/1.0.0-rc/yansi/#wrapping
[_wrap_]: https://docs.rs/yansi/1.0.0-rc/yansi/#wrapping
[_linger_]: https://docs.rs/yansi/1.0.0-rc/yansi/#lingering
[conditions]: https://docs.rs/yansi/1.0.0-rc/yansi/#per-style
[enabled]: https://docs.rs/yansi/1.0.0-rc/yansi/fn.enable.html
[disabled]: https://docs.rs/yansi/1.0.0-rc/yansi/fn.disable.html
[dynamically]: https://docs.rs/yansi/1.0.0-rc/yansi/fn.whenever.html
[enabled conditionally]: https://docs.rs/yansi/1.0.0-rc/yansi/struct.Condition.html
[TTY detection]: https://docs.rs/yansi/1.0.0-rc/yansi/struct.Condition.html#impl-Condition-1
[common environment variables]: https://docs.rs/yansi/1.0.0-rc/yansi/struct.Condition.html#impl-Condition-2
[Automatic Windows support]: https://docs.rs/yansi/1.0.0-rc/yansi/#windows
[Built-in (optional) conditions]: https://docs.rs/yansi/1.0.0-rc/yansi/struct.Condition.html#built-in-conditions
[hyperlinking]: https://docs.rs/yansi/1.0.0-rc/yansi/hyperlink/index.html
[`Style` constructors are `const`]: https://docs.rs/yansi/1.0.0-rc/yansi/#uniform-const-builders
[Featureful `no_std`]: https://docs.rs/yansi/1.0.0-rc/yansi/#crate-features

## License

`yansi` is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
