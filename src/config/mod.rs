/// File Reading settings (Memory)
///
/// Full = Reads the whole file in memory
/// Partial(u32) = Reads the specified amount of "protobufs" in memory
/// Auto = Minimal memory usage (1 "protobuf" spec per allocation)
pub enum FileReadMemorySettings {
    Full, // Reads the whole file in memory
    Partial(u32), // Reads the specified amount of "protobufs" in memory
    Auto, // Minimal memory usage (1 "protobuf" spec per allocation
}

/// File Reading settings (Concurrency)
///
/// Full = Spawns n amount of workers, where n = CPU threads
/// Partial(u32) = Spawns the specified amount of workers
/// Seq = Single thread
pub enum FileReadConcurrencySettings {
    Full, // Spawns n amount of workers, where n = CPU threads
    Partial(u32), // Spawns the specified amount of workers
    Seq, // Single thread
}

/// Replay parsing settings
///
/// All = Reads & parses everything
/// Manual = Reads & parses specified events
/// Callbacks = Reads & parses on callback registration
pub enum ParsingPropertiesMode {
    All, // Reads & parses everything
    Manual, // Reads & parses specified events
    Callbacks, // Reads & parses on callback registration
}

pub struct ParsingSettings {
    pub file_read_memory_settings: FileReadMemorySettings,
    pub file_read_concurrency_settings: FileReadConcurrencySettings,
    pub parsing_properties_mode: ParsingPropertiesMode,
}