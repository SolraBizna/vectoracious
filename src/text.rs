use crate::*;

use std::rc::Rc;

use psilo_text::{TextHandler, AtlasHandler};
use rustybuzz::{Face, UnicodeBuffer};

/// a struct that contains all the information we need to make a new glyph
/// and/Sigor atlas
struct TBnR<'a> {
    atlases: &'a mut Vec<u32>,
    atlas_size: u32, atlas_recip_size: f32,
    renderer: &'a mut Box<dyn Renderer>,
}

pub struct TextBatch {
    atlases: Vec<u32>,
    batches: Vec<Vec<BatchTextVert>>,
    text_handler: TextHandler<u32, AtlasCoords>,
    atlas_size: u32,
    // the reciprocal of the atlas size (always exact if the atlas size is a
    // power of two)
    atlas_recip_size: f32,
}

#[derive(Debug,PartialEq,Clone,Copy)]
struct AtlasCoords {
    x1: f32, y1: f32, x2: f32, y2: f32,
    u1: f16, v1: f16, u2: f16, v2: f16,
}

impl<'a> AtlasHandler for TBnR<'a> {
    type AtlasID = u32;
    type AtlasCoords = AtlasCoords;
    type E = anyhow::Error;
    fn new_atlas(&mut self) -> Result<u32, Self::E> {
        let ret_index = match self.atlases.len().try_into() {
            Ok(x) => x,
            _ => panic!("You have billions of atlases, something has gone \
                         extremely wrong."),
        };
        self.atlases.push(self.renderer.new_text_atlas(self.atlas_size)?);
        Ok(ret_index)
    }
    fn get_atlas_size(&mut self) -> (u32, u32) {
        (self.atlas_size, self.atlas_size)
    }
    fn add_to_atlas(&mut self, atlas: u32,
                    render_x_min: f32, render_y_min: f32,
                    render_x_max: f32, render_y_max: f32,
                    glyph_x: u32, glyph_y: u32, glyph_w: u32, glyph_h: u32,
                    pixels: &[u8]) -> Result<AtlasCoords, Self::E> {
        self.renderer.new_text_glyph(self.atlases[atlas as usize],
                                     glyph_x, glyph_y,
                                     glyph_w, glyph_h, pixels)?;
        let u1 = (glyph_x as f32 + 0.5) * self.atlas_recip_size;
        let v1 = (glyph_y as f32 + 0.5) * self.atlas_recip_size;
        let u2 = u1 + (glyph_w as f32 - 1.0) * self.atlas_recip_size;
        let v2 = v1 + (glyph_h as f32 - 1.0) * self.atlas_recip_size;
        let u1 = f16::from_f32(u1);
        let v1 = f16::from_f32(v1);
        let u2 = f16::from_f32(u2);
        let v2 = f16::from_f32(v2);
        let x1 = render_x_min;
        let x2 = render_x_max;
        let y1 = render_y_min;
        let y2 = render_y_max;
        Ok(AtlasCoords { u1, v1, u2, v2, x1, x2, y1, y2 })

    }
}

impl TextBatch {
    pub fn new(renderer: &mut Box<dyn Renderer>) -> TextBatch {
        let atlas_size = renderer.get_text_atlas_size();
        TextBatch { atlases: vec![], batches: vec![],
                    text_handler: TextHandler::new(),
                    atlas_size, atlas_recip_size: 1.0 / (atlas_size as f32) }
    }
    pub fn add_face(&mut self, face_data: Rc<Vec<u8>>, index: u32,
                    border_texels: f32, texels_per_em_x: f32,
                    texels_per_em_y: f32) -> Option<usize> {
        self.text_handler.add_face(face_data,
                                   index,
                                   border_texels,
                                   texels_per_em_x,
                                   texels_per_em_y)
    }
    pub fn get_face(&self, i: usize) -> Option<&Face> {
        self.text_handler.get_face(i)
    }
    pub fn get_face_mut(&mut self, i: usize) -> Option<&mut Face> {
        self.text_handler.get_face_mut(i)
    }
    pub fn batch_text(&mut self,
                      renderer: &mut Box<dyn Renderer>,
                      face_id: u32, fill_color: Color, stroke_color: Color,
                      x_align: f32, rtl: bool,
                      transform: &Transform, text: &str) {
        let face = self.text_handler.get_face(face_id as usize).unwrap();
        let mut buf = UnicodeBuffer::new();
        use rustybuzz::Direction;
        buf.set_direction(if rtl { Direction::RightToLeft } else { Direction::LeftToRight });
        buf.push_str(text);
        let buf = rustybuzz::shape(face, &[], buf);
        let infos = buf.glyph_infos();
        let positions = buf.glyph_positions();
        let em_per = 1.0 / face.units_per_em() as f32;
        let mut advance_w = 0.0;
        let mut _advance_h = 0.0;
        for position in positions.iter() {
            advance_w += position.x_advance as f32 * em_per;
            _advance_h += position.y_advance as f32 * em_per;
        }
        let mut x = advance_w * -0.5 + advance_w * -0.5 * x_align;
        let mut y = 0.0;
        for (info, position) in infos.iter().zip(positions.iter()) {
            let x_advance = position.x_advance as f32 * em_per;
            let y_advance = position.y_advance as f32 * em_per;
            let glyph = self.text_handler.get_glyph(face_id as usize, info.glyph_id as u16, &mut TBnR { atlases: &mut self.atlases, atlas_size: self.atlas_size, atlas_recip_size: self.atlas_recip_size, renderer }).unwrap();
            match glyph {
                Some((textureid, coords)) => {
                    let x_offset = position.x_offset as f32 * em_per;
                    let y_offset = position.y_offset as f32 * em_per;
                    let p1 = Point::new(x + coords.x1+x_offset,
                                        y + coords.y1+y_offset);
                    let p1 = transform.transform_point(&p1);
                    let p2 = Point::new(x + coords.x2+x_offset,
                                        y + coords.y2+y_offset);
                    let p2 = transform.transform_point(&p2);
                    while textureid as usize >= self.batches.len() {
                        self.batches.push(vec![]);
                    }
                    let batch = &mut self.batches[textureid as usize];
                    batch.push(BatchTextVert { u: coords.u1, v: coords.v1,
                                               x: f16::from_f32(p1.x),
                                               y: f16::from_f32(p1.y),
                                               fill_r: fill_color.r,
                                               fill_g: fill_color.g,
                                               fill_b: fill_color.b,
                                               fill_a: fill_color.a,
                                               stroke_r: stroke_color.r,
                                               stroke_g: stroke_color.g,
                                               stroke_b: stroke_color.b,
                                               stroke_a: stroke_color.a,
                    });
                    batch.push(BatchTextVert { u: coords.u2, v: coords.v1,
                                               x: f16::from_f32(p2.x),
                                               y: f16::from_f32(p1.y),
                                               fill_r: fill_color.r,
                                               fill_g: fill_color.g,
                                               fill_b: fill_color.b,
                                               fill_a: fill_color.a,
                                               stroke_r: stroke_color.r,
                                               stroke_g: stroke_color.g,
                                               stroke_b: stroke_color.b,
                                               stroke_a: stroke_color.a,
                    });
                    batch.push(BatchTextVert { u: coords.u2, v: coords.v2,
                                               x: f16::from_f32(p2.x),
                                               y: f16::from_f32(p2.y),
                                               fill_r: fill_color.r,
                                               fill_g: fill_color.g,
                                               fill_b: fill_color.b,
                                               fill_a: fill_color.a,
                                               stroke_r: stroke_color.r,
                                               stroke_g: stroke_color.g,
                                               stroke_b: stroke_color.b,
                                               stroke_a: stroke_color.a,
                    });
                    batch.push(BatchTextVert { u: coords.u1, v: coords.v2,
                                               x: f16::from_f32(p1.x),
                                               y: f16::from_f32(p2.y),
                                               fill_r: fill_color.r,
                                               fill_g: fill_color.g,
                                               fill_b: fill_color.b,
                                               fill_a: fill_color.a,
                                               stroke_r: stroke_color.r,
                                               stroke_g: stroke_color.g,
                                               stroke_b: stroke_color.b,
                                               stroke_a: stroke_color.a,
                    });
                },
                None => (),
            };
            x += x_advance;
            y += y_advance;
        }
    }
    pub fn render(&mut self, renderer: &mut Box<dyn Renderer>) {
        for (&atlas, batch) in self.atlases.iter().zip(self.batches.iter_mut()) {
            if batch.len() > 0 {
                renderer.render_text_batch(atlas, batch);
                batch.clear();
            }
        }
    }
}
