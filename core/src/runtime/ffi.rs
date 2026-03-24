use std::ffi::{c_char, c_int, c_uchar, c_ulonglong, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FfiPointField {
    pub name_ptr: *const c_char,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

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

pub type EngineHandle = *mut c_void;

pub type EngineCreateFn = unsafe extern "C" fn(config_path: *const c_char) -> EngineHandle;

pub type EngineDestroyFn = unsafe extern "C" fn(handle: EngineHandle);

pub type EngineProcessPacketFn =
    unsafe extern "C" fn(handle: EngineHandle, data_ptr: *const c_uchar, data_len: usize) -> c_int;

pub type EngineHasFrameFn = unsafe extern "C" fn(handle: EngineHandle) -> bool;

pub type EnginePopFrameFn =
    unsafe extern "C" fn(handle: EngineHandle, out_frame: *mut FfiPointCloudFrame) -> c_int;

pub type EngineFreeFrameFn = unsafe extern "C" fn(frame: *mut FfiPointCloudFrame);
