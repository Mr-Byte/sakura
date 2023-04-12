use std;
use std::marker::PhantomData;

use binaryen_sys::BinaryenFunctionRef;

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
