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
    pub(crate) fn as_ptr(&mut self) -> BinaryenExpressionRef {
        self.inner.as_ptr()
    }
}

unsafe impl<'module> UnsafeMaybe for Option<Expression<'module>> {
    type Out = BinaryenExpression;

    fn as_ptr_or_null(self: &mut Self) -> *mut Self::Out {
        self.as_mut().map_or(std::ptr::null_mut(), Expression::as_ptr)
    }
}

impl Module {
    pub fn expr_binary(
        &self,
        op: Operator,
        mut lhs: Expression,
        mut rhs: Expression,
    ) -> Expression {
        let expr =
            unsafe { BinaryenBinary(self.module, op.into_i32(), lhs.as_ptr(), rhs.as_ptr()) };

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
        mut condition: Option<Expression>,
        mut value: Option<Expression>,
    ) -> Expression {
        let name = CString::new(name).expect("failed to convert C string");
        let expr = unsafe {
            BinaryenBreak(
                self.module,
                name.as_ptr(),
                condition.as_ptr_or_null(),
                value.as_ptr_or_null(),
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
        mut condition: Expression,
        mut if_true: Expression,
        mut if_false: Option<Expression>,
    ) -> Expression {
        let expr = unsafe {
            BinaryenIf(self.module, condition.as_ptr(), if_true.as_ptr(), if_false.as_ptr_or_null())
        };

        Expression::new(expr)
    }

    pub fn expr_local_get(&self, index: u32, ty: Type) -> Expression {
        let expr = unsafe { BinaryenLocalGet(self.module, index, ty.into_usize()) };

        Expression::new(expr)
    }

    pub fn expr_local_set(&self, index: u32, mut value: Expression) -> Expression {
        let expr = unsafe { BinaryenLocalSet(self.module, index, value.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_loop(&self, label: &str, mut body: Expression) -> Expression {
        let label = CString::new(label).expect("failed to convert C string");
        let expr = unsafe { BinaryenLoop(self.module, label.as_ptr(), body.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_return(&self, mut value: Expression) -> Expression {
        let expr = unsafe { BinaryenReturn(self.module, value.as_ptr()) };

        Expression::new(expr)
    }

    pub fn expr_select(
        &self,
        mut condition: Expression,
        mut if_true: Expression,
        mut if_false: Expression,
        type_: Type,
    ) -> Expression {
        let expr = unsafe {
            BinaryenSelect(
                self.module,
                condition.as_ptr(),
                if_true.as_ptr(),
                if_false.as_ptr(),
                type_.into_usize(),
            )
        };

        Expression::new(expr)
    }

    pub fn expr_unary(&self, op: Operator, mut expr: Expression) -> Expression {
        let expr = unsafe { BinaryenUnary(self.module, op.into_i32(), expr.as_ptr()) };

        Expression::new(expr)
    }
}
