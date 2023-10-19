use super::*;

use core::slice;
use std::{
    collections::{HashMap, hash_map::Entry},
    env,
    f32::consts::{E,PI},
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
    vbos: [GLuint; 2],
    num_elements: GLint,
}

struct ModelCache {
    models: HashMap<u32, CachedModel>,
}

impl ModelCache {
    fn new() -> ModelCache { ModelCache { models: HashMap::new() } }
    fn purge(&mut self, gl: &Procs, model: &Model) {
        match self.models.entry(model.unique_id) {
            Entry::Occupied(x) => {
                unsafe {
                    let v = x.get();
                    gl.DeleteVertexArrays(1, &v.vao);
                    gl.DeleteBuffers(2, &v.vbos[0]);
                }
                x.remove_entry();
            },
            _ => (),
        }
    }
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
                // Unsafe justification: okay, look, we're just gonna be using
                // unsafe every time we talk to OpenGL. We aren't going to be
                // able to avoid that. Okay? Okay.
                unsafe {
                    let mut vao = 0;
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
                                  transmute(verts[..].as_ptr()),
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
                                  transmute(indices[..].as_ptr()),
                                  GL_STATIC_DRAW);
                    let num_elements = indices.len() as GLint;
                    CachedModel {
                        vao, vbos, num_elements,
                    }
                }
            })
    }
}

/// Renders using OpenGL 3.2 Core.
struct OpenGL32 {
    window: Window,
    ctx: GLContext,
    gl: Procs,
    last_batch_type: LastBatchType,
    /// Program for drawing model triangles
    program_model: GLuint,
    /// Program for rendering text quads
    program_text: GLuint,
    /// Program for straight pixel-to-pixel blit
    program_blit: GLuint,
    /// Program for smooth pixel-to-pixel blit
    program_blit_smooth: GLuint,
    /// Program for Blit-With-Matrix
    program_bwm: GLuint,
    /// Program for Blit-With-Matrix, but smooth
    program_bwm_smooth: GLuint,
    /// Program for horizontal component of Gaussian blur
    program_bloomx: GLuint,
    /// Program for vertical component of Gaussian blur
    program_bloomy: GLuint,
    /// Program for merging two framebuffers by addition
    program_merge: GLuint,
    /// Program for merging two framebuffers by addition, one of which is
    /// smaller than the other
    program_mergeup: GLuint,
    /// Program for merging two framebuffers by addition, where both are
    /// smaller than the framebuffer they're being drawn into
    program_upmergeup: GLuint,
    /// Program for merging two framebuffers by addition
    /// (and multiplying the first one by a matrix)
    program_merge_wm: GLuint,
    /// Program for merging two framebuffers by addition, one of which is
    /// smaller than the other
    /// (and multiplying the first one by a matrix)
    program_mergeup_wm: GLuint,
    /// Program for merging two framebuffers by addition, where both are
    /// smaller than the framebuffer they're being drawn into
    /// (and multiplying the first one by a matrix)
    program_upmergeup_wm: GLuint,
    /// Program for downsampling a framebuffer
    program_downsample: GLuint,
    /// Program for downsampling a framebuffer and also applying a color matrix
    program_downsample_wm: GLuint,
    bound_texture: Option<GLuint>,
    force_multisample: bool,
    can_downsample_with_blit: bool,
    can_delinearize_with_blitframebuffer: bool,
    quad_vao: GLuint,
    quad_vb: GLuint,
    bloom_vao: GLuint,
    bloom_vb: GLuint,
    ui_vao: GLuint,
    ui_vb: GLuint,
    world_ds_vao: GLuint,
    world_ds_vb: GLuint,
    world_final_vao: GLuint,
    world_final_vb: GLuint,
    /// this VAO has normalized texture coordinates, instead of depending on
    /// the size
    updog_vao: GLuint,
    model_cache: ModelCache,
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
    /// Width of world's multisample resolving framebuffer
    world_res_w: u32,
    /// Height of world's multisample resolving framebuffer
    world_res_h: u32,
    /// Texture created to resolve the world's multisamples
    world_res_tex: GLuint,
    /// Framebuffer created to resolve the world's multisamples
    world_res_fb: GLuint,
    /// Width of UI's multisample resolving framebuffer
    ui_res_w: u32,
    /// Height of UI's multisample resolving framebuffer
    ui_res_h: u32,
    /// Texture created to resolve the UI's multisamples
    ui_res_tex: GLuint,
    /// Framebuffer created to resolve the UI's multisamples
    ui_res_fb: GLuint,
    /// Width of world's oversample resolving framebuffer
    world_ds_w: u32,
    /// Height of world's oversample resolving framebuffer
    world_ds_h: u32,
    /// Final, resolved, downsampled, etc. world width
    world_final_w: u32,
    /// Final, resolved, downsampled, etc. world height
    world_final_h: u32,
    /// Texture created to resolve the world's oversamples and color matrix
    world_ds_tex: GLuint,
    /// Framebuffer created to resolve the world's oversamples and color matrix
    world_ds_fb: GLuint,
    /// Width of bloom framebuffer
    bloom_w: u32,
    /// Height of bloom framebuffer
    bloom_h: u32,
    /// Number of X samples we'll use for each pixel of the bloom framebuffer
    bloom_under_x: u32,
    /// Number of Y samples we'll use for each pixel of the bloom framebuffer
    bloom_under_y: u32,
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
    /// True if the world is being rendered undersampled
    world_is_undersampled: bool,
}

/// Number of bytes to make our VBOs
const BUF_SIZE: usize = 262144; // 256KiB at a time
/// Number of indices to put into our IBO (is that a thing?)
const QUAD_IB_COUNT: usize = BUF_SIZE / (size_of::<MergedTextVert>()*4);
/// Number of bytes to make our IBO
const QUAD_IB_SIZE: usize = QUAD_IB_COUNT * 6 * 2;

const MODEL_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/model.frag");
const MODEL_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/model.vert");
const TEXT_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/text.frag");
const TEXT_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/text.vert");
const BLIT_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/blit.frag");
const BLIT_SMOOTH_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/blit_smooth.frag");
const BLIT_VERTEX_SOURCE: &[u8] = include_bytes!("gl32/blit.vert");
const BWM_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/bwm.frag");
// bwm program uses the blit vertex shader
const BWM_SMOOTH_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/bwm_smooth.frag");
// bwm_smooth program uses the blit vertex shader
const BLOOM_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/bloom.frag");
// bloom program uses the blit vertex shader
const MERGE_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/merge.frag");
// merge program uses the blit vertex shader
const MERGEUP_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/mergeup.frag");
// mergeup program uses the blit vertex shader
const UPMERGEUP_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/upmergeup.frag");
// upmergeup program uses the blit vertex shader
const DOWNSAMPLE_FRAGMENT_SOURCE: &[u8] = include_bytes!("gl32/downsample.frag");
// downsample ALSO uses the blit vertex shader, foo

const BLOOM_X_SUPPLEMENT: &[u8] = br#"
#define BLOOM_HORIZ 1
"#;
const BLOOM_Y_SUPPLEMENT: &[u8] = br#"
#define BLOOM_VERT 1
"#;
const WITH_MATRIX_SUPPLEMENT: &[u8] = br#"
#define WITH_MATRIX 1
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
        GL_DEBUG_TYPE_OTHER_ARB => if severity <= GL_DEBUG_SEVERITY_LOW_ARB { "something boring".to_owned() } else { "something spooky".to_owned() },
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
        GL_DEBUG_SEVERITY_LOW_ARB => {
            info!("{} detected {}: [LOW, {}] {}",
                  source, typ, id, message);
        },
        _ => {
            debug!("{} detected {}: [???, {}] {}",
                   source, typ, id, message);
        },
    }
    if env::var_os("RUST_BACKTRACE").is_some() {
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

struct CompiledShader<'a>(GLuint, &'a Procs);

impl Deref for CompiledShader<'_> {
    type Target = GLuint;
    fn deref(&self) -> &GLuint {
        &self.0
    }
}

impl Drop for CompiledShader<'_> {
    fn drop(&mut self) {
        unsafe {
            self.1.DeleteShader(self.0)
        }
    }
}

fn compile_shader<'a>(gl: &'a Procs, wat: &str, typ: GLenum, texts: &[&[u8]])
    -> anyhow::Result<CompiledShader<'a>> {
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
        Ok(CompiledShader(shader, &gl))
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
    gl_attr.set_context_version(3, 2);
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
                Err(anyhow!("Unable to create window for OpenGL 3.2 \
                             context: {}", x))?;
        },
    };
    let ctx = window.gl_create_context()
        .map_err(|x|anyhow!("Unable to create OpenGL 3.2 context: {}",x))?;
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
    unsafe {
        for _ in 0 .. 5 { // grumble grumble
            gl.ClearColor(0.25, 0.25, 0.25, 1.0);
            gl.Clear(GL_COLOR_BUFFER_BIT);
            window.gl_swap_window();
        }
        // If we have the appropriate extension, let's make the debug messages
        // FLY!
        if gl.has_ARB_debug_output {
            debug!("ARB_debug_output extension is present. OpenGL errors will \
                    be detected promptly.");
            gl.Enable(0x92E0); // Mesa bug workaround
            gl.GetError(); // ?!
            #[cfg(debug_assertions)]
            gl.Enable(GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB);
            gl.DebugMessageCallbackARB(Some(debug_callback), null());
        }
        else {
            info!("ARB_debug_output extension is missing. OpenGL errors may \
                   not be detected promptly.");
        }
        let (program_model, program_text, program_blit, program_blit_smooth,
             program_bwm, program_bwm_smooth, program_bloomx, program_bloomy,
             program_merge, program_mergeup, program_upmergeup,
             program_merge_wm, program_mergeup_wm, program_upmergeup_wm,
             program_downsample, program_downsample_wm);
        {
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
            let fshader_blit_smooth = compile_shader(&gl,
                                                     "the smooth blit fragment \
                                                      shader",
                                                     GL_FRAGMENT_SHADER,
                                                     &[BLIT_SMOOTH_FRAGMENT_SOURCE])?;
            let fshader_bwm = compile_shader(&gl,"the blit-w-mat fragment shader",
                                             GL_FRAGMENT_SHADER,
                                             &[BWM_FRAGMENT_SOURCE])?;
            let fshader_bwm_smooth = compile_shader(&gl,
                                                    "the smooth blit-w-mat \
                                                     fragment shader",
                                                    GL_FRAGMENT_SHADER,
                                                    &[BWM_SMOOTH_FRAGMENT_SOURCE])?;
            let fshader_bloomx = compile_shader(&gl, "the bloom-x fragment shader",
                                                GL_FRAGMENT_SHADER,
                                                &[BLOOM_X_SUPPLEMENT,
                                                  BLOOM_FRAGMENT_SOURCE])?;
            let fshader_bloomy = compile_shader(&gl, "the bloom-y fragment shader",
                                                GL_FRAGMENT_SHADER,
                                                &[BLOOM_Y_SUPPLEMENT,
                                                  BLOOM_FRAGMENT_SOURCE])?;
            let fshader_merge = compile_shader(&gl,"the merge fragment shader",
                                               GL_FRAGMENT_SHADER,
                                               &[MERGE_FRAGMENT_SOURCE])?;
            let fshader_mergeup = compile_shader(&gl,"the mergeup fragment shader",
                                                 GL_FRAGMENT_SHADER,
                                                 &[MERGEUP_FRAGMENT_SOURCE])?;
            let fshader_upmergeup = compile_shader(&gl,"the upmergeup fragment \
                                                        shader",
                                                   GL_FRAGMENT_SHADER,
                                                   &[UPMERGEUP_FRAGMENT_SOURCE])?;
            let fshader_merge_wm = compile_shader(&gl,"the merge (with matrix) \
                                                       fragment shader",
                                                  GL_FRAGMENT_SHADER,
                                                  &[WITH_MATRIX_SUPPLEMENT,
                                                    MERGE_FRAGMENT_SOURCE])?;
            let fshader_mergeup_wm = compile_shader(&gl,"the mergeup (with \
                                                         matrix) fragment shader",
                                                    GL_FRAGMENT_SHADER,
                                                    &[WITH_MATRIX_SUPPLEMENT,
                                                      MERGEUP_FRAGMENT_SOURCE])?;
            let fshader_upmergeup_wm = compile_shader(&gl,"the upmergeup (with \
                                                           matrix) fragment \
                                                           shader",
                                                      GL_FRAGMENT_SHADER,
                                                      &[WITH_MATRIX_SUPPLEMENT,
                                                        UPMERGEUP_FRAGMENT_SOURCE])?;
            let fshader_downsample
                = compile_shader(&gl, "the downsample fragment shader",
                                 GL_FRAGMENT_SHADER,
                                 &[DOWNSAMPLE_FRAGMENT_SOURCE])?;
            let fshader_downsample_wm
                = compile_shader(&gl,"the downsample with matrix shader",
                                 GL_FRAGMENT_SHADER,
                                 &[WITH_MATRIX_SUPPLEMENT,
                                   DOWNSAMPLE_FRAGMENT_SOURCE])?;
            program_model = link_program(&gl, "the model shader program",
                                             &[*vshader_model, *fshader_model])?;
            program_text = link_program(&gl, "the text shader program",
                                            &[*vshader_text, *fshader_text])?;
            program_blit = link_program(&gl, "the blit shader program",
                                            &[*vshader_blit, *fshader_blit])?;
            program_blit_smooth = link_program(&gl, "the smooth blit shader \
                                                         program",
                                                   &[*vshader_blit,
                                                     *fshader_blit_smooth])?;
            program_bwm = link_program(&gl, "the blit-w-mat shader program",
                                           &[*vshader_blit, *fshader_bwm])?;
            program_bwm_smooth = link_program(&gl,
                                                  "the smooth blit-w-mat shader \
                                                   program",
                                                  &[*vshader_blit,
                                                    *fshader_bwm_smooth])?;
            program_bloomx = link_program(&gl, "the bloom-x shader program",
                                              &[*vshader_blit, *fshader_bloomx])?;
            program_bloomy = link_program(&gl, "the bloom-y shader program",
                                              &[*vshader_blit, *fshader_bloomy])?;
            program_merge = link_program(&gl, "the merge shader program",
                                             &[*vshader_blit, *fshader_merge])?;
            program_mergeup = link_program(&gl, "the mergeup shader program",
                                               &[*vshader_blit, *fshader_mergeup])?;
            program_upmergeup = link_program(&gl, "the upmergeup shader program",
                                                 &[*vshader_blit, *fshader_upmergeup])?;
            program_merge_wm = link_program(&gl, "the merge (with matrix) shader \
                                                      program",
                                                &[*vshader_blit, *fshader_merge_wm])?;
            program_mergeup_wm = link_program(&gl, "the mergeup (with matrix) \
                                                        shader program",
                                                  &[*vshader_blit,
                                                    *fshader_mergeup_wm])?;
            program_upmergeup_wm = link_program(&gl, "the upmergeup (with matrix) \
                                                          shader program",
                                                    &[*vshader_blit,
                                                      *fshader_upmergeup_wm])?;
            program_downsample = link_program(&gl, "the downsample program",
                                                  &[*vshader_blit,
                                                    *fshader_downsample])?;
            program_downsample_wm = link_program(&gl, "downsample with matrix",
                                                     &[*vshader_blit,
                                                       *fshader_downsample_wm])?;
        }
        // Generate VAOs, VBOs, textures, and framebuffers
        let mut vaos = [0; 6];
        gl.GenVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        let quad_vao = vaos[0];
        let bloom_vao = vaos[1];
        let ui_vao = vaos[2];
        let world_ds_vao = vaos[3];
        let updog_vao = vaos[4];
        let world_final_vao = vaos[5];
        let mut vbos = [0; 7];
        gl.GenBuffers(vbos.len() as GLint, &mut vbos[0]);
        let quad_ib = vbos[0];
        let quad_vb = vbos[1];
        let bloom_vb = vbos[2];
        let ui_vb = vbos[3];
        let world_ds_vb = vbos[4];
        let updog_vb = vbos[5];
        let world_final_vb = vbos[6];
        let mut tex = [0; 9];
        gl.GenTextures(tex.len() as GLint, &mut tex[0]);
        let world_tex = tex[0];
        let ui_tex = tex[1];
        let bloom_tex = [tex[2], tex[3]];
        let gauss_tex = [tex[4], tex[5]];
        let world_res_tex = tex[6];
        let world_ds_tex = tex[7];
        let ui_res_tex = tex[8];
        let mut fb = [0; 7];
        gl.GenFramebuffers(fb.len() as GLint, &mut fb[0]);
        let world_fb = fb[0];
        let ui_fb = fb[1];
        let bloom_fb = [fb[2], fb[3]];
        let world_res_fb = fb[4];
        let world_ds_fb = fb[5];
        let ui_res_fb = fb[6];
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
        for &(vao, vb) in &[(bloom_vao, bloom_vb),
                            (ui_vao, ui_vb),
                            (world_ds_vao, world_ds_vb),
                            (updog_vao, updog_vb),
                            (world_final_vao, world_final_vb)] {
            // Bloom and Blit-With-Matrix and Downsample and... stuff
            gl.BindVertexArray(vao);
            gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, quad_ib);
            gl.BindBuffer(GL_ARRAY_BUFFER, vb);
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
        }
        let buf: [f32; 16] = [
            -1.0, -1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            -1.0, 1.0, 0.0, 1.0,
        ];
        gl.BindBuffer(GL_ARRAY_BUFFER, updog_vb);
        gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                      GL_STATIC_DRAW);
        // Do linear-to-sRGB compression before writing to the framebuffer and
        // decompression after reading (for blending)
        gl.Enable(GL_FRAMEBUFFER_SRGB);
        // Set up blending for premultiplied alpha
        gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        // We're gonna be uploading a lot of unaligned pixel data, yuck.
        gl.PixelStorei(GL_UNPACK_ALIGNMENT, 1);
        // Oh, and let's set the uniforms and/or find them!
        for &prog in &[program_blit, program_blit_smooth,
                       program_bwm, program_bwm_smooth,
                       program_bloomx, program_bloomy,
                       program_downsample, program_downsample_wm] {
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
        for &prog in &[program_merge, program_mergeup, program_upmergeup,
                       program_merge_wm, program_mergeup_wm,
                       program_upmergeup_wm] {
            setup_uniforms(&gl, prog, "merge", &[
                (b"src1\0", &|gl, loc|
                 gl.Uniform1i(loc, 0)),
                (b"src2\0", &|gl, loc|
                 gl.Uniform1i(loc, 1)),
            ]);
        }
        gl.UseProgram(program_text);
        setup_uniforms(&gl, program_text, "text", &[
            (b"atlas\0", &|gl, loc| gl.Uniform1i(loc, 0)),
        ]);
        gl.UseProgram(program_model);
        // Set up initial values for the uniforms... which, COINCIDENTALLY, are
        // also what we want for the multisample test
        setup_uniforms(&gl, program_model, "model", &[
            (b"transform\0", &|gl,loc| gl.UniformMatrix3fv(loc, 1, 0,
                (&[1.0f32, 0.0, 0.0,
                    0.0, 1.0, 0.0,
                    0.0, 0.0, 1.0]).as_ptr())),
            (b"opacity\0", &|gl,loc| gl.Uniform1f(loc, 1.0)),
            (b"colors\0", &|gl,loc| {
                gl.Uniform4fv(loc, 8,
                    [
                        1.0f32, 0.0, 1.0, 1.0,
                        1.0, 0.0, 0.0, 1.0,
                        0.0, 1.0, 0.0, 1.0,
                        1.0, 1.0, 0.0, 1.0,
                        0.0, 0.0, 1.0, 1.0,
                        1.0, 0.0, 1.0, 1.0,
                        0.0, 1.0, 1.0, 1.0,
                        1.0, 1.0, 1.0, 1.0,
                    ].as_ptr())
            })
        ]);
        let mut max_sample_count = 0;
        gl.GetIntegerv(GL_MAX_SAMPLES, &mut max_sample_count);
        if let Ok(value) = env::var("GL_MAX_SAMPLES") {
            match value.parse::<GLint>() {
                Ok(x) => {
                    let x = x.max(1);
                    if x > max_sample_count {
                        warn!("GL_MAX_SAMPLES found in environment, but the \
                               GL-reported value ({}) was smaller than the \
                               provided value ({}). Using the GL-reported \
                               value.", max_sample_count, x);
                    }
                    else {
                        info!("GL_MAX_SAMPLES found in environment. \
                               Overriding GL-reported value ({}) with \
                               provided value ({})", max_sample_count, x);
                        max_sample_count = x;
                    }
                },
                _ => warn!("GL_MAX_SAMPLES environment variable had a non-\
                            parseable value. Ignoring."),
            }
        }
        let max_multisample_power;
        if max_sample_count >= 32 { max_multisample_power = 5 }
        else if max_sample_count >= 16 { max_multisample_power = 4 }
        else if max_sample_count >= 8 { max_multisample_power = 3 }
        else if max_sample_count >= 4 { max_multisample_power = 2 }
        else if max_sample_count >= 2 { max_multisample_power = 1 }
        else { max_multisample_power = 0 }
        assertgl(&gl, "initializing the context")?;
        let force_multisample = if max_multisample_power <= 0 { false }
        else { check_multisample_bug(&gl, program_model)? };
        let can_downsample_with_blit
            = check_downsample_with_blit(&gl, program_blit)?;
        let can_delinearize_with_blitframebuffer
            = check_blit_delinearization(&gl)?;
        Ok(Box::new(OpenGL32 {
            window, ctx, gl, last_batch_type: LastBatchType::None,
            program_model, program_text, program_bwm, bound_texture:
            None, force_multisample, quad_vao, quad_vb, model_cache:
            ModelCache::new(),
            max_multisample_power, world_w: 0, world_h: 0,
            world_samples: 0, bloom_w: 0, bloom_h: 0, bloom_under_x:
            0, bloom_under_y: 0, world_super_x: 0, world_super_y: 0,
            world_tex, world_fb, ui_tex, ui_fb, ui_w: 0, ui_h: 0,
            ui_samples: 0, ui_res_w: 0, ui_res_h: 0, ui_res_fb,
            ui_res_tex, bloom_fb, bloom_tex, gauss_tex, gauss_radius:
            [0.0,0.0], program_bloomx, program_bloomy, world_res_tex,
            bloom_vao, bloom_vb, program_blit, is_blending: false,
            world_res_fb, world_ds_fb, world_ds_tex, world_ds_w: 0,
            world_ds_h: 0, world_res_w: 0, world_res_h: 0,
            program_merge, world_ds_vb, ui_vb, program_downsample,
            program_downsample_wm, ui_vao, world_ds_vao,
            program_mergeup, program_upmergeup, world_is_undersampled:
            false, updog_vao, can_downsample_with_blit,
            program_merge_wm, program_mergeup_wm,
            program_upmergeup_wm, world_final_vao, world_final_vb,
            world_final_w: 0, world_final_h: 0, program_bwm_smooth,
            can_delinearize_with_blitframebuffer, program_blit_smooth,
        }))
    }
}

impl Renderer for OpenGL32 {
    fn begin_rendering(&mut self, params: &RenderParams) -> anyhow::Result<()> {
        self.window.gl_make_current(&self.ctx)
            .map_err(|x| anyhow!("OpenGL context lost! {}", x))?;
        let (win_w, win_h) = self.window.drawable_size();
        let world_render_x = params.world_scale.x.min(1.0).max(1.0 / 32.0);
        let world_render_y = params.world_scale.y.min(1.0).max(1.0 / 32.0);
        let w = if world_render_x == 1.0 { win_w }
        else { (win_w as f32 * world_render_x + 0.5).floor() as u32 };
        let h = if world_render_y == 1.0 { win_h }
        else { (win_h as f32 * world_render_y + 0.5).floor() as u32 };
        self.world_is_undersampled = w != win_w || h != win_h;
        let num_samples = params.world_oversamples.min(5);
        let ms_bits = num_samples.min(self.max_multisample_power);
        let os_bits = num_samples - ms_bits;
        let x_os_bits = os_bits / 2;
        let y_os_bits = (os_bits + 1) / 2; // ??!
        let world_super_x = 1 << x_os_bits;
        let world_super_y = 1 << y_os_bits;
        self.world_super_x = world_super_x;
        self.world_super_y = world_super_y;
        let x_us_bits = params.bloom_undersamples / 2;
        let y_us_bits = (params.bloom_undersamples + 1) / 2;
        let bloom_under_x = 1 << x_us_bits;
        let bloom_under_y = 1 << y_us_bits;
        self.bloom_under_x = bloom_under_x;
        self.bloom_under_y = bloom_under_y;
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
                // we do this in case there was already a multisampled texture
                // and we create a non-multisampled one, or vice versa. we
                // don't want stale textures hanging around. (there are other
                // places here where that can happen but eh) -SB
                gl.DeleteTextures(1, &self.world_tex);
                gl.GenTextures(1, &mut self.world_tex);
                if self.world_samples > 1 {
                    gl.BindTexture(GL_TEXTURE_2D_MULTISAMPLE, self.world_tex);
                    gl.TexImage2DMultisample(GL_TEXTURE_2D_MULTISAMPLE,
                                             world_samples as GLint,GL_RGBA16F,
                                             world_w as GLint,world_h as GLint,
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
                else {
                    gl.BindTexture(GL_TEXTURE_2D, self.world_tex);
                    gl.TexImage2D(GL_TEXTURE_2D, 0,GL_RGBA16F as GLint,
                                             world_w as GLint,world_h as GLint,
                                             0,GL_RGBA,GL_HALF_FLOAT,null());
                    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                     GL_LINEAR as GLint);
                    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                     GL_LINEAR as GLint);
                    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                                     GL_CLAMP_TO_EDGE as GLint);
                    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                                     GL_CLAMP_TO_EDGE as GLint);
                    assertgl(gl, "creating multisampled float texture")?;
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_fb);
                    gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                            GL_COLOR_ATTACHMENT0,
                                            GL_TEXTURE_2D,
                                            self.world_tex, 0);
                    assertgl(gl, "creating multisampled framebuffer")?;
                    if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                    != GL_FRAMEBUFFER_COMPLETE {
                        return Err(anyhow!("world framebuffer wasn't \
                                            complete, but had no errors?!"))
                    }
                }
            }
        }
        let world_res_w = w * world_super_x;
        let world_res_h = h * world_super_y;
        if (self.world_res_w != world_res_w || self.world_res_h != world_res_h)
        && self.world_samples > 1 {
            debug!("recreating world-resolve framebuffer at {}x{}",
                   world_w, world_h);
            self.world_res_w = world_res_w;
            self.world_res_h = world_res_h;
            unsafe {
                gl.BindTexture(GL_TEXTURE_2D, self.world_res_tex);
                gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint,
                              world_res_w as GLint, world_res_h as GLint,
                              0, GL_RGBA, GL_HALF_FLOAT, null());
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                 GL_LINEAR as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                 GL_LINEAR as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                                 GL_CLAMP_TO_EDGE as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                                 GL_CLAMP_TO_EDGE as GLint);
                assertgl(gl, "creating float texture")?;
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_res_fb);
                gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                        GL_COLOR_ATTACHMENT0,
                                        GL_TEXTURE_2D,
                                        self.world_res_tex, 0);
                assertgl(gl, "creating float framebuffer")?;
                if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                    != GL_FRAMEBUFFER_COMPLETE {
                        return Err(anyhow!("world resolve framebuffer wasn't \
                                            complete, but had no errors?!"))
                }
            }
        }
        let world_ds_w = w;
        let world_ds_h = h;
        if (self.world_ds_w != world_ds_w || self.world_ds_h != world_ds_h)
        && (world_super_x > 1 || world_super_y > 1) {
            debug!("recreating world-downsample framebuffer at {}x{}",
                   world_ds_w, world_ds_h);
            self.world_ds_w = world_ds_w;
            self.world_ds_h = world_ds_h;
            unsafe {
                gl.BindTexture(GL_TEXTURE_2D, self.world_ds_tex);
                gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint,
                              world_ds_w as GLint, world_ds_h as GLint,
                              0, GL_RGBA, GL_HALF_FLOAT, null());
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                 GL_LINEAR as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                 GL_LINEAR as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                                 GL_CLAMP_TO_EDGE as GLint);
                gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                                 GL_CLAMP_TO_EDGE as GLint);
                assertgl(gl, "creating float texture")?;
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_ds_fb);
                gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                        GL_COLOR_ATTACHMENT0,
                                        GL_TEXTURE_2D,
                                        self.world_ds_tex, 0);
                assertgl(gl, "creating float framebuffer")?;
                if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                    != GL_FRAMEBUFFER_COMPLETE {
                        return Err(anyhow!("world resolve framebuffer wasn't \
                                            complete, but had no errors?!"))
                    }
                let buf = [
                    -1.0, -1.0, 0.0, 0.0,
                    1.0, -1.0, world_ds_w as f32, 0.0,
                    1.0, 1.0, world_ds_w as f32, world_ds_h as f32,
                    -1.0, 1.0, 0.0, world_ds_h as f32,
                ];
                gl.BindBuffer(GL_ARRAY_BUFFER, self.world_ds_vb);
                gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                              GL_STATIC_DRAW);
            }
        }
        if params.is_bloom_enabled() {
            let bloom_w = w / bloom_under_x;
            let bloom_h = h / bloom_under_y;
            if self.bloom_w != bloom_w || self.bloom_h != bloom_h {
                // note: bit wasteful, we make two framebuffers even if we're
                // not doing bloom... meh, we use practically no VRAM otherwise
                // so we're good!
                debug!("recreating bloom framebuffers at {}x{}",
                       bloom_w, bloom_h);
                self.bloom_w = bloom_w;
                self.bloom_h = bloom_h;
                unsafe {
                    for &prog in &[self.program_bloomx, self.program_bloomy] {
                        setup_uniforms(&gl, prog, "bloom", &[
                            (b"max_uv\0", &|gl, loc|
                             gl.Uniform2i(loc, bloom_w as GLint,
                                          bloom_h as GLint)),
                        ]);
                    }
                    self.bound_texture = None;
                    for pong in 0..2 {
                        gl.BindTexture(GL_TEXTURE_2D, self.bloom_tex[pong]);
                        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint,
                                      bloom_w as GLint, bloom_h as GLint,
                                      0, GL_RGB, GL_HALF_FLOAT, null());
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                         GL_LINEAR as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                         GL_LINEAR as GLint);
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
                        1.0, -1.0, bloom_w as f32, 0.0,
                        1.0, 1.0, bloom_w as f32, bloom_h as f32,
                        -1.0, 1.0, 0.0, bloom_h as f32,
                    ];
                    gl.BindBuffer(GL_ARRAY_BUFFER, self.bloom_vb);
                    gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                                  GL_STATIC_DRAW);
                }
            }
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
                    let left = 1.0 / 2.0 * PI * radius * radius;
                    for i in 0 .. samples {
                        let x = i as f32;
                        let right = -(x * x / (2.0 * radius * radius));
                        buf.push(E.powf(right) * left);
                    }
                    let total = buf.iter().fold(-buf[0], |a, &x| a+x*2.0);
                    for el in buf.iter_mut() {
                        *el /= total;
                    }
                    unsafe {
                        gl.BindTexture(GL_TEXTURE_1D,self.gauss_tex[axis]);
                        gl.TexImage1D(GL_TEXTURE_1D, 0,
                                      GL_R32F as GLint,
                                      samples as GLint,
                                      0, GL_RED, GL_FLOAT,
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
        let ui_sample_bits = params.ui_oversamples.min(5)
            .min(self.max_multisample_power);
        let ui_samples = 1 << ui_sample_bits;
        let ui_w = win_w;
        let ui_h = win_h;
        if self.ui_w != ui_w || self.ui_h != ui_h
        || self.ui_samples != ui_samples {
            debug!("recreating UI framebuffer at {}x{}x{}",
                   ui_w, ui_h, ui_samples);
            self.ui_w = ui_w;
            self.ui_h = ui_h;
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
            unsafe {
                let buf = [
                    -1.0, -1.0, 0.0, 0.0,
                    1.0, -1.0, ui_w as f32, 0.0,
                    1.0, 1.0, ui_w as f32, ui_h as f32,
                    -1.0, 1.0, 0.0, ui_h as f32,
                ];
                gl.BindBuffer(GL_ARRAY_BUFFER, self.ui_vb);
                gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                              GL_STATIC_DRAW);
            }
        }
        if ui_samples > 1 && !self.can_delinearize_with_blitframebuffer {
            let ui_res_w = win_w;
            let ui_res_h = win_h;
            if self.ui_res_w != ui_res_w || self.ui_res_h != ui_res_h {
                    debug!("recreating UI resolve framebuffer at {}x{}",
                           ui_res_w, ui_res_h);
                    self.ui_res_w = ui_res_w;
                    self.ui_res_h = ui_res_h;
                    unsafe {
                        gl.BindTexture(GL_TEXTURE_2D, self.ui_res_tex);
                        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint,
                                      ui_res_w as GLint, ui_res_h as GLint,
                                      0, GL_RGBA, GL_HALF_FLOAT, null());
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                                         GL_LINEAR as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                                         GL_LINEAR as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                                         GL_CLAMP_TO_EDGE as GLint);
                        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                                         GL_CLAMP_TO_EDGE as GLint);
                        assertgl(gl, "creating float texture")?;
                        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                                           self.ui_res_fb);
                        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER,
                                                GL_COLOR_ATTACHMENT0,
                                                GL_TEXTURE_2D,
                                                self.ui_res_tex, 0);
                        assertgl(gl, "creating float framebuffer")?;
                        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
                            != GL_FRAMEBUFFER_COMPLETE {
                                return Err(anyhow!("UI resolve framebuffer \
                                                    wasn't complete, but had \
                                                    no errors?!"))
                            }
                    }
                }
        }
        if self.world_final_w != w || self.world_final_h != h {
            self.world_final_w = w;
            self.world_final_h = h;
            unsafe {
                let buf = [
                    -1.0, -1.0, 0.0, 0.0,
                    1.0, -1.0, w as f32, 0.0,
                    1.0, 1.0, w as f32, h as f32,
                    -1.0, 1.0, 0.0, h as f32,
                ];
                gl.BindBuffer(GL_ARRAY_BUFFER, self.world_final_vb);
                gl.BufferData(GL_ARRAY_BUFFER, 64, transmute(&buf[0]),
                              GL_STATIC_DRAW);
            }
        }
        unsafe {
            gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_fb);
            gl.Viewport(0, 0, world_w as GLint, world_h as GLint);
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
        let world_src = self.resolve_world(params);
        // STAGE 2: Do bloom (optional)
        if params.is_bloom_enabled() {
            let (world_src_tex, _world_src_fb) = world_src
                .expect("If bloom is enabled, we expect to be told which \
                         texture to use! (logic error)");
            self.do_bloom(params, world_src_tex);
        }
        // The result of bloom is now waiting in the UI framebuffer (which,
        // again, might be the default framebuffer)
        unsafe {
            self.gl.BindFramebuffer(GL_READ_FRAMEBUFFER, 0);
            self.gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.which_ui_fb());
            self.gl.Viewport(0, 0, self.ui_w as GLint, self.ui_h as GLint);
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
                // (viewport will still be OK)
                if self.can_delinearize_with_blitframebuffer {
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
                    gl.BlitFramebuffer(0,0, self.ui_w as i32, self.ui_h as i32,
                                       0,0, self.ui_w as i32, self.ui_h as i32,
                                       GL_COLOR_BUFFER_BIT, GL_NEAREST);
                }
                else {
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.ui_res_fb);
                    gl.BlitFramebuffer(0,0, self.ui_w as i32, self.ui_h as i32,
                                       0,0, self.ui_w as i32, self.ui_h as i32,
                                       GL_COLOR_BUFFER_BIT, GL_NEAREST);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, 0);
                    gl.UseProgram(self.program_blit);
                    gl.BindTexture(GL_TEXTURE_2D, self.ui_res_tex);
                    gl.BindVertexArray(self.ui_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
            }
        }
        self.window.gl_swap_window();
    }
    fn get_size(&self) -> (u32, u32) {
        self.window.size()
    }
    fn resized(&mut self, _w: u32, _h: u32) -> anyhow::Result<()> {
        Ok(())
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
                gl.TexImage2D(GL_PROXY_TEXTURE_2D, 0, GL_RGB8 as GLint,
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
        // whose bounds are predictable. If `GL_UNPACK_ALIGNMENT` gets set to a
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
    fn purge_model(&mut self, model: &Model) {
        self.model_cache.purge(&self.gl, model);
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
            setup_uniforms(gl, self.program_model, "model", &[
                (b"transform\0", &|gl,loc| gl.UniformMatrix3fv(loc, 1, 0,
                    transform.into_inner().as_slice()
                    .as_ptr())),
                (b"opacity\0", &|gl,loc| gl.Uniform1f(loc, opacity)),
                (b"colors\0", &|gl,loc| {
                    for n in 0 .. model.colors.len() {
                        let color = color_overrides.get(n)
                            .unwrap_or(&model.colors[n]);
                        gl.Uniform4f(loc + (n as GLint),
                                     color.r, color.g, color.b, color.a);
                    }
                })
            ]);
            gl.DrawElements(GL_TRIANGLES, cached.num_elements,
                            GL_UNSIGNED_SHORT, null());
        }
    }
    fn get_window(&self) -> &Window {
        &self.window
    }
}

impl OpenGL32 {
    fn which_ui_fb(&self) -> GLuint {
        if self.ui_samples > 1 { self.ui_fb }
        else { 0 }
    }
    fn resolve_world(&mut self, params: &RenderParams)
        -> Option<(GLuint, GLuint)> {
        let gl = &self.gl;
        let ret;
        unsafe {
            // Okay. The questions that have an effect on our choices:
            let is_bloom_enabled = params.is_bloom_enabled();
            let is_resolve_needed = self.world_samples > 1;
            let is_downsample_needed
                = self.world_super_x > 1 || self.world_super_y > 1;
            let is_bwm_needed = params.world_mat != COLOR_IDENTITY_MATRIX;
            let may_blit_to_ui = self.can_delinearize_with_blitframebuffer
                || self.which_ui_fb() != 0;
            // If bloom is NOT enabled, AND there is no need for a BWM...
            if may_blit_to_ui && !is_bloom_enabled && !is_bwm_needed
                && self.world_super_x <= 2 && self.world_super_y <= 2
                && self.world_samples <= 1 {
                // ...then we can just blit directly to whichever UI
                // framebuffer and be done with it.
                // TODO: cut out world_fb if the pipeline gets really simple
                gl.BindFramebuffer(GL_READ_FRAMEBUFFER, self.world_fb);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                                   self.which_ui_fb());
                gl.Viewport(0, 0, self.ui_w as GLint, self.ui_h as GLint);
                let filter = if self.can_downsample_with_blit && (self.world_super_x > 1 || self.world_super_y > 1) { GL_LINEAR } else { GL_NEAREST };
                // ;)
                gl.BlitFramebuffer(0, 0, self.world_w as i32,
                                   self.world_h as i32,
                                   0, 0, self.ui_w as i32, self.ui_h as i32,
                                   GL_COLOR_BUFFER_BIT, filter);
                ret = None;
            }
            else {
                // Bloom is enabled OR we need a BWM. (Or we just aren't
                // allowed to use BlitFramebuffer to delinearize)
                let (world_src_tex, world_src_fb) = if is_resolve_needed {
                    gl.BindFramebuffer(GL_READ_FRAMEBUFFER, self.world_fb);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_res_fb);
                    gl.Viewport(0, 0, self.world_res_w as GLint,
                                self.world_res_h as GLint);
                    gl.BlitFramebuffer(0, 0, self.world_w as i32,
                                       self.world_h as i32,
                                       0, 0, self.world_res_w as i32,
                                       self.world_res_h as i32,
                                       GL_COLOR_BUFFER_BIT, GL_NEAREST);
                    (self.world_res_tex, self.world_res_fb)
                }
                else {
                    (self.world_tex, self.world_fb)
                };
                let (world_src_tex, world_src_fb) = if is_downsample_needed {
                    gl.UseProgram(self.program_downsample);
                    setup_uniforms(&gl, self.program_downsample,
                                   "downsampling", &[
                                       (b"dim\0", &|gl, loc|
                                        gl.Uniform3i(loc,
                                                     self.world_super_x as GLint,
                                                     self.world_super_y as GLint,
                                                     (self.world_super_x *
                                                      self.world_super_y)
                                                     as GLint)),
                                   ]);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.Viewport(0, 0, self.world_ds_w as i32,
                                self.world_ds_h as i32);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.world_ds_fb);
                    gl.BindVertexArray(self.world_ds_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT,null());
                    (self.world_ds_tex, self.world_ds_fb)
                }
                else {
                    (world_src_tex, world_src_fb)
                };
                if is_bloom_enabled {
                    ret = Some((world_src_tex, world_src_fb));
                }
                else {
                    ret = None;
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER,
                                       self.which_ui_fb());
                    gl.Viewport(0, 0, self.ui_w as GLint, self.ui_h as GLint);
                    if is_bwm_needed {
                        if self.world_is_undersampled {
                            gl.BindVertexArray(self.updog_vao);
                            gl.UseProgram(self.program_bwm_smooth);
                            setup_uniforms(&gl, self.program_bwm_smooth,
                                           "blit-with-matrix (smooth)", &[
                                (b"mat\0", &|gl, loc|
                                gl.UniformMatrix4x3fv(loc, 1, 0,
                                                       &params.world_mat[0]))
                                           ]);
                        }
                        else {
                            gl.BindVertexArray(self.world_final_vao);
                            gl.UseProgram(self.program_bwm);
                            setup_uniforms(&gl, self.program_bwm,
                                           "blit-with-matrix", &[
                                (b"mat\0", &|gl, loc|
                                 gl.UniformMatrix4x3fv(loc, 1, 0,
                                                       &params.world_mat[0]))
                                           ]);
                        }
                        gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                        gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT,null());
                    }
                    else if may_blit_to_ui {
                        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, world_src_fb);
                        let filter = if self.world_final_w != self.ui_w
                            || self.world_final_h != self.ui_h { GL_LINEAR }
                        else { GL_NEAREST };
                        gl.BlitFramebuffer(0, 0, self.world_final_w as GLint,
                                           self.world_final_h as GLint,
                                           0, 0, self.ui_w as GLint,
                                           self.ui_h as GLint,
                                           GL_COLOR_BUFFER_BIT, filter);
                    }
                    else {
                        if self.world_is_undersampled {
                            gl.BindVertexArray(self.updog_vao);
                            gl.UseProgram(self.program_blit_smooth);
                        }
                        else {
                            gl.BindVertexArray(self.world_final_vao);
                            gl.UseProgram(self.program_blit);
                        }
                        gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                        gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT,null());
                    }
                }
            }
        }
        #[cfg(debug_assertions)]
        assertgl(gl, "resolving the world").unwrap();
        ret
    }
    fn do_bloom(&mut self, params: &RenderParams, world_src_tex: GLuint) {
        // NOTE: blending is disabled when we get here
        let world_needs_bwm = params.world_mat != COLOR_IDENTITY_MATRIX;
        let bloom_needs_bwm = params.bloom_mat != COLOR_IDENTITY_MATRIX;
        let radii_same = params.bloom_radius.y
            == params.bloom_radius.x;
        let gl = &self.gl;
        unsafe {
            let mut src_tex = world_src_tex;
            let mut dst_fb = self.bloom_fb[0];
            gl.BindVertexArray(self.bloom_vao);
            gl.Viewport(0, 0, self.bloom_w as GLint, self.bloom_h as GLint);
            // apply the bloom matrix, first of all.
            if self.bloom_under_x > 1 || self.bloom_under_y > 1 {
                // TODO: downsample without matrix
                gl.UseProgram(self.program_downsample_wm);
                setup_uniforms(&gl, self.program_downsample_wm,
                               "downsampling with matrix", &[
                                   (b"mat\0", &|gl, loc|
                                    gl.UniformMatrix4x3fv(loc, 1, 0,
                                          &params.bloom_mat[0])),
                                   (b"dim\0", &|gl, loc|
                                    gl.Uniform3i(loc,
                                                 self.bloom_under_x as GLint,
                                                 self.bloom_under_y as GLint,
                                                 (self.bloom_under_x *
                                                  self.bloom_under_y)
                                                 as GLint)),
                ]);
                gl.BindTexture(GL_TEXTURE_2D, src_tex);
                gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, dst_fb);
                gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT,null());
                // TODO: wrap this operation in a type
                if dst_fb == self.bloom_fb[0] {
                    src_tex = self.bloom_tex[0];
                    dst_fb = self.bloom_fb[1];
                }
                else {
                    src_tex = self.bloom_tex[1];
                    dst_fb = self.bloom_fb[0];
                }
            }
            else {
                if bloom_needs_bwm {
                    gl.UseProgram(self.program_bwm);
                    setup_uniforms(&gl, self.program_bwm,
                                   "blit-with-matrix", &[
                        (b"mat\0", &|gl, loc|
                         gl.UniformMatrix4x3fv(loc, 1, 0,
                                               &params.bloom_mat[0])),
                                   ]);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, dst_fb);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT,null());
                    if dst_fb == self.bloom_fb[0] {
                        src_tex = self.bloom_tex[0];
                        dst_fb = self.bloom_fb[1];
                    }
                    else {
                        src_tex = self.bloom_tex[1];
                        dst_fb = self.bloom_fb[0];
                    }
                }
            }
            // now do the X iterations
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
                    src_tex = self.bloom_tex[1];
                    dst_fb = self.bloom_fb[0];
                }
            }
            // now do the Y iterations
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
                    src_tex = self.bloom_tex[1];
                    dst_fb = self.bloom_fb[0];
                }
            }
            gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, self.which_ui_fb());
            gl.Viewport(0, 0, self.ui_w as GLint, self.ui_h as GLint);
            // oh boy, let's have ALL THE CODE PATHS.
            if self.world_is_undersampled {
                if params.show_bloom_only {
                    gl.UseProgram(self.program_blit);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.BindVertexArray(self.bloom_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
                else if world_needs_bwm {
                    gl.UseProgram(self.program_upmergeup_wm);
                    setup_uniforms(&gl, self.program_upmergeup_wm,
                                   "upmergeup (with matrix)", &[
                        (b"mat1\0", &|gl, loc|
                         gl.UniformMatrix4x3fv(loc, 1, 0,
                                               &params.world_mat[0])),
                    ]);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.updog_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null())
                }
                else {
                    gl.UseProgram(self.program_upmergeup);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.updog_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null())
                }
            }
            else if params.bloom_undersamples > 0 {
                if params.show_bloom_only {
                    gl.UseProgram(self.program_blit);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.BindVertexArray(self.bloom_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
                else if world_needs_bwm {
                    gl.UseProgram(self.program_mergeup_wm);
                    setup_uniforms(&gl, self.program_mergeup_wm,
                                   "mergeup (with matrix)", &[
                        (b"mat1\0", &|gl, loc|
                         gl.UniformMatrix4x3fv(loc, 1, 0,
                                               &params.world_mat[0])),
                        (b"updog2\0", &|gl, loc| {
                            let x = (self.bloom_under_x * self.bloom_w) as f32;
                            let y = (self.bloom_under_y * self.bloom_h) as f32;
                         gl.Uniform4f(loc, 1.0 / x, 1.0 / y, 0.0, 0.0)
                        }),
                    ]);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.ui_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
                else {
                    gl.UseProgram(self.program_mergeup);
                    setup_uniforms(&gl, self.program_mergeup, "mergeup", &[
                        (b"updog2\0", &|gl, loc| {
                            let x = (self.bloom_under_x * self.bloom_w) as f32;
                            let y = (self.bloom_under_y * self.bloom_h) as f32;
                         gl.Uniform4f(loc, 1.0 / x, 1.0 / y, 0.0, 0.0)
                        }),
                    ]);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.ui_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
            }
            else {
                if params.show_bloom_only {
                    gl.UseProgram(self.program_blit);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.BindVertexArray(self.bloom_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
                else if world_needs_bwm {
                    gl.UseProgram(self.program_merge_wm);
                    setup_uniforms(&gl, self.program_merge_wm,
                                   "merge (with matrix)", &[
                        (b"mat1\0", &|gl, loc|
                         gl.UniformMatrix4x3fv(loc, 1, 0,
                                               &params.world_mat[0])),
                    ]);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.ui_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
                else {
                    gl.UseProgram(self.program_merge);
                    gl.ActiveTexture(GL_TEXTURE1);
                    gl.BindTexture(GL_TEXTURE_2D, src_tex);
                    gl.ActiveTexture(GL_TEXTURE0);
                    gl.BindTexture(GL_TEXTURE_2D, world_src_tex);
                    gl.BindVertexArray(self.ui_vao);
                    gl.DrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, null());
                }
            }
        }
    }
}

/// Tests for Mesa bug #4613. `true` = affected by the bug, can't ever disable
/// `GL_MULTISAMPLE`. `false` = unaffected, can disable `GL_MULTISAMPLE` when
/// doing a shader that doesn't benefit from it.
fn check_multisample_bug(gl: &Procs, program_model: GLuint)
    -> anyhow::Result<bool> {
    if env::var("TEST_FORCE_MULTISAMPLE").is_ok() {
        warn!("Saw TEST_FORCE_MULTISAMPLE in the environment. We will never disable GL_MULTISAMPLE!");
        return Ok(false)
    }
    const MULTISAMPLE_COVERAGE_TEST_BATCH: &[ModelVert] = &[
        ModelVert { x: -1.0, y: -1.0, c: 0 },
        ModelVert { x:  1.0, y: -1.0, c: 0 },
        ModelVert { x: -1.0, y:  1.0, c: 0 },
    ];
    let mut fbs = [0; 2];
    let mut texs = [0; 2];
    let mut vbs = [0; 1];
    let mut vaos = [0; 1];
    unsafe {
        // generate framebuffers, textures, VBO, VAO
        gl.GenFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.GenTextures(texs.len() as GLint, &mut texs[0]);
        gl.GenBuffers(vbs.len() as GLint, &mut vbs[0]);
        gl.GenVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        gl.BindVertexArray(vaos[0]);
        // framebuffer 0: multisampled
        gl.BindTexture(GL_TEXTURE_2D_MULTISAMPLE, texs[0]);
        gl.TexImage2DMultisample(GL_TEXTURE_2D_MULTISAMPLE, 2, GL_RGBA16F,
                                 16, 16, 0);
        assertgl(gl, "creating multisampled float texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D_MULTISAMPLE, texs[0], 0);
        assertgl(gl, "creating multisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("multisample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // framebuffer 1: ...unisampled?
        gl.BindTexture(GL_TEXTURE_2D, texs[1]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint, 16, 16, 0,
                      GL_RGBA, GL_FLOAT, null());
        assertgl(gl, "creating unisampled float texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, texs[1], 0);
        assertgl(gl, "creating unisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("multisample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // bind framebuffer 0, draw the thing
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.Viewport(0, 0, 16, 16);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbs[0]);
        gl.BufferData(GL_ARRAY_BUFFER, 36,
                      transmute(&MULTISAMPLE_COVERAGE_TEST_BATCH[0]),
                      GL_STATIC_DRAW);
        gl.UseProgram(program_model);
        setup_model_attribs(gl, program_model);
        // clear with green, draw with magenta
        gl.Disable(GL_MULTISAMPLE);
        gl.Clear(GL_COLOR_BUFFER_BIT);
        gl.ClearColor(0.0, 1.0, 0.0, 0.0);
        gl.DrawArrays(GL_TRIANGLES, 0, 3);
        gl.Enable(GL_MULTISAMPLE);
        // blit across
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[0]);
        gl.BlitFramebuffer(0, 0, 16, 16, 0, 0, 16, 16,
                           GL_COLOR_BUFFER_BIT, GL_NEAREST);
        // grab the lower left pixel
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[1]);
        let mut buf = [0.0f32; 3];
        gl.ReadPixels(0, 0, 1, 1, GL_RGB, GL_FLOAT, transmute(&mut buf[0]));
        // clean up
        gl.DeleteFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.DeleteTextures(texs.len() as GLint, &mut texs[0]);
        gl.DeleteBuffers(vbs.len() as GLint, &mut vbs[0]);
        gl.DeleteVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        assertgl(gl, "checking for the multisample bug")?;
        // correct answer is 1.0, 0.0, 1.0
        if &buf == &[1.0, 0.0, 1.0] {
            debug!("Didn't find Mesa bug #4613. Nice.");
            Ok(false)
        }
        else {
            warn!("Your video driver doesn't correctly support \
                   non-multisampled rendering into a multisampled \
                   framebuffer. Enabling a workaround, but performance will \
                   suffer slightly.");
            trace!("answer given was: {:?}", buf);
            Ok(true)
        }
    }
}

/// Tests whether `gl.BlitFramebuffer(..., GL_LINEAR)` can be used to do 2x or
/// 4x downsampling. This is preferred, when possible, because it uses a fixed-
/// function part of the video pipeline that is often going to be faster than
/// our generic downsample shader.
fn check_downsample_with_blit(gl: &Procs, program_blit: GLuint)
    -> anyhow::Result<bool> {
    if env::var("TEST_DISABLE_DOWNSAMPLE_WITH_BLIT").is_ok() {
        warn!("Saw TEST_DISABLE_DOWNSAMPLE_WITH_BLIT in the environment. We will pretend you can't downsample with BlitFramebuffer!");
        return Ok(false)
    }
    const SOURCE_TEXTURE: [f32; 12] = [
        1.0, 0.0, 0.0,
        0.0, 0.5, 0.0,
        0.0, 0.0, 0.25,
        0.0, 0.5, 0.25,
    ];
    const TEST_BATCH: [f32; 24] = [
        -1.0, -1.0, 0.0, 0.0,
         1.0, -1.0, 2.0, 0.0,
         1.0,  1.0, 2.0, 2.0,
         1.0,  1.0, 2.0, 2.0,
        -1.0,  1.0, 0.0, 2.0,
        -1.0, -1.0, 0.0, 0.0,
    ];
    let mut fbs = [0; 2];
    let mut texs = [0; 3];
    let mut vbs = [0; 1];
    let mut vaos = [0; 1];
    unsafe {
        // generate framebuffers, textures, VBO
        gl.GenFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.GenTextures(texs.len() as GLint, &mut texs[0]);
        gl.GenBuffers(vbs.len() as GLint, &mut vbs[0]);
        gl.GenVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        gl.BindVertexArray(vaos[0]);
        // framebuffer 0: unisampled 2x2
        gl.BindTexture(GL_TEXTURE_2D, texs[0]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint, 2, 2, 0,
                      GL_RGBA, GL_FLOAT, null());
        assertgl(gl, "creating unisampled float texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, texs[0], 0);
        assertgl(gl, "creating unisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("downsample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // framebuffer 1: unisampled 1x1
        gl.BindTexture(GL_TEXTURE_2D, texs[1]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint, 1, 1, 0,
                      GL_RGBA, GL_FLOAT, null());
        assertgl(gl, "creating unisampled float texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, texs[1], 0);
        assertgl(gl, "creating unisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("downsample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // the test texture
        gl.BindTexture(GL_TEXTURE_2D, texs[2]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint, 2, 2, 0,
                      GL_RGB, GL_FLOAT, transmute(&SOURCE_TEXTURE[0]));
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
                         GL_LINEAR as GLint);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
                         GL_LINEAR as GLint);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,
                         GL_CLAMP_TO_EDGE as GLint);
        gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,
                         GL_CLAMP_TO_EDGE as GLint);
        // bind framebuffer 0, draw the thing
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.Viewport(0, 0, 2, 2);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbs[0]);
        gl.BufferData(GL_ARRAY_BUFFER, 96,
                      transmute(&TEST_BATCH[0]),
                      GL_STATIC_DRAW);
        gl.UseProgram(program_blit);
        setup_attribs(&gl, program_blit, "blit", &[
            (b"pos\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 16,
                                    transmute(0usize))),
            (b"uv_in\0", &|gl, loc|
             gl.VertexAttribPointer(loc, 2, GL_FLOAT, 0, 16,
                                    transmute(8usize))),
        ]);
        gl.DrawArrays(GL_TRIANGLES, 0, 6);
        // blit down
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[0]);
        gl.BlitFramebuffer(0, 0, 2, 2, 0, 0, 1, 1,
                           GL_COLOR_BUFFER_BIT, GL_LINEAR);
        // grab the pixel
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[1]);
        let mut buf = [0.0f32; 3];
        gl.ReadPixels(0, 0, 1, 1, GL_RGB, GL_FLOAT, transmute(&mut buf[0]));
        // clean up
        gl.DeleteFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.DeleteTextures(texs.len() as GLint, &mut texs[0]);
        gl.DeleteBuffers(vbs.len() as GLint, &mut vbs[0]);
        gl.DeleteVertexArrays(vaos.len() as GLint, &mut vaos[0]);
        assertgl(gl, "checking for the linear framebuffer blit optimization")?;
        // correct answer is 0.25, 0.25, 0.125
        if &buf == &[0.25, 0.25, 0.125] {
            debug!("Downsampling with BlitFramebuffer is accurate. Nice.");
            Ok(false)
        }
        else {
            warn!("Your video driver doesn't correctly support downsampling \
                   via BlitFramebuffer. Performance will suffer slightly.");
            trace!("answer given was: {:?}", buf);
            Ok(true)
        }
    }
}

/// Tests whether `gl.BlitFramebuffer(...)` can be used to blit from a linear
/// framebuffer to an sRGB framebuffer without screwups.
fn check_blit_delinearization(gl: &Procs)
    -> anyhow::Result<bool> {
    if env::var("TEST_DISABLE_BLIT_DELINEARIZATION").is_ok() {
        warn!("Saw TEST_DISABLE_BLIT_DELINEARIZATION in the environment. We will pretend that blit delinearization never works!");
        return Ok(false)
    }
    let mut fbs = [0; 2];
    let mut texs = [0; 3];
    unsafe {
        // generate framebuffers, textures, VBO
        gl.GenFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.GenTextures(texs.len() as GLint, &mut texs[0]);
        // framebuffer 0: unisampled 1x1, linear
        gl.BindTexture(GL_TEXTURE_2D, texs[0]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGBA16F as GLint, 1, 1, 0,
                      GL_RGBA, GL_FLOAT, null());
        assertgl(gl, "creating unisampled float texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, texs[0], 0);
        assertgl(gl, "creating unisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("downsample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // framebuffer 1: unisampled 1x1, 32-bit
        gl.BindTexture(GL_TEXTURE_2D, texs[1]);
        gl.TexImage2D(GL_TEXTURE_2D, 0, GL_SRGB8_ALPHA8 as GLint, 1, 1, 0,
                      GL_RGBA, GL_UNSIGNED_BYTE, null());
        assertgl(gl, "creating unisampled 32-bit texture")?;
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.FramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0,
                                GL_TEXTURE_2D, texs[1], 0);
        assertgl(gl, "creating unisampled framebuffer")?;
        if gl.CheckFramebufferStatus(GL_DRAW_FRAMEBUFFER)
        != GL_FRAMEBUFFER_COMPLETE {
            return Err(anyhow!("downsample test framebuffer wasn't complete, \
                                but had no errors?!"))
        }
        // bind framebuffer 0, draw the thing
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[0]);
        gl.ClearColor(0.5, 0.5, 0.5, 0.5);
        gl.Clear(GL_COLOR_BUFFER_BIT);
        // blit down
        gl.BindFramebuffer(GL_DRAW_FRAMEBUFFER, fbs[1]);
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[0]);
        gl.BlitFramebuffer(0, 0, 2, 2, 0, 0, 1, 1,
                           GL_COLOR_BUFFER_BIT, GL_LINEAR);
        // grab the pixel
        gl.BindFramebuffer(GL_READ_FRAMEBUFFER, fbs[1]);
        let mut buf = [0.0f32; 4];
        gl.ReadPixels(0, 0, 1, 1, GL_RGBA, GL_FLOAT, transmute(&mut buf[0]));
        // clean up
        gl.DeleteFramebuffers(fbs.len() as GLint, &mut fbs[0]);
        gl.DeleteTextures(texs.len() as GLint, &mut texs[0]);
        assertgl(gl, "checking for BlitFramebuffer sRGB conversion")?;
        if buf[3] >= 0.49 && buf[3] <= 0.51 && buf[0] > 0.7 {
            debug!("We can use BlitFramebuffer to do sRGB conversion. Nice.");
            Ok(true)
        }
        else {
            warn!("Your video driver doesn't support sRGB conversion in \
                   BlitFramebuffer. Performance will suffer slightly.");
            trace!("answer given was: {:?}", buf);
            Ok(false)
        }
    }
}
