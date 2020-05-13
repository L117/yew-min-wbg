Yew Min WASM Bindgen Boilerplate <3
===================================

[yew]: https://yew.rs
[wasm-bindgen]: https://crates.io/crates/wasm-bindgen
[wasm-bindgen-cli]: https://crates.io/crates/wasm-bindgen-cli
[watchexec]: https://crates.io/crates/watchexec
[miniserve]: https://crates.io/crates/miniserve
[rustup]: https://rustup.rs/
[just]: https://crates.io/crates/just
[yew-wasm-pack-template]: https://github.com/yewstack/yew-wasm-pack-template
[yew-wasm-pack-minimal]: https://github.com/yewstack/yew-wasm-pack-minimal

## About

This repository shows how to create minimal [Yew][yew] web application with barebones [wasm-bindgen][wasm-bindgen], without any of JavaScript tooling and only with 3 lines of hand-written JavaScript glue.

## What are pros?

- Fast builds. All tools used in this repo are wirtten in Rust and compiled into native binaries that run blazingly fast.
- No need to learn JS or waste time figuring out how to use JS tooling, it's enough to know Rust and a bit of HTML to get started.
- Minimalism. Everything is stripped down to the very minimum, so it's probably as easy to understand as possible.

## What are cons?

- No integration with existing JS ecosystem by default (It is certainly possible, though!). If you wish to add some JS to your project, you're on your own.
  - Because there's no integration with JS ecosystem, your toolset will be pretty limited.

## What are alternatives?

- Yew [yew-wasm-pack-minimal] - if you want set-up that able to integrate with JS tooling (E.g. bundlers).
- Yew [yew-wasm-pack-template] - if you want set-up with familiar tools like Yarn and Webpack.

## Building

First and foremost, to get started with this example, you will need to add nightly toolchain and `wasm32-unknown-unknown` target support via [rustup]. Example itself **does not require nightly**. Do this in your terminal of choice:
```sh
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown
```

This example also uses some handy tools to make your life easier:

- [just] - `make`-inspired task-runner, does routine job for you.
- [miniserve] - static-only HTTP server, delivers content to your browser.
- [watchexec] - combination of `watch` and `exec`, triggers automatic rebuilds on file changes.
- [wasm-bindgen-cli] - generates most of JS glue to let this app interact with DOM.

To install all of these do this in your terminal:
```sh
cargo install just watchexec wasm-bindgen-cli
cargo +nightly install miniserve
```
Now you're all set and ready to roll! Run this in your terminal, wait for completion and head to http://localhost:8000
```sh
just watch
```

## Included recipes

`justfile` contains tasks that can be run.

- `just clean` - destroys `target` and `dist` directories.
- `just build` - (re)builds project, without cleaning.
- `just serve` - serves files from `dist` directory.
- `just watch` - rebuilds project on changes an invokes `serve`.

It also makes sense to combine some of these. And it's certainly possible to add your own custom recipes!

## License

This project licensed under the terms of either Apache 2.0 or MIT license.