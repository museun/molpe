#[derive(Debug)]
pub enum Error {
    InvalidData, // TODO get an address
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

mod header;
mod reader;
mod stream;
mod method;
mod image;
mod table;
mod tables;

pub use self::header::*;
pub use self::stream::*;
pub use self::reader::*;
pub use self::method::*;
pub use self::image::*;
pub use self::table::*;
pub use self::tables::*;
