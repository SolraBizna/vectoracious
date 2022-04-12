#![allow(dead_code,non_snake_case,non_upper_case_globals,unused_imports)]

/// This module was generated using the rglgen crate.
/// It is a binding for OpenGL 3.2.
///
/// It includes support for the following extensions:
/// - GL_ARB_debug_output

// The following comments are from the source XML file. It refers to that file,
// not this generated Rust code. Nevertheless, valuable copyright and
// provenance data may be present.
//
// Copyright 2013-2020 The Khronos Group Inc.
// SPDX-License-Identifier: Apache-2.0
//
// This file, gl.xml, is the OpenGL and OpenGL API Registry. The canonical
// version of the registry, together with documentation, schema, and Python
// generator scripts used to generate C header files for OpenGL and OpenGL ES,
// can always be found in the Khronos Registry at
// https://github.com/KhronosGroup/OpenGL-Registry
//

// *** TYPES ***
use libc;
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
// Not an actual GL type, though used in headers in the past
pub type GLvoid = libc::c_void;
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
pub type GLchar = libc::c_char;
pub type GLhalf = u16;
pub type GLintptr = usize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLsync = *mut libc::c_void;
pub type GLDEBUGPROCARB = Option<extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *const libc::c_void)>;

// *** VALUES ***
pub const GL_2D: u32 = 0x600;
pub const GL_2_BYTES: u32 = 0x1407;
pub const GL_3D: u32 = 0x601;
pub const GL_3D_COLOR: u32 = 0x602;
pub const GL_3D_COLOR_TEXTURE: u32 = 0x603;
pub const GL_3_BYTES: u32 = 0x1408;
pub const GL_4D_COLOR_TEXTURE: u32 = 0x604;
pub const GL_4_BYTES: u32 = 0x1409;
pub const GL_ACCUM: u32 = 0x100;
pub const GL_ACCUM_ALPHA_BITS: u32 = 0xd5b;
pub const GL_ACCUM_BLUE_BITS: u32 = 0xd5a;
pub const GL_ACCUM_BUFFER_BIT: u32 = 0x200;
pub const GL_ACCUM_CLEAR_VALUE: u32 = 0xb80;
pub const GL_ACCUM_GREEN_BITS: u32 = 0xd59;
pub const GL_ACCUM_RED_BITS: u32 = 0xd58;
pub const GL_ACTIVE_ATTRIBUTES: u32 = 0x8b89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: u32 = 0x8b8a;
pub const GL_ACTIVE_TEXTURE: u32 = 0x84e0;
pub const GL_ACTIVE_UNIFORMS: u32 = 0x8b86;
pub const GL_ACTIVE_UNIFORM_BLOCKS: u32 = 0x8a36;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: u32 = 0x8a35;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: u32 = 0x8b87;
pub const GL_ADD: u32 = 0x104;
pub const GL_ADD_SIGNED: u32 = 0x8574;
pub const GL_ALIASED_LINE_WIDTH_RANGE: u32 = 0x846e;
pub const GL_ALIASED_POINT_SIZE_RANGE: u32 = 0x846d;
pub const GL_ALL_ATTRIB_BITS: u32 = 0xffffffff;
pub const GL_ALPHA: u32 = 0x1906;
pub const GL_ALPHA12: u32 = 0x803d;
pub const GL_ALPHA16: u32 = 0x803e;
pub const GL_ALPHA4: u32 = 0x803b;
pub const GL_ALPHA8: u32 = 0x803c;
pub const GL_ALPHA_BIAS: u32 = 0xd1d;
pub const GL_ALPHA_BITS: u32 = 0xd55;
pub const GL_ALPHA_INTEGER: u32 = 0x8d97;
pub const GL_ALPHA_SCALE: u32 = 0xd1c;
pub const GL_ALPHA_TEST: u32 = 0xbc0;
pub const GL_ALPHA_TEST_FUNC: u32 = 0xbc1;
pub const GL_ALPHA_TEST_REF: u32 = 0xbc2;
pub const GL_ALREADY_SIGNALED: u32 = 0x911a;
pub const GL_ALWAYS: u32 = 0x207;
pub const GL_AMBIENT: u32 = 0x1200;
pub const GL_AMBIENT_AND_DIFFUSE: u32 = 0x1602;
pub const GL_AND: u32 = 0x1501;
pub const GL_AND_INVERTED: u32 = 0x1504;
pub const GL_AND_REVERSE: u32 = 0x1502;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_ARRAY_BUFFER_BINDING: u32 = 0x8894;
pub const GL_ATTACHED_SHADERS: u32 = 0x8b85;
pub const GL_ATTRIB_STACK_DEPTH: u32 = 0xbb0;
pub const GL_AUTO_NORMAL: u32 = 0xd80;
pub const GL_AUX0: u32 = 0x409;
pub const GL_AUX1: u32 = 0x40a;
pub const GL_AUX2: u32 = 0x40b;
pub const GL_AUX3: u32 = 0x40c;
pub const GL_AUX_BUFFERS: u32 = 0xc00;
pub const GL_BACK: u32 = 0x405;
pub const GL_BACK_LEFT: u32 = 0x402;
pub const GL_BACK_RIGHT: u32 = 0x403;
pub const GL_BGR: u32 = 0x80e0;
pub const GL_BGRA: u32 = 0x80e1;
pub const GL_BGRA_INTEGER: u32 = 0x8d9b;
pub const GL_BGR_INTEGER: u32 = 0x8d9a;
pub const GL_BITMAP: u32 = 0x1a00;
pub const GL_BITMAP_TOKEN: u32 = 0x704;
pub const GL_BLEND: u32 = 0xbe2;
pub const GL_BLEND_COLOR: u32 = 0x8005;
pub const GL_BLEND_DST: u32 = 0xbe0;
pub const GL_BLEND_DST_ALPHA: u32 = 0x80ca;
pub const GL_BLEND_DST_RGB: u32 = 0x80c8;
pub const GL_BLEND_EQUATION: u32 = 0x8009;
pub const GL_BLEND_EQUATION_ALPHA: u32 = 0x883d;
pub const GL_BLEND_EQUATION_RGB: u32 = 0x8009;
pub const GL_BLEND_SRC: u32 = 0xbe1;
pub const GL_BLEND_SRC_ALPHA: u32 = 0x80cb;
pub const GL_BLEND_SRC_RGB: u32 = 0x80c9;
pub const GL_BLUE: u32 = 0x1905;
pub const GL_BLUE_BIAS: u32 = 0xd1b;
pub const GL_BLUE_BITS: u32 = 0xd54;
pub const GL_BLUE_INTEGER: u32 = 0x8d96;
pub const GL_BLUE_SCALE: u32 = 0xd1a;
pub const GL_BOOL: u32 = 0x8b56;
pub const GL_BOOL_VEC2: u32 = 0x8b57;
pub const GL_BOOL_VEC3: u32 = 0x8b58;
pub const GL_BOOL_VEC4: u32 = 0x8b59;
pub const GL_BUFFER_ACCESS: u32 = 0x88bb;
pub const GL_BUFFER_ACCESS_FLAGS: u32 = 0x911f;
pub const GL_BUFFER_MAPPED: u32 = 0x88bc;
pub const GL_BUFFER_MAP_LENGTH: u32 = 0x9120;
pub const GL_BUFFER_MAP_OFFSET: u32 = 0x9121;
pub const GL_BUFFER_MAP_POINTER: u32 = 0x88bd;
pub const GL_BUFFER_SIZE: u32 = 0x8764;
pub const GL_BUFFER_USAGE: u32 = 0x8765;
pub const GL_BYTE: u32 = 0x1400;
pub const GL_C3F_V3F: u32 = 0x2a24;
pub const GL_C4F_N3F_V3F: u32 = 0x2a26;
pub const GL_C4UB_V2F: u32 = 0x2a22;
pub const GL_C4UB_V3F: u32 = 0x2a23;
pub const GL_CCW: u32 = 0x901;
pub const GL_CLAMP: u32 = 0x2900;
pub const GL_CLAMP_FRAGMENT_COLOR: u32 = 0x891b;
pub const GL_CLAMP_READ_COLOR: u32 = 0x891c;
pub const GL_CLAMP_TO_BORDER: u32 = 0x812d;
pub const GL_CLAMP_TO_EDGE: u32 = 0x812f;
pub const GL_CLAMP_VERTEX_COLOR: u32 = 0x891a;
pub const GL_CLEAR: u32 = 0x1500;
pub const GL_CLIENT_ACTIVE_TEXTURE: u32 = 0x84e1;
pub const GL_CLIENT_ALL_ATTRIB_BITS: u32 = 0xffffffff;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: u32 = 0xbb1;
pub const GL_CLIENT_PIXEL_STORE_BIT: u32 = 0x1;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: u32 = 0x2;
pub const GL_CLIP_DISTANCE6: u32 = 0x3006;
pub const GL_CLIP_DISTANCE7: u32 = 0x3007;
pub const GL_CLIP_PLANE0: u32 = 0x3000;
pub const GL_CLIP_PLANE1: u32 = 0x3001;
pub const GL_CLIP_PLANE2: u32 = 0x3002;
pub const GL_CLIP_PLANE3: u32 = 0x3003;
pub const GL_CLIP_PLANE4: u32 = 0x3004;
pub const GL_CLIP_PLANE5: u32 = 0x3005;
pub const GL_COEFF: u32 = 0xa00;
pub const GL_COLOR: u32 = 0x1800;
pub const GL_COLOR_ARRAY: u32 = 0x8076;
pub const GL_COLOR_ARRAY_BUFFER_BINDING: u32 = 0x8898;
pub const GL_COLOR_ARRAY_POINTER: u32 = 0x8090;
pub const GL_COLOR_ARRAY_SIZE: u32 = 0x8081;
pub const GL_COLOR_ARRAY_STRIDE: u32 = 0x8083;
pub const GL_COLOR_ARRAY_TYPE: u32 = 0x8082;
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8ce0;
pub const GL_COLOR_ATTACHMENT1: u32 = 0x8ce1;
pub const GL_COLOR_ATTACHMENT10: u32 = 0x8cea;
pub const GL_COLOR_ATTACHMENT11: u32 = 0x8ceb;
pub const GL_COLOR_ATTACHMENT12: u32 = 0x8cec;
pub const GL_COLOR_ATTACHMENT13: u32 = 0x8ced;
pub const GL_COLOR_ATTACHMENT14: u32 = 0x8cee;
pub const GL_COLOR_ATTACHMENT15: u32 = 0x8cef;
pub const GL_COLOR_ATTACHMENT16: u32 = 0x8cf0;
pub const GL_COLOR_ATTACHMENT17: u32 = 0x8cf1;
pub const GL_COLOR_ATTACHMENT18: u32 = 0x8cf2;
pub const GL_COLOR_ATTACHMENT19: u32 = 0x8cf3;
pub const GL_COLOR_ATTACHMENT2: u32 = 0x8ce2;
pub const GL_COLOR_ATTACHMENT20: u32 = 0x8cf4;
pub const GL_COLOR_ATTACHMENT21: u32 = 0x8cf5;
pub const GL_COLOR_ATTACHMENT22: u32 = 0x8cf6;
pub const GL_COLOR_ATTACHMENT23: u32 = 0x8cf7;
pub const GL_COLOR_ATTACHMENT24: u32 = 0x8cf8;
pub const GL_COLOR_ATTACHMENT25: u32 = 0x8cf9;
pub const GL_COLOR_ATTACHMENT26: u32 = 0x8cfa;
pub const GL_COLOR_ATTACHMENT27: u32 = 0x8cfb;
pub const GL_COLOR_ATTACHMENT28: u32 = 0x8cfc;
pub const GL_COLOR_ATTACHMENT29: u32 = 0x8cfd;
pub const GL_COLOR_ATTACHMENT3: u32 = 0x8ce3;
pub const GL_COLOR_ATTACHMENT30: u32 = 0x8cfe;
pub const GL_COLOR_ATTACHMENT31: u32 = 0x8cff;
pub const GL_COLOR_ATTACHMENT4: u32 = 0x8ce4;
pub const GL_COLOR_ATTACHMENT5: u32 = 0x8ce5;
pub const GL_COLOR_ATTACHMENT6: u32 = 0x8ce6;
pub const GL_COLOR_ATTACHMENT7: u32 = 0x8ce7;
pub const GL_COLOR_ATTACHMENT8: u32 = 0x8ce8;
pub const GL_COLOR_ATTACHMENT9: u32 = 0x8ce9;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
pub const GL_COLOR_CLEAR_VALUE: u32 = 0xc22;
pub const GL_COLOR_INDEX: u32 = 0x1900;
pub const GL_COLOR_INDEXES: u32 = 0x1603;
pub const GL_COLOR_LOGIC_OP: u32 = 0xbf2;
pub const GL_COLOR_MATERIAL: u32 = 0xb57;
pub const GL_COLOR_MATERIAL_FACE: u32 = 0xb55;
pub const GL_COLOR_MATERIAL_PARAMETER: u32 = 0xb56;
pub const GL_COLOR_SUM: u32 = 0x8458;
pub const GL_COLOR_WRITEMASK: u32 = 0xc23;
pub const GL_COMBINE: u32 = 0x8570;
pub const GL_COMBINE_ALPHA: u32 = 0x8572;
pub const GL_COMBINE_RGB: u32 = 0x8571;
pub const GL_COMPARE_R_TO_TEXTURE: u32 = 0x884e;
pub const GL_COMPILE: u32 = 0x1300;
pub const GL_COMPILE_AND_EXECUTE: u32 = 0x1301;
pub const GL_COMPILE_STATUS: u32 = 0x8b81;
pub const GL_COMPRESSED_ALPHA: u32 = 0x84e9;
pub const GL_COMPRESSED_INTENSITY: u32 = 0x84ec;
pub const GL_COMPRESSED_LUMINANCE: u32 = 0x84ea;
pub const GL_COMPRESSED_LUMINANCE_ALPHA: u32 = 0x84eb;
pub const GL_COMPRESSED_RED: u32 = 0x8225;
pub const GL_COMPRESSED_RED_RGTC1: u32 = 0x8dbb;
pub const GL_COMPRESSED_RG: u32 = 0x8226;
pub const GL_COMPRESSED_RGB: u32 = 0x84ed;
pub const GL_COMPRESSED_RGBA: u32 = 0x84ee;
pub const GL_COMPRESSED_RG_RGTC2: u32 = 0x8dbd;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8dbc;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: u32 = 0x8dbe;
pub const GL_COMPRESSED_SLUMINANCE: u32 = 0x8c4a;
pub const GL_COMPRESSED_SLUMINANCE_ALPHA: u32 = 0x8c4b;
pub const GL_COMPRESSED_SRGB: u32 = 0x8c48;
pub const GL_COMPRESSED_SRGB_ALPHA: u32 = 0x8c49;
pub const GL_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86a3;
pub const GL_CONDITION_SATISFIED: u32 = 0x911c;
pub const GL_CONSTANT: u32 = 0x8576;
pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
pub const GL_CONSTANT_ATTENUATION: u32 = 0x1207;
pub const GL_CONSTANT_COLOR: u32 = 0x8001;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: u32 = 0x2;
pub const GL_CONTEXT_CORE_PROFILE_BIT: u32 = 0x1;
pub const GL_CONTEXT_FLAGS: u32 = 0x821e;
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: u32 = 0x1;
pub const GL_CONTEXT_PROFILE_MASK: u32 = 0x9126;
pub const GL_COORD_REPLACE: u32 = 0x8862;
pub const GL_COPY: u32 = 0x1503;
pub const GL_COPY_INVERTED: u32 = 0x150c;
pub const GL_COPY_PIXEL_TOKEN: u32 = 0x706;
pub const GL_COPY_READ_BUFFER: u32 = 0x8f36;
pub const GL_COPY_WRITE_BUFFER: u32 = 0x8f37;
pub const GL_CULL_FACE: u32 = 0xb44;
pub const GL_CULL_FACE_MODE: u32 = 0xb45;
pub const GL_CURRENT_BIT: u32 = 0x1;
pub const GL_CURRENT_COLOR: u32 = 0xb00;
pub const GL_CURRENT_FOG_COORDINATE: u32 = 0x8453;
pub const GL_CURRENT_INDEX: u32 = 0xb01;
pub const GL_CURRENT_NORMAL: u32 = 0xb02;
pub const GL_CURRENT_PROGRAM: u32 = 0x8b8d;
pub const GL_CURRENT_QUERY: u32 = 0x8865;
pub const GL_CURRENT_RASTER_COLOR: u32 = 0xb04;
pub const GL_CURRENT_RASTER_DISTANCE: u32 = 0xb09;
pub const GL_CURRENT_RASTER_INDEX: u32 = 0xb05;
pub const GL_CURRENT_RASTER_POSITION: u32 = 0xb07;
pub const GL_CURRENT_RASTER_POSITION_VALID: u32 = 0xb08;
pub const GL_CURRENT_RASTER_SECONDARY_COLOR: u32 = 0x845f;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: u32 = 0xb06;
pub const GL_CURRENT_SECONDARY_COLOR: u32 = 0x8459;
pub const GL_CURRENT_TEXTURE_COORDS: u32 = 0xb03;
pub const GL_CURRENT_VERTEX_ATTRIB: u32 = 0x8626;
pub const GL_CW: u32 = 0x900;
pub const GL_DEBUG_CALLBACK_FUNCTION_ARB: u32 = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB: u32 = 0x8245;
pub const GL_DEBUG_LOGGED_MESSAGES_ARB: u32 = 0x9145;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: u32 = 0x8243;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB: u32 = 0x8242;
pub const GL_DEBUG_SEVERITY_HIGH_ARB: u32 = 0x9146;
pub const GL_DEBUG_SEVERITY_LOW_ARB: u32 = 0x9148;
pub const GL_DEBUG_SEVERITY_MEDIUM_ARB: u32 = 0x9147;
pub const GL_DEBUG_SOURCE_API_ARB: u32 = 0x8246;
pub const GL_DEBUG_SOURCE_APPLICATION_ARB: u32 = 0x824a;
pub const GL_DEBUG_SOURCE_OTHER_ARB: u32 = 0x824b;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB: u32 = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB: u32 = 0x8249;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB: u32 = 0x8247;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: u32 = 0x824d;
pub const GL_DEBUG_TYPE_ERROR_ARB: u32 = 0x824c;
pub const GL_DEBUG_TYPE_OTHER_ARB: u32 = 0x8251;
pub const GL_DEBUG_TYPE_PERFORMANCE_ARB: u32 = 0x8250;
pub const GL_DEBUG_TYPE_PORTABILITY_ARB: u32 = 0x824f;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: u32 = 0x824e;
pub const GL_DECAL: u32 = 0x2101;
pub const GL_DECR: u32 = 0x1e03;
pub const GL_DECR_WRAP: u32 = 0x8508;
pub const GL_DELETE_STATUS: u32 = 0x8b80;
pub const GL_DEPTH: u32 = 0x1801;
pub const GL_DEPTH24_STENCIL8: u32 = 0x88f0;
pub const GL_DEPTH32F_STENCIL8: u32 = 0x8cad;
pub const GL_DEPTH_ATTACHMENT: u32 = 0x8d00;
pub const GL_DEPTH_BIAS: u32 = 0xd1f;
pub const GL_DEPTH_BITS: u32 = 0xd56;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x100;
pub const GL_DEPTH_CLAMP: u32 = 0x864f;
pub const GL_DEPTH_CLEAR_VALUE: u32 = 0xb73;
pub const GL_DEPTH_COMPONENT: u32 = 0x1902;
pub const GL_DEPTH_COMPONENT16: u32 = 0x81a5;
pub const GL_DEPTH_COMPONENT24: u32 = 0x81a6;
pub const GL_DEPTH_COMPONENT32: u32 = 0x81a7;
pub const GL_DEPTH_COMPONENT32F: u32 = 0x8cac;
pub const GL_DEPTH_FUNC: u32 = 0xb74;
pub const GL_DEPTH_RANGE: u32 = 0xb70;
pub const GL_DEPTH_SCALE: u32 = 0xd1e;
pub const GL_DEPTH_STENCIL: u32 = 0x84f9;
pub const GL_DEPTH_STENCIL_ATTACHMENT: u32 = 0x821a;
pub const GL_DEPTH_TEST: u32 = 0xb71;
pub const GL_DEPTH_TEXTURE_MODE: u32 = 0x884b;
pub const GL_DEPTH_WRITEMASK: u32 = 0xb72;
pub const GL_DIFFUSE: u32 = 0x1201;
pub const GL_DITHER: u32 = 0xbd0;
pub const GL_DOMAIN: u32 = 0xa02;
pub const GL_DONT_CARE: u32 = 0x1100;
pub const GL_DOT3_RGB: u32 = 0x86ae;
pub const GL_DOT3_RGBA: u32 = 0x86af;
pub const GL_DOUBLE: u32 = 0x140a;
pub const GL_DOUBLEBUFFER: u32 = 0xc32;
pub const GL_DRAW_BUFFER: u32 = 0xc01;
pub const GL_DRAW_BUFFER0: u32 = 0x8825;
pub const GL_DRAW_BUFFER1: u32 = 0x8826;
pub const GL_DRAW_BUFFER10: u32 = 0x882f;
pub const GL_DRAW_BUFFER11: u32 = 0x8830;
pub const GL_DRAW_BUFFER12: u32 = 0x8831;
pub const GL_DRAW_BUFFER13: u32 = 0x8832;
pub const GL_DRAW_BUFFER14: u32 = 0x8833;
pub const GL_DRAW_BUFFER15: u32 = 0x8834;
pub const GL_DRAW_BUFFER2: u32 = 0x8827;
pub const GL_DRAW_BUFFER3: u32 = 0x8828;
pub const GL_DRAW_BUFFER4: u32 = 0x8829;
pub const GL_DRAW_BUFFER5: u32 = 0x882a;
pub const GL_DRAW_BUFFER6: u32 = 0x882b;
pub const GL_DRAW_BUFFER7: u32 = 0x882c;
pub const GL_DRAW_BUFFER8: u32 = 0x882d;
pub const GL_DRAW_BUFFER9: u32 = 0x882e;
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8ca9;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 0x8ca6;
pub const GL_DRAW_PIXEL_TOKEN: u32 = 0x705;
pub const GL_DST_ALPHA: u32 = 0x304;
pub const GL_DST_COLOR: u32 = 0x306;
pub const GL_DYNAMIC_COPY: u32 = 0x88ea;
pub const GL_DYNAMIC_DRAW: u32 = 0x88e8;
pub const GL_DYNAMIC_READ: u32 = 0x88e9;
pub const GL_EDGE_FLAG: u32 = 0xb43;
pub const GL_EDGE_FLAG_ARRAY: u32 = 0x8079;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: u32 = 0x889b;
pub const GL_EDGE_FLAG_ARRAY_POINTER: u32 = 0x8093;
pub const GL_EDGE_FLAG_ARRAY_STRIDE: u32 = 0x808c;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;
pub const GL_EMISSION: u32 = 0x1600;
pub const GL_ENABLE_BIT: u32 = 0x2000;
pub const GL_EQUAL: u32 = 0x202;
pub const GL_EQUIV: u32 = 0x1509;
pub const GL_EVAL_BIT: u32 = 0x10000;
pub const GL_EXP: u32 = 0x800;
pub const GL_EXP2: u32 = 0x801;
pub const GL_EXTENSIONS: u32 = 0x1f03;
pub const GL_EYE_LINEAR: u32 = 0x2400;
pub const GL_EYE_PLANE: u32 = 0x2502;
pub const GL_FALSE: u32 = 0x0;
pub const GL_FASTEST: u32 = 0x1101;
pub const GL_FEEDBACK: u32 = 0x1c01;
pub const GL_FEEDBACK_BUFFER_POINTER: u32 = 0xdf0;
pub const GL_FEEDBACK_BUFFER_SIZE: u32 = 0xdf1;
pub const GL_FEEDBACK_BUFFER_TYPE: u32 = 0xdf2;
pub const GL_FILL: u32 = 0x1b02;
pub const GL_FIRST_VERTEX_CONVENTION: u32 = 0x8e4d;
pub const GL_FIXED_ONLY: u32 = 0x891d;
pub const GL_FLAT: u32 = 0x1d00;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8dad;
pub const GL_FLOAT_MAT2: u32 = 0x8b5a;
pub const GL_FLOAT_MAT2x3: u32 = 0x8b65;
pub const GL_FLOAT_MAT2x4: u32 = 0x8b66;
pub const GL_FLOAT_MAT3: u32 = 0x8b5b;
pub const GL_FLOAT_MAT3x2: u32 = 0x8b67;
pub const GL_FLOAT_MAT3x4: u32 = 0x8b68;
pub const GL_FLOAT_MAT4: u32 = 0x8b5c;
pub const GL_FLOAT_MAT4x2: u32 = 0x8b69;
pub const GL_FLOAT_MAT4x3: u32 = 0x8b6a;
pub const GL_FLOAT_VEC2: u32 = 0x8b50;
pub const GL_FLOAT_VEC3: u32 = 0x8b51;
pub const GL_FLOAT_VEC4: u32 = 0x8b52;
pub const GL_FOG: u32 = 0xb60;
pub const GL_FOG_BIT: u32 = 0x80;
pub const GL_FOG_COLOR: u32 = 0xb66;
pub const GL_FOG_COORDINATE: u32 = 0x8451;
pub const GL_FOG_COORDINATE_ARRAY: u32 = 0x8457;
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: u32 = 0x889d;
pub const GL_FOG_COORDINATE_ARRAY_POINTER: u32 = 0x8456;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE: u32 = 0x8455;
pub const GL_FOG_COORDINATE_ARRAY_TYPE: u32 = 0x8454;
pub const GL_FOG_COORDINATE_SOURCE: u32 = 0x8450;
pub const GL_FOG_DENSITY: u32 = 0xb62;
pub const GL_FOG_END: u32 = 0xb64;
pub const GL_FOG_HINT: u32 = 0xc54;
pub const GL_FOG_INDEX: u32 = 0xb61;
pub const GL_FOG_MODE: u32 = 0xb65;
pub const GL_FOG_START: u32 = 0xb63;
pub const GL_FRAGMENT_DEPTH: u32 = 0x8452;
pub const GL_FRAGMENT_SHADER: u32 = 0x8b30;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8b8b;
pub const GL_FRAMEBUFFER: u32 = 0x8d40;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: u32 = 0x8da7;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8cd1;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8cd0;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8cd3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8cd4;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8cd2;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8ca6;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8cd5;
pub const GL_FRAMEBUFFER_DEFAULT: u32 = 0x8218;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8cd6;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: u32 = 0x8cdb;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: u32 = 0x8da8;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8cd7;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8d56;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: u32 = 0x8cdc;
pub const GL_FRAMEBUFFER_SRGB: u32 = 0x8db9;
pub const GL_FRAMEBUFFER_UNDEFINED: u32 = 0x8219;
pub const GL_FRAMEBUFFER_UNSUPPORTED: u32 = 0x8cdd;
pub const GL_FRONT: u32 = 0x404;
pub const GL_FRONT_AND_BACK: u32 = 0x408;
pub const GL_FRONT_FACE: u32 = 0xb46;
pub const GL_FRONT_LEFT: u32 = 0x400;
pub const GL_FRONT_RIGHT: u32 = 0x401;
pub const GL_FUNC_ADD: u32 = 0x8006;
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800b;
pub const GL_FUNC_SUBTRACT: u32 = 0x800a;
pub const GL_GENERATE_MIPMAP: u32 = 0x8191;
pub const GL_GENERATE_MIPMAP_HINT: u32 = 0x8192;
pub const GL_GEOMETRY_INPUT_TYPE: u32 = 0x8917;
pub const GL_GEOMETRY_OUTPUT_TYPE: u32 = 0x8918;
pub const GL_GEOMETRY_SHADER: u32 = 0x8dd9;
pub const GL_GEOMETRY_VERTICES_OUT: u32 = 0x8916;
pub const GL_GEQUAL: u32 = 0x206;
pub const GL_GREATER: u32 = 0x204;
pub const GL_GREEN: u32 = 0x1904;
pub const GL_GREEN_BIAS: u32 = 0xd19;
pub const GL_GREEN_BITS: u32 = 0xd53;
pub const GL_GREEN_INTEGER: u32 = 0x8d95;
pub const GL_GREEN_SCALE: u32 = 0xd18;
pub const GL_HALF_FLOAT: u32 = 0x140b;
pub const GL_HINT_BIT: u32 = 0x8000;
pub const GL_INCR: u32 = 0x1e02;
pub const GL_INCR_WRAP: u32 = 0x8507;
pub const GL_INDEX: u32 = 0x8222;
pub const GL_INDEX_ARRAY: u32 = 0x8077;
pub const GL_INDEX_ARRAY_BUFFER_BINDING: u32 = 0x8899;
pub const GL_INDEX_ARRAY_POINTER: u32 = 0x8091;
pub const GL_INDEX_ARRAY_STRIDE: u32 = 0x8086;
pub const GL_INDEX_ARRAY_TYPE: u32 = 0x8085;
pub const GL_INDEX_BITS: u32 = 0xd51;
pub const GL_INDEX_CLEAR_VALUE: u32 = 0xc20;
pub const GL_INDEX_LOGIC_OP: u32 = 0xbf1;
pub const GL_INDEX_MODE: u32 = 0xc30;
pub const GL_INDEX_OFFSET: u32 = 0xd13;
pub const GL_INDEX_SHIFT: u32 = 0xd12;
pub const GL_INDEX_WRITEMASK: u32 = 0xc21;
pub const GL_INFO_LOG_LENGTH: u32 = 0x8b84;
pub const GL_INT: u32 = 0x1404;
pub const GL_INTENSITY: u32 = 0x8049;
pub const GL_INTENSITY12: u32 = 0x804c;
pub const GL_INTENSITY16: u32 = 0x804d;
pub const GL_INTENSITY4: u32 = 0x804a;
pub const GL_INTENSITY8: u32 = 0x804b;
pub const GL_INTERLEAVED_ATTRIBS: u32 = 0x8c8c;
pub const GL_INTERPOLATE: u32 = 0x8575;
pub const GL_INT_SAMPLER_1D: u32 = 0x8dc9;
pub const GL_INT_SAMPLER_1D_ARRAY: u32 = 0x8dce;
pub const GL_INT_SAMPLER_2D: u32 = 0x8dca;
pub const GL_INT_SAMPLER_2D_ARRAY: u32 = 0x8dcf;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x9109;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910c;
pub const GL_INT_SAMPLER_2D_RECT: u32 = 0x8dcd;
pub const GL_INT_SAMPLER_3D: u32 = 0x8dcb;
pub const GL_INT_SAMPLER_BUFFER: u32 = 0x8dd0;
pub const GL_INT_SAMPLER_CUBE: u32 = 0x8dcc;
pub const GL_INT_VEC2: u32 = 0x8b53;
pub const GL_INT_VEC3: u32 = 0x8b54;
pub const GL_INT_VEC4: u32 = 0x8b55;
pub const GL_INVALID_ENUM: u32 = 0x500;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: u32 = 0x506;
pub const GL_INVALID_INDEX: u32 = 0xffffffff;
pub const GL_INVALID_OPERATION: u32 = 0x502;
pub const GL_INVALID_VALUE: u32 = 0x501;
pub const GL_INVERT: u32 = 0x150a;
pub const GL_KEEP: u32 = 0x1e00;
pub const GL_LAST_VERTEX_CONVENTION: u32 = 0x8e4e;
pub const GL_LEFT: u32 = 0x406;
pub const GL_LEQUAL: u32 = 0x203;
pub const GL_LESS: u32 = 0x201;
pub const GL_LIGHT0: u32 = 0x4000;
pub const GL_LIGHT1: u32 = 0x4001;
pub const GL_LIGHT2: u32 = 0x4002;
pub const GL_LIGHT3: u32 = 0x4003;
pub const GL_LIGHT4: u32 = 0x4004;
pub const GL_LIGHT5: u32 = 0x4005;
pub const GL_LIGHT6: u32 = 0x4006;
pub const GL_LIGHT7: u32 = 0x4007;
pub const GL_LIGHTING: u32 = 0xb50;
pub const GL_LIGHTING_BIT: u32 = 0x40;
pub const GL_LIGHT_MODEL_AMBIENT: u32 = 0xb53;
pub const GL_LIGHT_MODEL_COLOR_CONTROL: u32 = 0x81f8;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: u32 = 0xb51;
pub const GL_LIGHT_MODEL_TWO_SIDE: u32 = 0xb52;
pub const GL_LINE: u32 = 0x1b01;
pub const GL_LINEAR: u32 = 0x2601;
pub const GL_LINEAR_ATTENUATION: u32 = 0x1208;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const GL_LINES: u32 = 0x1;
pub const GL_LINES_ADJACENCY: u32 = 0xa;
pub const GL_LINE_BIT: u32 = 0x4;
pub const GL_LINE_LOOP: u32 = 0x2;
pub const GL_LINE_RESET_TOKEN: u32 = 0x707;
pub const GL_LINE_SMOOTH: u32 = 0xb20;
pub const GL_LINE_SMOOTH_HINT: u32 = 0xc52;
pub const GL_LINE_STIPPLE: u32 = 0xb24;
pub const GL_LINE_STIPPLE_PATTERN: u32 = 0xb25;
pub const GL_LINE_STIPPLE_REPEAT: u32 = 0xb26;
pub const GL_LINE_STRIP: u32 = 0x3;
pub const GL_LINE_STRIP_ADJACENCY: u32 = 0xb;
pub const GL_LINE_TOKEN: u32 = 0x702;
pub const GL_LINE_WIDTH: u32 = 0xb21;
pub const GL_LINE_WIDTH_GRANULARITY: u32 = 0xb23;
pub const GL_LINE_WIDTH_RANGE: u32 = 0xb22;
pub const GL_LINK_STATUS: u32 = 0x8b82;
pub const GL_LIST_BASE: u32 = 0xb32;
pub const GL_LIST_BIT: u32 = 0x20000;
pub const GL_LIST_INDEX: u32 = 0xb33;
pub const GL_LIST_MODE: u32 = 0xb30;
pub const GL_LOAD: u32 = 0x101;
pub const GL_LOGIC_OP: u32 = 0xbf1;
pub const GL_LOGIC_OP_MODE: u32 = 0xbf0;
pub const GL_LOWER_LEFT: u32 = 0x8ca1;
pub const GL_LUMINANCE: u32 = 0x1909;
pub const GL_LUMINANCE12: u32 = 0x8041;
pub const GL_LUMINANCE12_ALPHA12: u32 = 0x8047;
pub const GL_LUMINANCE12_ALPHA4: u32 = 0x8046;
pub const GL_LUMINANCE16: u32 = 0x8042;
pub const GL_LUMINANCE16_ALPHA16: u32 = 0x8048;
pub const GL_LUMINANCE4: u32 = 0x803f;
pub const GL_LUMINANCE4_ALPHA4: u32 = 0x8043;
pub const GL_LUMINANCE6_ALPHA2: u32 = 0x8044;
pub const GL_LUMINANCE8: u32 = 0x8040;
pub const GL_LUMINANCE8_ALPHA8: u32 = 0x8045;
pub const GL_LUMINANCE_ALPHA: u32 = 0x190a;
pub const GL_MAJOR_VERSION: u32 = 0x821b;
pub const GL_MAP1_COLOR_4: u32 = 0xd90;
pub const GL_MAP1_GRID_DOMAIN: u32 = 0xdd0;
pub const GL_MAP1_GRID_SEGMENTS: u32 = 0xdd1;
pub const GL_MAP1_INDEX: u32 = 0xd91;
pub const GL_MAP1_NORMAL: u32 = 0xd92;
pub const GL_MAP1_TEXTURE_COORD_1: u32 = 0xd93;
pub const GL_MAP1_TEXTURE_COORD_2: u32 = 0xd94;
pub const GL_MAP1_TEXTURE_COORD_3: u32 = 0xd95;
pub const GL_MAP1_TEXTURE_COORD_4: u32 = 0xd96;
pub const GL_MAP1_VERTEX_3: u32 = 0xd97;
pub const GL_MAP1_VERTEX_4: u32 = 0xd98;
pub const GL_MAP2_COLOR_4: u32 = 0xdb0;
pub const GL_MAP2_GRID_DOMAIN: u32 = 0xdd2;
pub const GL_MAP2_GRID_SEGMENTS: u32 = 0xdd3;
pub const GL_MAP2_INDEX: u32 = 0xdb1;
pub const GL_MAP2_NORMAL: u32 = 0xdb2;
pub const GL_MAP2_TEXTURE_COORD_1: u32 = 0xdb3;
pub const GL_MAP2_TEXTURE_COORD_2: u32 = 0xdb4;
pub const GL_MAP2_TEXTURE_COORD_3: u32 = 0xdb5;
pub const GL_MAP2_TEXTURE_COORD_4: u32 = 0xdb6;
pub const GL_MAP2_VERTEX_3: u32 = 0xdb7;
pub const GL_MAP2_VERTEX_4: u32 = 0xdb8;
pub const GL_MAP_COLOR: u32 = 0xd10;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: u32 = 0x10;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: u32 = 0x8;
pub const GL_MAP_INVALIDATE_RANGE_BIT: u32 = 0x4;
pub const GL_MAP_READ_BIT: u32 = 0x1;
pub const GL_MAP_STENCIL: u32 = 0xd11;
pub const GL_MAP_UNSYNCHRONIZED_BIT: u32 = 0x20;
pub const GL_MAP_WRITE_BIT: u32 = 0x2;
pub const GL_MATRIX_MODE: u32 = 0xba0;
pub const GL_MAX: u32 = 0x8008;
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88ff;
pub const GL_MAX_ATTRIB_STACK_DEPTH: u32 = 0xd35;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: u32 = 0xd3b;
pub const GL_MAX_CLIP_PLANES: u32 = 0xd32;
pub const GL_MAX_COLOR_ATTACHMENTS: u32 = 0x8cdf;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: u32 = 0x910e;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8a33;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8a32;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8b4d;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8a2e;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8a31;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851c;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB: u32 = 0x9144;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB: u32 = 0x9143;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: u32 = 0x910f;
pub const GL_MAX_DRAW_BUFFERS: u32 = 0x8824;
pub const GL_MAX_ELEMENTS_INDICES: u32 = 0x80e9;
pub const GL_MAX_ELEMENTS_VERTICES: u32 = 0x80e8;
pub const GL_MAX_EVAL_ORDER: u32 = 0xd30;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8a2d;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8b49;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: u32 = 0x9123;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: u32 = 0x9124;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: u32 = 0x8de0;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: u32 = 0x8c29;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8de1;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: u32 = 0x8a2c;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8ddf;
pub const GL_MAX_INTEGER_SAMPLES: u32 = 0x9110;
pub const GL_MAX_LIGHTS: u32 = 0xd31;
pub const GL_MAX_LIST_NESTING: u32 = 0xb31;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: u32 = 0xd36;
pub const GL_MAX_NAME_STACK_DEPTH: u32 = 0xd37;
pub const GL_MAX_PIXEL_MAP_TABLE: u32 = 0xd34;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;
pub const GL_MAX_PROJECTION_STACK_DEPTH: u32 = 0xd38;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: u32 = 0x84f8;
pub const GL_MAX_RENDERBUFFER_SIZE: u32 = 0x84e8;
pub const GL_MAX_SAMPLES: u32 = 0x8d57;
pub const GL_MAX_SAMPLE_MASK_WORDS: u32 = 0x8e59;
pub const GL_MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: u32 = 0x8c2b;
pub const GL_MAX_TEXTURE_COORDS: u32 = 0x8871;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
pub const GL_MAX_TEXTURE_LOD_BIAS: u32 = 0x84fd;
pub const GL_MAX_TEXTURE_SIZE: u32 = 0xd33;
pub const GL_MAX_TEXTURE_STACK_DEPTH: u32 = 0xd39;
pub const GL_MAX_TEXTURE_UNITS: u32 = 0x84e2;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8c8a;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8c8b;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8c80;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8a30;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8a2f;
pub const GL_MAX_VARYING_FLOATS: u32 = 0x8b4b;
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8b4c;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8a2b;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8b4a;
pub const GL_MAX_VIEWPORT_DIMS: u32 = 0xd3a;
pub const GL_MIN: u32 = 0x8007;
pub const GL_MINOR_VERSION: u32 = 0x821c;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;
pub const GL_MIRRORED_REPEAT: u32 = 0x8370;
pub const GL_MODELVIEW: u32 = 0x1700;
pub const GL_MODELVIEW_MATRIX: u32 = 0xba6;
pub const GL_MODELVIEW_STACK_DEPTH: u32 = 0xba3;
pub const GL_MODULATE: u32 = 0x2100;
pub const GL_MULT: u32 = 0x103;
pub const GL_MULTISAMPLE: u32 = 0x809d;
pub const GL_MULTISAMPLE_BIT: u32 = 0x20000000;
pub const GL_N3F_V3F: u32 = 0x2a25;
pub const GL_NAME_STACK_DEPTH: u32 = 0xd70;
pub const GL_NAND: u32 = 0x150e;
pub const GL_NEAREST: u32 = 0x2600;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const GL_NEVER: u32 = 0x200;
pub const GL_NICEST: u32 = 0x1102;
pub const GL_NONE: u32 = 0x0;
pub const GL_NOOP: u32 = 0x1505;
pub const GL_NOR: u32 = 0x1508;
pub const GL_NORMALIZE: u32 = 0xba1;
pub const GL_NORMAL_ARRAY: u32 = 0x8075;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING: u32 = 0x8897;
pub const GL_NORMAL_ARRAY_POINTER: u32 = 0x808f;
pub const GL_NORMAL_ARRAY_STRIDE: u32 = 0x807f;
pub const GL_NORMAL_ARRAY_TYPE: u32 = 0x807e;
pub const GL_NORMAL_MAP: u32 = 0x8511;
pub const GL_NOTEQUAL: u32 = 0x205;
pub const GL_NO_ERROR: u32 = 0x0;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86a2;
pub const GL_NUM_EXTENSIONS: u32 = 0x821d;
pub const GL_OBJECT_LINEAR: u32 = 0x2401;
pub const GL_OBJECT_PLANE: u32 = 0x2501;
pub const GL_OBJECT_TYPE: u32 = 0x9112;
pub const GL_ONE: u32 = 0x1;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x305;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x307;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x303;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x301;
pub const GL_OPERAND0_ALPHA: u32 = 0x8598;
pub const GL_OPERAND0_RGB: u32 = 0x8590;
pub const GL_OPERAND1_ALPHA: u32 = 0x8599;
pub const GL_OPERAND1_RGB: u32 = 0x8591;
pub const GL_OPERAND2_ALPHA: u32 = 0x859a;
pub const GL_OPERAND2_RGB: u32 = 0x8592;
pub const GL_OR: u32 = 0x1507;
pub const GL_ORDER: u32 = 0xa01;
pub const GL_OR_INVERTED: u32 = 0x150d;
pub const GL_OR_REVERSE: u32 = 0x150b;
pub const GL_OUT_OF_MEMORY: u32 = 0x505;
pub const GL_PACK_ALIGNMENT: u32 = 0xd05;
pub const GL_PACK_IMAGE_HEIGHT: u32 = 0x806c;
pub const GL_PACK_LSB_FIRST: u32 = 0xd01;
pub const GL_PACK_ROW_LENGTH: u32 = 0xd02;
pub const GL_PACK_SKIP_IMAGES: u32 = 0x806b;
pub const GL_PACK_SKIP_PIXELS: u32 = 0xd04;
pub const GL_PACK_SKIP_ROWS: u32 = 0xd03;
pub const GL_PACK_SWAP_BYTES: u32 = 0xd00;
pub const GL_PASS_THROUGH_TOKEN: u32 = 0x700;
pub const GL_PERSPECTIVE_CORRECTION_HINT: u32 = 0xc50;
pub const GL_PIXEL_MAP_A_TO_A: u32 = 0xc79;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: u32 = 0xcb9;
pub const GL_PIXEL_MAP_B_TO_B: u32 = 0xc78;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: u32 = 0xcb8;
pub const GL_PIXEL_MAP_G_TO_G: u32 = 0xc77;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: u32 = 0xcb7;
pub const GL_PIXEL_MAP_I_TO_A: u32 = 0xc75;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: u32 = 0xcb5;
pub const GL_PIXEL_MAP_I_TO_B: u32 = 0xc74;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: u32 = 0xcb4;
pub const GL_PIXEL_MAP_I_TO_G: u32 = 0xc73;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: u32 = 0xcb3;
pub const GL_PIXEL_MAP_I_TO_I: u32 = 0xc70;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: u32 = 0xcb0;
pub const GL_PIXEL_MAP_I_TO_R: u32 = 0xc72;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: u32 = 0xcb2;
pub const GL_PIXEL_MAP_R_TO_R: u32 = 0xc76;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: u32 = 0xcb6;
pub const GL_PIXEL_MAP_S_TO_S: u32 = 0xc71;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: u32 = 0xcb1;
pub const GL_PIXEL_MODE_BIT: u32 = 0x20;
pub const GL_PIXEL_PACK_BUFFER: u32 = 0x88eb;
pub const GL_PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ed;
pub const GL_PIXEL_UNPACK_BUFFER: u32 = 0x88ec;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88ef;
pub const GL_POINT: u32 = 0x1b00;
pub const GL_POINTS: u32 = 0x0;
pub const GL_POINT_BIT: u32 = 0x2;
pub const GL_POINT_DISTANCE_ATTENUATION: u32 = 0x8129;
pub const GL_POINT_FADE_THRESHOLD_SIZE: u32 = 0x8128;
pub const GL_POINT_SIZE: u32 = 0xb11;
pub const GL_POINT_SIZE_GRANULARITY: u32 = 0xb13;
pub const GL_POINT_SIZE_MAX: u32 = 0x8127;
pub const GL_POINT_SIZE_MIN: u32 = 0x8126;
pub const GL_POINT_SIZE_RANGE: u32 = 0xb12;
pub const GL_POINT_SMOOTH: u32 = 0xb10;
pub const GL_POINT_SMOOTH_HINT: u32 = 0xc51;
pub const GL_POINT_SPRITE: u32 = 0x8861;
pub const GL_POINT_SPRITE_COORD_ORIGIN: u32 = 0x8ca0;
pub const GL_POINT_TOKEN: u32 = 0x701;
pub const GL_POLYGON: u32 = 0x9;
pub const GL_POLYGON_BIT: u32 = 0x8;
pub const GL_POLYGON_MODE: u32 = 0xb40;
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 0x8038;
pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const GL_POLYGON_OFFSET_LINE: u32 = 0x2a02;
pub const GL_POLYGON_OFFSET_POINT: u32 = 0x2a01;
pub const GL_POLYGON_OFFSET_UNITS: u32 = 0x2a00;
pub const GL_POLYGON_SMOOTH: u32 = 0xb41;
pub const GL_POLYGON_SMOOTH_HINT: u32 = 0xc53;
pub const GL_POLYGON_STIPPLE: u32 = 0xb42;
pub const GL_POLYGON_STIPPLE_BIT: u32 = 0x10;
pub const GL_POLYGON_TOKEN: u32 = 0x703;
pub const GL_POSITION: u32 = 0x1203;
pub const GL_PREVIOUS: u32 = 0x8578;
pub const GL_PRIMARY_COLOR: u32 = 0x8577;
pub const GL_PRIMITIVES_GENERATED: u32 = 0x8c87;
pub const GL_PRIMITIVE_RESTART: u32 = 0x8f9d;
pub const GL_PRIMITIVE_RESTART_INDEX: u32 = 0x8f9e;
pub const GL_PROJECTION: u32 = 0x1701;
pub const GL_PROJECTION_MATRIX: u32 = 0xba7;
pub const GL_PROJECTION_STACK_DEPTH: u32 = 0xba4;
pub const GL_PROVOKING_VERTEX: u32 = 0x8e4f;
pub const GL_PROXY_TEXTURE_1D: u32 = 0x8063;
pub const GL_PROXY_TEXTURE_1D_ARRAY: u32 = 0x8c19;
pub const GL_PROXY_TEXTURE_2D: u32 = 0x8064;
pub const GL_PROXY_TEXTURE_2D_ARRAY: u32 = 0x8c1b;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: u32 = 0x9101;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9103;
pub const GL_PROXY_TEXTURE_3D: u32 = 0x8070;
pub const GL_PROXY_TEXTURE_CUBE_MAP: u32 = 0x851b;
pub const GL_PROXY_TEXTURE_RECTANGLE: u32 = 0x84f7;
pub const GL_Q: u32 = 0x2003;
pub const GL_QUADRATIC_ATTENUATION: u32 = 0x1209;
pub const GL_QUADS: u32 = 0x7;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: u32 = 0x8e4c;
pub const GL_QUAD_STRIP: u32 = 0x8;
pub const GL_QUERY_BY_REGION_NO_WAIT: u32 = 0x8e16;
pub const GL_QUERY_BY_REGION_WAIT: u32 = 0x8e15;
pub const GL_QUERY_COUNTER_BITS: u32 = 0x8864;
pub const GL_QUERY_NO_WAIT: u32 = 0x8e14;
pub const GL_QUERY_RESULT: u32 = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 0x8867;
pub const GL_QUERY_WAIT: u32 = 0x8e13;
pub const GL_R: u32 = 0x2002;
pub const GL_R11F_G11F_B10F: u32 = 0x8c3a;
pub const GL_R16: u32 = 0x822a;
pub const GL_R16F: u32 = 0x822d;
pub const GL_R16I: u32 = 0x8233;
pub const GL_R16UI: u32 = 0x8234;
pub const GL_R16_SNORM: u32 = 0x8f98;
pub const GL_R32F: u32 = 0x822e;
pub const GL_R32I: u32 = 0x8235;
pub const GL_R32UI: u32 = 0x8236;
pub const GL_R3_G3_B2: u32 = 0x2a10;
pub const GL_R8: u32 = 0x8229;
pub const GL_R8I: u32 = 0x8231;
pub const GL_R8UI: u32 = 0x8232;
pub const GL_R8_SNORM: u32 = 0x8f94;
pub const GL_RASTERIZER_DISCARD: u32 = 0x8c89;
pub const GL_READ_BUFFER: u32 = 0xc02;
pub const GL_READ_FRAMEBUFFER: u32 = 0x8ca8;
pub const GL_READ_FRAMEBUFFER_BINDING: u32 = 0x8caa;
pub const GL_READ_ONLY: u32 = 0x88b8;
pub const GL_READ_WRITE: u32 = 0x88ba;
pub const GL_RED: u32 = 0x1903;
pub const GL_RED_BIAS: u32 = 0xd15;
pub const GL_RED_BITS: u32 = 0xd52;
pub const GL_RED_INTEGER: u32 = 0x8d94;
pub const GL_RED_SCALE: u32 = 0xd14;
pub const GL_REFLECTION_MAP: u32 = 0x8512;
pub const GL_RENDER: u32 = 0x1c00;
pub const GL_RENDERBUFFER: u32 = 0x8d41;
pub const GL_RENDERBUFFER_ALPHA_SIZE: u32 = 0x8d53;
pub const GL_RENDERBUFFER_BINDING: u32 = 0x8ca7;
pub const GL_RENDERBUFFER_BLUE_SIZE: u32 = 0x8d52;
pub const GL_RENDERBUFFER_DEPTH_SIZE: u32 = 0x8d54;
pub const GL_RENDERBUFFER_GREEN_SIZE: u32 = 0x8d51;
pub const GL_RENDERBUFFER_HEIGHT: u32 = 0x8d43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8d44;
pub const GL_RENDERBUFFER_RED_SIZE: u32 = 0x8d50;
pub const GL_RENDERBUFFER_SAMPLES: u32 = 0x8cab;
pub const GL_RENDERBUFFER_STENCIL_SIZE: u32 = 0x8d55;
pub const GL_RENDERBUFFER_WIDTH: u32 = 0x8d42;
pub const GL_RENDERER: u32 = 0x1f01;
pub const GL_RENDER_MODE: u32 = 0xc40;
pub const GL_REPEAT: u32 = 0x2901;
pub const GL_REPLACE: u32 = 0x1e01;
pub const GL_RESCALE_NORMAL: u32 = 0x803a;
pub const GL_RETURN: u32 = 0x102;
pub const GL_RG: u32 = 0x8227;
pub const GL_RG16: u32 = 0x822c;
pub const GL_RG16F: u32 = 0x822f;
pub const GL_RG16I: u32 = 0x8239;
pub const GL_RG16UI: u32 = 0x823a;
pub const GL_RG16_SNORM: u32 = 0x8f99;
pub const GL_RG32F: u32 = 0x8230;
pub const GL_RG32I: u32 = 0x823b;
pub const GL_RG32UI: u32 = 0x823c;
pub const GL_RG8: u32 = 0x822b;
pub const GL_RG8I: u32 = 0x8237;
pub const GL_RG8UI: u32 = 0x8238;
pub const GL_RG8_SNORM: u32 = 0x8f95;
pub const GL_RGB: u32 = 0x1907;
pub const GL_RGB10: u32 = 0x8052;
pub const GL_RGB10_A2: u32 = 0x8059;
pub const GL_RGB12: u32 = 0x8053;
pub const GL_RGB16: u32 = 0x8054;
pub const GL_RGB16F: u32 = 0x881b;
pub const GL_RGB16I: u32 = 0x8d89;
pub const GL_RGB16UI: u32 = 0x8d77;
pub const GL_RGB16_SNORM: u32 = 0x8f9a;
pub const GL_RGB32F: u32 = 0x8815;
pub const GL_RGB32I: u32 = 0x8d83;
pub const GL_RGB32UI: u32 = 0x8d71;
pub const GL_RGB4: u32 = 0x804f;
pub const GL_RGB5: u32 = 0x8050;
pub const GL_RGB5_A1: u32 = 0x8057;
pub const GL_RGB8: u32 = 0x8051;
pub const GL_RGB8I: u32 = 0x8d8f;
pub const GL_RGB8UI: u32 = 0x8d7d;
pub const GL_RGB8_SNORM: u32 = 0x8f96;
pub const GL_RGB9_E5: u32 = 0x8c3d;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_RGBA12: u32 = 0x805a;
pub const GL_RGBA16: u32 = 0x805b;
pub const GL_RGBA16F: u32 = 0x881a;
pub const GL_RGBA16I: u32 = 0x8d88;
pub const GL_RGBA16UI: u32 = 0x8d76;
pub const GL_RGBA16_SNORM: u32 = 0x8f9b;
pub const GL_RGBA2: u32 = 0x8055;
pub const GL_RGBA32F: u32 = 0x8814;
pub const GL_RGBA32I: u32 = 0x8d82;
pub const GL_RGBA32UI: u32 = 0x8d70;
pub const GL_RGBA4: u32 = 0x8056;
pub const GL_RGBA8: u32 = 0x8058;
pub const GL_RGBA8I: u32 = 0x8d8e;
pub const GL_RGBA8UI: u32 = 0x8d7c;
pub const GL_RGBA8_SNORM: u32 = 0x8f97;
pub const GL_RGBA_INTEGER: u32 = 0x8d99;
pub const GL_RGBA_MODE: u32 = 0xc31;
pub const GL_RGB_INTEGER: u32 = 0x8d98;
pub const GL_RGB_SCALE: u32 = 0x8573;
pub const GL_RG_INTEGER: u32 = 0x8228;
pub const GL_RIGHT: u32 = 0x407;
pub const GL_S: u32 = 0x2000;
pub const GL_SAMPLER_1D: u32 = 0x8b5d;
pub const GL_SAMPLER_1D_ARRAY: u32 = 0x8dc0;
pub const GL_SAMPLER_1D_ARRAY_SHADOW: u32 = 0x8dc3;
pub const GL_SAMPLER_1D_SHADOW: u32 = 0x8b61;
pub const GL_SAMPLER_2D: u32 = 0x8b5e;
pub const GL_SAMPLER_2D_ARRAY: u32 = 0x8dc1;
pub const GL_SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8dc4;
pub const GL_SAMPLER_2D_MULTISAMPLE: u32 = 0x9108;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910b;
pub const GL_SAMPLER_2D_RECT: u32 = 0x8b63;
pub const GL_SAMPLER_2D_RECT_SHADOW: u32 = 0x8b64;
pub const GL_SAMPLER_2D_SHADOW: u32 = 0x8b62;
pub const GL_SAMPLER_3D: u32 = 0x8b5f;
pub const GL_SAMPLER_BUFFER: u32 = 0x8dc2;
pub const GL_SAMPLER_CUBE: u32 = 0x8b60;
pub const GL_SAMPLER_CUBE_SHADOW: u32 = 0x8dc5;
pub const GL_SAMPLES: u32 = 0x80a9;
pub const GL_SAMPLES_PASSED: u32 = 0x8914;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809e;
pub const GL_SAMPLE_ALPHA_TO_ONE: u32 = 0x809f;
pub const GL_SAMPLE_BUFFERS: u32 = 0x80a8;
pub const GL_SAMPLE_COVERAGE: u32 = 0x80a0;
pub const GL_SAMPLE_COVERAGE_INVERT: u32 = 0x80ab;
pub const GL_SAMPLE_COVERAGE_VALUE: u32 = 0x80aa;
pub const GL_SAMPLE_MASK: u32 = 0x8e51;
pub const GL_SAMPLE_MASK_VALUE: u32 = 0x8e52;
pub const GL_SAMPLE_POSITION: u32 = 0x8e50;
pub const GL_SCISSOR_BIT: u32 = 0x80000;
pub const GL_SCISSOR_BOX: u32 = 0xc10;
pub const GL_SCISSOR_TEST: u32 = 0xc11;
pub const GL_SECONDARY_COLOR_ARRAY: u32 = 0x845e;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: u32 = 0x889c;
pub const GL_SECONDARY_COLOR_ARRAY_POINTER: u32 = 0x845d;
pub const GL_SECONDARY_COLOR_ARRAY_SIZE: u32 = 0x845a;
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: u32 = 0x845c;
pub const GL_SECONDARY_COLOR_ARRAY_TYPE: u32 = 0x845b;
pub const GL_SELECT: u32 = 0x1c02;
pub const GL_SELECTION_BUFFER_POINTER: u32 = 0xdf3;
pub const GL_SELECTION_BUFFER_SIZE: u32 = 0xdf4;
pub const GL_SEPARATE_ATTRIBS: u32 = 0x8c8d;
pub const GL_SEPARATE_SPECULAR_COLOR: u32 = 0x81fa;
pub const GL_SET: u32 = 0x150f;
pub const GL_SHADER_SOURCE_LENGTH: u32 = 0x8b88;
pub const GL_SHADER_TYPE: u32 = 0x8b4f;
pub const GL_SHADE_MODEL: u32 = 0xb54;
pub const GL_SHADING_LANGUAGE_VERSION: u32 = 0x8b8c;
pub const GL_SHININESS: u32 = 0x1601;
pub const GL_SHORT: u32 = 0x1402;
pub const GL_SIGNALED: u32 = 0x9119;
pub const GL_SIGNED_NORMALIZED: u32 = 0x8f9c;
pub const GL_SINGLE_COLOR: u32 = 0x81f9;
pub const GL_SLUMINANCE: u32 = 0x8c46;
pub const GL_SLUMINANCE8: u32 = 0x8c47;
pub const GL_SLUMINANCE8_ALPHA8: u32 = 0x8c45;
pub const GL_SLUMINANCE_ALPHA: u32 = 0x8c44;
pub const GL_SMOOTH: u32 = 0x1d01;
pub const GL_SOURCE0_ALPHA: u32 = 0x8588;
pub const GL_SOURCE0_RGB: u32 = 0x8580;
pub const GL_SOURCE1_ALPHA: u32 = 0x8589;
pub const GL_SOURCE1_RGB: u32 = 0x8581;
pub const GL_SOURCE2_ALPHA: u32 = 0x858a;
pub const GL_SOURCE2_RGB: u32 = 0x8582;
pub const GL_SPECULAR: u32 = 0x1202;
pub const GL_SPHERE_MAP: u32 = 0x2402;
pub const GL_SPOT_CUTOFF: u32 = 0x1206;
pub const GL_SPOT_DIRECTION: u32 = 0x1204;
pub const GL_SPOT_EXPONENT: u32 = 0x1205;
pub const GL_SRC_ALPHA: u32 = 0x302;
pub const GL_SRC_ALPHA_SATURATE: u32 = 0x308;
pub const GL_SRC_COLOR: u32 = 0x300;
pub const GL_SRGB: u32 = 0x8c40;
pub const GL_SRGB8: u32 = 0x8c41;
pub const GL_SRGB8_ALPHA8: u32 = 0x8c43;
pub const GL_SRGB_ALPHA: u32 = 0x8c42;
pub const GL_STACK_OVERFLOW: u32 = 0x503;
pub const GL_STACK_UNDERFLOW: u32 = 0x504;
pub const GL_STATIC_COPY: u32 = 0x88e6;
pub const GL_STATIC_DRAW: u32 = 0x88e4;
pub const GL_STATIC_READ: u32 = 0x88e5;
pub const GL_STENCIL: u32 = 0x1802;
pub const GL_STENCIL_ATTACHMENT: u32 = 0x8d20;
pub const GL_STENCIL_BACK_FAIL: u32 = 0x8801;
pub const GL_STENCIL_BACK_FUNC: u32 = 0x8800;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;
pub const GL_STENCIL_BACK_REF: u32 = 0x8ca3;
pub const GL_STENCIL_BACK_VALUE_MASK: u32 = 0x8ca4;
pub const GL_STENCIL_BACK_WRITEMASK: u32 = 0x8ca5;
pub const GL_STENCIL_BITS: u32 = 0xd57;
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x400;
pub const GL_STENCIL_CLEAR_VALUE: u32 = 0xb91;
pub const GL_STENCIL_FAIL: u32 = 0xb94;
pub const GL_STENCIL_FUNC: u32 = 0xb92;
pub const GL_STENCIL_INDEX: u32 = 0x1901;
pub const GL_STENCIL_INDEX1: u32 = 0x8d46;
pub const GL_STENCIL_INDEX16: u32 = 0x8d49;
pub const GL_STENCIL_INDEX4: u32 = 0x8d47;
pub const GL_STENCIL_INDEX8: u32 = 0x8d48;
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 0xb95;
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 0xb96;
pub const GL_STENCIL_REF: u32 = 0xb97;
pub const GL_STENCIL_TEST: u32 = 0xb90;
pub const GL_STENCIL_VALUE_MASK: u32 = 0xb93;
pub const GL_STENCIL_WRITEMASK: u32 = 0xb98;
pub const GL_STEREO: u32 = 0xc33;
pub const GL_STREAM_COPY: u32 = 0x88e2;
pub const GL_STREAM_DRAW: u32 = 0x88e0;
pub const GL_STREAM_READ: u32 = 0x88e1;
pub const GL_SUBPIXEL_BITS: u32 = 0xd50;
pub const GL_SUBTRACT: u32 = 0x84e7;
pub const GL_SYNC_CONDITION: u32 = 0x9113;
pub const GL_SYNC_FENCE: u32 = 0x9116;
pub const GL_SYNC_FLAGS: u32 = 0x9115;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: u32 = 0x1;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;
pub const GL_SYNC_STATUS: u32 = 0x9114;
pub const GL_T: u32 = 0x2001;
pub const GL_T2F_C3F_V3F: u32 = 0x2a2a;
pub const GL_T2F_C4F_N3F_V3F: u32 = 0x2a2c;
pub const GL_T2F_C4UB_V3F: u32 = 0x2a29;
pub const GL_T2F_N3F_V3F: u32 = 0x2a2b;
pub const GL_T2F_V3F: u32 = 0x2a27;
pub const GL_T4F_C4F_N3F_V4F: u32 = 0x2a2d;
pub const GL_T4F_V4F: u32 = 0x2a28;
pub const GL_TEXTURE: u32 = 0x1702;
pub const GL_TEXTURE0: u32 = 0x84c0;
pub const GL_TEXTURE1: u32 = 0x84c1;
pub const GL_TEXTURE10: u32 = 0x84ca;
pub const GL_TEXTURE11: u32 = 0x84cb;
pub const GL_TEXTURE12: u32 = 0x84cc;
pub const GL_TEXTURE13: u32 = 0x84cd;
pub const GL_TEXTURE14: u32 = 0x84ce;
pub const GL_TEXTURE15: u32 = 0x84cf;
pub const GL_TEXTURE16: u32 = 0x84d0;
pub const GL_TEXTURE17: u32 = 0x84d1;
pub const GL_TEXTURE18: u32 = 0x84d2;
pub const GL_TEXTURE19: u32 = 0x84d3;
pub const GL_TEXTURE2: u32 = 0x84c2;
pub const GL_TEXTURE20: u32 = 0x84d4;
pub const GL_TEXTURE21: u32 = 0x84d5;
pub const GL_TEXTURE22: u32 = 0x84d6;
pub const GL_TEXTURE23: u32 = 0x84d7;
pub const GL_TEXTURE24: u32 = 0x84d8;
pub const GL_TEXTURE25: u32 = 0x84d9;
pub const GL_TEXTURE26: u32 = 0x84da;
pub const GL_TEXTURE27: u32 = 0x84db;
pub const GL_TEXTURE28: u32 = 0x84dc;
pub const GL_TEXTURE29: u32 = 0x84dd;
pub const GL_TEXTURE3: u32 = 0x84c3;
pub const GL_TEXTURE30: u32 = 0x84de;
pub const GL_TEXTURE31: u32 = 0x84df;
pub const GL_TEXTURE4: u32 = 0x84c4;
pub const GL_TEXTURE5: u32 = 0x84c5;
pub const GL_TEXTURE6: u32 = 0x84c6;
pub const GL_TEXTURE7: u32 = 0x84c7;
pub const GL_TEXTURE8: u32 = 0x84c8;
pub const GL_TEXTURE9: u32 = 0x84c9;
pub const GL_TEXTURE_1D: u32 = 0xde0;
pub const GL_TEXTURE_1D_ARRAY: u32 = 0x8c18;
pub const GL_TEXTURE_2D: u32 = 0xde1;
pub const GL_TEXTURE_2D_ARRAY: u32 = 0x8c1a;
pub const GL_TEXTURE_2D_MULTISAMPLE: u32 = 0x9100;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9102;
pub const GL_TEXTURE_3D: u32 = 0x806f;
pub const GL_TEXTURE_ALPHA_SIZE: u32 = 0x805f;
pub const GL_TEXTURE_ALPHA_TYPE: u32 = 0x8c13;
pub const GL_TEXTURE_BASE_LEVEL: u32 = 0x813c;
pub const GL_TEXTURE_BINDING_1D: u32 = 0x8068;
pub const GL_TEXTURE_BINDING_1D_ARRAY: u32 = 0x8c1c;
pub const GL_TEXTURE_BINDING_2D: u32 = 0x8069;
pub const GL_TEXTURE_BINDING_2D_ARRAY: u32 = 0x8c1d;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: u32 = 0x9104;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: u32 = 0x9105;
pub const GL_TEXTURE_BINDING_3D: u32 = 0x806a;
pub const GL_TEXTURE_BINDING_BUFFER: u32 = 0x8c2c;
pub const GL_TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;
pub const GL_TEXTURE_BINDING_RECTANGLE: u32 = 0x84f6;
pub const GL_TEXTURE_BIT: u32 = 0x40000;
pub const GL_TEXTURE_BLUE_SIZE: u32 = 0x805e;
pub const GL_TEXTURE_BLUE_TYPE: u32 = 0x8c12;
pub const GL_TEXTURE_BORDER: u32 = 0x1005;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
pub const GL_TEXTURE_BUFFER: u32 = 0x8c2a;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: u32 = 0x8c2d;
pub const GL_TEXTURE_COMPARE_FUNC: u32 = 0x884d;
pub const GL_TEXTURE_COMPARE_MODE: u32 = 0x884c;
pub const GL_TEXTURE_COMPONENTS: u32 = 0x1003;
pub const GL_TEXTURE_COMPRESSED: u32 = 0x86a1;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: u32 = 0x86a0;
pub const GL_TEXTURE_COMPRESSION_HINT: u32 = 0x84ef;
pub const GL_TEXTURE_COORD_ARRAY: u32 = 0x8078;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: u32 = 0x889a;
pub const GL_TEXTURE_COORD_ARRAY_POINTER: u32 = 0x8092;
pub const GL_TEXTURE_COORD_ARRAY_SIZE: u32 = 0x8088;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: u32 = 0x808a;
pub const GL_TEXTURE_COORD_ARRAY_TYPE: u32 = 0x8089;
pub const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851a;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: u32 = 0x884f;
pub const GL_TEXTURE_DEPTH: u32 = 0x8071;
pub const GL_TEXTURE_DEPTH_SIZE: u32 = 0x884a;
pub const GL_TEXTURE_DEPTH_TYPE: u32 = 0x8c16;
pub const GL_TEXTURE_ENV: u32 = 0x2300;
pub const GL_TEXTURE_ENV_COLOR: u32 = 0x2201;
pub const GL_TEXTURE_ENV_MODE: u32 = 0x2200;
pub const GL_TEXTURE_FILTER_CONTROL: u32 = 0x8500;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: u32 = 0x9107;
pub const GL_TEXTURE_GEN_MODE: u32 = 0x2500;
pub const GL_TEXTURE_GEN_Q: u32 = 0xc63;
pub const GL_TEXTURE_GEN_R: u32 = 0xc62;
pub const GL_TEXTURE_GEN_S: u32 = 0xc60;
pub const GL_TEXTURE_GEN_T: u32 = 0xc61;
pub const GL_TEXTURE_GREEN_SIZE: u32 = 0x805d;
pub const GL_TEXTURE_GREEN_TYPE: u32 = 0x8c11;
pub const GL_TEXTURE_HEIGHT: u32 = 0x1001;
pub const GL_TEXTURE_INTENSITY_SIZE: u32 = 0x8061;
pub const GL_TEXTURE_INTENSITY_TYPE: u32 = 0x8c15;
pub const GL_TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
pub const GL_TEXTURE_LOD_BIAS: u32 = 0x8501;
pub const GL_TEXTURE_LUMINANCE_SIZE: u32 = 0x8060;
pub const GL_TEXTURE_LUMINANCE_TYPE: u32 = 0x8c14;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_TEXTURE_MATRIX: u32 = 0xba8;
pub const GL_TEXTURE_MAX_LEVEL: u32 = 0x813d;
pub const GL_TEXTURE_MAX_LOD: u32 = 0x813b;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_TEXTURE_MIN_LOD: u32 = 0x813a;
pub const GL_TEXTURE_PRIORITY: u32 = 0x8066;
pub const GL_TEXTURE_RECTANGLE: u32 = 0x84f5;
pub const GL_TEXTURE_RED_SIZE: u32 = 0x805c;
pub const GL_TEXTURE_RED_TYPE: u32 = 0x8c10;
pub const GL_TEXTURE_RESIDENT: u32 = 0x8067;
pub const GL_TEXTURE_SAMPLES: u32 = 0x9106;
pub const GL_TEXTURE_SHARED_SIZE: u32 = 0x8c3f;
pub const GL_TEXTURE_STACK_DEPTH: u32 = 0xba5;
pub const GL_TEXTURE_STENCIL_SIZE: u32 = 0x88f1;
pub const GL_TEXTURE_WIDTH: u32 = 0x1000;
pub const GL_TEXTURE_WRAP_R: u32 = 0x8072;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_TIMEOUT_EXPIRED: u32 = 0x911b;
pub const GL_TIMEOUT_IGNORED: u64 = 0xffffffffffffffff;
pub const GL_TRANSFORM_BIT: u32 = 0x1000;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8c8e;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8c8f;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8c7f;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8c85;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8c84;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8c88;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8c83;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: u32 = 0x8c76;
pub const GL_TRANSPOSE_COLOR_MATRIX: u32 = 0x84e6;
pub const GL_TRANSPOSE_MODELVIEW_MATRIX: u32 = 0x84e3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX: u32 = 0x84e4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX: u32 = 0x84e5;
pub const GL_TRIANGLES: u32 = 0x4;
pub const GL_TRIANGLES_ADJACENCY: u32 = 0xc;
pub const GL_TRIANGLE_FAN: u32 = 0x6;
pub const GL_TRIANGLE_STRIP: u32 = 0x5;
pub const GL_TRIANGLE_STRIP_ADJACENCY: u32 = 0xd;
pub const GL_TRUE: u32 = 0x1;
pub const GL_UNIFORM_ARRAY_STRIDE: u32 = 0x8a3c;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8a42;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8a43;
pub const GL_UNIFORM_BLOCK_BINDING: u32 = 0x8a3f;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8a40;
pub const GL_UNIFORM_BLOCK_INDEX: u32 = 0x8a3a;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: u32 = 0x8a41;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8a46;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x8a45;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8a44;
pub const GL_UNIFORM_BUFFER: u32 = 0x8a11;
pub const GL_UNIFORM_BUFFER_BINDING: u32 = 0x8a28;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8a34;
pub const GL_UNIFORM_BUFFER_SIZE: u32 = 0x8a2a;
pub const GL_UNIFORM_BUFFER_START: u32 = 0x8a29;
pub const GL_UNIFORM_IS_ROW_MAJOR: u32 = 0x8a3e;
pub const GL_UNIFORM_MATRIX_STRIDE: u32 = 0x8a3d;
pub const GL_UNIFORM_NAME_LENGTH: u32 = 0x8a39;
pub const GL_UNIFORM_OFFSET: u32 = 0x8a3b;
pub const GL_UNIFORM_SIZE: u32 = 0x8a38;
pub const GL_UNIFORM_TYPE: u32 = 0x8a37;
pub const GL_UNPACK_ALIGNMENT: u32 = 0xcf5;
pub const GL_UNPACK_IMAGE_HEIGHT: u32 = 0x806e;
pub const GL_UNPACK_LSB_FIRST: u32 = 0xcf1;
pub const GL_UNPACK_ROW_LENGTH: u32 = 0xcf2;
pub const GL_UNPACK_SKIP_IMAGES: u32 = 0x806d;
pub const GL_UNPACK_SKIP_PIXELS: u32 = 0xcf4;
pub const GL_UNPACK_SKIP_ROWS: u32 = 0xcf3;
pub const GL_UNPACK_SWAP_BYTES: u32 = 0xcf0;
pub const GL_UNSIGNALED: u32 = 0x9118;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: u32 = 0x8362;
pub const GL_UNSIGNED_BYTE_3_3_2: u32 = 0x8032;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8c3b;
pub const GL_UNSIGNED_INT_10_10_10_2: u32 = 0x8036;
pub const GL_UNSIGNED_INT_24_8: u32 = 0x84fa;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8c3e;
pub const GL_UNSIGNED_INT_8_8_8_8: u32 = 0x8035;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: u32 = 0x8367;
pub const GL_UNSIGNED_INT_SAMPLER_1D: u32 = 0x8dd1;
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: u32 = 0x8dd6;
pub const GL_UNSIGNED_INT_SAMPLER_2D: u32 = 0x8dd2;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8dd7;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x910a;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910d;
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: u32 = 0x8dd5;
pub const GL_UNSIGNED_INT_SAMPLER_3D: u32 = 0x8dd3;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: u32 = 0x8dd8;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8dd4;
pub const GL_UNSIGNED_INT_VEC2: u32 = 0x8dc6;
pub const GL_UNSIGNED_INT_VEC3: u32 = 0x8dc7;
pub const GL_UNSIGNED_INT_VEC4: u32 = 0x8dc8;
pub const GL_UNSIGNED_NORMALIZED: u32 = 0x8c17;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: u32 = 0x8366;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: u32 = 0x8365;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: u32 = 0x8364;
pub const GL_UPPER_LEFT: u32 = 0x8ca2;
pub const GL_V2F: u32 = 0x2a20;
pub const GL_V3F: u32 = 0x2a21;
pub const GL_VALIDATE_STATUS: u32 = 0x8b83;
pub const GL_VENDOR: u32 = 0x1f00;
pub const GL_VERSION: u32 = 0x1f02;
pub const GL_VERTEX_ARRAY: u32 = 0x8074;
pub const GL_VERTEX_ARRAY_BINDING: u32 = 0x85b5;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING: u32 = 0x8896;
pub const GL_VERTEX_ARRAY_POINTER: u32 = 0x808e;
pub const GL_VERTEX_ARRAY_SIZE: u32 = 0x807a;
pub const GL_VERTEX_ARRAY_STRIDE: u32 = 0x807c;
pub const GL_VERTEX_ARRAY_TYPE: u32 = 0x807b;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889f;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88fd;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886a;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;
pub const GL_VERTEX_PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const GL_VERTEX_PROGRAM_TWO_SIDE: u32 = 0x8643;
pub const GL_VERTEX_SHADER: u32 = 0x8b31;
pub const GL_VIEWPORT: u32 = 0xba2;
pub const GL_VIEWPORT_BIT: u32 = 0x800;
pub const GL_WAIT_FAILED: u32 = 0x911d;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: u32 = 0x889e;
pub const GL_WRITE_ONLY: u32 = 0x88b9;
pub const GL_XOR: u32 = 0x1506;
pub const GL_ZERO: u32 = 0x0;
pub const GL_ZOOM_X: u32 = 0xd16;
pub const GL_ZOOM_Y: u32 = 0xd17;

// *** COMMANDS ***
pub struct Procs {
    procs: [*const (); 670],
    pub has_ARB_debug_output: bool,
}

use std::fmt;
impl fmt::Debug for Procs {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Procs{{...}}")?;
        Ok(())
    }
}
extern "C" fn glDebugMessageCallbackARB_null_imp(_: GLDEBUGPROCARB, _: *const libc::c_void) -> libc::c_void { missing_ext_panic("glDebugMessageCallbackARB", "GL_ARB_debug_output"); }
extern "C" fn glDebugMessageControlARB_null_imp(_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *const GLuint, _: GLboolean) -> libc::c_void { missing_ext_panic("glDebugMessageControlARB", "GL_ARB_debug_output"); }
extern "C" fn glDebugMessageInsertARB_null_imp(_: GLenum, _: GLenum, _: GLuint, _: GLenum, _: GLsizei, _: *const GLchar) -> libc::c_void { missing_ext_panic("glDebugMessageInsertARB", "GL_ARB_debug_output"); }
extern "C" fn glGetDebugMessageLogARB_null_imp(_: GLuint, _: GLsizei, _: *mut GLenum, _: *mut GLenum, _: *mut GLuint, _: *mut GLenum, _: *mut GLsizei, _: *mut GLchar) -> GLuint { missing_ext_panic("glGetDebugMessageLogARB", "GL_ARB_debug_output"); }

#[inline(never)] fn missing_ext_panic(name: &str, ext: &str) -> ! {
    panic!("{} called, but the requisite extension ({}) is not present",
        name, ext);
}

use std::mem::{transmute, MaybeUninit};
use std::ffi::CStr;
impl Procs {
    pub fn new<E, F: Fn(&[u8])->Result<*const(),E>>(get_proc: F)
                 -> Result<Procs, E> {
        let mut procs: [MaybeUninit<*const()>; 670] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        Procs::getprocs(&get_proc, &mut procs[0..666], &[
            b"glAccum\0",
            b"glActiveTexture\0",
            b"glAlphaFunc\0",
            b"glAreTexturesResident\0",
            b"glArrayElement\0",
            b"glAttachShader\0",
            b"glBegin\0",
            b"glBeginConditionalRender\0",
            b"glBeginQuery\0",
            b"glBeginTransformFeedback\0",
            b"glBindAttribLocation\0",
            b"glBindBuffer\0",
            b"glBindBufferBase\0",
            b"glBindBufferRange\0",
            b"glBindFragDataLocation\0",
            b"glBindFramebuffer\0",
            b"glBindRenderbuffer\0",
            b"glBindTexture\0",
            b"glBindVertexArray\0",
            b"glBitmap\0",
            b"glBlendColor\0",
            b"glBlendEquation\0",
            b"glBlendEquationSeparate\0",
            b"glBlendFunc\0",
            b"glBlendFuncSeparate\0",
            b"glBlitFramebuffer\0",
            b"glBufferData\0",
            b"glBufferSubData\0",
            b"glCallList\0",
            b"glCallLists\0",
            b"glCheckFramebufferStatus\0",
            b"glClampColor\0",
            b"glClear\0",
            b"glClearAccum\0",
            b"glClearBufferfi\0",
            b"glClearBufferfv\0",
            b"glClearBufferiv\0",
            b"glClearBufferuiv\0",
            b"glClearColor\0",
            b"glClearDepth\0",
            b"glClearIndex\0",
            b"glClearStencil\0",
            b"glClientActiveTexture\0",
            b"glClientWaitSync\0",
            b"glClipPlane\0",
            b"glColor3b\0",
            b"glColor3bv\0",
            b"glColor3d\0",
            b"glColor3dv\0",
            b"glColor3f\0",
            b"glColor3fv\0",
            b"glColor3i\0",
            b"glColor3iv\0",
            b"glColor3s\0",
            b"glColor3sv\0",
            b"glColor3ub\0",
            b"glColor3ubv\0",
            b"glColor3ui\0",
            b"glColor3uiv\0",
            b"glColor3us\0",
            b"glColor3usv\0",
            b"glColor4b\0",
            b"glColor4bv\0",
            b"glColor4d\0",
            b"glColor4dv\0",
            b"glColor4f\0",
            b"glColor4fv\0",
            b"glColor4i\0",
            b"glColor4iv\0",
            b"glColor4s\0",
            b"glColor4sv\0",
            b"glColor4ub\0",
            b"glColor4ubv\0",
            b"glColor4ui\0",
            b"glColor4uiv\0",
            b"glColor4us\0",
            b"glColor4usv\0",
            b"glColorMask\0",
            b"glColorMaski\0",
            b"glColorMaterial\0",
            b"glColorPointer\0",
            b"glCompileShader\0",
            b"glCompressedTexImage1D\0",
            b"glCompressedTexImage2D\0",
            b"glCompressedTexImage3D\0",
            b"glCompressedTexSubImage1D\0",
            b"glCompressedTexSubImage2D\0",
            b"glCompressedTexSubImage3D\0",
            b"glCopyBufferSubData\0",
            b"glCopyPixels\0",
            b"glCopyTexImage1D\0",
            b"glCopyTexImage2D\0",
            b"glCopyTexSubImage1D\0",
            b"glCopyTexSubImage2D\0",
            b"glCopyTexSubImage3D\0",
            b"glCreateProgram\0",
            b"glCreateShader\0",
            b"glCullFace\0",
            b"glDeleteBuffers\0",
            b"glDeleteFramebuffers\0",
            b"glDeleteLists\0",
            b"glDeleteProgram\0",
            b"glDeleteQueries\0",
            b"glDeleteRenderbuffers\0",
            b"glDeleteShader\0",
            b"glDeleteSync\0",
            b"glDeleteTextures\0",
            b"glDeleteVertexArrays\0",
            b"glDepthFunc\0",
            b"glDepthMask\0",
            b"glDepthRange\0",
            b"glDetachShader\0",
            b"glDisable\0",
            b"glDisableClientState\0",
            b"glDisableVertexAttribArray\0",
            b"glDisablei\0",
            b"glDrawArrays\0",
            b"glDrawArraysInstanced\0",
            b"glDrawBuffer\0",
            b"glDrawBuffers\0",
            b"glDrawElements\0",
            b"glDrawElementsBaseVertex\0",
            b"glDrawElementsInstanced\0",
            b"glDrawElementsInstancedBaseVertex\0",
            b"glDrawPixels\0",
            b"glDrawRangeElements\0",
            b"glDrawRangeElementsBaseVertex\0",
            b"glEdgeFlag\0",
            b"glEdgeFlagPointer\0",
            b"glEdgeFlagv\0",
            b"glEnable\0",
            b"glEnableClientState\0",
            b"glEnableVertexAttribArray\0",
            b"glEnablei\0",
            b"glEnd\0",
            b"glEndConditionalRender\0",
            b"glEndList\0",
            b"glEndQuery\0",
            b"glEndTransformFeedback\0",
            b"glEvalCoord1d\0",
            b"glEvalCoord1dv\0",
            b"glEvalCoord1f\0",
            b"glEvalCoord1fv\0",
            b"glEvalCoord2d\0",
            b"glEvalCoord2dv\0",
            b"glEvalCoord2f\0",
            b"glEvalCoord2fv\0",
            b"glEvalMesh1\0",
            b"glEvalMesh2\0",
            b"glEvalPoint1\0",
            b"glEvalPoint2\0",
            b"glFeedbackBuffer\0",
            b"glFenceSync\0",
            b"glFinish\0",
            b"glFlush\0",
            b"glFlushMappedBufferRange\0",
            b"glFogCoordPointer\0",
            b"glFogCoordd\0",
            b"glFogCoorddv\0",
            b"glFogCoordf\0",
            b"glFogCoordfv\0",
            b"glFogf\0",
            b"glFogfv\0",
            b"glFogi\0",
            b"glFogiv\0",
            b"glFramebufferRenderbuffer\0",
            b"glFramebufferTexture\0",
            b"glFramebufferTexture1D\0",
            b"glFramebufferTexture2D\0",
            b"glFramebufferTexture3D\0",
            b"glFramebufferTextureLayer\0",
            b"glFrontFace\0",
            b"glFrustum\0",
            b"glGenBuffers\0",
            b"glGenFramebuffers\0",
            b"glGenLists\0",
            b"glGenQueries\0",
            b"glGenRenderbuffers\0",
            b"glGenTextures\0",
            b"glGenVertexArrays\0",
            b"glGenerateMipmap\0",
            b"glGetActiveAttrib\0",
            b"glGetActiveUniform\0",
            b"glGetActiveUniformBlockName\0",
            b"glGetActiveUniformBlockiv\0",
            b"glGetActiveUniformName\0",
            b"glGetActiveUniformsiv\0",
            b"glGetAttachedShaders\0",
            b"glGetAttribLocation\0",
            b"glGetBooleani_v\0",
            b"glGetBooleanv\0",
            b"glGetBufferParameteri64v\0",
            b"glGetBufferParameteriv\0",
            b"glGetBufferPointerv\0",
            b"glGetBufferSubData\0",
            b"glGetClipPlane\0",
            b"glGetCompressedTexImage\0",
            b"glGetDoublev\0",
            b"glGetError\0",
            b"glGetFloatv\0",
            b"glGetFragDataLocation\0",
            b"glGetFramebufferAttachmentParameteriv\0",
            b"glGetInteger64i_v\0",
            b"glGetInteger64v\0",
            b"glGetIntegeri_v\0",
            b"glGetIntegerv\0",
            b"glGetLightfv\0",
            b"glGetLightiv\0",
            b"glGetMapdv\0",
            b"glGetMapfv\0",
            b"glGetMapiv\0",
            b"glGetMaterialfv\0",
            b"glGetMaterialiv\0",
            b"glGetMultisamplefv\0",
            b"glGetPixelMapfv\0",
            b"glGetPixelMapuiv\0",
            b"glGetPixelMapusv\0",
            b"glGetPointerv\0",
            b"glGetPolygonStipple\0",
            b"glGetProgramInfoLog\0",
            b"glGetProgramiv\0",
            b"glGetQueryObjectiv\0",
            b"glGetQueryObjectuiv\0",
            b"glGetQueryiv\0",
            b"glGetRenderbufferParameteriv\0",
            b"glGetShaderInfoLog\0",
            b"glGetShaderSource\0",
            b"glGetShaderiv\0",
            b"glGetString\0",
            b"glGetStringi\0",
            b"glGetSynciv\0",
            b"glGetTexEnvfv\0",
            b"glGetTexEnviv\0",
            b"glGetTexGendv\0",
            b"glGetTexGenfv\0",
            b"glGetTexGeniv\0",
            b"glGetTexImage\0",
            b"glGetTexLevelParameterfv\0",
            b"glGetTexLevelParameteriv\0",
            b"glGetTexParameterIiv\0",
            b"glGetTexParameterIuiv\0",
            b"glGetTexParameterfv\0",
            b"glGetTexParameteriv\0",
            b"glGetTransformFeedbackVarying\0",
            b"glGetUniformBlockIndex\0",
            b"glGetUniformIndices\0",
            b"glGetUniformLocation\0",
            b"glGetUniformfv\0",
            b"glGetUniformiv\0",
            b"glGetUniformuiv\0",
            b"glGetVertexAttribIiv\0",
            b"glGetVertexAttribIuiv\0",
            b"glGetVertexAttribPointerv\0",
            b"glGetVertexAttribdv\0",
            b"glGetVertexAttribfv\0",
            b"glGetVertexAttribiv\0",
            b"glHint\0",
            b"glIndexMask\0",
            b"glIndexPointer\0",
            b"glIndexd\0",
            b"glIndexdv\0",
            b"glIndexf\0",
            b"glIndexfv\0",
            b"glIndexi\0",
            b"glIndexiv\0",
            b"glIndexs\0",
            b"glIndexsv\0",
            b"glIndexub\0",
            b"glIndexubv\0",
            b"glInitNames\0",
            b"glInterleavedArrays\0",
            b"glIsBuffer\0",
            b"glIsEnabled\0",
            b"glIsEnabledi\0",
            b"glIsFramebuffer\0",
            b"glIsList\0",
            b"glIsProgram\0",
            b"glIsQuery\0",
            b"glIsRenderbuffer\0",
            b"glIsShader\0",
            b"glIsSync\0",
            b"glIsTexture\0",
            b"glIsVertexArray\0",
            b"glLightModelf\0",
            b"glLightModelfv\0",
            b"glLightModeli\0",
            b"glLightModeliv\0",
            b"glLightf\0",
            b"glLightfv\0",
            b"glLighti\0",
            b"glLightiv\0",
            b"glLineStipple\0",
            b"glLineWidth\0",
            b"glLinkProgram\0",
            b"glListBase\0",
            b"glLoadIdentity\0",
            b"glLoadMatrixd\0",
            b"glLoadMatrixf\0",
            b"glLoadName\0",
            b"glLoadTransposeMatrixd\0",
            b"glLoadTransposeMatrixf\0",
            b"glLogicOp\0",
            b"glMap1d\0",
            b"glMap1f\0",
            b"glMap2d\0",
            b"glMap2f\0",
            b"glMapBuffer\0",
            b"glMapBufferRange\0",
            b"glMapGrid1d\0",
            b"glMapGrid1f\0",
            b"glMapGrid2d\0",
            b"glMapGrid2f\0",
            b"glMaterialf\0",
            b"glMaterialfv\0",
            b"glMateriali\0",
            b"glMaterialiv\0",
            b"glMatrixMode\0",
            b"glMultMatrixd\0",
            b"glMultMatrixf\0",
            b"glMultTransposeMatrixd\0",
            b"glMultTransposeMatrixf\0",
            b"glMultiDrawArrays\0",
            b"glMultiDrawElements\0",
            b"glMultiDrawElementsBaseVertex\0",
            b"glMultiTexCoord1d\0",
            b"glMultiTexCoord1dv\0",
            b"glMultiTexCoord1f\0",
            b"glMultiTexCoord1fv\0",
            b"glMultiTexCoord1i\0",
            b"glMultiTexCoord1iv\0",
            b"glMultiTexCoord1s\0",
            b"glMultiTexCoord1sv\0",
            b"glMultiTexCoord2d\0",
            b"glMultiTexCoord2dv\0",
            b"glMultiTexCoord2f\0",
            b"glMultiTexCoord2fv\0",
            b"glMultiTexCoord2i\0",
            b"glMultiTexCoord2iv\0",
            b"glMultiTexCoord2s\0",
            b"glMultiTexCoord2sv\0",
            b"glMultiTexCoord3d\0",
            b"glMultiTexCoord3dv\0",
            b"glMultiTexCoord3f\0",
            b"glMultiTexCoord3fv\0",
            b"glMultiTexCoord3i\0",
            b"glMultiTexCoord3iv\0",
            b"glMultiTexCoord3s\0",
            b"glMultiTexCoord3sv\0",
            b"glMultiTexCoord4d\0",
            b"glMultiTexCoord4dv\0",
            b"glMultiTexCoord4f\0",
            b"glMultiTexCoord4fv\0",
            b"glMultiTexCoord4i\0",
            b"glMultiTexCoord4iv\0",
            b"glMultiTexCoord4s\0",
            b"glMultiTexCoord4sv\0",
            b"glNewList\0",
            b"glNormal3b\0",
            b"glNormal3bv\0",
            b"glNormal3d\0",
            b"glNormal3dv\0",
            b"glNormal3f\0",
            b"glNormal3fv\0",
            b"glNormal3i\0",
            b"glNormal3iv\0",
            b"glNormal3s\0",
            b"glNormal3sv\0",
            b"glNormalPointer\0",
            b"glOrtho\0",
            b"glPassThrough\0",
            b"glPixelMapfv\0",
            b"glPixelMapuiv\0",
            b"glPixelMapusv\0",
            b"glPixelStoref\0",
            b"glPixelStorei\0",
            b"glPixelTransferf\0",
            b"glPixelTransferi\0",
            b"glPixelZoom\0",
            b"glPointParameterf\0",
            b"glPointParameterfv\0",
            b"glPointParameteri\0",
            b"glPointParameteriv\0",
            b"glPointSize\0",
            b"glPolygonMode\0",
            b"glPolygonOffset\0",
            b"glPolygonStipple\0",
            b"glPopAttrib\0",
            b"glPopClientAttrib\0",
            b"glPopMatrix\0",
            b"glPopName\0",
            b"glPrimitiveRestartIndex\0",
            b"glPrioritizeTextures\0",
            b"glProvokingVertex\0",
            b"glPushAttrib\0",
            b"glPushClientAttrib\0",
            b"glPushMatrix\0",
            b"glPushName\0",
            b"glRasterPos2d\0",
            b"glRasterPos2dv\0",
            b"glRasterPos2f\0",
            b"glRasterPos2fv\0",
            b"glRasterPos2i\0",
            b"glRasterPos2iv\0",
            b"glRasterPos2s\0",
            b"glRasterPos2sv\0",
            b"glRasterPos3d\0",
            b"glRasterPos3dv\0",
            b"glRasterPos3f\0",
            b"glRasterPos3fv\0",
            b"glRasterPos3i\0",
            b"glRasterPos3iv\0",
            b"glRasterPos3s\0",
            b"glRasterPos3sv\0",
            b"glRasterPos4d\0",
            b"glRasterPos4dv\0",
            b"glRasterPos4f\0",
            b"glRasterPos4fv\0",
            b"glRasterPos4i\0",
            b"glRasterPos4iv\0",
            b"glRasterPos4s\0",
            b"glRasterPos4sv\0",
            b"glReadBuffer\0",
            b"glReadPixels\0",
            b"glRectd\0",
            b"glRectdv\0",
            b"glRectf\0",
            b"glRectfv\0",
            b"glRecti\0",
            b"glRectiv\0",
            b"glRects\0",
            b"glRectsv\0",
            b"glRenderMode\0",
            b"glRenderbufferStorage\0",
            b"glRenderbufferStorageMultisample\0",
            b"glRotated\0",
            b"glRotatef\0",
            b"glSampleCoverage\0",
            b"glSampleMaski\0",
            b"glScaled\0",
            b"glScalef\0",
            b"glScissor\0",
            b"glSecondaryColor3b\0",
            b"glSecondaryColor3bv\0",
            b"glSecondaryColor3d\0",
            b"glSecondaryColor3dv\0",
            b"glSecondaryColor3f\0",
            b"glSecondaryColor3fv\0",
            b"glSecondaryColor3i\0",
            b"glSecondaryColor3iv\0",
            b"glSecondaryColor3s\0",
            b"glSecondaryColor3sv\0",
            b"glSecondaryColor3ub\0",
            b"glSecondaryColor3ubv\0",
            b"glSecondaryColor3ui\0",
            b"glSecondaryColor3uiv\0",
            b"glSecondaryColor3us\0",
            b"glSecondaryColor3usv\0",
            b"glSecondaryColorPointer\0",
            b"glSelectBuffer\0",
            b"glShadeModel\0",
            b"glShaderSource\0",
            b"glStencilFunc\0",
            b"glStencilFuncSeparate\0",
            b"glStencilMask\0",
            b"glStencilMaskSeparate\0",
            b"glStencilOp\0",
            b"glStencilOpSeparate\0",
            b"glTexBuffer\0",
            b"glTexCoord1d\0",
            b"glTexCoord1dv\0",
            b"glTexCoord1f\0",
            b"glTexCoord1fv\0",
            b"glTexCoord1i\0",
            b"glTexCoord1iv\0",
            b"glTexCoord1s\0",
            b"glTexCoord1sv\0",
            b"glTexCoord2d\0",
            b"glTexCoord2dv\0",
            b"glTexCoord2f\0",
            b"glTexCoord2fv\0",
            b"glTexCoord2i\0",
            b"glTexCoord2iv\0",
            b"glTexCoord2s\0",
            b"glTexCoord2sv\0",
            b"glTexCoord3d\0",
            b"glTexCoord3dv\0",
            b"glTexCoord3f\0",
            b"glTexCoord3fv\0",
            b"glTexCoord3i\0",
            b"glTexCoord3iv\0",
            b"glTexCoord3s\0",
            b"glTexCoord3sv\0",
            b"glTexCoord4d\0",
            b"glTexCoord4dv\0",
            b"glTexCoord4f\0",
            b"glTexCoord4fv\0",
            b"glTexCoord4i\0",
            b"glTexCoord4iv\0",
            b"glTexCoord4s\0",
            b"glTexCoord4sv\0",
            b"glTexCoordPointer\0",
            b"glTexEnvf\0",
            b"glTexEnvfv\0",
            b"glTexEnvi\0",
            b"glTexEnviv\0",
            b"glTexGend\0",
            b"glTexGendv\0",
            b"glTexGenf\0",
            b"glTexGenfv\0",
            b"glTexGeni\0",
            b"glTexGeniv\0",
            b"glTexImage1D\0",
            b"glTexImage2D\0",
            b"glTexImage2DMultisample\0",
            b"glTexImage3D\0",
            b"glTexImage3DMultisample\0",
            b"glTexParameterIiv\0",
            b"glTexParameterIuiv\0",
            b"glTexParameterf\0",
            b"glTexParameterfv\0",
            b"glTexParameteri\0",
            b"glTexParameteriv\0",
            b"glTexSubImage1D\0",
            b"glTexSubImage2D\0",
            b"glTexSubImage3D\0",
            b"glTransformFeedbackVaryings\0",
            b"glTranslated\0",
            b"glTranslatef\0",
            b"glUniform1f\0",
            b"glUniform1fv\0",
            b"glUniform1i\0",
            b"glUniform1iv\0",
            b"glUniform1ui\0",
            b"glUniform1uiv\0",
            b"glUniform2f\0",
            b"glUniform2fv\0",
            b"glUniform2i\0",
            b"glUniform2iv\0",
            b"glUniform2ui\0",
            b"glUniform2uiv\0",
            b"glUniform3f\0",
            b"glUniform3fv\0",
            b"glUniform3i\0",
            b"glUniform3iv\0",
            b"glUniform3ui\0",
            b"glUniform3uiv\0",
            b"glUniform4f\0",
            b"glUniform4fv\0",
            b"glUniform4i\0",
            b"glUniform4iv\0",
            b"glUniform4ui\0",
            b"glUniform4uiv\0",
            b"glUniformBlockBinding\0",
            b"glUniformMatrix2fv\0",
            b"glUniformMatrix2x3fv\0",
            b"glUniformMatrix2x4fv\0",
            b"glUniformMatrix3fv\0",
            b"glUniformMatrix3x2fv\0",
            b"glUniformMatrix3x4fv\0",
            b"glUniformMatrix4fv\0",
            b"glUniformMatrix4x2fv\0",
            b"glUniformMatrix4x3fv\0",
            b"glUnmapBuffer\0",
            b"glUseProgram\0",
            b"glValidateProgram\0",
            b"glVertex2d\0",
            b"glVertex2dv\0",
            b"glVertex2f\0",
            b"glVertex2fv\0",
            b"glVertex2i\0",
            b"glVertex2iv\0",
            b"glVertex2s\0",
            b"glVertex2sv\0",
            b"glVertex3d\0",
            b"glVertex3dv\0",
            b"glVertex3f\0",
            b"glVertex3fv\0",
            b"glVertex3i\0",
            b"glVertex3iv\0",
            b"glVertex3s\0",
            b"glVertex3sv\0",
            b"glVertex4d\0",
            b"glVertex4dv\0",
            b"glVertex4f\0",
            b"glVertex4fv\0",
            b"glVertex4i\0",
            b"glVertex4iv\0",
            b"glVertex4s\0",
            b"glVertex4sv\0",
            b"glVertexAttrib1d\0",
            b"glVertexAttrib1dv\0",
            b"glVertexAttrib1f\0",
            b"glVertexAttrib1fv\0",
            b"glVertexAttrib1s\0",
            b"glVertexAttrib1sv\0",
            b"glVertexAttrib2d\0",
            b"glVertexAttrib2dv\0",
            b"glVertexAttrib2f\0",
            b"glVertexAttrib2fv\0",
            b"glVertexAttrib2s\0",
            b"glVertexAttrib2sv\0",
            b"glVertexAttrib3d\0",
            b"glVertexAttrib3dv\0",
            b"glVertexAttrib3f\0",
            b"glVertexAttrib3fv\0",
            b"glVertexAttrib3s\0",
            b"glVertexAttrib3sv\0",
            b"glVertexAttrib4Nbv\0",
            b"glVertexAttrib4Niv\0",
            b"glVertexAttrib4Nsv\0",
            b"glVertexAttrib4Nub\0",
            b"glVertexAttrib4Nubv\0",
            b"glVertexAttrib4Nuiv\0",
            b"glVertexAttrib4Nusv\0",
            b"glVertexAttrib4bv\0",
            b"glVertexAttrib4d\0",
            b"glVertexAttrib4dv\0",
            b"glVertexAttrib4f\0",
            b"glVertexAttrib4fv\0",
            b"glVertexAttrib4iv\0",
            b"glVertexAttrib4s\0",
            b"glVertexAttrib4sv\0",
            b"glVertexAttrib4ubv\0",
            b"glVertexAttrib4uiv\0",
            b"glVertexAttrib4usv\0",
            b"glVertexAttribI1i\0",
            b"glVertexAttribI1iv\0",
            b"glVertexAttribI1ui\0",
            b"glVertexAttribI1uiv\0",
            b"glVertexAttribI2i\0",
            b"glVertexAttribI2iv\0",
            b"glVertexAttribI2ui\0",
            b"glVertexAttribI2uiv\0",
            b"glVertexAttribI3i\0",
            b"glVertexAttribI3iv\0",
            b"glVertexAttribI3ui\0",
            b"glVertexAttribI3uiv\0",
            b"glVertexAttribI4bv\0",
            b"glVertexAttribI4i\0",
            b"glVertexAttribI4iv\0",
            b"glVertexAttribI4sv\0",
            b"glVertexAttribI4ubv\0",
            b"glVertexAttribI4ui\0",
            b"glVertexAttribI4uiv\0",
            b"glVertexAttribI4usv\0",
            b"glVertexAttribIPointer\0",
            b"glVertexAttribPointer\0",
            b"glVertexPointer\0",
            b"glViewport\0",
            b"glWaitSync\0",
            b"glWindowPos2d\0",
            b"glWindowPos2dv\0",
            b"glWindowPos2f\0",
            b"glWindowPos2fv\0",
            b"glWindowPos2i\0",
            b"glWindowPos2iv\0",
            b"glWindowPos2s\0",
            b"glWindowPos2sv\0",
            b"glWindowPos3d\0",
            b"glWindowPos3dv\0",
            b"glWindowPos3f\0",
            b"glWindowPos3fv\0",
            b"glWindowPos3i\0",
            b"glWindowPos3iv\0",
            b"glWindowPos3s\0",
            b"glWindowPos3sv\0",
        ])?;
        procs[666].write(glDebugMessageCallbackARB_null_imp as *const ());
        procs[667].write(glDebugMessageControlARB_null_imp as *const ());
        procs[668].write(glDebugMessageInsertARB_null_imp as *const ());
        procs[669].write(glGetDebugMessageLogARB_null_imp as *const ());
        let procs = unsafe { transmute(procs) };
        #[allow(unused_mut)] let mut ret = Procs {
            procs,
            has_ARB_debug_output: false,
        };
        let disabled_extensions = std::env::var("GL_DISABLED_EXTENSIONS");
        let disabled_extensions = disabled_extensions.as_ref()
            .map(|x| x.as_bytes()).unwrap_or(b"");
        let disabled_extensions
            = build_disabled_extension_list(disabled_extensions);
        let mut num_extensions = 0;
        unsafe { ret.GetIntegerv(GL_NUM_EXTENSIONS, &mut num_extensions) };
        for i in 0 .. num_extensions as GLuint {
            let ext = unsafe {CStr::from_ptr(transmute(ret.GetStringi(GL_EXTENSIONS, i)))}.to_bytes();
            if disabled_extensions.contains(ext) { continue }
            match ext {
                b"GL_ARB_debug_output" => ret.has_ARB_debug_output = true,
            _ => (),
            }
        }
        if ret.has_ARB_debug_output {
            Procs::getprocs(&get_proc,
                            unsafe { transmute(&mut ret.procs[666..670]) }, &[
                b"glDebugMessageCallbackARB\0",
                b"glDebugMessageControlARB\0",
                b"glDebugMessageInsertARB\0",
                b"glGetDebugMessageLogARB\0",
            ])?;
        }
        Ok(ret)
    }
    fn getprocs<E, F: Fn(&[u8])->Result<*const(),E>>(get_proc: &F, range: &mut[MaybeUninit<*const ()>], names: &[&[u8]]) -> Result<(), E> {
        debug_assert_eq!(range.len(), names.len());
        for i in 0..range.len() {
            range[i].write(unsafe {transmute(get_proc(names[i])?)});
        }
        Ok(())
    }
    #[inline(always)] pub unsafe fn Accum(&self, op: GLenum, value: GLfloat) { (transmute::<_, extern "C" fn(op: GLenum, value: GLfloat)>(self.procs[0]))(op, value) }
    #[inline(always)] pub unsafe fn ActiveTexture(&self, texture: GLenum) { (transmute::<_, extern "C" fn(texture: GLenum)>(self.procs[1]))(texture) }
    #[inline(always)] pub unsafe fn AlphaFunc(&self, func: GLenum, r#ref: GLfloat) { (transmute::<_, extern "C" fn(func: GLenum, r#ref: GLfloat)>(self.procs[2]))(func, r#ref) }
    #[inline(always)] pub unsafe fn AreTexturesResident(&self, n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean { (transmute::<_, extern "C" fn(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean>(self.procs[3]))(n, textures, residences) }
    #[inline(always)] pub unsafe fn ArrayElement(&self, i: GLint) { (transmute::<_, extern "C" fn(i: GLint)>(self.procs[4]))(i) }
    #[inline(always)] pub unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) { (transmute::<_, extern "C" fn(program: GLuint, shader: GLuint)>(self.procs[5]))(program, shader) }
    #[inline(always)] pub unsafe fn Begin(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[6]))(mode) }
    #[inline(always)] pub unsafe fn BeginConditionalRender(&self, id: GLuint, mode: GLenum) { (transmute::<_, extern "C" fn(id: GLuint, mode: GLenum)>(self.procs[7]))(id, mode) }
    #[inline(always)] pub unsafe fn BeginQuery(&self, target: GLenum, id: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, id: GLuint)>(self.procs[8]))(target, id) }
    #[inline(always)] pub unsafe fn BeginTransformFeedback(&self, primitiveMode: GLenum) { (transmute::<_, extern "C" fn(primitiveMode: GLenum)>(self.procs[9]))(primitiveMode) }
    #[inline(always)] pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) { (transmute::<_, extern "C" fn(program: GLuint, index: GLuint, name: *const GLchar)>(self.procs[10]))(program, index, name) }
    #[inline(always)] pub unsafe fn BindBuffer(&self, target: GLenum, buffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, buffer: GLuint)>(self.procs[11]))(target, buffer) }
    #[inline(always)] pub unsafe fn BindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint)>(self.procs[12]))(target, index, buffer) }
    #[inline(always)] pub unsafe fn BindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>(self.procs[13]))(target, index, buffer, offset, size) }
    #[inline(always)] pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) { (transmute::<_, extern "C" fn(program: GLuint, color: GLuint, name: *const GLchar)>(self.procs[14]))(program, color, name) }
    #[inline(always)] pub unsafe fn BindFramebuffer(&self, target: GLenum, framebuffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, framebuffer: GLuint)>(self.procs[15]))(target, framebuffer) }
    #[inline(always)] pub unsafe fn BindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, renderbuffer: GLuint)>(self.procs[16]))(target, renderbuffer) }
    #[inline(always)] pub unsafe fn BindTexture(&self, target: GLenum, texture: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, texture: GLuint)>(self.procs[17]))(target, texture) }
    #[inline(always)] pub unsafe fn BindVertexArray(&self, array: GLuint) { (transmute::<_, extern "C" fn(array: GLuint)>(self.procs[18]))(array) }
    #[inline(always)] pub unsafe fn Bitmap(&self, width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte) { (transmute::<_, extern "C" fn(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte)>(self.procs[19]))(width, height, xorig, yorig, xmove, ymove, bitmap) }
    #[inline(always)] pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>(self.procs[20]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn BlendEquation(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[21]))(mode) }
    #[inline(always)] pub unsafe fn BlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) { (transmute::<_, extern "C" fn(modeRGB: GLenum, modeAlpha: GLenum)>(self.procs[22]))(modeRGB, modeAlpha) }
    #[inline(always)] pub unsafe fn BlendFunc(&self, sfactor: GLenum, dfactor: GLenum) { (transmute::<_, extern "C" fn(sfactor: GLenum, dfactor: GLenum)>(self.procs[23]))(sfactor, dfactor) }
    #[inline(always)] pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { (transmute::<_, extern "C" fn(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum)>(self.procs[24]))(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
    #[inline(always)] pub unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { (transmute::<_, extern "C" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum)>(self.procs[25]))(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    #[inline(always)] pub unsafe fn BufferData(&self, target: GLenum, size: GLsizeiptr, data: *const libc::c_void, usage: GLenum) { (transmute::<_, extern "C" fn(target: GLenum, size: GLsizeiptr, data: *const libc::c_void, usage: GLenum)>(self.procs[26]))(target, size, data, usage) }
    #[inline(always)] pub unsafe fn BufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const libc::c_void)>(self.procs[27]))(target, offset, size, data) }
    #[inline(always)] pub unsafe fn CallList(&self, list: GLuint) { (transmute::<_, extern "C" fn(list: GLuint)>(self.procs[28]))(list) }
    #[inline(always)] pub unsafe fn CallLists(&self, n: GLsizei, r#type: GLenum, lists: *const libc::c_void) { (transmute::<_, extern "C" fn(n: GLsizei, r#type: GLenum, lists: *const libc::c_void)>(self.procs[29]))(n, r#type, lists) }
    #[inline(always)] pub unsafe fn CheckFramebufferStatus(&self, target: GLenum) -> GLenum { (transmute::<_, extern "C" fn(target: GLenum) -> GLenum>(self.procs[30]))(target) }
    #[inline(always)] pub unsafe fn ClampColor(&self, target: GLenum, clamp: GLenum) { (transmute::<_, extern "C" fn(target: GLenum, clamp: GLenum)>(self.procs[31]))(target, clamp) }
    #[inline(always)] pub unsafe fn Clear(&self, mask: GLbitfield) { (transmute::<_, extern "C" fn(mask: GLbitfield)>(self.procs[32]))(mask) }
    #[inline(always)] pub unsafe fn ClearAccum(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>(self.procs[33]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn ClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { (transmute::<_, extern "C" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint)>(self.procs[34]))(buffer, drawbuffer, depth, stencil) }
    #[inline(always)] pub unsafe fn ClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) { (transmute::<_, extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat)>(self.procs[35]))(buffer, drawbuffer, value) }
    #[inline(always)] pub unsafe fn ClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) { (transmute::<_, extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint)>(self.procs[36]))(buffer, drawbuffer, value) }
    #[inline(always)] pub unsafe fn ClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) { (transmute::<_, extern "C" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint)>(self.procs[37]))(buffer, drawbuffer, value) }
    #[inline(always)] pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>(self.procs[38]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn ClearDepth(&self, depth: GLdouble) { (transmute::<_, extern "C" fn(depth: GLdouble)>(self.procs[39]))(depth) }
    #[inline(always)] pub unsafe fn ClearIndex(&self, c: GLfloat) { (transmute::<_, extern "C" fn(c: GLfloat)>(self.procs[40]))(c) }
    #[inline(always)] pub unsafe fn ClearStencil(&self, s: GLint) { (transmute::<_, extern "C" fn(s: GLint)>(self.procs[41]))(s) }
    #[inline(always)] pub unsafe fn ClientActiveTexture(&self, texture: GLenum) { (transmute::<_, extern "C" fn(texture: GLenum)>(self.procs[42]))(texture) }
    #[inline(always)] pub unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { (transmute::<_, extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum>(self.procs[43]))(sync, flags, timeout) }
    #[inline(always)] pub unsafe fn ClipPlane(&self, plane: GLenum, equation: *const GLdouble) { (transmute::<_, extern "C" fn(plane: GLenum, equation: *const GLdouble)>(self.procs[44]))(plane, equation) }
    #[inline(always)] pub unsafe fn Color3b(&self, red: GLbyte, green: GLbyte, blue: GLbyte) { (transmute::<_, extern "C" fn(red: GLbyte, green: GLbyte, blue: GLbyte)>(self.procs[45]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3bv(&self, v: *const GLbyte) { (transmute::<_, extern "C" fn(v: *const GLbyte)>(self.procs[46]))(v) }
    #[inline(always)] pub unsafe fn Color3d(&self, red: GLdouble, green: GLdouble, blue: GLdouble) { (transmute::<_, extern "C" fn(red: GLdouble, green: GLdouble, blue: GLdouble)>(self.procs[47]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[48]))(v) }
    #[inline(always)] pub unsafe fn Color3f(&self, red: GLfloat, green: GLfloat, blue: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat)>(self.procs[49]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[50]))(v) }
    #[inline(always)] pub unsafe fn Color3i(&self, red: GLint, green: GLint, blue: GLint) { (transmute::<_, extern "C" fn(red: GLint, green: GLint, blue: GLint)>(self.procs[51]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[52]))(v) }
    #[inline(always)] pub unsafe fn Color3s(&self, red: GLshort, green: GLshort, blue: GLshort) { (transmute::<_, extern "C" fn(red: GLshort, green: GLshort, blue: GLshort)>(self.procs[53]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[54]))(v) }
    #[inline(always)] pub unsafe fn Color3ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte) { (transmute::<_, extern "C" fn(red: GLubyte, green: GLubyte, blue: GLubyte)>(self.procs[55]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3ubv(&self, v: *const GLubyte) { (transmute::<_, extern "C" fn(v: *const GLubyte)>(self.procs[56]))(v) }
    #[inline(always)] pub unsafe fn Color3ui(&self, red: GLuint, green: GLuint, blue: GLuint) { (transmute::<_, extern "C" fn(red: GLuint, green: GLuint, blue: GLuint)>(self.procs[57]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3uiv(&self, v: *const GLuint) { (transmute::<_, extern "C" fn(v: *const GLuint)>(self.procs[58]))(v) }
    #[inline(always)] pub unsafe fn Color3us(&self, red: GLushort, green: GLushort, blue: GLushort) { (transmute::<_, extern "C" fn(red: GLushort, green: GLushort, blue: GLushort)>(self.procs[59]))(red, green, blue) }
    #[inline(always)] pub unsafe fn Color3usv(&self, v: *const GLushort) { (transmute::<_, extern "C" fn(v: *const GLushort)>(self.procs[60]))(v) }
    #[inline(always)] pub unsafe fn Color4b(&self, red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) { (transmute::<_, extern "C" fn(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte)>(self.procs[61]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4bv(&self, v: *const GLbyte) { (transmute::<_, extern "C" fn(v: *const GLbyte)>(self.procs[62]))(v) }
    #[inline(always)] pub unsafe fn Color4d(&self, red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble) { (transmute::<_, extern "C" fn(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble)>(self.procs[63]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[64]))(v) }
    #[inline(always)] pub unsafe fn Color4f(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>(self.procs[65]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[66]))(v) }
    #[inline(always)] pub unsafe fn Color4i(&self, red: GLint, green: GLint, blue: GLint, alpha: GLint) { (transmute::<_, extern "C" fn(red: GLint, green: GLint, blue: GLint, alpha: GLint)>(self.procs[67]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[68]))(v) }
    #[inline(always)] pub unsafe fn Color4s(&self, red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) { (transmute::<_, extern "C" fn(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort)>(self.procs[69]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[70]))(v) }
    #[inline(always)] pub unsafe fn Color4ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) { (transmute::<_, extern "C" fn(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte)>(self.procs[71]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4ubv(&self, v: *const GLubyte) { (transmute::<_, extern "C" fn(v: *const GLubyte)>(self.procs[72]))(v) }
    #[inline(always)] pub unsafe fn Color4ui(&self, red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) { (transmute::<_, extern "C" fn(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint)>(self.procs[73]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4uiv(&self, v: *const GLuint) { (transmute::<_, extern "C" fn(v: *const GLuint)>(self.procs[74]))(v) }
    #[inline(always)] pub unsafe fn Color4us(&self, red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) { (transmute::<_, extern "C" fn(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort)>(self.procs[75]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn Color4usv(&self, v: *const GLushort) { (transmute::<_, extern "C" fn(v: *const GLushort)>(self.procs[76]))(v) }
    #[inline(always)] pub unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { (transmute::<_, extern "C" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean)>(self.procs[77]))(red, green, blue, alpha) }
    #[inline(always)] pub unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) { (transmute::<_, extern "C" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean)>(self.procs[78]))(index, r, g, b, a) }
    #[inline(always)] pub unsafe fn ColorMaterial(&self, face: GLenum, mode: GLenum) { (transmute::<_, extern "C" fn(face: GLenum, mode: GLenum)>(self.procs[79]))(face, mode) }
    #[inline(always)] pub unsafe fn ColorPointer(&self, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[80]))(size, r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn CompileShader(&self, shader: GLuint) { (transmute::<_, extern "C" fn(shader: GLuint)>(self.procs[81]))(shader) }
    #[inline(always)] pub unsafe fn CompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[82]))(target, level, internalformat, width, border, imageSize, data) }
    #[inline(always)] pub unsafe fn CompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[83]))(target, level, internalformat, width, height, border, imageSize, data) }
    #[inline(always)] pub unsafe fn CompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[84]))(target, level, internalformat, width, height, depth, border, imageSize, data) }
    #[inline(always)] pub unsafe fn CompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[85]))(target, level, xoffset, width, format, imageSize, data) }
    #[inline(always)] pub unsafe fn CompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[86]))(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
    #[inline(always)] pub unsafe fn CompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const libc::c_void)>(self.procs[87]))(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
    #[inline(always)] pub unsafe fn CopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { (transmute::<_, extern "C" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr)>(self.procs[88]))(readTarget, writeTarget, readOffset, writeOffset, size) }
    #[inline(always)] pub unsafe fn CopyPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, r#type: GLenum) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, r#type: GLenum)>(self.procs[89]))(x, y, width, height, r#type) }
    #[inline(always)] pub unsafe fn CopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint)>(self.procs[90]))(target, level, internalformat, x, y, width, border) }
    #[inline(always)] pub unsafe fn CopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint)>(self.procs[91]))(target, level, internalformat, x, y, width, height, border) }
    #[inline(always)] pub unsafe fn CopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei)>(self.procs[92]))(target, level, xoffset, x, y, width) }
    #[inline(always)] pub unsafe fn CopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>(self.procs[93]))(target, level, xoffset, yoffset, x, y, width, height) }
    #[inline(always)] pub unsafe fn CopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>(self.procs[94]))(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
    #[inline(always)] pub unsafe fn CreateProgram(&self, ) -> GLuint { (transmute::<_, extern "C" fn() -> GLuint>(self.procs[95]))() }
    #[inline(always)] pub unsafe fn CreateShader(&self, r#type: GLenum) -> GLuint { (transmute::<_, extern "C" fn(r#type: GLenum) -> GLuint>(self.procs[96]))(r#type) }
    #[inline(always)] pub unsafe fn CullFace(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[97]))(mode) }
    #[inline(always)] pub unsafe fn DebugMessageCallbackARB(&self, callback: GLDEBUGPROCARB, userParam: *const libc::c_void) { (transmute::<_, extern "C" fn(callback: GLDEBUGPROCARB, userParam: *const libc::c_void)>(self.procs[666]))(callback, userParam) }
    #[inline(always)] pub unsafe fn DebugMessageControlARB(&self, source: GLenum, r#type: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) { (transmute::<_, extern "C" fn(source: GLenum, r#type: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean)>(self.procs[667]))(source, r#type, severity, count, ids, enabled) }
    #[inline(always)] pub unsafe fn DebugMessageInsertARB(&self, source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) { (transmute::<_, extern "C" fn(source: GLenum, r#type: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar)>(self.procs[668]))(source, r#type, id, severity, length, buf) }
    #[inline(always)] pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, buffers: *const GLuint)>(self.procs[98]))(n, buffers) }
    #[inline(always)] pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, framebuffers: *const GLuint)>(self.procs[99]))(n, framebuffers) }
    #[inline(always)] pub unsafe fn DeleteLists(&self, list: GLuint, range: GLsizei) { (transmute::<_, extern "C" fn(list: GLuint, range: GLsizei)>(self.procs[100]))(list, range) }
    #[inline(always)] pub unsafe fn DeleteProgram(&self, program: GLuint) { (transmute::<_, extern "C" fn(program: GLuint)>(self.procs[101]))(program) }
    #[inline(always)] pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, ids: *const GLuint)>(self.procs[102]))(n, ids) }
    #[inline(always)] pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, renderbuffers: *const GLuint)>(self.procs[103]))(n, renderbuffers) }
    #[inline(always)] pub unsafe fn DeleteShader(&self, shader: GLuint) { (transmute::<_, extern "C" fn(shader: GLuint)>(self.procs[104]))(shader) }
    #[inline(always)] pub unsafe fn DeleteSync(&self, sync: GLsync) { (transmute::<_, extern "C" fn(sync: GLsync)>(self.procs[105]))(sync) }
    #[inline(always)] pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, textures: *const GLuint)>(self.procs[106]))(n, textures) }
    #[inline(always)] pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, arrays: *const GLuint)>(self.procs[107]))(n, arrays) }
    #[inline(always)] pub unsafe fn DepthFunc(&self, func: GLenum) { (transmute::<_, extern "C" fn(func: GLenum)>(self.procs[108]))(func) }
    #[inline(always)] pub unsafe fn DepthMask(&self, flag: GLboolean) { (transmute::<_, extern "C" fn(flag: GLboolean)>(self.procs[109]))(flag) }
    #[inline(always)] pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) { (transmute::<_, extern "C" fn(n: GLdouble, f: GLdouble)>(self.procs[110]))(n, f) }
    #[inline(always)] pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) { (transmute::<_, extern "C" fn(program: GLuint, shader: GLuint)>(self.procs[111]))(program, shader) }
    #[inline(always)] pub unsafe fn Disable(&self, cap: GLenum) { (transmute::<_, extern "C" fn(cap: GLenum)>(self.procs[112]))(cap) }
    #[inline(always)] pub unsafe fn DisableClientState(&self, array: GLenum) { (transmute::<_, extern "C" fn(array: GLenum)>(self.procs[113]))(array) }
    #[inline(always)] pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) { (transmute::<_, extern "C" fn(index: GLuint)>(self.procs[114]))(index) }
    #[inline(always)] pub unsafe fn Disablei(&self, target: GLenum, index: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint)>(self.procs[115]))(target, index) }
    #[inline(always)] pub unsafe fn DrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) { (transmute::<_, extern "C" fn(mode: GLenum, first: GLint, count: GLsizei)>(self.procs[116]))(mode, first, count) }
    #[inline(always)] pub unsafe fn DrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) { (transmute::<_, extern "C" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei)>(self.procs[117]))(mode, first, count, instancecount) }
    #[inline(always)] pub unsafe fn DrawBuffer(&self, buf: GLenum) { (transmute::<_, extern "C" fn(buf: GLenum)>(self.procs[118]))(buf) }
    #[inline(always)] pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const GLenum) { (transmute::<_, extern "C" fn(n: GLsizei, bufs: *const GLenum)>(self.procs[119]))(n, bufs) }
    #[inline(always)] pub unsafe fn DrawElements(&self, mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void) { (transmute::<_, extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void)>(self.procs[120]))(mode, count, r#type, indices) }
    #[inline(always)] pub unsafe fn DrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, basevertex: GLint) { (transmute::<_, extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, basevertex: GLint)>(self.procs[121]))(mode, count, r#type, indices, basevertex) }
    #[inline(always)] pub unsafe fn DrawElementsInstanced(&self, mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, instancecount: GLsizei) { (transmute::<_, extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, instancecount: GLsizei)>(self.procs[122]))(mode, count, r#type, indices, instancecount) }
    #[inline(always)] pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, instancecount: GLsizei, basevertex: GLint) { (transmute::<_, extern "C" fn(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, instancecount: GLsizei, basevertex: GLint)>(self.procs[123]))(mode, count, r#type, indices, instancecount, basevertex) }
    #[inline(always)] pub unsafe fn DrawPixels(&self, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[124]))(width, height, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn DrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const libc::c_void) { (transmute::<_, extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const libc::c_void)>(self.procs[125]))(mode, start, end, count, r#type, indices) }
    #[inline(always)] pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, basevertex: GLint) { (transmute::<_, extern "C" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const libc::c_void, basevertex: GLint)>(self.procs[126]))(mode, start, end, count, r#type, indices, basevertex) }
    #[inline(always)] pub unsafe fn EdgeFlag(&self, flag: GLboolean) { (transmute::<_, extern "C" fn(flag: GLboolean)>(self.procs[127]))(flag) }
    #[inline(always)] pub unsafe fn EdgeFlagPointer(&self, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(stride: GLsizei, pointer: *const libc::c_void)>(self.procs[128]))(stride, pointer) }
    #[inline(always)] pub unsafe fn EdgeFlagv(&self, flag: *const GLboolean) { (transmute::<_, extern "C" fn(flag: *const GLboolean)>(self.procs[129]))(flag) }
    #[inline(always)] pub unsafe fn Enable(&self, cap: GLenum) { (transmute::<_, extern "C" fn(cap: GLenum)>(self.procs[130]))(cap) }
    #[inline(always)] pub unsafe fn EnableClientState(&self, array: GLenum) { (transmute::<_, extern "C" fn(array: GLenum)>(self.procs[131]))(array) }
    #[inline(always)] pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) { (transmute::<_, extern "C" fn(index: GLuint)>(self.procs[132]))(index) }
    #[inline(always)] pub unsafe fn Enablei(&self, target: GLenum, index: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint)>(self.procs[133]))(target, index) }
    #[inline(always)] pub unsafe fn End(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[134]))() }
    #[inline(always)] pub unsafe fn EndConditionalRender(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[135]))() }
    #[inline(always)] pub unsafe fn EndList(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[136]))() }
    #[inline(always)] pub unsafe fn EndQuery(&self, target: GLenum) { (transmute::<_, extern "C" fn(target: GLenum)>(self.procs[137]))(target) }
    #[inline(always)] pub unsafe fn EndTransformFeedback(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[138]))() }
    #[inline(always)] pub unsafe fn EvalCoord1d(&self, u: GLdouble) { (transmute::<_, extern "C" fn(u: GLdouble)>(self.procs[139]))(u) }
    #[inline(always)] pub unsafe fn EvalCoord1dv(&self, u: *const GLdouble) { (transmute::<_, extern "C" fn(u: *const GLdouble)>(self.procs[140]))(u) }
    #[inline(always)] pub unsafe fn EvalCoord1f(&self, u: GLfloat) { (transmute::<_, extern "C" fn(u: GLfloat)>(self.procs[141]))(u) }
    #[inline(always)] pub unsafe fn EvalCoord1fv(&self, u: *const GLfloat) { (transmute::<_, extern "C" fn(u: *const GLfloat)>(self.procs[142]))(u) }
    #[inline(always)] pub unsafe fn EvalCoord2d(&self, u: GLdouble, v: GLdouble) { (transmute::<_, extern "C" fn(u: GLdouble, v: GLdouble)>(self.procs[143]))(u, v) }
    #[inline(always)] pub unsafe fn EvalCoord2dv(&self, u: *const GLdouble) { (transmute::<_, extern "C" fn(u: *const GLdouble)>(self.procs[144]))(u) }
    #[inline(always)] pub unsafe fn EvalCoord2f(&self, u: GLfloat, v: GLfloat) { (transmute::<_, extern "C" fn(u: GLfloat, v: GLfloat)>(self.procs[145]))(u, v) }
    #[inline(always)] pub unsafe fn EvalCoord2fv(&self, u: *const GLfloat) { (transmute::<_, extern "C" fn(u: *const GLfloat)>(self.procs[146]))(u) }
    #[inline(always)] pub unsafe fn EvalMesh1(&self, mode: GLenum, i1: GLint, i2: GLint) { (transmute::<_, extern "C" fn(mode: GLenum, i1: GLint, i2: GLint)>(self.procs[147]))(mode, i1, i2) }
    #[inline(always)] pub unsafe fn EvalMesh2(&self, mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) { (transmute::<_, extern "C" fn(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint)>(self.procs[148]))(mode, i1, i2, j1, j2) }
    #[inline(always)] pub unsafe fn EvalPoint1(&self, i: GLint) { (transmute::<_, extern "C" fn(i: GLint)>(self.procs[149]))(i) }
    #[inline(always)] pub unsafe fn EvalPoint2(&self, i: GLint, j: GLint) { (transmute::<_, extern "C" fn(i: GLint, j: GLint)>(self.procs[150]))(i, j) }
    #[inline(always)] pub unsafe fn FeedbackBuffer(&self, size: GLsizei, r#type: GLenum, buffer: *mut GLfloat) { (transmute::<_, extern "C" fn(size: GLsizei, r#type: GLenum, buffer: *mut GLfloat)>(self.procs[151]))(size, r#type, buffer) }
    #[inline(always)] pub unsafe fn FenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync { (transmute::<_, extern "C" fn(condition: GLenum, flags: GLbitfield) -> GLsync>(self.procs[152]))(condition, flags) }
    #[inline(always)] pub unsafe fn Finish(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[153]))() }
    #[inline(always)] pub unsafe fn Flush(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[154]))() }
    #[inline(always)] pub unsafe fn FlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) { (transmute::<_, extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr)>(self.procs[155]))(target, offset, length) }
    #[inline(always)] pub unsafe fn FogCoordPointer(&self, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[156]))(r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn FogCoordd(&self, coord: GLdouble) { (transmute::<_, extern "C" fn(coord: GLdouble)>(self.procs[157]))(coord) }
    #[inline(always)] pub unsafe fn FogCoorddv(&self, coord: *const GLdouble) { (transmute::<_, extern "C" fn(coord: *const GLdouble)>(self.procs[158]))(coord) }
    #[inline(always)] pub unsafe fn FogCoordf(&self, coord: GLfloat) { (transmute::<_, extern "C" fn(coord: GLfloat)>(self.procs[159]))(coord) }
    #[inline(always)] pub unsafe fn FogCoordfv(&self, coord: *const GLfloat) { (transmute::<_, extern "C" fn(coord: *const GLfloat)>(self.procs[160]))(coord) }
    #[inline(always)] pub unsafe fn Fogf(&self, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLfloat)>(self.procs[161]))(pname, param) }
    #[inline(always)] pub unsafe fn Fogfv(&self, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLfloat)>(self.procs[162]))(pname, params) }
    #[inline(always)] pub unsafe fn Fogi(&self, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLint)>(self.procs[163]))(pname, param) }
    #[inline(always)] pub unsafe fn Fogiv(&self, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLint)>(self.procs[164]))(pname, params) }
    #[inline(always)] pub unsafe fn FramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint)>(self.procs[165]))(target, attachment, renderbuffertarget, renderbuffer) }
    #[inline(always)] pub unsafe fn FramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint)>(self.procs[166]))(target, attachment, texture, level) }
    #[inline(always)] pub unsafe fn FramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>(self.procs[167]))(target, attachment, textarget, texture, level) }
    #[inline(always)] pub unsafe fn FramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>(self.procs[168]))(target, attachment, textarget, texture, level) }
    #[inline(always)] pub unsafe fn FramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint)>(self.procs[169]))(target, attachment, textarget, texture, level, zoffset) }
    #[inline(always)] pub unsafe fn FramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint)>(self.procs[170]))(target, attachment, texture, level, layer) }
    #[inline(always)] pub unsafe fn FrontFace(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[171]))(mode) }
    #[inline(always)] pub unsafe fn Frustum(&self, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) { (transmute::<_, extern "C" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble)>(self.procs[172]))(left, right, bottom, top, zNear, zFar) }
    #[inline(always)] pub unsafe fn GenBuffers(&self, n: GLsizei, buffers: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, buffers: *mut GLuint)>(self.procs[173]))(n, buffers) }
    #[inline(always)] pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, framebuffers: *mut GLuint)>(self.procs[174]))(n, framebuffers) }
    #[inline(always)] pub unsafe fn GenLists(&self, range: GLsizei) -> GLuint { (transmute::<_, extern "C" fn(range: GLsizei) -> GLuint>(self.procs[175]))(range) }
    #[inline(always)] pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, ids: *mut GLuint)>(self.procs[176]))(n, ids) }
    #[inline(always)] pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, renderbuffers: *mut GLuint)>(self.procs[177]))(n, renderbuffers) }
    #[inline(always)] pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, textures: *mut GLuint)>(self.procs[178]))(n, textures) }
    #[inline(always)] pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) { (transmute::<_, extern "C" fn(n: GLsizei, arrays: *mut GLuint)>(self.procs[179]))(n, arrays) }
    #[inline(always)] pub unsafe fn GenerateMipmap(&self, target: GLenum) { (transmute::<_, extern "C" fn(target: GLenum)>(self.procs[180]))(target) }
    #[inline(always)] pub unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar)>(self.procs[181]))(program, index, bufSize, length, size, r#type, name) }
    #[inline(always)] pub unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, r#type: *mut GLenum, name: *mut GLchar)>(self.procs[182]))(program, index, bufSize, length, size, r#type, name) }
    #[inline(always)] pub unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar)>(self.procs[183]))(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
    #[inline(always)] pub unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[184]))(program, uniformBlockIndex, pname, params) }
    #[inline(always)] pub unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar)>(self.procs[185]))(program, uniformIndex, bufSize, length, uniformName) }
    #[inline(always)] pub unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint)>(self.procs[186]))(program, uniformCount, uniformIndices, pname, params) }
    #[inline(always)] pub unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) { (transmute::<_, extern "C" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint)>(self.procs[187]))(program, maxCount, count, shaders) }
    #[inline(always)] pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint { (transmute::<_, extern "C" fn(program: GLuint, name: *const GLchar) -> GLint>(self.procs[188]))(program, name) }
    #[inline(always)] pub unsafe fn GetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint, data: *mut GLboolean)>(self.procs[189]))(target, index, data) }
    #[inline(always)] pub unsafe fn GetBooleanv(&self, pname: GLenum, data: *mut GLboolean) { (transmute::<_, extern "C" fn(pname: GLenum, data: *mut GLboolean)>(self.procs[190]))(pname, data) }
    #[inline(always)] pub unsafe fn GetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint64)>(self.procs[191]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[192]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut*mut libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut*mut libc::c_void)>(self.procs[193]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut libc::c_void)>(self.procs[194]))(target, offset, size, data) }
    #[inline(always)] pub unsafe fn GetClipPlane(&self, plane: GLenum, equation: *mut GLdouble) { (transmute::<_, extern "C" fn(plane: GLenum, equation: *mut GLdouble)>(self.procs[195]))(plane, equation) }
    #[inline(always)] pub unsafe fn GetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, img: *mut libc::c_void)>(self.procs[196]))(target, level, img) }
    #[inline(always)] pub unsafe fn GetDebugMessageLogARB(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { (transmute::<_, extern "C" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint>(self.procs[669]))(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
    #[inline(always)] pub unsafe fn GetDoublev(&self, pname: GLenum, data: *mut GLdouble) { (transmute::<_, extern "C" fn(pname: GLenum, data: *mut GLdouble)>(self.procs[197]))(pname, data) }
    #[inline(always)] pub unsafe fn GetError(&self, ) -> GLenum { (transmute::<_, extern "C" fn() -> GLenum>(self.procs[198]))() }
    #[inline(always)] pub unsafe fn GetFloatv(&self, pname: GLenum, data: *mut GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, data: *mut GLfloat)>(self.procs[199]))(pname, data) }
    #[inline(always)] pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint { (transmute::<_, extern "C" fn(program: GLuint, name: *const GLchar) -> GLint>(self.procs[200]))(program, name) }
    #[inline(always)] pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[201]))(target, attachment, pname, params) }
    #[inline(always)] pub unsafe fn GetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint, data: *mut GLint64)>(self.procs[202]))(target, index, data) }
    #[inline(always)] pub unsafe fn GetInteger64v(&self, pname: GLenum, data: *mut GLint64) { (transmute::<_, extern "C" fn(pname: GLenum, data: *mut GLint64)>(self.procs[203]))(pname, data) }
    #[inline(always)] pub unsafe fn GetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint, data: *mut GLint)>(self.procs[204]))(target, index, data) }
    #[inline(always)] pub unsafe fn GetIntegerv(&self, pname: GLenum, data: *mut GLint) { (transmute::<_, extern "C" fn(pname: GLenum, data: *mut GLint)>(self.procs[205]))(pname, data) }
    #[inline(always)] pub unsafe fn GetLightfv(&self, light: GLenum, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, params: *mut GLfloat)>(self.procs[206]))(light, pname, params) }
    #[inline(always)] pub unsafe fn GetLightiv(&self, light: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[207]))(light, pname, params) }
    #[inline(always)] pub unsafe fn GetMapdv(&self, target: GLenum, query: GLenum, v: *mut GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, query: GLenum, v: *mut GLdouble)>(self.procs[208]))(target, query, v) }
    #[inline(always)] pub unsafe fn GetMapfv(&self, target: GLenum, query: GLenum, v: *mut GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, query: GLenum, v: *mut GLfloat)>(self.procs[209]))(target, query, v) }
    #[inline(always)] pub unsafe fn GetMapiv(&self, target: GLenum, query: GLenum, v: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, query: GLenum, v: *mut GLint)>(self.procs[210]))(target, query, v) }
    #[inline(always)] pub unsafe fn GetMaterialfv(&self, face: GLenum, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, params: *mut GLfloat)>(self.procs[211]))(face, pname, params) }
    #[inline(always)] pub unsafe fn GetMaterialiv(&self, face: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[212]))(face, pname, params) }
    #[inline(always)] pub unsafe fn GetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, index: GLuint, val: *mut GLfloat)>(self.procs[213]))(pname, index, val) }
    #[inline(always)] pub unsafe fn GetPixelMapfv(&self, map: GLenum, values: *mut GLfloat) { (transmute::<_, extern "C" fn(map: GLenum, values: *mut GLfloat)>(self.procs[214]))(map, values) }
    #[inline(always)] pub unsafe fn GetPixelMapuiv(&self, map: GLenum, values: *mut GLuint) { (transmute::<_, extern "C" fn(map: GLenum, values: *mut GLuint)>(self.procs[215]))(map, values) }
    #[inline(always)] pub unsafe fn GetPixelMapusv(&self, map: GLenum, values: *mut GLushort) { (transmute::<_, extern "C" fn(map: GLenum, values: *mut GLushort)>(self.procs[216]))(map, values) }
    #[inline(always)] pub unsafe fn GetPointerv(&self, pname: GLenum, params: *mut*mut libc::c_void) { (transmute::<_, extern "C" fn(pname: GLenum, params: *mut*mut libc::c_void)>(self.procs[217]))(pname, params) }
    #[inline(always)] pub unsafe fn GetPolygonStipple(&self, mask: *mut GLubyte) { (transmute::<_, extern "C" fn(mask: *mut GLubyte)>(self.procs[218]))(mask) }
    #[inline(always)] pub unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)>(self.procs[219]))(program, bufSize, length, infoLog) }
    #[inline(always)] pub unsafe fn GetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(program: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[220]))(program, pname, params) }
    #[inline(always)] pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[221]))(id, pname, params) }
    #[inline(always)] pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) { (transmute::<_, extern "C" fn(id: GLuint, pname: GLenum, params: *mut GLuint)>(self.procs[222]))(id, pname, params) }
    #[inline(always)] pub unsafe fn GetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[223]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[224]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { (transmute::<_, extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar)>(self.procs[225]))(shader, bufSize, length, infoLog) }
    #[inline(always)] pub unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) { (transmute::<_, extern "C" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar)>(self.procs[226]))(shader, bufSize, length, source) }
    #[inline(always)] pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(shader: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[227]))(shader, pname, params) }
    #[inline(always)] pub unsafe fn GetString(&self, name: GLenum) -> *const GLubyte { (transmute::<_, extern "C" fn(name: GLenum) -> *const GLubyte>(self.procs[228]))(name) }
    #[inline(always)] pub unsafe fn GetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte { (transmute::<_, extern "C" fn(name: GLenum, index: GLuint) -> *const GLubyte>(self.procs[229]))(name, index) }
    #[inline(always)] pub unsafe fn GetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) { (transmute::<_, extern "C" fn(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint)>(self.procs[230]))(sync, pname, count, length, values) }
    #[inline(always)] pub unsafe fn GetTexEnvfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfloat)>(self.procs[231]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTexEnviv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[232]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTexGendv(&self, coord: GLenum, pname: GLenum, params: *mut GLdouble) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *mut GLdouble)>(self.procs[233]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn GetTexGenfv(&self, coord: GLenum, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *mut GLfloat)>(self.procs[234]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn GetTexGeniv(&self, coord: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[235]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn GetTexImage(&self, target: GLenum, level: GLint, format: GLenum, r#type: GLenum, pixels: *mut libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, format: GLenum, r#type: GLenum, pixels: *mut libc::c_void)>(self.procs[236]))(target, level, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn GetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat)>(self.procs[237]))(target, level, pname, params) }
    #[inline(always)] pub unsafe fn GetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint)>(self.procs[238]))(target, level, pname, params) }
    #[inline(always)] pub unsafe fn GetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[239]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLuint)>(self.procs[240]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLfloat)>(self.procs[241]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *mut GLint)>(self.procs[242]))(target, pname, params) }
    #[inline(always)] pub unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, r#type: *mut GLenum, name: *mut GLchar) { (transmute::<_, extern "C" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, r#type: *mut GLenum, name: *mut GLchar)>(self.procs[243]))(program, index, bufSize, length, size, r#type, name) }
    #[inline(always)] pub unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint { (transmute::<_, extern "C" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint>(self.procs[244]))(program, uniformBlockName) }
    #[inline(always)] pub unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *mut*const GLchar, uniformIndices: *mut GLuint) { (transmute::<_, extern "C" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *mut*const GLchar, uniformIndices: *mut GLuint)>(self.procs[245]))(program, uniformCount, uniformNames, uniformIndices) }
    #[inline(always)] pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint { (transmute::<_, extern "C" fn(program: GLuint, name: *const GLchar) -> GLint>(self.procs[246]))(program, name) }
    #[inline(always)] pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) { (transmute::<_, extern "C" fn(program: GLuint, location: GLint, params: *mut GLfloat)>(self.procs[247]))(program, location, params) }
    #[inline(always)] pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) { (transmute::<_, extern "C" fn(program: GLuint, location: GLint, params: *mut GLint)>(self.procs[248]))(program, location, params) }
    #[inline(always)] pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) { (transmute::<_, extern "C" fn(program: GLuint, location: GLint, params: *mut GLuint)>(self.procs[249]))(program, location, params) }
    #[inline(always)] pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[250]))(index, pname, params) }
    #[inline(always)] pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLuint)>(self.procs[251]))(index, pname, params) }
    #[inline(always)] pub unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut*mut libc::c_void) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, pointer: *mut*mut libc::c_void)>(self.procs[252]))(index, pname, pointer) }
    #[inline(always)] pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLdouble)>(self.procs[253]))(index, pname, params) }
    #[inline(always)] pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLfloat)>(self.procs[254]))(index, pname, params) }
    #[inline(always)] pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) { (transmute::<_, extern "C" fn(index: GLuint, pname: GLenum, params: *mut GLint)>(self.procs[255]))(index, pname, params) }
    #[inline(always)] pub unsafe fn Hint(&self, target: GLenum, mode: GLenum) { (transmute::<_, extern "C" fn(target: GLenum, mode: GLenum)>(self.procs[256]))(target, mode) }
    #[inline(always)] pub unsafe fn IndexMask(&self, mask: GLuint) { (transmute::<_, extern "C" fn(mask: GLuint)>(self.procs[257]))(mask) }
    #[inline(always)] pub unsafe fn IndexPointer(&self, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[258]))(r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn Indexd(&self, c: GLdouble) { (transmute::<_, extern "C" fn(c: GLdouble)>(self.procs[259]))(c) }
    #[inline(always)] pub unsafe fn Indexdv(&self, c: *const GLdouble) { (transmute::<_, extern "C" fn(c: *const GLdouble)>(self.procs[260]))(c) }
    #[inline(always)] pub unsafe fn Indexf(&self, c: GLfloat) { (transmute::<_, extern "C" fn(c: GLfloat)>(self.procs[261]))(c) }
    #[inline(always)] pub unsafe fn Indexfv(&self, c: *const GLfloat) { (transmute::<_, extern "C" fn(c: *const GLfloat)>(self.procs[262]))(c) }
    #[inline(always)] pub unsafe fn Indexi(&self, c: GLint) { (transmute::<_, extern "C" fn(c: GLint)>(self.procs[263]))(c) }
    #[inline(always)] pub unsafe fn Indexiv(&self, c: *const GLint) { (transmute::<_, extern "C" fn(c: *const GLint)>(self.procs[264]))(c) }
    #[inline(always)] pub unsafe fn Indexs(&self, c: GLshort) { (transmute::<_, extern "C" fn(c: GLshort)>(self.procs[265]))(c) }
    #[inline(always)] pub unsafe fn Indexsv(&self, c: *const GLshort) { (transmute::<_, extern "C" fn(c: *const GLshort)>(self.procs[266]))(c) }
    #[inline(always)] pub unsafe fn Indexub(&self, c: GLubyte) { (transmute::<_, extern "C" fn(c: GLubyte)>(self.procs[267]))(c) }
    #[inline(always)] pub unsafe fn Indexubv(&self, c: *const GLubyte) { (transmute::<_, extern "C" fn(c: *const GLubyte)>(self.procs[268]))(c) }
    #[inline(always)] pub unsafe fn InitNames(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[269]))() }
    #[inline(always)] pub unsafe fn InterleavedArrays(&self, format: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(format: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[270]))(format, stride, pointer) }
    #[inline(always)] pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(buffer: GLuint) -> GLboolean>(self.procs[271]))(buffer) }
    #[inline(always)] pub unsafe fn IsEnabled(&self, cap: GLenum) -> GLboolean { (transmute::<_, extern "C" fn(cap: GLenum) -> GLboolean>(self.procs[272]))(cap) }
    #[inline(always)] pub unsafe fn IsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(target: GLenum, index: GLuint) -> GLboolean>(self.procs[273]))(target, index) }
    #[inline(always)] pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(framebuffer: GLuint) -> GLboolean>(self.procs[274]))(framebuffer) }
    #[inline(always)] pub unsafe fn IsList(&self, list: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(list: GLuint) -> GLboolean>(self.procs[275]))(list) }
    #[inline(always)] pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(program: GLuint) -> GLboolean>(self.procs[276]))(program) }
    #[inline(always)] pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(id: GLuint) -> GLboolean>(self.procs[277]))(id) }
    #[inline(always)] pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(renderbuffer: GLuint) -> GLboolean>(self.procs[278]))(renderbuffer) }
    #[inline(always)] pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(shader: GLuint) -> GLboolean>(self.procs[279]))(shader) }
    #[inline(always)] pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean { (transmute::<_, extern "C" fn(sync: GLsync) -> GLboolean>(self.procs[280]))(sync) }
    #[inline(always)] pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(texture: GLuint) -> GLboolean>(self.procs[281]))(texture) }
    #[inline(always)] pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean { (transmute::<_, extern "C" fn(array: GLuint) -> GLboolean>(self.procs[282]))(array) }
    #[inline(always)] pub unsafe fn LightModelf(&self, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLfloat)>(self.procs[283]))(pname, param) }
    #[inline(always)] pub unsafe fn LightModelfv(&self, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLfloat)>(self.procs[284]))(pname, params) }
    #[inline(always)] pub unsafe fn LightModeli(&self, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLint)>(self.procs[285]))(pname, param) }
    #[inline(always)] pub unsafe fn LightModeliv(&self, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLint)>(self.procs[286]))(pname, params) }
    #[inline(always)] pub unsafe fn Lightf(&self, light: GLenum, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, param: GLfloat)>(self.procs[287]))(light, pname, param) }
    #[inline(always)] pub unsafe fn Lightfv(&self, light: GLenum, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, params: *const GLfloat)>(self.procs[288]))(light, pname, params) }
    #[inline(always)] pub unsafe fn Lighti(&self, light: GLenum, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, param: GLint)>(self.procs[289]))(light, pname, param) }
    #[inline(always)] pub unsafe fn Lightiv(&self, light: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(light: GLenum, pname: GLenum, params: *const GLint)>(self.procs[290]))(light, pname, params) }
    #[inline(always)] pub unsafe fn LineStipple(&self, factor: GLint, pattern: GLushort) { (transmute::<_, extern "C" fn(factor: GLint, pattern: GLushort)>(self.procs[291]))(factor, pattern) }
    #[inline(always)] pub unsafe fn LineWidth(&self, width: GLfloat) { (transmute::<_, extern "C" fn(width: GLfloat)>(self.procs[292]))(width) }
    #[inline(always)] pub unsafe fn LinkProgram(&self, program: GLuint) { (transmute::<_, extern "C" fn(program: GLuint)>(self.procs[293]))(program) }
    #[inline(always)] pub unsafe fn ListBase(&self, base: GLuint) { (transmute::<_, extern "C" fn(base: GLuint)>(self.procs[294]))(base) }
    #[inline(always)] pub unsafe fn LoadIdentity(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[295]))() }
    #[inline(always)] pub unsafe fn LoadMatrixd(&self, m: *const GLdouble) { (transmute::<_, extern "C" fn(m: *const GLdouble)>(self.procs[296]))(m) }
    #[inline(always)] pub unsafe fn LoadMatrixf(&self, m: *const GLfloat) { (transmute::<_, extern "C" fn(m: *const GLfloat)>(self.procs[297]))(m) }
    #[inline(always)] pub unsafe fn LoadName(&self, name: GLuint) { (transmute::<_, extern "C" fn(name: GLuint)>(self.procs[298]))(name) }
    #[inline(always)] pub unsafe fn LoadTransposeMatrixd(&self, m: *const GLdouble) { (transmute::<_, extern "C" fn(m: *const GLdouble)>(self.procs[299]))(m) }
    #[inline(always)] pub unsafe fn LoadTransposeMatrixf(&self, m: *const GLfloat) { (transmute::<_, extern "C" fn(m: *const GLfloat)>(self.procs[300]))(m) }
    #[inline(always)] pub unsafe fn LogicOp(&self, opcode: GLenum) { (transmute::<_, extern "C" fn(opcode: GLenum)>(self.procs[301]))(opcode) }
    #[inline(always)] pub unsafe fn Map1d(&self, target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble)>(self.procs[302]))(target, u1, u2, stride, order, points) }
    #[inline(always)] pub unsafe fn Map1f(&self, target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat)>(self.procs[303]))(target, u1, u2, stride, order, points) }
    #[inline(always)] pub unsafe fn Map2d(&self, target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble)>(self.procs[304]))(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
    #[inline(always)] pub unsafe fn Map2f(&self, target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat)>(self.procs[305]))(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
    #[inline(always)] pub unsafe fn MapBuffer(&self, target: GLenum, access: GLenum) -> *mut libc::c_void { (transmute::<_, extern "C" fn(target: GLenum, access: GLenum) -> *mut libc::c_void>(self.procs[306]))(target, access) }
    #[inline(always)] pub unsafe fn MapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut libc::c_void { (transmute::<_, extern "C" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut libc::c_void>(self.procs[307]))(target, offset, length, access) }
    #[inline(always)] pub unsafe fn MapGrid1d(&self, un: GLint, u1: GLdouble, u2: GLdouble) { (transmute::<_, extern "C" fn(un: GLint, u1: GLdouble, u2: GLdouble)>(self.procs[308]))(un, u1, u2) }
    #[inline(always)] pub unsafe fn MapGrid1f(&self, un: GLint, u1: GLfloat, u2: GLfloat) { (transmute::<_, extern "C" fn(un: GLint, u1: GLfloat, u2: GLfloat)>(self.procs[309]))(un, u1, u2) }
    #[inline(always)] pub unsafe fn MapGrid2d(&self, un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble) { (transmute::<_, extern "C" fn(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble)>(self.procs[310]))(un, u1, u2, vn, v1, v2) }
    #[inline(always)] pub unsafe fn MapGrid2f(&self, un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat) { (transmute::<_, extern "C" fn(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat)>(self.procs[311]))(un, u1, u2, vn, v1, v2) }
    #[inline(always)] pub unsafe fn Materialf(&self, face: GLenum, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, param: GLfloat)>(self.procs[312]))(face, pname, param) }
    #[inline(always)] pub unsafe fn Materialfv(&self, face: GLenum, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, params: *const GLfloat)>(self.procs[313]))(face, pname, params) }
    #[inline(always)] pub unsafe fn Materiali(&self, face: GLenum, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, param: GLint)>(self.procs[314]))(face, pname, param) }
    #[inline(always)] pub unsafe fn Materialiv(&self, face: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(face: GLenum, pname: GLenum, params: *const GLint)>(self.procs[315]))(face, pname, params) }
    #[inline(always)] pub unsafe fn MatrixMode(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[316]))(mode) }
    #[inline(always)] pub unsafe fn MultMatrixd(&self, m: *const GLdouble) { (transmute::<_, extern "C" fn(m: *const GLdouble)>(self.procs[317]))(m) }
    #[inline(always)] pub unsafe fn MultMatrixf(&self, m: *const GLfloat) { (transmute::<_, extern "C" fn(m: *const GLfloat)>(self.procs[318]))(m) }
    #[inline(always)] pub unsafe fn MultTransposeMatrixd(&self, m: *const GLdouble) { (transmute::<_, extern "C" fn(m: *const GLdouble)>(self.procs[319]))(m) }
    #[inline(always)] pub unsafe fn MultTransposeMatrixf(&self, m: *const GLfloat) { (transmute::<_, extern "C" fn(m: *const GLfloat)>(self.procs[320]))(m) }
    #[inline(always)] pub unsafe fn MultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) { (transmute::<_, extern "C" fn(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei)>(self.procs[321]))(mode, first, count, drawcount) }
    #[inline(always)] pub unsafe fn MultiDrawElements(&self, mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *mut*const libc::c_void, drawcount: GLsizei) { (transmute::<_, extern "C" fn(mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *mut*const libc::c_void, drawcount: GLsizei)>(self.procs[322]))(mode, count, r#type, indices, drawcount) }
    #[inline(always)] pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *mut*const libc::c_void, drawcount: GLsizei, basevertex: *const GLint) { (transmute::<_, extern "C" fn(mode: GLenum, count: *const GLsizei, r#type: GLenum, indices: *mut*const libc::c_void, drawcount: GLsizei, basevertex: *const GLint)>(self.procs[323]))(mode, count, r#type, indices, drawcount, basevertex) }
    #[inline(always)] pub unsafe fn MultiTexCoord1d(&self, target: GLenum, s: GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, s: GLdouble)>(self.procs[324]))(target, s) }
    #[inline(always)] pub unsafe fn MultiTexCoord1dv(&self, target: GLenum, v: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLdouble)>(self.procs[325]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord1f(&self, target: GLenum, s: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, s: GLfloat)>(self.procs[326]))(target, s) }
    #[inline(always)] pub unsafe fn MultiTexCoord1fv(&self, target: GLenum, v: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLfloat)>(self.procs[327]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord1i(&self, target: GLenum, s: GLint) { (transmute::<_, extern "C" fn(target: GLenum, s: GLint)>(self.procs[328]))(target, s) }
    #[inline(always)] pub unsafe fn MultiTexCoord1iv(&self, target: GLenum, v: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLint)>(self.procs[329]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord1s(&self, target: GLenum, s: GLshort) { (transmute::<_, extern "C" fn(target: GLenum, s: GLshort)>(self.procs[330]))(target, s) }
    #[inline(always)] pub unsafe fn MultiTexCoord1sv(&self, target: GLenum, v: *const GLshort) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLshort)>(self.procs[331]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord2d(&self, target: GLenum, s: GLdouble, t: GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble)>(self.procs[332]))(target, s, t) }
    #[inline(always)] pub unsafe fn MultiTexCoord2dv(&self, target: GLenum, v: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLdouble)>(self.procs[333]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord2f(&self, target: GLenum, s: GLfloat, t: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat)>(self.procs[334]))(target, s, t) }
    #[inline(always)] pub unsafe fn MultiTexCoord2fv(&self, target: GLenum, v: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLfloat)>(self.procs[335]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord2i(&self, target: GLenum, s: GLint, t: GLint) { (transmute::<_, extern "C" fn(target: GLenum, s: GLint, t: GLint)>(self.procs[336]))(target, s, t) }
    #[inline(always)] pub unsafe fn MultiTexCoord2iv(&self, target: GLenum, v: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLint)>(self.procs[337]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord2s(&self, target: GLenum, s: GLshort, t: GLshort) { (transmute::<_, extern "C" fn(target: GLenum, s: GLshort, t: GLshort)>(self.procs[338]))(target, s, t) }
    #[inline(always)] pub unsafe fn MultiTexCoord2sv(&self, target: GLenum, v: *const GLshort) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLshort)>(self.procs[339]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord3d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble)>(self.procs[340]))(target, s, t, r) }
    #[inline(always)] pub unsafe fn MultiTexCoord3dv(&self, target: GLenum, v: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLdouble)>(self.procs[341]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord3f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat)>(self.procs[342]))(target, s, t, r) }
    #[inline(always)] pub unsafe fn MultiTexCoord3fv(&self, target: GLenum, v: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLfloat)>(self.procs[343]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord3i(&self, target: GLenum, s: GLint, t: GLint, r: GLint) { (transmute::<_, extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint)>(self.procs[344]))(target, s, t, r) }
    #[inline(always)] pub unsafe fn MultiTexCoord3iv(&self, target: GLenum, v: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLint)>(self.procs[345]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord3s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort) { (transmute::<_, extern "C" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort)>(self.procs[346]))(target, s, t, r) }
    #[inline(always)] pub unsafe fn MultiTexCoord3sv(&self, target: GLenum, v: *const GLshort) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLshort)>(self.procs[347]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord4d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble)>(self.procs[348]))(target, s, t, r, q) }
    #[inline(always)] pub unsafe fn MultiTexCoord4dv(&self, target: GLenum, v: *const GLdouble) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLdouble)>(self.procs[349]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord4f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat)>(self.procs[350]))(target, s, t, r, q) }
    #[inline(always)] pub unsafe fn MultiTexCoord4fv(&self, target: GLenum, v: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLfloat)>(self.procs[351]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord4i(&self, target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) { (transmute::<_, extern "C" fn(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint)>(self.procs[352]))(target, s, t, r, q) }
    #[inline(always)] pub unsafe fn MultiTexCoord4iv(&self, target: GLenum, v: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLint)>(self.procs[353]))(target, v) }
    #[inline(always)] pub unsafe fn MultiTexCoord4s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) { (transmute::<_, extern "C" fn(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort)>(self.procs[354]))(target, s, t, r, q) }
    #[inline(always)] pub unsafe fn MultiTexCoord4sv(&self, target: GLenum, v: *const GLshort) { (transmute::<_, extern "C" fn(target: GLenum, v: *const GLshort)>(self.procs[355]))(target, v) }
    #[inline(always)] pub unsafe fn NewList(&self, list: GLuint, mode: GLenum) { (transmute::<_, extern "C" fn(list: GLuint, mode: GLenum)>(self.procs[356]))(list, mode) }
    #[inline(always)] pub unsafe fn Normal3b(&self, nx: GLbyte, ny: GLbyte, nz: GLbyte) { (transmute::<_, extern "C" fn(nx: GLbyte, ny: GLbyte, nz: GLbyte)>(self.procs[357]))(nx, ny, nz) }
    #[inline(always)] pub unsafe fn Normal3bv(&self, v: *const GLbyte) { (transmute::<_, extern "C" fn(v: *const GLbyte)>(self.procs[358]))(v) }
    #[inline(always)] pub unsafe fn Normal3d(&self, nx: GLdouble, ny: GLdouble, nz: GLdouble) { (transmute::<_, extern "C" fn(nx: GLdouble, ny: GLdouble, nz: GLdouble)>(self.procs[359]))(nx, ny, nz) }
    #[inline(always)] pub unsafe fn Normal3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[360]))(v) }
    #[inline(always)] pub unsafe fn Normal3f(&self, nx: GLfloat, ny: GLfloat, nz: GLfloat) { (transmute::<_, extern "C" fn(nx: GLfloat, ny: GLfloat, nz: GLfloat)>(self.procs[361]))(nx, ny, nz) }
    #[inline(always)] pub unsafe fn Normal3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[362]))(v) }
    #[inline(always)] pub unsafe fn Normal3i(&self, nx: GLint, ny: GLint, nz: GLint) { (transmute::<_, extern "C" fn(nx: GLint, ny: GLint, nz: GLint)>(self.procs[363]))(nx, ny, nz) }
    #[inline(always)] pub unsafe fn Normal3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[364]))(v) }
    #[inline(always)] pub unsafe fn Normal3s(&self, nx: GLshort, ny: GLshort, nz: GLshort) { (transmute::<_, extern "C" fn(nx: GLshort, ny: GLshort, nz: GLshort)>(self.procs[365]))(nx, ny, nz) }
    #[inline(always)] pub unsafe fn Normal3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[366]))(v) }
    #[inline(always)] pub unsafe fn NormalPointer(&self, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[367]))(r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn Ortho(&self, left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) { (transmute::<_, extern "C" fn(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble)>(self.procs[368]))(left, right, bottom, top, zNear, zFar) }
    #[inline(always)] pub unsafe fn PassThrough(&self, token: GLfloat) { (transmute::<_, extern "C" fn(token: GLfloat)>(self.procs[369]))(token) }
    #[inline(always)] pub unsafe fn PixelMapfv(&self, map: GLenum, mapsize: GLsizei, values: *const GLfloat) { (transmute::<_, extern "C" fn(map: GLenum, mapsize: GLsizei, values: *const GLfloat)>(self.procs[370]))(map, mapsize, values) }
    #[inline(always)] pub unsafe fn PixelMapuiv(&self, map: GLenum, mapsize: GLsizei, values: *const GLuint) { (transmute::<_, extern "C" fn(map: GLenum, mapsize: GLsizei, values: *const GLuint)>(self.procs[371]))(map, mapsize, values) }
    #[inline(always)] pub unsafe fn PixelMapusv(&self, map: GLenum, mapsize: GLsizei, values: *const GLushort) { (transmute::<_, extern "C" fn(map: GLenum, mapsize: GLsizei, values: *const GLushort)>(self.procs[372]))(map, mapsize, values) }
    #[inline(always)] pub unsafe fn PixelStoref(&self, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLfloat)>(self.procs[373]))(pname, param) }
    #[inline(always)] pub unsafe fn PixelStorei(&self, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLint)>(self.procs[374]))(pname, param) }
    #[inline(always)] pub unsafe fn PixelTransferf(&self, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLfloat)>(self.procs[375]))(pname, param) }
    #[inline(always)] pub unsafe fn PixelTransferi(&self, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLint)>(self.procs[376]))(pname, param) }
    #[inline(always)] pub unsafe fn PixelZoom(&self, xfactor: GLfloat, yfactor: GLfloat) { (transmute::<_, extern "C" fn(xfactor: GLfloat, yfactor: GLfloat)>(self.procs[377]))(xfactor, yfactor) }
    #[inline(always)] pub unsafe fn PointParameterf(&self, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLfloat)>(self.procs[378]))(pname, param) }
    #[inline(always)] pub unsafe fn PointParameterfv(&self, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLfloat)>(self.procs[379]))(pname, params) }
    #[inline(always)] pub unsafe fn PointParameteri(&self, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(pname: GLenum, param: GLint)>(self.procs[380]))(pname, param) }
    #[inline(always)] pub unsafe fn PointParameteriv(&self, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(pname: GLenum, params: *const GLint)>(self.procs[381]))(pname, params) }
    #[inline(always)] pub unsafe fn PointSize(&self, size: GLfloat) { (transmute::<_, extern "C" fn(size: GLfloat)>(self.procs[382]))(size) }
    #[inline(always)] pub unsafe fn PolygonMode(&self, face: GLenum, mode: GLenum) { (transmute::<_, extern "C" fn(face: GLenum, mode: GLenum)>(self.procs[383]))(face, mode) }
    #[inline(always)] pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) { (transmute::<_, extern "C" fn(factor: GLfloat, units: GLfloat)>(self.procs[384]))(factor, units) }
    #[inline(always)] pub unsafe fn PolygonStipple(&self, mask: *const GLubyte) { (transmute::<_, extern "C" fn(mask: *const GLubyte)>(self.procs[385]))(mask) }
    #[inline(always)] pub unsafe fn PopAttrib(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[386]))() }
    #[inline(always)] pub unsafe fn PopClientAttrib(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[387]))() }
    #[inline(always)] pub unsafe fn PopMatrix(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[388]))() }
    #[inline(always)] pub unsafe fn PopName(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[389]))() }
    #[inline(always)] pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) { (transmute::<_, extern "C" fn(index: GLuint)>(self.procs[390]))(index) }
    #[inline(always)] pub unsafe fn PrioritizeTextures(&self, n: GLsizei, textures: *const GLuint, priorities: *const GLfloat) { (transmute::<_, extern "C" fn(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat)>(self.procs[391]))(n, textures, priorities) }
    #[inline(always)] pub unsafe fn ProvokingVertex(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[392]))(mode) }
    #[inline(always)] pub unsafe fn PushAttrib(&self, mask: GLbitfield) { (transmute::<_, extern "C" fn(mask: GLbitfield)>(self.procs[393]))(mask) }
    #[inline(always)] pub unsafe fn PushClientAttrib(&self, mask: GLbitfield) { (transmute::<_, extern "C" fn(mask: GLbitfield)>(self.procs[394]))(mask) }
    #[inline(always)] pub unsafe fn PushMatrix(&self, ) { (transmute::<_, extern "C" fn()>(self.procs[395]))() }
    #[inline(always)] pub unsafe fn PushName(&self, name: GLuint) { (transmute::<_, extern "C" fn(name: GLuint)>(self.procs[396]))(name) }
    #[inline(always)] pub unsafe fn RasterPos2d(&self, x: GLdouble, y: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble)>(self.procs[397]))(x, y) }
    #[inline(always)] pub unsafe fn RasterPos2dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[398]))(v) }
    #[inline(always)] pub unsafe fn RasterPos2f(&self, x: GLfloat, y: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat)>(self.procs[399]))(x, y) }
    #[inline(always)] pub unsafe fn RasterPos2fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[400]))(v) }
    #[inline(always)] pub unsafe fn RasterPos2i(&self, x: GLint, y: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint)>(self.procs[401]))(x, y) }
    #[inline(always)] pub unsafe fn RasterPos2iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[402]))(v) }
    #[inline(always)] pub unsafe fn RasterPos2s(&self, x: GLshort, y: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort)>(self.procs[403]))(x, y) }
    #[inline(always)] pub unsafe fn RasterPos2sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[404]))(v) }
    #[inline(always)] pub unsafe fn RasterPos3d(&self, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[405]))(x, y, z) }
    #[inline(always)] pub unsafe fn RasterPos3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[406]))(v) }
    #[inline(always)] pub unsafe fn RasterPos3f(&self, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[407]))(x, y, z) }
    #[inline(always)] pub unsafe fn RasterPos3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[408]))(v) }
    #[inline(always)] pub unsafe fn RasterPos3i(&self, x: GLint, y: GLint, z: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, z: GLint)>(self.procs[409]))(x, y, z) }
    #[inline(always)] pub unsafe fn RasterPos3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[410]))(v) }
    #[inline(always)] pub unsafe fn RasterPos3s(&self, x: GLshort, y: GLshort, z: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort, z: GLshort)>(self.procs[411]))(x, y, z) }
    #[inline(always)] pub unsafe fn RasterPos3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[412]))(v) }
    #[inline(always)] pub unsafe fn RasterPos4d(&self, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>(self.procs[413]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn RasterPos4dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[414]))(v) }
    #[inline(always)] pub unsafe fn RasterPos4f(&self, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat)>(self.procs[415]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn RasterPos4fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[416]))(v) }
    #[inline(always)] pub unsafe fn RasterPos4i(&self, x: GLint, y: GLint, z: GLint, w: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, z: GLint, w: GLint)>(self.procs[417]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn RasterPos4iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[418]))(v) }
    #[inline(always)] pub unsafe fn RasterPos4s(&self, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort)>(self.procs[419]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn RasterPos4sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[420]))(v) }
    #[inline(always)] pub unsafe fn ReadBuffer(&self, src: GLenum) { (transmute::<_, extern "C" fn(src: GLenum)>(self.procs[421]))(src) }
    #[inline(always)] pub unsafe fn ReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *mut libc::c_void) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *mut libc::c_void)>(self.procs[422]))(x, y, width, height, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn Rectd(&self, x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble) { (transmute::<_, extern "C" fn(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble)>(self.procs[423]))(x1, y1, x2, y2) }
    #[inline(always)] pub unsafe fn Rectdv(&self, v1: *const GLdouble, v2: *const GLdouble) { (transmute::<_, extern "C" fn(v1: *const GLdouble, v2: *const GLdouble)>(self.procs[424]))(v1, v2) }
    #[inline(always)] pub unsafe fn Rectf(&self, x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) { (transmute::<_, extern "C" fn(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat)>(self.procs[425]))(x1, y1, x2, y2) }
    #[inline(always)] pub unsafe fn Rectfv(&self, v1: *const GLfloat, v2: *const GLfloat) { (transmute::<_, extern "C" fn(v1: *const GLfloat, v2: *const GLfloat)>(self.procs[426]))(v1, v2) }
    #[inline(always)] pub unsafe fn Recti(&self, x1: GLint, y1: GLint, x2: GLint, y2: GLint) { (transmute::<_, extern "C" fn(x1: GLint, y1: GLint, x2: GLint, y2: GLint)>(self.procs[427]))(x1, y1, x2, y2) }
    #[inline(always)] pub unsafe fn Rectiv(&self, v1: *const GLint, v2: *const GLint) { (transmute::<_, extern "C" fn(v1: *const GLint, v2: *const GLint)>(self.procs[428]))(v1, v2) }
    #[inline(always)] pub unsafe fn Rects(&self, x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) { (transmute::<_, extern "C" fn(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort)>(self.procs[429]))(x1, y1, x2, y2) }
    #[inline(always)] pub unsafe fn Rectsv(&self, v1: *const GLshort, v2: *const GLshort) { (transmute::<_, extern "C" fn(v1: *const GLshort, v2: *const GLshort)>(self.procs[430]))(v1, v2) }
    #[inline(always)] pub unsafe fn RenderMode(&self, mode: GLenum) -> GLint { (transmute::<_, extern "C" fn(mode: GLenum) -> GLint>(self.procs[431]))(mode) }
    #[inline(always)] pub unsafe fn RenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei)>(self.procs[432]))(target, internalformat, width, height) }
    #[inline(always)] pub unsafe fn RenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>(self.procs[433]))(target, samples, internalformat, width, height) }
    #[inline(always)] pub unsafe fn Rotated(&self, angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[434]))(angle, x, y, z) }
    #[inline(always)] pub unsafe fn Rotatef(&self, angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[435]))(angle, x, y, z) }
    #[inline(always)] pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) { (transmute::<_, extern "C" fn(value: GLfloat, invert: GLboolean)>(self.procs[436]))(value, invert) }
    #[inline(always)] pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) { (transmute::<_, extern "C" fn(maskNumber: GLuint, mask: GLbitfield)>(self.procs[437]))(maskNumber, mask) }
    #[inline(always)] pub unsafe fn Scaled(&self, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[438]))(x, y, z) }
    #[inline(always)] pub unsafe fn Scalef(&self, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[439]))(x, y, z) }
    #[inline(always)] pub unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>(self.procs[440]))(x, y, width, height) }
    #[inline(always)] pub unsafe fn SecondaryColor3b(&self, red: GLbyte, green: GLbyte, blue: GLbyte) { (transmute::<_, extern "C" fn(red: GLbyte, green: GLbyte, blue: GLbyte)>(self.procs[441]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3bv(&self, v: *const GLbyte) { (transmute::<_, extern "C" fn(v: *const GLbyte)>(self.procs[442]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3d(&self, red: GLdouble, green: GLdouble, blue: GLdouble) { (transmute::<_, extern "C" fn(red: GLdouble, green: GLdouble, blue: GLdouble)>(self.procs[443]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[444]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3f(&self, red: GLfloat, green: GLfloat, blue: GLfloat) { (transmute::<_, extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat)>(self.procs[445]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[446]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3i(&self, red: GLint, green: GLint, blue: GLint) { (transmute::<_, extern "C" fn(red: GLint, green: GLint, blue: GLint)>(self.procs[447]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[448]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3s(&self, red: GLshort, green: GLshort, blue: GLshort) { (transmute::<_, extern "C" fn(red: GLshort, green: GLshort, blue: GLshort)>(self.procs[449]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[450]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte) { (transmute::<_, extern "C" fn(red: GLubyte, green: GLubyte, blue: GLubyte)>(self.procs[451]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3ubv(&self, v: *const GLubyte) { (transmute::<_, extern "C" fn(v: *const GLubyte)>(self.procs[452]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3ui(&self, red: GLuint, green: GLuint, blue: GLuint) { (transmute::<_, extern "C" fn(red: GLuint, green: GLuint, blue: GLuint)>(self.procs[453]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3uiv(&self, v: *const GLuint) { (transmute::<_, extern "C" fn(v: *const GLuint)>(self.procs[454]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColor3us(&self, red: GLushort, green: GLushort, blue: GLushort) { (transmute::<_, extern "C" fn(red: GLushort, green: GLushort, blue: GLushort)>(self.procs[455]))(red, green, blue) }
    #[inline(always)] pub unsafe fn SecondaryColor3usv(&self, v: *const GLushort) { (transmute::<_, extern "C" fn(v: *const GLushort)>(self.procs[456]))(v) }
    #[inline(always)] pub unsafe fn SecondaryColorPointer(&self, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[457]))(size, r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn SelectBuffer(&self, size: GLsizei, buffer: *mut GLuint) { (transmute::<_, extern "C" fn(size: GLsizei, buffer: *mut GLuint)>(self.procs[458]))(size, buffer) }
    #[inline(always)] pub unsafe fn ShadeModel(&self, mode: GLenum) { (transmute::<_, extern "C" fn(mode: GLenum)>(self.procs[459]))(mode) }
    #[inline(always)] pub unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: *mut*const GLchar, length: *const GLint) { (transmute::<_, extern "C" fn(shader: GLuint, count: GLsizei, string: *mut*const GLchar, length: *const GLint)>(self.procs[460]))(shader, count, string, length) }
    #[inline(always)] pub unsafe fn StencilFunc(&self, func: GLenum, r#ref: GLint, mask: GLuint) { (transmute::<_, extern "C" fn(func: GLenum, r#ref: GLint, mask: GLuint)>(self.procs[461]))(func, r#ref, mask) }
    #[inline(always)] pub unsafe fn StencilFuncSeparate(&self, face: GLenum, func: GLenum, r#ref: GLint, mask: GLuint) { (transmute::<_, extern "C" fn(face: GLenum, func: GLenum, r#ref: GLint, mask: GLuint)>(self.procs[462]))(face, func, r#ref, mask) }
    #[inline(always)] pub unsafe fn StencilMask(&self, mask: GLuint) { (transmute::<_, extern "C" fn(mask: GLuint)>(self.procs[463]))(mask) }
    #[inline(always)] pub unsafe fn StencilMaskSeparate(&self, face: GLenum, mask: GLuint) { (transmute::<_, extern "C" fn(face: GLenum, mask: GLuint)>(self.procs[464]))(face, mask) }
    #[inline(always)] pub unsafe fn StencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) { (transmute::<_, extern "C" fn(fail: GLenum, zfail: GLenum, zpass: GLenum)>(self.procs[465]))(fail, zfail, zpass) }
    #[inline(always)] pub unsafe fn StencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { (transmute::<_, extern "C" fn(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum)>(self.procs[466]))(face, sfail, dpfail, dppass) }
    #[inline(always)] pub unsafe fn TexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) { (transmute::<_, extern "C" fn(target: GLenum, internalformat: GLenum, buffer: GLuint)>(self.procs[467]))(target, internalformat, buffer) }
    #[inline(always)] pub unsafe fn TexCoord1d(&self, s: GLdouble) { (transmute::<_, extern "C" fn(s: GLdouble)>(self.procs[468]))(s) }
    #[inline(always)] pub unsafe fn TexCoord1dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[469]))(v) }
    #[inline(always)] pub unsafe fn TexCoord1f(&self, s: GLfloat) { (transmute::<_, extern "C" fn(s: GLfloat)>(self.procs[470]))(s) }
    #[inline(always)] pub unsafe fn TexCoord1fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[471]))(v) }
    #[inline(always)] pub unsafe fn TexCoord1i(&self, s: GLint) { (transmute::<_, extern "C" fn(s: GLint)>(self.procs[472]))(s) }
    #[inline(always)] pub unsafe fn TexCoord1iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[473]))(v) }
    #[inline(always)] pub unsafe fn TexCoord1s(&self, s: GLshort) { (transmute::<_, extern "C" fn(s: GLshort)>(self.procs[474]))(s) }
    #[inline(always)] pub unsafe fn TexCoord1sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[475]))(v) }
    #[inline(always)] pub unsafe fn TexCoord2d(&self, s: GLdouble, t: GLdouble) { (transmute::<_, extern "C" fn(s: GLdouble, t: GLdouble)>(self.procs[476]))(s, t) }
    #[inline(always)] pub unsafe fn TexCoord2dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[477]))(v) }
    #[inline(always)] pub unsafe fn TexCoord2f(&self, s: GLfloat, t: GLfloat) { (transmute::<_, extern "C" fn(s: GLfloat, t: GLfloat)>(self.procs[478]))(s, t) }
    #[inline(always)] pub unsafe fn TexCoord2fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[479]))(v) }
    #[inline(always)] pub unsafe fn TexCoord2i(&self, s: GLint, t: GLint) { (transmute::<_, extern "C" fn(s: GLint, t: GLint)>(self.procs[480]))(s, t) }
    #[inline(always)] pub unsafe fn TexCoord2iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[481]))(v) }
    #[inline(always)] pub unsafe fn TexCoord2s(&self, s: GLshort, t: GLshort) { (transmute::<_, extern "C" fn(s: GLshort, t: GLshort)>(self.procs[482]))(s, t) }
    #[inline(always)] pub unsafe fn TexCoord2sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[483]))(v) }
    #[inline(always)] pub unsafe fn TexCoord3d(&self, s: GLdouble, t: GLdouble, r: GLdouble) { (transmute::<_, extern "C" fn(s: GLdouble, t: GLdouble, r: GLdouble)>(self.procs[484]))(s, t, r) }
    #[inline(always)] pub unsafe fn TexCoord3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[485]))(v) }
    #[inline(always)] pub unsafe fn TexCoord3f(&self, s: GLfloat, t: GLfloat, r: GLfloat) { (transmute::<_, extern "C" fn(s: GLfloat, t: GLfloat, r: GLfloat)>(self.procs[486]))(s, t, r) }
    #[inline(always)] pub unsafe fn TexCoord3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[487]))(v) }
    #[inline(always)] pub unsafe fn TexCoord3i(&self, s: GLint, t: GLint, r: GLint) { (transmute::<_, extern "C" fn(s: GLint, t: GLint, r: GLint)>(self.procs[488]))(s, t, r) }
    #[inline(always)] pub unsafe fn TexCoord3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[489]))(v) }
    #[inline(always)] pub unsafe fn TexCoord3s(&self, s: GLshort, t: GLshort, r: GLshort) { (transmute::<_, extern "C" fn(s: GLshort, t: GLshort, r: GLshort)>(self.procs[490]))(s, t, r) }
    #[inline(always)] pub unsafe fn TexCoord3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[491]))(v) }
    #[inline(always)] pub unsafe fn TexCoord4d(&self, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) { (transmute::<_, extern "C" fn(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble)>(self.procs[492]))(s, t, r, q) }
    #[inline(always)] pub unsafe fn TexCoord4dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[493]))(v) }
    #[inline(always)] pub unsafe fn TexCoord4f(&self, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) { (transmute::<_, extern "C" fn(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat)>(self.procs[494]))(s, t, r, q) }
    #[inline(always)] pub unsafe fn TexCoord4fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[495]))(v) }
    #[inline(always)] pub unsafe fn TexCoord4i(&self, s: GLint, t: GLint, r: GLint, q: GLint) { (transmute::<_, extern "C" fn(s: GLint, t: GLint, r: GLint, q: GLint)>(self.procs[496]))(s, t, r, q) }
    #[inline(always)] pub unsafe fn TexCoord4iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[497]))(v) }
    #[inline(always)] pub unsafe fn TexCoord4s(&self, s: GLshort, t: GLshort, r: GLshort, q: GLshort) { (transmute::<_, extern "C" fn(s: GLshort, t: GLshort, r: GLshort, q: GLshort)>(self.procs[498]))(s, t, r, q) }
    #[inline(always)] pub unsafe fn TexCoord4sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[499]))(v) }
    #[inline(always)] pub unsafe fn TexCoordPointer(&self, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[500]))(size, r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn TexEnvf(&self, target: GLenum, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, param: GLfloat)>(self.procs[501]))(target, pname, param) }
    #[inline(always)] pub unsafe fn TexEnvfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfloat)>(self.procs[502]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexEnvi(&self, target: GLenum, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, param: GLint)>(self.procs[503]))(target, pname, param) }
    #[inline(always)] pub unsafe fn TexEnviv(&self, target: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint)>(self.procs[504]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexGend(&self, coord: GLenum, pname: GLenum, param: GLdouble) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, param: GLdouble)>(self.procs[505]))(coord, pname, param) }
    #[inline(always)] pub unsafe fn TexGendv(&self, coord: GLenum, pname: GLenum, params: *const GLdouble) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *const GLdouble)>(self.procs[506]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn TexGenf(&self, coord: GLenum, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, param: GLfloat)>(self.procs[507]))(coord, pname, param) }
    #[inline(always)] pub unsafe fn TexGenfv(&self, coord: GLenum, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *const GLfloat)>(self.procs[508]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn TexGeni(&self, coord: GLenum, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, param: GLint)>(self.procs[509]))(coord, pname, param) }
    #[inline(always)] pub unsafe fn TexGeniv(&self, coord: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(coord: GLenum, pname: GLenum, params: *const GLint)>(self.procs[510]))(coord, pname, params) }
    #[inline(always)] pub unsafe fn TexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[511]))(target, level, internalformat, width, border, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[512]))(target, level, internalformat, width, height, border, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { (transmute::<_, extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>(self.procs[513]))(target, samples, internalformat, width, height, fixedsamplelocations) }
    #[inline(always)] pub unsafe fn TexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[514]))(target, level, internalformat, width, height, depth, border, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { (transmute::<_, extern "C" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>(self.procs[515]))(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
    #[inline(always)] pub unsafe fn TexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint)>(self.procs[516]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLuint)>(self.procs[517]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, param: GLfloat)>(self.procs[518]))(target, pname, param) }
    #[inline(always)] pub unsafe fn TexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLfloat)>(self.procs[519]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, param: GLint)>(self.procs[520]))(target, pname, param) }
    #[inline(always)] pub unsafe fn TexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) { (transmute::<_, extern "C" fn(target: GLenum, pname: GLenum, params: *const GLint)>(self.procs[521]))(target, pname, params) }
    #[inline(always)] pub unsafe fn TexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[522]))(target, level, xoffset, width, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[523]))(target, level, xoffset, yoffset, width, height, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void) { (transmute::<_, extern "C" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, r#type: GLenum, pixels: *const libc::c_void)>(self.procs[524]))(target, level, xoffset, yoffset, zoffset, width, height, depth, format, r#type, pixels) }
    #[inline(always)] pub unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *mut*const GLchar, bufferMode: GLenum) { (transmute::<_, extern "C" fn(program: GLuint, count: GLsizei, varyings: *mut*const GLchar, bufferMode: GLenum)>(self.procs[525]))(program, count, varyings, bufferMode) }
    #[inline(always)] pub unsafe fn Translated(&self, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[526]))(x, y, z) }
    #[inline(always)] pub unsafe fn Translatef(&self, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[527]))(x, y, z) }
    #[inline(always)] pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) { (transmute::<_, extern "C" fn(location: GLint, v0: GLfloat)>(self.procs[528]))(location, v0) }
    #[inline(always)] pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat)>(self.procs[529]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLint)>(self.procs[530]))(location, v0) }
    #[inline(always)] pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLint)>(self.procs[531]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLuint)>(self.procs[532]))(location, v0) }
    #[inline(always)] pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint)>(self.procs[533]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) { (transmute::<_, extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat)>(self.procs[534]))(location, v0, v1) }
    #[inline(always)] pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat)>(self.procs[535]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLint, v1: GLint)>(self.procs[536]))(location, v0, v1) }
    #[inline(always)] pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLint)>(self.procs[537]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLuint, v1: GLuint)>(self.procs[538]))(location, v0, v1) }
    #[inline(always)] pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint)>(self.procs[539]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { (transmute::<_, extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat)>(self.procs[540]))(location, v0, v1, v2) }
    #[inline(always)] pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat)>(self.procs[541]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint)>(self.procs[542]))(location, v0, v1, v2) }
    #[inline(always)] pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLint)>(self.procs[543]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint)>(self.procs[544]))(location, v0, v1, v2) }
    #[inline(always)] pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint)>(self.procs[545]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { (transmute::<_, extern "C" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat)>(self.procs[546]))(location, v0, v1, v2, v3) }
    #[inline(always)] pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLfloat)>(self.procs[547]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint)>(self.procs[548]))(location, v0, v1, v2, v3) }
    #[inline(always)] pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLint)>(self.procs[549]))(location, count, value) }
    #[inline(always)] pub unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { (transmute::<_, extern "C" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint)>(self.procs[550]))(location, v0, v1, v2, v3) }
    #[inline(always)] pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, value: *const GLuint)>(self.procs[551]))(location, count, value) }
    #[inline(always)] pub unsafe fn UniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) { (transmute::<_, extern "C" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint)>(self.procs[552]))(program, uniformBlockIndex, uniformBlockBinding) }
    #[inline(always)] pub unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[553]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[554]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[555]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[556]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[557]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[558]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[559]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[560]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { (transmute::<_, extern "C" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat)>(self.procs[561]))(location, count, transpose, value) }
    #[inline(always)] pub unsafe fn UnmapBuffer(&self, target: GLenum) -> GLboolean { (transmute::<_, extern "C" fn(target: GLenum) -> GLboolean>(self.procs[562]))(target) }
    #[inline(always)] pub unsafe fn UseProgram(&self, program: GLuint) { (transmute::<_, extern "C" fn(program: GLuint)>(self.procs[563]))(program) }
    #[inline(always)] pub unsafe fn ValidateProgram(&self, program: GLuint) { (transmute::<_, extern "C" fn(program: GLuint)>(self.procs[564]))(program) }
    #[inline(always)] pub unsafe fn Vertex2d(&self, x: GLdouble, y: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble)>(self.procs[565]))(x, y) }
    #[inline(always)] pub unsafe fn Vertex2dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[566]))(v) }
    #[inline(always)] pub unsafe fn Vertex2f(&self, x: GLfloat, y: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat)>(self.procs[567]))(x, y) }
    #[inline(always)] pub unsafe fn Vertex2fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[568]))(v) }
    #[inline(always)] pub unsafe fn Vertex2i(&self, x: GLint, y: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint)>(self.procs[569]))(x, y) }
    #[inline(always)] pub unsafe fn Vertex2iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[570]))(v) }
    #[inline(always)] pub unsafe fn Vertex2s(&self, x: GLshort, y: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort)>(self.procs[571]))(x, y) }
    #[inline(always)] pub unsafe fn Vertex2sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[572]))(v) }
    #[inline(always)] pub unsafe fn Vertex3d(&self, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[573]))(x, y, z) }
    #[inline(always)] pub unsafe fn Vertex3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[574]))(v) }
    #[inline(always)] pub unsafe fn Vertex3f(&self, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[575]))(x, y, z) }
    #[inline(always)] pub unsafe fn Vertex3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[576]))(v) }
    #[inline(always)] pub unsafe fn Vertex3i(&self, x: GLint, y: GLint, z: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, z: GLint)>(self.procs[577]))(x, y, z) }
    #[inline(always)] pub unsafe fn Vertex3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[578]))(v) }
    #[inline(always)] pub unsafe fn Vertex3s(&self, x: GLshort, y: GLshort, z: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort, z: GLshort)>(self.procs[579]))(x, y, z) }
    #[inline(always)] pub unsafe fn Vertex3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[580]))(v) }
    #[inline(always)] pub unsafe fn Vertex4d(&self, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>(self.procs[581]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn Vertex4dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[582]))(v) }
    #[inline(always)] pub unsafe fn Vertex4f(&self, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat)>(self.procs[583]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn Vertex4fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[584]))(v) }
    #[inline(always)] pub unsafe fn Vertex4i(&self, x: GLint, y: GLint, z: GLint, w: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, z: GLint, w: GLint)>(self.procs[585]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn Vertex4iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[586]))(v) }
    #[inline(always)] pub unsafe fn Vertex4s(&self, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort, z: GLshort, w: GLshort)>(self.procs[587]))(x, y, z, w) }
    #[inline(always)] pub unsafe fn Vertex4sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[588]))(v) }
    #[inline(always)] pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, x: GLdouble)>(self.procs[589]))(index, x) }
    #[inline(always)] pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLdouble)>(self.procs[590]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, x: GLfloat)>(self.procs[591]))(index, x) }
    #[inline(always)] pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLfloat)>(self.procs[592]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) { (transmute::<_, extern "C" fn(index: GLuint, x: GLshort)>(self.procs[593]))(index, x) }
    #[inline(always)] pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[594]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble)>(self.procs[595]))(index, x, y) }
    #[inline(always)] pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLdouble)>(self.procs[596]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat)>(self.procs[597]))(index, x, y) }
    #[inline(always)] pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLfloat)>(self.procs[598]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) { (transmute::<_, extern "C" fn(index: GLuint, x: GLshort, y: GLshort)>(self.procs[599]))(index, x, y) }
    #[inline(always)] pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[600]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[601]))(index, x, y, z) }
    #[inline(always)] pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLdouble)>(self.procs[602]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[603]))(index, x, y, z) }
    #[inline(always)] pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLfloat)>(self.procs[604]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) { (transmute::<_, extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort)>(self.procs[605]))(index, x, y, z) }
    #[inline(always)] pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[606]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLbyte)>(self.procs[607]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[608]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[609]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) { (transmute::<_, extern "C" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte)>(self.procs[610]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLubyte)>(self.procs[611]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[612]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLushort)>(self.procs[613]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLbyte)>(self.procs[614]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble)>(self.procs[615]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLdouble)>(self.procs[616]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat)>(self.procs[617]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLfloat)>(self.procs[618]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[619]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { (transmute::<_, extern "C" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort)>(self.procs[620]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[621]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLubyte)>(self.procs[622]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[623]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const GLushort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLushort)>(self.procs[624]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLint)>(self.procs[625]))(index, x) }
    #[inline(always)] pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[626]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLuint)>(self.procs[627]))(index, x) }
    #[inline(always)] pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[628]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLint, y: GLint)>(self.procs[629]))(index, x, y) }
    #[inline(always)] pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[630]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLuint, y: GLuint)>(self.procs[631]))(index, x, y) }
    #[inline(always)] pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[632]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint)>(self.procs[633]))(index, x, y, z) }
    #[inline(always)] pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[634]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint)>(self.procs[635]))(index, x, y, z) }
    #[inline(always)] pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[636]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLbyte)>(self.procs[637]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint)>(self.procs[638]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const GLint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLint)>(self.procs[639]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const GLshort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLshort)>(self.procs[640]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLubyte)>(self.procs[641]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) { (transmute::<_, extern "C" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint)>(self.procs[642]))(index, x, y, z, w) }
    #[inline(always)] pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLuint)>(self.procs[643]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const GLushort) { (transmute::<_, extern "C" fn(index: GLuint, v: *const GLushort)>(self.procs[644]))(index, v) }
    #[inline(always)] pub unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[645]))(index, size, r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(index: GLuint, size: GLint, r#type: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[646]))(index, size, r#type, normalized, stride, pointer) }
    #[inline(always)] pub unsafe fn VertexPointer(&self, size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void) { (transmute::<_, extern "C" fn(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const libc::c_void)>(self.procs[647]))(size, r#type, stride, pointer) }
    #[inline(always)] pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>(self.procs[648]))(x, y, width, height) }
    #[inline(always)] pub unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) { (transmute::<_, extern "C" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64)>(self.procs[649]))(sync, flags, timeout) }
    #[inline(always)] pub unsafe fn WindowPos2d(&self, x: GLdouble, y: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble)>(self.procs[650]))(x, y) }
    #[inline(always)] pub unsafe fn WindowPos2dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[651]))(v) }
    #[inline(always)] pub unsafe fn WindowPos2f(&self, x: GLfloat, y: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat)>(self.procs[652]))(x, y) }
    #[inline(always)] pub unsafe fn WindowPos2fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[653]))(v) }
    #[inline(always)] pub unsafe fn WindowPos2i(&self, x: GLint, y: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint)>(self.procs[654]))(x, y) }
    #[inline(always)] pub unsafe fn WindowPos2iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[655]))(v) }
    #[inline(always)] pub unsafe fn WindowPos2s(&self, x: GLshort, y: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort)>(self.procs[656]))(x, y) }
    #[inline(always)] pub unsafe fn WindowPos2sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[657]))(v) }
    #[inline(always)] pub unsafe fn WindowPos3d(&self, x: GLdouble, y: GLdouble, z: GLdouble) { (transmute::<_, extern "C" fn(x: GLdouble, y: GLdouble, z: GLdouble)>(self.procs[658]))(x, y, z) }
    #[inline(always)] pub unsafe fn WindowPos3dv(&self, v: *const GLdouble) { (transmute::<_, extern "C" fn(v: *const GLdouble)>(self.procs[659]))(v) }
    #[inline(always)] pub unsafe fn WindowPos3f(&self, x: GLfloat, y: GLfloat, z: GLfloat) { (transmute::<_, extern "C" fn(x: GLfloat, y: GLfloat, z: GLfloat)>(self.procs[660]))(x, y, z) }
    #[inline(always)] pub unsafe fn WindowPos3fv(&self, v: *const GLfloat) { (transmute::<_, extern "C" fn(v: *const GLfloat)>(self.procs[661]))(v) }
    #[inline(always)] pub unsafe fn WindowPos3i(&self, x: GLint, y: GLint, z: GLint) { (transmute::<_, extern "C" fn(x: GLint, y: GLint, z: GLint)>(self.procs[662]))(x, y, z) }
    #[inline(always)] pub unsafe fn WindowPos3iv(&self, v: *const GLint) { (transmute::<_, extern "C" fn(v: *const GLint)>(self.procs[663]))(v) }
    #[inline(always)] pub unsafe fn WindowPos3s(&self, x: GLshort, y: GLshort, z: GLshort) { (transmute::<_, extern "C" fn(x: GLshort, y: GLshort, z: GLshort)>(self.procs[664]))(x, y, z) }
    #[inline(always)] pub unsafe fn WindowPos3sv(&self, v: *const GLshort) { (transmute::<_, extern "C" fn(v: *const GLshort)>(self.procs[665]))(v) }
}

fn build_disabled_extension_list(disabled_extensions: &[u8])
            -> std::collections::HashSet<&[u8]> {
    disabled_extensions.split(|&x| {
        !((x >= b'0' && x <= b'9')
          || (x >= b'A' && x <= b'Z')
          || (x >= b'a' && x <= b'z')
          || (x == b'_'))
    }).filter_map(|x| {
        match x {
            b"" => None,
            x => Some(x)
        }
    }).collect()
}
