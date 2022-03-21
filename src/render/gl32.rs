use super::*;

use std::{
    collections::HashMap,
    mem::{size_of, transmute},
    ptr::{null, null_mut},
};

use anyhow::anyhow;
use log::{warn, debug, info};
use sdl2::{
    video::{GLContext, GLProfile, Window, WindowBuilder},
    VideoSubsystem,
};

mod binding;
use binding::*;

use super::glerr::ERROR_TABLE;

#[derive(Debug,PartialEq,Eq)]
enum LastBatchType {
    None, Model, Text
}

#[derive(Debug)] #[repr(C)]
struct ModelVert {
    x: f32, y: f32, c: u32,
}

struct CachedModel {
    vao: GLuint,
    num_elements: GLint,
}

struct ModelCache {
    models: HashMap<u32, CachedModel>,
}

impl ModelCache {
    fn new() -> ModelCache { ModelCache { models: HashMap::new() } }
    fn get_or_make_cached(&mut self, gl: &Procs, program_model: GLuint,
                          model: &Model) -> &CachedModel {
        self.models.entry(model.unique_id)
            .or_insert_with(|| {
                let verts: Vec<ModelVert> = model.points.iter()
                    .map(|p| {
                        ModelVert {
                            x: p.point.x,
                            y: p.point.y,
                            c: p.color_idx as u32,
                        }
                    }).collect();
                let mut vao = 0;
                let num_elements;
                // Unsafe justification: okay, look, we're just gonna be using
                // unsafe every time we talk to OpenGL. We aren't going to be
                // able to avoid that. Okay? Okay.
                unsafe {
                    gl.GenVertexArrays(1, &mut vao);
                    let mut vbos = [0; 2];
                    gl.GenBuffers(2, &mut vbos[0]);
                    let vb = vbos[0];
                    let ib = vbos[1];
                    gl.BindBuffer(GL_ARRAY_BUFFER, vb);
                    let size = size_of::<ModelVert>() * verts.len();
                    // something
                    gl.BufferData(GL_ARRAY_BUFFER,
                                  size as GLsizeiptr,
                                  transmute(&verts[0]),
                                  GL_STATIC_DRAW);
                    // Model: X___Y___C___
                    gl.BindVertexArray(vao);
                    setup_model_attribs(&gl, program_model);
                    gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ib);
                    let mut indices: Vec<u16>
                        = Vec::with_capacity(model.triangles.len()*3);
                    for &(a,b,c) in model.triangles.iter() {
                        indices.push(a);
                        indices.push(b);
                        indices.push(c);
                    }
                    gl.BufferData(GL_ELEMENT_ARRAY_BUFFER,
                                  (indices.len() * 2) as GLsizeiptr,
                                  transmute(&indices[0]),
                                  GL_STATIC_DRAW);
                    num_elements = indices.len() as GLint;
                }
                CachedModel {
                    vao, num_elements,
                }
            })
    }
}

/// Renders using OpenGL 3.0 Core.
struct OpenGL32 {
    window: Window,
    ctx: GLContext,
    gl: Procs,
    last_batch_type: LastBatchType,
    program_model: GLuint,
    program_text: GLuint,
    bound_texture: Option<GLuint>,
    force_multisample: bool,
    quad_vao: GLuint,
    quad_vb: GLuint,
    model_cache: ModelCache,
    loc_transform: GLint,
    loc_opacity: GLint,
    loc_colors: GLint,
}

/// Number of bytes to make our VBOs
const BUF_SIZE: usize = 262144; // 256KiB at a time
/// Number of indices to put into our IBO (is that a thing?)
const QUAD_IB_COUNT: usize = BUF_SIZE / (size_of::<MergedTextVert>()*4);
/// Number of bytes to make our IBO
const QUAD_IB_SIZE: usize = QUAD_IB_COUNT * 6 * 2;

const MULTISAMPLE_COVERAGE_TEST_BATCH: &[ModelVert] = &[
    ModelVert { x: -1.0, y: -1.0, c: 0 },
    ModelVert { x:  1.0, y: -1.0, c: 0 },
    ModelVert { x: -1.0, y:  1.0, c: 0 },
];

const MODEL_FRAGMENT_TEXT: &[u8] = include_bytes!("gl32/model.frag");
const TEXT_FRAGMENT_TEXT: &[u8] = include_bytes!("gl32/text.frag");
const MODEL_VERTEX_TEXT: &[u8] = include_bytes!("gl32/model.vert");
const TEXT_VERTEX_TEXT: &[u8] = include_bytes!("gl32/text.vert");

/// Check for OpenGL errors. If there were any, complain.
fn assertgl(gl: &Procs, wo: &str) -> anyhow::Result<()> {
    let mut errors = vec![];
    'outer: loop {
        // Unsafe justification: glGetError is safe to call.
        let e = unsafe { gl.GetError() };
        if e == 0 { break }
        for &(code, name) in ERROR_TABLE.iter() {
            if code == e { errors.push(name); continue 'outer; }
        }
        errors.push("unknown");
    }
    if errors.len() == 0 { Ok(()) }
    else {
        Err(anyhow!("OpenGL errors were detected while {}: {:?}", wo, errors))
    }
}

fn compile_shader(gl: &Procs, wat: &str, typ: GLenum, text: &[u8])
    -> anyhow::Result<GLuint> {
    // Unsafe justification: Lots of GL calls. We're careful about length but
    // we do assume the GL implementation doesn't lie to us in ways that are
    // TECHNICALLY allowed by the standard.
    unsafe {
        let shader = gl.CreateShader(typ);
        gl.ShaderSource(shader, 1,
                        (&mut [text.as_ptr() as *const GLchar]).as_mut_ptr(),
                        (&[text.len() as GLint]).as_ptr());
        gl.CompileShader(shader);
        let mut status: GLint = 0;
        gl.GetShaderiv(shader, GL_COMPILE_STATUS, &mut status);
        let mut log_length: GLint = 0;
        gl.GetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut log_length);
        let mut info_log: Vec<u8> = vec![];
        assert!(log_length >= 0);
        info_log.clear();
        info_log.resize(log_length as usize, 0);
        if log_length > 1 {
            gl.GetShaderInfoLog(shader, log_length, null_mut(),
                                transmute(&mut info_log[0]));
        }
        if status == 0 {
            Err(anyhow!("Unable to compile {}!\n{}", wat,
                      String::from_utf8_lossy(&info_log[..info_log.len()-1])))?
        }
        else if log_length > 1 {
            warn!("Diagnostics were generated while compiling {}:\n{}", wat,
                  String::from_utf8_lossy(&info_log[..info_log.len()-1]));
        }
        assertgl(gl, "compiling a shader").unwrap();
        Ok(shader)
    }
}

fn link_program(gl: &Procs, wat: &str, shaders: &[GLuint])
    -> anyhow::Result<GLuint> {
    // Unsafe justification: See `compile_shader`
    unsafe {
        let program = gl.CreateProgram();
        for &shader in shaders {
            gl.AttachShader(program, shader);
        }
        gl.LinkProgram(program);
        let mut status: GLint = 0;
        gl.GetProgramiv(program, GL_LINK_STATUS, &mut status);
        let mut log_length: GLint = 0;
        gl.GetProgramiv(program, GL_INFO_LOG_LENGTH, &mut log_length);
        let mut info_log: Vec<u8> = vec![];
        assert!(log_length >= 0);
        info_log.clear();
        info_log.resize(log_length as usize, 0);
        if log_length > 1 {
            gl.GetProgramInfoLog(program, log_length, null_mut(),
                                 transmute(&mut info_log[0]));
        }
        if status == 0 {
            Err(anyhow!("Unable to link {}!\n{}", wat,
                      String::from_utf8_lossy(&info_log[..info_log.len()-1])))?
        }
        else if log_length > 1 {
            warn!("Diagnostics were generated while linking {}:\n{}", wat,
                  String::from_utf8_lossy(&info_log[..info_log.len()-1]));
        }
        assertgl(gl, "linking a shader program").unwrap();
        Ok(program)
    }
}

fn setup_attribs(gl: &Procs, program: GLuint, program_name: &str,
                 attribs: &[(&[u8], fn(&Procs, GLuint) -> GLvoid)]) {
    for &(name, f) in attribs.iter() {
        debug_assert!(name.ends_with(&[0]));
        unsafe {
            let loc = gl.GetAttribLocation(program, transmute(name.as_ptr()));
            if loc < 0 {
                warn!("couldn't find expected shader attribute {:?} \
                       in the {:?} program",
                      String::from_utf8_lossy(&name[..name.len()-1]),
                      program_name);
            }
            else {
                let loc = loc as GLuint;
                gl.EnableVertexAttribArray(loc);
                f(gl, loc);
            }
        }
    }
}

fn setup_model_attribs(gl: &Procs, program: GLuint) {
    setup_attribs(&gl, program, "model", &[
        (b"pos\0", |gl, loc| unsafe {
         gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 12,
                                transmute(0usize))
        }),
        (b"color_index\0", |gl, loc| unsafe {
         gl.VertexAttribIPointer(loc, 1, GL_UNSIGNED_INT, 12,
                                 transmute(8usize))
        }),
    ]);
}

fn map_buffer(gl: &Procs, vbo: GLuint) -> &mut [u8] {
    // Unsafe justification: holy cow mmap. also, our caller is responsible for
    // fencing this access into safety, not us!
    unsafe {
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
        let ptr = gl.MapBufferRange(GL_ARRAY_BUFFER, 0, BUF_SIZE as GLsizeiptr,
                                    GL_MAP_WRITE_BIT
                                    | GL_MAP_UNSYNCHRONIZED_BIT
                                    | GL_MAP_INVALIDATE_BUFFER_BIT
                                    | GL_MAP_FLUSH_EXPLICIT_BIT);
        if ptr.is_null() {
            assertgl(gl, "mapping a buffer").unwrap()
        }
        std::slice::from_raw_parts_mut(transmute(ptr), BUF_SIZE)
    }
}

/// Set up an OpenGL 3.2 rendering context.
pub(crate) fn create<F>(video: &VideoSubsystem, builder_maker: &mut F)
    -> anyhow::Result<Box<dyn Renderer>>
where F: FnMut() -> WindowBuilder
{
    let mut window = None;
    for &samples in &[16, 8, 4, 2, 0] {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        if samples > 1 {
            gl_attr.set_multisample_buffers(1);
            gl_attr.set_multisample_samples(samples);
        }
        else {
            gl_attr.set_multisample_buffers(0);
            gl_attr.set_multisample_samples(1);
        }
        gl_attr.set_depth_size(0);
        gl_attr.set_framebuffer_srgb_compatible(true);
        let res = builder_maker().opengl().allow_highdpi().build();
        match res {
            Ok(x) => {
                debug!("Created window with {} multisample samples.", samples);
                window = Some(x);
                break;
            }
            Err(x) => {
                debug!("Unable to create window with {} multisample samples: \
                        {}", samples, x);
                if samples == 1 {
                    return
                        Err(anyhow!("Unable to create window for OpenGL 3.0 \
                                     context: {}", x))?;
                }
            },
        }
    }
    let window = window.unwrap();
    let ctx = window.gl_create_context()
        .map_err(|x|anyhow!("Unable to create OpenGL 3.0 context: {}",x))?;
    let gl = Procs::new(|proc| {
        // Our SDL2 binding provides a `gl_get_proc_address` binding, but it
        // only takes &str and adds a null terminator to it before calling.
        // rglgen bindings already contain the null terminator. So, we need to
        // call it ourselves.
        //
        // Unsafe justification: the input is known to be a static,
        // null-terminated string.
        let ret = unsafe {
            sdl2_sys::SDL_GL_GetProcAddress(transmute(proc.as_ptr()))
        };
        if ret.is_null() {
            Err(anyhow!("Unable to find the procedure named {}: {}",
                        String::from_utf8_lossy(&proc[..proc.len()-1]),
                        sdl2::get_error()))
        }
        else {
            // Unsafe justification: a non-null return address is a valid
            // OpenGL procedure entry point.
            Ok(unsafe{transmute(ret)})
        }
    })?;
    // Unsafe justification: Initial state for OpenGL context. All pointer
    // operations are statically-bounded, and an error check is performed
    // afterwards.
    let (program_model, program_text, force_multisample, quad_ib,
         quad_vb, loc_transform, loc_opacity, loc_colors);
    let mut quad_vao = 0;
    unsafe {
        // Compile and link the shaders... oh boy.
        let fshader_model = compile_shader(&gl, "the model fragment shader",
                                           GL_FRAGMENT_SHADER,
                                           MODEL_FRAGMENT_TEXT)?;
        let fshader_text = compile_shader(&gl, "the text fragment shader",
                                          GL_FRAGMENT_SHADER,
                                          TEXT_FRAGMENT_TEXT)?;
        let vshader_model = compile_shader(&gl, "the model vertex shader",
                                           GL_VERTEX_SHADER,
                                           MODEL_VERTEX_TEXT)?;
        let vshader_text = compile_shader(&gl, "the text vertex shader",
                                          GL_VERTEX_SHADER,
                                          TEXT_VERTEX_TEXT)?;
        program_model = link_program(&gl, "the model shader program",
                                     &[vshader_model, fshader_model])?;
        program_text = link_program(&gl, "the text shader program",
                                    &[vshader_text, fshader_text])?;
        gl.DeleteShader(vshader_model);
        gl.DeleteShader(vshader_text);
        gl.DeleteShader(fshader_model);
        gl.DeleteShader(fshader_text);
        // Generate buffers to render text "quads" into
        gl.GenVertexArrays(1, &mut quad_vao);
        gl.BindVertexArray(quad_vao);
        let mut quad_vbos = [0; 2];
        gl.GenBuffers(2, &mut quad_vbos[0]);
        quad_ib = quad_vbos[0];
        quad_vb = quad_vbos[1];
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, quad_ib);
        let mut quad_ib_buf = Vec::with_capacity(QUAD_IB_COUNT * 6);
        assert!(QUAD_IB_COUNT < 65536 / 4);
        for n in 0 .. QUAD_IB_COUNT as u16 {
            quad_ib_buf.push(n*4);
            quad_ib_buf.push(n*4+1);
            quad_ib_buf.push(n*4+2);
            quad_ib_buf.push(n*4+2);
            quad_ib_buf.push(n*4+3);
            quad_ib_buf.push(n*4);
        }
        gl.BufferData(GL_ELEMENT_ARRAY_BUFFER, QUAD_IB_SIZE as GLsizeiptr,
                      transmute(&quad_ib_buf[0]),
                      GL_STATIC_DRAW);
        gl.BindBuffer(GL_ARRAY_BUFFER, quad_vb);
        gl.BufferData(GL_ARRAY_BUFFER, BUF_SIZE as GLsizeiptr, null(),
                      GL_DYNAMIC_DRAW);
        // Text: X_Y_U_V_R_G_B_A_R_G_B_A_
        setup_attribs(&gl, program_text, "text", &[
            (b"pos\0", |gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_HALF_FLOAT, 0, 24,
                                    transmute(0usize))),
            (b"uv_in\0", |gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_HALF_FLOAT, 0, 24,
                                    transmute(4usize))),
            (b"vert_fillcolor\0", |gl, loc|
             gl.VertexAttribPointer(loc, 4, GL_HALF_FLOAT, 0, 24,
                                    transmute(8usize))),
            (b"vert_strokecolor\0", |gl, loc|
             gl.VertexAttribPointer(loc, 4, GL_HALF_FLOAT, 0, 24,
                                    transmute(16usize))),
        ]);
        // Do linear-to-sRGB compression before writing to the framebuffer and
        // decompression after reading (for blending)
        gl.Enable(GL_FRAMEBUFFER_SRGB);
        // Set up blending for premultiplied alpha
        gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        // We're gonna be uploading a lot of unaligned pixel data, yuck.
        gl.PixelStorei(GL_UNPACK_ALIGNMENT, 1);
        // Oh, and let's set the uniforms and/or find them!
        gl.UseProgram(program_text);
        let loc = gl.GetUniformLocation(program_text,
                                        transmute(b"atlas\0"));
        if loc >= 0 {
            gl.Uniform1i(loc, 0); // texture unit 0
        }
        gl.UseProgram(program_model);
        loc_transform = gl.GetUniformLocation(program_model,
                                              transmute(b"transform\0"));
        loc_colors = gl.GetUniformLocation(program_model,
                                           transmute(b"colors\0"));
        loc_opacity = gl.GetUniformLocation(program_model,
                                            transmute(b"opacity\0"));
        // Set up initial values for the uniforms... which, COINCIDENTALLY, are
        // also what we want for the multisample test
        if loc_transform >= 0 {
            gl.UniformMatrix3fv(loc_transform, 1, 0,
                                (&[1.0, 0.0, 0.0,
                                   0.0, 1.0, 0.0,
                                   0.0, 0.0, 1.0]).as_ptr());
        }
        if loc_opacity >= 0 {
            gl.Uniform1f(loc_opacity, 1.0);
        }
        if loc_colors >= 0 {
            gl.Uniform4fv(loc_colors, 8,
                          [
                              1.0, 0.0, 1.0, 1.0,
                              1.0, 0.0, 0.0, 1.0,
                              0.0, 1.0, 0.0, 1.0,
                              1.0, 1.0, 0.0, 1.0,
                              0.0, 0.0, 1.0, 1.0,
                              1.0, 0.0, 1.0, 1.0,
                              0.0, 1.0, 1.0, 1.0,
                              1.0, 1.0, 1.0, 1.0,
                          ].as_ptr());
        }
        assertgl(&gl, "initializing the context")?;
        // Before we're done, check for Mesa bug #4613.
        // We have to do this check, and otherwise assume multisampling is in
        // use, in case somebody uses their vendor's special control panel to
        // force multisampling to be used.
        gl.ClearColor(0.0, 1.0, 0.0, 0.0);
        gl.Clear(GL_COLOR_BUFFER_BIT);
        gl.Disable(GL_MULTISAMPLE);
        let mut testvao = 0;
        gl.GenVertexArrays(1, &mut testvao);
        gl.BindVertexArray(testvao);
        let mut testvb = 0;
        gl.GenBuffers(1, &mut testvb);
        setup_model_attribs(&gl, program_model);
        gl.BufferData(GL_ARRAY_BUFFER, 12 * 4,
                      transmute(&MULTISAMPLE_COVERAGE_TEST_BATCH[0]),
                      GL_STREAM_DRAW);
        gl.DrawArrays(GL_TRIANGLES, 0, 3);
        gl.Enable(GL_MULTISAMPLE);
        let mut testfb = 0;
        let mut testtex = 0;
        gl.GenFramebuffers(1, &mut testfb);
        gl.GenTextures(1, &mut testtex);
        gl.BindTexture(GL_TEXTURE_2D, testtex);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                         GL_NEAREST as GLint);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                         GL_NEAREST as GLint);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, 1, 1, 0, GL_RGBA,
                      GL_UNSIGNED_BYTE, transmute(b"aaaa"));
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, 0);
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, testfb);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, testtex, 0);
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
            != GL_FRAMEBUFFER_COMPLETE {
                warn!("Couldn't set up framebuffer for multisample coverage \
                       check. Assuming the bug is NOT PRESENT. Sorry if your \
                       text all comes out dim.");
                force_multisample = false;
            }
        else {
            gl.BlitFramebuffer(0, 0, 1, 1, 0, 0, 1, 1, GL_COLOR_BUFFER_BIT,
                               GL_NEAREST);
            let mut buf = [0u8; 3];
            gl.BindFramebuffer(GL_READ_FRAMEBUFFER, testfb);
            gl.ReadPixels(0, 0, 1, 1, GL_RGB, GL_UNSIGNED_BYTE,
                          transmute(&mut buf[0]));
            gl.BindFramebuffer(GL_READ_FRAMEBUFFER, 0);
            gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
            force_multisample = if buf[0]==255 && buf[1]==0 && buf[2]==255 {
                info!("Checked for multisample coverage quirk, didn't see it. \
                       Great!");
                false
            }
            else {
                warn!("Your driver doesn't handle GL_MULTISAMPLE being \
                       disabled correctly. We have no choice but to leave it \
                       enabled. Performance of text rendering will suffer.");
                true
            };
        }
        gl.DeleteFramebuffers(1, &testfb);
        gl.DeleteTextures(1, &testtex);
        gl.DeleteBuffers(1, &testvb);
        gl.DeleteVertexArrays(1, &testvao);
        assertgl(&gl, "checking for the multisample bug")?;
    }
    Ok(Box::new(OpenGL32 {
        window, ctx, gl, last_batch_type: LastBatchType::None,
        program_model, program_text, bound_texture: None,
        force_multisample, quad_vao, quad_vb, model_cache: ModelCache::new(),
        loc_transform, loc_colors, loc_opacity,
    }))
}

impl Renderer for OpenGL32 {
    fn begin_rendering(&mut self) -> anyhow::Result<()> {
        self.window.gl_make_current(&self.ctx)
            .map_err(|x| anyhow!("OpenGL context lost! {}", x))?;
        assertgl(&self.gl, "starting rendering (this means the error probably \
                            occurred while rendering the last frame, but \
                            wasn't caught when it arose)").unwrap();
        Ok(())
    }
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let gl = &self.gl;
        // Unsafe justification: Only primitive data are sent, and these
        // functions cannot error.
        unsafe {
            gl.ClearColor(r, g, b, a);
            gl.Clear(GL_COLOR_BUFFER_BIT);
        } assertgl(gl, "clearing the screen").unwrap();
    }
    fn present(&mut self) {
        assertgl(&self.gl, "finishing rendering (this means the error \
                            probably occurred while rendering this frame, \
                            but wasn't caught when it arose)").unwrap();
        self.window.gl_swap_window();
    }
    fn get_size(&self) -> (u32, u32) {
        self.window.size()
    }
    fn enable_blend(&mut self) {
        let gl = &self.gl;
        // Unsafe justification: Trivially safe.
        unsafe { gl.Enable(GL_BLEND); }
    }
    fn disable_blend(&mut self) {
        let gl = &self.gl;
        // Unsafe justification: Trivially safe.
        unsafe { gl.Disable(GL_BLEND); }
    }
    fn get_text_atlas_size(&mut self) -> u32 {
        let gl = &self.gl;
        // Unsafe justification: Trivially safe. The only pointer passed is
        // null, and it has no side effect since we're using a proxy texture.
        unsafe {
            let mut max_size = 0;
            gl.GetIntegerv(GL_MAX_TEXTURE_SIZE, &mut max_size);
            // standard mandates 1024 but we can cope with less
            assert!(max_size >= 256);
            // round max_size up to the nearest >= power of two
            max_size = 0x40000000
                >> i32::leading_zeros(max_size).saturating_sub(2);
            // if it's small, double it (the spec says this should be rejected,
            // but, hey, maybe we'll get "lucky")
            if max_size < 32768 {
                max_size = max_size * 2;
            }
            // okay, now repeatedly try the proxy texture thing
            while max_size >= 256 {
                gl.TexImage2D(GL_PROXY_TEXTURE_2D, 0, GL_RGB as GLint,
                              max_size, max_size, 0, GL_RGB, GL_UNSIGNED_BYTE,
                              null());
                let mut got_width = 0;
                gl.GetTexLevelParameteriv(GL_PROXY_TEXTURE_2D, 0,
                                          GL_TEXTURE_WIDTH, &mut got_width);
                // (discard errors)
                while gl.GetError() != GL_NO_ERROR {}
                if got_width == max_size {
                    return max_size as u32;
                }
                max_size /= 2;
            }
        }
        panic!("Your OpenGL implementation appears to have an absurdly small \
                maximum texture size!");
    }
    fn new_text_atlas(&mut self, atlas_size: u32) -> anyhow::Result<u32> {
        let gl = &self.gl;
        let mut texture = 0;
        // Unsafe justification: Simple OpenGL calls. The only pointer passed
        // is to glGenTextures, and is a stack-allocated value.
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(GL_TEXTURE_2D, texture);
            gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGB8 as GLint,
                          atlas_size as GLsizei, atlas_size as GLsizei, 0,
                          GL_RGB, GL_UNSIGNED_BYTE, null());
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                             GL_LINEAR as GLint);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                             GL_LINEAR as GLint);
            self.bound_texture = Some(texture);
        } assertgl(gl, "making a new atlas texture").unwrap();
        Ok(texture as u32)
    }
    fn new_text_glyph(&mut self, atlas: u32,
                      glyph_x: u32, glyph_y: u32, glyph_w: u32, glyph_h: u32,
                      pixels: &[u8]) -> anyhow::Result<()> {
        let gl = &self.gl;
        let atlas = atlas as GLuint;
        if Some(atlas) != self.bound_texture {
            // Unsafe justification: Binding a texture. We assume that `atlas`
            // is a texture ID returned from a previous call of
            // `new_text_atlas`. If it isn't, an OpenGL error will be raised.
            unsafe { gl.BindTexture(GL_TEXTURE_2D, atlas) };
            self.bound_texture = Some(atlas);
        }
        // Unsafe justification: The only pointer we're passing is to a slice
        // whose bounds are predictable. If `GL_UNPACK_ALIGNMENT` gets set to a
        // value other than 1, this will be unsound!
        debug_assert!(pixels.len() >= glyph_w as usize * glyph_h as usize * 3);
        unsafe {
            gl.TexSubImage2D(GL_TEXTURE_2D, 0,
                             glyph_x as GLint, glyph_y as GLint,
                             glyph_w as GLsizei, glyph_h as GLsizei,
                             GL_RGB, GL_UNSIGNED_BYTE, transmute(&pixels[0]));
        } assertgl(gl, "uploading a glyph image").unwrap();
        Ok(())
    }
    fn new_text_batch(&mut self) -> TextBatch {
        TextBatch::Merged(MergedTextBatch::new())
    }
    fn render_text_batch(&mut self, atlases: &[u32],
                         batch: TextBatch) {
        // TODO: more efficient text rendering. We still can't know ahead of
        // time all the separate VBOs we might put it into, but what we CAN do
        // is map a VBO, copy several accumulated atlases' worth of text verts
        // into it, and render each subset that's in the buffer. We'll need a
        // new API design for that. For now, let's just make things keep work-
        // ing.
        let batch = match batch {
            TextBatch::Merged(x) => x,
            TextBatch::Split(_) => panic!("We can't render split text \
                                           batches, and we don't hand them \
                                           out"),
        };
        // Unsafe justification: Passing in data. Every time we have data to
        // pass in, we are passing a slice. We provide the slice pointer and
        // its size-converted length at every turn. We check errors at
        // completion.
        // addendum: we assume that `atlas` is a valid texture ID, but if it
        // isn't, an OpenGL error will be raised.
        unsafe {
            let gl = &self.gl;
            if self.last_batch_type != LastBatchType::Text {
                gl.BindVertexArray(self.quad_vao);
                gl.BindBuffer(GL_ARRAY_BUFFER, self.quad_vb);
                gl.UseProgram(self.program_text);
                if !self.force_multisample {
                    gl.Disable(GL_MULTISAMPLE);
                }
                self.last_batch_type = LastBatchType::Text;
                assertgl(gl, "switching to the text shader").unwrap();
            }
            for (_, verts) in atlases.iter().zip(batch.verts.iter()) {
                for seg in verts.chunks(QUAD_IB_COUNT*4) {
                    let chunksize
                        = (seg.len() * size_of::<MergedTextVert>())
                        as GLsizeiptr;
                    gl.BufferData(GL_ARRAY_BUFFER, BUF_SIZE as GLsizeiptr,
                                  null(), GL_STREAM_DRAW);
                    gl.BufferSubData(GL_ARRAY_BUFFER, 0,
                                     chunksize, transmute(seg.as_ptr()));
                    gl.DrawElements(GL_TRIANGLES,
                                    (seg.len() / 4 * 6) as GLint,
                                    GL_UNSIGNED_SHORT, null());
                }
            }
        } assertgl(&self.gl, "rendering a text batch").unwrap();
    }
    fn render_model(&mut self, model: &Model,
                    transform: &Transform, color_overrides: &[Color],
                    opacity: f32) {
        let cached = self.model_cache.get_or_make_cached(&self.gl,
                                                         self.program_model,
                                                         model);
        let gl = &self.gl;
        unsafe {
            if self.last_batch_type != LastBatchType::Model {
                gl.UseProgram(self.program_model);
                if !self.force_multisample {
                    gl.Enable(GL_MULTISAMPLE);
                }
                self.last_batch_type = LastBatchType::Model;
                assertgl(gl, "switching to the model shader").unwrap();
            }
            gl.BindVertexArray(cached.vao);
            if self.loc_transform >= 0 {
                gl.UniformMatrix3fv(self.loc_transform, 1, 0,
                                    transform.into_inner().as_slice()
                                    .as_ptr());
            }
            if self.loc_opacity >= 0 {
                gl.Uniform1f(self.loc_opacity, opacity);
            }
            if self.loc_colors >= 0 {
                for n in 0 .. model.colors.len() {
                    let color = color_overrides.get(n)
                        .unwrap_or(&model.colors[n]);
                    gl.Uniform4f(self.loc_colors + (n as GLint),
                                 color.r.to_f32(), color.g.to_f32(),
                                 color.b.to_f32(), color.a.to_f32());
                }
            }
            gl.DrawElements(GL_TRIANGLES, cached.num_elements,
                            GL_UNSIGNED_SHORT, null());
        }
    }
}


