use binaryen_sys::{BinaryenTypeCreate, BinaryenTypeInt32};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Type(usize);

impl Type {
    pub fn new(types: &[Type]) -> Type {
        let ty = unsafe {
            let tys = types.as_ptr() as *mut usize;
            BinaryenTypeCreate(tys, types.len() as u32)
        };

        Type(ty)
    }

    pub fn int32() -> Type {
        let ty = unsafe { BinaryenTypeInt32() };

        Type(ty)
    }

    pub(crate) const fn inner(self) -> usize {
        self.0
    }
}
