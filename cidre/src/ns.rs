use crate::objc;

pub use objc::{
    ns::{Integer, UInteger},
    Class, Id, Sel,
};

mod range;
pub use range::Range;

mod process_info;
pub use process_info::ProcessInfo;
pub use process_info::ThermalState as ProcessInfoThermalState;

pub mod objc_runtime;
pub use objc_runtime::ExceptionName;

pub mod exception;
pub use exception::get_uncaought_exception_handler;
pub use exception::set_uncaught_exception_handler;
pub use exception::try_catch;
pub use exception::Exception;
pub use exception::UncaughtExceptionHandler;

mod port;
pub use port::MachPort;
pub use port::MachPortDelegate;
pub use port::MachPortDelegateImpl;
pub use port::Port;

mod url_request;
pub use url_request::Attribution as URLRequestAttribution;
pub use url_request::CachePolicy as URLRequestCachePolicy;
pub use url_request::NetworkServiceType as URLRequestNetworkServiceType;
pub use url_request::URLRequest;
pub use url_request::URLRequestMut;

mod url_response;
pub use url_response::HTTPURLResponse;
pub use url_response::URLResponse;

mod url_session;
pub use url_session::Configuration as URLSessionConfiguration;
pub use url_session::DataTask as URLSessionDataTask;
pub use url_session::DownloadTask as URLSessionDownloadTask;
pub use url_session::MultipathServiceType as URLSessionMultipathServiceType;
pub use url_session::Session as URLSession;
pub use url_session::StreamTask as URLSessionStreamTask;
pub use url_session::Task as URLSessionTask;
pub use url_session::TaskPriority as URLSessionTaskPriority;
pub use url_session::TaskState as URLSessionTaskState;
pub use url_session::UploadTask as URLSessionUploadTask;
pub use url_session::WebSocketCloseCode as URLSessionWebSocketCloseCode;
pub use url_session::WebSocketMessage as URLSessionWebSocketMessage;
pub use url_session::WebSocketMessageType as URLSessionWebSocketMessageType;
pub use url_session::WebSocketTask as URLSessionWebSocketTask;

mod url_cache;
pub use url_cache::CachedURLResponse;
pub use url_cache::StoragePolicy as URLCacheStoragePolicy;
pub use url_cache::URLCache;

mod value;
pub use value::Number;
pub use value::Value;

mod null;
pub use null::Null;

mod array;
pub use array::Array;
pub use array::ArrayMut;

mod dictionary;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryMut;

mod enumerator;
pub use enumerator::FEIterator;
pub use enumerator::FastEnumeration;
pub use enumerator::FastEnumerationState;

mod set;
pub use set::Set;
pub use set::SetMut;

mod data;
pub use data::Data;
pub use data::DataMut;
pub use data::ReadingOptions as DataReadingOptions;
pub use data::SearchOptions as DataSearchOptions;
pub use data::WritingOptions as DataWritingOptions;

mod regular_expression;
pub use regular_expression::MatchingFlags;
pub use regular_expression::MatchingOptions;
pub use regular_expression::Options as RegularExpressionOptions;
pub use regular_expression::RegularExpression;

mod text_checking_result;
pub use text_checking_result::TextCheckingResult;
pub use text_checking_result::Type as TextCheckingType;

mod string;
pub use string::Encoding as StringEncoding;
pub use string::String;
pub use string::StringMut;

mod url;
pub use url::ResourceKey as URLResourceKey;
pub use url::URL;

mod uuid;
pub use uuid::UUID;

mod run_loop;
pub use run_loop::RunLoop;
pub use run_loop::RunLoopMode;

mod date;
pub use date::Date;
pub use date::TimeInterval;
pub use date::TIME_INTERVAL_SINCE_1970;

mod error;
pub use error::Domain as ErrorDomain;
pub use error::Error;

mod timer;
pub use timer::Timer;

mod file_manager;
pub use file_manager::DirectoryEnumerationOptions;
pub use file_manager::FileAttributeKey;
pub use file_manager::FileAttributeType;
pub use file_manager::FileManager;
pub use file_manager::FileProtectionType;
pub use file_manager::ItemReplacementOptions as FileManagerItemReplacementOptions;
pub use file_manager::URLRelationship;
pub use file_manager::VolumeEnumerationOptions;

mod path_utilities;
pub use path_utilities::full_user_name;
pub use path_utilities::home_directory;
pub use path_utilities::home_directory_for_user;
pub use path_utilities::home_directory_for_user_ar;
pub use path_utilities::search_path_for_dirs_in_domains;
pub use path_utilities::search_path_for_dirs_in_domains_ar;
pub use path_utilities::temporary_directory;
pub use path_utilities::user_name;
pub use path_utilities::SearchPathDirectory;
pub use path_utilities::SearchPathDomainMask;

mod notification;
pub use notification::Notification;
pub use notification::NotificationCenter;
pub use notification::NotificationName;

mod operation;
pub use operation::Operation;
pub use operation::OperationQueue;

mod coder;
pub use coder::Coder;
pub use coder::DecodingFailurePolicy;

mod key_value_observing;
pub use key_value_observing::KVChange;
pub use key_value_observing::KVChangeKey;
pub use key_value_observing::KVOOptions;
pub use key_value_observing::KVSetMutationKind;
