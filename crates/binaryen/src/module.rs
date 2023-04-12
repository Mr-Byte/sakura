use std::ffi::CString;

use binaryen_sys::{
    BinaryenAddFunction, BinaryenAddFunctionExport, BinaryenBinary, BinaryenBlock,
    BinaryenExpressionRef, BinaryenLocalGet, BinaryenModuleCreate, BinaryenModuleDispose,
    BinaryenModulePrint, BinaryenUnary,
};

use crate::{Expression, Function, Operator, Type};

#[repr(C)]
pub struct Module {
    pub(crate) module: binaryen_sys::BinaryenModuleRef,
}

impl Module {
    pub fn new() -> Module {
        let module = unsafe { BinaryenModuleCreate() };

        Module { module }
    }

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

    pub fn print(&self) {
        unsafe { BinaryenModulePrint(self.module) }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.module.is_null() {
            return;
        }

        unsafe { BinaryenModuleDispose(self.module) }
    }
}
