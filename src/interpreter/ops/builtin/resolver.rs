use std::mem;

use crate::bindings::tflite as bindings;
use crate::interpreter::op_resolver::OpResolver;
use std::ffi::c_void;

cpp! {{
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite::ops::builtin;
}}

pub struct Resolver {
    handle: Box<bindings::OpResolver>,
}

impl Resolver {
    pub fn add_custom(&mut self, name: &str, registration: * const c_void) {
        let name = ::std::ffi::CString::new(name).unwrap();
        let name_ptr = name.as_ptr();
        let resolver_ptr = self.handle.as_mut() as * mut _;
        unsafe {
            cpp!([resolver_ptr as "BuiltinOpResolver*", name_ptr as "const char*", registration as "TfLiteRegistration*"] -> () as "void" {
                resolver_ptr->AddCustom(name_ptr, registration);
                return;
            })
        }
    }
}

impl Drop for Resolver {
    #[allow(clippy::useless_transmute, clippy::forget_copy, deprecated)]
    fn drop(&mut self) {
        let handle = Box::into_raw(mem::take(&mut self.handle));
        unsafe {
            cpp!([handle as "BuiltinOpResolver*"] {
                delete handle;
            });
        }
    }
}

impl OpResolver for Resolver {
    fn get_resolver_handle(&self) -> &bindings::OpResolver {
        self.handle.as_ref()
    }
}

impl Default for Resolver {
    #[allow(clippy::forget_copy, deprecated)]
    fn default() -> Self {
        let handle = unsafe {
            cpp!([] -> *mut bindings::OpResolver as "OpResolver*" {
                return new BuiltinOpResolver();
            })
        };
        let handle = unsafe { Box::from_raw(handle) };
        Self { handle }
    }
}
