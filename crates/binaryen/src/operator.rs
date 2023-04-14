#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Operator(i32);

impl Operator {
    #[inline]
    pub(crate) fn into_i32(self) -> i32 {
        self.0
    }
}

impl Operator {
    pub fn clz_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenClzInt32() };
        Operator(op)
    }

    pub fn ctz_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCtzInt32() };
        Operator(op)
    }

    pub fn popcnt_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPopcntInt32() };
        Operator(op)
    }

    pub fn neg_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegFloat32() };
        Operator(op)
    }

    pub fn abs_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsFloat32() };
        Operator(op)
    }

    pub fn ceil_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCeilFloat32() };
        Operator(op)
    }

    pub fn floor_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenFloorFloat32() };
        Operator(op)
    }

    pub fn trunc_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncFloat32() };
        Operator(op)
    }

    pub fn nearest_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNearestFloat32() };
        Operator(op)
    }

    pub fn sqrt_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSqrtFloat32() };
        Operator(op)
    }

    pub fn eqz_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqZInt32() };
        Operator(op)
    }

    pub fn clz_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenClzInt64() };
        Operator(op)
    }

    pub fn ctz_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCtzInt64() };
        Operator(op)
    }

    pub fn popcnt_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPopcntInt64() };
        Operator(op)
    }

    pub fn neg_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegFloat64() };
        Operator(op)
    }

    pub fn abs_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsFloat64() };
        Operator(op)
    }

    pub fn ceil_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCeilFloat64() };
        Operator(op)
    }

    pub fn floor_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenFloorFloat64() };
        Operator(op)
    }

    pub fn trunc_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncFloat64() };
        Operator(op)
    }

    pub fn nearest_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNearestFloat64() };
        Operator(op)
    }

    pub fn sqrt_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSqrtFloat64() };
        Operator(op)
    }

    pub fn eqz_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqZInt64() };
        Operator(op)
    }

    pub fn extend_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendSInt32() };
        Operator(op)
    }

    pub fn extend_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendUInt32() };
        Operator(op)
    }

    pub fn wrap_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenWrapInt64() };
        Operator(op)
    }

    pub fn trunc_s_f32_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat32ToInt32() };
        Operator(op)
    }

    pub fn trunc_s_f32_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat32ToInt64() };
        Operator(op)
    }

    pub fn trunc_u_f32_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat32ToInt32() };
        Operator(op)
    }

    pub fn trunc_u_f32_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat32ToInt64() };
        Operator(op)
    }

    pub fn trunc_s_f64_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat64ToInt32() };
        Operator(op)
    }

    pub fn trunc_s_f64_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat64ToInt64() };
        Operator(op)
    }

    pub fn trunc_u_f64_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat64ToInt32() };
        Operator(op)
    }

    pub fn trunc_u_f64_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat64ToInt64() };
        Operator(op)
    }

    pub fn reinterpret_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReinterpretFloat32() };
        Operator(op)
    }

    pub fn reinterpret_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReinterpretFloat64() };
        Operator(op)
    }

    pub fn convert_s_i32_to_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt32ToFloat32() };
        Operator(op)
    }

    pub fn convert_s_i32_to_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt32ToFloat64() };
        Operator(op)
    }

    pub fn convert_u_i32_to_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt32ToFloat32() };
        Operator(op)
    }

    pub fn convert_u_i32_to_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt32ToFloat64() };
        Operator(op)
    }

    pub fn convert_s_i64_to_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt64ToFloat32() };
        Operator(op)
    }

    pub fn convert_s_i64_to_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt64ToFloat64() };
        Operator(op)
    }

    pub fn convert_u_i64_to_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt64ToFloat32() };
        Operator(op)
    }

    pub fn convert_u_i64_to_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt64ToFloat64() };
        Operator(op)
    }

    pub fn promote_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPromoteFloat32() };
        Operator(op)
    }

    pub fn demote_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDemoteFloat64() };
        Operator(op)
    }

    pub fn reinterpret_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReinterpretInt32() };
        Operator(op)
    }

    pub fn reinterpret_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReinterpretInt64() };
        Operator(op)
    }

    pub fn extend_s8_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendS8Int32() };
        Operator(op)
    }

    pub fn extend_s16_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendS16Int32() };
        Operator(op)
    }

    pub fn extend_s8_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendS8Int64() };
        Operator(op)
    }

    pub fn extend_s16_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendS16Int64() };
        Operator(op)
    }

    pub fn extend_s32_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendS32Int64() };
        Operator(op)
    }

    pub fn add_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddInt32() };
        Operator(op)
    }

    pub fn sub_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubInt32() };
        Operator(op)
    }

    pub fn mul_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulInt32() };
        Operator(op)
    }

    pub fn div_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivSInt32() };
        Operator(op)
    }

    pub fn div_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivUInt32() };
        Operator(op)
    }

    pub fn rem_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRemSInt32() };
        Operator(op)
    }

    pub fn rem_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRemUInt32() };
        Operator(op)
    }

    pub fn and_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAndInt32() };
        Operator(op)
    }

    pub fn or_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenOrInt32() };
        Operator(op)
    }

    pub fn xor_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenXorInt32() };
        Operator(op)
    }

    pub fn shl_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlInt32() };
        Operator(op)
    }

    pub fn shr_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUInt32() };
        Operator(op)
    }

    pub fn shr_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSInt32() };
        Operator(op)
    }

    pub fn rot_l_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRotLInt32() };
        Operator(op)
    }

    pub fn rot_r_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRotRInt32() };
        Operator(op)
    }

    pub fn eq_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqInt32() };
        Operator(op)
    }

    pub fn ne_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeInt32() };
        Operator(op)
    }

    pub fn lt_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSInt32() };
        Operator(op)
    }

    pub fn lt_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtUInt32() };
        Operator(op)
    }

    pub fn le_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSInt32() };
        Operator(op)
    }

    pub fn le_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeUInt32() };
        Operator(op)
    }

    pub fn gt_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSInt32() };
        Operator(op)
    }

    pub fn gt_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtUInt32() };
        Operator(op)
    }

    pub fn ge_s_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSInt32() };
        Operator(op)
    }

    pub fn ge_u_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeUInt32() };
        Operator(op)
    }

    pub fn add_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddInt64() };
        Operator(op)
    }

    pub fn sub_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubInt64() };
        Operator(op)
    }

    pub fn mul_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulInt64() };
        Operator(op)
    }

    pub fn div_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivSInt64() };
        Operator(op)
    }

    pub fn div_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivUInt64() };
        Operator(op)
    }

    pub fn rem_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRemSInt64() };
        Operator(op)
    }

    pub fn rem_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRemUInt64() };
        Operator(op)
    }

    pub fn and_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAndInt64() };
        Operator(op)
    }

    pub fn or_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenOrInt64() };
        Operator(op)
    }

    pub fn xor_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenXorInt64() };
        Operator(op)
    }

    pub fn shl_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlInt64() };
        Operator(op)
    }

    pub fn shr_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUInt64() };
        Operator(op)
    }

    pub fn shr_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSInt64() };
        Operator(op)
    }

    pub fn rot_l_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRotLInt64() };
        Operator(op)
    }

    pub fn rot_r_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRotRInt64() };
        Operator(op)
    }

    pub fn eq_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqInt64() };
        Operator(op)
    }

    pub fn ne_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeInt64() };
        Operator(op)
    }

    pub fn lt_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSInt64() };
        Operator(op)
    }

    pub fn lt_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtUInt64() };
        Operator(op)
    }

    pub fn le_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSInt64() };
        Operator(op)
    }

    pub fn le_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeUInt64() };
        Operator(op)
    }

    pub fn gt_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSInt64() };
        Operator(op)
    }

    pub fn gt_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtUInt64() };
        Operator(op)
    }

    pub fn ge_s_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSInt64() };
        Operator(op)
    }

    pub fn ge_u_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeUInt64() };
        Operator(op)
    }

    pub fn add_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddFloat32() };
        Operator(op)
    }

    pub fn sub_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubFloat32() };
        Operator(op)
    }

    pub fn mul_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulFloat32() };
        Operator(op)
    }

    pub fn div_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivFloat32() };
        Operator(op)
    }

    pub fn copy_sign_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCopySignFloat32() };
        Operator(op)
    }

    pub fn min_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinFloat32() };
        Operator(op)
    }

    pub fn max_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxFloat32() };
        Operator(op)
    }

    pub fn eq_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqFloat32() };
        Operator(op)
    }

    pub fn ne_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeFloat32() };
        Operator(op)
    }

    pub fn lt_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtFloat32() };
        Operator(op)
    }

    pub fn le_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeFloat32() };
        Operator(op)
    }

    pub fn gt_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtFloat32() };
        Operator(op)
    }

    pub fn ge_f32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeFloat32() };
        Operator(op)
    }

    pub fn add_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddFloat64() };
        Operator(op)
    }

    pub fn sub_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubFloat64() };
        Operator(op)
    }

    pub fn mul_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulFloat64() };
        Operator(op)
    }

    pub fn div_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivFloat64() };
        Operator(op)
    }

    pub fn copy_sign_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCopySignFloat64() };
        Operator(op)
    }

    pub fn min_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinFloat64() };
        Operator(op)
    }

    pub fn max_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxFloat64() };
        Operator(op)
    }

    pub fn eq_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqFloat64() };
        Operator(op)
    }

    pub fn ne_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeFloat64() };
        Operator(op)
    }

    pub fn lt_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtFloat64() };
        Operator(op)
    }

    pub fn le_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeFloat64() };
        Operator(op)
    }

    pub fn gt_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtFloat64() };
        Operator(op)
    }

    pub fn ge_f64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeFloat64() };
        Operator(op)
    }

    pub fn atomic_rmw_add() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWAdd() };
        Operator(op)
    }

    pub fn atomic_rmw_sub() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWSub() };
        Operator(op)
    }

    pub fn atomic_rmw_and() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWAnd() };
        Operator(op)
    }

    pub fn atomic_rmw_or() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWOr() };
        Operator(op)
    }

    pub fn atomic_rmw_xor() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWXor() };
        Operator(op)
    }

    pub fn atomic_rmw_xchg() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWXchg() };
        Operator(op)
    }

    pub fn trunc_sat_s_f32_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat32ToInt32() };
        Operator(op)
    }

    pub fn trunc_sat_s_f32_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat32ToInt64() };
        Operator(op)
    }

    pub fn trunc_sat_u_f32_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat32ToInt32() };
        Operator(op)
    }

    pub fn trunc_sat_u_f32_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat32ToInt64() };
        Operator(op)
    }

    pub fn trunc_sat_s_f64_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat64ToInt32() };
        Operator(op)
    }

    pub fn trunc_sat_s_f64_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat64ToInt64() };
        Operator(op)
    }

    pub fn trunc_sat_u_f64_to_i32() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat64ToInt32() };
        Operator(op)
    }

    pub fn trunc_sat_u_f64_to_i64() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat64ToInt64() };
        Operator(op)
    }

    pub fn splat_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI8x16() };
        Operator(op)
    }

    pub fn extract_lane_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneSVecI8x16() };
        Operator(op)
    }

    pub fn extract_lane_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneUVecI8x16() };
        Operator(op)
    }

    pub fn replace_lane_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI8x16() };
        Operator(op)
    }

    pub fn splat_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI16x8() };
        Operator(op)
    }

    pub fn extract_lane_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneSVecI16x8() };
        Operator(op)
    }

    pub fn extract_lane_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneUVecI16x8() };
        Operator(op)
    }

    pub fn replace_lane_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI16x8() };
        Operator(op)
    }

    pub fn splat_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI32x4() };
        Operator(op)
    }

    pub fn extract_lane_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecI32x4() };
        Operator(op)
    }

    pub fn replace_lane_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI32x4() };
        Operator(op)
    }

    pub fn splat_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI64x2() };
        Operator(op)
    }

    pub fn extract_lane_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecI64x2() };
        Operator(op)
    }

    pub fn replace_lane_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI64x2() };
        Operator(op)
    }

    pub fn splat_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecF32x4() };
        Operator(op)
    }

    pub fn extract_lane_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecF32x4() };
        Operator(op)
    }

    pub fn replace_lane_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecF32x4() };
        Operator(op)
    }

    pub fn splat_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSplatVecF64x2() };
        Operator(op)
    }

    pub fn extract_lane_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecF64x2() };
        Operator(op)
    }

    pub fn replace_lane_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecF64x2() };
        Operator(op)
    }

    pub fn eq_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecI8x16() };
        Operator(op)
    }

    pub fn ne_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecI8x16() };
        Operator(op)
    }

    pub fn lt_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI8x16() };
        Operator(op)
    }

    pub fn lt_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI8x16() };
        Operator(op)
    }

    pub fn gt_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI8x16() };
        Operator(op)
    }

    pub fn gt_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI8x16() };
        Operator(op)
    }

    pub fn le_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI8x16() };
        Operator(op)
    }

    pub fn le_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI8x16() };
        Operator(op)
    }

    pub fn ge_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI8x16() };
        Operator(op)
    }

    pub fn ge_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI8x16() };
        Operator(op)
    }

    pub fn eq_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecI16x8() };
        Operator(op)
    }

    pub fn ne_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecI16x8() };
        Operator(op)
    }

    pub fn lt_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI16x8() };
        Operator(op)
    }

    pub fn lt_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI16x8() };
        Operator(op)
    }

    pub fn gt_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI16x8() };
        Operator(op)
    }

    pub fn gt_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI16x8() };
        Operator(op)
    }

    pub fn le_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI16x8() };
        Operator(op)
    }

    pub fn le_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI16x8() };
        Operator(op)
    }

    pub fn ge_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI16x8() };
        Operator(op)
    }

    pub fn ge_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI16x8() };
        Operator(op)
    }

    pub fn eq_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecI32x4() };
        Operator(op)
    }

    pub fn ne_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecI32x4() };
        Operator(op)
    }

    pub fn lt_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI32x4() };
        Operator(op)
    }

    pub fn lt_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI32x4() };
        Operator(op)
    }

    pub fn gt_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI32x4() };
        Operator(op)
    }

    pub fn gt_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI32x4() };
        Operator(op)
    }

    pub fn le_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI32x4() };
        Operator(op)
    }

    pub fn le_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI32x4() };
        Operator(op)
    }

    pub fn ge_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI32x4() };
        Operator(op)
    }

    pub fn ge_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI32x4() };
        Operator(op)
    }

    pub fn eq_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecI64x2() };
        Operator(op)
    }

    pub fn ne_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecI64x2() };
        Operator(op)
    }

    pub fn lt_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI64x2() };
        Operator(op)
    }

    pub fn gt_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI64x2() };
        Operator(op)
    }

    pub fn le_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI64x2() };
        Operator(op)
    }

    pub fn ge_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI64x2() };
        Operator(op)
    }

    pub fn eq_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecF32x4() };
        Operator(op)
    }

    pub fn ne_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecF32x4() };
        Operator(op)
    }

    pub fn lt_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtVecF32x4() };
        Operator(op)
    }

    pub fn gt_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtVecF32x4() };
        Operator(op)
    }

    pub fn le_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeVecF32x4() };
        Operator(op)
    }

    pub fn ge_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeVecF32x4() };
        Operator(op)
    }

    pub fn eq_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenEqVecF64x2() };
        Operator(op)
    }

    pub fn ne_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNeVecF64x2() };
        Operator(op)
    }

    pub fn lt_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLtVecF64x2() };
        Operator(op)
    }

    pub fn gt_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGtVecF64x2() };
        Operator(op)
    }

    pub fn le_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLeVecF64x2() };
        Operator(op)
    }

    pub fn ge_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenGeVecF64x2() };
        Operator(op)
    }

    pub fn not_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNotVec128() };
        Operator(op)
    }

    pub fn and_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAndVec128() };
        Operator(op)
    }

    pub fn or_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenOrVec128() };
        Operator(op)
    }

    pub fn xor_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenXorVec128() };
        Operator(op)
    }

    pub fn and_not_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAndNotVec128() };
        Operator(op)
    }

    pub fn bitselect_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBitselectVec128() };
        Operator(op)
    }

    pub fn relaxed_fma_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmaVecF32x4() };
        Operator(op)
    }

    pub fn relaxed_fms_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmsVecF32x4() };
        Operator(op)
    }

    pub fn relaxed_fma_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmaVecF64x2() };
        Operator(op)
    }

    pub fn relaxed_fms_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmsVecF64x2() };
        Operator(op)
    }

    pub fn laneselect_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI8x16() };
        Operator(op)
    }

    pub fn laneselect_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI16x8() };
        Operator(op)
    }

    pub fn laneselect_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI32x4() };
        Operator(op)
    }

    pub fn laneselect_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI64x2() };
        Operator(op)
    }

    pub fn dot_i8x16i7x16_add_s_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDotI8x16I7x16AddSToVecI32x4() };
        Operator(op)
    }

    pub fn any_true_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAnyTrueVec128() };
        Operator(op)
    }

    pub fn popcnt_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPopcntVecI8x16() };
        Operator(op)
    }

    pub fn abs_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI8x16() };
        Operator(op)
    }

    pub fn neg_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecI8x16() };
        Operator(op)
    }

    pub fn all_true_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI8x16() };
        Operator(op)
    }

    pub fn bitmask_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI8x16() };
        Operator(op)
    }

    pub fn shl_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlVecI8x16() };
        Operator(op)
    }

    pub fn shr_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI8x16() };
        Operator(op)
    }

    pub fn shr_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI8x16() };
        Operator(op)
    }

    pub fn add_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecI8x16() };
        Operator(op)
    }

    pub fn add_sat_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddSatSVecI8x16() };
        Operator(op)
    }

    pub fn add_sat_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddSatUVecI8x16() };
        Operator(op)
    }

    pub fn sub_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecI8x16() };
        Operator(op)
    }

    pub fn sub_sat_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubSatSVecI8x16() };
        Operator(op)
    }

    pub fn sub_sat_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubSatUVecI8x16() };
        Operator(op)
    }

    pub fn min_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI8x16() };
        Operator(op)
    }

    pub fn min_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI8x16() };
        Operator(op)
    }

    pub fn max_s_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI8x16() };
        Operator(op)
    }

    pub fn max_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI8x16() };
        Operator(op)
    }

    pub fn avgr_u_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAvgrUVecI8x16() };
        Operator(op)
    }

    pub fn abs_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI16x8() };
        Operator(op)
    }

    pub fn neg_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecI16x8() };
        Operator(op)
    }

    pub fn all_true_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI16x8() };
        Operator(op)
    }

    pub fn bitmask_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI16x8() };
        Operator(op)
    }

    pub fn shl_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlVecI16x8() };
        Operator(op)
    }

    pub fn shr_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI16x8() };
        Operator(op)
    }

    pub fn shr_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI16x8() };
        Operator(op)
    }

    pub fn add_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecI16x8() };
        Operator(op)
    }

    pub fn add_sat_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddSatSVecI16x8() };
        Operator(op)
    }

    pub fn add_sat_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddSatUVecI16x8() };
        Operator(op)
    }

    pub fn sub_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecI16x8() };
        Operator(op)
    }

    pub fn sub_sat_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubSatSVecI16x8() };
        Operator(op)
    }

    pub fn sub_sat_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubSatUVecI16x8() };
        Operator(op)
    }

    pub fn mul_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulVecI16x8() };
        Operator(op)
    }

    pub fn min_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI16x8() };
        Operator(op)
    }

    pub fn min_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI16x8() };
        Operator(op)
    }

    pub fn max_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI16x8() };
        Operator(op)
    }

    pub fn max_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI16x8() };
        Operator(op)
    }

    pub fn avgr_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAvgrUVecI16x8() };
        Operator(op)
    }

    pub fn q15_mulr_sat_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenQ15MulrSatSVecI16x8() };
        Operator(op)
    }

    pub fn ext_mul_low_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI16x8() };
        Operator(op)
    }

    pub fn ext_mul_high_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI16x8() };
        Operator(op)
    }

    pub fn ext_mul_low_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI16x8() };
        Operator(op)
    }

    pub fn ext_mul_high_u_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI16x8() };
        Operator(op)
    }

    pub fn abs_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI32x4() };
        Operator(op)
    }

    pub fn neg_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecI32x4() };
        Operator(op)
    }

    pub fn all_true_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI32x4() };
        Operator(op)
    }

    pub fn bitmask_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI32x4() };
        Operator(op)
    }

    pub fn shl_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlVecI32x4() };
        Operator(op)
    }

    pub fn shr_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI32x4() };
        Operator(op)
    }

    pub fn shr_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI32x4() };
        Operator(op)
    }

    pub fn add_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecI32x4() };
        Operator(op)
    }

    pub fn sub_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecI32x4() };
        Operator(op)
    }

    pub fn mul_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulVecI32x4() };
        Operator(op)
    }

    pub fn min_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI32x4() };
        Operator(op)
    }

    pub fn min_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI32x4() };
        Operator(op)
    }

    pub fn max_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI32x4() };
        Operator(op)
    }

    pub fn max_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI32x4() };
        Operator(op)
    }

    pub fn dot_s_vec_i16x8_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDotSVecI16x8ToVecI32x4() };
        Operator(op)
    }

    pub fn ext_mul_low_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI32x4() };
        Operator(op)
    }

    pub fn ext_mul_high_s_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI32x4() };
        Operator(op)
    }

    pub fn ext_mul_low_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI32x4() };
        Operator(op)
    }

    pub fn ext_mul_high_u_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI32x4() };
        Operator(op)
    }

    pub fn abs_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI64x2() };
        Operator(op)
    }

    pub fn neg_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecI64x2() };
        Operator(op)
    }

    pub fn all_true_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI64x2() };
        Operator(op)
    }

    pub fn bitmask_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI64x2() };
        Operator(op)
    }

    pub fn shl_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShlVecI64x2() };
        Operator(op)
    }

    pub fn shr_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI64x2() };
        Operator(op)
    }

    pub fn shr_u_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI64x2() };
        Operator(op)
    }

    pub fn add_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecI64x2() };
        Operator(op)
    }

    pub fn sub_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecI64x2() };
        Operator(op)
    }

    pub fn mul_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulVecI64x2() };
        Operator(op)
    }

    pub fn ext_mul_low_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI64x2() };
        Operator(op)
    }

    pub fn ext_mul_high_s_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI64x2() };
        Operator(op)
    }

    pub fn ext_mul_low_u_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI64x2() };
        Operator(op)
    }

    pub fn ext_mul_high_u_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI64x2() };
        Operator(op)
    }

    pub fn abs_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecF32x4() };
        Operator(op)
    }

    pub fn neg_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecF32x4() };
        Operator(op)
    }

    pub fn sqrt_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSqrtVecF32x4() };
        Operator(op)
    }

    pub fn add_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecF32x4() };
        Operator(op)
    }

    pub fn sub_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecF32x4() };
        Operator(op)
    }

    pub fn mul_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulVecF32x4() };
        Operator(op)
    }

    pub fn div_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivVecF32x4() };
        Operator(op)
    }

    pub fn min_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinVecF32x4() };
        Operator(op)
    }

    pub fn max_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxVecF32x4() };
        Operator(op)
    }

    pub fn p_min_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPMinVecF32x4() };
        Operator(op)
    }

    pub fn p_max_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPMaxVecF32x4() };
        Operator(op)
    }

    pub fn ceil_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCeilVecF32x4() };
        Operator(op)
    }

    pub fn floor_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenFloorVecF32x4() };
        Operator(op)
    }

    pub fn trunc_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncVecF32x4() };
        Operator(op)
    }

    pub fn nearest_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNearestVecF32x4() };
        Operator(op)
    }

    pub fn abs_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAbsVecF64x2() };
        Operator(op)
    }

    pub fn neg_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNegVecF64x2() };
        Operator(op)
    }

    pub fn sqrt_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSqrtVecF64x2() };
        Operator(op)
    }

    pub fn add_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenAddVecF64x2() };
        Operator(op)
    }

    pub fn sub_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSubVecF64x2() };
        Operator(op)
    }

    pub fn mul_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMulVecF64x2() };
        Operator(op)
    }

    pub fn div_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDivVecF64x2() };
        Operator(op)
    }

    pub fn min_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMinVecF64x2() };
        Operator(op)
    }

    pub fn max_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenMaxVecF64x2() };
        Operator(op)
    }

    pub fn p_min_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPMinVecF64x2() };
        Operator(op)
    }

    pub fn p_max_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPMaxVecF64x2() };
        Operator(op)
    }

    pub fn ceil_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenCeilVecF64x2() };
        Operator(op)
    }

    pub fn floor_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenFloorVecF64x2() };
        Operator(op)
    }

    pub fn trunc_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncVecF64x2() };
        Operator(op)
    }

    pub fn nearest_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNearestVecF64x2() };
        Operator(op)
    }

    pub fn ext_add_pairwise_s_vec_i8x16_to_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseSVecI8x16ToI16x8() };
        Operator(op)
    }

    pub fn ext_add_pairwise_u_vec_i8x16_to_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseUVecI8x16ToI16x8() };
        Operator(op)
    }

    pub fn ext_add_pairwise_s_vec_i16x8_to_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseSVecI16x8ToI32x4() };
        Operator(op)
    }

    pub fn ext_add_pairwise_u_vec_i16x8_to_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseUVecI16x8ToI32x4() };
        Operator(op)
    }

    pub fn trunc_sat_s_vec_f32x4_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSVecF32x4ToVecI32x4() };
        Operator(op)
    }

    pub fn trunc_sat_u_vec_f32x4_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUVecF32x4ToVecI32x4() };
        Operator(op)
    }

    pub fn convert_s_vec_i32x4_to_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertSVecI32x4ToVecF32x4() };
        Operator(op)
    }

    pub fn convert_u_vec_i32x4_to_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertUVecI32x4ToVecF32x4() };
        Operator(op)
    }

    pub fn load8_splat_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad8SplatVec128() };
        Operator(op)
    }

    pub fn load16_splat_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad16SplatVec128() };
        Operator(op)
    }

    pub fn load32_splat_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad32SplatVec128() };
        Operator(op)
    }

    pub fn load64_splat_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad64SplatVec128() };
        Operator(op)
    }

    pub fn load8x8s_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad8x8SVec128() };
        Operator(op)
    }

    pub fn load8x8u_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad8x8UVec128() };
        Operator(op)
    }

    pub fn load16x4s_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad16x4SVec128() };
        Operator(op)
    }

    pub fn load16x4u_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad16x4UVec128() };
        Operator(op)
    }

    pub fn load32x2s_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad32x2SVec128() };
        Operator(op)
    }

    pub fn load32x2u_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad32x2UVec128() };
        Operator(op)
    }

    pub fn load32_zero_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad32ZeroVec128() };
        Operator(op)
    }

    pub fn load64_zero_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad64ZeroVec128() };
        Operator(op)
    }

    pub fn load8_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad8LaneVec128() };
        Operator(op)
    }

    pub fn load16_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad16LaneVec128() };
        Operator(op)
    }

    pub fn load32_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad32LaneVec128() };
        Operator(op)
    }

    pub fn load64_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenLoad64LaneVec128() };
        Operator(op)
    }

    pub fn store8_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStore8LaneVec128() };
        Operator(op)
    }

    pub fn store16_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStore16LaneVec128() };
        Operator(op)
    }

    pub fn store32_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStore32LaneVec128() };
        Operator(op)
    }

    pub fn store64_lane_vec128() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStore64LaneVec128() };
        Operator(op)
    }

    pub fn narrow_s_vec_i16x8_to_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNarrowSVecI16x8ToVecI8x16() };
        Operator(op)
    }

    pub fn narrow_u_vec_i16x8_to_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNarrowUVecI16x8ToVecI8x16() };
        Operator(op)
    }

    pub fn narrow_s_vec_i32x4_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNarrowSVecI32x4ToVecI16x8() };
        Operator(op)
    }

    pub fn narrow_u_vec_i32x4_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenNarrowUVecI32x4ToVecI16x8() };
        Operator(op)
    }

    pub fn extend_low_s_vec_i8x16_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI8x16ToVecI16x8() };
        Operator(op)
    }

    pub fn extend_high_s_vec_i8x16_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI8x16ToVecI16x8() };
        Operator(op)
    }

    pub fn extend_low_u_vec_i8x16_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI8x16ToVecI16x8() };
        Operator(op)
    }

    pub fn extend_high_u_vec_i8x16_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI8x16ToVecI16x8() };
        Operator(op)
    }

    pub fn extend_low_s_vec_i16x8_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI16x8ToVecI32x4() };
        Operator(op)
    }

    pub fn extend_high_s_vec_i16x8_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI16x8ToVecI32x4() };
        Operator(op)
    }

    pub fn extend_low_u_vec_i16x8_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI16x8ToVecI32x4() };
        Operator(op)
    }

    pub fn extend_high_u_vec_i16x8_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI16x8ToVecI32x4() };
        Operator(op)
    }

    pub fn extend_low_s_vec_i32x4_to_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI32x4ToVecI64x2() };
        Operator(op)
    }

    pub fn extend_high_s_vec_i32x4_to_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI32x4ToVecI64x2() };
        Operator(op)
    }

    pub fn extend_low_u_vec_i32x4_to_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI32x4ToVecI64x2() };
        Operator(op)
    }

    pub fn extend_high_u_vec_i32x4_to_vec_i64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI32x4ToVecI64x2() };
        Operator(op)
    }

    pub fn convert_low_s_vec_i32x4_to_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertLowSVecI32x4ToVecF64x2() };
        Operator(op)
    }

    pub fn convert_low_u_vec_i32x4_to_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenConvertLowUVecI32x4ToVecF64x2() };
        Operator(op)
    }

    pub fn trunc_sat_zero_s_vec_f64x2_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatZeroSVecF64x2ToVecI32x4() };
        Operator(op)
    }

    pub fn trunc_sat_zero_u_vec_f64x2_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenTruncSatZeroUVecF64x2ToVecI32x4() };
        Operator(op)
    }

    pub fn demote_zero_vec_f64x2_to_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDemoteZeroVecF64x2ToVecF32x4() };
        Operator(op)
    }

    pub fn promote_low_vec_f32x4_to_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenPromoteLowVecF32x4ToVecF64x2() };
        Operator(op)
    }

    pub fn relaxed_trunc_s_vec_f32x4_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncSVecF32x4ToVecI32x4() };
        Operator(op)
    }

    pub fn relaxed_trunc_u_vec_f32x4_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncUVecF32x4ToVecI32x4() };
        Operator(op)
    }

    pub fn relaxed_trunc_zero_s_vec_f64x2_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncZeroSVecF64x2ToVecI32x4() };
        Operator(op)
    }

    pub fn relaxed_trunc_zero_u_vec_f64x2_to_vec_i32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncZeroUVecF64x2ToVecI32x4() };
        Operator(op)
    }

    pub fn swizzle_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenSwizzleVecI8x16() };
        Operator(op)
    }

    pub fn relaxed_swizzle_vec_i8x16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedSwizzleVecI8x16() };
        Operator(op)
    }

    pub fn relaxed_min_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMinVecF32x4() };
        Operator(op)
    }

    pub fn relaxed_max_vec_f32x4() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMaxVecF32x4() };
        Operator(op)
    }

    pub fn relaxed_min_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMinVecF64x2() };
        Operator(op)
    }

    pub fn relaxed_max_vec_f64x2() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMaxVecF64x2() };
        Operator(op)
    }

    pub fn relaxed_q15_mulr_s_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRelaxedQ15MulrSVecI16x8() };
        Operator(op)
    }

    pub fn dot_i8x16i7x16s_to_vec_i16x8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenDotI8x16I7x16SToVecI16x8() };
        Operator(op)
    }

    pub fn ref_as_non_null() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRefAsNonNull() };
        Operator(op)
    }

    pub fn ref_as_extern_internalize() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRefAsExternInternalize() };
        Operator(op)
    }

    pub fn ref_as_extern_externalize() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenRefAsExternExternalize() };
        Operator(op)
    }

    pub fn br_on_null() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBrOnNull() };
        Operator(op)
    }

    pub fn br_on_non_null() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBrOnNonNull() };
        Operator(op)
    }

    pub fn br_on_cast() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBrOnCast() };
        Operator(op)
    }

    pub fn br_on_cast_fail() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenBrOnCastFail() };
        Operator(op)
    }

    pub fn string_new_utf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewUTF8() };
        Operator(op)
    }

    pub fn string_new_wtf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF8() };
        Operator(op)
    }

    pub fn string_new_replace() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewReplace() };
        Operator(op)
    }

    pub fn string_new_wtf16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF16() };
        Operator(op)
    }

    pub fn string_new_utf8_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewUTF8Array() };
        Operator(op)
    }

    pub fn string_new_wtf8_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF8Array() };
        Operator(op)
    }

    pub fn string_new_replace_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewReplaceArray() };
        Operator(op)
    }

    pub fn string_new_wtf16_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF16Array() };
        Operator(op)
    }

    pub fn string_new_from_code_point() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringNewFromCodePoint() };
        Operator(op)
    }

    pub fn string_measure_utf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureUTF8() };
        Operator(op)
    }

    pub fn string_measure_wtf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF8() };
        Operator(op)
    }

    pub fn string_measure_wtf16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF16() };
        Operator(op)
    }

    pub fn string_measure_is_usv() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureIsUSV() };
        Operator(op)
    }

    pub fn string_measure_wtf16_view() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF16View() };
        Operator(op)
    }

    pub fn string_encode_utf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeUTF8() };
        Operator(op)
    }

    pub fn string_encode_wtf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF8() };
        Operator(op)
    }

    pub fn string_encode_wtf16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF16() };
        Operator(op)
    }

    pub fn string_encode_utf8_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeUTF8Array() };
        Operator(op)
    }

    pub fn string_encode_wtf8_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF8Array() };
        Operator(op)
    }

    pub fn string_encode_wtf16_array() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF16Array() };
        Operator(op)
    }

    pub fn string_as_wtf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringAsWTF8() };
        Operator(op)
    }

    pub fn string_as_wtf16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringAsWTF16() };
        Operator(op)
    }

    pub fn string_as_iter() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringAsIter() };
        Operator(op)
    }

    pub fn string_iter_move_advance() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringIterMoveAdvance() };
        Operator(op)
    }

    pub fn string_iter_move_rewind() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringIterMoveRewind() };
        Operator(op)
    }

    pub fn string_slice_wtf8() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringSliceWTF8() };
        Operator(op)
    }

    pub fn string_slice_wtf16() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringSliceWTF16() };
        Operator(op)
    }

    pub fn string_eq_equal() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEqEqual() };
        Operator(op)
    }

    pub fn string_eq_compare() -> Operator {
        let op = unsafe { binaryen_sys::BinaryenStringEqCompare() };
        Operator(op)
    }
}
