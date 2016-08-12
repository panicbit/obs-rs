#include <obs/obs.h>
#include <obs/graphics/graphics.h>
#include <obs/graphics/image-file.h>

gs_image_file_t* rust_gs_image_file_open(char* path) {
    gs_image_file_t* img = bzalloc(sizeof(gs_image_file_t));
    gs_image_file_init(img, path);
    obs_enter_graphics();
    gs_image_file_init_texture(img);
    obs_leave_graphics();
    return img;
}

gs_texture_t* rust_gs_image_file_texture(gs_image_file_t* this) {
    return this->texture;
}

uint32_t rust_gs_image_file_width(gs_image_file_t* this) {
    return this->cx;
}

uint32_t rust_gs_image_file_height(gs_image_file_t* this) {
    return this->cy;
}

void rust_gs_image_file_destroy(gs_image_file_t* this) {
    obs_enter_graphics();
    gs_image_file_free(this);
    obs_leave_graphics();
    bfree(this);
}
