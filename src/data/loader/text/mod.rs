pub mod txt;
pub mod json;
pub mod xml;

pub use self::txt::load_txt_files;
pub use self::json::load_json_files;
pub use self::xml::load_xml_files;
