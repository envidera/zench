# Changelog

## 0.2.1

### fixed
 - fix(command): remove 'bench' requirement to execute benchmarking. mod tests were unintentionally triggering benchmarks due to the cargo test bench keyword.

## 0.2.0

### Highlights
- Simplified API for faster, cleaner usage
- Improved benchmark reporting display

### feature 

- New output display format, **tabled** (by default) 

### changed

- `Bench::new()` replaced by the `bench!` macro. Now only `zench::bench` needs to be imported.
- `bench!` automatically wraps inline expressions with `bx()`. Block expressions are left unchanged.
- More Cargo-native: `ZENCH=warn` is no longer required for default usage.



## 0.1.2 - 0.1.3
- Fixed docs

## 0.1.1
- Update crate description
- Update CI

## 0.1.0 (2026-02-14)

- Initial release on Github

### Known issues

- Documentation is incomplete.
