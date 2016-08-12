use libc::{c_void, c_char};
use super::ObsData;

pub enum ObsSource {}

extern {
    pub fn rust_obs_register_input_source(
        id: *const c_char,
        new: extern fn(settings: *mut ObsData, source: *mut ObsSource) -> *mut c_void,
        width: unsafe extern fn(this: *mut c_void) -> u32,
        height: unsafe extern fn(this: *mut c_void) -> u32,
        render: unsafe extern fn(this: *mut c_void, effect: *mut super::Effect),
        destroy: unsafe extern fn(this: *mut c_void),
    );
}
