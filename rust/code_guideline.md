# Agent Rules

## Rust Code Guidelines

+ Prefer iterators over loops.
+ Prefer chains of small closures over a single complex one.

+ Prefer `.map_err(...)` over `Ok(...?)`.
+ Prefer `.map(|_| ()).map_err(...)` over `...?; Ok(())`.

+ Put `use` statements as near to their usage site as possible.
+ `use SomeEnum::*;` before matching against some enum.
