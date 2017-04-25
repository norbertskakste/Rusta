#[test]
fn it_works() {
    println!("{}", "I gueess???")
}

mod config;

fn new_parser_builder() -> config::ParsingSettings {
    config::ParsingSettings{
        file_read_memory_settings: config::FileReadMemorySettings::Full,
        file_read_concurrency_settings: config::FileReadConcurrencySettings::Full,
        parsing_properties_mode: config::ParsingPropertiesMode::All,
    }
}