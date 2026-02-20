# Agent Rules

## Rust Code Guidelines

+ Prefer iterators over loops.
+ Prefer chains of small closures over a single big complex closure.

+ Prefer `.map_err(...)` over `Ok(...?)`.
+ Prefer `.map(|_| ()).map_err(...)` over `...?; Ok(())`.

+ Put `use` statements as near to their usage site as possible.
+ `use SomeEnum::*;` before matching against some enum.
