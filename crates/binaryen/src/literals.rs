use binaryen_sys::{
    BinaryenLiteral, BinaryenLiteralFloat32, BinaryenLiteralFloat32Bits, BinaryenLiteralFloat64,
    BinaryenLiteralFloat64Bits, BinaryenLiteralInt32, BinaryenLiteralInt64, BinaryenLiteralVec128,
};

#[derive(Clone)]
#[repr(C)]
pub struct Literal(BinaryenLiteral);

impl Literal {
    #[inline]
    pub(crate) fn into_inner(self) -> BinaryenLiteral {
        self.0
    }

    pub fn i32(value: i32) -> Literal {
        let literal = unsafe { BinaryenLiteralInt32(value) };

        Literal(literal)
    }

    pub fn i64(value: i64) -> Literal {
        let literal = unsafe { BinaryenLiteralInt64(value) };

        Literal(literal)
    }

    pub fn f32(value: f32) -> Literal {
        let literal = unsafe { BinaryenLiteralFloat32(value) };

        Literal(literal)
    }

    pub fn f64(value: f64) -> Literal {
        let literal = unsafe { BinaryenLiteralFloat64(value) };

        Literal(literal)
    }

    pub fn f32_bits(value: i32) -> Literal {
        let literal = unsafe { BinaryenLiteralFloat32Bits(value) };

        Literal(literal)
    }

    pub fn f64_bits(value: i64) -> Literal {
        let literal = unsafe { BinaryenLiteralFloat64Bits(value) };

        Literal(literal)
    }

    pub fn vec128(value: &[u8; 16]) -> Literal {
        let literal = unsafe { BinaryenLiteralVec128(value as *const u8) };

        Literal(literal)
    }
}
