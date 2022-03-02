use super::*;

use std::{
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

#[derive(Debug,PartialEq,Eq)]
enum LastBatchType {
    None, Model, Text
}

/// Renders using OpenGL 3.0 Core.
struct OpenGL32 {
    window: Window,
    ctx: GLContext,
    gl: Procs,
    last_batch_type: LastBatchType,
    program_model: GLuint,
    program_text: GLuint,
    vaos: [GLuint; 2],
    bound_texture: Option<GLuint>,
    force_multisample: bool,
}

const BUF_SIZE: usize = 262128; // 256KiB at a time, exactly 16383 elements
// ^^^ if you change this, make sure it's a multiple of 48! ^^^
// it needs to contain an integer number of entire triangles!

const MULTISAMPLE_COVERAGE_TEST_BATCH: &[BatchModelVert] = &[
    BatchModelVert { x: -1.0, y: -1.0,
                     r: f16::ONE, g: f16::ZERO, b: f16::ONE, a: f16::ONE },
    BatchModelVert { x: 1.0, y: -1.0,
                     r: f16::ONE, g: f16::ZERO, b: f16::ONE, a: f16::ONE },
    BatchModelVert { x: -1.0, y: 1.0,
                     r: f16::ONE, g: f16::ZERO, b: f16::ONE, a: f16::ONE },
];

const VERTEX_TEXT: &[u8] = include_bytes!("gl32/vert.glsl");
const MODEL_FRAGMENT_TEXT: &[u8] = include_bytes!("gl32/model.glsl");
const TEXT_FRAGMENT_TEXT: &[u8] = include_bytes!("gl32/text.glsl");

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
pub fn create<F>(video: &VideoSubsystem, builder_maker: &mut F)
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
    let mut vbo: GLuint = 0;
    let mut vaos = [0; 2];
    let force_multisample;
    let (program_model, program_text);
    unsafe {
        // Compile and link the shaders... oh boy.
        let vshader = compile_shader(&gl, "the vertex shader",
                                     GL_VERTEX_SHADER, VERTEX_TEXT)?;
        let fshader_model = compile_shader(&gl, "the model fragment shader",
                                           GL_FRAGMENT_SHADER,
                                           MODEL_FRAGMENT_TEXT)?;
        let fshader_text = compile_shader(&gl, "the text fragment shader",
                                          GL_FRAGMENT_SHADER,
                                          TEXT_FRAGMENT_TEXT)?;
        program_model = link_program(&gl, "the model shader program",
                                     &[vshader, fshader_model])?;
        program_text = link_program(&gl, "the text shader program",
                                    &[vshader, fshader_text])?;
        gl.DeleteShader(vshader);
        gl.DeleteShader(fshader_model);
        gl.DeleteShader(fshader_text);
        // Generate a buffer object to use to stream the vertices
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
        // Generate two VAOs, one for rendering model, one for rendering text
        gl.GenVertexArrays(2, &mut vaos[0]);
        // Model: X___Y___R_G_B_A_
        gl.BindVertexArray(vaos[0]);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 2, GL_FLOAT, 0, 16,
                               transmute(0usize));
        gl.EnableVertexAttribArray(2);
        gl.VertexAttribPointer(2, 4, GL_HALF_FLOAT, 0, 16,
                               transmute(8usize));
        // Text: X_Y_U_V_R1G1B1A1R2G2B2A2
        gl.BindVertexArray(vaos[1]);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 2, GL_HALF_FLOAT, 0, 24,
                               transmute(0usize));
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1, 2, GL_HALF_FLOAT, 0, 24,
                               transmute(4usize));
        gl.EnableVertexAttribArray(2);
        gl.VertexAttribPointer(2, 4, GL_HALF_FLOAT, 0, 24,
                               transmute(8usize));
        gl.EnableVertexAttribArray(3);
        gl.VertexAttribPointer(3, 4, GL_HALF_FLOAT, 0, 24,
                               transmute(16usize));
        // (Note: the Xusize values are being shoved into a pointer parameter,
        // but since we're using VBOs, the pointers aren't really pointers, but
        // offsets into the VBO!)
        // Do linear-to-sRGB compression before writing to the framebuffer and
        // decompression after reading (for blending)
        gl.Enable(GL_FRAMEBUFFER_SRGB);
        // Set up blending for premultiplied alpha
        gl.BlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
        // We're gonna be uploading a lot of unaligned pixel data, yuck.
        gl.PixelStorei(GL_UNPACK_ALIGNMENT, 1);
        // Oh, and let's set the uniform!
        gl.UseProgram(program_text);
        let loc = gl.GetUniformLocation(program_text,
                                        transmute(b"atlas\0"));
        if loc != -1 {
            gl.Uniform1i(loc, 0); // texture unit 0
        }
        // Before we're done, check for Mesa bug #4613.
        // We have to do this check, and otherwise assume multisampling is in
        // use, in case somebody uses their vendor's special control panel to
        // force multisampling to be used.
        gl.ClearColor(0.0, 1.0, 0.0, 0.0);
        gl.Clear(GL_COLOR_BUFFER_BIT);
        gl.BindVertexArray(vaos[0]);
        gl.UseProgram(program_model);
        gl.Disable(GL_MULTISAMPLE);
        gl.BufferData(GL_ARRAY_BUFFER,
                      (size_of::<BatchModelVert>()*3) as GLsizeiptr,
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
    } assertgl(&gl, "initializing the context")?;
    Ok(Box::new(OpenGL32 {
        window, ctx, gl, last_batch_type: LastBatchType::None,
        program_model, program_text, bound_texture: None, vaos,
        force_multisample,
    }))
}

impl Renderer for OpenGL32 {
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
    fn render_model_batch(&mut self, batch: &[BatchModelVert]) {
        let gl = &self.gl;
        // Unsafe justification: Passing in data. Every time we have data to
        // pass in, we are passing a slice. We provide the slice pointer and
        // its size-converted length at every turn. We check errors at
        // completion.
        unsafe {
            if self.last_batch_type != LastBatchType::Model {
                gl.BindVertexArray(self.vaos[0]);
                gl.UseProgram(self.program_model);
                if !self.force_multisample {
                    gl.Enable(GL_MULTISAMPLE);
                }
                gl.Disable(GL_TEXTURE_2D);
                self.last_batch_type = LastBatchType::Model;
                assertgl(gl, "switching to the model shader").unwrap();
            }
            // assertion: our vbo is still bound
            for seg in batch.chunks(BUF_SIZE / size_of::<BatchModelVert>()) {
                let chunksize
                    = (seg.len() * size_of::<BatchModelVert>()) as GLsizeiptr;
                gl.BufferData(GL_ARRAY_BUFFER, chunksize,
                              null(), GL_STREAM_DRAW);
                gl.BufferSubData(GL_ARRAY_BUFFER, 0,
                                 chunksize, transmute(seg.as_ptr()));
                gl.DrawArrays(GL_TRIANGLES, 0, seg.len() as GLint);
            }
        } assertgl(gl, "rendering a model batch").unwrap();
    }
    fn render_text_batch(&mut self, atlas: u32, batch: &[BatchTextVert]) {
        let gl = &self.gl;
        // Unsafe justification: see render_model_batch
        // addendum: we assume that `atlas` is a valid texture ID, but if it
        // isn't, an OpenGL error will be raised.
        unsafe {
            if self.last_batch_type != LastBatchType::Text {
                gl.BindVertexArray(self.vaos[1]);
                gl.UseProgram(self.program_text);
                if !self.force_multisample {
                    gl.Disable(GL_MULTISAMPLE);
                }
                gl.Enable(GL_TEXTURE_2D);
                self.last_batch_type = LastBatchType::Text;
                assertgl(gl, "switching to the text shader").unwrap();
            }
            gl.BindTexture(GL_TEXTURE_2D, atlas);
            // assertion: our vbo is still bound
            for seg in batch.chunks(BUF_SIZE / size_of::<BatchTextVert>()) {
                let chunksize
                    = (seg.len() * size_of::<BatchTextVert>()) as GLsizeiptr;
                gl.BufferData(GL_ARRAY_BUFFER, chunksize,
                              null(), GL_STREAM_DRAW);
                gl.BufferSubData(GL_ARRAY_BUFFER, 0,
                                 chunksize, transmute(seg.as_ptr()));
                gl.DrawArrays(GL_QUADS, 0, seg.len() as GLint);
            }
        } assertgl(gl, "rendering a text batch").unwrap();
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
            // if it's small, double it
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
                if got_width == max_size {
                    return max_size as u32;
                }
                max_size /= 2;
            }
        } assertgl(gl, "checking the maximum texture size").unwrap();
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
            gl.TexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint,
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
}
