use libloading::Library;

use crate::runtime::ffi::{
    EngineCallApiFn, EngineCreateFn, EngineDestroyFn, EngineFreeApiBufferFn, EngineFreeFrameFn,
    EngineGetApiCountFn, EngineGetApiInfoFn, EngineHasFrameFn, EnginePopFrameFn,
    EngineProcessPacketFn,
};

pub struct EngineLibrary {
    _library: Library,
    pub create: EngineCreateFn,
    pub destroy: EngineDestroyFn,
    pub process_packet: EngineProcessPacketFn,
    pub has_frame: EngineHasFrameFn,
    pub pop_frame: EnginePopFrameFn,
    pub free_frame: EngineFreeFrameFn,
    pub get_api_count: EngineGetApiCountFn,
    pub get_api_info: EngineGetApiInfoFn,
    pub call_api: EngineCallApiFn,
    pub free_api_buffer: EngineFreeApiBufferFn,
}

impl EngineLibrary {
    pub unsafe fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let library = Library::new(path)?;

        let create = *library.get::<EngineCreateFn>(b"engine_create")?;
        let destroy = *library.get::<EngineDestroyFn>(b"engine_destroy")?;
        let process_packet = *library.get::<EngineProcessPacketFn>(b"engine_process_packet")?;
        let has_frame = *library.get::<EngineHasFrameFn>(b"engine_has_frame")?;
        let pop_frame = *library.get::<EnginePopFrameFn>(b"engine_pop_frame")?;
        let free_frame = *library.get::<EngineFreeFrameFn>(b"engine_free_frame")?;

        let get_api_count = *library.get::<EngineGetApiCountFn>(b"engine_get_api_count")?;
        let get_api_info = *library.get::<EngineGetApiInfoFn>(b"engine_get_api_info")?;
        let call_api = *library.get::<EngineCallApiFn>(b"engine_call_api")?;
        let free_api_buffer = *library.get::<EngineFreeApiBufferFn>(b"engine_free_api_buffer")?;

        Ok(Self {
            _library: library,
            create,
            destroy,
            process_packet,
            has_frame,
            pop_frame,
            free_frame,
            get_api_count,
            get_api_info,
            call_api,
            free_api_buffer,
        })
    }
}
