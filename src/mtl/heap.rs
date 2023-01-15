use crate::{
    arc, define_mtl, define_obj_type, mtl, ns,
    objc::{msg_send, Obj},
};

#[derive(Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum Type {
    Automatic = 0,
    Placement = 1,
    Sparce = 2,
}

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_mtl!(
        storage_mode,
        set_storage_mode,
        cpu_cache_mode,
        set_cpu_cache_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode,
        resource_options,
        set_resource_options
    );

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let mut desc = mtl::HeapDescriptor::new();
    /// assert_eq!(0, desc.size());
    /// desc.set_size(1024);
    /// assert_eq!(1024, desc.size());
    ///
    /// let device = mtl::Device::default().unwrap();
    /// let heap = device.new_heap_with_descriptor(&desc).unwrap();
    /// assert!(heap.size() >= 1024);
    /// ```
    pub fn new() -> arc::R<Self> {
        unsafe { MTLHeapDescriptor_new() }
    }

    pub fn size(&self) -> usize {
        unsafe { self.call0(msg_send::size) }
    }

    pub fn set_size(&mut self, value: usize) {
        unsafe { self.call1(msg_send::set_size, value) }
    }
}

define_obj_type!(Heap(ns::Id));

impl Heap {
    define_mtl!(
        device,
        label,
        set_label,
        hazard_tracking_mode,
        resource_options
    );

    pub fn size(&self) -> usize {
        unsafe { self.call0(msg_send::size) }
    }

    #[inline]
    pub fn buffer_with_length_and_options(
        &self,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buffer>> {
        unsafe { rsel_newBufferWithLength_options(self, length, options) }
    }

    #[inline]
    pub fn texture_with_descriptor(
        &self,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::R<mtl::Texture>> {
        unsafe { rsel_newTextureWithDescriptor(self, descriptor) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {

    fn MTLHeapDescriptor_new() -> arc::R<Descriptor>;

    fn rsel_newBufferWithLength_options(
        id: &ns::Id,
        length: usize,
        options: mtl::ResourceOptions,
    ) -> Option<arc::R<mtl::Buffer>>;

    fn rsel_newTextureWithDescriptor(
        id: &ns::Id,
        descriptor: &mtl::TextureDescriptor,
    ) -> Option<arc::R<mtl::Texture>>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut desc = mtl::HeapDescriptor::new();
        assert_eq!(0, desc.size());
        desc.set_size(1024);
        assert_eq!(1024, desc.size());

        let device = mtl::Device::default().unwrap();
        let heap = device.new_heap_with_descriptor(&desc).unwrap();
        assert!(heap.size() >= 1024);
    }
}
