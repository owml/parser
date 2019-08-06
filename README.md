# owens-ml

## Overview

### About

`owens-ml` is an easy-to-parse, generalized (can be used in place of json/yaml/toml/similar), implamented with **Rust** + **the `nom` library**.

Some of the key objectives for `owens-ml` are described in the bullet-points below:

- **Easy & efficiant to parse; no infering types.**
- Generalized syntax; cross compatibile with JSON, TOML, YAML or Python Dicts with very small to none changes in layout.
- *Relativly* easy to read; this is not a main priority for this mu language but it is always nice to have.
- ***(Future)*** owens-ml compatibility with serde to allow translations to and from `owens-ml`.

### Implamentation Notes

This implamentation of the `owens-ml` spec is designed to be a "learning project" for `nom` and should result in a lightweight implamentation with a library and an optional (*but slightly useless*) REPL if a user runs this implamentation directly in their terminal.

### Example `owens-ml` syntax

```json
(s) "hello there" (o) {
    (s) "woo" (a-i) [
        4234,
        5,
        34
    ],
    (i) 3423 (o) {
        (s) "ids" (o) {
            (i) 423 (s) "scOwez",
            (i) 4234 (s) "gdude",
            (i) 234 (s) "bisk",
            (s) "username_dynamic" (s) "woo"
        }
    }
},
(s) "cool_array" (a-s) [
    "woo",
    "cool",
    "awesome",
    ":)"
]
```

As you can see, `owens-ml` is a fiercely statically-typed markup language with a different annotation for *every* different type. This has a reason: **parsing ease**.
