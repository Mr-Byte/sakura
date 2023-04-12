use std;
use std::marker::PhantomData;

use binaryen_sys::BinaryenExpressionRef;

pub struct Expression<'module> {
    pub(crate) inner: BinaryenExpressionRef,
    pub(crate) _marker: std::marker::PhantomData<&'module mut ()>,
}

impl<'module> Expression<'module> {
    pub(crate) fn new(inner: BinaryenExpressionRef) -> Self {
        Self { inner, _marker: PhantomData }
    }
}
