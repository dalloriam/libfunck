//! Funck is a small library providing the tools required to write & deploy functions to the Funcky
//! FaaS platform .

mod error;
mod export;
mod funcktion;
mod request;
mod response;

pub use error::{CallError, CallResult};
pub use funcktion::Funcktion;
pub use request::Request;
pub use response::Response;
