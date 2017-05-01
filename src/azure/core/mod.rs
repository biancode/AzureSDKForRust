extern crate crypto;
extern crate hyper;
extern crate url;
extern crate rustc_serialize as serialize;

#[macro_use]
pub mod errors;
pub mod parsing;
#[macro_use]
pub mod enumerations;
pub mod incompletevector;
pub mod lease;

pub mod range;
pub mod ba512_range;

#[derive(Debug, Copy, Clone)]
pub enum HTTPMethod {
    Get,
    Put,
    Post,
    Delete,
}
