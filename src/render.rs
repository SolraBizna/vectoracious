use crate::*;

use sdl2::{
    VideoSubsystem, video::WindowBuilder
};

mod gl32;
mod glerr;

pub(crate) trait Renderer {
    /// Called to enter the rendering state.
    fn begin_rendering(&mut self, params: &RenderParams) -> anyhow::Result<()>;
    /// Called exactly once after `begin_rendering` and before `present`.
    fn finish_world(&mut self, params: &RenderParams);
    /// Called while rendering.
    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32);
    /// Called while rendering.
    fn render_model(&mut self, model: &Model,
                    transform: &Transform, color_overrides: &[Color],
                    opacity: f32);
    // These next two functions will be replaced!
    /// Called while rendering.
    fn new_text_batch(&mut self) -> TextBatch;
    /// Called while rendering.
    fn render_text_batch(&mut self, atlases: &[u32],
                         batch: TextBatch);
    /// Won't be called if there is an open model batch.
    /// Called while rendering.
    fn enable_blend(&mut self);
    /// Called while rendering.
    fn disable_blend(&mut self);
    /// Returns the size (width, height) of the atlases this renderer will use
    /// to render text. This may be an expensive operation; it will be called
    /// only once per `TextBatch` initialization, which should be once per
    /// program run.
    ///
    /// Could be called either during rendering or not during rendering.
    fn get_text_atlas_size(&mut self) -> u32;
    /// `atlas_size` should be the value returned from `get_text_atlas_size`
    ///
    /// Called while rendering.
    fn new_text_atlas(&mut self, atlas_size: u32) -> anyhow::Result<u32>;
    /// Called while rendering.
    fn new_text_glyph(&mut self, atlas: u32,
                      glyph_x: u32, glyph_y: u32, glyph_w: u32, glyph_h: u32,
                      pixels: &[u8]) -> anyhow::Result<()>;
    /// Called when finished rendering.
    fn present(&mut self);
    /// Return the current drawable size.
    fn get_size(&self) -> (u32, u32);
    /// Attempt to resize the underlying drawable when the window size is
    /// reported as changed.
    fn resized(&mut self, w: u32, h: u32) -> anyhow::Result<()>;
}

pub(crate) fn create_renderer<F>(video: &VideoSubsystem, mut builder_maker: F)
    -> anyhow::Result<Box<dyn Renderer>>
where F: FnMut() -> WindowBuilder
{
    gl32::create(video, &mut builder_maker)
}

