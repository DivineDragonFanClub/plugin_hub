# Hello, World!

Now that your work environment is ready, you will be creating your first Skyline Plugin in Rust.

> Due to the nature of code modding, you will not be able to run the outcome directly on your computer, but fear not. You will get to test it in due time.

### Creating and Initializing a Skyline Plugin

Using a terminal, navigate to the workspace you previously created during the Installation chapter and type the following command:

```console
cargo skyline new helloworld
```
> The very first time you create a project with cargo-skyline in a directory, you will be prompted to download the source code for the Rust language. It is a necessary step to build Rust programs that are capable of using the Nintendo Switch features and should last between 3 and 10 minutes depending on your download speed.

### Compiling

You've just create a new Skyline Plugin project, so let's go through the compilation steps.

Before loading it through Cobalt, you must compile your plugin using cargo-skyline by entering the following command in the directory:

```console
cargo skyline build --release
```

> If you have already compiled a Rust software previously you'll notice that the build command works the same way, with the extra `skyline` appended to the `cargo` command. This is called
> a [cargo subcommand](https://doc.rust-lang.org/cargo/reference/external-tools.html) and is made available to you by cargo-skyline.

After compiling successfully, Rust outputs a Skyline Plugin in */target/aarch64-skyline-switch/release/libhelloworld.nro* rather than in your current directory.