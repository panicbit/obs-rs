use std::ptr;
use std::ops::{Deref, DerefMut};
use raw;

pub struct EffectParam {
    raw: *mut raw::EffectParam
}

impl EffectParam {
    pub unsafe fn from_raw(raw: *mut raw::EffectParam) -> EffectParam {
        EffectParam {
            raw: raw
        }
    }

    pub fn set_texture(&mut self, texture: &super::Texture) {
        unsafe {
            raw::gs_effect_set_texture(self.raw, texture.as_raw());
        }
    }

    unsafe fn forget(&mut self) {
        self.raw = ptr::null_mut();
    }

    pub fn as_raw(&self) -> *mut raw::EffectParam {
        self.raw
    }
}

impl Drop for EffectParam {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // raw::rust_gs_effect_param_destroy(self.raw);
        }
    }
}

pub struct RefMut {
    inner: EffectParam
}

impl RefMut {
    pub unsafe fn from_raw(raw: *const raw::EffectParam) -> Option<Self> {
        if raw.is_null() {
            return None;
        }

        Some(RefMut {
            inner: EffectParam::from_raw(raw as *mut _)
        })
    }
}

impl Deref for RefMut {
    type Target = EffectParam;
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
