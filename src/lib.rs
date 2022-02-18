use half::f16;

mod color;
pub use color::*;
mod model;
pub use model::*;
mod render;
pub use render::*;

pub type Vector = nalgebra::Vector2<f32>;
pub type Point = nalgebra::Point2<f32>;
pub type Transform = nalgebra::Transform2<f32>;
pub type Rotation = nalgebra::Rotation2<f32>;
pub type Scale = nalgebra::Scale2<f32>;
pub type Translation = nalgebra::Translation2<f32>;
