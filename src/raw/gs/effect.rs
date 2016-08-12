use libc::c_char;
use super::Texture;

pub enum Effect {}
pub enum EffectParam {}

extern {
    pub fn gs_effect_get_param_by_name(effect: *mut Effect, name: *const c_char) -> *mut EffectParam;
    pub fn gs_effect_set_texture(effect_param: *mut EffectParam, texture: *mut Texture);
}
