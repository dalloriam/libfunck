use std::any::Any;

use crate::CallError;

/// The Funcktion trait must be implemented by any cloud functions to be loaded in the system.
///
/// This trait is usually implemented via the mandatory [`export`] macro. If you decide to implement
/// this yourself, beware since the `_call_internal` method will be called by the server through FFI.
/// This means that `_call_internal` **MUST** handle _all_ unwinding panics to avoid undefined
/// behavior.
///
/// [`export`]: ./macro.export!.html
pub trait Funcktion: Any + Send + Sync {
    /// Returns the name of the funcktion.
    fn name(&self) -> &'static str;

    /// Generated wrapper that executes a funcktion, catching any panics that occur.
    fn _call_internal(&self) -> Result<(), CallError>;
}
