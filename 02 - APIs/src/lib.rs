use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, configbasicmenuitem_update_text, ConfigBasicMenuItemGaugeMethods}, BasicMenuResult};

pub mod cobapi {
    use std::ffi::c_void;
    use engage::menu::config::ConfigBasicMenuItem;

    extern "C" {
        fn cobapi_register_configmenuitem_cb(callback: *const c_void);
    }

    pub fn register_configmenuitem(cb: extern "C" fn() -> &'static mut ConfigBasicMenuItem) {
        unsafe { cobapi_register_configmenuitem_cb(cb as _)}
    }
}

pub struct PpSetting;

impl ConfigBasicMenuItemGaugeMethods for PpSetting {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_f(this.gauge_ratio, 0.0, 1.0, 0.1);

        if this.gauge_ratio != result {
            this.gauge_ratio = result;

            unsafe {
                configbasicmenuitem_update_text(this, None);
            }
            Self::set_help_text(this, None);


            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = format!("Current PP level: {}", this.gauge_ratio).into();
    }
}

#[no_mangle]
extern "C" fn pp_slider_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_gauge::<PpSetting>("Plugin API pp meter")
}


#[skyline::main(name = "api")]
pub fn main() {
    println!("Cobalt API example loaded");
    cobapi::register_configmenuitem(pp_slider_callback);
}
