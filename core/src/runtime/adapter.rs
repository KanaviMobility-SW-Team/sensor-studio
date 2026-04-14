use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};

use bytes::Bytes;

use crate::engine::Engine;
use crate::runtime::ffi::{
    EngineHandle, FFI_STATUS_OK, FfiApiBuffer, FfiApiInfo, FfiPointCloudFrame,
};
use crate::runtime::loader::EngineLibrary;
use crate::types::pointcloud::{PointCloudFrame, PointField, PointFieldDataType};

#[derive(Debug, Clone)]
pub struct EngineExtensionApiInfo {
    pub name: String,
    pub description: String,
    pub input_schema_json: String,
    pub output_schema_json: String,
}

pub struct FfiEngineAdapter {
    id: String,
    library: EngineLibrary,
    handle: EngineHandle,
}

impl FfiEngineAdapter {
    pub unsafe fn new(
        id: String,
        library: EngineLibrary,
        config_json: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config_cstring = CString::new(config_json)?;
        let config_ptr = config_cstring.as_ptr();

        let handle = unsafe { (library.create)(config_ptr) };
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
        sender_addr: std::net::SocketAddr,
    ) -> Result<Vec<PointCloudFrame>, Box<dyn std::error::Error>> {
        let sender_str =
            CString::new(sender_addr.to_string()).unwrap_or_else(|_| CString::new("").unwrap());

        let status = unsafe {
            (self.library.process_packet)(
                self.handle,
                packet.as_ptr(),
                packet.len(),
                sender_str.as_ptr(),
            )
        };

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

    fn read_c_string(ptr: *const std::ffi::c_char) -> Result<String, Box<dyn std::error::Error>> {
        if ptr.is_null() {
            return Ok(String::new());
        }

        Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_string())
    }

    fn read_api_buffer(&self, buffer: &FfiApiBuffer) -> Result<String, Box<dyn std::error::Error>> {
        if buffer.data_ptr.is_null() || buffer.data_len == 0 {
            return Ok(String::new());
        }

        let bytes = unsafe { std::slice::from_raw_parts(buffer.data_ptr, buffer.data_len) };

        Ok(std::str::from_utf8(bytes)?.to_string())
    }

    pub fn list_extension_apis(
        &self,
    ) -> Result<Vec<EngineExtensionApiInfo>, Box<dyn std::error::Error>> {
        let count = unsafe { (self.library.get_api_count)(self.handle) };

        let mut apis = Vec::with_capacity(count);

        for index in 0..count {
            let mut ffi_info = std::mem::MaybeUninit::<FfiApiInfo>::uninit();

            let status =
                unsafe { (self.library.get_api_info)(self.handle, index, ffi_info.as_mut_ptr()) };

            if status != FFI_STATUS_OK {
                return Err(format!("failed to get api info at index {index}: {status}").into());
            }

            let ffi_info = unsafe { ffi_info.assume_init() };

            apis.push(EngineExtensionApiInfo {
                name: Self::read_c_string(ffi_info.name_ptr)?,
                description: Self::read_c_string(ffi_info.description_ptr)?,
                input_schema_json: Self::read_c_string(ffi_info.input_schema_json_ptr)?,
                output_schema_json: Self::read_c_string(ffi_info.output_schema_json_ptr)?,
            });
        }

        Ok(apis)
    }

    pub fn call_extension_api(
        &self,
        api_name: &str,
        input_json: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_name = CString::new(api_name)?;
        let input_json = CString::new(input_json)?;

        let mut buffer = std::mem::MaybeUninit::<FfiApiBuffer>::uninit();

        let status = unsafe {
            (self.library.call_api)(
                self.handle,
                api_name.as_ptr(),
                input_json.as_ptr(),
                buffer.as_mut_ptr(),
            )
        };

        if status != FFI_STATUS_OK {
            return Err(format!("failed to call extension api: {status}").into());
        }

        let mut buffer = unsafe { buffer.assume_init() };
        let output = self.read_api_buffer(&buffer)?;

        unsafe {
            (self.library.free_api_buffer)(&mut buffer as *mut FfiApiBuffer);
        }

        Ok(output)
    }
}

// FfiEngineAdapter가 스레드 간에 이동해도 안전함을 컴파일러에게 강제로 알려줍니다.
unsafe impl Send for FfiEngineAdapter {}

impl Engine for FfiEngineAdapter {
    fn id(&self) -> &str {
        &self.id
    }

    fn process(&mut self, chunk: Bytes, sender_addr: std::net::SocketAddr) -> Vec<PointCloudFrame> {
        self.process_packet(&chunk, sender_addr)
            .unwrap_or_else(|error| {
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

#[derive(Clone)]
pub struct SharedFfiEngineAdapter {
    inner: Arc<Mutex<FfiEngineAdapter>>,
    id: String,
}

impl SharedFfiEngineAdapter {
    pub fn new(id: String, inner: Arc<Mutex<FfiEngineAdapter>>) -> Self {
        Self { inner, id }
    }
}

impl Engine for SharedFfiEngineAdapter {
    fn id(&self) -> &str {
        &self.id
    }

    fn process(&mut self, chunk: Bytes, sender_addr: std::net::SocketAddr) -> Vec<PointCloudFrame> {
        tokio::task::block_in_place(|| match self.inner.lock() {
            Ok(mut adapter) => adapter.process(chunk, sender_addr),
            Err(error) => {
                eprintln!("failed to lock ffi engine adapter: {error}");
                Vec::new()
            }
        })
    }
}
