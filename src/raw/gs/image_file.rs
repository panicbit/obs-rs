use libc::c_char;
use super::Texture;

pub enum ImageFile {}

extern {
    pub fn rust_gs_image_file_open(path: *const c_char) -> *mut ImageFile;
    pub fn rust_gs_image_file_texture(this: *mut ImageFile) -> *mut Texture;
    pub fn rust_gs_image_file_width(this: *mut ImageFile) -> u32;
    pub fn rust_gs_image_file_height(this: *mut ImageFile) -> u32;
    pub fn rust_gs_image_file_destroy(this: *mut ImageFile);
}
