# kson 🦀

> The name comes from **Krab** + **JSON** — a JSON-to-Rust-struct parser (and back), built as a learning project.

`kson` is a Rust library for parsing JSON into native Rust types and serializing Rust types back into JSON, implemented from scratch (no `serde`) as an exercise to really understand tokenizers, parsers, and error handling in Rust.

## Why?

This project isn't trying to replace `serde_json` (which is excellent and highly optimized) — it's a hands-on exercise to learn:

- How a tokenizer/lexer works
- Recursive-descent parsers
- Designing types with `enum` to represent arbitrary data (AST)
- Idiomatic error handling in Rust
- (Optional, future) Derive macros with `proc_macro2` / `syn` / `quote`

## Project status

🚧 Actively in development — personal learning project, not recommended for production use.

## Installation

Since this is a local/personal crate, add it to your `Cargo.toml`:

```toml
[dependencies]
kson = { path = "../kson" }
```

## Basic usage

```rust
use kson::JsonValue;

fn main() {
    let input = r#"{"name": "Ramón", "active": true, "age": 28}"#;

    let value = kson::parse(input).expect("Invalid JSON");

    match value {
        JsonValue::Object(map) => {
            println!("Parsed {} fields", map.len());
        }
        _ => println!("Not an object"),
    }

    let back_to_string = kson::to_string(&value);
    println!("{}", back_to_string);
}
```

## Supported types

- `null`
- `bool` (`true` / `false`)
- Numbers (`f64`)
- Strings
- Arrays (`Vec<JsonValue>`)
- Objects (`HashMap<String, JsonValue>`)

## Roadmap

- [ ] Basic tokenizer
- [ ] Recursive-descent parser → `JsonValue`
- [ ] `JsonValue` → `String` serialization
- [ ] Error handling with position (line/column)
- [ ] Derive macros (`#[derive(ToJson)]`, `#[derive(FromJson)]`)
- [ ] Benchmarks against `serde_json`

## License

MIT — personal learning project.