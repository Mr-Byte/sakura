#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct BinaryOp(i32);

impl BinaryOp {
    pub(crate) fn inner(self) -> i32 {
        self.0
    }

    pub fn clz_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenClzInt32() };
        BinaryOp(op)
    }

    pub fn ctz_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCtzInt32() };
        BinaryOp(op)
    }

    pub fn popcnt_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPopcntInt32() };
        BinaryOp(op)
    }

    pub fn neg_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegFloat32() };
        BinaryOp(op)
    }

    pub fn abs_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsFloat32() };
        BinaryOp(op)
    }

    pub fn ceil_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCeilFloat32() };
        BinaryOp(op)
    }

    pub fn floor_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenFloorFloat32() };
        BinaryOp(op)
    }

    pub fn trunc_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncFloat32() };
        BinaryOp(op)
    }

    pub fn nearest_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNearestFloat32() };
        BinaryOp(op)
    }

    pub fn sqrt_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSqrtFloat32() };
        BinaryOp(op)
    }

    pub fn eqz_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqZInt32() };
        BinaryOp(op)
    }

    pub fn clz_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenClzInt64() };
        BinaryOp(op)
    }

    pub fn ctz_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCtzInt64() };
        BinaryOp(op)
    }

    pub fn popcnt_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPopcntInt64() };
        BinaryOp(op)
    }

    pub fn neg_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegFloat64() };
        BinaryOp(op)
    }

    pub fn abs_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsFloat64() };
        BinaryOp(op)
    }

    pub fn ceil_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCeilFloat64() };
        BinaryOp(op)
    }

    pub fn floor_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenFloorFloat64() };
        BinaryOp(op)
    }

    pub fn trunc_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncFloat64() };
        BinaryOp(op)
    }

    pub fn nearest_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNearestFloat64() };
        BinaryOp(op)
    }

    pub fn sqrt_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSqrtFloat64() };
        BinaryOp(op)
    }

    pub fn eqz_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqZInt64() };
        BinaryOp(op)
    }

    pub fn extend_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendSInt32() };
        BinaryOp(op)
    }

    pub fn extend_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendUInt32() };
        BinaryOp(op)
    }

    pub fn wrap_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenWrapInt64() };
        BinaryOp(op)
    }

    pub fn trunc_sfloat32_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat32ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sfloat32_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat32ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_ufloat32_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat32ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_ufloat32_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat32ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_sfloat64_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat64ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sfloat64_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSFloat64ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_ufloat64_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat64ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_ufloat64_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncUFloat64ToInt64() };
        BinaryOp(op)
    }

    pub fn reinterpret_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReinterpretFloat32() };
        BinaryOp(op)
    }

    pub fn reinterpret_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReinterpretFloat64() };
        BinaryOp(op)
    }

    pub fn convert_sint32_to_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt32ToFloat32() };
        BinaryOp(op)
    }

    pub fn convert_sint32_to_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt32ToFloat64() };
        BinaryOp(op)
    }

    pub fn convert_uint32_to_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt32ToFloat32() };
        BinaryOp(op)
    }

    pub fn convert_uint32_to_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt32ToFloat64() };
        BinaryOp(op)
    }

    pub fn convert_sint64_to_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt64ToFloat32() };
        BinaryOp(op)
    }

    pub fn convert_sint64_to_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertSInt64ToFloat64() };
        BinaryOp(op)
    }

    pub fn convert_uint64_to_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt64ToFloat32() };
        BinaryOp(op)
    }

    pub fn convert_uint64_to_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertUInt64ToFloat64() };
        BinaryOp(op)
    }

    pub fn promote_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPromoteFloat32() };
        BinaryOp(op)
    }

    pub fn demote_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDemoteFloat64() };
        BinaryOp(op)
    }

    pub fn reinterpret_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReinterpretInt32() };
        BinaryOp(op)
    }

    pub fn reinterpret_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReinterpretInt64() };
        BinaryOp(op)
    }

    pub fn extend_s8_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendS8Int32() };
        BinaryOp(op)
    }

    pub fn extend_s16_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendS16Int32() };
        BinaryOp(op)
    }

    pub fn extend_s8_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendS8Int64() };
        BinaryOp(op)
    }

    pub fn extend_s16_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendS16Int64() };
        BinaryOp(op)
    }

    pub fn extend_s32_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendS32Int64() };
        BinaryOp(op)
    }

    pub fn add_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddInt32() };
        BinaryOp(op)
    }

    pub fn sub_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubInt32() };
        BinaryOp(op)
    }

    pub fn mul_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulInt32() };
        BinaryOp(op)
    }

    pub fn div_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivSInt32() };
        BinaryOp(op)
    }

    pub fn div_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivUInt32() };
        BinaryOp(op)
    }

    pub fn rem_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRemSInt32() };
        BinaryOp(op)
    }

    pub fn rem_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRemUInt32() };
        BinaryOp(op)
    }

    pub fn and_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAndInt32() };
        BinaryOp(op)
    }

    pub fn or_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenOrInt32() };
        BinaryOp(op)
    }

    pub fn xor_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenXorInt32() };
        BinaryOp(op)
    }

    pub fn shl_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlInt32() };
        BinaryOp(op)
    }

    pub fn shr_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUInt32() };
        BinaryOp(op)
    }

    pub fn shr_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSInt32() };
        BinaryOp(op)
    }

    pub fn rot_l_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRotLInt32() };
        BinaryOp(op)
    }

    pub fn rot_r_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRotRInt32() };
        BinaryOp(op)
    }

    pub fn eq_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqInt32() };
        BinaryOp(op)
    }

    pub fn ne_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeInt32() };
        BinaryOp(op)
    }

    pub fn lt_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSInt32() };
        BinaryOp(op)
    }

    pub fn lt_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtUInt32() };
        BinaryOp(op)
    }

    pub fn le_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSInt32() };
        BinaryOp(op)
    }

    pub fn le_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeUInt32() };
        BinaryOp(op)
    }

    pub fn gt_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSInt32() };
        BinaryOp(op)
    }

    pub fn gt_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtUInt32() };
        BinaryOp(op)
    }

    pub fn ge_sint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSInt32() };
        BinaryOp(op)
    }

    pub fn ge_uint32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeUInt32() };
        BinaryOp(op)
    }

    pub fn add_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddInt64() };
        BinaryOp(op)
    }

    pub fn sub_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubInt64() };
        BinaryOp(op)
    }

    pub fn mul_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulInt64() };
        BinaryOp(op)
    }

    pub fn div_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivSInt64() };
        BinaryOp(op)
    }

    pub fn div_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivUInt64() };
        BinaryOp(op)
    }

    pub fn rem_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRemSInt64() };
        BinaryOp(op)
    }

    pub fn rem_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRemUInt64() };
        BinaryOp(op)
    }

    pub fn and_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAndInt64() };
        BinaryOp(op)
    }

    pub fn or_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenOrInt64() };
        BinaryOp(op)
    }

    pub fn xor_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenXorInt64() };
        BinaryOp(op)
    }

    pub fn shl_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlInt64() };
        BinaryOp(op)
    }

    pub fn shr_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUInt64() };
        BinaryOp(op)
    }

    pub fn shr_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSInt64() };
        BinaryOp(op)
    }

    pub fn rot_l_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRotLInt64() };
        BinaryOp(op)
    }

    pub fn rot_r_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRotRInt64() };
        BinaryOp(op)
    }

    pub fn eq_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqInt64() };
        BinaryOp(op)
    }

    pub fn ne_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeInt64() };
        BinaryOp(op)
    }

    pub fn lt_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSInt64() };
        BinaryOp(op)
    }

    pub fn lt_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtUInt64() };
        BinaryOp(op)
    }

    pub fn le_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSInt64() };
        BinaryOp(op)
    }

    pub fn le_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeUInt64() };
        BinaryOp(op)
    }

    pub fn gt_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSInt64() };
        BinaryOp(op)
    }

    pub fn gt_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtUInt64() };
        BinaryOp(op)
    }

    pub fn ge_sint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSInt64() };
        BinaryOp(op)
    }

    pub fn ge_uint64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeUInt64() };
        BinaryOp(op)
    }

    pub fn add_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddFloat32() };
        BinaryOp(op)
    }

    pub fn sub_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubFloat32() };
        BinaryOp(op)
    }

    pub fn mul_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulFloat32() };
        BinaryOp(op)
    }

    pub fn div_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivFloat32() };
        BinaryOp(op)
    }

    pub fn copy_sign_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCopySignFloat32() };
        BinaryOp(op)
    }

    pub fn min_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinFloat32() };
        BinaryOp(op)
    }

    pub fn max_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxFloat32() };
        BinaryOp(op)
    }

    pub fn eq_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqFloat32() };
        BinaryOp(op)
    }

    pub fn ne_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeFloat32() };
        BinaryOp(op)
    }

    pub fn lt_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtFloat32() };
        BinaryOp(op)
    }

    pub fn le_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeFloat32() };
        BinaryOp(op)
    }

    pub fn gt_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtFloat32() };
        BinaryOp(op)
    }

    pub fn ge_float32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeFloat32() };
        BinaryOp(op)
    }

    pub fn add_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddFloat64() };
        BinaryOp(op)
    }

    pub fn sub_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubFloat64() };
        BinaryOp(op)
    }

    pub fn mul_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulFloat64() };
        BinaryOp(op)
    }

    pub fn div_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivFloat64() };
        BinaryOp(op)
    }

    pub fn copy_sign_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCopySignFloat64() };
        BinaryOp(op)
    }

    pub fn min_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinFloat64() };
        BinaryOp(op)
    }

    pub fn max_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxFloat64() };
        BinaryOp(op)
    }

    pub fn eq_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqFloat64() };
        BinaryOp(op)
    }

    pub fn ne_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeFloat64() };
        BinaryOp(op)
    }

    pub fn lt_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtFloat64() };
        BinaryOp(op)
    }

    pub fn le_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeFloat64() };
        BinaryOp(op)
    }

    pub fn gt_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtFloat64() };
        BinaryOp(op)
    }

    pub fn ge_float64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeFloat64() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_add() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWAdd() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_sub() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWSub() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_and() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWAnd() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_or() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWOr() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_xor() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWXor() };
        BinaryOp(op)
    }

    pub fn atomic_rmw_xchg() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAtomicRMWXchg() };
        BinaryOp(op)
    }

    pub fn trunc_sat_sfloat32_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat32ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sat_sfloat32_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat32ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_sat_ufloat32_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat32ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sat_ufloat32_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat32ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_sat_sfloat64_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat64ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sat_sfloat64_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSFloat64ToInt64() };
        BinaryOp(op)
    }

    pub fn trunc_sat_ufloat64_to_int32() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat64ToInt32() };
        BinaryOp(op)
    }

    pub fn trunc_sat_ufloat64_to_int64() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUFloat64ToInt64() };
        BinaryOp(op)
    }

    pub fn splat_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI8x16() };
        BinaryOp(op)
    }

    pub fn extract_lane_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneSVecI8x16() };
        BinaryOp(op)
    }

    pub fn extract_lane_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneUVecI8x16() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI8x16() };
        BinaryOp(op)
    }

    pub fn splat_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI16x8() };
        BinaryOp(op)
    }

    pub fn extract_lane_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneSVecI16x8() };
        BinaryOp(op)
    }

    pub fn extract_lane_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneUVecI16x8() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI16x8() };
        BinaryOp(op)
    }

    pub fn splat_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI32x4() };
        BinaryOp(op)
    }

    pub fn extract_lane_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecI32x4() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI32x4() };
        BinaryOp(op)
    }

    pub fn splat_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecI64x2() };
        BinaryOp(op)
    }

    pub fn extract_lane_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecI64x2() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecI64x2() };
        BinaryOp(op)
    }

    pub fn splat_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecF32x4() };
        BinaryOp(op)
    }

    pub fn extract_lane_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecF32x4() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecF32x4() };
        BinaryOp(op)
    }

    pub fn splat_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSplatVecF64x2() };
        BinaryOp(op)
    }

    pub fn extract_lane_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtractLaneVecF64x2() };
        BinaryOp(op)
    }

    pub fn replace_lane_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenReplaceLaneVecF64x2() };
        BinaryOp(op)
    }

    pub fn eq_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecI8x16() };
        BinaryOp(op)
    }

    pub fn ne_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecI8x16() };
        BinaryOp(op)
    }

    pub fn lt_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI8x16() };
        BinaryOp(op)
    }

    pub fn lt_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI8x16() };
        BinaryOp(op)
    }

    pub fn gt_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI8x16() };
        BinaryOp(op)
    }

    pub fn gt_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI8x16() };
        BinaryOp(op)
    }

    pub fn le_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI8x16() };
        BinaryOp(op)
    }

    pub fn le_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI8x16() };
        BinaryOp(op)
    }

    pub fn ge_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI8x16() };
        BinaryOp(op)
    }

    pub fn ge_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI8x16() };
        BinaryOp(op)
    }

    pub fn eq_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecI16x8() };
        BinaryOp(op)
    }

    pub fn ne_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecI16x8() };
        BinaryOp(op)
    }

    pub fn lt_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI16x8() };
        BinaryOp(op)
    }

    pub fn lt_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI16x8() };
        BinaryOp(op)
    }

    pub fn gt_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI16x8() };
        BinaryOp(op)
    }

    pub fn gt_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI16x8() };
        BinaryOp(op)
    }

    pub fn le_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI16x8() };
        BinaryOp(op)
    }

    pub fn le_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI16x8() };
        BinaryOp(op)
    }

    pub fn ge_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI16x8() };
        BinaryOp(op)
    }

    pub fn ge_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI16x8() };
        BinaryOp(op)
    }

    pub fn eq_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecI32x4() };
        BinaryOp(op)
    }

    pub fn ne_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecI32x4() };
        BinaryOp(op)
    }

    pub fn lt_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI32x4() };
        BinaryOp(op)
    }

    pub fn lt_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtUVecI32x4() };
        BinaryOp(op)
    }

    pub fn gt_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI32x4() };
        BinaryOp(op)
    }

    pub fn gt_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtUVecI32x4() };
        BinaryOp(op)
    }

    pub fn le_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI32x4() };
        BinaryOp(op)
    }

    pub fn le_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeUVecI32x4() };
        BinaryOp(op)
    }

    pub fn ge_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI32x4() };
        BinaryOp(op)
    }

    pub fn ge_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeUVecI32x4() };
        BinaryOp(op)
    }

    pub fn eq_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecI64x2() };
        BinaryOp(op)
    }

    pub fn ne_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecI64x2() };
        BinaryOp(op)
    }

    pub fn lt_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtSVecI64x2() };
        BinaryOp(op)
    }

    pub fn gt_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtSVecI64x2() };
        BinaryOp(op)
    }

    pub fn le_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeSVecI64x2() };
        BinaryOp(op)
    }

    pub fn ge_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeSVecI64x2() };
        BinaryOp(op)
    }

    pub fn eq_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecF32x4() };
        BinaryOp(op)
    }

    pub fn ne_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecF32x4() };
        BinaryOp(op)
    }

    pub fn lt_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtVecF32x4() };
        BinaryOp(op)
    }

    pub fn gt_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtVecF32x4() };
        BinaryOp(op)
    }

    pub fn le_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeVecF32x4() };
        BinaryOp(op)
    }

    pub fn ge_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeVecF32x4() };
        BinaryOp(op)
    }

    pub fn eq_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenEqVecF64x2() };
        BinaryOp(op)
    }

    pub fn ne_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNeVecF64x2() };
        BinaryOp(op)
    }

    pub fn lt_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLtVecF64x2() };
        BinaryOp(op)
    }

    pub fn gt_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGtVecF64x2() };
        BinaryOp(op)
    }

    pub fn le_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLeVecF64x2() };
        BinaryOp(op)
    }

    pub fn ge_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenGeVecF64x2() };
        BinaryOp(op)
    }

    pub fn not_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNotVec128() };
        BinaryOp(op)
    }

    pub fn and_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAndVec128() };
        BinaryOp(op)
    }

    pub fn or_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenOrVec128() };
        BinaryOp(op)
    }

    pub fn xor_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenXorVec128() };
        BinaryOp(op)
    }

    pub fn and_not_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAndNotVec128() };
        BinaryOp(op)
    }

    pub fn bitselect_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBitselectVec128() };
        BinaryOp(op)
    }

    pub fn relaxed_fma_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmaVecF32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_fmsvec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmsVecF32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_fma_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmaVecF64x2() };
        BinaryOp(op)
    }

    pub fn relaxed_fmsvec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedFmsVecF64x2() };
        BinaryOp(op)
    }

    pub fn laneselect_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI8x16() };
        BinaryOp(op)
    }

    pub fn laneselect_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI16x8() };
        BinaryOp(op)
    }

    pub fn laneselect_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI32x4() };
        BinaryOp(op)
    }

    pub fn laneselect_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLaneselectI64x2() };
        BinaryOp(op)
    }

    pub fn dot_i8x16i7x16_add_s_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDotI8x16I7x16AddSToVecI32x4() };
        BinaryOp(op)
    }

    pub fn any_true_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAnyTrueVec128() };
        BinaryOp(op)
    }

    pub fn popcnt_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPopcntVecI8x16() };
        BinaryOp(op)
    }

    pub fn abs_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI8x16() };
        BinaryOp(op)
    }

    pub fn neg_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecI8x16() };
        BinaryOp(op)
    }

    pub fn all_true_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI8x16() };
        BinaryOp(op)
    }

    pub fn bitmask_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI8x16() };
        BinaryOp(op)
    }

    pub fn shl_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlVecI8x16() };
        BinaryOp(op)
    }

    pub fn shr_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI8x16() };
        BinaryOp(op)
    }

    pub fn shr_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI8x16() };
        BinaryOp(op)
    }

    pub fn add_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecI8x16() };
        BinaryOp(op)
    }

    pub fn add_sat_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddSatSVecI8x16() };
        BinaryOp(op)
    }

    pub fn add_sat_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddSatUVecI8x16() };
        BinaryOp(op)
    }

    pub fn sub_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecI8x16() };
        BinaryOp(op)
    }

    pub fn sub_sat_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubSatSVecI8x16() };
        BinaryOp(op)
    }

    pub fn sub_sat_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubSatUVecI8x16() };
        BinaryOp(op)
    }

    pub fn min_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI8x16() };
        BinaryOp(op)
    }

    pub fn min_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI8x16() };
        BinaryOp(op)
    }

    pub fn max_svec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI8x16() };
        BinaryOp(op)
    }

    pub fn max_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI8x16() };
        BinaryOp(op)
    }

    pub fn avgr_uvec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAvgrUVecI8x16() };
        BinaryOp(op)
    }

    pub fn abs_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI16x8() };
        BinaryOp(op)
    }

    pub fn neg_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecI16x8() };
        BinaryOp(op)
    }

    pub fn all_true_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI16x8() };
        BinaryOp(op)
    }

    pub fn bitmask_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI16x8() };
        BinaryOp(op)
    }

    pub fn shl_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlVecI16x8() };
        BinaryOp(op)
    }

    pub fn shr_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI16x8() };
        BinaryOp(op)
    }

    pub fn shr_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI16x8() };
        BinaryOp(op)
    }

    pub fn add_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecI16x8() };
        BinaryOp(op)
    }

    pub fn add_sat_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddSatSVecI16x8() };
        BinaryOp(op)
    }

    pub fn add_sat_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddSatUVecI16x8() };
        BinaryOp(op)
    }

    pub fn sub_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecI16x8() };
        BinaryOp(op)
    }

    pub fn sub_sat_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubSatSVecI16x8() };
        BinaryOp(op)
    }

    pub fn sub_sat_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubSatUVecI16x8() };
        BinaryOp(op)
    }

    pub fn mul_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulVecI16x8() };
        BinaryOp(op)
    }

    pub fn min_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI16x8() };
        BinaryOp(op)
    }

    pub fn min_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI16x8() };
        BinaryOp(op)
    }

    pub fn max_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI16x8() };
        BinaryOp(op)
    }

    pub fn max_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI16x8() };
        BinaryOp(op)
    }

    pub fn avgr_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAvgrUVecI16x8() };
        BinaryOp(op)
    }

    pub fn q15_mulr_sat_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenQ15MulrSatSVecI16x8() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI16x8() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI16x8() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI16x8() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_uvec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI16x8() };
        BinaryOp(op)
    }

    pub fn abs_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI32x4() };
        BinaryOp(op)
    }

    pub fn neg_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecI32x4() };
        BinaryOp(op)
    }

    pub fn all_true_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI32x4() };
        BinaryOp(op)
    }

    pub fn bitmask_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI32x4() };
        BinaryOp(op)
    }

    pub fn shl_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlVecI32x4() };
        BinaryOp(op)
    }

    pub fn shr_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI32x4() };
        BinaryOp(op)
    }

    pub fn shr_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI32x4() };
        BinaryOp(op)
    }

    pub fn add_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecI32x4() };
        BinaryOp(op)
    }

    pub fn sub_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecI32x4() };
        BinaryOp(op)
    }

    pub fn mul_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulVecI32x4() };
        BinaryOp(op)
    }

    pub fn min_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinSVecI32x4() };
        BinaryOp(op)
    }

    pub fn min_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinUVecI32x4() };
        BinaryOp(op)
    }

    pub fn max_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxSVecI32x4() };
        BinaryOp(op)
    }

    pub fn max_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxUVecI32x4() };
        BinaryOp(op)
    }

    pub fn dot_svec_i16x8_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDotSVecI16x8ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI32x4() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_svec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI32x4() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI32x4() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_uvec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI32x4() };
        BinaryOp(op)
    }

    pub fn abs_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecI64x2() };
        BinaryOp(op)
    }

    pub fn neg_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecI64x2() };
        BinaryOp(op)
    }

    pub fn all_true_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAllTrueVecI64x2() };
        BinaryOp(op)
    }

    pub fn bitmask_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBitmaskVecI64x2() };
        BinaryOp(op)
    }

    pub fn shl_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShlVecI64x2() };
        BinaryOp(op)
    }

    pub fn shr_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrSVecI64x2() };
        BinaryOp(op)
    }

    pub fn shr_uvec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenShrUVecI64x2() };
        BinaryOp(op)
    }

    pub fn add_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecI64x2() };
        BinaryOp(op)
    }

    pub fn sub_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecI64x2() };
        BinaryOp(op)
    }

    pub fn mul_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulVecI64x2() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowSVecI64x2() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_svec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighSVecI64x2() };
        BinaryOp(op)
    }

    pub fn ext_mul_low_uvec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulLowUVecI64x2() };
        BinaryOp(op)
    }

    pub fn ext_mul_high_uvec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtMulHighUVecI64x2() };
        BinaryOp(op)
    }

    pub fn abs_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecF32x4() };
        BinaryOp(op)
    }

    pub fn neg_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecF32x4() };
        BinaryOp(op)
    }

    pub fn sqrt_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSqrtVecF32x4() };
        BinaryOp(op)
    }

    pub fn add_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecF32x4() };
        BinaryOp(op)
    }

    pub fn sub_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecF32x4() };
        BinaryOp(op)
    }

    pub fn mul_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulVecF32x4() };
        BinaryOp(op)
    }

    pub fn div_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivVecF32x4() };
        BinaryOp(op)
    }

    pub fn min_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinVecF32x4() };
        BinaryOp(op)
    }

    pub fn max_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxVecF32x4() };
        BinaryOp(op)
    }

    pub fn p_min_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPMinVecF32x4() };
        BinaryOp(op)
    }

    pub fn p_max_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPMaxVecF32x4() };
        BinaryOp(op)
    }

    pub fn ceil_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCeilVecF32x4() };
        BinaryOp(op)
    }

    pub fn floor_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenFloorVecF32x4() };
        BinaryOp(op)
    }

    pub fn trunc_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncVecF32x4() };
        BinaryOp(op)
    }

    pub fn nearest_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNearestVecF32x4() };
        BinaryOp(op)
    }

    pub fn abs_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAbsVecF64x2() };
        BinaryOp(op)
    }

    pub fn neg_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNegVecF64x2() };
        BinaryOp(op)
    }

    pub fn sqrt_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSqrtVecF64x2() };
        BinaryOp(op)
    }

    pub fn add_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenAddVecF64x2() };
        BinaryOp(op)
    }

    pub fn sub_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSubVecF64x2() };
        BinaryOp(op)
    }

    pub fn mul_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMulVecF64x2() };
        BinaryOp(op)
    }

    pub fn div_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDivVecF64x2() };
        BinaryOp(op)
    }

    pub fn min_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMinVecF64x2() };
        BinaryOp(op)
    }

    pub fn max_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenMaxVecF64x2() };
        BinaryOp(op)
    }

    pub fn p_min_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPMinVecF64x2() };
        BinaryOp(op)
    }

    pub fn p_max_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPMaxVecF64x2() };
        BinaryOp(op)
    }

    pub fn ceil_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenCeilVecF64x2() };
        BinaryOp(op)
    }

    pub fn floor_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenFloorVecF64x2() };
        BinaryOp(op)
    }

    pub fn trunc_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncVecF64x2() };
        BinaryOp(op)
    }

    pub fn nearest_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNearestVecF64x2() };
        BinaryOp(op)
    }

    pub fn ext_add_pairwise_svec_i8x16_to_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseSVecI8x16ToI16x8() };
        BinaryOp(op)
    }

    pub fn ext_add_pairwise_uvec_i8x16_to_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseUVecI8x16ToI16x8() };
        BinaryOp(op)
    }

    pub fn ext_add_pairwise_svec_i16x8_to_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseSVecI16x8ToI32x4() };
        BinaryOp(op)
    }

    pub fn ext_add_pairwise_uvec_i16x8_to_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtAddPairwiseUVecI16x8ToI32x4() };
        BinaryOp(op)
    }

    pub fn trunc_sat_svec_f32x4_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatSVecF32x4ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn trunc_sat_uvec_f32x4_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatUVecF32x4ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn convert_svec_i32x4_to_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertSVecI32x4ToVecF32x4() };
        BinaryOp(op)
    }

    pub fn convert_uvec_i32x4_to_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertUVecI32x4ToVecF32x4() };
        BinaryOp(op)
    }

    pub fn load8_splat_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad8SplatVec128() };
        BinaryOp(op)
    }

    pub fn load16_splat_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad16SplatVec128() };
        BinaryOp(op)
    }

    pub fn load32_splat_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad32SplatVec128() };
        BinaryOp(op)
    }

    pub fn load64_splat_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad64SplatVec128() };
        BinaryOp(op)
    }

    pub fn load8x8svec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad8x8SVec128() };
        BinaryOp(op)
    }

    pub fn load8x8uvec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad8x8UVec128() };
        BinaryOp(op)
    }

    pub fn load16x4svec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad16x4SVec128() };
        BinaryOp(op)
    }

    pub fn load16x4uvec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad16x4UVec128() };
        BinaryOp(op)
    }

    pub fn load32x2svec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad32x2SVec128() };
        BinaryOp(op)
    }

    pub fn load32x2uvec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad32x2UVec128() };
        BinaryOp(op)
    }

    pub fn load32_zero_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad32ZeroVec128() };
        BinaryOp(op)
    }

    pub fn load64_zero_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad64ZeroVec128() };
        BinaryOp(op)
    }

    pub fn load8_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad8LaneVec128() };
        BinaryOp(op)
    }

    pub fn load16_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad16LaneVec128() };
        BinaryOp(op)
    }

    pub fn load32_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad32LaneVec128() };
        BinaryOp(op)
    }

    pub fn load64_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenLoad64LaneVec128() };
        BinaryOp(op)
    }

    pub fn store8_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStore8LaneVec128() };
        BinaryOp(op)
    }

    pub fn store16_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStore16LaneVec128() };
        BinaryOp(op)
    }

    pub fn store32_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStore32LaneVec128() };
        BinaryOp(op)
    }

    pub fn store64_lane_vec128() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStore64LaneVec128() };
        BinaryOp(op)
    }

    pub fn narrow_svec_i16x8_to_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNarrowSVecI16x8ToVecI8x16() };
        BinaryOp(op)
    }

    pub fn narrow_uvec_i16x8_to_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNarrowUVecI16x8ToVecI8x16() };
        BinaryOp(op)
    }

    pub fn narrow_svec_i32x4_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNarrowSVecI32x4ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn narrow_uvec_i32x4_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenNarrowUVecI32x4ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn extend_low_svec_i8x16_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI8x16ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn extend_high_svec_i8x16_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI8x16ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn extend_low_uvec_i8x16_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI8x16ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn extend_high_uvec_i8x16_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI8x16ToVecI16x8() };
        BinaryOp(op)
    }

    pub fn extend_low_svec_i16x8_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI16x8ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn extend_high_svec_i16x8_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI16x8ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn extend_low_uvec_i16x8_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI16x8ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn extend_high_uvec_i16x8_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI16x8ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn extend_low_svec_i32x4_to_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowSVecI32x4ToVecI64x2() };
        BinaryOp(op)
    }

    pub fn extend_high_svec_i32x4_to_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighSVecI32x4ToVecI64x2() };
        BinaryOp(op)
    }

    pub fn extend_low_uvec_i32x4_to_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendLowUVecI32x4ToVecI64x2() };
        BinaryOp(op)
    }

    pub fn extend_high_uvec_i32x4_to_vec_i64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenExtendHighUVecI32x4ToVecI64x2() };
        BinaryOp(op)
    }

    pub fn convert_low_svec_i32x4_to_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertLowSVecI32x4ToVecF64x2() };
        BinaryOp(op)
    }

    pub fn convert_low_uvec_i32x4_to_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenConvertLowUVecI32x4ToVecF64x2() };
        BinaryOp(op)
    }

    pub fn trunc_sat_zero_svec_f64x2_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatZeroSVecF64x2ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn trunc_sat_zero_uvec_f64x2_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenTruncSatZeroUVecF64x2ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn demote_zero_vec_f64x2_to_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDemoteZeroVecF64x2ToVecF32x4() };
        BinaryOp(op)
    }

    pub fn promote_low_vec_f32x4_to_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenPromoteLowVecF32x4ToVecF64x2() };
        BinaryOp(op)
    }

    pub fn relaxed_trunc_svec_f32x4_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncSVecF32x4ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_trunc_uvec_f32x4_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncUVecF32x4ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_trunc_zero_svec_f64x2_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncZeroSVecF64x2ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_trunc_zero_uvec_f64x2_to_vec_i32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedTruncZeroUVecF64x2ToVecI32x4() };
        BinaryOp(op)
    }

    pub fn swizzle_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenSwizzleVecI8x16() };
        BinaryOp(op)
    }

    pub fn relaxed_swizzle_vec_i8x16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedSwizzleVecI8x16() };
        BinaryOp(op)
    }

    pub fn relaxed_min_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMinVecF32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_max_vec_f32x4() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMaxVecF32x4() };
        BinaryOp(op)
    }

    pub fn relaxed_min_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMinVecF64x2() };
        BinaryOp(op)
    }

    pub fn relaxed_max_vec_f64x2() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedMaxVecF64x2() };
        BinaryOp(op)
    }

    pub fn relaxed_q15_mulr_svec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRelaxedQ15MulrSVecI16x8() };
        BinaryOp(op)
    }

    pub fn dot_i8x16i7x16s_to_vec_i16x8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenDotI8x16I7x16SToVecI16x8() };
        BinaryOp(op)
    }

    pub fn ref_as_non_null() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRefAsNonNull() };
        BinaryOp(op)
    }

    pub fn ref_as_extern_internalize() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRefAsExternInternalize() };
        BinaryOp(op)
    }

    pub fn ref_as_extern_externalize() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenRefAsExternExternalize() };
        BinaryOp(op)
    }

    pub fn br_on_null() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBrOnNull() };
        BinaryOp(op)
    }

    pub fn br_on_non_null() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBrOnNonNull() };
        BinaryOp(op)
    }

    pub fn br_on_cast() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBrOnCast() };
        BinaryOp(op)
    }

    pub fn br_on_cast_fail() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenBrOnCastFail() };
        BinaryOp(op)
    }

    pub fn string_new_utf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewUTF8() };
        BinaryOp(op)
    }

    pub fn string_new_wtf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF8() };
        BinaryOp(op)
    }

    pub fn string_new_replace() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewReplace() };
        BinaryOp(op)
    }

    pub fn string_new_wtf16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF16() };
        BinaryOp(op)
    }

    pub fn string_new_utf8_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewUTF8Array() };
        BinaryOp(op)
    }

    pub fn string_new_wtf8_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF8Array() };
        BinaryOp(op)
    }

    pub fn string_new_replace_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewReplaceArray() };
        BinaryOp(op)
    }

    pub fn string_new_wtf16_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewWTF16Array() };
        BinaryOp(op)
    }

    pub fn string_new_from_code_point() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringNewFromCodePoint() };
        BinaryOp(op)
    }

    pub fn string_measure_utf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureUTF8() };
        BinaryOp(op)
    }

    pub fn string_measure_wtf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF8() };
        BinaryOp(op)
    }

    pub fn string_measure_wtf16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF16() };
        BinaryOp(op)
    }

    pub fn string_measure_is_usv() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureIsUSV() };
        BinaryOp(op)
    }

    pub fn string_measure_wtf16_view() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringMeasureWTF16View() };
        BinaryOp(op)
    }

    pub fn string_encode_utf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeUTF8() };
        BinaryOp(op)
    }

    pub fn string_encode_wtf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF8() };
        BinaryOp(op)
    }

    pub fn string_encode_wtf16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF16() };
        BinaryOp(op)
    }

    pub fn string_encode_utf8_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeUTF8Array() };
        BinaryOp(op)
    }

    pub fn string_encode_wtf8_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF8Array() };
        BinaryOp(op)
    }

    pub fn string_encode_wtf16_array() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEncodeWTF16Array() };
        BinaryOp(op)
    }

    pub fn string_as_wtf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringAsWTF8() };
        BinaryOp(op)
    }

    pub fn string_as_wtf16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringAsWTF16() };
        BinaryOp(op)
    }

    pub fn string_as_iter() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringAsIter() };
        BinaryOp(op)
    }

    pub fn string_iter_move_advance() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringIterMoveAdvance() };
        BinaryOp(op)
    }

    pub fn string_iter_move_rewind() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringIterMoveRewind() };
        BinaryOp(op)
    }

    pub fn string_slice_wtf8() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringSliceWTF8() };
        BinaryOp(op)
    }

    pub fn string_slice_wtf16() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringSliceWTF16() };
        BinaryOp(op)
    }

    pub fn string_eq_equal() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEqEqual() };
        BinaryOp(op)
    }

    pub fn string_eq_compare() -> BinaryOp {
        let op = unsafe { binaryen_sys::BinaryenStringEqCompare() };
        BinaryOp(op)
    }
}
