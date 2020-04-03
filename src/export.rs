/// `export!()` auto-implements the [`Funcktion`] trait and generates an FFI wrapper to construct the exported structure.
///
/// The implementation of the [`Funcktion`] trait wraps the provided callback in an panic handler
/// so as not to unwind through the FFI boundary. The macro also auto-generates an `extern "C"` function
/// that constructs the provided struct and returns a raw mutable pointer to it. This constructor function
/// will then be used by the funcktion server to load your funcktion object.
///
/// # Examples
/// ```
/// use funck::{Request, Response, CallError};
///
/// const FUNCTION_NAME: &str = "my_function";
///
/// #[derive(Default)]
/// pub struct MyFunck;
///
/// impl MyFunck {
///     fn some_function(&self, request: Request) -> Result<Response, CallError> {
///         Ok(Response::new().with_text(String::from("Hello from FFI")))
///     }
/// }
///
/// funck::export!(MyFunck, MyFunck::some_function, FUNCTION_NAME);
/// ```
///
/// [`Funcktion`]: ./trait.Funcktion.html
#[macro_export]
macro_rules! export {
    ($function_type:ty, $call_path:path, $str:tt) => {
        // Auto Funcktion implementation.
        impl $crate::Funcktion for $function_type {
            // Return the user-provided name as a `&'static str`.
            fn name(&self) -> &'static str {
                $str
            }
            fn _call_internal(&self, req: $crate::Request) -> $crate::CallResult<$crate::Response> {
                // Generate a panic handler for the exported function.
                // This is necessary because unwinding a panic across FFI boundaries is UB.
                match std::panic::catch_unwind(|| $call_path(self, req)) {
                    Ok(r) => r,
                    Err(e) => Err($crate::CallError::new("FFI: caught unwinding panic")),
                }
            }
        }

        // Generated call for creating raw Funcktion instances.
        #[no_mangle]
        pub extern "C" fn _funck_create() -> *mut $crate::Funcktion {
            let constructor: fn() -> $function_type = <$function_type>::default;
            let obj = constructor();
            let boxed_obj: Box<$crate::Funcktion> = Box::new(obj);
            Box::into_raw(boxed_obj)
        }
    };
}
