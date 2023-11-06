use crate::{arc, cf, define_mtl, define_obj_type, define_options, mtl, ns, objc};

#[cfg(feature = "io")]
use crate::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum Type {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
    D2MultisampleArray = 8,
    TextureBuffer = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum Swizzle {
    Zero = 0,
    One = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Alpha = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct SwizzleChannels {
    pub red: Swizzle,
    pub green: Swizzle,
    pub blue: Swizzle,
    pub alpha: Swizzle,
}

impl Default for SwizzleChannels {
    #[inline]
    fn default() -> SwizzleChannels {
        SwizzleChannels {
            red: Swizzle::Red,
            green: Swizzle::Green,
            blue: Swizzle::Blue,
            alpha: Swizzle::Alpha,
        }
    }
}

define_obj_type!(SharedTextureHandle(ns::Id));

impl SharedTextureHandle {
    define_mtl!(device, label);
}

define_options!(Usage(usize));

impl Usage {
    pub const UNKNOWN: Self = Self(0x0000);
    pub const SHADER_READ: Self = Self(0x0001);
    pub const SHADER_WRITE: Self = Self(0x0002);
    pub const RENDER_TARGET: Self = Self(0x0004);
    pub const PIXEL_FROMAT_VIEW: Self = Self(0x0010);

    pub fn to_cf_number(&self) -> arc::R<cf::Number> {
        cf::Number::from_i64(self.0 as _)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(usize)]
pub enum Compression {
    Lossless = 0,
    Lossy = 1,
}

define_obj_type!(Desc(ns::Id), MTL_TEXTURE_DESCRIPTOR);

impl Desc {
    #[objc::cls_msg_send(texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
    pub fn new_2d_with_pixel_format_ar(
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> arc::Rar<Desc>;

    #[objc::cls_rar_retain()]
    pub fn new_2d_with_pixel_format(
        pixel_format: mtl::PixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> arc::R<Desc>;

    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let td = mtl::TextureDescriptor::new_cube_with_pixel_format(mtl::PixelFormat::A8Unorm, 100, false);
    ///
    /// assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    ///
    /// ```
    #[objc::cls_msg_send(textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
    pub fn new_cube_with_pixel_format_ar(
        pixel_format: mtl::PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> arc::Rar<Desc>;

    #[objc::cls_rar_retain()]
    pub fn new_cube_with_pixel_format(
        pixel_format: mtl::PixelFormat,
        size: usize,
        mipmapped: bool,
    ) -> arc::R<Desc>;

    #[objc::cls_msg_send(texture2DDescriptorWithPixelFormat:width:resourceOptions:usage:)]
    pub fn new_2d_with_resource_options_ar(
        pixel_format: mtl::PixelFormat,
        width: usize,
        resource_options: mtl::resource::Options,
        usage: Usage,
    ) -> arc::Rar<Desc>;

    #[objc::cls_rar_retain()]
    pub fn new_2d_with_resource_options(
        pixel_format: mtl::PixelFormat,
        width: usize,
        resource_options: mtl::resource::Options,
        usage: Usage,
    ) -> arc::R<Desc>;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(setTextureType:)]
    pub fn set_texture_type(&mut self, val: Type);

    #[inline]
    pub fn with_texture_type(&mut self, val: Type) -> &mut Self {
        self.set_texture_type(val);
        self
    }

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: mtl::PixelFormat);

    #[inline]
    pub fn with_pixel_format(&mut self, val: mtl::PixelFormat) -> &mut Self {
        self.set_pixel_format(val);
        self
    }

    define_mtl!(
        width,
        set_width,
        height,
        set_height,
        depth,
        set_depth,
        resource_options,
        set_resource_options,
        cpu_cache_mode,
        set_cpu_cache_mode,
        storage_mode,
        set_storage_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode
    );

    #[objc::msg_send(mipmapLevelCount)]
    pub fn mipmap_level_count(&self) -> usize;

    #[objc::msg_send(setMipmapLevelCount:)]
    pub fn set_mipmap_level_count(&mut self, val: usize);

    #[objc::msg_send(sampleCount)]
    pub fn sample_count(&self) -> usize;

    #[objc::msg_send(setSampleCount:)]
    pub fn set_sample_count(&mut self, val: usize);

    #[objc::msg_send(arrayLength)]
    pub fn array_len(&self) -> usize;

    #[objc::msg_send(setArrayLength:)]
    pub fn set_array_len(&mut self, val: usize);

    #[objc::msg_send(usage)]
    pub fn usage(&self) -> Usage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, val: Usage);

    /// Allow GPU-optimization for the contents of this texture. The default value is true.
    #[objc::msg_send(allowGPUOptimizedContents)]
    pub fn allow_gpu_optimized_contents(&self) -> bool;

    #[objc::msg_send(setAllowGPUOptimizedContents:)]
    pub fn set_allow_gpu_optimized_contents(&mut self, val: bool);

    #[objc::msg_send(compressionType)]
    pub fn compression_type(&self) -> Compression;

    #[objc::msg_send(setCompressionType:)]
    pub fn set_compression_type(&mut self, val: Compression);

    #[objc::msg_send(swizzle)]
    pub fn swizzle(&self) -> SwizzleChannels;

    #[objc::msg_send(setSwizzle:)]
    pub fn set_swizzle(&mut self, val: SwizzleChannels);
}

define_obj_type!(Texture(mtl::Resource));

impl Texture {
    define_mtl!(width, height, depth, gpu_resource_id);

    #[objc::msg_send(parentTexture)]
    pub fn parent_texture(&self) -> Option<&Texture>;

    #[objc::msg_send(newTextureViewWithPixelFormat:)]
    pub fn texture_view_with_pixel_format(
        &self,
        pixel_format: mtl::PixelFormat,
    ) -> Option<arc::R<Texture>>;

    #[cfg(feature = "io")]
    #[objc::msg_send(iosurface)]
    pub fn io_surf(&self) -> Option<&io::Surf>;

    #[objc::msg_send(iosurfacePlane)]
    pub fn io_surf_plane(&self) -> usize;

    #[objc::msg_send(textureType)]
    pub fn texture_type(&self) -> Type;

    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_TEXTURE_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics1() {
        let mut td =
            mtl::TextureDesc::new_2d_with_pixel_format(mtl::PixelFormat::A8UNorm, 100, 200, false);

        assert_eq!(td.texture_type(), mtl::TextureType::D2);
        assert_eq!(td.pixel_format(), mtl::PixelFormat::A8UNorm);
        assert_eq!(td.width(), 100);
        assert_eq!(td.height(), 200);
        assert_eq!(td.depth(), 1);
        assert_eq!(td.mipmap_level_count(), 1);
        assert_eq!(td.sample_count(), 1);
        assert_eq!(td.array_len(), 1);

        td.set_width(200);
        assert_eq!(td.width(), 200);

        td.set_height(300);
        assert_eq!(td.height(), 300);
    }

    #[test]
    fn basics2() {
        let td =
            mtl::TextureDesc::new_cube_with_pixel_format(mtl::PixelFormat::A8UNorm, 100, false);

        assert_eq!(td.texture_type(), mtl::TextureType::Cube);
    }

    #[test]
    fn basics3() {
        let device = mtl::Device::default().unwrap();

        let td =
            mtl::TextureDesc::new_2d_with_pixel_format(mtl::PixelFormat::A8UNorm, 100, 200, false);

        let t = device.new_texture(&td).unwrap();

        assert_eq!(t.width(), 100);
        assert_eq!(t.height(), 200);
        assert_eq!(t.depth(), 1);
    }

    #[test]
    fn basics4() {
        let device = mtl::Device::default().unwrap();

        let td =
            mtl::TextureDesc::new_2d_with_pixel_format(mtl::PixelFormat::A8UNorm, 100, 200, false);

        let t = device.new_texture(&td).unwrap();

        assert!(t.parent_texture().is_none());
        assert!(t.io_surf().is_none());
        assert_eq!(t.texture_type(), mtl::texture::Type::D2);
        assert!(t.io_surf().is_none());
        assert_eq!(t.io_surf_plane(), 0);

        let tv = t
            .texture_view_with_pixel_format(mtl::PixelFormat::A8UNorm)
            .unwrap();

        assert!(tv.parent_texture().is_some());
        assert_eq!(tv.width(), 100);
        assert_eq!(tv.height(), 200);
    }
}
