# Unity Crate

The Unit crate is the most useful crate to start with. This crate allows us to interface with Il2Cpp, letting us manipulate Unity proccess and structures. More documentation on the crate can be found [here](https://divinedragonfanclub.github.io/unity/unity/index.html).

 You can find the crate's source code [here](https://github.com/DivineDragonFanClub/unity).

### Prelude

The prelude gives us quick access to the more commonly used structs and classes. Some examples are Il2CppString, a string that functions will use. MethodInfo, the resprentation of a C# Method. We also can use OptionalMethod, a type alias for Option<&MethodInfo>.

Most code mods should import this at the very beginning, as functions will need MethodInfo or OptionalMethod as an input.

### Hooking

Hooking is how we will change the game's functions, it is similar to Skyline hooking. Hooking with the unity crate is a macro, it can be called with `#[unity::hook()]`. For example a real hook would look like`#[unity::hook("App", "Unit", "GetJobName")]`. This macro allows us to hook by a function's symbol name. This method is preferred over Skyline hooks as it is version agonistic, meaning if the game were to update, then the hook would still be able to function. Sometimes, a Unity hook may not be the ideal solution or not function properly, so Skyline hooks still may be required. However, if a Unity hook can be used, then you should use it for most cases.

The hook macro must be run before a function, for example:

```rs
#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, method_info: OptionalMethod) -> &'static Il2CppString {
    call_original!(this, method_info)
}
```

The `call_original!()` is another macro, it calls the original function, so this hook is not doing much. However, we can add more logic before or after the call original macro, or entirely change the function and not call the macro at all. the arguments names this and method_info are important to note, but we will explore those more later.

### From Offset

From Offset is another method to access the game's functions. It is called in the same way as a hook, `#[unity::from_offset()]` and the inputs of the function are the same as hook, using the symbol's name. This allows us to access a function without needing to use the call original macro. One caveat to this, is that calling this function would require an unsafe block. The example below shows how the from_offset macro can be used:

```rs
#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, method_info: OptionalMethod) -> &'static Il2CppString;
```

### This and Method Info

Often you will see this and method in the Ghidra decompiler as arguments for functions, most functions have these somewhere in their arguments often with this being the first and method being last. It is essentially you follow the naming scheme for these arguments. The hook will fail if these arguments are present in the Ghidra decompile and not present in the Rust function. You can see [here](https://github.com/DivineDragonFanClub/unity/blob/bfa366f25c7c4450a5acf98098ff4ec570d27dc7/unity-macro/src/scan.rs#L54) why these arguments names are important for determining the argument count of the hooked function. Appending an underscore to these arguments if they are not used will typically be fine.

### Class

Class is another macro to help us define structures in Rust. Much like with hook with a function, it should be called before defining a structure. You can call it with the macro `#[unity::class()]`. An example of a real class macro call would look like `#[unity::class("App", "InfoUtil_StatusSkill")]`. Doing this simply associates this structure with the one in the code, if we want to access it's field, we still need to define them in our Rust code. You should open the data type and reference the fields in Ghidra in order to properly define the fields. You may add padding to define fields that you don't need but are between fields you do, just be sure the offsets and sizes are correct or the fields will be misaligned. Below is an example of a defined structure:

```rs
#[unity::class("App", "InfoUtil_StatusSkill")]
pub struct StatusSkill {
    pub data: Option<&'static SkillData>,
    pub isactive: bool,
    pub category: i32,
}
```

### Il2CppString

An Il2CppString is a representation of an C# string used by Il2Cpp. It is a type alias for `Il2CppObject<SystemString>`. We can create a new Il2CppString using the `new()` function and with a str as the input. To convert from an Il2CppString to a String in Rust, we must use the function `to_string()`