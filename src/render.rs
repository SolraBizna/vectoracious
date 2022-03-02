use crate::*;

use sdl2::{
    VideoSubsystem, video::WindowBuilder
};

mod gl32;

#[repr(C)]
#[derive(Debug)]
pub struct BatchModelVert {
    pub x: f32, pub y: f32,
    pub r: f16, pub g: f16, pub b: f16, pub a: f16,
}

#[repr(C)]
#[derive(Debug)]
// note: YUCK! In OpenGL 4+ we could use glVertexAttribDivisor...
pub struct BatchTextVert {
    pub x: f16, pub y: f16, pub u: f16, pub v: f16,
    // fill color
    pub fill_r: f16, pub fill_g: f16, pub fill_b: f16, pub fill_a: f16,
    // stroke color
    pub stroke_r: f16, pub stroke_g: f16, pub stroke_b: f16, pub stroke_a: f16,
}

pub trait Renderer {
    /// Call this before calling `clear`, `render_batch`, or `present`.
    fn start_rendering(&mut self) -> anyhow::Result<()>;
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);
    fn render_model_batch(&mut self, batch: &[BatchModelVert]);
    fn render_text_batch(&mut self, atlas: u32, batch: &[BatchTextVert]);
    fn enable_blend(&mut self);
    fn disable_blend(&mut self);
    /// Returns the size (width, height) of the atlases this renderer will use
    /// to render text. This may be an expensive operation; it will be called
    /// only once per `TextBatch` initialization, which should be once per
    /// program run.
    fn get_text_atlas_size(&mut self) -> u32;
    /// `atlas_size` should be a value returned from `get_text_atlas_size`
    fn new_text_atlas(&mut self, atlas_size: u32) -> anyhow::Result<u32>;
    fn new_text_glyph(&mut self, atlas: u32,
                      glyph_x: u32, glyph_y: u32, glyph_w: u32, glyph_h: u32,
                      pixels: &[u8]) -> anyhow::Result<()>;
    /// Call this when you're done rendering the frame.
    fn present(&mut self);
    fn get_size(&self) -> (u32, u32);
    #[allow(unused_variables)]
    fn resize(&self, w: u32, h: u32) -> Result<(),()> { Err(()) }
}

pub fn create_renderer<F>(video: &VideoSubsystem, mut builder_maker: F)
    -> anyhow::Result<Box<dyn Renderer>>
where F: FnMut() -> WindowBuilder
{
    gl32::create(video, &mut builder_maker)
}
