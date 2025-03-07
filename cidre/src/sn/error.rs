use crate::ns;

impl ns::ErrorDomain {
    pub fn sound_analysis() -> &'static Self {
        unsafe { SNErrorDomain }
    }
}

#[link(name = "SoundAnalysis", kind = "framework")]
unsafe extern "C" {
    static SNErrorDomain: &'static ns::ErrorDomain;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    /// An error that represents a failure that no other error handles.
    #[doc(alias = "SNErrorCodeUnknownError")]
    UnknownError = 1,

    /// An error that occurs when the framework fails to analyze audio.
    #[doc(alias = "SNErrorCodeOperationFailed")]
    OpFailed = 2,

    /// An error that indicates the audio data’s format isn’t valid.
    #[doc(alias = "SNErrorCodeInvalidFormat")]
    InvalidFormat = 3,

    /// An error that indicates the sound classifier’s underlying Core ML model is an invalid model type.
    #[doc(alias = "SNErrorCodeInvalidModel")]
    InvalidModel = 4,

    /// An error that indicates an audio file is invalid.
    #[doc(alias = "SNErrorCodeInvalidFile")]
    InvalidFile = 5,
}
