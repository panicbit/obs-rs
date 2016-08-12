use std::ffi::CString;
use libc::c_void;
use raw;
use gs::effect;

pub trait InputSource: Sized {
    fn id() -> &'static str;
    fn new(/* settings, obs_source */) -> Self;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn render(&self, effect: Option<&mut effect::Effect>);
    fn register() {
        unsafe {
            let id = CString::new(Self::id()).expect("str contains null");

            raw::rust_obs_register_input_source(
                id.into_raw(),
                Self::raw_new,
                Self::raw_width,
                Self::raw_height,
                Self::raw_render,
                Self::raw_destroy,
            )
        }
    }
}

trait RawInputSource: InputSource {
    extern fn raw_new(settings: *mut raw::ObsData, source: *mut raw::ObsSource) -> *mut c_void {
        let this = Self::new();
        let this = Box::new(this);

        Box::into_raw(this) as _
    }

    unsafe extern fn raw_width(this: *mut c_void) -> u32 {
        let this: &Self = &*(this as *mut Self);
        this.width()
    }

    unsafe extern fn raw_height(this: *mut c_void) -> u32 {
        let this: &Self = &*(this as *mut Self);
        this.height()
    }

    unsafe extern fn raw_render(this: *mut c_void, effect: *mut raw::Effect) {
        let this: &Self = &*(this as *mut Self);
        let mut effect = effect::RefMut::from_raw(effect);
        let effect = effect.as_mut().map(|e| &mut **e);
        this.render(effect);
    }

    unsafe extern fn raw_destroy(this: *mut c_void) {
        println!("Rust: Destroying!");
        Box::from_raw(this as *mut Self);
    }
}

impl<T: InputSource> RawInputSource for T {}
