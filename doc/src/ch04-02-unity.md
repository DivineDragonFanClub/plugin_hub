# Unity Crate

The Unit crate is the most useful crate to start with. This crate allows us to interface with Il2Cpp, letting us manipulate Unity proccess and structures. More documentation on the crate can be found [here](https://divinedragonfanclub.github.io/unity/unity/index.html).

### Prelude

The prelude gives us quick access to the more commonly used structs and classes. Some examples are Il2CppString, a string that functions will use. MethodInfo, the resprentation of a C# Method. We also can use OptionalMethod, a type alias for Option<&MethodInfo>.

Most code mods should import this at the very beginning, as functions will need MethodInfo or OptionalMethod as an input.

### Hooking

Hooking is how we will change the game's functions, it is similar to Skyline hooking. Hooking with the unity crate is a macro, it can be called with `#unity::hook()`. For example a real hook would look like`#[unity::hook("App", "Unit", "GetJobName")]`. This macro allows us to hook by a function's symbol name. This method is preferred over Skyline hooks as it is version agonistic, meaning if the game were to update, then the hook would still be able to function. Sometimes, a Unity hook may not be the ideal solution or not function properly, so Skyline hooks still may be required. However, if a Unity hook can be used, then you should use it for most cases.

The hook macro must be run before a function, for example:

```rs
#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, method_info: OptionalMethod) -> &'static Il2CppString {
    call_original!(this, method_info)
}
```

The `call_original!()` is another macro, it calls the original function, so this hook is not doing much. However, we can add more logic before or after the call original macro, or entirely change the function and not call the macro at all. the input names this and method_info are important to note, but we will explore those more later.

### From Offset

From Offset is another method to access the game's functions. It is called in the same way as a hook, `#[unity::from_offset()]` and the inputs of the function are the same as hook, using the symbol's name. This allows us to access a function without needing to use the call original macro. One caveat to this, is that calling this function would require an unsafe block. The example below shows how the from_offset macro can be used:

```rs
#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, method_info: OptionalMethod) -> &'static Il2CppString;
```