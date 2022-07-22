use std::{
    ops::{Deref,DerefMut},
    rc::Rc,
    sync::Arc,
};

use half::f16;
use rustybuzz::Face;
use sdl2::video::WindowContext;
use sdl2_sys::SDL_Window;

mod color;
pub use color::*;
mod model;
pub use model::*;
mod batch;
pub(crate) use batch::*;
mod text;
pub(crate) use text::*;
mod render;
pub(crate) use render::*;

pub type Vector = nalgebra::Vector2<f32>;
pub type Point = nalgebra::Point2<f32>;
pub type Transform = nalgebra::Transform2<f32>;
pub type Rotation = nalgebra::Rotation2<f32>;
pub type Scale = nalgebra::Scale2<f32>;
pub type Translation = nalgebra::Translation2<f32>;
pub type ColorMatrix = [f32; 12];

pub const COLOR_IDENTITY_MATRIX: ColorMatrix = [
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, 0.0, 0.0,
];

#[derive(Clone,Copy,Debug)]
pub struct RenderParams {
    /// How many effective oversamples are desired, when rendering the world,
    /// in units of powers of two. Thus, 0 = 1x, 1 = 2x, 2 = 4x, 3 = 8x, etc.
    /// The default is 2, i.e. 4x oversampling.
    ///
    /// The renderer will try to achieve this through hardware multisampling,
    /// and will make up the difference with software (shader-based)
    /// supersampling. There may be an implementation-specific limit to the
    /// number of achievable oversamples, and that limit may be zero!
    ///
    /// Values above 5 are both costly and pointless.
    pub world_oversamples: u32,
    /// How many effective oversamples are desired, when rendering the UI, in
    /// units of powers of two. This is much like `world_oversamples`, except
    /// that the renderer will *not* make up the difference with supersampling.
    /// Only hardware multisampling will be used.
    ///
    /// The default is 0, on the assumption that your UI will only contain text
    /// (which doesn't benefit from multisampling) and straight edges (which
    /// actively look worse when multisampled). As an added bonus, having zero
    /// multisamples will be more efficient in most renderers, since it will
    /// mean that the rendering can take place directly in the default
    /// framebuffer and not in multisampled la-la land.
    pub ui_oversamples: u32,
    /// Whether bloom is enabled at all. (In order to be *actually* enabled,
    /// `bloom_radius` and `bloom_iterations` must be nonzero.)
    /// Bloom is enabled by default.
    pub bloom_enabled: bool,
    /// Number of *undersamples* desired when rendering the bloom, in units of
    /// powers of two. 0 = the bloom will be carried out at full resolution.
    /// 1 = halved in one direction, 2 = halved in both directions, and so on.
    ///
    /// Default: 2
    pub bloom_undersamples: u32,
    /// Color matrix to multiply by before applying the bloom blur. Note that
    /// alpha is fixed at one, so you can specify an offset here.
    ///
    /// The default is to multiply color by 4 but also subtract 3, so that
    /// only colors brighter than 75% will bloom, and 100% will bloom quite
    /// brightly.
    pub bloom_mat: ColorMatrix,
    /// The number of pixels for one standard deviation of blur. Default is 4,
    /// which in combination with the default bloom_iterations results in an
    /// effective blur radius of 32 "pixels"
    ///
    /// A `bloom_radius` of N results in `(N*2+1)*6` samples per pixel (because
    /// we sample out to 3 standard deviations, that number might be 3 times
    /// higher than you expect)
    pub bloom_radius: Vector,
    /// The number of times to iterate the Gaussian blur. Default is 4.
    ///
    /// Weird DSP note: Applying an N-radius Gaussian blur M times is exactly
    /// the same as applying an N*M-radius Gaussian blur, but potentially a lot
    /// more efficient.
    pub bloom_iterations: [u32; 2],
    /// A color matrix to apply to the world pixels after they're drawn to the
    /// screen. (This won't include the bloom.)
    ///
    /// The default is identity, i.e. the world pixels are drawn without
    /// modification.
    pub world_mat: ColorMatrix,
    /// If bloom is enabled, this controls whether to draw only the bloom to
    /// the screen. (Good for tweaking how bloom affects your scene, I guess.)
    pub show_bloom_only: bool,
    /// If the given dimension of this vector is less than one, render at a
    /// lower resolution on that axis. e.g. {0.5, 0.25} will render half
    /// resolution horizontally, one quarter resolution vertically.
    ///
    /// Default is, naturally, {1.0, 1.0}.
    pub world_scale: Vector,
}

impl Default for RenderParams {
    fn default() -> RenderParams {
        RenderParams {
            world_oversamples: 2,
            ui_oversamples: 0,
            bloom_enabled: true,
            bloom_undersamples: 2,
            bloom_mat: [
                4.0, 0.0, 0.0,
                0.0, 4.0, 0.0,
                0.0, 0.0, 4.0,
                -3.0, -3.0, -3.0,
            ],
            bloom_radius: Vector::new(4.0, 4.0),
            bloom_iterations: [4, 4],
            world_mat: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 0.0,
            ],
            show_bloom_only: false,
            world_scale: Vector::new(1.0, 1.0),
        }
    }
}

impl RenderParams {
    pub fn new() -> RenderParams { Default::default() }
    /// Returns true if bloom is enabled *and* there are no nonsense values.
    pub fn is_bloom_enabled(&self) -> bool {
        self.bloom_enabled
            && (self.bloom_radius.x >= 1.0 || self.bloom_radius.y >= 1.0)
            && (self.bloom_iterations[0] > 0 || self.bloom_iterations[1] > 0)
    }
}

pub struct Context {
    renderer: Box<dyn Renderer>,
    text_handler: TextHandler,
    /// The render parameters that will be used in the next render. Sensible
    /// defaults are provided. Set individual fields to obtain desired results.
    ///
    /// Restore defaults with `render_params = Default::default()`
    pub render_params: RenderParams,
}

pub struct Render<'a> {
    ctx: &'a mut Context,
    text_batch: Option<TextBatch>,
}

pub struct RenderWorld<'a>(Render<'a>);
pub struct RenderUI<'a>(Render<'a>);

impl<'a> Deref for RenderWorld<'a> {
    type Target = Render<'a>;
    fn deref(&'_ self) -> &Render<'a> { &self.0 }
}

impl<'a> Deref for RenderUI<'a> {
    type Target = Render<'a>;
    fn deref(&'_ self) -> &Render<'a> { &self.0 }
}

impl<'a> DerefMut for RenderWorld<'a> {
    fn deref_mut(&'_ mut self) -> &mut Render<'a> { &mut self.0 }
}

impl<'a> DerefMut for RenderUI<'a> {
    fn deref_mut(&'_ mut self) -> &mut Render<'a> { &mut self.0 }
}

impl Context {
    pub fn initialize<F>(video: &sdl2::VideoSubsystem, builder_maker: F) -> anyhow::Result<Context>
    where
        F: FnMut() -> sdl2::video::WindowBuilder,
    {
        let mut renderer = create_renderer(video, builder_maker)?;
        let text_handler = TextHandler::new(&mut renderer);
        Ok(Context {
            renderer,
            text_handler,
            render_params: Default::default(),
        })
    }
    /// Returns a reference to the `WindowContext` of the underlying `Window`,
    /// and a pointer to the underlying `SDL_Window` struct. You must make sure
    /// that you only do safe things to this window, and that you only do them
    /// while the `WindowContext` yet lives!
    pub fn get_window_context(&self) -> (Rc<WindowContext>, *mut SDL_Window) {
        self.renderer.get_window_context()
    }
    #[deprecated]
    pub fn set_render_params(&mut self, params: &RenderParams) {
        self.render_params = *params
    }
    pub fn begin_rendering_world(&mut self) -> anyhow::Result<RenderWorld> {
        self.renderer.begin_rendering(&self.render_params)?;
        Ok(RenderWorld(Render { ctx: self, text_batch: None }))
    }
    pub fn add_face(
        &mut self,
        face_data: Arc<Vec<u8>>,
        index: u32,
        border_texels: f32,
        texels_per_em_x: f32,
        texels_per_em_y: f32,
    ) -> Option<usize> {
        self.text_handler.add_face(
            face_data,
            index,
            border_texels,
            texels_per_em_x,
            texels_per_em_y,
        )
    }
    pub fn get_face(&self, i: usize) -> Option<&Face> {
        self.text_handler.get_face(i)
    }
    pub fn get_face_mut(&mut self, i: usize) -> Option<&mut Face> {
        self.text_handler.get_face_mut(i)
    }
    fn finish_world(&mut self) {
        self.renderer.finish_world(&self.render_params);
    }
    pub fn resized(&mut self, w: u32, h: u32) -> anyhow::Result<()> {
        self.renderer.resized(w, h)
    }
    pub fn purge_model(&mut self, model: &Model) {
        self.renderer.purge_model(model);
    }
}

impl Render<'_> {
    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.ctx.renderer.clear(r, g, b, a);
    }
    pub fn disable_blend(&mut self) {
        self.flush();
        self.ctx.renderer.disable_blend();
    }
    pub fn enable_blend(&mut self) {
        self.flush();
        self.ctx.renderer.enable_blend();
    }
    pub fn flush(&mut self) {
        match self.text_batch.take() {
            None => (),
            Some(b) => {
                self.ctx.renderer.render_text_batch(self.ctx.text_handler.get_atlases(), b);
            },
        }
    }
    pub fn model(&mut self, model: &Model, transform: &Transform,
                 color_overrides: &[Color], opacity: f32) {
        self.flush(); // only if it's text? TODO or something
        self.ctx.renderer.render_model(model, transform, color_overrides,
                                       opacity)
    }
    pub fn text(&mut self,
                face_id: u32, fill_color: Color, stroke_color: Color,
                x_align: f32, rtl: bool,
                transform: &Transform, text: &str) {
        if self.text_batch.is_none() {
            self.text_batch = Some(self.ctx.renderer.new_text_batch());
        }
        match &mut self.text_batch {
            Some(TextBatch::Merged(x)) => {
                self.ctx.text_handler.batch_text(&mut self.ctx.renderer,
                                                 x, face_id, fill_color,
                                                 stroke_color, x_align, rtl,
                                                 transform, text);
            },
            Some(TextBatch::Split(x)) => {
                self.ctx.text_handler.batch_text(&mut self.ctx.renderer,
                                                 x, face_id, fill_color,
                                                 stroke_color, x_align, rtl,
                                                 transform, text);
            },
            None => unreachable!(),
        };
    }
    pub(crate) fn end_rendering(mut self) {
        self.flush();
        self.ctx.renderer.present();
    }
    pub fn get_size(&self) -> (u32, u32) {
        self.ctx.renderer.get_size()
    }
    pub fn purge_model(&mut self, model: &Model) {
        self.ctx.renderer.purge_model(model);
    }
}

impl<'a> RenderWorld<'a> {
    pub fn begin_ui(mut self) -> RenderUI<'a> {
        self.ctx.finish_world();
        RenderUI(self.0)
    }
    pub fn end_rendering(self) {
        self.begin_ui().end_rendering()
    }
}

impl<'a> RenderUI<'a> {
    pub fn end_rendering(self) {
        self.0.end_rendering()
    }
}
