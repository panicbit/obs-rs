use std::ptr;
use std::ops::{Deref, DerefMut};
use std::ffi::CString;
use raw;
use super::effect_param;

pub struct Effect {
    raw: *mut raw::Effect
}

impl Effect {
    pub unsafe fn from_raw(raw: *mut raw::Effect) -> Effect {
        Effect {
            raw: raw
        }
    }

    pub fn param_by_name(&mut self, name: &str) -> Option<effect_param::RefMut> {
        let name = CString::new(name).expect("str contains null");

        unsafe {
            let param = raw::gs_effect_get_param_by_name(self.raw, name.as_ptr());
            effect_param::RefMut::from_raw(param)
        }
    }

    unsafe fn forget(&mut self) {
        self.raw = ptr::null_mut();
    }

    pub fn as_raw(&self) -> *mut raw::Effect {
        self.raw
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // raw::rust_gs_effect_destroy(self.raw);
        }
    }
}

pub struct RefMut {
    inner: Effect
}

impl RefMut {
    pub unsafe fn from_raw(raw: *const raw::Effect) -> Option<RefMut> {
        if raw.is_null() {
            return None;
        }

        Some(RefMut {
            inner: Effect::from_raw(raw as *mut _)
        })
    }
}

impl Deref for RefMut {
    type Target = Effect;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RefMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl Drop for RefMut {
    fn drop(&mut self) {
        unsafe {
            self.inner.forget();
        }
    }
}
