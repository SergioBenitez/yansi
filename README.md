# yansi

A dead simple ANSI terminal color painting library.

```rust
use yansi::Paint;
use yansi::Color::White;

println!("{} light, {} light!", Paint::green("Green"), Paint::red("red"));
```

See the [documentation](https://sergio.bz/rustdocs/yansi) for more.

## License

State is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
