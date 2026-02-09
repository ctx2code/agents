# Agent Rules

## Usage of Commonly Used Rust Crates

+ Prefer using `anyhow::Result` when there's `anyhow` and a `Result` is needed.
+ Use `.context(...)` before returning with `?`.
+ Use `.context(...)` with a description about the whole operation.

+ Prefer `dotenvy::var` over `dotenvy::dotenv; ...; std::env::var`.
