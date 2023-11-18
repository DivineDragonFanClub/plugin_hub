# Plugin EA

GitHub decided Wikis aren't allowed on private repositories, so explanations will be provided here.

## Setup
> [!WARNING]
> Make sure the Cobalt version you use is [v1.0.0](https://github.com/Raytwo/Cobalt/releases/tag/v0.28.0) or higher.  

Plugins go in the [root of your mod](https://github.com/Raytwo/Cobalt/wiki/Managing-your-Mods) as ``.nro`` files.

1. Install [Rustup](https://rustup.rs/). When asked about what kind of toolchain you want between minimal or complete, pick the middle ground.
2. Open a terminal and run ``cargo install cargo-skyline``
3. Prepare a directory where you'd like to store your plugin's project
4. In that directory, run ``cargo skyline new`` followed by your project's name (no space)
5. You'll be prompted to install the Rust STD. This is a heavy and lengthy download, so confirm and do something else for 5-10 minutes. This is a one time process.

## Build
1. Your project can be built by running ``cargo skyline build --release`` inside of the project's directory.
2. Assuming your code is correct, the plugin can be found in ``./target/aarch64-skyline-switch/release/`` as a ``.nro`` file.

## Work environment
It is recommended to use [VS Code](https://code.visualstudio.com/download) for Rust plugins, as well as the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.

## Resources and documentation
Cobalt only supports Rust plugins (C support will probably not be implemented), so it is recommended to get familiar with the language.  
That being said, Cobalt plugins are not as complex and powerful as a desktop software and the necessary knowledge ceiling will not be the same.

Assuming you already have a some familiarity with system programming (C, C++, Rust, ...), you will have an easier time to adapt.

However, be prepared to lose some of your programming habits as Rust is stricter when it comes to memory and ownership of values and you will definitely get slapped on the wrist for things that you think would've been OK in another language.  
It is mostly explained by the fact that Rust looks out for you and make sure that you don't compile code that *could* break. You might be annoyed by this at first, but see it as a training to write better code in other languages down the road!

### Learning the language
Quick route for impatient people with programming knowledge:
* [Rust by example](https://doc.rust-lang.org/rust-by-example/)

Normal route for beginners or people who would like to understand the language deeper:
* [The Rust Programming Language Book](https://doc.rust-lang.org/book/) (free and online, a physical book exists)

### Plugin-specific
The skyline-rs template and Wiki are a good introduction to the basics of the language and the hooking system
* [Template](https://github.com/ultimate-research/skyline-rs-template)
* [Wiki](https://github.com/ultimate-research/skyline-rs-template/wiki)

### Real-life example(s)
Here are some open-source plugins which can be looked at for reference, do keep in mind that they do not use anything Cobalt-specific.
* [p5rcbt](https://github.com/Raytwo/p5rcbt), good introduction to hooks, inline hooks and organizing a project.
* [ARCropolis](https://github.com/Raytwo/ARCropolis/tree/master), large scale plugin with multiple contributors and a lot of bad decisions. Has a ton of examples when it comes to modding techniques.
* [aldebaran-rs](https://github.com/three-houses-research-team/aldebaran-rs/blob/master/src/forge.rs), FETH modloader, relatively simple to read and follow. Baby's first plugin level of difficulty.

### Crate documentation
Due to the early access aspect of plugin development, some repositories remain privates and that means the documentation cannot be published online.  

As an alternative, you can locally clone the [engage](https://github.com/DivineDragonFanClub/engage) crate and run ``cargo skyline doc -o`` in the repository to get an overview of the library and what it has to offer.  
It will be filled over time with more explanations of the various types and usage instructions.

## Switch crates
Some crates are not compatible with the Switch out of the box for various reasons (OS-specific changes, ...) and it might happen that you cannot build when adding a dependency.  
While this is unfortunate, not much can be done aside from forking and adapting the library to work on Switch if possible.

The [skyline-rs/awesome-libraries](https://github.com/skyline-rs/awesome-libraries) repository contains a few examples of libraries that have been adapted by the community to work on Switch.
