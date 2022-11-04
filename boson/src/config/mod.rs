// TODO: Move into a configurable format like JSON/TOML later

pub const BYTE_ENDIAN: &str = "little";
pub const FRAME_STACK_SIZE: usize = 2048;
pub const DATA_STACK_SIZE: usize = 20480;
pub const GLOBAL_POOL_SIZE: usize = 65536;

// static-data-stack:
// This will allocate data-stack as a static array
// provides a performance gain as vector resize will be avoided.
pub const USE_STATIC_DATA_STACK: bool = true;

// enable-concurrency
// Enabl-concurrency features, if disabled
// multi-threading code will run sequentially and join/async will throw errors.
pub const ENABLE_CONCURRENCY: bool = true;
