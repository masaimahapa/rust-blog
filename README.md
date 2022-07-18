# Rust Blog with Yew
This is a blog created with Rust and Yew to demonstrate how Rust can be used as a full stack language.

Please read my [blog post](https://www.masaimahapa.co.za/posts/30-days-of-rust-day-24) for better context.

## How to run
Make sure you have the latest version of Rust
```
rustup update
```
Secondly, install trunk;
```
cargo install trunk
```
lastly add the WebAssembly build target
```
rustup target add wasm32-unknown-unknown
```
Run the server;

```cmd
trunk serve --open
```

![yew web app masai](/rust-blog-masai.png)