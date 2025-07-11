pub mod monolith;
pub mod moe;
pub mod hybrid;

pub use self::monolith::monolith_ai_with_snn;
pub use self::moe::moe_with_all_ai;
pub use self::hybrid::hybrid_ai_system;
