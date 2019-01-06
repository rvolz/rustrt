# rustrt, a Rust ray tracer toy

A toy implementation of Jamis Bucks [The Ray Tracer Challenge](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge) in Rust, with the usual goal: learning about the language and ray tracing.

The implementation has been started, but ...

## Testing

The repo contains the Cucumber features from the book, test with [cucumber-rust](https://github.com/bbqsrc/cucumber-rust). To run them all use:

```
cargo test
```

To run specific features use the filter flag:

```
cargo test --test cucumber -- -f "features/tup*"
```

## Notes

* The matrices.feature had to be changed, because cucumber_rust/gherkin_rust can't deal with tables without headers currently (0.5.1/0.4.1).