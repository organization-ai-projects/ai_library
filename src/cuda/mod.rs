pub mod bindings;
pub mod utils;

pub use self::bindings::cuda_init;
pub use self::utils::is_cuda_available;
