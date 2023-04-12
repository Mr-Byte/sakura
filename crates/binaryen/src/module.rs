use binaryen_sys::{BinaryenModuleCreate, BinaryenModuleDispose, BinaryenModulePrint};

#[repr(C)]
pub struct Module {
    pub(crate) module: binaryen_sys::BinaryenModuleRef,
}

impl Module {
    pub fn new() -> Module {
        let module = unsafe { BinaryenModuleCreate() };

        Module { module }
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
