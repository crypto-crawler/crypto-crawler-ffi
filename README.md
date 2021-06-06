# crypto-crawler-ffi

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-ffi/CI/main)](https://github.com/soulmachine/crypto-crawler-ffi/actions?query=branch%3Amain)

FFI bindings for the [crypto-crawler](https://crates.io/crates/crypto-crawler) crate.

## Build

On Mac OS X:

```bash
brew install openssl@1.1
cargo build --release
```

For Windows and Linux:

```bash
cargo install cross

cross build --target x86_64-unknown-linux-gnu --release

cross build --target x86_64-pc-windows-gnu --release
```

## References

- [cffi vs cpython vs pyo3, what should I use?](https://www.reddit.com/r/rust/comments/fxe99l/cffi_vs_cpython_vs_pyo3_what_should_i_use/)
- [Evolving Our Rust With Milksnake](https://blog.sentry.io/2017/11/14/evolving-our-rust-with-milksnake)
- [How to write Rust instead of C and get away with it](https://ep2018.europython.eu/media/conference/slides/how-to-write-rust-instead-of-c-and-get-away-with-it-yes-its-a-python-talk.pdf)
- [avro-rs-ffi](https://github.com/flavray/avro-rs-ffi) and [pyavro-rs](https://github.com/flavray/pyavro-rs)
- [A dive into packaging native python extensions](https://blog.schuetze.link/2018/07/21/a-dive-into-packaging-native-python-extensions.html)
- [Rust + Python | Perl FFI Strings](https://dean.serenevy.net/blog/2020/Dec/python-rust-string-ffi/)
- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Calling Rust From Python](https://bheisler.github.io/post/calling-rust-in-python/)
- [Writing Python Extensions in Rust](https://kushaldas.in/posts/writing-python-extensions-in-rust.html)
