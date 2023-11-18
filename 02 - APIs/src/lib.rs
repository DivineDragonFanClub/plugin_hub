use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemGaugeMethods}, BasicMenuResult};

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
            // The setting didn't change, just return a empty result.
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

#[no_mangle]
extern "C" fn pp_slider_callback() -> &'static mut ConfigBasicMenuItem {
    // Your callback must return a ConfigBasicMenu, which you can acquire by using new_gauge or new_switch.

    ConfigBasicMenuItem::new_gauge::<PpSetting>("Plugin API pp meter")
}


#[skyline::main(name = "api")]
pub fn main() {
    println!("Cobalt API example loaded");
    cobapi::install_game_setting(pp_slider_callback);
}
