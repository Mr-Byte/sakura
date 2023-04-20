use binaryen_sys::{
    BinaryenHeapType, BinaryenHeapTypeGetBottom, BinaryenHeapTypeIsArray, BinaryenHeapTypeIsBasic,
    BinaryenHeapTypeIsBottom, BinaryenHeapTypeIsSignature, BinaryenHeapTypeIsStruct,
    BinaryenHeapTypeIsSubType, BinaryenType, BinaryenTypeArity, BinaryenTypeCreate,
    BinaryenTypeExpand, BinaryenTypeFromHeapType,
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

macro_rules! impl_types {
    ($($type:ident => $native_type:ident),*) => {
        impl Type {
        $(
            pub fn $type() -> Type {
                let ty = unsafe { binaryen_sys::$native_type() };
                Type(ty)
            }
        )*
        }
    };
}

impl_types! {
    none => BinaryenTypeNone,
    i32 => BinaryenTypeInt32,
    i64 => BinaryenTypeInt64,
    f32 => BinaryenTypeFloat32,
    f64 => BinaryenTypeFloat64,
    vec128 => BinaryenTypeVec128,
    func_ref => BinaryenTypeFuncref,
    extern_ref => BinaryenTypeExternref,
    any_ref => BinaryenTypeAnyref,
    eq_ref => BinaryenTypeEqref,
    i31_ref => BinaryenTypeI31ref,
    struct_ref => BinaryenTypeStructref,
    array_ref => BinaryenTypeArrayref,
    string_ref => BinaryenTypeStringref,
    string_view_wtf8 => BinaryenTypeStringviewWTF8,
    string_view_wtf16 => BinaryenTypeStringviewWTF16,
    string_view_iter => BinaryenTypeStringviewIter,
    null_ref => BinaryenTypeNullref,
    null_extern_ref => BinaryenTypeNullExternref,
    null_func_ref => BinaryenTypeNullFuncref,
    unreachable => BinaryenTypeUnreachable,
    auto => BinaryenTypeAuto
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

macro_rules! impl_heap_types {
    ($($type:ident => $heap_type:ident),*) => {
        impl HeapType {
        $(
            pub fn $type() -> HeapType {
                let ty = unsafe { binaryen_sys::$heap_type() };
                HeapType(ty)
            }
        )*
        }
    };
}

impl_heap_types! {
    ext => BinaryenHeapTypeExt,
    func => BinaryenHeapTypeFunc,
    any => BinaryenHeapTypeAny,
    eq => BinaryenHeapTypeEq,
    i32 => BinaryenHeapTypeI31,
    heap_struct => BinaryenHeapTypeStruct,
    array => BinaryenHeapTypeArray,
    string => BinaryenHeapTypeString,
    string_view_wtf8 => BinaryenHeapTypeStringviewWTF8,
    string_view_wtf16 => BinaryenHeapTypeStringviewWTF16,
    string_view_iter => BinaryenHeapTypeStringviewIter,
    none => BinaryenHeapTypeNone,
    no_ext => BinaryenHeapTypeNoext,
    no_func => BinaryenHeapTypeNofunc
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
