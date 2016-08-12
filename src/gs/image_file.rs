use std::ffi::CString;
use raw;
use gs::texture;

pub struct ImageFile {
    raw: *mut raw::ImageFile
}

impl ImageFile {
    pub fn open(path: &str) -> ImageFile {
        let path = CString::new(path).expect("string contains null");

        unsafe {
            ImageFile {
                raw: raw::rust_gs_image_file_open(path.as_ptr())
            }
        }
    }

    pub fn texture(&self) -> texture::Ref {
        unsafe {
            let texture = raw::rust_gs_image_file_texture(self.raw);
            texture::Ref::from_raw(texture)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            raw::rust_gs_image_file_width(self.raw)
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            raw::rust_gs_image_file_height(self.raw)
        }
    }
}

impl Drop for ImageFile {
    fn drop(&mut self) {
        unsafe {
            raw::rust_gs_image_file_destroy(self.raw);
        }
    }
}
