## Installation

The first step is to install Rust. Considering the official book already does a good job of explaining the process (as well as being available in multiple languages), please head over to [The Rust Programming Language book](https://doc.rust-lang.org/book/ch01-01-installation.html) to find out how to install it for your device.

> If you prefer to install it by yourself, please visit the [rustup](https://rustup.rs/) page and make sure to pick the "default" option when asked.

### Installing Git

Git is a version control system. It is used to manage your coding projects by taking "snapshots" of your code as key points in time, collaborate with other people, distribution and so on.  
This book will not teach you how to use it, but it is important to have it on your system to follow the next steps smoothly.

Head over to [the official website](https://git-scm.com/downloads) and install the appropriate version for your operating system. The default settings are perfectly fine for our purpose, but feel free to tweak them if you are familiar with it!

### Installing cargo-skyline

Cargo-skyline is a command-line interface tool used to facilitate compiling Rust code for the Nintendo Switch as well as listen to logging messages from your console.

To install it, open a terminal and type the following command:
```console
cargo install cargo-skyline
```

### Preparing a workspace

Once finished, please pick a directory to store your Rust projects in. Do not pick your desktop as your preferred storage, instead, choose an easy to access location such as your Documents or a dedicated hard drive disc with around 5GB of free space at the very least.