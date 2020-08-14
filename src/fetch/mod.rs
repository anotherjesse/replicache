#[cfg_attr(target_arch = "wasm32", path = "browser_client.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "rust_client.rs")]
pub mod client;

pub mod errors;
