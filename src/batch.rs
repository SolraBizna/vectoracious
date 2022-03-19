use crate::*;

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

pub(crate) trait TextBatchable {
    fn push(&mut self, atlas: u32, x: f32, y: f32, u: f16, v: f16,
           fill: Color, stroke: Color);
    fn clear(&mut self);
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
