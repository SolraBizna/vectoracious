use std::{
    mem::replace,
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

enum Batch {
    Nope,
    Model(ModelBatch),
    Text(TextBatch),
}

impl Batch {
    fn take(&mut self) -> Batch {
        replace(self, Batch::Nope)
    }
    fn is_model(&self) -> bool {
        match self {
            &Batch::Model(_) => true,
            _ => false,
        }
    }
    fn is_text(&self) -> bool {
        match self {
            &Batch::Text(_) => true,
            _ => false,
        }
    }
}

pub struct Context {
    renderer: Box<dyn Renderer>,
    text_handler: TextHandler,
}

pub struct Render<'a> {
    ctx: &'a mut Context, 
    batch: Batch,
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
        Ok(Render { ctx: self, batch: Batch::Nope })
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
        match self.batch.take() {
            Batch::Nope => (),
            Batch::Model(b) => {
                self.ctx.renderer.consume_model_batch(b);
            },
            Batch::Text(b) => {
                self.ctx.renderer.render_text_batch(self.ctx.text_handler.get_atlases(), b);
            },
        }
    }
    fn ensure_model_batch(&mut self) {
        if !self.batch.is_model() {
            self.flush();
            self.batch = Batch::Model(self.ctx.renderer.open_model_batch());
        }
    }
    fn ensure_text_batch(&mut self) {
        if !self.batch.is_text() {
            self.flush();
            self.batch = Batch::Text(self.ctx.renderer.new_text_batch());
        }
    }
    pub fn model(&mut self, model: &Model, transform: &Transform,
                 color_overrides: &[Color], opacity: f32) {
        self.ensure_model_batch();
        let batch = match self.batch.take() {
            Batch::Model(ModelBatch::Merged(x)) => {
                model.render(transform, color_overrides, opacity, x,
                             &mut self.ctx.renderer)
            },
            Batch::Model(ModelBatch::Split(x)) => {
                model.render(transform, color_overrides, opacity, x,
                             &mut self.ctx.renderer)
            },
            _ => unreachable!(),
        };
        self.batch = Batch::Model(batch);
    }
    pub fn text(&mut self,
                face_id: u32, fill_color: Color, stroke_color: Color,
                x_align: f32, rtl: bool,
                transform: &Transform, text: &str) {
        self.ensure_text_batch();
        match &mut self.batch {
            Batch::Text(TextBatch::Merged(x)) => {
                self.ctx.text_handler.batch_text(&mut self.ctx.renderer,
                                                 x, face_id, fill_color,
                                                 stroke_color, x_align, rtl,
                                                 transform, text);
            },
            Batch::Text(TextBatch::Split(x)) => {
                self.ctx.text_handler.batch_text(&mut self.ctx.renderer,
                                                 x, face_id, fill_color,
                                                 stroke_color, x_align, rtl,
                                                 transform, text);
            },
            _ => unreachable!(),
        };
    }
    pub fn end(mut self) {
        self.flush();
        self.ctx.renderer.present();
    }
}
