#include <obs/obs-module.h>
#include <obs/graphics/image-file.h>

OBS_DECLARE_MODULE();

static const char* demo_source_get_name(void *type_data) {
    return "Ruuuuuust 😁";
}

static void demo_source_render(void *data, gs_effect_t *effect) {
    gs_image_file_t* img = data;

    gs_reset_blend_state();
    gs_effect_set_texture(gs_effect_get_param_by_name(effect, "image"), img->texture);
    gs_draw_sprite(img->texture, 0, img->cx, img->cy);
}

void rust_obs_register_input_source(
    char* id,
    void* (*create)(),
    uint32_t (*get_width)(void* this),
    uint32_t (*get_height)(void* this),
    void (*render_video)(void* this, gs_effect_t*),
    void (*destroy)(void* this)
) {
    struct obs_source_info init = {
        .id             = id,
        .type           = OBS_SOURCE_TYPE_INPUT,
        .output_flags   = OBS_SOURCE_VIDEO,
        .get_name       = demo_source_get_name,
        .create         = create,
        .destroy        = destroy,
        .update         = NULL,
        .get_defaults   = NULL,
        .show           = NULL,
        .hide           = NULL,
        .get_width      = get_width,
        .get_height     = get_height,
        .video_render   = render_video,
        .video_tick     = NULL,
        .get_properties = NULL,
    };
    struct obs_source_info* info = malloc(sizeof(struct obs_source_info));
    *info = init;
    obs_register_source(info);
}
