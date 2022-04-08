use crate::*;

use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Mul,
};

#[derive(Clone,Copy)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

fn debug_out_component(c: f32, fmt: &mut Formatter<'_>) -> FmtResult {
    let c = (c * 10000.0).round() as i32;
    let (sign, c) = if c < 0 { ('-', -c) } else { ('+', c) };
    write!(fmt, "{}", sign)?;
    if c > 99999 { write!(fmt, "*.***") }
    else { write!(fmt, "{}.{:04}", c / 10000, c % 10000) }
}
impl Debug for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "Color{{")?;
        debug_out_component(self.r, fmt)?;
        write!(fmt, "/")?;
        debug_out_component(self.g, fmt)?;
        write!(fmt, "/")?;
        debug_out_component(self.b, fmt)?;
        write!(fmt, "/")?;
        debug_out_component(self.a, fmt)?;
        write!(fmt, "}}")
    }
}

pub trait EncodedComponent {
    fn to_float(&self) -> f32;
    fn to_half(&self) -> f16 { f16::from_f32(self.to_float()) }
}

pub trait SrgbComponent : EncodedComponent {
    fn to_linear_float(&self) -> f32 {
        Color::srgb_to_linear(self.to_float())
    }
    fn to_linear_half(&self) -> f16 {
        f16::from_f32(self.to_linear_float())
    }
}

impl EncodedComponent for u8 {
    fn to_float(&self) -> f32 { *self as f32 / 255.0 }
}
impl SrgbComponent for u8 {}

impl EncodedComponent for u16 {
    fn to_float(&self) -> f32 { *self as f32 / 65535.0 }
}
impl SrgbComponent for u16 {}

impl EncodedComponent for f16 {
    fn to_float(&self) -> f32 { self.to_f32() }
    fn to_half(&self) -> f16 { *self }
}
impl SrgbComponent for f16 {}

impl EncodedComponent for f32 {
    fn to_float(&self) -> f32 { *self }
}
impl SrgbComponent for f32 {}

impl EncodedComponent for f64 {
    fn to_float(&self) -> f32 { *self as f32 }
}
impl SrgbComponent for f64 {}

impl Color {
    /// Convert a linear intensity into an sRGB-compressed intensity.
    pub fn linear_to_srgb(input: f32) -> f32 {
        if input <= 0.0031308 { input * 12.92 }
        else { 1.055 * input.powf(1.0 / 2.4) - 0.055 }
    }
    /// Convert an sRGB-compressed intensity into a linear intensity.
    pub fn srgb_to_linear(input: f32) -> f32 {
        if input <= 0.04045 { input / 12.92 }
        else { ((input + 0.055) / 1.055).powf(2.4) }
    }
    /// Create a color from the given sRGB color. Components may be encoded as
    /// any type supported by `EncodedComponent`, including `u8` and `f32`.
    pub fn from_srgb<R,G,B,A> (r: R, g: G, b: B, a: A) -> Color
    where R: SrgbComponent, G: SrgbComponent,
          B: SrgbComponent, A: EncodedComponent
    {
        Color {
            r: r.to_linear_float(), g: g.to_linear_float(),
            b: b.to_linear_float(), a: a.to_float(),
        }
    }
    /// Create a color from the given linear color. Components may be encoded
    /// as any type supported by `EncodedComponent`, including `u8` and `f32`.
    pub fn from_linear<R,G,B,A> (r: R, g: G, b: B, a: A) -> Color
    where R: EncodedComponent, G: EncodedComponent,
          B: EncodedComponent, A: EncodedComponent
    {
        Color {
            r: r.to_float(), g: g.to_float(),
            b: b.to_float(), a: a.to_float(),
        }
    }
    /// Assuming this `Color` has not yet been premultiplied, this returns a
    /// premultiplied version of the color.
    pub fn premultiplied(&self) -> Color {
        let a = self.a;
        Color {
            r: self.r * a,
            g: self.g * a,
            b: self.b * a,
            a,
        }
    }
    /// Make a color less (or more) opaque based on a passed-in opacity value.
    pub fn opacity(&self, a: f32) -> Color {
        if a == 1.0 {
            *self
        }
        else {
            Color {
                r: self.r * a,
                g: self.g * a,
                b: self.b * a,
                a: self.a * a,
            }
        }
    }
    /// Interpolate between ourselves (alpha = 0.0) and another color (alpha =
    /// 1.0).
    pub fn lerp(&self, other: Color, a: f32) -> Color {
        let r_a = 1.0 - a;
        Color {
            r: self.r * r_a + other.r * a,
            g: self.g * r_a + other.g * a,
            b: self.b * r_a + other.b * a,
            a: self.a * r_a + other.a * a,
        }
    }
}

impl Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

