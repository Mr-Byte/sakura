use std::marker::PhantomData;
use std::{self, ffi::CString};

use binaryen_sys::{
    BinaryenBinary, BinaryenBlock, BinaryenBreak, BinaryenCall, BinaryenConst, BinaryenExpression,
    BinaryenExpressionRef, BinaryenIf, BinaryenLocalGet, BinaryenLocalSet, BinaryenLoop,
    BinaryenReturn, BinaryenSelect, BinaryenUnary,
};

use crate::unsafe_util::UnsafeMaybe;
use crate::{Literal, Module, Operator, Type};

#[derive(Clone)]
#[repr(C)]
pub struct Expression<'module> {
    inner: std::ptr::NonNull<BinaryenExpression>,
    _marker: std::marker::PhantomData<&'module mut BinaryenExpression>,
}

impl<'module> Expression<'module> {
    fn new(inner: BinaryenExpressionRef) -> Self {
        Self {
            inner: std::ptr::NonNull::new(inner).expect("binaryen produced null expression ptr"),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn into_ptr(self) -> BinaryenExpressionRef {
        self.inner.as_ptr()
    }
}

unsafe impl<'module> UnsafeMaybe for Option<Expression<'module>> {
    type Out = BinaryenExpression;

    #[inline]
    fn into_nullable_ptr(self) -> *mut Self::Out {
        self.map_or(std::ptr::null_mut(), Expression::into_ptr)
    }
}

impl Module {
    pub fn expr_binary(&self, op: Operator, lhs: Expression, rhs: Expression) -> Expression {
        let expr =
            unsafe { BinaryenBinary(self.module, op.into_i32(), lhs.into_ptr(), rhs.into_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_block(
        &self,
        name: Option<&str>,
        children: &mut [Expression],
        type_: Type,
    ) -> Expression {
        let name = name.map(CString::new).transpose().expect("failed to convert C string");
        let expr = unsafe {
            let name_ptr = name.as_ref().map_or(std::ptr::null(), |str| str.as_ptr());

            BinaryenBlock(
                self.module,
                name_ptr,
                children.as_mut_ptr() as *mut BinaryenExpressionRef,
                children.len() as u32,
                type_.into_usize(),
            )
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
            BinaryenBreak(
                self.module,
                name.as_ptr(),
                condition.into_nullable_ptr(),
                value.into_nullable_ptr(),
            )
        };
        Expression::new(expr)
    }

    pub fn expr_call(
        &self,
        target: &str,
        args: &mut [Expression<'_>],
        return_type: Type,
    ) -> Expression {
        let target = CString::new(target).expect("failed to convert C string");
        let expr = unsafe {
            BinaryenCall(
                self.module,
                target.as_ptr(),
                args.as_mut_ptr() as *mut BinaryenExpressionRef,
                args.len() as u32,
                return_type.into_usize(),
            )
        };

        Expression::new(expr)
    }

    //TODO: Determine if tail calls are needed.

    pub fn expr_const(&self, value: Literal) -> Expression {
        let expr = unsafe { BinaryenConst(self.module, value.into_inner()) };
        Expression::new(expr)
    }

    pub fn expr_if(
        &self,
        condition: Expression,
        if_true: Expression,
        if_false: Option<Expression>,
    ) -> Expression {
        let expr = unsafe {
            BinaryenIf(
                self.module,
                condition.into_ptr(),
                if_true.into_ptr(),
                if_false.into_nullable_ptr(),
            )
        };

        Expression::new(expr)
    }

    pub fn expr_local_get(&self, index: u32, ty: Type) -> Expression {
        let expr = unsafe { BinaryenLocalGet(self.module, index, ty.into_usize()) };

        Expression::new(expr)
    }

    pub fn expr_local_set(&self, index: u32, value: Expression) -> Expression {
        let expr = unsafe { BinaryenLocalSet(self.module, index, value.into_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_loop(&self, label: &str, body: Expression) -> Expression {
        let label = CString::new(label).expect("failed to convert C string");
        let expr = unsafe { BinaryenLoop(self.module, label.as_ptr(), body.into_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_return(&self, value: Expression) -> Expression {
        let expr = unsafe { BinaryenReturn(self.module, value.into_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_select(
        &self,
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        type_: Type,
    ) -> Expression {
        let expr = unsafe {
            BinaryenSelect(
                self.module,
                condition.into_ptr(),
                if_true.into_ptr(),
                if_false.into_ptr(),
                type_.into_usize(),
            )
        };

        Expression::new(expr)
    }

    pub fn expr_unary(&self, op: Operator, expr: Expression) -> Expression {
        let expr = unsafe { BinaryenUnary(self.module, op.into_i32(), expr.into_ptr()) };

        Expression::new(expr)
    }
}
