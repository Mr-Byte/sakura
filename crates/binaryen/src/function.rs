use std::marker::PhantomData;
use std::{self, ffi::CString};

use binaryen_sys::{
    BinaryenAddFunction, BinaryenAddFunctionExport, BinaryenFunction, BinaryenFunctionRef,
    BinaryenSetStart,
};

use crate::{Expression, Module, Type};

#[repr(C)]
pub struct Function<'module> {
    inner: std::ptr::NonNull<BinaryenFunction>,
    _marker: std::marker::PhantomData<&'module mut BinaryenFunction>,
}

impl<'module> Function<'module> {
    fn new(inner: BinaryenFunctionRef) -> Self {
        Self {
            inner: std::ptr::NonNull::new(inner)
                .expect("binaryen produced function expression ptr"),
            _marker: PhantomData,
        }
    }
}

impl Module {
    pub fn set_start(&self, mut start: Function) {
        unsafe { BinaryenSetStart(self.module, start.inner.as_mut()) };
    }

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
                params.into_usize(),
                results.into_usize(),
                // TODO: Handle variadic types?
                std::ptr::null_mut(),
                0,
                body.as_ptr(),
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
