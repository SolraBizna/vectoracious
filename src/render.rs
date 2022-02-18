use crate::*;

use sdl2::{
    VideoSubsystem, video::WindowBuilder
};

mod gl32;

#[repr(C)]
#[derive(Debug)]
pub struct BatchVert {
    pub x: f32, pub y: f32,
    pub r: f16, pub g: f16, pub b: f16, pub a: f16,
}

pub trait Renderer {
    /// Call this before calling `clear`, `render_batch`, or `present`.
    fn start_rendering(&mut self) -> anyhow::Result<()>;
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);
    fn render_batch(&mut self, batch: &[BatchVert]);
    fn enable_blend(&mut self);
    fn disable_blend(&mut self);
    /// Call this when you're done rendering the frame.
    fn present(&mut self);
    fn get_size(&self) -> (u32, u32);
    #[allow(unused_variables)]
    fn resize(&self, w: u32, h: u32) -> Result<(),()> { Err(()) }
}

pub fn create_renderer(video: &VideoSubsystem, builder: WindowBuilder)
    -> anyhow::Result<Box<dyn Renderer>> {
    gl32::create(video, builder)
}
