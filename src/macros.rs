macro_rules! style_builder_for {
    ($T:ty, $($name:ident: $property:ident),*) => ($(
    docify!([
        Enables the @[_$name]_ style on @{"`self`"}.

        @fence @rust
        use yansi::Paint; @nl @nl

        @{"println!(\"Using "} @[$name] @{": {}\", "}
            @{r#"Paint::new("hi")"#} @[.$name] @{"());"}
        @fence
    ];
        #[inline]
        pub fn $name(mut self) -> $T {
            self.properties().set(Properties::$property);
            self
        }
    );)*)
}
