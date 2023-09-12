// Currently needed because we use these functionality, they'll be removable when the Rust language stabilizes them
#![feature(lazy_cell, ptr_sub_ptr)]

// We include the Unity prelude because it contains a few types that get reused all over the place.
// This is a convenience over including every single type by hand.
use unity::prelude::*;

/// The simplest way to hook, which is by immediate address relative to .text
/// This is also the weakest and most unreliable hooking method, as an update to the game would make these invalid.
/// Only rely on this if the other hooking methods don't work.
#[skyline::hook(offset = 0x1BEF870)]
pub fn app_logo_show_image_hook(enable: bool, name: &'static Il2CppString, method_info: OptionalMethod) {
    println!("Boot picture is enabled: {}", enable);

    // Call_original!() is a utility method when hooking that lets you call the function you hooked.
    // You are allowed to provide it your own arguments instead if the need arises.
    // It copies the signature of your hook including the return type, so you can call it if you need the result, for example.
    call_original!(enable, name, method_info)
}

/// The prefered method of hooking, making use of Il2Cpp behind the scenes to get the address.
/// This method is resistant to updates assuming the developers don't remove the function.
/// Assuming something goes wrong, the code will panic and tell you what happened, so it is also more convenient.
/// 
/// If multiple methods have the same name, the one with the same amount of arguments as your hook will be picked in priority.
/// 
/// Some arguments are not counted based on their name, so pay attention to this
/// * this: Use this to represent the class instance if the method is not static
/// * method_info: Every Il2Cpp method receives a MethodInfo as its last argument for inheritance reasons, which you usually would not care about
/// 
/// Appending a _ in front of these argument names is acceptable to remove warnings.
#[unity::hook("App", "MainMenuSequence", "CreateDLCNewsMessage")]
pub fn mainmenusequence_create_dlc_news_message_hook(
    this: &Il2CppObject<()>, // For demonstration sake, we are not going to define the MainMenuSequence type in this example. As explained above, this doesn't count as an argument.
    patch_number: i32, // The argument received by the method
    method_info: OptionalMethod, // Almost every Il2Cpp function receives a MethodInfo as the last argument. As explained above, this doesn't count as an argument.
) -> &'static Il2CppString // The return type of the method.
{
    // We call the original method here to get back the result of it so we can print it.
    let news_message = call_original!(this, patch_number, method_info);
    // You can use the arguments yourself if you need to.
    println!("Patch version of the game: {}", patch_number);
    // We try to print the news message for the currently installed update. We also deal with the situation where there are none, so the game does not crash!
    println!("Update news message returned: {}", news_message.get_string().unwrap_or(String::from("none available")));

    // Lastly, we need to return something to the game, so let's make something up.
    // We are expected to return a Il2CppString, which can be done through the Into<&Il2CppString> implementation on &str. This is mostly for convenience.
    // Notice we don't use a ; at the end, as Rust functions are considered statements.
    // We also don't use the return keyword, as the function (a statement) is evaluated and therefore returns its result automatically.
    "This is a custom news message written from a plugin!".into()
}

#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hooks!(app_logo_show_image_hook, mainmenusequence_create_dlc_news_message_hook);
}
