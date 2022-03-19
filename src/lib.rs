use std::{
    rc::Rc,
};

use half::f16;

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

pub struct Context {
    renderer: Box<dyn Renderer>,
    text_handler: TextHandler,
}

pub struct Render<'a> {
    ctx: &'a mut Context,
    text_batch: Option<TextBatch>,
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
        })
    }
    pub fn begin_render(&mut self) -> anyhow::Result<Render> {
        self.renderer.begin_rendering()?;
        Ok(Render { ctx: self, text_batch: None })
    }
    pub fn add_face(
        &mut self,
        face_data: Rc<Vec<u8>>,
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
    pub fn end(mut self) {
        self.flush();
        self.ctx.renderer.present();
    }
}
