# rust-dtlc

Rust implementation of a dependently typed λ-calculus.

## Synopsis

### Design

The interpreter is based on ideas from the following sources:

- Löh, McBride, Swierstra / [Simply Easy!](http://strictlypositive.org/Easy.pdf) ([revised version](http://www.andres-loeh.de/LambdaPi/LambdaPi.pdf))
- Augustsson / [Simpler, Easier!](http://augustss.blogspot.com/2007/10/simpler-easier-in-recent-paper-simply.html)

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo run         ## run binary (does nothing useful yet)
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```
