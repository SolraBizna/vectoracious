use super::*;

use core::slice;
use std::{
    collections::HashMap,
    fmt::Write,
    io::Write as _, // we just want its trait methods
    mem::{size_of, transmute},
    ptr::{null, null_mut},
};

use anyhow::anyhow;
use log::{warn, debug, info, error, trace};
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
    program_blit: GLuint,
    program_bwm: GLuint,
    program_bloomx: GLuint,
    program_bloomy: GLuint,
    bound_texture: Option<GLuint>,
    force_multisample: bool,
    quad_vao: GLuint,
    quad_vb: GLuint,
    bloom_vao: GLuint,
    bloom_vb: GLuint,
    model_cache: ModelCache,
    loc_transform: GLint,
    loc_opacity: GLint,
    loc_colors: GLint,
    max_multisample_power: u32,
    is_blending: bool,
    /// Width of world framebuffer
    world_w: u32,
    /// Height of world framebuffer
    world_h: u32,
    /// Number of multisamples in world framebuffer
    world_samples: u32,
    /// Number of X oversamples we'll use for the world framebuffer
    world_super_x: u32,
    /// Number of Y oversamples we'll use for the world framebuffer
    world_super_y: u32,
    /// Texture created for the world framebuffer
    world_tex: GLuint,
    /// Framebuffer created for the world
    world_fb: GLuint,
    /// Width of bloom framebuffer
    bloom_w: u32,
    /// Height of bloom framebuffer
    bloom_h: u32,
    /// Texture created for bloom framebuffers
    bloom_tex: [GLuint; 2],
    /// Bloom framebuffers
    bloom_fb: [GLuint; 2],
    /// Width of UI framebuffer
    ui_w: u32,
    /// Height of UI framebuffer
    ui_h: u32,
    /// Number of multisamples in UI framebuffer
    ui_samples: u32,
    /// Texture created for the UI framebuffer
    ui_tex: GLuint,
    /// Framebuffer created for the UI. If `ui_samples` is 1, don't bind this,
    /// bind the default framebuffer instead!
    ui_fb: GLuint,
    /// The texture created to store samples of the Gaussian function, for
    /// bloom
    gauss_tex: [GLuint; 2],
    /// The standard deviation radius currently stored in the Gauss texture
    gauss_radius: [f32; 2],
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

const MODEL_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/model.frag");
const MODEL_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/model.vert");
const TEXT_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/text.frag");
const TEXT_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/text.vert");
const BLIT_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/blit.frag");
const BLIT_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/blit.vert");
const BWM_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/bwm.frag");
// bwm program uses the blit vertex shader
const BLOOM_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/bloom.frag");
// bloom program uses the blit vertex shader
const BLOOM_X_SUPPLEMENT: &[u8] = br#"
#define BLOOM_HORIZ 1
"#;
const BLOOM_Y_SUPPLEMENT: &[u8] = br#"
#define BLOOM_VERT 1
"#;

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

extern "C" fn debug_callback(source: GLenum, typ: GLenum, id: GLuint,
                             severity: GLenum, length: GLsizei,
                             message: *const GLchar, _: *const GLvoid) {
    let source = match source {
        GL_DEBUG_SOURCE_API_ARB => "OpenGL".to_owned(),
        GL_DEBUG_SOURCE_APPLICATION_ARB => "the user(?!)".to_owned(),
        GL_DEBUG_SOURCE_SHADER_COMPILER_ARB =>"the shader compiler".to_owned(),
        GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB => "the window system".to_owned(),
        GL_DEBUG_SOURCE_THIRD_PARTY_ARB => "a third party".to_owned(),
        GL_DEBUG_SOURCE_OTHER_ARB => "\"other\"(!?)".to_owned(),
        x => format!("(unknown 0x{:x})", x),
    };
    let typ = match typ {
        GL_DEBUG_TYPE_ERROR_ARB => "an error".to_owned(),
        GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB => "deprecated behavior".to_owned(),
        GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB => "undefined behavior".to_owned(),
        GL_DEBUG_TYPE_PORTABILITY_ARB => "unportable usage".to_owned(),
        GL_DEBUG_TYPE_PERFORMANCE_ARB => "poorly-performing usage".to_owned(),
        GL_DEBUG_TYPE_OTHER_ARB => "something spooky".to_owned(),
        x => format!("(unknown 0x{:x})", x),
    };
    let message = unsafe {
        slice::from_raw_parts(message as *const u8, length as usize)
    };
    let message = String::from_utf8_lossy(message);
    let message = message.strip_suffix("\n").unwrap_or(&message);
    match severity {
        GL_DEBUG_SEVERITY_HIGH_ARB => {
            error!("{} detected {}: [HIGH, {}] {}",
                   source, typ, id, message);
        },
        GL_DEBUG_SEVERITY_MEDIUM_ARB => {
            warn!("{} detected {}: [MEDIUM, {}] {}",
                  source, typ, id, message);
        },
        _ => {
            warn!("{} detected {}: [LOW, {}] {}",
                  source, typ, id, message);
        },
    }
    if std::env::var_os("RUST_BACKTRACE").is_some() {
        let mut backtrace = String::new();
        backtrace::trace(|frame| {
            backtrace::resolve_frame(frame, |res| {
                // @%!@#Q$&%
                let symbol = match res.name() {
                    Some(x) => format!("{}", x),
                    None => "???".to_owned(),
                };
                if symbol.starts_with("backtrace:") { return }
                else if symbol.contains("::debug_callback") { return }
                let filename = match res.filename() {
                    Some(x) => x.file_name().unwrap()
                        .to_os_string().into_string().unwrap(),
                    None => "???".to_owned(),
                };
                let lineno = res.lineno().unwrap_or(0);
                write!(backtrace, "{} at {}:{}\n", symbol,
                       filename, lineno).unwrap();
            });
            true
        });
        trace!("backtrace:\n{}", backtrace);
    }
}

fn compile_shader(gl: &Procs, wat: &str, typ: GLenum, texts: &[&[u8]])
    -> anyhow::Result<GLuint> {
    const SHADER_VERSION_SUPPLEMENT: &[u8] = br#"
#version 140
"#;
    // Unsafe justification: Lots of GL calls. We're careful about length but
    // we do assume the GL implementation doesn't lie to us in ways that are
    // TECHNICALLY allowed by the standard.
    unsafe {
        let shader = gl.CreateShader(typ);
        // if we get more than 9 source elements, this won't be enough,
        // but screw that
        let needed_len = texts.iter().fold(SHADER_VERSION_SUPPLEMENT.len(),
                                           |a, x| a + x.len() + 11);
        let mut buf: Vec<u8> = Vec::with_capacity(needed_len);
        buf.extend_from_slice(SHADER_VERSION_SUPPLEMENT);
        for (n, &text) in texts.iter().enumerate() {
            write!(buf, "#line 0 {}\n", n).unwrap();
            buf.write_all(text).unwrap();
            buf.write_all(b"\n").unwrap();
        }
        gl.ShaderSource(shader, 1,
                        transmute(&&buf[0]),
                        (&[buf.len() as GLint]).as_ptr());
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
                 attribs: &[(&[u8], &dyn Fn(&Procs, GLuint))]) {
    unsafe {
        gl.UseProgram(program);
        for &(name, f) in attribs.iter() {
            debug_assert!(name.ends_with(&[0]));
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

fn setup_uniforms(gl: &Procs, program: GLuint, program_name: &str,
                  uniforms: &[(&[u8], &dyn Fn(&Procs, GLint))]) {
    unsafe {
        gl.UseProgram(program);
        for &(name, f) in uniforms.iter() {
            debug_assert!(name.ends_with(&[0]));
            let loc = gl.GetUniformLocation(program, transmute(name.as_ptr()));
            if loc < 0 {
                warn!("couldn't find expected shader uniform {:?} \
                       in the {:?} program",
                      String::from_utf8_lossy(&name[..name.len()-1]),
                      program_name);
            }
            else {
                let loc = loc as GLint;
                f(gl, loc);
            }
        }
    }
}

fn setup_model_attribs(gl: &Procs, program: GLuint) {
    setup_attribs(&gl, program, "model", &[
        (b"pos\0", &|gl, loc| unsafe {
         gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 12,
                                transmute(0usize))
        }),
        (b"color_index\0", &|gl, loc| unsafe {
         gl.VertexAttribIPointer(loc, 1, GL_UNSIGNED_INT, 12,
                                 transmute(8usize))
        }),
    ]);
}

/// Set up an OpenGL 3.2 rendering context.
pub(crate) fn create<F>(video: &VideoSubsystem, builder_maker: &mut F)
    -> anyhow::Result<Box<dyn Renderer>>
where F: FnMut() -> WindowBuilder
{
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    #[cfg(debug_assertions)]
    gl_attr.set_context_flags().debug();
    gl_attr.set_multisample_buffers(0);
    gl_attr.set_multisample_samples(0);
    gl_attr.set_depth_size(0);
    // TODO: fail gracefully if sRGB compatible default framebuffer not avail-
    // able
    gl_attr.set_framebuffer_srgb_compatible(true);
    let window = match builder_maker().opengl().allow_highdpi().build() {
        Ok(x) => x,
        Err(x) => {
            return
                Err(anyhow!("Unable to create window for OpenGL 3.0 \
                             context: {}", x))?;
        },
    };
    let (w, h) = window.size();
    debug!("Window dimensions: {}x{}", w, h);
    let ctx = window.gl_create_context()
        .map_err(|x|anyhow!("Unable to create OpenGL 3.0 context: {}",x))?;
    window.gl_make_current(&ctx)
        .map_err(|x| anyhow!("OpenGL context lost! {}", x))?;
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
    let (program_model, program_text, force_multisample, quad_vao, quad_ib,
         quad_vb, loc_transform, loc_opacity, loc_colors, program_blit,
         max_multisample_power, program_bwm, ui_tex, bloom_tex, world_tex,
         ui_fb, bloom_fb, world_fb, program_bloomx, program_bloomy,
         gauss_tex, bloom_vao, bloom_vb);
    unsafe {
        // If we have the appropriate extension, let's make the debug messages
        // FLY!
        if gl.has_ARB_debug_output {
            info!("ARB_debug_output extension is present. OpenGL errors will \
                   be detected promptly.");
            gl.Enable(0x92E0);
            gl.GetError(); // ?!
            gl.Enable(GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB);
            gl.DebugMessageCallbackARB(Some(debug_callback), null());
        }
        // Compile and link the shaders... oh boy.
        let fshader_model = compile_shader(&gl, "the model fragment shader",
                                           GL_FRAGMENT_SHADER,
                                           &[MODEL_FRAGMENT_SOURCE])?;
        let vshader_model = compile_shader(&gl, "the model vertex shader",
                                           GL_VERTEX_SHADER,
                                           &[MODEL_VERTEX_SOURCE])?;
        let fshader_text = compile_shader(&gl, "the text fragment shader",
                                          GL_FRAGMENT_SHADER,
                                          &[TEXT_FRAGMENT_SOURCE])?;
        let vshader_text = compile_shader(&gl, "the text vertex shader",
                                          GL_VERTEX_SHADER,
                                          &[TEXT_VERTEX_SOURCE])?;
        let vshader_blit = compile_shader(&gl, "the blit vertex shader",
                                          GL_VERTEX_SHADER,
                                          &[BLIT_VERTEX_SOURCE])?;
        let fshader_blit = compile_shader(&gl,"the blit fragment shader",
                                          GL_FRAGMENT_SHADER,
                                          &[BLIT_FRAGMENT_SOURCE])?;
        let fshader_bwm = compile_shader(&gl,"the blit-w-mat fragment shader",
                                         GL_FRAGMENT_SHADER,
                                         &[BWM_FRAGMENT_SOURCE])?;
        let fshader_bloomx = compile_shader(&gl, "the bloom-x fragment shader",
                                            GL_FRAGMENT_SHADER,
                                            &[BLOOM_X_SUPPLEMENT,
                                              BLOOM_FRAGMENT_SOURCE])?;
        let fshader_bloomy = compile_shader(&gl, "the bloom-y fragment shader",
                                            GL_FRAGMENT_SHADER,
                                            &[BLOOM_Y_SUPPLEMENT,
                                              BLOOM_FRAGMENT_SOURCE])?;
        program_model = link_program(&gl, "the model shader program",
                                     &[vshader_model, fshader_model])?;
        program_text = link_program(&gl, "the text shader program",
                                    &[vshader_text, fshader_text])?;
        program_blit = link_program(&gl, "the blit shader program",
                                    &[vshader_blit, fshader_blit])?;
        program_bwm = link_program(&gl, "the blit-w-mat shader program",
                                    &[vshader_blit, fshader_bwm])?;
        program_bloomx = link_program(&gl, "the bloom-x shader program",
                                      &[vshader_blit, fshader_bloomx])?;
        program_bloomy = link_program(&gl, "the bloom-y shader program",
                                      &[vshader_blit, fshader_bloomy])?;
        gl.DeleteShader(vshader_model);
        gl.DeleteShader(fshader_model);
        gl.DeleteShader(vshader_text);
        gl.DeleteShader(fshader_text);
        gl.DeleteShader(vshader_blit);
        gl.DeleteShader(fshader_blit);
        gl.DeleteShader(fshader_bwm);
        gl.DeleteShader(fshader_bloomx);
        gl.DeleteShader(fshader_bloomy);
        // Generate VAOs, VBOs, textures, and framebuffers
        let mut vaos = [0; 2];
        gl.GenVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        quad_vao = vaos[0];
        bloom_vao = vaos[1];
        let mut vbos = [0; 3];
        gl.GenBuffers(vbos.len() as GLint, &mut vbos[0]);
        quad_ib = vbos[0];
        quad_vb = vbos[1];
        bloom_vb = vbos[2];
        let mut tex = [0; 6];
        gl.GenTextures(tex.len() as GLint, &mut tex[0]);
        world_tex = tex[0];
        ui_tex = tex[1];
        bloom_tex = [tex[2], tex[3]];
        gauss_tex = [tex[4], tex[5]];
        let mut fb = [0; 4];
        gl.GenFramebuffers(fb.len() as GLint, &mut fb[0]);
        world_fb = fb[0];
        ui_fb = fb[1];
        bloom_fb = [fb[2], fb[3]];
        // Text quad drawing stuff
        gl.BindVertexArray(quad_vao);
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
            (b"pos\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_HALF_FLOAT, 0, 24,
                                    transmute(0usize))),
            (b"uv_in\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_HALF_FLOAT, 0, 24,
                                    transmute(4usize))),
            (b"vert_fillcolor\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 4, GL_HALF_FLOAT, 0, 24,
                                    transmute(8usize))),
            (b"vert_strokecolor\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 4, GL_HALF_FLOAT, 0, 24,
                                    transmute(16usize))),
        ]);
        // Bloom and Blit-With-Matrix stuff
        gl.BindVertexArray(bloom_vao);
        gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, quad_ib);
        gl.BindBuffer(GL_ARRAY_BUFFER, bloom_vb);
        gl.BufferData(GL_ARRAY_BUFFER, 64, null(), GL_STATIC_DRAW);
        // Bloom/BWM: X___Y___U___V___
        setup_attribs(&gl, program_bloomx, "bloom", &[
            (b"pos\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 16,
                                    transmute(0usize))),
            (b"uv_in\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 16,
                                    transmute(8usize))),
        ]);
        // Do linear-to-sRGB compression before writing to the framebuffer and
        // decompression after reading (for blending)
        gl.Enable(GL_FRAMEBUFFER_SRGB);
        // Set up blending for premultiplied alpha
        gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        // We're gonna be uploading a lot of unaligned pixel data, yuck.
        gl.PixelStorei(GL_UNPACK_ALIGNMENT, 1);
        // Oh, and let's set the uniforms and/or find them!
        for &prog in &[program_blit, program_bwm, program_bloomx,
                       program_bloomy] {
            setup_uniforms(&gl, prog, "(a blit program)", &[
                (b"src\0", &|gl, loc|
                 gl.Uniform1i(loc, 0)),
            ]);
        }
        for &prog in &[program_bloomx, program_bloomy] {
            setup_uniforms(&gl, prog, "(a bloom program)", &[
                (b"gauss\0", &|gl, loc|
                 gl.Uniform1i(loc, 1)),
            ]);
        }
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
        let mut max = 0;
        gl.GetIntegerv(GL_MAX_SAMPLES, &mut max);
        if max >= 32 { max_multisample_power = 5 }
        else if max >= 16 { max_multisample_power = 4 }
        else if max >= 8 { max_multisample_power = 3 }
        else if max >= 4 { max_multisample_power = 2 }
        else if max >= 2 { max_multisample_power = 1 }
        else { max_multisample_power = 0 }
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
        program_model, program_text, program_bwm, bound_texture: None,
        force_multisample, quad_vao, quad_vb, model_cache: ModelCache::new(),
        loc_transform, loc_colors, loc_opacity, max_multisample_power,
        world_w: 0, world_h: 0, world_samples: 0, bloom_w: 0, bloom_h: 0,
        world_super_x: 0, world_super_y: 0, world_tex, world_fb, ui_tex, ui_fb,
        ui_w: 0, ui_h: 0, ui_samples: 0, bloom_fb, bloom_tex, gauss_tex,
        gauss_radius: [0.0,0.0], program_bloomx, program_bloomy,
        bloom_vao, bloom_vb, program_blit, is_blending: false,
    }))
}

impl Renderer for OpenGL32 {
    fn begin_rendering(&mut self, params: &RenderParams) -> anyhow::Result<()> {
        self.window.gl_make_current(&self.ctx)
            .map_err(|x| anyhow!("OpenGL context lost! {}", x))?;
        let (w, h) = self.get_size();
        let num_samples = params.world_oversamples.min(5);
        let ms_bits = num_samples.min(self.max_multisample_power);
        let os_bits = num_samples - ms_bits;
        let x_os_bits = os_bits / 2;
        let y_os_bits = (os_bits + 1) / 2; // ??!
        if x_os_bits != 0 || y_os_bits != 0 {
            todo!("supersampling bwm")
        }
        let world_super_x = 1 << x_os_bits;
        let world_super_y = 1 << y_os_bits;
        let world_w = w * world_super_x;
        let world_h = h * world_super_y;
        let world_samples = 1 << ms_bits;
        let gl = &self.gl;
        if self.world_w != world_w || self.world_h != world_h
        || self.world_samples != world_samples {
            debug!("recreating world framebuffer at {}x{}x{}",
                   world_w, world_h, world_samples);
            self.world_w = world_w;
            self.world_h = world_h;
            self.world_samples = world_samples;
            unsafe {
                gl.BindTexture(GL_TEXTURE_2D_MULTISAMPLE, self.world_tex);
                gl.TexImage2DMultisample(GL_TEXTURE_2D_MULTISAMPLE,
                                         world_samples as GLint, GL_RGB16F,
                                         world_w as GLint, world_h as GLint,
                                         0);
                assertgl(gl, "creating multisampled float texture")?;
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_fb);
                gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                        GL_COLOR_ATTACHMENT0,
                                        GL_TEXTURE_2D_MULTISAMPLE,
                                        self.world_tex, 0);
                assertgl(gl, "creating multisampled framebuffer")?;
                if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                    != GL_FRAMEBUFFER_COMPLETE {
                        return Err(anyhow!("world framebuffer wasn't \
                                            complete, but had no errors?!"))
                }
            }
        }
        if params.is_bloom_enabled() /*TODO|| !params.world_mat.is_identity(0.0)*/ {
            let bloom_w = w;
            let bloom_h = h;
            if self.bloom_w != bloom_w || self.bloom_h != bloom_h {
                // note: bit wasteful, we make two framebuffers even if we're
                // not doing bloom... meh, we use practically no VRAM otherwise
                // so we're good!
                debug!("recreating bloom/resolve framebuffers at {}x{}",
                       bloom_w, bloom_h);
                self.bloom_w = bloom_w;
                self.bloom_h = bloom_h;
                unsafe {
                    self.bound_texture = None;
                    for pong in 0..2 {
                        gl.BindTexture(GL_TEXTURE_2D, self.bloom_tex[pong]);
                        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGB16F as GLint,
                                      bloom_w as GLint, bloom_h as GLint,
                                      0, GL_RGB, GL_HALF_FLOAT, null());
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                         GL_NEAREST as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                         GL_NEAREST as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                                         GL_CLAMP_TO_EDGE as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                                         GL_CLAMP_TO_EDGE as GLint);
                        assertgl(gl, "creating bloom texture")?;
                        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                                           self.bloom_fb[pong]);
                        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                                GL_COLOR_ATTACHMENT0,
                                                GL_TEXTURE_2D,
                                                self.bloom_tex[pong], 0);
                        assertgl(gl, "creating bloom framebuffer")?;
                        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                            != GL_FRAMEBUFFER_COMPLETE {
                                return Err(anyhow!("bloom framebuffer wasn't \
                                                    complete but had no \
                                                    errors?!"))
                            }
                    }
                    // while we're at it, make a new buffer for the "vertices"
                    let buf = [
                        -1.0, -1.0, 0.0, 0.0,
                        1.0, -1.0, bloom_w as f32 + 0.0, 0.0,
                        1.0, 1.0, bloom_w as f32 + 0.0, bloom_h as f32 + 0.0,
                        -1.0, 1.0, 0.0, bloom_h as f32 + 0.0,
                    ];
                    gl.BindBuffer(GL_ARRAY_BUFFER, self.bloom_vb);
                    gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                                  GL_STATIC_DRAW);
                }
            }
            if params.is_bloom_enabled() {
                let radii_same = params.bloom_radius.y
                    == params.bloom_radius.x;
                for axis in 0 .. 2 {
                    // if X and Y are the same, don't recalculate for both
                    if axis == 1 && radii_same { continue }
                    if self.gauss_radius[axis] != params.bloom_radius[axis] {
                        let radius = params.bloom_radius[axis];
                        self.gauss_radius[axis] = radius;
                        let samples = (radius * 3.0).ceil() as usize + 1;
                        debug!("creating gauss texture for {} radius \
                                ({} samples)", radius, samples);
                        let mut buf: Vec<f32> = Vec::with_capacity(samples);
                        for i in 0 .. samples {
                            let th = (i as f32) / radius;
                            buf.push(1.0);
                        }
                        let total = buf.iter().fold(0.0, |a, &x| a+x);
                        for el in buf.iter_mut() {
                            *el /= total;
                        }
                        eprintln!("{:?}", buf);
                        unsafe {
                            gl.BindTexture(GL_TEXTURE_1D,self.gauss_tex[axis]);
                            gl.TexImage1D(GL_TEXTURE_1D, 0, GL_R32F as GLint,
                                          samples as GLint,
                                          0, GL_LUMINANCE, GL_FLOAT,
                                          transmute(&buf[0]));
                            gl.TexParameteri(GL_TEXTURE_1D,
                                             GL_TEXTURE_MIN_FILTER,
                                             GL_NEAREST as GLint);
                            gl.TexParameteri(GL_TEXTURE_1D,
                                             GL_TEXTURE_MAG_FILTER,
                                             GL_NEAREST as GLint);
                            let mut progs = Vec::with_capacity(4);
                            if axis == 0 {
                                progs.push(self.program_bloomx);
                            }
                            if axis == 1 || radii_same {
                                progs.push(self.program_bloomy);
                            }
                            for &prog in progs.iter() {
                                setup_uniforms(&gl, prog, "bloom", &[
                                    (b"num_samples\0", &|gl, loc|
                                        gl.Uniform1i(loc, samples as GLint)),
                                ]);
                            }
                        }
                    }
                }
            }
        }
        let ui_sample_bits = params.ui_oversamples.min(5)
            .min(self.max_multisample_power);
        let ui_samples = 1 << ui_sample_bits;
        let ui_w = w;
        let ui_h = h;
        if self.ui_w != w || self.ui_h != h || self.ui_samples != ui_samples {
            debug!("recreating UI framebuffer at {}x{}x{}",
                   ui_w, ui_h, ui_samples);
            self.ui_w = w;
            self.ui_h = h;
            self.ui_samples = ui_samples;
            if ui_samples == 1 {
                debug!("  (using default framebuffer instead)");
                // TODO: can we unallocate the texture without having to
                // generate a new one later?
            }
            else {
                unsafe {
                    gl.BindTexture(GL_TEXTURE_2D_MULTISAMPLE, self.ui_tex);
                    gl.TexImage2DMultisample(GL_TEXTURE_2D_MULTISAMPLE,
                                             ui_samples as GLint, GL_SRGB8,
                                             ui_w as GLint, ui_h as GLint,
                                             0);
                    assertgl(gl, "creating multisampled float texture")?;
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.ui_fb);
                    gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                            GL_COLOR_ATTACHMENT0,
                                            GL_TEXTURE_2D_MULTISAMPLE,
                                            self.ui_tex, 0);
                    assertgl(gl, "creating multisampled framebuffer")?;
                    if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                        != GL_FRAMEBUFFER_COMPLETE {
                            return Err(anyhow!("UI framebuffer wasn't \
                                                complete, but had no errors?!"))
                        }
                }
            }
        }
        unsafe {
            gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_fb);
        };
        #[cfg(debug_assertions)]
        assertgl(&self.gl, "starting rendering (this means the error probably \
                            occurred while rendering the last frame, but \
                            wasn't caught when it arose)").unwrap();
        Ok(())
    }
    fn finish_world(&mut self, params: &RenderParams) {
        // (note: blending is DISABLED at the start of this, and restored to
        // user state the end. multisampling is DISABLED at the start of this,
        // and gets restored the next time the batch type gets frobbed.)
        unsafe {
            if self.is_blending { self.gl.Disable(GL_BLEND); }
            if !self.force_multisample { self.gl.Disable(GL_MULTISAMPLE); }
        }
        self.last_batch_type = LastBatchType::None;
        self.bound_texture = None;
        // STAGE 1: Resolve world, BWM world
        let world_src_tex = self.resolve_world(params);
        // If bloom is required, the world is waiting in bloom_fb[1] for its
        // first round of blurring.
        // The post-mat world is waiting in the ui framebuffer (which might be
        // the default framebuffer)
        // STAGE 2: Do bloom (optional)
        if params.is_bloom_enabled() {
            let world_src_tex = world_src_tex
                .expect("If bloom is enabled, we expect to be told which \
                         texture to use! (logic error)");
            unsafe { self.gl.BlendFunc(GL_ONE, GL_ONE); }
            self.do_bloom(params, world_src_tex);
            unsafe { self.gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA); }
        }
        // The result of bloom is now waiting in the UI framebuffer (which,
        // again, might be the default framebuffer)
        unsafe {
            self.gl.BindFramebuffer(GL_READ_FRAMEBUFFER, 0);
            self.gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.which_ui_fb());
            if self.is_blending { self.gl.Enable(GL_BLEND); }
        }
    }
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let gl = &self.gl;
        // Unsafe justification: Only primitive data are sent, and these
        // functions cannot error.
        unsafe {
            gl.ClearColor(r, g, b, a);
            gl.Clear(GL_COLOR_BUFFER_BIT);
        }
        #[cfg(debug_assertions)]
        assertgl(gl, "clearing the screen").unwrap();
    }
    fn present(&mut self) {
        let gl = &self.gl;
        #[cfg(debug_assertions)]
        assertgl(gl, "finishing rendering (this means the error probably \
                      occurred while rendering this frame, but wasn't caught \
                      when it arose)").unwrap();
        // STAGE 3: maybe resolve UI framebuffer
        if self.ui_samples > 1 {
            unsafe {
                gl.BindFramebuffer(GL_READ_FRAMEBUFFER, self.ui_fb);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
                gl.BlitFramebuffer(0, 0, self.ui_w as i32, self.ui_h as i32,
                                   0, 0, self.ui_w as i32, self.ui_h as i32,
                                   GL_COLOR_BUFFER_BIT, GL_NEAREST);
            }
        }
        self.window.gl_swap_window();
    }
    fn get_size(&self) -> (u32, u32) {
        self.window.drawable_size()
    }
    fn enable_blend(&mut self) {
        let gl = &self.gl;
        if !self.is_blending {
            // Unsafe justification: Trivially safe.
            unsafe { gl.Enable(GL_BLEND); }
            self.is_blending = true;
        }
    }
    fn disable_blend(&mut self) {
        let gl = &self.gl;
        if self.is_blending {
            // Unsafe justification: Trivially safe.
            unsafe { gl.Disable(GL_BLEND); }
            self.is_blending = false;
        }
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
        }
        #[cfg(debug_assertions)]
        assertgl(gl, "making a new atlas texture").unwrap();
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
        // whose bounds are predictable. If `GL_UNPACK_ALIGNMENT`  to a
        // value other than 1, this will be unsound!
        debug_assert!(pixels.len() >= glyph_w as usize * glyph_h as usize * 3);
        unsafe {
            gl.TexSubImage2D(GL_TEXTURE_2D, 0,
                             glyph_x as GLint, glyph_y as GLint,
                             glyph_w as GLsizei, glyph_h as GLsizei,
                             GL_RGB, GL_UNSIGNED_BYTE, transmute(&pixels[0]));
        }
        #[cfg(debug_assertions)]
        assertgl(gl, "uploading a glyph image").unwrap();
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
                #[cfg(debug_assertions)]
                assertgl(gl, "switching to the text shader").unwrap();
            }
            for (&atlas, verts) in atlases.iter().zip(batch.verts.iter()) {
                for seg in verts.chunks(QUAD_IB_COUNT*4) {
                    let chunksize
                        = (seg.len() * size_of::<MergedTextVert>())
                        as GLsizeiptr;
                    if self.bound_texture != Some(atlas) {
                        gl.BindTexture(GL_TEXTURE_2D, atlas);
                        self.bound_texture = Some(atlas);
                    }
                    gl.BufferData(GL_ARRAY_BUFFER, BUF_SIZE as GLsizeiptr,
                                  null(), GL_STREAM_DRAW);
                    gl.BufferSubData(GL_ARRAY_BUFFER, 0,
                                     chunksize, transmute(seg.as_ptr()));
                    gl.DrawElements(GL_TRIANGLES,
                                    (seg.len() / 4 * 6) as GLint,
                                    GL_UNSIGNED_SHORT, null());
                }
            }
        }
        #[cfg(debug_assertions)]
        assertgl(&self.gl, "rendering a text batch").unwrap();
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
                #[cfg(debug_assertions)]
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

impl OpenGL32 {
    fn which_ui_fb(&self) -> GLuint {
        if self.ui_samples > 1 { self.ui_fb }
        else { 0 }
    }
    fn resolve_world(&mut self, params: &RenderParams) -> Option<GLuint> {
        let gl = &self.gl;
        let ret;
        unsafe {
            // Okay. The questions that have an effect on our choices:
            let is_bloom_enabled = params.is_bloom_enabled();
            let is_resolve_needed = self.world_samples > 1;
            if self.world_super_x > 1 || self.world_super_y > 1 {
                todo!("supersampling");
            }
            let is_bwm_needed = false;//TODO !params.world_mat.is_identity(0.0);
            // If bloom is NOT enabled, AND there is no need for a BWM...
            if !is_bloom_enabled && !is_bwm_needed {
                // ...then we can just blit directly to whichever UI
                // framebuffer and be done with it.
                // TODO: cut out world_fb if the pipeline gets really simple
                gl.BindFramebuffer(GL_READ_FRAMEBUFFER, self.world_fb);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                                   self.which_ui_fb());
                gl.BlitFramebuffer(0,0, self.ui_w as i32, self.ui_h as i32,
                                   0,0, self.ui_w as i32, self.ui_h as i32,
                                   GL_COLOR_BUFFER_BIT, GL_NEAREST);
                ret = None;
            }
            else {
                // Bloom is enabled OR we need a BWM.
                let (world_src_tex, world_src_fb);
                if is_resolve_needed {
                    gl.BindFramebuffer(GL_READ_FRAMEBUFFER, self.world_fb);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.bloom_fb[1]);
                    gl.BlitFramebuffer(0,0, self.ui_w as i32, self.ui_h as i32,
                                       0,0, self.ui_w as i32, self.ui_h as i32,
                                       GL_COLOR_BUFFER_BIT, GL_NEAREST);
                    (world_src_tex, world_src_fb)
                        = (self.bloom_tex[1], self.bloom_fb[1]);
                }
                else {
                    (world_src_tex, world_src_fb)
                        = (self.world_tex, self.world_fb);
                }
                if is_bwm_needed {
                    todo!("scv");
                }
                else {
                    gl.BindFramebuffer(GL_READ_FRAMEBUFFER, world_src_fb);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,self.which_ui_fb());
                    gl.BlitFramebuffer(0,0, self.ui_w as i32, self.ui_h as i32,
                                       0,0, self.ui_w as i32, self.ui_h as i32,
                                       GL_COLOR_BUFFER_BIT, GL_NEAREST);
                }
                ret = Some(world_src_tex);
            }
        }
        #[cfg(debug_assertions)]
        assertgl(gl, "resolving the world").unwrap();
        ret
    }
    fn do_bloom(&mut self, params: &RenderParams, world_src_tex: GLuint) {
        // NOTE: blending is disabled when we get here
        let need_color_matrix = true;//TODO !params.bloom_premat.is_identity(0.0);
        let radii_same = params.bloom_radius.y
            == params.bloom_radius.x;
        let gl = &self.gl;
        unsafe {
            let mut src_tex = world_src_tex;
            let mut dst_fb = self.bloom_fb[0];
            gl.BindVertexArray(self.bloom_vao);
            gl.ActiveTexture(GL_TEXTURE1);
            gl.BindTexture(GL_TEXTURE_1D, self.gauss_tex[0]);
            gl.ActiveTexture(GL_TEXTURE0);
            gl.UseProgram(self.program_bloomx);
            for _ in 0 .. params.bloom_iterations[0] {
                gl.BindTexture(GL_TEXTURE_2D, src_tex);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, dst_fb);
                gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                if dst_fb == self.bloom_fb[0] {
                    src_tex = self.bloom_tex[0];
                    dst_fb = self.bloom_fb[1];
                }
                else {
                    src_tex = self.bloom_tex[0];
                    dst_fb = self.bloom_fb[1];
                }
            }
            if !radii_same {
                gl.ActiveTexture(GL_TEXTURE1);
                gl.BindTexture(GL_TEXTURE_1D, self.gauss_tex[1]);
                gl.ActiveTexture(GL_TEXTURE0);
            }
            gl.UseProgram(self.program_bloomy);
            for _ in 0 .. params.bloom_iterations[1] {
                gl.BindTexture(GL_TEXTURE_2D, src_tex);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, dst_fb);
                gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                if dst_fb == self.bloom_fb[0] {
                    src_tex = self.bloom_tex[0];
                    dst_fb = self.bloom_fb[1];
                }
                else {
                    src_tex = self.bloom_tex[0];
                    dst_fb = self.bloom_fb[1];
                }
            }
            // TODO: optimize out last blit
            gl.Enable(GL_BLEND);
            gl.UseProgram(self.program_blit);
            gl.BindTexture(GL_TEXTURE_2D, src_tex);
            gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                               self.which_ui_fb());
            gl.ClearColor(0.0, 0.0, 0.0, 0.0);
            //gl.Clear(GL_COLOR_BUFFER_BIT);
            gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
        }
    }
}
