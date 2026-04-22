//! C 호환 FFI 데이터 구조체 및 라이프사이클 함수 모듈

use std::ffi::{c_char, c_int, c_uchar, c_ulonglong, c_void};

/// FFI 성공 상태
pub const FFI_STATUS_OK: c_int = 0;
/// FFI 프레임 없음 상태
pub const FFI_STATUS_NO_FRAME: c_int = 1;
/// FFI 에러 상태
pub const FFI_STATUS_ERROR: c_int = -1;

/// C 호환 포인트 필드 레이아웃
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiPointField {
    pub name_ptr: *const c_char,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

/// C 호환 포인트 클라우드 프레임 메타데이터 및 포인터
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiPointCloudFrame {
    pub timestamp_ns: c_ulonglong,
    pub frame_id_ptr: *const c_char,
    pub width: u32,
    pub height: u32,
    pub point_step: u32,
    pub row_step: u32,
    pub fields_ptr: *const FfiPointField,
    pub fields_len: usize,
    pub is_dense: bool,
    pub data_ptr: *const c_uchar,
    pub data_len: usize,
}

/// 외부 노출 API 메타데이터 정보 포인터
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiApiInfo {
    pub name_ptr: *const c_char,
    pub description_ptr: *const c_char,
    pub input_schema_json_ptr: *const c_char,
    pub output_schema_json_ptr: *const c_char,
}

/// 단일 외부 API 결과 버퍼 포인터
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiApiBuffer {
    pub data_ptr: *const c_uchar,
    pub data_len: usize,
}

pub type EngineHandle = *mut c_void;

pub type EngineCreateFn = unsafe extern "C" fn(config_json_ptr: *const c_char) -> EngineHandle;

pub type EngineDestroyFn = unsafe extern "C" fn(handle: EngineHandle);

pub type EngineProcessPacketFn = unsafe extern "C" fn(
    handle: EngineHandle,
    data_ptr: *const c_uchar,
    data_len: usize,
    sender_info_ptr: *const c_char,
) -> c_int;

pub type EngineHasFrameFn = unsafe extern "C" fn(handle: EngineHandle) -> bool;

pub type EnginePopFrameFn =
    unsafe extern "C" fn(handle: EngineHandle, out_frame: *mut FfiPointCloudFrame) -> c_int;

pub type EngineFreeFrameFn = unsafe extern "C" fn(frame: *mut FfiPointCloudFrame);

pub type EngineGetApiCountFn = unsafe extern "C" fn(handle: EngineHandle) -> usize;

pub type EngineGetApiInfoFn =
    unsafe extern "C" fn(handle: EngineHandle, index: usize, out_info: *mut FfiApiInfo) -> c_int;

pub type EngineCallApiFn = unsafe extern "C" fn(
    handle: EngineHandle,
    api_name_ptr: *const c_char,
    input_json_ptr: *const c_char,
    out_buffer: *mut FfiApiBuffer,
) -> c_int;

pub type EngineFreeApiBufferFn = unsafe extern "C" fn(buffer: *mut FfiApiBuffer);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfiLogLevel {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

pub type FfiLogCallback =
    unsafe extern "C" fn(level: FfiLogLevel, target_ptr: *const c_char, message_ptr: *const c_char);

pub type EngineSetLoggerFn =
    unsafe extern "C" fn(callback: Option<FfiLogCallback>, level: FfiLogLevel) -> i32;
