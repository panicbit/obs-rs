extern crate libc;
pub mod source;
pub mod gs;
pub mod raw;

pub use source::InputSource;
pub use gs::Effect;

#[macro_export]
macro_rules! obs_module {
    ($INIT_FN:ident) => (
        #[no_mangle]
        pub extern fn obs_module_load() -> bool {
            $INIT_FN()
        }
    )
}
