use std::ptr;
use std::ops::Deref;
use raw;

pub struct Texture {
    raw: *mut raw::Texture
}

impl Texture {
    pub unsafe fn from_raw(raw: *mut raw::Texture) -> Texture {
        Texture {
            raw: raw
        }
    }

    unsafe fn forget(&mut self) {
        self.raw = ptr::null_mut();
    }

    pub fn as_raw(&self) -> *mut raw::Texture {
        self.raw
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // raw::rust_gs_texture_destroy(self.raw);
        }
    }
}

pub struct Ref {
    inner: Texture
}

impl Ref {
    pub unsafe fn from_raw(raw: *const raw::Texture) -> Ref {
        Ref {
            inner: Texture::from_raw(raw as *mut _)
        }
    }
}

impl Deref for Ref {
    type Target = Texture;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        unsafe {
            self.inner.forget();
        }
    }
}
