use std;
use std::marker::PhantomData;

use binaryen_sys::BinaryenExpressionRef;

#[derive(Clone)]
#[repr(C)]
pub struct Expression<'module> {
    inner: BinaryenExpressionRef,
    _marker: std::marker::PhantomData<&'module mut ()>,
}

impl<'module> Expression<'module> {
    pub(crate) fn new(inner: BinaryenExpressionRef) -> Self {
        Self { inner, _marker: PhantomData }
    }

    pub(crate) fn inner(&self) -> BinaryenExpressionRef {
        self.inner
    }
}
