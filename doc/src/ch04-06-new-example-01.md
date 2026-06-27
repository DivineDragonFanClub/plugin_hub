# New Example Part 1

To begin, create a new skyline project. You can name it whatever you'd like. Open the project in your preferred coding environment and open lib.rs. We can delete the `println!("Hello from skyline plugin");` as we won't need it.

Next, let us add the crates.

### Adding Crates

We will begin with adding the unity crate. We can add the crate by opening a terminal in our project folder or coding enviroment and entering `cargo add --git "https://github.com/DivineDragonFanClub/unity-nx" --tag "0.1.1" `, however this will give us an error, we must enter the package name as well. So to add the unity crate let's run `cargo add --git "https://github.com/DivineDragonFanClub/unity-nx" unity-nx --tag "0.1.1" `. 

The tag is important as it allows us to reference a specific version of the crate, so we can still build if any breaking changes affect our code. 

Next, let's add the engage-il2cpp crate. We can add it the same way as unity, `cargo add --git "https://github.com/DivineDragonFanClub/engage-il2cpp" engage-il2cpp --tag "0.1.1"`. With both these crates installed, let's add the unity-nx crate prelude to our project. To do so, we must add `use unity::prelude::*;` to our code. Add it at the beginning and now our code should look like:
```rs
use unity::prelude::*;

#[skyline::main(name = "book-example-3")]
pub fn main() {
    
}
```
> For calling the crate in Rust code, the names engage and unity will be used. For packages, the names engage-il2cpp and unity-nx will be used.

Additionally, our cargo.toml file should have the two crates we added.

```toml
[package]
name = "book-example-3"
version = "0.1.0"
authors = []
edition = "2021"

[package.metadata.skyline]
titleid = "01006A800016E000" # Smash Ultimate

[lib]
crate-type = ["cdylib"]

[dependencies]
engage-il2cpp = { git = "https://github.com/DivineDragonFanClub/engage-il2cpp", tag = "0.1.1" }
skyline = "0.6.0"
unity-nx = { git = "https://github.com/DivineDragonFanClub/unity-nx", tag = "0.1.1" }

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
lto = true
```

### Hooking a function

Next, we will hook a function. We will use the example from Chapter 3, `App.JobData$$GetLearnJobSkillLevel`. Add a unity hook macro before main with a new function defined below it. You should reference Ghidra to get the arguments for the function.

![Decompiler](assets/decompiler.png)


You can see the two arguments, this and method. As mentioned before, these argument names are very important and we should keep them in our Rust function. This this argument has data type `App_JobData_o`. Luckily, the engage crate has this structure defined already. We can import it by adding `use engage::app::jobdata::JobData`
` below our unity prelude import.

```rs
use unity::prelude::*;
use engage::app::jobdata::JobData;
```

Aftering adding the use for JobData, We will get a error, `cannot find app in engage`. This is because we do not have the feature for JobData enabled. If you have the engage extension previously mentioned, there will also be a warning `engage-il2cpp: missing 1 required feature: app-jobdata`. If you right click this warning in the problems tab you can automatically add all features that were detected to be needed for the code to compile, otherwise you can manually add these features. Once you add a feature, you may get more warning that another feature is needed, you can enable all features until all required features are enabled. For now, our Cargo.toml should look as such:
```toml
[package]
name = "book-example-3"
version = "0.1.0"
authors = []
edition = "2021"

[package.metadata.skyline]
titleid = "01006A800016E000" # Smash Ultimate

[lib]
crate-type = ["cdylib"]

[dependencies]
engage-il2cpp = { git = "https://github.com/DivineDragonFanClub/engage-il2cpp", tag = "0.1.1", features = [
    "app-jobdata",
    "system-collections-generic-dictionary_2",
    "system-collections-generic-list_1",
    "system-object",
] }
skyline = "0.6.0"
unity-nx = { git = "https://github.com/DivineDragonFanClub/unity-nx", tag = "0.1.1" }

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
lto = true
```

With that, we should be ready to write the macro and function. Again, start with the macro call `#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]` then define the function after it. The function can be named anything, but it is important for it to be recognizable to you and others who may read your code. For the arguments, we can define this with `this: JobData` and method with `method_info: OptionalMethod`. OptionalMethod was imported with the unity prelude. The function returns a 32 bit integer, i32 in Rust. With the function and hook defined, we should have as follows:

```rs
#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: JobData, method_info: OptionalMethod) -> i32 {

}
```

Let us add the call original macro to it. The arguments for the call original macro should be this and method_info.

```rs
#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: JobData, method_info: OptionalMethod) -> i32 {
    call_original!(this, method_info)
}
```

If we were to compile this now, we wouldn't see much change. Let's assign the call original to a variable and return that variable.

```rs
#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: JobData, method_info: OptionalMethod) -> i32 {
    let level = call_original!(this, method_info);
    level
}
```

Add a println call before the return to see if we hooked the function correctly.

```rs
#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: JobData, method_info: OptionalMethod) -> i32 {
    let level = call_original!(this, method_info);
    println!("Howdy!");
    level
}
```

In order for the hook to run, we need to install it in main. We can use the skyline install_hook macro to do so.

```rs
#[skyline::main(name = "book-example-3")]
pub fn main() {
    skyline::install_hook!(jobdata_getlearnjobskilllevel);
}
```

Overall our file should look as follows:

```rs
use engage::app::jobdata::JobData;
use unity::prelude::*;

#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: JobData, method_info: OptionalMethod) -> i32 {
    let level = call_original!(this, method_info);
    println!("Howdy");
    level
}
#[skyline::main(name = "book-example-3")]
pub fn main() {
    skyline::install_hook!(jobdata_getlearnjobskilllevel);
}

```

Now compile the program to an nro and install it to your mods folder. Run the game and check the class change screen to see if our hook runs.

![Our hook running](assets/hook-ran.png)

Our hook is running, but it's not doing much yet. In the next part we will rewrite the original function in Rust instead of relying in the call_original macro.
