use std::ffi::{CStr, CString};

use bytes::Bytes;

use crate::engine::Engine;
use crate::runtime::ffi::{EngineHandle, FFI_STATUS_OK, FfiPointCloudFrame};
use crate::runtime::loader::EngineLibrary;
use crate::types::pointcloud::{PointCloudFrame, PointField, PointFieldDataType};

pub struct FfiEngineAdapter {
    id: String,
    library: EngineLibrary,
    handle: EngineHandle,
}

impl FfiEngineAdapter {
    pub unsafe fn new(
        id: String,
        library: EngineLibrary,
        config_path: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_cstring = match config_path {
            Some(path) => Some(CString::new(path)?),
            None => None,
        };

        let config_ptr = config_cstring
            .as_ref()
            .map_or(std::ptr::null(), |value| value.as_ptr());

        let handle = (library.create)(config_ptr);
        if handle.is_null() {
            return Err("failed to create engine handle".into());
        }

        Ok(Self {
            id,
            library,
            handle,
        })
    }

    fn process_packet(
        &mut self,
        packet: &Bytes,
    ) -> Result<Vec<PointCloudFrame>, Box<dyn std::error::Error>> {
        let status =
            unsafe { (self.library.process_packet)(self.handle, packet.as_ptr(), packet.len()) };

        if status != FFI_STATUS_OK {
            return Err(format!("engine process_packet failed: {status}").into());
        }

        let mut frames = Vec::new();

        loop {
            let has_frame = unsafe { (self.library.has_frame)(self.handle) };
            if !has_frame {
                break;
            }

            let mut ffi_frame = std::mem::MaybeUninit::<FfiPointCloudFrame>::uninit();

            let pop_status =
                unsafe { (self.library.pop_frame)(self.handle, ffi_frame.as_mut_ptr()) };

            if pop_status != FFI_STATUS_OK {
                return Err(format!("engine pop_frame failed: {pop_status}").into());
            }

            let mut ffi_frame = unsafe { ffi_frame.assume_init() };
            match self.convert_frame(&ffi_frame) {
                Ok(frame) => frames.push(frame),
                Err(error) => eprintln!("failed to convert ffi frame: {error}"),
            }

            unsafe {
                (self.library.free_frame)(&mut ffi_frame as *mut FfiPointCloudFrame);
            }
        }

        Ok(frames)
    }

    fn convert_frame(
        &self,
        ffi_frame: &FfiPointCloudFrame,
    ) -> Result<PointCloudFrame, Box<dyn std::error::Error>> {
        let frame_id = if ffi_frame.frame_id_ptr.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(ffi_frame.frame_id_ptr) }
                .to_str()?
                .to_string()
        };

        let fields = if ffi_frame.fields_ptr.is_null() || ffi_frame.fields_len == 0 {
            Vec::new()
        } else {
            let ffi_fields =
                unsafe { std::slice::from_raw_parts(ffi_frame.fields_ptr, ffi_frame.fields_len) };

            ffi_fields
                .iter()
                .map(|field| {
                    let name = if field.name_ptr.is_null() {
                        String::new()
                    } else {
                        unsafe { CStr::from_ptr(field.name_ptr) }
                            .to_str()
                            .unwrap_or_default()
                            .to_string()
                    };

                    PointField {
                        name,
                        offset: field.offset,
                        datatype: Self::convert_datatype(field.datatype),
                        count: field.count,
                    }
                })
                .collect()
        };

        let data = if ffi_frame.data_ptr.is_null() || ffi_frame.data_len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ffi_frame.data_ptr, ffi_frame.data_len) }.to_vec()
        };

        Ok(PointCloudFrame {
            timestamp_ns: ffi_frame.timestamp_ns,
            frame_id,
            width: ffi_frame.width,
            height: ffi_frame.height,
            point_step: ffi_frame.point_step,
            row_step: ffi_frame.row_step,
            fields,
            is_dense: ffi_frame.is_dense,
            data,
        })
    }

    fn convert_datatype(datatype: u8) -> PointFieldDataType {
        match datatype {
            1 => PointFieldDataType::Int8,
            2 => PointFieldDataType::Uint8,
            3 => PointFieldDataType::Int16,
            4 => PointFieldDataType::Uint16,
            5 => PointFieldDataType::Int32,
            6 => PointFieldDataType::Uint32,
            7 => PointFieldDataType::Float32,
            8 => PointFieldDataType::Float64,
            _ => PointFieldDataType::Unknown,
        }
    }
}

// FfiEngineAdapter가 스레드 간에 이동해도 안전함을 컴파일러에게 강제로 알려줍니다.
unsafe impl Send for FfiEngineAdapter {}

impl Engine for FfiEngineAdapter {
    fn id(&self) -> &str {
        &self.id
    }

    fn process(&mut self, chunk: Bytes) -> Vec<PointCloudFrame> {
        self.process_packet(&chunk).unwrap_or_else(|error| {
            eprintln!("Error processing packet in FfiEngineAdapter: {error}");
            Vec::new()
        })
    }
}

impl Drop for FfiEngineAdapter {
    fn drop(&mut self) {
        unsafe {
            (self.library.destroy)(self.handle);
        }
    }
}
