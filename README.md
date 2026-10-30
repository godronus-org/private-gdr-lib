# gdr-lib

POC org library (spike 4). Use it from a FastEdge app as a git dependency pinned to a release tag:

```toml
[dependencies]
gdr-lib = { git = "https://github.com/godronus-org/gdr-lib", tag = "v0.1.0" }
```

```rust
let line = gdr_lib::greeting("hello"); // "hello from gdr-lib"
```
