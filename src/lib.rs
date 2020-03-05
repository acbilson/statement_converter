//extern crate csv;

mod input;
pub use crate::input::Params;
pub use crate::input::Source;

mod reader;

pub use crate::reader::get_reader;
/*
mod console;
pub use crate::console::convert_to_journal;

mod parser;
*/