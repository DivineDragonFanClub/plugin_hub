use unity::prelude::*;

use engage::menu::{
    BasicMenuResult,
    config::{
        ConfigBasicMenuItem,
        ConfigBasicMenuItemGaugeMethods
    }
};

// These use statements make it so you don't have to write cobapi::Event every time you want to use the Event type.
use cobapi::{
    Event,
    SystemEvent,
};

////////////////////////////////////////////////////////
// About custom settings
///////////////////////////////////////////////////////

// This empty structure is here to represent your setting.
// It is to be used to implement ConfigBasicMenuItemGaugeMethods/ConfigBasicMenuItemSwitchMethods
pub struct PpSetting;

impl ConfigBasicMenuItemGaugeMethods for PpSetting {
    // This function can be used to set default values on your setting prior to it being added to the menu.
    fn init_content(this: &mut ConfigBasicMenuItem) {
        this.gauge_ratio = 0.5;
    }

    // This function is called every frame while the setting is hovered.
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        // Call change_key_value so the game can check if the user is changing the setting, and compare with the previous value to know if they did.
        let result = ConfigBasicMenuItem::change_key_value_f(this.gauge_ratio, 0.0, 1.0, 0.1);

        // Comparing the value the game is reporting with the previous one.
        if this.gauge_ratio != result {
            // The value changed, so we assign it.
            this.gauge_ratio = result;
            
            // Change the help text by calling our set_help_text method.
            Self::set_help_text(this, None);
            // Ask the setting to refresh the text to display.
            this.update_text();

            // Play a confirmation sound.
            BasicMenuResult::se_cursor()
        } else {
            // The setting didn't change, just return a empty result that does nothing.
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        // This function is to be called when a value changes to reflect it in the help message.
        // You can read the setting if needed.

        if this.gauge_ratio <= 0.0 {
            this.help_text = "No PP level...".into();
        } else {
            this.help_text = format!("Current PP level: {}", this.gauge_ratio).into();
        }
    }
}

#[no_mangle] // no_mangle is an attribute used to ask Rust not to modify your function name to facilitate communication with code from other sources.
extern "C" fn pp_slider_callback() -> &'static mut ConfigBasicMenuItem {
    // Your callback must return a ConfigBasicMenu, which you can acquire by using new_gauge or new_switch.
    ConfigBasicMenuItem::new_gauge::<PpSetting>("Plugin API pp meter")
}

////////////////////////////////////////////////////////
// About Event Listeners
////////////////////////////////////////////////////////


#[no_mangle]
extern "C" fn my_system_event_listener(event: &Event<SystemEvent>) {
    // Here is an example of how to properly deal with everything you're receiving.
    match event {
        Event::Args(ev) => match ev {
            SystemEvent::CatalogLoaded => println!("If you only care about knowing when files have been added to the game, handle it here."),
            SystemEvent::SaveLoaded { slot_id } => {
                println!("Events can sometimes carry values that you can use. This one tells you the slot of the save being loaded!");
                println!("The slot being loaded was #{}", slot_id);
            },
            // This syntax means you do not intend to deal with the other events and will do nothing if they are received.
            _ => (),
        },
        Event::Missing => panic!("Handle the case where the event is missing here, if you care."),
    }

    // If you're rather do it brief and ignore missing events, the following is also acceptable.
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::CatalogLoaded => println!("If you only care about knowing when files have been added to the game, handle it here."),
            SystemEvent::SaveLoaded { slot_id } => {
                println!("Events can sometimes carry values that you can use. This one tells you the slot of the save being loaded!");
                println!("The slot being loaded was #{}", slot_id);
            },
            // This syntax means you do not intend to deal with the other events and will do nothing if they are received.
            _ => (),
        }
    } else {
        println!("We received a missing event, and we don't care!");
    }

    // If you only care about a single event, the following is fine too.
    if let Event::Args(ev) = event {
        if let SystemEvent::CatalogLoaded = ev {
            println!("If you only care about knowing when files have been added to the game, handle it here.")
        } else {
            println!("We received a valid event that is not CatalogLoaded, and we don't care!");
        }
    } else {
        println!("We received a missing event, and we don't care!");
    }

    // Note that you do NOT need the else statement if you do not want to handle this case.

    // Assuming you want to stop listening to SystemEvents for any reason, you can call the following method.
    cobapi::unregister_system_event_handler(my_system_event_listener);
}


#[skyline::main(name = "api")]
pub fn main() {
    println!("Cobalt API example loaded");
    // Adds your setting to the Settings menu, in-game.
    cobapi::install_game_setting(pp_slider_callback);
    // Adds your setting tot he Global Settings menu, in the Cobalt menu (located on the title screen).
    cobapi::install_global_game_setting(pp_slider_callback);
    // Register your SystemEvent listener.
    cobapi::register_system_event_handler(my_system_event_listener);
}
