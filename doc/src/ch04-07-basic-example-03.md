# Basic Example Part 3

### Adding More Logic

Lets add more logic to our hooked function. Lets make it so General learns their skill at level 20 instead of 5.

We can start by adding a nested if else function into the original one, because General should have a max level less than 40. For this example onwards, we will be using the if else example.

To identify General as the class in JobData, we need to check one of it's fields, there are multiple we can check, but we will use JID, as its a unique identifier. General's JID is `JID_ジェネラル`. The jid field in this should match with this if it is General.

> Japanese characters will not print to the console correctly , if you plan to print them, be aware of this.

The jid field is an Il2CppString, as mentioned in the unity crate page, we can use `to_string()` to convert it to a rust string. Alternatively, we can make a new Il2CppString from `JID_ジェネラル`. We can write the function as below:

```rs
#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: &JobData, _method_info: OptionalMethod) -> i32 {
    if this.max_level < 40 {
        if this.jid.to_string() == "JID_ジェネラル" {
            20 
        } else {
            5
        }
    } else {
        25
    }
}
```

Now, we can compile and test ingame to check if General learns it's skill at level 20.

![General Example](assets/general-example.png)

Our added logic works and General learns it's skill at level 20.

### Concluding the Basic Example

Congratulations, you have successfully recreated a function's logic and added your own. As a frame of reference, the example we used for this example is as follows:

```rs
#![feature(ptr_sub_ptr)]

use unity::prelude::*;
use engage::gamedata::JobData;

#[unity::hook("App", "JobData", "GetLearnJobSkillLevel")]
pub fn jobdata_getlearnjobskilllevel(this: &JobData, _method_info: OptionalMethod) -> i32 {
    if this.max_level < 40 {
        if this.jid.to_string() == "JID_ジェネラル" {
            20 
        } else {
            5
        }
    } else {
        25
    }
}


#[skyline::main(name = "book-example")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!(
            "Example plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            67,
            "Example plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hook!(jobdata_getlearnjobskilllevel);
}
```