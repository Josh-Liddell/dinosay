# Dinosay

In this project I demonstrate defining and implementing a custom trait in Rust, named Dinosay, to extend built in string types. The trait adds a method that formats a given text into a wrapped speech box and displays it alongside an ASCII dinosaur figure.

#### Usage:
```bash
cargo install dinosay
```

```bash
dinosay 'Hello world!'
```

OR

```bash
cargo add dinosay
```

```rust
use dinosay::Dinosay;

fn main() {
    "Hello Friends!".dinosay();
}
```
