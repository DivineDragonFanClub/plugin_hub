## Analyzing the Template

Let's start learning about Skyline by analyzing the project you've set up in the previous chapter! If you check out the *lib.rs* file inside of your plugin's */src* directory, you'll be greeted with the following:

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

Now, let's look at the inside of your ``entry point``, which has a single function call:

```rust
println!("Hello from skyline plugin");
```

As the name might imply, ``println!`` is a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) (yes!) to print text and it stands for ``print line new``.  

You might be wondering where exactly text would be printed on a Nintendo Switch and the answer is: it depends.

### Listening to messages

Text will be sent over your local internet to whoever is willing to listen (that's you!). This means you will need something to listen to the messages. But worry not! ``cargo-skyline`` already has everything you need to do so, which we'll explain in a bit.

If you are working on emulator, the messages will also show up in the console that opens alongside it. You might need to change some settings to have it open and print messages though.

On console however, messages over network are your only (easy) alternative.

If you wish to listen to messages over network, use the following command in a terminal:

```
cargo skyline listen
```

If this is your first time trying to listen to messages, you will get a warning about having to configure your IP. Follow the instructions displayed in the terminal after finding your IP.

* On console, the IP can be found in the Settings applet, when checking the status of your internet connection.
* On emulator, use the address ``0.0.0.0`` (but prefer looking at the console as it is faster and more reliable)