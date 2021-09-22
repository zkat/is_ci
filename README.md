This crate tells you if you're in a CI environment or not. It does not tell
you **which** you're in, but it makes a good effort to make sure to accurately
tell you whether you're in one.

This crate is based on the
[`@npmcli/ci-detect`](https://npm.im/@npmcli/ci-detect) package.

If you need more information about the specific CI environment you're running
in and you can handle a heavier dependency, please consider using
[`ci_info`](https://crates.io/crates/ci_info) instead.

## Example

```rust
// This is it. This is the entire API.
let am_i_in_ci_right_now = is_ci::is_ci();
```

## License

`is_ci` is released to the Rust community under the [ISC License](./LICENSE).

It is based on `@npmcli/ci-detect` which is released to the community under
the [ISC License](https://github.com/npm/ci-detect/blob/main/LICENSE).
