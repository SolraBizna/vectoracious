use super::*;

use std::{
    mem::{size_of, transmute},
    ptr::{null, null_mut},
};

use anyhow::anyhow;
use log::warn;
use sdl2::{
    video::{GLContext, GLProfile, Window, WindowBuilder},
    VideoSubsystem,
};

mod binding;
use binding::*;

/// Renders using OpenGL 3.0 Core.
struct OpenGL30 {
    window: Window,
    ctx: GLContext,
    gl: Procs,
}

const BUF_SIZE: usize = 262128; // 256KiB at a time, exactly 16383 elements
// ^^^ if you change this, make sure it's a multiple of 48! ^^^
// it needs to contain an integer number of entire triangles!

const VERTEX_TEXT: &[u8] = include_bytes!("gl32/vert.glsl");
const FRAGMENT_TEXT: &[u8] = include_bytes!("gl32/frag.glsl");

/// Check for OpenGL errors. If there were any, complain.
fn assertgl(gl: &Procs, wo: &str) -> anyhow::Result<()> {
    let mut errors = vec![];
    loop {
        // Unsafe justification: glGetError is safe to call.
        let e = unsafe { gl.GetError() };
        if e == 0 { break }
        errors.push(e);
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
        Ok(program)
    }
}

/// Set up an OpenGL 3.0 rendering context.
pub fn create(video: &VideoSubsystem, mut builder: WindowBuilder)
    -> anyhow::Result<Box<dyn Renderer>> {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    // TODO: antialiasing
    gl_attr.set_depth_size(0);
    gl_attr.set_framebuffer_srgb_compatible(true);
    let window = builder.opengl().allow_highdpi().build()
        .map_err(|x| anyhow!("Unable to create window for OpenGL 3.0 \
                              context: {}", x))?;
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
    let mut vbo: GLuint = 0;
    unsafe {
        // Compile and link the shaders... oh boy.
        let vshader = compile_shader(&gl, "the vertex shader",
                                     GL_VERTEX_SHADER, VERTEX_TEXT)?;
        let fshader = compile_shader(&gl, "the fragment shader",
                                     GL_FRAGMENT_SHADER, FRAGMENT_TEXT)?;
        let program = link_program(&gl, "the shader program",
                                   &[vshader, fshader])?;
        gl.UseProgram(program);
        gl.DeleteShader(vshader);
        gl.DeleteShader(fshader);
        // Generate a buffer object to use to stream the vertices
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
        // Set up the array state for our interleaved `X___Y___R_G_B_A_` scheme
        // Note: these are not real pointers!
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 2, GL_FLOAT, 0, 16,
                               transmute(0usize));
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1, 4, GL_HALF_FLOAT, 0, 16,
                               transmute(8usize));
        // Do linear-to-sRGB compression before writing to the framebuffer
        gl.Enable(GL_FRAMEBUFFER_SRGB);
        // Set up blending for premultiplied alpha
        gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
    } assertgl(&gl, "initializing the context")?;
    Ok(Box::new(OpenGL30 { window, ctx, gl }))
}

impl Renderer for OpenGL30 {
    fn start_rendering(&mut self) -> anyhow::Result<()> {
        self.window.gl_make_current(&self.ctx)
            .map_err(|x| anyhow!("OpenGL context lost! {}", x))
    }
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let gl = &self.gl;
        // Unsafe justification: Only primitive data are sent, and these
        // functions cannot error.
        unsafe {
            gl.ClearColor(r, g, b, a);
            gl.Clear(GL_COLOR_BUFFER_BIT);
        }
    }
    fn render_batch(&mut self, batch: &[BatchVert]) {
        let gl = &self.gl;
        // Unsafe justification: Passing in data. Every time we have data to
        // pass in, we are passing a slice. We provide the slice pointer and
        // its size-converted length at every turn. We check errors at
        // completion.
        unsafe {
            // assertion: our vbo is still bound
            for seg in batch.chunks(BUF_SIZE / size_of::<BatchVert>()) {
                let chunksize
                    = (seg.len() * size_of::<BatchVert>()) as GLsizeiptr;
                gl.BufferData(GL_ARRAY_BUFFER, chunksize,
                              null(), GL_STREAM_DRAW);
                gl.BufferSubData(GL_ARRAY_BUFFER, 0,
                                 chunksize, transmute(seg.as_ptr()));
                gl.DrawArrays(GL_TRIANGLES, 0, seg.len() as GLint);
            }
        } assertgl(gl, "rendering a batch").unwrap();
    }
    fn present(&mut self) {
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
}
