use std::marker::PhantomData;
use std::{self, ffi::CString};

use binaryen_sys::{BinaryenAddFunction, BinaryenAddFunctionExport, BinaryenFunctionRef};

use crate::{Expression, Module, Type};

#[repr(C)]
pub struct Function<'module> {
    inner: BinaryenFunctionRef,
    _marker: std::marker::PhantomData<&'module mut ()>,
}

impl<'module> Function<'module> {
    pub(crate) fn new(inner: BinaryenFunctionRef) -> Self {
        Self { inner, _marker: PhantomData }
    }
}

impl Module {
    pub fn add_function(
        &self,
        name: &str,
        params: Type,
        results: Type,
        body: Expression,
    ) -> Function<'_> {
        let name = CString::new(name).expect("failed to convert C string");
        let func = unsafe {
            BinaryenAddFunction(
                self.module,
                name.as_ptr(),
                params.inner(),
                results.inner(),
                // TODO: Handle variadic types?
                std::ptr::null_mut(),
                0,
                body.inner(),
            )
        };

        Function::new(func)
    }

    pub fn add_function_export(&self, internal_name: &str, external_name: &str) {
        let internal_name = CString::new(internal_name).expect("failed to convert C string");
        let external_name = CString::new(external_name).expect("failed to convert C string");

        unsafe {
            BinaryenAddFunctionExport(self.module, internal_name.as_ptr(), external_name.as_ptr());
        }
    }
}
