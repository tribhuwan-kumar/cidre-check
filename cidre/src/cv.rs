pub mod base;
pub use base::Flags as TimeFlags;
pub use base::OptionFlags;
pub use base::SmpteTime;
pub use base::Time;
pub use base::TimeStamp;
pub use base::TimeStampFlags;

mod _return;
pub use _return::Return;
pub use _return::err;

pub mod buffer;
pub use buffer::AttachMode;
pub use buffer::Buf;

mod image_buffer;
pub use image_buffer::ImageBuf;
pub use image_buffer::attachment as image_buf_attachment;
pub use image_buffer::attachment as image_buf_attach;

pub mod pixel_buffer;
pub use pixel_buffer::PixelBuf;
pub use pixel_buffer::PixelFormat;
pub use pixel_buffer::keys as pixel_buffer_keys;

pub mod pixel_buffer_pool;
pub use pixel_buffer_pool::FlushFlags as PixelBufPoolFlushFlags;
pub use pixel_buffer_pool::PixelBufPool;

pub mod pixel_format_description;
pub use pixel_format_description::all_pixel_formats as pixel_format_desc_array_with_all_pixel_formats;
pub use pixel_format_description::create as pixel_format_desc_create;

#[cfg(feature = "mtl")]
pub mod metal;
#[cfg(feature = "mtl")]
pub use metal::Texture as MetalTexture;
#[cfg(feature = "mtl")]
pub use metal::TextureCache as MetalTextureCache;
#[cfg(feature = "mtl")]
pub use metal::texture_cache_keys as metal_texture_cache_keys;
#[cfg(feature = "mtl")]
pub use metal::texture_keys as metal_texture_keys;

#[cfg(target_os = "macos")]
pub mod display_link;
#[cfg(target_os = "macos")]
pub use display_link::DisplayLink;
#[cfg(target_os = "macos")]
pub use display_link::OutputCb as DisplayLinkOutputCb;

mod host_time;
pub use host_time::current_host_time;
pub use host_time::host_clock_frequency;
pub use host_time::host_clock_minimum_time_delta;

#[link(name = "CoreVideo", kind = "framework")]
unsafe extern "C" {}
