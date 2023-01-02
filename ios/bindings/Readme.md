# iOS bindings

<https://medium.com/visly/rust-on-ios-39f799b3c1dd>

<https://users.rust-lang.org/t/using-rust-with-swift-xcode-on-ios/31996>

<https://github.com/thombles/dw2019rust>

<https://www.youtube.com/watch?v=lKYSQ4JkSLU>


```shell

```

```shell
cargo lipo --release
cbindgen src/lib.rs -l c > target/universal/release/rust_ios.h
```

`target/universal/release/librust_ios.a`
``