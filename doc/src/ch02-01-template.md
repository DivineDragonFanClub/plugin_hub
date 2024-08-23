# Template

Let's start learning about Skyline by analyzing the project you've initialized in the previous chapter! If you check out the *lib.rs* file inside of your plugin's */src* directory, you'll be greeted with the following:

```rust
#[skyline::main(name = "hello_world")]
pub fn main() {
    println!("Hello from skyline plugin");
}
```

So far, so good, right? Assuming you're already familiar with the Rust language or programming in general, you should recognize this as the `entry point` of your program save from a small difference. Let's observe what's happening here!

> If you are not familiar with Rust and the concept of macros, please read the [following paragraph](https://doc.rust-lang.org/book/ch01-02-hello-world.html#anatomy-of-a-rust-program) of the official book before proceeding.

```rust
#[skyline::main(name = "hello_world")]
```

This line represents a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html), which is an advanced but powerful Rust concept that can essentially write new Rust code when attached to an item. As a user, you don't have to worry too much about them besides knowing how to call them!

This specific macro needs to be used once in your entire plugin by attaching it to your ``entry point`` which is a fancy way to say "the first function that will be called". This is needed so plugin loaders such as Skyline, ARCropolis and Cobalt can get your plugin started.

The interesting part is the following:

```rust
name = "hello_world"
```

Observant readers might notice that the string is already filled with the name of the project you've created using cargo-skyline. This is the internal name of your plugin, and will be used in crash reports to let you know when some code was ran from your plugin prior to the crash.  
You are free to change it after creating your project, but try to keep it relatively short!