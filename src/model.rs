use crate::*;

use std::{
    error::Error,
    fmt::{Display, Debug, Formatter, Result as FmtResult},
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
    sync::atomic::{AtomicU32, Ordering},
};

static NEXT_UNIQUE_ID: AtomicU32 = AtomicU32::new(0);

/// Used internally to implement the `bb` command
struct BoundingBox {
    in_x: f32, in_y: f32, in_w: f32, in_h: f32,
    out_x: f32, out_y: f32, out_w: f32, out_h: f32,
}

#[derive(Debug, PartialEq)]
pub struct ModelPoint {
    pub point: Point,
    pub color_idx: u8,
}

pub struct Model {
    pub(crate) points: Vec<ModelPoint>,
    pub(crate) triangles: Vec<(u16,u16,u16)>,
    pub(crate) colors: Vec<Color>,
    pub(crate) unique_id: u32,
}

impl Debug for Model {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "Model {{ {} verts, {} triangles, {} colors }}",
               self.points.len(), self.triangles.len(), self.colors.len())
    }
}

#[derive(Debug)]
pub struct V2DParseError {
    lineno: u32,
    wat: &'static str,
    #[allow(dead_code)] // this field is for Debug, on purpose
    inner: SomeKindaError,
}

impl Error for V2DParseError {}

impl Display for V2DParseError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "line {}: {}", self.lineno, self.wat)
    }
}

#[derive(Debug)]
pub enum SomeKindaError {
    Nothing, // What are you saying, Saruman?
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
}

impl From<()> for SomeKindaError {
    fn from(_: ()) -> SomeKindaError { SomeKindaError::Nothing }
}

impl From<ParseIntError> for SomeKindaError {
    fn from(x: ParseIntError) -> SomeKindaError {
        SomeKindaError::ParseIntError(x)
    }
}

impl From<ParseFloatError> for SomeKindaError {
    fn from(x: ParseFloatError) -> SomeKindaError {
        SomeKindaError::ParseFloatError(x)
    }
}

fn from_component_to_f32(src: &str) -> Result<f32, SomeKindaError> {
    if src.find('.').is_some() {
        Ok(f32::from_str(src)?)
    }
    else {
        Ok(u8::from_str_radix(src, 10)? as f32 / 255.0)
    }
}

fn inner_parse_color(args: &[&str]) -> Result<(f32,f32,f32,f32), SomeKindaError> {
    let (r, g, b, a) = match args.len() {
        1 => {
            let (r,g,b,a) = match args[0].len() {
                3 => (u8::from_str_radix(&args[0][0..1], 16)? * 17,
                      u8::from_str_radix(&args[0][1..2], 16)? * 17,
                      u8::from_str_radix(&args[0][2..3], 16)? * 17,
                      255),
                4 => (u8::from_str_radix(&args[0][0..1], 16)? * 17,
                      u8::from_str_radix(&args[0][1..2], 16)? * 17,
                      u8::from_str_radix(&args[0][2..3], 16)? * 17,
                      u8::from_str_radix(&args[0][3..4], 16)? * 17),
                6 => (u8::from_str_radix(&args[0][0..2], 16)?,
                      u8::from_str_radix(&args[0][2..4], 16)?,
                      u8::from_str_radix(&args[0][4..6], 16)?,
                      255),
                8 => (u8::from_str_radix(&args[0][0..2], 16)?,
                      u8::from_str_radix(&args[0][2..4], 16)?,
                      u8::from_str_radix(&args[0][4..6], 16)?,
                      u8::from_str_radix(&args[0][6..8], 16)?),
                _ => return Err(())?
            };
            (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
        }
        3 => (from_component_to_f32(&args[0])?,
              from_component_to_f32(&args[1])?,
              from_component_to_f32(&args[2])?,
              1.0),
        4 => (from_component_to_f32(&args[0])?,
              from_component_to_f32(&args[1])?,
              from_component_to_f32(&args[2])?,
              from_component_to_f32(&args[3])?),
        _ => return Err(())?
    };
    Ok((r, g, b, a))
}

/// Parses a color specification from a V2D, for the `*c` commands.
///
/// - linear: If true, the color components are linear, otherwise sRGB.
/// - prepremultiplied: If true, the color components have been pre-multiplied
///     i.e., as part of a `pc` command.
fn parse_color(linear: bool, prepremultiplied: bool, src: &[&str])
    -> Result<Color, SomeKindaError> {
    let color = inner_parse_color(src)?;
    let color = if linear {Color::from_linear(color.0,color.1,color.2,color.3)}
    else { Color::from_srgb(color.0,color.1,color.2,color.3) };
    if !prepremultiplied { color.premultiplied(); }
    Ok(color)
}

/// Parses a bounding box specification from a V2D.
fn parse_bounding_box(args: &[&str]) -> Result<BoundingBox, SomeKindaError> {
    if args.len() != 8 { return Err(())? }
    Ok(BoundingBox {
        in_x: args[0].parse()?, in_y: args[1].parse()?,
        in_w: args[2].parse()?, in_h: args[3].parse()?,
        out_x: args[4].parse()?, out_y: args[5].parse()?,
        out_w: args[6].parse()?, out_h: args[7].parse()?
    })
}

/// Checks to see if a given point is already in the list, and reuses it if so.
/// Otherwise, inserts it into the list. Returns the index in the list at which
/// the point can now be found.
fn find_or_insert_point(point: ModelPoint, points: &mut Vec<ModelPoint>) -> usize {
    for n in (0 .. points.len()).rev() {
        if points[n] == point { return n }
    }
    let ret = points.len();
    points.push(point);
    ret
}

impl Model {
    pub fn from_components(points: Vec<ModelPoint>,
                           triangles: Vec<(u16,u16,u16)>,
                           colors: Vec<Color>) -> Result<Model, &'static str> {
        if points.len() > 65536 {
            return Err("Too many points in model (maximum is 65 536)")
        }
        if colors.len() > 16 {
            return Err("Too many colors in model (maximum is 16)");
        }
        for &(a,b,c) in triangles.iter() {
            if a as usize >= points.len() || b as usize >= points.len()
            || c as usize >= points.len() {
                return Err("Triangle in model has an out-of-range index")
            }
        }
        Ok(Model { points, triangles, colors,
                   unique_id: NEXT_UNIQUE_ID.fetch_add(1, Ordering::Relaxed)})
    }
    pub fn from_v2d(input: &str) -> Result<Model, V2DParseError> {
        let mut bounding_box = BoundingBox {
            in_x: 0.0, in_y: 0.0, in_w: 1.0, in_h: 1.0,
            out_x: 0.0, out_y: 0.0, out_w: 1.0, out_h: 1.0,
        };
        let mut points = Vec::new();
        let mut triangles = Vec::new();
        let mut colors = Vec::new();
        let mut lineno: u32 = 0;
        for line in input.split('\n') {
            let line = line.trim();
            lineno = lineno.saturating_add(1);
            let mut parsed = vec![];
            let line = line.split('#').next().unwrap_or("");
            for word in line.split(|x| x == ' ' || x == ',' || x == '|') {
                if word.len() > 0 { parsed.push(word); }
            }
            if parsed.len() == 0 { continue }
            match parsed[0] {
                "c" | "pc" | "lc" | "lpc" | "plc" => {
                    let linear = parsed[0].find('l').is_some();
                    let prepremultiplied = parsed[0].find('p').is_some();
                    match parse_color(linear, prepremultiplied, &parsed[1..]) {
                        Ok(x) => {
                            // TODO: lift the maximum to 256 some day
                            if colors.len() > 16 {
                                return Err(V2DParseError {
                                    lineno,
                                    wat: "too many colors (maximum of 16)",
                                    inner: ().into()
                                })
                            }
                            colors.push(x);
                        },
                        Err(x) => return Err(V2DParseError {
                            lineno,
                            wat: "bad color",
                            inner: x
                        })
                    }
                },
                "bb" => match parse_bounding_box(&parsed[1..]) {
                    Ok(x) => bounding_box = x,
                    Err(x) => return Err(V2DParseError {
                        lineno,
                        wat: "bad bounding box",
                        inner: x
                    })
                },
                "p" => {
                    if parsed.len() % 2 != 0 || parsed.len() < 8 {
                        return Err(V2DParseError {
                            lineno, wat: "wrong number of parameters for \
                                          polygon", inner: ().into()
                        })
                    }
                    let colorid = match parsed[1].parse::<u8>() {
                        Ok(x) => x,
                        Err(x) => 
                            return Err(V2DParseError {
                                lineno, wat: "bad color ID for polygon",
                                inner: x.into(),
                            })
                    };
                    if colorid as usize >= colors.len() {
                        return Err(V2DParseError {
                            lineno, wat: "out of range color ID for polygon",
                            inner: ().into()
                        })
                    }
                    let mut this_polygon = Vec::with_capacity(parsed.len()
                                                              / 2 - 1);
                    for i in (2 .. parsed.len()).step_by(2) {
                        let (x, y): (f32,f32)
                            = match (parsed[i].parse(), parsed[i+1].parse()) {
                                (Ok(x), Ok(y)) => (x, y),
                                _ => {
                                    return Err(V2DParseError {
                                        lineno, wat: "bad point in polygon",
                                        inner: ().into(),
                                    })
                                }
                            };
                        let x = (x - bounding_box.in_x) / bounding_box.in_w
                            * bounding_box.out_w + bounding_box.out_x;
                        let y = (y - bounding_box.in_y) / bounding_box.in_h
                            * bounding_box.out_h + bounding_box.out_y;
                        let model_point = ModelPoint {
                            point: Point::new(x, y),
                            color_idx: colorid,
                        };
                        let index: u16 = match
                            find_or_insert_point(model_point, &mut points)
                            .try_into() {
                                Ok(x) => x,
                                Err(_) => {
                                    return Err(V2DParseError {
                                        lineno, wat: "too many points \
                                                      (a V2D can't have more \
                                                      than 65 536 unique \
                                                      points)",
                                        inner: ().into(),
                                    })
                                },
                            };
                        this_polygon.push(index);
                    }
                    // now triangulate the polygon
                    assert!(this_polygon.len() >= 3);
                    triangles.reserve(this_polygon.len()/2*3);
                    for i in 1 .. this_polygon.len()-1 {
                        triangles.push((this_polygon[0], this_polygon[i],
                                        this_polygon[i+1]));
                    }
                },
                _ => return Err(V2DParseError {
                    lineno, wat: "unknown command",
                    inner: SomeKindaError::Nothing
                })
            }
        }
        Ok(Model { points, triangles, colors,
                   unique_id: NEXT_UNIQUE_ID.fetch_add(1, Ordering::Relaxed)})
    }
}

