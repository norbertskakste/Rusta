/// File Reading settings (Memory)
///
/// Full = Reads the whole file in memory
/// Partial(u32) = Reads the specified amount of "protobufs" in memory
/// Auto = Minimal memory usage (1 "protobuf" spec per allocation)
enum FileReadMemorySettings {
    Full, // Reads the whole file in memory
    Partial(u32), // Reads the specified amount of "protobufs" in memory
    Auto, // Minimal memory usage (1 "protobuf" spec per allocation
}

/// File Reading settings (Concurrency)
///
/// Full = Spawns n amount of workers, where n = CPU threads
/// Partial(u32) = Spawns the specified amount of workers
/// Seq = Single thread
enum FileReadConcurrencySettings {
    Full, // Spawns n amount of workers, where n = CPU threads
    Partial(u32), // Spawns the specified amount of workers
    Seq, // Single thread
}

/// Replay parsing settings
///
/// All = Reads & parses everything
/// Manual = Reads & parses specified events
/// Callbacks = Reads & parses on callback registration
enum ParsingPropertiesMode {
    All, // Reads & parses everything
    Manual, // Reads & parses specified events
    Callbacks, // Reads & parses on callback registration
}

struct ParsingSettings {
    file_read_memory_settings: FileReadMemorySettings,
    file_read_concurrency_settings: FileReadConcurrencySettings,
    parsing_properties_mode: ParsingPropertiesMode,
}