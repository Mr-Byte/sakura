use binaryen_sys::{
    BinaryenHeapType, BinaryenHeapTypeAny, BinaryenHeapTypeArray, BinaryenHeapTypeEq,
    BinaryenHeapTypeExt, BinaryenHeapTypeFunc, BinaryenHeapTypeGetBottom, BinaryenHeapTypeI31,
    BinaryenHeapTypeIsArray, BinaryenHeapTypeIsBasic, BinaryenHeapTypeIsBottom,
    BinaryenHeapTypeIsSignature, BinaryenHeapTypeIsStruct, BinaryenHeapTypeIsSubType,
    BinaryenHeapTypeNoext, BinaryenHeapTypeNofunc, BinaryenHeapTypeNone, BinaryenHeapTypeString,
    BinaryenHeapTypeStringviewIter, BinaryenHeapTypeStringviewWTF16,
    BinaryenHeapTypeStringviewWTF8, BinaryenHeapTypeStruct, BinaryenType, BinaryenTypeAnyref,
    BinaryenTypeArity, BinaryenTypeArrayref, BinaryenTypeAuto, BinaryenTypeCreate,
    BinaryenTypeEqref, BinaryenTypeExpand, BinaryenTypeExternref, BinaryenTypeFloat32,
    BinaryenTypeFloat64, BinaryenTypeFromHeapType, BinaryenTypeFuncref, BinaryenTypeI31ref,
    BinaryenTypeInt32, BinaryenTypeInt64, BinaryenTypeNone, BinaryenTypeNullExternref,
    BinaryenTypeNullFuncref, BinaryenTypeNullref, BinaryenTypeStringref,
    BinaryenTypeStringviewIter, BinaryenTypeStringviewWTF16, BinaryenTypeStringviewWTF8,
    BinaryenTypeStructref, BinaryenTypeUnreachable, BinaryenTypeVec128,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Type(BinaryenType);

impl Type {
    pub fn new(types: &[Type]) -> Type {
        let ty = unsafe {
            let tys = types.as_ptr() as *mut usize;
            BinaryenTypeCreate(tys, types.len() as u32)
        };

        Type(ty)
    }

    pub fn arity(self) -> u32 {
        unsafe { BinaryenTypeArity(self.0) }
    }

    pub fn expand(self) -> Vec<Type> {
        let size = self.arity() as usize;

        unsafe {
            let mut buffer = std::mem::ManuallyDrop::new(vec![0; size]);
            BinaryenTypeExpand(self.0, buffer.as_mut_ptr());

            Vec::from_raw_parts(buffer.as_mut_ptr() as *mut Type, buffer.len(), buffer.capacity())
        }
    }

    #[inline]
    pub(crate) fn into_usize(self) -> usize {
        self.0
    }
}

impl Type {
    pub fn none() -> Type {
        let ty = unsafe { BinaryenTypeNone() };
        Type(ty)
    }

    pub fn i32() -> Type {
        let ty = unsafe { BinaryenTypeInt32() };
        Type(ty)
    }

    pub fn i64() -> Type {
        let ty = unsafe { BinaryenTypeInt64() };
        Type(ty)
    }

    pub fn f32() -> Type {
        let ty = unsafe { BinaryenTypeFloat32() };
        Type(ty)
    }

    pub fn f64() -> Type {
        let ty = unsafe { BinaryenTypeFloat64() };
        Type(ty)
    }

    pub fn vec128() -> Type {
        let ty = unsafe { BinaryenTypeVec128() };
        Type(ty)
    }

    pub fn func_ref() -> Type {
        let ty = unsafe { BinaryenTypeFuncref() };
        Type(ty)
    }

    pub fn extern_ref() -> Type {
        let ty = unsafe { BinaryenTypeExternref() };
        Type(ty)
    }

    pub fn any_ref() -> Type {
        let ty = unsafe { BinaryenTypeAnyref() };
        Type(ty)
    }

    pub fn eq_ref() -> Type {
        let ty = unsafe { BinaryenTypeEqref() };
        Type(ty)
    }

    pub fn i31_ref() -> Type {
        let ty = unsafe { BinaryenTypeI31ref() };
        Type(ty)
    }

    pub fn struct_ref() -> Type {
        let ty = unsafe { BinaryenTypeStructref() };
        Type(ty)
    }

    pub fn array_ref() -> Type {
        let ty = unsafe { BinaryenTypeArrayref() };
        Type(ty)
    }

    pub fn string_ref() -> Type {
        let ty = unsafe { BinaryenTypeStringref() };
        Type(ty)
    }

    pub fn string_view_wtf8() -> Type {
        let ty = unsafe { BinaryenTypeStringviewWTF8() };
        Type(ty)
    }

    pub fn string_view_wtf16() -> Type {
        let ty = unsafe { BinaryenTypeStringviewWTF16() };
        Type(ty)
    }

    pub fn string_view_iter() -> Type {
        let ty = unsafe { BinaryenTypeStringviewIter() };
        Type(ty)
    }

    pub fn null_ref() -> Type {
        let ty = unsafe { BinaryenTypeNullref() };
        Type(ty)
    }

    pub fn null_extern_ref() -> Type {
        let ty = unsafe { BinaryenTypeNullExternref() };
        Type(ty)
    }

    pub fn null_func_ref() -> Type {
        let ty = unsafe { BinaryenTypeNullFuncref() };
        Type(ty)
    }

    pub fn unreachable() -> Type {
        let ty = unsafe { BinaryenTypeUnreachable() };
        Type(ty)
    }

    pub fn auto() -> Type {
        let ty = unsafe { BinaryenTypeAuto() };
        Type(ty)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct HeapType(BinaryenHeapType);

impl HeapType {
    #[inline]
    pub(crate) fn into_usize(self) -> usize {
        self.0
    }

    pub fn into_type(self, nullable: bool) -> Type {
        let ty = unsafe { BinaryenTypeFromHeapType(self.0, nullable) };

        Type(ty)
    }

    pub fn is_array(self) -> bool {
        unsafe { BinaryenHeapTypeIsArray(self.0) }
    }

    pub fn is_basic(self) -> bool {
        unsafe { BinaryenHeapTypeIsBasic(self.0) }
    }

    pub fn is_bottom(self) -> bool {
        unsafe { BinaryenHeapTypeIsBottom(self.0) }
    }

    pub fn is_signature(self) -> bool {
        unsafe { BinaryenHeapTypeIsSignature(self.0) }
    }

    pub fn is_struct(self) -> bool {
        unsafe { BinaryenHeapTypeIsStruct(self.0) }
    }

    pub fn is_subtype(self, other: HeapType) -> bool {
        unsafe { BinaryenHeapTypeIsSubType(self.0, other.0) }
    }

    pub fn get_bottom(self) -> HeapType {
        let ty = unsafe { BinaryenHeapTypeGetBottom(self.0) };
        HeapType(ty)
    }
}

impl HeapType {
    pub fn ext() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeExt() };
        HeapType(ty)
    }

    pub fn func() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeFunc() };
        HeapType(ty)
    }

    pub fn any() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeAny() };
        HeapType(ty)
    }

    pub fn eq() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeEq() };
        HeapType(ty)
    }

    pub fn i32() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeI31() };
        HeapType(ty)
    }

    pub fn heap_struct() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeStruct() };
        HeapType(ty)
    }

    pub fn array() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeArray() };
        HeapType(ty)
    }

    pub fn string() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeString() };
        HeapType(ty)
    }

    pub fn string_view_wtf8() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeStringviewWTF8() };
        HeapType(ty)
    }

    pub fn string_view_wtf16() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeStringviewWTF16() };
        HeapType(ty)
    }

    pub fn string_view_iter() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeStringviewIter() };
        HeapType(ty)
    }

    pub fn none() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeNone() };
        HeapType(ty)
    }

    pub fn no_ext() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeNoext() };
        HeapType(ty)
    }

    pub fn no_func() -> HeapType {
        let ty = unsafe { BinaryenHeapTypeNofunc() };
        HeapType(ty)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn type_expand_doesnt_explode_because_unsafe_code() {
        let composite_type = Type::new(&[Type::i32(), Type::i64()]);
        let expanded = composite_type.expand();

        assert_eq!(2, expanded.len(), "expanded type length is not correct");
        assert_eq!(Type::i32(), expanded[0], "first type was not i32");
        assert_eq!(Type::i64(), expanded[1], "second type was not i32")
    }
}
