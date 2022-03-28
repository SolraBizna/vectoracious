#!/bin/sh

set -e

if [ $# -ne 1 ]; then
    echo "Usage: regen_bindings.sh path/to/gl.xml"
    echo ""
    echo "gl.xml is the official Khronos XML representation of the OpenGL API."
    echo "At the time of this writing, it can be found at:"
    echo ""
    echo "https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/main/xml/gl.xml"
    exit 1
fi

#rglgen "$1" -t gl2.1 GL_ARB_color_buffer_float GL_ARB_half_float_pixel > src/render/gl21/binding.rs
rglgen "$1" -t gl3.2 GL_ARB_debug_output > src/render/gl32/binding.rs
