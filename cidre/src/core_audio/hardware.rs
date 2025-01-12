use std::ffi::c_void;

use crate::{
    arc,
    at::AudioBufListN,
    cat, cf,
    core_audio::{Class, Obj, PropAddr, PropElement, PropScope, PropSelector},
    os, sys,
};

#[cfg(all(feature = "blocks", feature = "dispatch"))]
use crate::{blocks, dispatch};

#[doc(alias = "AudioObjectPropertyListenerProc")]
pub type PropListenerFn<T = c_void> = extern "C-unwind" fn(
    obj_id: Obj,
    number_addresses: u32,
    addresses: *const PropAddr,
    client_data: *mut T,
) -> os::Status;

#[doc(alias = "AudioObjectPropertyListenerBlock")]
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub type PropListenerBlock =
    blocks::EscBlock<fn(number_addresses: u32, addresses: *const PropAddr)>;

impl Obj {
    #[doc(alias = "AudioObjectSetPropertyData")]
    pub fn set_prop<T: Sized>(&self, address: &PropAddr, val: &T) -> os::Result {
        unsafe {
            AudioObjectSetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                std::mem::size_of_val(val) as u32,
                val as *const _ as _,
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectHasProperty")]
    pub fn has_prop(&self, address: &PropAddr) -> bool {
        unsafe { AudioObjectHasProperty(*self, address) }
    }

    #[doc(alias = "AudioObjectIsPropertySettable")]
    pub fn is_prop_settable(&self, address: &PropAddr) -> os::Result<bool> {
        os::result_init(|res| unsafe { AudioObjectIsPropertySettable(*self, address, res) })
    }

    #[doc(alias = "AudioObjectGetPropertyDataSize")]
    pub fn prop_size(&self, address: &PropAddr) -> os::Result<u32> {
        os::result_init(|res| unsafe {
            AudioObjectGetPropertyDataSize(*self, address, 0, std::ptr::null(), res)
        })
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop<T: Sized>(&self, address: &PropAddr) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                val.as_mut_ptr() as _,
            )
            .result()?;
            Ok(val.assume_init())
        }
    }

    pub fn bool_prop(&self, address: &PropAddr) -> os::Result<bool> {
        let res: u32 = self.prop(address)?;
        Ok(res == 1)
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop_with_qualifier<T: Sized, Q: Sized>(
        &self,
        address: &PropAddr,
        qualifier: &Q,
    ) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let qualifier_size = std::mem::size_of::<Q>() as u32;
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                address,
                qualifier_size,
                qualifier as *const Q as *const _,
                &mut data_size,
                val.as_mut_ptr() as _,
            )
            .result()?;
            Ok(val.assume_init())
        }
    }

    pub fn cf_prop<T: arc::Release>(&self, address: &PropAddr) -> os::Result<arc::R<T>> {
        let mut data_size = std::mem::size_of::<arc::R<T>>() as u32;
        os::result_init(|res| unsafe {
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                res as _,
            )
        })
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop_vec<T: Sized>(&self, address: &PropAddr) -> os::Result<Vec<T>> {
        let mut data_size = self.prop_size(address)?;
        unsafe {
            let len = (data_size as usize) / std::mem::size_of::<T>();
            if len == 0 {
                return Ok(vec![]);
            }
            let mut out = Vec::<T>::with_capacity(len);
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                out.as_mut_ptr().cast(),
            )
            .result()?;
            out.set_len(len);
            Ok(out)
        }
    }

    #[doc(alias = "AudioObjectShow")]
    pub fn show(&self) {
        unsafe { AudioObjectShow(*self) }
    }

    #[doc(alias = "AudioObjectAddPropertyListener")]
    pub fn add_prop_listener<T>(
        &self,
        address: &PropAddr,
        listener: PropListenerFn<T>,
        client_data: *mut T,
    ) -> os::Result {
        unsafe {
            AudioObjectAddPropertyListener(
                *self,
                address,
                std::mem::transmute(listener),
                client_data.cast(),
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectRemovePropertyListener")]
    pub fn remove_prop_listener<T>(
        &self,
        address: &PropAddr,
        listener: PropListenerFn<T>,
        client_data: *mut T,
    ) -> os::Result {
        unsafe {
            AudioObjectRemovePropertyListener(
                *self,
                address,
                std::mem::transmute(listener),
                client_data.cast(),
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectAddPropertyListenerBlock")]
    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    pub fn add_prop_listener_block(
        &self,
        address: &PropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: &mut PropListenerBlock,
    ) -> os::Result {
        unsafe {
            AudioObjectAddPropertyListenerBlock(*self, address, dispatch_queue, listener).result()
        }
    }

    #[doc(alias = "AudioObjectRemovePropertyListenerBlock")]
    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    pub fn remove_prop_listener_block(
        &self,
        address: &PropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: &mut PropListenerBlock,
    ) -> os::Result {
        unsafe {
            AudioObjectRemovePropertyListenerBlock(*self, address, dispatch_queue, listener)
                .result()
        }
    }

    pub fn name(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&PropSelector::NAME.global_addr())
    }

    pub fn class(&self) -> os::Result<Class> {
        self.prop(&PropSelector::CLASS.global_addr())
    }

    pub fn base_class(&self) -> os::Result<Class> {
        self.prop(&PropSelector::BASE_CLASS.global_addr())
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Process(pub Obj);

impl std::ops::Deref for Process {
    type Target = Obj;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Process {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Process {
    pub fn pid(&self) -> os::Result<sys::Pid> {
        self.prop(&PropSelector::PROCESS_PID.global_addr())
    }

    pub fn bundle_id(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&PropSelector::PROCESS_BUNDLE_ID.global_addr())
    }

    pub fn is_running(&self) -> os::Result<bool> {
        self.bool_prop(&PropSelector::PROCESS_IS_RUNNING.global_addr())
    }

    pub fn is_running_input(&self) -> os::Result<bool> {
        self.bool_prop(&PropSelector::PROCESS_IS_RUNNING_INPUT.global_addr())
    }

    pub fn is_running_output(&self) -> os::Result<bool> {
        self.bool_prop(&PropSelector::PROCESS_IS_RUNNING_OUTPUT.global_addr())
    }

    pub fn devices(&self) -> os::Result<Vec<Device>> {
        self.prop_vec(&PropSelector::PROCESS_DEVICES.global_addr())
    }

    pub fn with_pid(pid: sys::Pid) -> os::Result<Self> {
        System::OBJ.prop_with_qualifier(
            &PropSelector::HARDWARE_TRANSLATE_PID_TO_PROCESS_OBJ.global_addr(),
            &pid,
        )
    }

    pub fn list() -> os::Result<Vec<Self>> {
        System::OBJ.prop_vec(&PropSelector::HARDWARE_PROCESS_OBJ_LIST.global_addr())
    }
}

/// Processes AudioObjectPropertySelector values provided by the Process class.
impl PropSelector {
    /// A pid_t indicating the process ID associated with the process.
    #[doc(alias = "kAudioProcessPropertyPID")]
    pub const PROCESS_PID: Self = Self(u32::from_be_bytes(*b"ppid"));

    /// A cf::String that contains the bundle ID of the process. The caller is
    /// responsible for releasing the returned cf::Object.
    #[doc(alias = "kAudioProcessPropertyBundleID")]
    pub const PROCESS_BUNDLE_ID: Self = Self(u32::from_be_bytes(*b"pbid"));

    /// An array of AudioObjectIds that represent the devices currently used by the
    /// process for input or used by the process for output. The scope will select
    /// the input or output device list.
    pub const PROCESS_DEVICES: Self = Self(u32::from_be_bytes(*b"pdv#"));

    /// A u32 where a value of 0 indicates that there is not audio IO in progress
    /// in the process, and a value of 1 indicates that there is audio IO in progress
    /// in the process. Note that audio IO may in progress even if no input or output
    /// streams are active.
    pub const PROCESS_IS_RUNNING: Self = Self(u32::from_be_bytes(*b"pir?"));

    /// A u32 where a value of 0 indicates that the process is not running any
    /// IO or there is not any active input streams, and a value of 1 indicates that
    /// the process is running IO and there is at least one active input stream.
    pub const PROCESS_IS_RUNNING_INPUT: Self = Self(u32::from_be_bytes(*b"piri"));

    /// A u32 where a value of 0 indicates that the process is not running any
    /// IO or there is not any active output streams, and a value of 1 indicates that
    /// the process is running IO and there is at least one active output stream.
    pub const PROCESS_IS_RUNNING_OUTPUT: Self = Self(u32::from_be_bytes(*b"piro"));
}

pub struct System(Obj);

impl std::ops::Deref for System {
    type Target = Obj;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for System {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl System {
    #[doc(alias = "kAudioObjectSystemObject")]
    pub const OBJ: System = System(Obj(1));

    #[doc(alias = "kAudioHardwarePropertyDevices")]
    pub fn devices() -> os::Result<Vec<Device>> {
        Self::OBJ.prop_vec(&PropSelector::HARDWARE_DEVICES.global_addr())
    }

    #[doc(alias = "kAudioHardwarePropertyDefaultInputDevice")]
    pub fn default_input_device() -> os::Result<Device> {
        Self::OBJ.prop(&PropSelector::HARDWARE_DEFAULT_INPUT_DEVICE.global_addr())
    }

    #[doc(alias = "kAudioHardwarePropertyDefaultOutputDevice")]
    pub fn default_output_device() -> os::Result<Device> {
        Self::OBJ.prop(&PropSelector::HARDWARE_DEFAULT_OUTPUT_DEVICE.global_addr())
    }
}

/// AudioSystemObject Properties
impl PropSelector {
    #[doc(alias = "kAudioHardwarePropertyProcessInputMute")]
    pub const HARDWARE_PROCESS_INPUT_MUTE: Self = Self(u32::from_be_bytes(*b"pmin"));

    /// An array of the AudioObjectIds that represent all the devices currently
    /// available to the system.
    #[doc(alias = "kAudioHardwarePropertyDevices")]
    pub const HARDWARE_DEVICES: Self = Self(u32::from_be_bytes(*b"dev#"));

    /// The AudioObjectId of the default input AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultInputDevice")]
    pub const HARDWARE_DEFAULT_INPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dIn "));

    /// The AudioObjectId of the default output AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultOutputDevice")]
    pub const HARDWARE_DEFAULT_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dOut"));

    /// The AudioObjectId of the output AudioDevice to use for system related sound
    /// from the alert sound to digital call progress.
    #[doc(alias = "kAudioHardwarePropertyDefaultSystemOutputDevice")]
    pub const HARDWARE_DEFAULT_SYS_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"sOut"));

    /// This property fetches the AudioObjectId that corresponds to the AudioDevice
    /// that has the given UID. The UID is passed in via the qualifier as a cf::String
    /// while the AudioObjectId for the AudioDevice is returned to the caller as the
    /// property's data. Note that an error is not returned if the UID doesn't refer
    /// to any AudioDevices. Rather, this property will return kAudioObjectUnknown
    /// as the value of the property.
    #[doc(alias = "kAudioHardwarePropertyTranslateUIDToDevice")]
    pub const HARDWARE_TRANSLATE_UID_TO_DEVICE: Self = Self(u32::from_be_bytes(*b"uidd"));

    /// An array of AudioObjectIds that represent the Process objects for all client processes
    /// currently connected to the system.
    #[doc(alias = "kAudioHardwarePropertyProcessObjectList")]
    pub const HARDWARE_PROCESS_OBJ_LIST: Self = Self(u32::from_be_bytes(*b"prs#"));

    /// This property fetches the AudioObjectID that corresponds to the Process object
    /// that has the given PID. The PID is passed in via the qualifier as a pid_t
    /// while the AudioObjectID for the Process is returned to the caller as the
    /// property's data. Note that an error is not returned if the PID doesn't refer
    /// to any Process. Rather, this property will return kAudioObjectUnknown
    /// as the value of the property.
    #[doc(alias = "kAudioHardwarePropertyTranslatePIDToProcessObject")]
    pub const HARDWARE_TRANSLATE_PID_TO_PROCESS_OBJ: Self = Self(u32::from_be_bytes(*b"id2p"));
}

/// AudioAggregateDevice Properties
impl PropSelector {
    /// A CFArray of CFStrings that contain the UIDs of all the devices, active or
    /// inactive, contained in the AudioAggregateDevice. The order of the items in
    /// the array is significant and is used to determine the order of the streams
    /// of the AudioAggregateDevice. The caller is responsible for releasing the
    /// returned CFObject.
    #[doc(alias = "kAudioAggregateDevicePropertyFullSubDeviceList")]
    pub const AGGREGATE_DEVICE_FULL_SUB_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"grup"));

    /// An array of AudioObjectIDs for all the active sub-devices in the aggregate
    /// device.
    #[doc(alias = "kAudioAggregateDevicePropertyActiveSubDeviceList")]
    pub const AGGREGATE_DEVICE_ACTIVE_SUB_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"agrp"));

    /// A CFDictionary that describes the composition of the AudioAggregateDevice.
    /// The keys for this CFDicitionary are defined in the AudioAggregateDevice
    /// Constants section. The caller is responsible for releasing the returned CFObject.
    #[doc(alias = "kAudioAggregateDevicePropertyComposition")]
    pub const AGGREGATE_DEVICE_COMPOSITION: Self = Self(u32::from_be_bytes(*b"acom"));

    /// A CFString that contains the UID for the AudioDevice that is currently
    /// serving as the time base of the aggregate device. The caller is
    /// responsible for releasing the returned CFObject.
    #[doc(alias = "kAudioAggregateDevicePropertyMainSubDevice")]
    pub const AGGREGATE_DEVICE_MAIN_SUB_DEVICE: Self = Self(u32::from_be_bytes(*b"amst"));

    /// A CFString that contains the UID for the AudioClockDevice that is currently
    /// serving as the time base of the aggregate device. If the aggregate
    /// device includes both a main audio device and a clock device, the clock
    /// device will control the time base. Setting this property will enable
    /// drift correction for all subdevices in the aggregate device. The caller is
    /// responsible for releasing the returned CFObject.
    #[doc(alias = "kAudioAggregateDevicePropertyClockDevice")]
    pub const AGGREGATE_DEVICE_CLOCK_DEVICE: Self = Self(u32::from_be_bytes(*b"apcd"));

    /// A CFArray of CFStrings that contain the UUIDs of all the tap objects in the
    /// contained in the AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDevicePropertyTapList")]
    pub const AGGREGATE_DEVICE_TAP_LIST: Self = Self(u32::from_be_bytes(*b"tap#"));

    /// An array of AudioObjectIDs for all the active sub-taps in the aggregate device.
    #[doc(alias = "kAudioAggregateDevicePropertySubTapList")]
    pub const AGGREGATE_DEVICE_SUB_TAP_LIST: Self = Self(u32::from_be_bytes(*b"atap"));
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Device(pub Obj);

impl std::ops::Deref for Device {
    type Target = Obj;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Device {
    pub fn uid(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&PropSelector::DEVICE_UID.global_addr())
    }

    pub fn nominal_sample_rate(&self) -> os::Result<f64> {
        self.prop(&PropSelector::DEVICE_NOMINAL_SAMPLE_RATE.global_addr())
    }

    pub fn stream_cfg(&self, scope: PropScope) -> os::Result<AudioBufListN> {
        let addr = PropAddr {
            selector: PropSelector::DEVICE_STREAM_CFG,
            scope,
            element: PropElement::WILDCARD,
        };
        let mut size = self.prop_size(&addr)?;
        let mut res = AudioBufListN::new(size as _);
        let obj = Obj(self.0 .0);
        unsafe {
            AudioObjectGetPropertyData(
                obj,
                &addr,
                0,
                std::ptr::null(),
                &mut size,
                res.as_mut_ptr() as _,
            )
        }
        .result()?;
        Ok(res)
    }

    pub fn input_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(PropScope::INPUT)
    }

    pub fn output_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(PropScope::OUTPUT)
    }

    #[doc(alias = "AudioDeviceCreateIOProcID")]
    pub fn create_io_proc_id<const IN: usize, const ON: usize, T>(
        &self,
        proc: DeviceIoProc<IN, ON, T>,
        client_data: Option<&mut T>,
    ) -> os::Result<DeviceIoProcId> {
        let mut res = None;
        let obj = Obj(self.0 .0);
        unsafe {
            AudioDeviceCreateIOProcID(
                obj,
                std::mem::transmute(proc),
                std::mem::transmute(client_data),
                &mut res,
            )
            .result()?;
            Ok(res.unwrap_unchecked())
        }
    }

    #[doc(alias = "AudioDeviceCreateIOProcIDWithBlock")]
    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    pub fn create_io_proc_id_with_block<const IN: usize, const ON: usize>(
        &self,
        dispatch_queue: Option<&dispatch::Queue>,
        block: &mut DeviceIoBlock<IN, ON>,
    ) -> os::Result<DeviceIoProcId> {
        let mut res = None;
        let obj = Obj(self.0 .0);
        unsafe {
            AudioDeviceCreateIOProcIDWithBlock(
                &mut res,
                obj,
                dispatch_queue,
                std::mem::transmute(block),
            )
            .result()?;
            Ok(res.unwrap_unchecked())
        }
    }
}

/// AudioDevice Properties
impl PropSelector {
    /// An os::Status that contains any error codes generated by loading the IOAudio
    /// driver plug-in for the AudioDevice or kAudioHardwareNoError if the plug-in
    /// loaded successfully. This property only exists for IOAudio-based
    /// AudioDevices whose driver has specified a plug-in to load.
    #[doc(alias = "kAudioDevicePropertyPlugIn")]
    pub const DEVICE_PLUG_IN: Self = Self(u32::from_be_bytes(*b"plug"));

    /// The type of this property is a UInt32, but its value has no meaning. This
    /// property exists so that clients can listen to it and be told when the
    /// configuration of the AudioDevice has changed in ways that cannot otherwise
    /// be conveyed through other notifications. In response to this notification,
    /// clients should re-evaluate everything they need to know about the device,
    /// particularly the layout and values of the controls.
    #[doc(alias = "kAudioDevicePropertyDeviceHasChanged")]
    pub const DEVICE_HAS_CHANGED: Self = Self(u32::from_be_bytes(*b"diff"));

    /// u32 where 1 means that the AudioDevice is running in at least one
    /// process on the system and 0 means that it isn't running at all.
    #[doc(alias = "kAudioDevicePropertyDeviceIsRunningSomewhere")]
    pub const DEVICE_IS_RUNNING_SOMEWHERE: Self = Self(u32::from_be_bytes(*b"gone"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when the AudioDevice detects that an IO cycle has
    /// run past its deadline. Note that the notification for this property is
    /// usually sent from the AudioDevice's IO thread.
    #[doc(alias = "kAudioDeviceProcessorOverload")]
    pub const DEVICE_PROCESSOR_OVERLOAD: Self = Self(u32::from_be_bytes(*b"over"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when IO on the device has stopped outside of the
    /// normal mechanisms. This typically comes up when IO is stopped after
    /// AudioDeviceStart has returned successfully but prior to the notification for
    /// kAudioDevicePropertyIsRunning being sent.
    #[doc(alias = "kAudioDevicePropertyIOStoppedAbnormally")]
    pub const DEVICE_IO_STOPPED_ABNORMALLY: Self = Self(u32::from_be_bytes(*b"stpd"));

    /// A pid_t indicating the process that currently owns exclusive access to the
    /// AudioDevice or a value of -1 indicating that the device is currently
    /// available to all processes.

    /// If the AudioDevice is in a non-mixable mode,
    /// the HAL will automatically take hog mode on behalf of the first process to
    /// start an IOProc.
    ///
    /// Note that when setting this property, the value passed in is ignored. If
    /// another process owns exclusive access, that remains unchanged. If the
    /// current process owns exclusive access, it is released and made available to
    /// all processes again. If no process has exclusive access (meaning the current
    /// value is -1), this process gains ownership of exclusive access.  On return,
    /// the pid_t pointed to by inPropertyData will contain the new value of the
    /// property.
    #[doc(alias = "kAudioDevicePropertyHogMode")]
    pub const DEVICE_HOG_MODE: Self = Self(u32::from_be_bytes(*b"oink"));

    /// A u32 whose value indicates the number of frames in the IO buffers.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSize")]
    pub const DEVICE_BUF_FRAME_SIZE: Self = Self(u32::from_be_bytes(*b"fsiz"));

    /// An AudioValueRange indicating the minimum and maximum values, inclusive, for
    /// kAudioDevicePropertyBufferFrameSize.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSizeRange")]
    pub const DEVICE_BUF_FRAME_SIZE_RANGE: Self = Self(u32::from_be_bytes(*b"fsz#"));

    /// A u32 that, if implemented by a device, indicates that the sizes of the
    /// buffers passed to an IOProc will vary by a small amount. The value of this
    /// property will indicate the largest buffer that will be passed and
    /// kAudioDevicePropertyBufferFrameSize will indicate the smallest buffer that
    /// will get passed to the IOProc. The usage of this property is narrowed to
    /// only allow for devices whose buffer sizes vary by small amounts greater than
    /// kAudioDevicePropertyBufferFrameSize. It is not intended to be a license for
    /// devices to be able to send buffers however they please. Rather, it is
    /// intended to allow for hardware whose natural rhythms lead to this necessity.
    #[doc(alias = "kAudioDevicePropertyUsesVariableBufferFrameSizes")]
    pub const DEVICE_USES_VARIABLE_BUF_FRAME_SIZES: Self = Self(u32::from_be_bytes(*b"vfsz"));

    /// A f32 whose range is from 0 to 1. This value indicates how much of the
    /// client portion of the IO cycle the process will use. The client portion of
    /// the IO cycle is the portion of the cycle in which the device calls the
    /// IOProcs so this property does not the apply to the duration of the entire
    /// cycle.
    #[doc(alias = "kAudioDevicePropertyIOCycleUsage")]
    pub const DEVICE_IO_CYCLE_USAGE: Self = Self(u32::from_be_bytes(*b"ncyc"));

    /// This property returns the stream configuration of the device in an
    /// AudioBufListN (with the buffer pointers set to NULL) which describes the
    /// list of streams and the number of channels in each stream. This corresponds
    /// to what will be passed into the IOProc.
    #[doc(alias = "kAudioDevicePropertyStreamConfiguration")]
    pub const DEVICE_STREAM_CFG: Self = Self(u32::from_be_bytes(*b"slay"));

    /// An AudioHardwareIOProcStreamUsage structure which details the stream usage
    /// of a given IO proc. If a stream is marked as not being used, the given
    /// IOProc will see a corresponding NULL buffer pointer in the AudioBufferList
    /// passed to its IO proc. Note that the number of streams detailed in the
    /// AudioHardwareIOProcStreamUsage must include all the streams of that
    /// direction on the device. Also, when getting the value of the property, one
    /// must fill out the mIOProc field of the AudioHardwareIOProcStreamUsage with
    /// the address of the of the IOProc whose stream usage is to be retrieved.
    #[doc(alias = "kAudioDevicePropertyIOProcStreamUsage")]
    pub const DEVICE_IO_PROC_STREAM_USAGE: Self = Self(u32::from_be_bytes(*b"suse"));

    /// A f64 that indicates the current actual sample rate of the AudioDevice
    /// as measured by its time stamps.
    #[doc(alias = "kAudioDevicePropertyActualSampleRate")]
    pub const DEVICE_ACTUAL_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"asrt"));

    /// A cf::String that contains the UID for the AudioClockDevice that is currently
    /// serving as the main time base of the device. The caller is responsible
    /// for releasing the returned cf::String.
    #[doc(alias = "kAudioDevicePropertyClockDevice")]
    pub const DEVICE_CLOCK_DEVICE: Self = Self(u32::from_be_bytes(*b"apcd"));

    /// An os_workgroup_t that represents the thread workgroup the AudioDevice's
    /// IO thread belongs to. The caller is responsible for releasing the returned
    /// object.
    #[doc(alias = "kAudioDevicePropertyIOThreadOSWorkgroup")]
    pub const DEVICE_IO_THREAD_OS_WORKGROUP: Self = Self(u32::from_be_bytes(*b"oswg"));

    /// A u32 where a non-zero value indicates that the current process's audio
    /// will be zeroed out by the system. Note that this property does not apply to
    /// aggregate devices, just real, physical devices.
    #[doc(alias = "kAudioDevicePropertyProcessMute")]
    pub const DEVICE_PROCESS_MUTE: Self = Self(u32::from_be_bytes(*b"appm"));
}

/// AudioDevice Properties Implemented via AudioControl objects
///
/// AudioObjectPropertySelector values for AudioDevice properties that are
/// implemented by AudioControl objects.
///
/// These properties are also accessible by locating the AudioControl object
/// attached to the AudioDevice and using that object to access the properties of
/// the control.
impl PropSelector {
    /// A u32 where a value of 0 means that there isn't anything plugged into the
    /// jack associated withe given element and scope. This property is implemented
    /// by an AudioJackControl, a subclass of AudioBooleanControl.
    #[doc(alias = "kAudioDevicePropertyJackIsConnected")]
    pub const DEVICE_JACK_IS_CONNECTED: Self = Self(u32::from_be_bytes(*b"jack"));

    /// A f32 that represents the value of the volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all Float32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the Float32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeScalar")]
    pub const DEVICE_VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"volm"));

    /// A f32 that represents the value of the volume control in dB. Note that
    /// the set of all f32 values in the dB range for the control is much larger
    /// than the set of actual values that the hardware can select. This means that
    /// the f32 range has a many to one mapping with the underlying hardware
    /// values. As such, setting a dB value will result in the control taking on the
    /// value nearest to what was set. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeDecibels")]
    pub const DEVICE_VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"vold"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeRangeDecibels")]
    pub const DEVICE_VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"vdb#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeScalarToDecibels")]
    pub const DEVICE_VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"v2db"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeDecibelsToScalar")]
    pub const DEVICE_VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"db2v"));

    /// A f32 where 0.0 is full left, 1.0 is full right, and 0.5 is center. This
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioStereoPanControl.
    #[doc(alias = "kAudioDevicePropertyStereoPan")]
    pub const DEVICE_STEREO_PAN: Self = Self(u32::from_be_bytes(*b"span"));

    /// An array of two u32s that indicate which elements of the owning object
    /// the signal is being panned between. This property is implemented by an
    /// AudioControl object that is a subclass of AudioStereoPanControl.
    #[doc(alias = "kAudioDevicePropertyStereoPanChannels")]
    pub const DEVICE_STEREO_PAN_CHANNELS: Self = Self(u32::from_be_bytes(*b"spn#"));

    /// A u32 where a value of 1 means that mute is enabled making that element
    /// inaudible. The property is implemented by an AudioControl object that is a
    /// subclass of AudioMuteControl.
    #[doc(alias = "kAudioDevicePropertyMute")]
    pub const DEVICE_MUTE: Self = Self(u32::from_be_bytes(*b"mute"));

    /// A u32 where a value of 1 means that just that element is audible and the
    /// other elements are inaudible. The property is implemented by an AudioControl
    /// object that is a subclass of AudioSoloControl.
    #[doc(alias = "kAudioDevicePropertySolo")]
    pub const DEVICE_SOLO: Self = Self(u32::from_be_bytes(*b"solo"));

    /// A u32 where a value of 1 means that the AudioDevice has enabled phantom
    /// power for the given element. The property is implemented by an AudioControl
    /// object that is a subclass of AudioPhantomPowerControl.
    #[doc(alias = "kAudioDevicePropertyPhantomPower")]
    pub const DEVICE_PHANTOM_POWER: Self = Self(u32::from_be_bytes(*b"phan"));

    /// A u32 where a value of 1 means that phase of the signal for the given
    /// element has been flipped 180 degrees. The property is implemented by an
    /// AudioControl object that is a subclass of AudioPhaseInvertControl.
    #[doc(alias = "kAudioDevicePropertyPhaseInvert")]
    pub const DEVICE_PHASE_INVERT: Self = Self(u32::from_be_bytes(*b"phsi"));

    /// A u32 where a value of 1 means that the signal for the element has
    /// exceeded the sample range. Once a clip light is turned on, it is to stay on
    /// until either the value of the control is set to false or the current IO
    /// session stops and a new IO session starts. The property is implemented by an
    /// AudioControl object that is a subclass of AudioClipLightControl.
    #[doc(alias = "kAudioDevicePropertyClipLight")]
    pub const DEVICE_CLIP_LIGHT: Self = Self(u32::from_be_bytes(*b"clip"));

    /// A u32 where a value of 1 means that the talkback channel is enabled. The
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioTalkbackControl.
    #[doc(alias = "kAudioDevicePropertyTalkback")]
    pub const DEVICE_TALKBACK: Self = Self(u32::from_be_bytes(*b"talb"));

    /// A u32 where a value of 1 means that the listenback channel is enabled.
    /// The property is implemented by an AudioControl object that is a subclass of
    /// AudioListenbackControl.
    #[doc(alias = "kAudioDevicePropertyListenback")]
    pub const DEVICE_LISTENBACK: Self = Self(u32::from_be_bytes(*b"lsnb"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// data sources. This property is implemented by an AudioControl object that is
    /// a subclass of AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSource")]
    pub const DEVICE_DATA_SRC: Self = Self(u32::from_be_bytes(*b"ssrc"));

    /// An array of u32s that are represent all the IDs of all the data sources
    /// currently available. This property is implemented by an AudioControl object
    /// that is a subclass of AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSources")]
    pub const DEVICE_DATA_SRCS: Self = Self(u32::from_be_bytes(*b"ssc#"));

    /// This property translates the given data source item ID into a human readable
    /// name using an AudioValueTranslation structure. The input data is the u32
    /// containing the item ID to translated and the output data is a cf::String. The
    /// caller is responsible for releasing the returned cf::Object. This property is
    /// implemented by an AudioControl object that is a subclass of
    /// AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSourceNameForIDCFString")]
    pub const DEVICE_DATA_SRC_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"lscn"));

    /// This property returns a u32 that identifies the kind of data source
    /// the item ID refers to using an AudioValueTranslation structure. The input
    /// data is the u32 containing the item ID and the output data is the u32.
    #[doc(alias = "kAudioDevicePropertyDataSourceKindForID")]
    pub const DEVICE_DATA_SRC_KIND_FOR_ID: Self = Self(u32::from_be_bytes(*b"ssck"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// clock sources. This property is implemented by an AudioControl object that
    /// is a subclass of AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSource")]
    pub const DEVICE_CLOCK_SRC: Self = Self(u32::from_be_bytes(*b"csrc"));

    /// An array of u32s that are represent all the IDs of all the clock sources
    /// currently available. This property is implemented by an AudioControl object
    /// that is a subclass of AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSources")]
    pub const DEVICE_CLOCK_SRCS: Self = Self(u32::from_be_bytes(*b"csc#"));

    /// This property translates the given clock source item ID into a human
    /// readable name using an AudioValueTranslation structure. The input data is
    /// the u32 containing the item ID to translated and the output data is a
    /// cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSourceNameForIDCFString")]
    pub const DEVICE_CLOCK_SRC_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"lcsn"));

    /// This property returns a u32 that identifies the kind of clock source
    /// the item ID refers to using an AudioValueTranslation structure. The input
    /// data is the u32 containing the item ID and the output data is the u32.
    #[doc(alias = "kAudioDevicePropertyClockSourceKindForID")]
    pub const DEVICE_CLOCK_SRC_KIND_FOR_ID: Self = Self(u32::from_be_bytes(*b"csck"));

    /// A u32 where a value of 0 means that play through is off and a value of 1
    /// means that it is on. This property is implemented by an AudioControl object
    /// that is a subclass of AudioMuteControl. Further, the control that implements
    /// this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThru")]
    pub const DEVICE_PLAY_THRU: Self = Self(u32::from_be_bytes(*b"thru"));

    /// A u32 where a value of 1 means that just that play through element is
    /// audible and the other elements are inaudible. The property is implemented by
    /// an AudioControl object that is a subclass of AudioSoloControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruSolo")]
    pub const DEVICE_PLAY_THRU_SOLO: Self = Self(u32::from_be_bytes(*b"thrs"));

    /// A f32 that represents the value of the volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all f32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the f32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioVolumeControl.Further, the control that implements this
    /// property is only available through kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeScalar")]
    pub const DEVICE_PLAY_THRU_VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"mvsc"));

    /// A f32 that represents the value of the volume control in dB. Note that
    /// the set of all f32 values in the dB range for the control is much larger
    /// than the set of actual values that the hardware can select. This means that
    /// the f32 range has a many to one mapping with the underlying hardware
    /// values. As such, setting a dB value will result in the control taking on the
    /// value nearest to what was set. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeDecibels")]
    pub const DEVICE_PLAY_THRU_VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"mvdb"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioVolumeControl. Further, the control that
    /// implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeRangeDecibels")]
    pub const DEVICE_PLAY_THRU_VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"mvd#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeScalarToDecibels")]
    pub const DEVICE_PLAY_THRU_VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"mv2d"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeDecibelsToScalar")]
    pub const DEVICE_PLAY_THRU_VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"mv2s"));

    /// A f32 where 0.0 is full left, 1.0 is full right, and 0.5 is center. This
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioStereoPanControl. Further, the control that implements this property is
    /// only available through kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruStereoPan")]
    pub const DEVICE_PLAY_THRU_STEREO_PAN: Self = Self(u32::from_be_bytes(*b"mspn"));

    /// An array of two u32s that indicate which elements of the owning object
    /// the signal is being panned between. This property is implemented by an
    /// AudioControl object that is a subclass of AudioStereoPanControl. Further,
    /// the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruStereoPanChannels")]
    pub const DEVICE_PLAY_THRU_STEREO_PAN_CHANNELS: Self = Self(u32::from_be_bytes(*b"msp#"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// play through data destinations. This property is implemented by an
    /// AudioControl object that is a subclass of AudioDataDestinationControl.
    /// Further, the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestination")]
    pub const DEVICE_PLAY_THRU_DST: Self = Self(u32::from_be_bytes(*b"mdds"));

    /// An array of u32s that are represent all the IDs of all the play through
    /// data destinations currently available. This property is implemented by an
    /// AudioControl object that is a subclass of AudioDataDestinationControl.
    /// Further, the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestinations")]
    pub const DEVICE_PLAY_THRU_DSTS: Self = Self(u32::from_be_bytes(*b"mdd#"));

    /// This property translates the given play through data destination item ID
    /// into a human readable name using an AudioValueTranslation structure. The
    /// input data is the u32 containing the item ID to translated and the output
    /// data is a cf::String. The caller is responsible for releasing the returned
    /// cf::Object. This property is implemented by an AudioControl object that is a
    /// subclass of AudioDataDestinationControl. Further, the control that
    /// implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestinationNameForIDCFString")]
    pub const DEVICE_PLAY_THRU_DST_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"mddc"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// nominal line levels. This property is implemented by an AudioControl object
    /// that is a subclass of AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevel")]
    pub const DEVICE_CHANNEL_NOMINAL_LINE_LEVEL: Self = Self(u32::from_be_bytes(*b"nlvl"));

    /// An array of u32s that represent all the IDs of all the nominal line
    /// levels currently available. This property is implemented by an AudioControl
    /// object that is a subclass of AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevels")]
    pub const DEVICE_CHANNEL_NOMINAL_LINE_LEVELS: Self = Self(u32::from_be_bytes(*b"nlv#"));

    /// This property translates the given nominal line level item ID into a human
    /// readable name using an AudioValueTranslation structure. The input data is
    /// the u32 containing the item ID to be translated and the output data is a
    /// cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString")]
    pub const DEVICE_CHANNEL_NOMINAL_LINE_LEVEL_NAME_FOR_IDCF_STR: Self =
        Self(u32::from_be_bytes(*b"lcnl"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// high pass filter setting. This property is implemented by an AudioControl
    /// object that is a subclass of AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSetting")]
    pub const DEVICE_HIGH_PASS_FILTER_SETTING: Self = Self(u32::from_be_bytes(*b"hipf"));

    /// An array of u32s that represent all the IDs of all the high pass filter
    /// settings currently available. This property is implemented by an
    /// AudioControl object that is a subclass of AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSettings")]
    pub const DEVICE_HIGH_PASS_FILTER_SETTINGS: Self = Self(u32::from_be_bytes(*b"hip#"));

    /// This property translates the given high pass filter setting item ID into a
    /// human readable name using an AudioValueTranslation structure. The input data
    /// is the u32 containing the item ID to be translated and the output data is
    /// a cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSettingNameForIDCFString")]
    pub const DEVICE_HIGH_PASS_FILTER_SETTING_NAME_FOR_IDCF_STR: Self =
        Self(u32::from_be_bytes(*b"hipl"));

    /// A f32 that represents the value of the LFE volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all f32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the f32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeScalar")]
    pub const DEVICE_SUB_VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"svlm"));

    /// A f32 that represents the value of the LFE volume control in dB. Note
    /// that the set of all f32 values in the dB range for the control is much
    /// larger than the set of actual values that the hardware can select. This
    /// means that the f32 range has a many to one mapping with the underlying
    /// hardware values. As such, setting a dB value will result in the control
    /// taking on the value nearest to what was set. This property is implemented by
    /// an AudioControl object that is a subclass of AudioLFE VolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeDecibels")]
    pub const DEVICE_SUB_VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"svld"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeRangeDecibels")]
    pub const DEVICE_SUB_VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"svd#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeScalarToDecibels")]
    pub const DEVICE_SUB_VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"sv2d"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeDecibelsToScalar")]
    pub const DEVICE_SUB_VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"sd2v"));

    /// A u32 where a value of 1 means that mute is enabled making the LFE on
    ///
    /// that element inaudible. The property is implemented by an AudioControl
    /// object that is a subclass of AudioLFEMuteControl.
    #[doc(alias = "kAudioDevicePropertySubMute")]
    pub const DEVICE_SUB_MUTE: Self = Self(u32::from_be_bytes(*b"smut"));

    /// A u32 where 0 disables voice activity detection process and non-zero enables it.
    ///
    /// Voice activity detection can be used with input audio and has echo cancellation.
    /// Detection works when a process mute is used, but not with hardware mute.
    #[doc(alias = "kAudioDevicePropertyVoiceActivityDetectionEnable")]
    pub const DEVICE_VOICE_ACTIVITY_DETECTION_ENABLE: Self = Self(u32::from_be_bytes(*b"vAd+"));

    /// A read-only u32 where 0 indicates no voice currently detected and 1 indicates voice.
    ///
    /// Used in conjunction with VOICE_ACTIVITY_DETECTION_ENABLE.
    /// A client would normally register to listen to this property for changes and then query
    /// the state rather than continuously poll the value.
    /// NOTE: If input audio is not active/runnning or the voice activity detection is disabled,
    /// then it is not analyzed and this will provide 0.
    #[doc(alias = "kAudioDevicePropertyVoiceActivityDetectionState")]
    pub const DEVICE_VOICE_ACTIVITY_DETECTION_STATE: Self = Self(u32::from_be_bytes(*b"vAdS"));
}

/// AudioObjectPropertySelector values provided by the Tap Object class.
/// The Tap class is a subclass of the AudioObject class. the class
/// has just the global scope, kAudioObjectPropertyScopeGlobal, and only a master element.
impl PropSelector {
    /// A cf::String that contains a persistent identifier for the Tap. A Taps UID
    /// persists until the tap is destroyed. The caller is responsible for releasing
    /// the returned cf::Object.
    #[doc(alias = "kAudioTapPropertyUID")]
    pub const TAP_UID: Self = Self(u32::from_be_bytes(*b"tuid"));

    /// The CATapDescription used to initially create this tap. This property can be used
    /// to modify and set the description of an existing tap.
    #[doc(alias = "kAudioTapPropertyDescription")]
    pub const TAP_DESCRIPTION: Self = Self(u32::from_be_bytes(*b"tdsc"));

    /// An AudioStreamBasicDescription that describes the current data format for
    /// the tap. This is the format of that data that will be accessible in any aggregate
    /// device that contains the tap.
    #[doc(alias = "kAudioTapPropertyFormat")]
    pub const TAP_FORMAT: Self = Self(u32::from_be_bytes(*b"tfmt"));
}

#[doc(alias = "AudioDeviceIOProc")]
pub type DeviceIoProc<const IN: usize = 1, const ON: usize = 1, T = std::ffi::c_void> =
    extern "C" fn(
        device: Device,
        now: &cat::AudioTimeStamp,
        input_data: &cat::AudioBufList<IN>,
        input_time: &cat::AudioTimeStamp,
        output_data: &mut cat::AudioBufList<ON>,
        output_time: &cat::AudioTimeStamp,
        client_data: Option<&mut T>,
    ) -> os::Status;

#[doc(alias = "AudioDeviceIOBlock")]
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub type DeviceIoBlock<const IN: usize = 1, const ON: usize = 1> = blocks::EscBlock<
    fn(
        now: &cat::AudioTimeStamp,
        input_data: &cat::AudioBufList<IN>,
        input_time: &cat::AudioTimeStamp,
        output_data: &mut cat::AudioBufList<ON>,
        output_time: &cat::AudioTimeStamp,
    ),
>;

pub type DeviceIoProcId = DeviceIoProc;

#[repr(transparent)]
pub struct AggregateDevice(Device);

impl std::ops::Deref for AggregateDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for AggregateDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Device> for Device {
    fn as_ref(&self) -> &Device {
        self
    }
}

impl AsRef<Device> for AggregateDevice {
    fn as_ref(&self) -> &Device {
        &self.0
    }
}

pub struct StartedDevice<D: AsRef<Device>> {
    device: D,
    proc_id: Option<DeviceIoProcId>,
}

impl<D: AsRef<Device>> Drop for StartedDevice<D> {
    fn drop(&mut self) {
        let device = Device(self.device.as_ref().0);
        let res = unsafe { AudioDeviceStop(device, self.proc_id) };
        debug_assert!(res.is_ok());
    }
}

#[doc(alias = "AudioDeviceStart")]
pub fn device_start<D: AsRef<Device>>(
    device: D,
    proc_id: Option<DeviceIoProcId>,
) -> os::Result<StartedDevice<D>> {
    let dev = Device(device.as_ref().0);
    unsafe { AudioDeviceStart(dev, proc_id).result()? }
    Ok(StartedDevice { device, proc_id })
}

mod common_keys {
    use crate::cf;

    pub fn uid() -> &'static cf::String {
        cf::str!(c"uid")
    }

    pub fn name() -> &'static cf::String {
        cf::str!(c"name")
    }
}

pub mod sub_tap_keys {
    use crate::cf;

    #[doc(alias = "kAudioSubTapUIDKey")]
    pub use super::common_keys::uid;

    #[doc(alias = "kAudioSubTapDriftCompensationKey")]
    pub fn drift_compensation() -> &'static cf::String {
        cf::str!(c"drift")
    }
}

pub mod sub_device_keys {
    #[doc(alias = "kAudioSubDeviceUIDKey")]
    pub use super::common_keys::uid;

    #[doc(alias = "kAudioSubDeviceNameKey")]
    pub use super::common_keys::name;
}

pub mod aggregate_device_keys {
    use crate::cf;

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the UID
    /// of the AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceUIDKey")]
    pub use super::common_keys::uid;

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the
    /// human readable name of the AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceNameKey")]
    pub use super::common_keys::name;

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFArray of CFDictionaries that
    /// describe each sub-device in the AudioAggregateDevice. The keys for this
    /// CFDictionary are defined in the AudioSubDevice section.
    #[doc(alias = "kAudioAggregateDeviceSubDeviceListKey")]
    pub fn sub_device_list() -> &'static cf::String {
        cf::str!(c"subdevices")
    }

    /// The key used in a cf::Dictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a cf::String that contains the
    /// UID for the sub-device that is the time source for the
    /// AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceMainSubDeviceKey")]
    pub fn main_sub_device() -> &'static cf::String {
        cf::str!(c"master")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the
    /// UID for the clock device that is the time source for the
    /// AudioAggregateDevice. If the aggregate device includes both a main audio
    /// device and a clock device, the clock device will control the time base.
    #[doc(alias = "kAudioAggregateDeviceClockDeviceKey")]
    pub fn clock_device() -> &'static cf::String {
        cf::str!(c"clock")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a value of 0
    /// means that the AudioAggregateDevice is to be published to the entire system and
    /// a value of 1 means that the AudioAggregateDevice is private to the process that
    /// created it. Note that a private AudioAggregateDevice is not persistent across
    /// launches of the process that created it. Note that if this key is not present,
    /// it implies that the AudioAggregateDevice is published to the entire system.
    #[doc(alias = "kAudioAggregateDeviceIsPrivateKey")]
    pub fn is_private() -> &'static cf::String {
        cf::str!(c"private")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a value of 0
    /// means that the sub-devices of the AudioAggregateDevice are arranged such that
    /// the output streams are all fed the same data.
    #[doc(alias = "kAudioAggregateDeviceIsStackedKey")]
    pub fn is_stacked() -> &'static cf::String {
        cf::str!(c"stacked")
    }

    /// The key used in a CFDictionary that describes the Tap composition of an
    /// AudioAggregateDevice.  The value for this key is a CFArray of CFDictionaries
    /// that describe each tap in the AudioAggregateDevice.  The keys for this
    /// CFDictionary are defined in the AudioTap section.
    #[doc(alias = "kAudioAggregateDeviceTapListKey")]
    pub fn tap_list() -> &'static cf::String {
        cf::str!(c"taps")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a non-zero  
    /// value indicates that this aggregate device’s start should wait for the first
    /// tap that receives audio. When this key is used, calling AudioDeviceStart with
    /// the aggregate device will wait until a tapped process begins receiving its  
    /// first audio from any tapped applications. The composition must also include
    /// the private key so that the aggregate is private to the process that created
    /// it.
    #[doc(alias = "kAudioAggregateDeviceTapAutoStartKey")]
    pub fn tap_auto_start() -> &'static cf::String {
        cf::str!(c"tapautostart")
    }
}

impl AggregateDevice {
    pub fn with_desc(desc: &cf::DictionaryOf<cf::String, cf::Type>) -> os::Result<Self> {
        let mut res = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioHardwareCreateAggregateDevice(desc, res.as_mut_ptr()).result()?;
            Ok(res.assume_init())
        }
    }

    pub fn composition(&self) -> os::Result<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        self.cf_prop(&PropSelector::AGGREGATE_DEVICE_COMPOSITION.global_addr())
    }
}

impl Drop for AggregateDevice {
    fn drop(&mut self) {
        unsafe {
            let mut id = Self(Device(Obj::UNKNOWN));
            std::mem::swap(&mut self.0, &mut id);
            let res = AudioHardwareDestroyAggregateDevice(id).result();
            debug_assert!(res.is_ok());
        }
    }
}

#[link(name = "CoreAudio", kind = "framework")]
extern "C-unwind" {

    fn AudioObjectShow(objectId: Obj);

    fn AudioObjectHasProperty(objectId: Obj, address: *const PropAddr) -> bool;

    fn AudioObjectIsPropertySettable(
        objectId: Obj,
        address: *const PropAddr,
        out_is_settable: *mut bool,
    ) -> os::Status;

    fn AudioObjectGetPropertyData(
        objectId: Obj,
        address: *const PropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
        data: *mut c_void,
    ) -> os::Status;

    fn AudioObjectSetPropertyData(
        objectId: Obj,
        address: *const PropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn AudioObjectGetPropertyDataSize(
        objectId: Obj,
        address: *const PropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
    ) -> os::Status;

    fn AudioObjectAddPropertyListener(
        objectId: Obj,
        address: *const PropAddr,
        listener: PropListenerFn,
        client_data: *mut c_void,
    ) -> os::Status;

    fn AudioObjectRemovePropertyListener(
        objectId: Obj,
        address: *const PropAddr,
        listener: PropListenerFn,
        client_data: *mut c_void,
    ) -> os::Status;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    fn AudioObjectAddPropertyListenerBlock(
        objectId: Obj,
        address: *const PropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: *mut PropListenerBlock,
    ) -> os::Status;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    fn AudioObjectRemovePropertyListenerBlock(
        objectId: Obj,
        address: *const PropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: *mut PropListenerBlock,
    ) -> os::Status;

    fn AudioDeviceCreateIOProcID(
        device: Obj,
        proc: DeviceIoProc,
        client_data: *mut std::ffi::c_void,
        out_proc_id: *mut Option<DeviceIoProcId>,
    ) -> os::Status;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    fn AudioDeviceCreateIOProcIDWithBlock(
        out_proc_id: *mut Option<DeviceIoProcId>,
        device: Obj,
        dispatch_queue: Option<&dispatch::Queue>,
        block: &mut DeviceIoBlock,
    ) -> os::Status;

    fn AudioHardwareCreateAggregateDevice(
        desc: &cf::DictionaryOf<cf::String, cf::Type>,
        out_device_id: *mut AggregateDevice,
    ) -> os::Status;

    fn AudioHardwareDestroyAggregateDevice(device_id: AggregateDevice) -> os::Status;

    fn AudioDeviceStart(device: Device, proc_id: Option<DeviceIoProcId>) -> os::Status;
    fn AudioDeviceStop(device: Device, proc_id: Option<DeviceIoProcId>) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::{
        cat, cf,
        core_audio::{
            aggregate_device_keys as agg_keys, AggregateDevice, Class, Device, Obj, Process,
            PropAddr, PropElement, PropScope, PropSelector, System, TapDesc,
        },
        ns, os,
    };

    #[test]
    fn list_devices() {
        let addr = PropSelector::HARDWARE_DEFAULT_INPUT_DEVICE.global_addr();
        let _device: Device = System::OBJ.prop(&addr).unwrap();

        let addr = PropAddr {
            selector: PropSelector::HARDWARE_DEVICES,
            scope: PropScope::INPUT,
            element: PropElement::MAIN,
        };
        let devices: Vec<Device> = System::OBJ.prop_vec(&addr).unwrap();

        assert!(!devices.is_empty());
    }

    #[test]
    fn tap() {
        let desc = TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
        let tap = desc.create_process_tap().unwrap();
        assert_eq!(tap.class().unwrap(), Class::TAP);
        assert_eq!(tap.base_class().unwrap(), Class::OBJECT);
    }

    #[test]
    fn aggregate_device() {
        let output_device = System::default_output_device().unwrap();
        let output_uid = output_device.uid().unwrap();
        let uuid = cf::Uuid::new().to_cf_string();
        let dict = cf::DictionaryOf::with_keys_values(
            &[
                agg_keys::is_private(),
                agg_keys::is_stacked(),
                agg_keys::tap_auto_start(),
                agg_keys::name(),
                agg_keys::main_sub_device(),
                agg_keys::uid(),
            ],
            &[
                cf::Boolean::value_true().as_type_ref(),
                cf::Boolean::value_false(),
                cf::Boolean::value_true(),
                cf::str!(c"Tap"),
                &output_uid,
                &uuid,
            ],
        );
        let agg_device = AggregateDevice::with_desc(&dict).unwrap();
        assert_eq!(agg_device.class().unwrap(), Class::AGGREGATE_DEVICE);
        assert_eq!(agg_device.base_class().unwrap(), Class::DEVICE);

        extern "C" fn proc(
            _device: Device,
            _now: &cat::AudioTimeStamp,
            _input_data: &cat::AudioBufList<1>,
            _input_time: &cat::AudioTimeStamp,
            _output_data: &mut cat::AudioBufList<1>,
            _output_time: &cat::AudioTimeStamp,
            _client_data: Option<&mut std::ffi::c_void>,
        ) -> os::Status {
            os::Status::NO_ERR
        }

        let _proc_id = agg_device.create_io_proc_id(proc, None).unwrap();
    }

    #[test]
    fn process() {
        let obj = Process::with_pid(0).unwrap();
        assert_eq!(obj.0, Obj::UNKNOWN);

        let objs = Process::list().unwrap();
        for obj in objs {
            obj.bundle_id().unwrap().show();
        }

        let pid = ns::ProcessInfo::current().process_id();
        let obj = Process::with_pid(pid).unwrap();
        assert_ne!(obj.0, Obj::UNKNOWN);
        assert_eq!(obj.base_class().unwrap(), Class::OBJECT);
        let process_id = obj.pid().unwrap();
        assert_eq!(pid, process_id);
        println!("{:?}", obj.bundle_id());

        assert!(!obj.is_running().unwrap());

        let _devices = obj.devices().unwrap();
    }

    #[test]
    fn system() {
        assert_eq!(System::OBJ.class().unwrap(), Class::SYSTEM);
        assert!(System::OBJ.base_class().is_err());
    }
}
