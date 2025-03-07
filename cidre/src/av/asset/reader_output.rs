use crate::{arc, av, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cm")]
use crate::cm;

define_obj_type!(
    #[doc(alias = "AVAssetReaderOutput")]
    pub ReaderOutput(ns::Id)
);

define_obj_type!(
    #[doc(alias = "AVAssetReaderTrackOutput")]
    pub ReaderTrackOutput(ReaderOutput)
);

define_obj_type!(
    #[doc(alias = "AVAssetReaderAudioMixOutput")]
    pub ReaderAudioMixOutput(ReaderOutput)
);

define_obj_type!(
    #[doc(alias = "AVAssetReaderVideoCompositionOutput")]
    pub ReaderVideoCompositionOutput(ReaderOutput)
);
define_obj_type!(
    #[doc(alias = "AVAssetReaderSampleReferenceOutput")]
    pub ReaderSampleReferenceOutput(ReaderOutput)
);

define_obj_type!(
    #[doc(alias = "AVAssetReaderOutputMetadataAdaptor")]
    pub ReaderOutputMetadataAdaptor(ns::Id)
);

define_obj_type!(
    #[doc(alias = "AVAssetReaderOutputCaptionAdaptor")]
    pub ReaderOutputCaptionAdaptor(ns::Id)
);

/// Is an abstract class that defines an interface for reading a single collection
/// of samples of a common media type from an av::AssetReader.
///
/// Clients can read the media data of an asset by adding one or more concrete
/// instances of av::AssetReaderOutput to an AVAssetReader using the -[AVAssetReader addOutput:] method.
///
/// IMPORTANT PERFORMANCE NOTE: Make sure to set the always_copies_sample_data property to false
/// if you do not need to modify the sample data in-place, to avoid unnecessary and inefficient copying.
impl ReaderOutput {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> arc::R<av::MediaType>;

    /// Indicates whether or not the data in buffers gets copied before being vended to the client.
    ///
    /// When the value of this property is true, the AVAssetReaderOutput will always vend a buffer
    /// with copied data to the client. Data in such buffers can be freely modified by the client.
    /// When the value of this property is false, the buffers vended to the client may not be copied.
    /// Such buffers may still be referenced by other entities. The result of modifying a buffer
    /// whose data hasn't been copied is undefined. Requesting buffers whose data hasn't been copied
    /// when possible can lead to performance improvements.
    ///
    /// Default value is true
    #[objc::msg_send(alwaysCopiesSampleData)]
    pub fn always_copies_sample_data(&self) -> bool;

    #[objc::msg_send(setAlwaysCopiesSampleData:)]
    pub fn set_always_copies_sample_data(&mut self, value: bool);

    #[objc::msg_send(supportsRandomAccess)]
    pub fn supports_random_access(&self) -> bool;

    /// Copies the next sample buffer for the output synchronously.
    ///
    /// This method will return None if there are no more sample buffers available for
    /// the receiver within the time range specified by its av::AssetReader's time_range property,
    /// or if there is an error that prevents the av::AssetReader from reading more media data.
    /// When this method returns None, clients should check the value of the associated av::AssetReader's
    /// status property to determine why no more samples could be read.
    ///
    /// This method throws an exception if this output is not added to an instance of av::AssetReader
    /// (using add_output()) and start_reading() is not called on that asset reader.
    #[cfg(feature = "cm")]
    #[objc::msg_send(copyNextSampleBuffer)]
    pub unsafe fn next_sample_buf_throws(&mut self) -> Option<arc::Retained<cm::SampleBuf>>;

    #[cfg(feature = "cm")]
    #[inline]
    pub fn next_sample_buf<'ear>(&mut self) -> ns::ExResult<'ear, Option<arc::R<cm::SampleBuf>>> {
        ns::try_catch(|| unsafe { self.next_sample_buf_throws() })
    }
}

impl arc::A<ReaderTrackOutput> {
    #[objc::msg_send(initWithTrack:outputSettings:)]
    pub fn init_with_track_throws(
        self,
        track: &av::asset::Track,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<ReaderTrackOutput>;
}

impl ReaderTrackOutput {
    define_cls!(AV_ASSET_READER_TRACK_OUTPUT);

    /// Returns an instance of AVAssetReaderTrackOutput for reading from the specified track and
    /// supplying media data according to the specified output settings.
    ///
    /// The track must be one of the tracks contained by the target AVAssetReader's asset.
    /// A value of nil for outputSettings configures the output to vend samples in their
    /// original format as stored by the specified track.  Initialization will fail if the output settings
    /// cannot be used with the specified track.
    ///
    /// ReaderTrackOutput can only produce uncompressed output. For audio output settings, this means that
    /// AVFormatIDKey must be kAudioFormatLinearPCM. For video output settings, this means that the dictionary
    /// must follow the rules for uncompressed video output, as laid out in AVVideoSettings.h.
    /// ReaderTrackOutput does not support the AVAudioSettings.h key AVSampleRateConverterAudioQualityKey
    /// or the following AVVideoSettings.h keys:
    ///
    ///  - AVVideoCleanApertureKey
    ///  - AVVideoPixelAspectRatioKey
    ///  - AVVideoScalingModeKey
    ///
    /// When constructing video output settings the choice of pixel format will affect the performance
    /// and quality of the decompression. For optimal performance when decompressing video the requested pixel
    /// format should be one that the decoder supports natively to avoid unnecessary conversions.
    /// Below are some recommendations:
    ///   - For H.264 use kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange, or kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
    ///     if the video is known to be full range. For JPEG on iOS, use kCVPixelFormatType_420YpCbCr8BiPlanarFullRange.
    ///   - For other codecs on OSX, kCVPixelFormatType_422YpCbCr8 is the preferred pixel format for video
    ///     and is generally the most performant when decoding. If you need to work in the RGB domain then kCVPixelFormatType_32BGRA
    ///     is recommended on iOS and kCVPixelFormatType_32ARGB is recommended on OSX.
    ///   - ProRes encoded media can contain up to 12bits/ch. If your source is ProRes encoded and you wish to preserve more
    ///     than 8bits/ch during decompression then use one of the following pixel formats:
    ///     kCVPixelFormatType_4444AYpCbCr16, kCVPixelFormatType_422YpCbCr16, kCVPixelFormatType_422YpCbCr10, or kCVPixelFormatType_64ARGB.
    ///     av::AssetReader does not support scaling with any of these high bit depth pixel formats.
    ///     If you use them then do not specify kCVPixelBufferWidthKey or kCVPixelBufferHeightKey in your outputSettings dictionary.
    ///     If you plan to append these sample buffers to an AVAssetWriterInput then note that only the ProRes encoders support these pixel formats.
    ///   - ProRes 4444 encoded media can contain a mathematically lossless alpha channel. To preserve the alpha channel during decompression use
    ///     a pixel format with an alpha component such as kCVPixelFormatType_4444AYpCbCr16 or kCVPixelFormatType_64ARGB.
    ///     To test whether your source contains an alpha channel check that the track's format description has kCMFormatDescriptionExtension_Depth
    ///     and that its value is 32.
    pub fn with_track_throws(
        track: &av::asset::Track,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_track_throws(track, output_settings)
    }

    pub fn with_track<'ear>(
        track: &av::asset::Track,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| Self::with_track_throws(track, output_settings))
    }

    #[objc::msg_send(resetForReadingTimeRanges:)]
    pub fn reset_for_reading_time_ranges(&mut self, ranges: &ns::Array<ns::Value>);
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_ASSET_READER_TRACK_OUTPUT: &'static objc::Class<ReaderTrackOutput>;
}
