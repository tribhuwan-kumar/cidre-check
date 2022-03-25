use crate::{cf, ns};
use crate::{define_mtl, define_obj_type};

#[repr(transparent)]
pub struct ResourceUsage(pub usize);

impl ResourceUsage {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const SAMPLE: Self = Self(1 << 2);
}

#[repr(transparent)]
pub struct BarrierScope(pub usize);

impl BarrierScope {
    pub const BUFFERS: Self = Self(1 << 0);
    pub const TEXTURES: Self = Self(1 << 1);
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_obj_type!(CommandEncoder(ns::Id));

impl CommandEncoder {
    define_mtl!(device, mut label);

    pub fn end_encoding(&mut self) {
        unsafe { wsel_endEncoding(self) }
    }

    pub fn insert_debug_signpost(&mut self, signpost: &cf::String) {
        unsafe { wsel_insertDebugSignpost(self, signpost) }
    }

    pub fn push_debug_group(&mut self, debug_group: &cf::String) {
        unsafe { wsel_pushDebugGroup(self, debug_group) }
    }

    pub fn pop_debug_group(&mut self) {
        unsafe { wsel_popDebugGroup(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_endEncoding(id: &mut ns::Id);
    fn wsel_insertDebugSignpost(id: &mut ns::Id, signpost: &cf::String);
    fn wsel_pushDebugGroup(id: &mut ns::Id, debug_group: &cf::String);
    fn wsel_popDebugGroup(id: &mut ns::Id);
}
