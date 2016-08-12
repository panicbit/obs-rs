use raw;

pub mod image_file;
pub use self::image_file::ImageFile;

pub mod texture;
pub use self::texture::Texture;

pub mod effect;
pub use self::effect::Effect;

pub mod effect_param;
pub use self::effect_param::EffectParam;

pub fn reset_blend_state() {
    unsafe {
        raw::gs_reset_blend_state();
    }
}

pub fn draw_sprite(texture: &Texture, /* flip,*/ width: u32, height: u32) {
    unsafe {
        raw::gs_draw_sprite(texture.as_raw(), 0, width, height);
    }
}
