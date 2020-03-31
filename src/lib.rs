//! Funck is a small library providing the tools required to write & deploy functions to the Funcky
//! FaaS platform .

mod error;
mod export;
mod funcktion;

pub use error::{CallError, CallResult};
pub use funcktion::Funcktion;
