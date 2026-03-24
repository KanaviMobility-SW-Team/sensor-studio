use libloading::{Library, Symbol};

use crate::runtime::ffi::{
    EngineCreateFn, EngineDestroyFn, EngineFreeFrameFn, EngineHasFrameFn, EnginePopFrameFn,
    EngineProcessPacketFn,
};

pub struct ExternalEngineLibrary {
    _library: Library,
    pub create: EngineCreateFn,
    pub destroy: EngineDestroyFn,
    pub process_packet: EngineProcessPacketFn,
    pub has_frame: EngineHasFrameFn,
    pub pop_frame: EnginePopFrameFn,
    pub free_frame: EngineFreeFrameFn,
}

impl ExternalEngineLibrary {
    pub unsafe fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let library = Library::new(path)?;

        let create = *library.get::<EngineCreateFn>(b"engine_create")?;
        let destroy = *library.get::<EngineDestroyFn>(b"engine_destroy")?;
        let process_packet = *library.get::<EngineProcessPacketFn>(b"engine_process_packet")?;
        let has_frame = *library.get::<EngineHasFrameFn>(b"engine_has_frame")?;
        let pop_frame = *library.get::<EnginePopFrameFn>(b"engine_pop_frame")?;
        let free_frame = *library.get::<EngineFreeFrameFn>(b"engine_free_frame")?;

        Ok(Self {
            _library: library,
            create,
            destroy,
            process_packet,
            has_frame,
            pop_frame,
            free_frame,
        })
    }
}
