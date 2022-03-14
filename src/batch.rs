use crate::*;

use std::{
    slice,
    mem::{size_of, transmute},
};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct MergedModelVert {
    pub x: f32, pub y: f32,
    pub r: f16, pub g: f16, pub b: f16, pub a: f16,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SplitModelVert {
    pub x: f32, pub y: f32,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SplitModelPrim {
    pub r: f16, pub g: f16, pub b: f16, pub a: f16,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct MergedTextVert {
    pub x: f16, pub y: f16, pub u: f16, pub v: f16,
    // fill color
    pub fill_r: f16, pub fill_g: f16, pub fill_b: f16, pub fill_a: f16,
    // stroke color
    pub stroke_r: f16, pub stroke_g: f16, pub stroke_b: f16, pub stroke_a: f16,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SplitTextVert {
    pub x: f16, pub y: f16, pub u: f16, pub v: f16,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SplitTextPrim {
    // fill color
    pub fill_r: f16, pub fill_g: f16, pub fill_b: f16, pub fill_a: f16,
    // stroke color
    pub stroke_r: f16, pub stroke_g: f16, pub stroke_b: f16, pub stroke_a: f16,
}

pub(crate) struct MergedModelBatch {
    pub verts: &'static mut[MergedModelVert],
    /// the number of VERTICES batched
    pub n: usize,
}

impl MergedModelBatch {
    pub fn new(buf: &mut[u8]) -> MergedModelBatch {
        let num_prims_that_fit = buf.len() / (size_of::<MergedModelVert>()*3);
        assert!(num_prims_that_fit > 0);
        let num_verts_that_fit = num_prims_that_fit * 3;
        // unsafe justification: We're carefully borrowing a subset of this
        // byte array as another type, to later send to the video card. We
        // are honest about the lifetime, so another mutable borrow of buf
        // should be impossible.
        unsafe {
            MergedModelBatch {
                verts: slice::from_raw_parts_mut(transmute(&mut buf[0]),
                                                 num_verts_that_fit),
                n: 0,
            }
        }
    }
}

pub(crate) struct SplitModelBatch {
    pub verts: &'static mut[SplitModelVert],
    pub prims: &'static mut[SplitModelPrim],
    /// the number of PRIMITIVES batched
    pub n: usize,
}

impl SplitModelBatch {
    pub fn new(buf: &mut[u8]) -> SplitModelBatch {
        let num_prims_that_fit = buf.len()
            / (size_of::<SplitModelVert>() * 3 + size_of::<SplitModelPrim>());
        assert!(num_prims_that_fit > 0);
        let num_verts_that_fit = num_prims_that_fit * 3;
        let offset = num_verts_that_fit * 3 * size_of::<SplitModelVert>();
        // unsafe justification: See MergedModelBatch::new(). Additionally,
        // while we *are* mutably borrowing from the same slice twice, we're
        // doing so in a non-overlapping fashion, which should make it okay.
        unsafe {
            SplitModelBatch {
                verts: slice::from_raw_parts_mut(transmute(&mut buf[0]),
                                                 num_verts_that_fit),
                prims: slice::from_raw_parts_mut(transmute(&mut buf[offset]),
                                                 num_prims_that_fit),
                n: 0,
            }
        }
    }
}

pub(crate) struct MergedTextBatch {
    pub verts: Vec<Vec<MergedTextVert>>,
}

impl MergedTextBatch {
    pub fn new() -> MergedTextBatch {
        MergedTextBatch { verts: Vec::new() }
    }
}

pub(crate) struct SplitTextBatch {
    pub verts: Vec<Vec<SplitTextVert>>,
    pub prims: Vec<Vec<SplitTextPrim>>,
}

impl SplitTextBatch {
    pub fn new() -> SplitTextBatch {
        SplitTextBatch { verts: Vec::new(), prims: Vec::new() }
    }
}

pub(crate) trait ModelBatchable {
    fn push(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32,
            color: Color) -> Result<(),()>;
    fn clear(&mut self);
}

pub(crate) trait TextBatchable {
    fn push(&mut self, atlas: u32, x: f32, y: f32, u: f16, v: f16,
           fill: Color, stroke: Color);
    fn clear(&mut self);
}

impl ModelBatchable for MergedModelBatch {
    fn push(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32,
            color: Color) -> Result<(),()> {
        if self.n == self.verts.len() {
            Err(())
        }
        else {
            self.verts[self.n] = MergedModelVert {
                x: x1, y: y1,
                r: color.r, g: color.g, b: color.b, a: color.a,
            };
            self.verts[self.n+1] = MergedModelVert {
                x: x2, y: y2,
                r: color.r, g: color.g, b: color.b, a: color.a,
            };
            self.verts[self.n+2] = MergedModelVert {
                x: x3, y: y3,
                r: color.r, g: color.g, b: color.b, a: color.a,
            };
            self.n += 3;
            Ok(())
        }
    }
    fn clear(&mut self) {
        self.n = 0;
    }
}

impl ModelBatchable for SplitModelBatch {
    fn push(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32,
            color: Color) -> Result<(),()> {
        if self.n == self.verts.len() {
            Err(())
        }
        else {
            let m = self.n * 3;
            self.verts[m] = SplitModelVert {
                x: x1, y: y1,
            };
            self.verts[m+1] = SplitModelVert {
                x: x2, y: y2,
            };
            self.verts[m+2] = SplitModelVert {
                x: x3, y: y3,
            };
            self.prims[self.n] = SplitModelPrim {
                r: color.r, g: color.g, b: color.b, a: color.a,
            };
            self.n += 1;
            todo!()
        }
    }
    fn clear(&mut self) {
        self.n = 0;
    }
}

impl TextBatchable for MergedTextBatch {
    fn push(&mut self, atlas: u32, x: f32, y: f32, u: f16, v: f16,
            fill: Color, stroke: Color) {
        let atlas = atlas as usize;
        while self.verts.len() <= atlas {
            self.verts.push(Vec::new());
        }
        self.verts[atlas].push(MergedTextVert {
            x: f16::from_f32(x), y: f16::from_f32(y), u, v,
            fill_r: fill.r, fill_g: fill.g, fill_b: fill.b, fill_a: fill.a,
            stroke_r: stroke.r, stroke_g: stroke.g, stroke_b: stroke.b,
            stroke_a: stroke.a,
        });
    }
    fn clear(&mut self) {
        self.verts.clear();
    }
}

impl TextBatchable for SplitTextBatch {
    fn push(&mut self, atlas: u32, x: f32, y: f32, u: f16, v: f16,
            fill: Color, stroke: Color) {
        let atlas = atlas as usize;
        debug_assert_eq!(self.verts.len(), self.prims.len());
        while self.verts.len() <= atlas {
            self.verts.push(Vec::new());
            self.prims.push(Vec::new());
        }
        self.verts[atlas].push(SplitTextVert {
            x: f16::from_f32(x), y: f16::from_f32(y), u, v,
        });
        self.prims[atlas].push(SplitTextPrim {
            fill_r: fill.r, fill_g: fill.g, fill_b: fill.b, fill_a: fill.a,
            stroke_r: stroke.r, stroke_g: stroke.g, stroke_b: stroke.b,
            stroke_a: stroke.a,
        });
    }
    fn clear(&mut self) {
        self.verts.clear();
        self.prims.clear();
    }
}

pub(crate) enum ModelBatch {
    Merged(MergedModelBatch),
    Split(SplitModelBatch),
}

impl From<ModelBatch> for MergedModelBatch {
    fn from(other: ModelBatch) -> MergedModelBatch {
        match other {
            ModelBatch::Merged(x) => x,
            _ => panic!("Got our merges and splits mixed up! Renderer bug!"),
        }
    }
}

impl From<MergedModelBatch> for ModelBatch {
    fn from(other: MergedModelBatch) -> ModelBatch {
        ModelBatch::Merged(other)
    }
}

impl From<ModelBatch> for SplitModelBatch {
    fn from(other: ModelBatch) -> SplitModelBatch {
        match other {
            ModelBatch::Split(x) => x,
            _ => panic!("Got our merges and splits mixed up! Renderer bug!"),
        }
    }
}

impl From<SplitModelBatch> for ModelBatch {
    fn from(other: SplitModelBatch) -> ModelBatch {
        ModelBatch::Split(other)
    }
}

pub(crate) enum TextBatch {
    Merged(MergedTextBatch),
    Split(SplitTextBatch),
}

impl From<TextBatch> for MergedTextBatch {
    fn from(other: TextBatch) -> MergedTextBatch {
        match other {
            TextBatch::Merged(x) => x,
            _ => panic!("Got our merges and splits mixed up! Renderer bug!"),
        }
    }
}

impl From<MergedTextBatch> for TextBatch {
    fn from(other: MergedTextBatch) -> TextBatch {
        TextBatch::Merged(other)
    }
}

impl From<TextBatch> for SplitTextBatch {
    fn from(other: TextBatch) -> SplitTextBatch {
        match other {
            TextBatch::Split(x) => x,
            _ => panic!("Got our merges and splits mixed up! Renderer bug!"),
        }
    }
}

impl From<SplitTextBatch> for TextBatch {
    fn from(other: SplitTextBatch) -> TextBatch {
        TextBatch::Split(other)
    }
}
