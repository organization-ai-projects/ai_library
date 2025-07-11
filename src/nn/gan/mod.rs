pub mod generator;
pub mod discriminator;
pub mod training;

pub use self::generator::GanGenerator;
pub use self::discriminator::GanDiscriminator;
pub use self::training::train_gan_step;
