use std::marker::PhantomData;
use std::{self, ffi::CString};

use binaryen_sys::{
    BinaryenBinary, BinaryenBlock, BinaryenBreak, BinaryenExpression, BinaryenExpressionRef,
    BinaryenIf, BinaryenLocalGet, BinaryenLocalSet, BinaryenLoop, BinaryenReturn, BinaryenUnary,
};

use crate::unsafe_util::UnsafeMaybe;
use crate::{Module, Operator, Type};

#[derive(Clone)]
#[repr(C)]
pub struct Expression<'module> {
    inner: std::ptr::NonNull<BinaryenExpression>,
    _marker: std::marker::PhantomData<&'module mut BinaryenExpression>,
}

impl<'module> Expression<'module> {
    pub(crate) fn new(inner: BinaryenExpressionRef) -> Self {
        Self {
            inner: std::ptr::NonNull::new(inner).expect("binaryen produced null expression ptr"),
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> BinaryenExpressionRef {
        self.inner.as_ptr()
    }
}

unsafe impl<'module> UnsafeMaybe for Option<Expression<'module>> {
    type Out = BinaryenExpression;

    fn as_ptr(self: Self) -> *mut Self::Out {
        unsafe { std::mem::transmute(self) }
    }
}

impl Module {
    pub fn expr_binary(&self, op: Operator, lhs: Expression, rhs: Expression) -> Expression {
        let expr = unsafe { BinaryenBinary(self.module, op.inner(), lhs.as_ptr(), rhs.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_block(&self, name: Option<&str>, children: &[Expression], ty: Type) -> Expression {
        let name = name.map(CString::new).transpose().expect("failed to convert C string");
        let expr = unsafe {
            let name_ptr = name.as_ref().map_or(std::ptr::null(), |str| str.as_ptr());
            let children_len = children.len();
            let children: *mut BinaryenExpressionRef = std::mem::transmute(children.as_ptr());

            BinaryenBlock(self.module, name_ptr, children, children_len as u32, ty.inner())
        };

        Expression::new(expr)
    }

    pub fn expr_break(
        &self,
        name: &str,
        condition: Option<Expression>,
        value: Option<Expression>,
    ) -> Expression {
        let name = CString::new(name).expect("failed to convert C string");
        let expr = unsafe {
            BinaryenBreak(self.module, name.as_ptr(), condition.as_ptr(), value.as_ptr())
        };
        Expression::new(expr)
    }

    pub fn expr_if(
        &self,
        condition: Expression,
        if_true: Expression,
        if_false: Option<Expression>,
    ) -> Expression {
        let expr = unsafe {
            BinaryenIf(self.module, condition.as_ptr(), if_true.as_ptr(), if_false.as_ptr())
        };

        Expression::new(expr)
    }

    pub fn expr_local_get(&self, index: u32, ty: Type) -> Expression {
        let expr = unsafe { BinaryenLocalGet(self.module, index, ty.inner()) };

        Expression::new(expr)
    }

    pub fn expr_local_set(&self, index: u32, value: Expression) -> Expression {
        let expr = unsafe { BinaryenLocalSet(self.module, index, value.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_loop(&self, label: &str, body: Expression) -> Expression {
        let label = CString::new(label).expect("failed to convert C string");
        let expr = unsafe { BinaryenLoop(self.module, label.as_ptr(), body.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_return(&self, value: Expression) -> Expression {
        let expr = unsafe { BinaryenReturn(self.module, value.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_unary(&self, op: Operator, expr: Expression) -> Expression {
        let expr = unsafe { BinaryenUnary(self.module, op.inner(), expr.as_ptr()) };

        Expression::new(expr)
    }
}
