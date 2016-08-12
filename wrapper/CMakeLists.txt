cmake_minimum_required (VERSION 2.6)
project (obs_plugin_wrapper)

set (CMAKE_POSITION_INDEPENDENT_CODE ON)

add_library (obs_plugin_wrapper STATIC
    plugin.c
    gs/image_file.c
)

install (TARGETS obs_plugin_wrapper ARCHIVE DESTINATION .)
