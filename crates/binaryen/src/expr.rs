use std::marker::PhantomData;
use std::{self, ffi::CString};

use binaryen_sys::{
    BinaryenBinary, BinaryenBlock, BinaryenExpressionRef, BinaryenLocalGet, BinaryenUnary,
};

use crate::{Module, Operator, Type};

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

impl Module {
    pub fn local_get(&self, index: u32, ty: Type) -> Expression {
        let expr = unsafe { BinaryenLocalGet(self.module, index, ty.inner()) };

        Expression::new(expr)
    }

    pub fn binary_expr(&self, op: Operator, lhs: Expression, rhs: Expression) -> Expression {
        let expr = unsafe { BinaryenBinary(self.module, op.inner(), lhs.inner(), rhs.inner()) };

        Expression::new(expr)
    }

    pub fn unary_expr(&self, op: Operator, expr: Expression) -> Expression {
        let expr = unsafe { BinaryenUnary(self.module, op.inner(), expr.inner()) };

        Expression::new(expr)
    }

    pub fn block(&self, name: Option<&str>, children: &[Expression], ty: Type) -> Expression {
        let name = name.map(CString::new).transpose().expect("failed to convert C string");
        let expr = unsafe {
            let name_ptr = name.as_ref().map_or(std::ptr::null(), |str| str.as_ptr());
            let children_len = children.len();
            let children: *mut BinaryenExpressionRef = std::mem::transmute(children.as_ptr());

            BinaryenBlock(self.module, name_ptr, children, children_len as u32, ty.inner())
        };

        Expression::new(expr)
    }
}
