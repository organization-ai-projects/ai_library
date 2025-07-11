pub mod jpg;
pub mod png;
// pub mod bmp; // à ajouter si besoin

pub use self::jpg::load_jpg_images;
pub use self::png::load_png_images;
