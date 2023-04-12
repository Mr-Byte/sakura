use std::ffi::CString;

use binaryen_sys::{
    BinaryenAddFunction, BinaryenBinary, BinaryenLocalGet, BinaryenModuleCreate,
    BinaryenModuleDispose, BinaryenModulePrint,
};

use crate::{BinaryOp, Expression, Function, Type};

pub struct Module {
    pub(crate) module: binaryen_sys::BinaryenModuleRef,
}

impl Module {
    pub fn new() -> Module {
        let module = unsafe { BinaryenModuleCreate() };

        Module { module }
    }

    pub fn local_get(&self, index: u32, ty: Type) -> Expression<'_> {
        let expr = unsafe { BinaryenLocalGet(self.module, index, ty.inner()) };

        Expression::new(expr)
    }

    pub fn binary_expr(&self, op: BinaryOp, lhs: Expression, rhs: Expression) -> Expression<'_> {
        let expr = unsafe { BinaryenBinary(self.module, op.inner(), lhs.inner, rhs.inner) };

        Expression::new(expr)
    }

    pub fn add_function(
        &self,
        name: &str,
        params: Type,
        results: Type,
        body: Expression,
    ) -> Function<'_> {
        let func = unsafe {
            let name = CString::new(name).unwrap();

            BinaryenAddFunction(
                self.module,
                name.as_ptr(),
                params.inner(),
                results.inner(),
                std::ptr::null_mut(),
                0,
                body.inner,
            )
        };

        Function::new(func)
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

#[cfg(test)]
pub(crate) mod test {
    use crate::Type;

    use super::*;

    #[test]
    fn test_print_module() {
        let module = Module::new();
        let params = Type::new(&[Type::int32(), Type::int32()]);
        let results = Type::int32();

        let x = module.local_get(0, Type::int32());
        let y = module.local_get(1, Type::int32());
        let add = module.binary_expr(BinaryOp::add_int32(), x, y);

        let _adder = module.add_function("adder", params, results, add);

        module.print();
    }
}
