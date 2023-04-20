#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Operator(i32);

impl Operator {
    #[inline]
    pub(crate) fn into_i32(self) -> i32 {
        self.0
    }
}

macro_rules! impl_operators {
    ($($op:ident => $native_op:ident),*) => {
        impl Operator {
        $(
            pub fn $op() -> Operator {
                let op = unsafe { binaryen_sys::$native_op() };
                Operator(op)
            }
        )*
        }
    };
}

impl_operators! {
    clz_i32 => BinaryenClzInt32,
    ctz_i32 => BinaryenCtzInt32,
    popcnt_i32 => BinaryenPopcntInt32,
    neg_f32 => BinaryenNegFloat32,
    abs_f32 => BinaryenAbsFloat32,
    ceil_f32 => BinaryenCeilFloat32,
    floor_f32 => BinaryenFloorFloat32,
    trunc_f32 => BinaryenTruncFloat32,
    nearest_f32 => BinaryenNearestFloat32,
    sqrt_f32 => BinaryenSqrtFloat32,
    eqz_i32 => BinaryenEqZInt32,
    clz_i64 => BinaryenClzInt64,
    ctz_i64 => BinaryenCtzInt64,
    popcnt_i64 => BinaryenPopcntInt64,
    neg_f64 => BinaryenNegFloat64,
    abs_f64 => BinaryenAbsFloat64,
    ceil_f64 => BinaryenCeilFloat64,
    floor_f64 => BinaryenFloorFloat64,
    trunc_f64 => BinaryenTruncFloat64,
    nearest_f64 => BinaryenNearestFloat64,
    sqrt_f64 => BinaryenSqrtFloat64,
    eqz_i64 => BinaryenEqZInt64,
    extend_s_i32 => BinaryenExtendSInt32,
    extend_u_i32 => BinaryenExtendUInt32,
    wrap_i64 => BinaryenWrapInt64,
    trunc_s_f32_to_i32 => BinaryenTruncSFloat32ToInt32,
    trunc_s_f32_to_i64 => BinaryenTruncSFloat32ToInt64,
    trunc_u_f32_to_i32 => BinaryenTruncUFloat32ToInt32,
    trunc_u_f32_to_i64 => BinaryenTruncUFloat32ToInt64,
    trunc_s_f64_to_i32 => BinaryenTruncSFloat64ToInt32,
    trunc_s_f64_to_i64 => BinaryenTruncSFloat64ToInt64,
    trunc_u_f64_to_i32 => BinaryenTruncUFloat64ToInt32,
    trunc_u_f64_to_i64 => BinaryenTruncUFloat64ToInt64,
    reinterpret_f32 => BinaryenReinterpretFloat32,
    reinterpret_f64 => BinaryenReinterpretFloat64,
    convert_s_i32_to_f32 => BinaryenConvertSInt32ToFloat32,
    convert_s_i32_to_f64 => BinaryenConvertSInt32ToFloat64,
    convert_u_i32_to_f32 => BinaryenConvertUInt32ToFloat32,
    convert_u_i32_to_f64 => BinaryenConvertUInt32ToFloat64,
    convert_s_i64_to_f32 => BinaryenConvertSInt64ToFloat32,
    convert_s_i64_to_f64 => BinaryenConvertSInt64ToFloat64,
    convert_u_i64_to_f32 => BinaryenConvertUInt64ToFloat32,
    convert_u_i64_to_f64 => BinaryenConvertUInt64ToFloat64,
    promote_f32 => BinaryenPromoteFloat32,
    demote_f64 => BinaryenDemoteFloat64,
    reinterpret_i32 => BinaryenReinterpretInt32,
    reinterpret_i64 => BinaryenReinterpretInt64,
    extend_s8_i32 => BinaryenExtendS8Int32,
    extend_s16_i32 => BinaryenExtendS16Int32,
    extend_s8_i64 => BinaryenExtendS8Int64,
    extend_s16_i64 => BinaryenExtendS16Int64,
    extend_s32_i64 => BinaryenExtendS32Int64,
    add_i32 => BinaryenAddInt32,
    sub_i32 => BinaryenSubInt32,
    mul_i32 => BinaryenMulInt32,
    div_s_i32 => BinaryenDivSInt32,
    div_u_i32 => BinaryenDivUInt32,
    rem_s_i32 => BinaryenRemSInt32,
    rem_u_i32 => BinaryenRemUInt32,
    and_i32 => BinaryenAndInt32,
    or_i32 => BinaryenOrInt32,
    xor_i32 => BinaryenXorInt32,
    shl_i32 => BinaryenShlInt32,
    shr_u_i32 => BinaryenShrUInt32,
    shr_s_i32 => BinaryenShrSInt32,
    rot_l_i32 => BinaryenRotLInt32,
    rot_r_i32 => BinaryenRotRInt32,
    eq_i32 => BinaryenEqInt32,
    ne_i32 => BinaryenNeInt32,
    lt_s_i32 => BinaryenLtSInt32,
    lt_u_i32 => BinaryenLtUInt32,
    le_s_i32 => BinaryenLeSInt32,
    le_u_i32 => BinaryenLeUInt32,
    gt_s_i32 => BinaryenGtSInt32,
    gt_u_i32 => BinaryenGtUInt32,
    ge_s_i32 => BinaryenGeSInt32,
    ge_u_i32 => BinaryenGeUInt32,
    add_i64 => BinaryenAddInt64,
    sub_i64 => BinaryenSubInt64,
    mul_i64 => BinaryenMulInt64,
    div_s_i64 => BinaryenDivSInt64,
    div_u_i64 => BinaryenDivUInt64,
    rem_s_i64 => BinaryenRemSInt64,
    rem_u_i64 => BinaryenRemUInt64,
    and_i64 => BinaryenAndInt64,
    or_i64 => BinaryenOrInt64,
    xor_i64 => BinaryenXorInt64,
    shl_i64 => BinaryenShlInt64,
    shr_u_i64 => BinaryenShrUInt64,
    shr_s_i64 => BinaryenShrSInt64,
    rot_l_i64 => BinaryenRotLInt64,
    rot_r_i64 => BinaryenRotRInt64,
    eq_i64 => BinaryenEqInt64,
    ne_i64 => BinaryenNeInt64,
    lt_s_i64 => BinaryenLtSInt64,
    lt_u_i64 => BinaryenLtUInt64,
    le_s_i64 => BinaryenLeSInt64,
    le_u_i64 => BinaryenLeUInt64,
    gt_s_i64 => BinaryenGtSInt64,
    gt_u_i64 => BinaryenGtUInt64,
    ge_s_i64 => BinaryenGeSInt64,
    ge_u_i64 => BinaryenGeUInt64,
    add_f32 => BinaryenAddFloat32,
    sub_f32 => BinaryenSubFloat32,
    mul_f32 => BinaryenMulFloat32,
    div_f32 => BinaryenDivFloat32,
    copy_sign_f32 => BinaryenCopySignFloat32,
    min_f32 => BinaryenMinFloat32,
    max_f32 => BinaryenMaxFloat32,
    eq_f32 => BinaryenEqFloat32,
    ne_f32 => BinaryenNeFloat32,
    lt_f32 => BinaryenLtFloat32,
    le_f32 => BinaryenLeFloat32,
    gt_f32 => BinaryenGtFloat32,
    ge_f32 => BinaryenGeFloat32,
    add_f64 => BinaryenAddFloat64,
    sub_f64 => BinaryenSubFloat64,
    mul_f64 => BinaryenMulFloat64,
    div_f64 => BinaryenDivFloat64,
    copy_sign_f64 => BinaryenCopySignFloat64,
    min_f64 => BinaryenMinFloat64,
    max_f64 => BinaryenMaxFloat64,
    eq_f64 => BinaryenEqFloat64,
    ne_f64 => BinaryenNeFloat64,
    lt_f64 => BinaryenLtFloat64,
    le_f64 => BinaryenLeFloat64,
    gt_f64 => BinaryenGtFloat64,
    ge_f64 => BinaryenGeFloat64,
    atomic_rmw_add => BinaryenAtomicRMWAdd,
    atomic_rmw_sub => BinaryenAtomicRMWSub,
    atomic_rmw_and => BinaryenAtomicRMWAnd,
    atomic_rmw_or => BinaryenAtomicRMWOr,
    atomic_rmw_xor => BinaryenAtomicRMWXor,
    atomic_rmw_xchg => BinaryenAtomicRMWXchg,
    trunc_sat_s_f32_to_i32 => BinaryenTruncSatSFloat32ToInt32,
    trunc_sat_s_f32_to_i64 => BinaryenTruncSatSFloat32ToInt64,
    trunc_sat_u_f32_to_i32 => BinaryenTruncSatUFloat32ToInt32,
    trunc_sat_u_f32_to_i64 => BinaryenTruncSatUFloat32ToInt64,
    trunc_sat_s_f64_to_i32 => BinaryenTruncSatSFloat64ToInt32,
    trunc_sat_s_f64_to_i64 => BinaryenTruncSatSFloat64ToInt64,
    trunc_sat_u_f64_to_i32 => BinaryenTruncSatUFloat64ToInt32,
    trunc_sat_u_f64_to_i64 => BinaryenTruncSatUFloat64ToInt64,
    splat_vec_i8x16 => BinaryenSplatVecI8x16,
    extract_lane_s_vec_i8x16 => BinaryenExtractLaneSVecI8x16,
    extract_lane_u_vec_i8x16 => BinaryenExtractLaneUVecI8x16,
    replace_lane_vec_i8x16 => BinaryenReplaceLaneVecI8x16,
    splat_vec_i16x8 => BinaryenSplatVecI16x8,
    extract_lane_s_vec_i16x8 => BinaryenExtractLaneSVecI16x8,
    extract_lane_u_vec_i16x8 => BinaryenExtractLaneUVecI16x8,
    replace_lane_vec_i16x8 => BinaryenReplaceLaneVecI16x8,
    splat_vec_i32x4 => BinaryenSplatVecI32x4,
    extract_lane_vec_i32x4 => BinaryenExtractLaneVecI32x4,
    replace_lane_vec_i32x4 => BinaryenReplaceLaneVecI32x4,
    splat_vec_i64x2 => BinaryenSplatVecI64x2,
    extract_lane_vec_i64x2 => BinaryenExtractLaneVecI64x2,
    replace_lane_vec_i64x2 => BinaryenReplaceLaneVecI64x2,
    splat_vec_f32x4 => BinaryenSplatVecF32x4,
    extract_lane_vec_f32x4 => BinaryenExtractLaneVecF32x4,
    replace_lane_vec_f32x4 => BinaryenReplaceLaneVecF32x4,
    splat_vec_f64x2 => BinaryenSplatVecF64x2,
    extract_lane_vec_f64x2 => BinaryenExtractLaneVecF64x2,
    replace_lane_vec_f64x2 => BinaryenReplaceLaneVecF64x2,
    eq_vec_i8x16 => BinaryenEqVecI8x16,
    ne_vec_i8x16 => BinaryenNeVecI8x16,
    lt_s_vec_i8x16 => BinaryenLtSVecI8x16,
    lt_u_vec_i8x16 => BinaryenLtUVecI8x16,
    gt_s_vec_i8x16 => BinaryenGtSVecI8x16,
    gt_u_vec_i8x16 => BinaryenGtUVecI8x16,
    le_s_vec_i8x16 => BinaryenLeSVecI8x16,
    le_u_vec_i8x16 => BinaryenLeUVecI8x16,
    ge_s_vec_i8x16 => BinaryenGeSVecI8x16,
    ge_u_vec_i8x16 => BinaryenGeUVecI8x16,
    eq_vec_i16x8 => BinaryenEqVecI16x8,
    ne_vec_i16x8 => BinaryenNeVecI16x8,
    lt_s_vec_i16x8 => BinaryenLtSVecI16x8,
    lt_u_vec_i16x8 => BinaryenLtUVecI16x8,
    gt_s_vec_i16x8 => BinaryenGtSVecI16x8,
    gt_u_vec_i16x8 => BinaryenGtUVecI16x8,
    le_s_vec_i16x8 => BinaryenLeSVecI16x8,
    le_u_vec_i16x8 => BinaryenLeUVecI16x8,
    ge_s_vec_i16x8 => BinaryenGeSVecI16x8,
    ge_u_vec_i16x8 => BinaryenGeUVecI16x8,
    eq_vec_i32x4 => BinaryenEqVecI32x4,
    ne_vec_i32x4 => BinaryenNeVecI32x4,
    lt_s_vec_i32x4 => BinaryenLtSVecI32x4,
    lt_u_vec_i32x4 => BinaryenLtUVecI32x4,
    gt_s_vec_i32x4 => BinaryenGtSVecI32x4,
    gt_u_vec_i32x4 => BinaryenGtUVecI32x4,
    le_s_vec_i32x4 => BinaryenLeSVecI32x4,
    le_u_vec_i32x4 => BinaryenLeUVecI32x4,
    ge_s_vec_i32x4 => BinaryenGeSVecI32x4,
    ge_u_vec_i32x4 => BinaryenGeUVecI32x4,
    eq_vec_i64x2 => BinaryenEqVecI64x2,
    ne_vec_i64x2 => BinaryenNeVecI64x2,
    lt_s_vec_i64x2 => BinaryenLtSVecI64x2,
    gt_s_vec_i64x2 => BinaryenGtSVecI64x2,
    le_s_vec_i64x2 => BinaryenLeSVecI64x2,
    ge_s_vec_i64x2 => BinaryenGeSVecI64x2,
    eq_vec_f32x4 => BinaryenEqVecF32x4,
    ne_vec_f32x4 => BinaryenNeVecF32x4,
    lt_vec_f32x4 => BinaryenLtVecF32x4,
    gt_vec_f32x4 => BinaryenGtVecF32x4,
    le_vec_f32x4 => BinaryenLeVecF32x4,
    ge_vec_f32x4 => BinaryenGeVecF32x4,
    eq_vec_f64x2 => BinaryenEqVecF64x2,
    ne_vec_f64x2 => BinaryenNeVecF64x2,
    lt_vec_f64x2 => BinaryenLtVecF64x2,
    gt_vec_f64x2 => BinaryenGtVecF64x2,
    le_vec_f64x2 => BinaryenLeVecF64x2,
    ge_vec_f64x2 => BinaryenGeVecF64x2,
    not_vec128 => BinaryenNotVec128,
    and_vec128 => BinaryenAndVec128,
    or_vec128 => BinaryenOrVec128,
    xor_vec128 => BinaryenXorVec128,
    and_not_vec128 => BinaryenAndNotVec128,
    bitselect_vec128 => BinaryenBitselectVec128,
    relaxed_fma_vec_f32x4 => BinaryenRelaxedFmaVecF32x4,
    relaxed_fms_vec_f32x4 => BinaryenRelaxedFmsVecF32x4,
    relaxed_fma_vec_f64x2 => BinaryenRelaxedFmaVecF64x2,
    relaxed_fms_vec_f64x2 => BinaryenRelaxedFmsVecF64x2,
    laneselect_i8x16 => BinaryenLaneselectI8x16,
    laneselect_i16x8 => BinaryenLaneselectI16x8,
    laneselect_i32x4 => BinaryenLaneselectI32x4,
    laneselect_i64x2 => BinaryenLaneselectI64x2,
    dot_i8x16i7x16_add_s_to_vec_i32x4 => BinaryenDotI8x16I7x16AddSToVecI32x4,
    any_true_vec128 => BinaryenAnyTrueVec128,
    popcnt_vec_i8x16 => BinaryenPopcntVecI8x16,
    abs_vec_i8x16 => BinaryenAbsVecI8x16,
    neg_vec_i8x16 => BinaryenNegVecI8x16,
    all_true_vec_i8x16 => BinaryenAllTrueVecI8x16,
    bitmask_vec_i8x16 => BinaryenBitmaskVecI8x16,
    shl_vec_i8x16 => BinaryenShlVecI8x16,
    shr_s_vec_i8x16 => BinaryenShrSVecI8x16,
    shr_u_vec_i8x16 => BinaryenShrUVecI8x16,
    add_vec_i8x16 => BinaryenAddVecI8x16,
    add_sat_s_vec_i8x16 => BinaryenAddSatSVecI8x16,
    add_sat_u_vec_i8x16 => BinaryenAddSatUVecI8x16,
    sub_vec_i8x16 => BinaryenSubVecI8x16,
    sub_sat_s_vec_i8x16 => BinaryenSubSatSVecI8x16,
    sub_sat_u_vec_i8x16 => BinaryenSubSatUVecI8x16,
    min_s_vec_i8x16 => BinaryenMinSVecI8x16,
    min_u_vec_i8x16 => BinaryenMinUVecI8x16,
    max_s_vec_i8x16 => BinaryenMaxSVecI8x16,
    max_u_vec_i8x16 => BinaryenMaxUVecI8x16,
    avgr_u_vec_i8x16 => BinaryenAvgrUVecI8x16,
    abs_vec_i16x8 => BinaryenAbsVecI16x8,
    neg_vec_i16x8 => BinaryenNegVecI16x8,
    all_true_vec_i16x8 => BinaryenAllTrueVecI16x8,
    bitmask_vec_i16x8 => BinaryenBitmaskVecI16x8,
    shl_vec_i16x8 => BinaryenShlVecI16x8,
    shr_s_vec_i16x8 => BinaryenShrSVecI16x8,
    shr_u_vec_i16x8 => BinaryenShrUVecI16x8,
    add_vec_i16x8 => BinaryenAddVecI16x8,
    add_sat_s_vec_i16x8 => BinaryenAddSatSVecI16x8,
    add_sat_u_vec_i16x8 => BinaryenAddSatUVecI16x8,
    sub_vec_i16x8 => BinaryenSubVecI16x8,
    sub_sat_s_vec_i16x8 => BinaryenSubSatSVecI16x8,
    sub_sat_u_vec_i16x8 => BinaryenSubSatUVecI16x8,
    mul_vec_i16x8 => BinaryenMulVecI16x8,
    min_s_vec_i16x8 => BinaryenMinSVecI16x8,
    min_u_vec_i16x8 => BinaryenMinUVecI16x8,
    max_s_vec_i16x8 => BinaryenMaxSVecI16x8,
    max_u_vec_i16x8 => BinaryenMaxUVecI16x8,
    avgr_u_vec_i16x8 => BinaryenAvgrUVecI16x8,
    q15_mulr_sat_s_vec_i16x8 => BinaryenQ15MulrSatSVecI16x8,
    ext_mul_low_s_vec_i16x8 => BinaryenExtMulLowSVecI16x8,
    ext_mul_high_s_vec_i16x8 => BinaryenExtMulHighSVecI16x8,
    ext_mul_low_u_vec_i16x8 => BinaryenExtMulLowUVecI16x8,
    ext_mul_high_u_vec_i16x8 => BinaryenExtMulHighUVecI16x8,
    abs_vec_i32x4 => BinaryenAbsVecI32x4,
    neg_vec_i32x4 => BinaryenNegVecI32x4,
    all_true_vec_i32x4 => BinaryenAllTrueVecI32x4,
    bitmask_vec_i32x4 => BinaryenBitmaskVecI32x4,
    shl_vec_i32x4 => BinaryenShlVecI32x4,
    shr_s_vec_i32x4 => BinaryenShrSVecI32x4,
    shr_u_vec_i32x4 => BinaryenShrUVecI32x4,
    add_vec_i32x4 => BinaryenAddVecI32x4,
    sub_vec_i32x4 => BinaryenSubVecI32x4,
    mul_vec_i32x4 => BinaryenMulVecI32x4,
    min_s_vec_i32x4 => BinaryenMinSVecI32x4,
    min_u_vec_i32x4 => BinaryenMinUVecI32x4,
    max_s_vec_i32x4 => BinaryenMaxSVecI32x4,
    max_u_vec_i32x4 => BinaryenMaxUVecI32x4,
    dot_s_vec_i16x8_to_vec_i32x4 => BinaryenDotSVecI16x8ToVecI32x4,
    ext_mul_low_s_vec_i32x4 => BinaryenExtMulLowSVecI32x4,
    ext_mul_high_s_vec_i32x4 => BinaryenExtMulHighSVecI32x4,
    ext_mul_low_u_vec_i32x4 => BinaryenExtMulLowUVecI32x4,
    ext_mul_high_u_vec_i32x4 => BinaryenExtMulHighUVecI32x4,
    abs_vec_i64x2 => BinaryenAbsVecI64x2,
    neg_vec_i64x2 => BinaryenNegVecI64x2,
    all_true_vec_i64x2 => BinaryenAllTrueVecI64x2,
    bitmask_vec_i64x2 => BinaryenBitmaskVecI64x2,
    shl_vec_i64x2 => BinaryenShlVecI64x2,
    shr_s_vec_i64x2 => BinaryenShrSVecI64x2,
    shr_u_vec_i64x2 => BinaryenShrUVecI64x2,
    add_vec_i64x2 => BinaryenAddVecI64x2,
    sub_vec_i64x2 => BinaryenSubVecI64x2,
    mul_vec_i64x2 => BinaryenMulVecI64x2,
    ext_mul_low_s_vec_i64x2 => BinaryenExtMulLowSVecI64x2,
    ext_mul_high_s_vec_i64x2 => BinaryenExtMulHighSVecI64x2,
    ext_mul_low_u_vec_i64x2 => BinaryenExtMulLowUVecI64x2,
    ext_mul_high_u_vec_i64x2 => BinaryenExtMulHighUVecI64x2,
    abs_vec_f32x4 => BinaryenAbsVecF32x4,
    neg_vec_f32x4 => BinaryenNegVecF32x4,
    sqrt_vec_f32x4 => BinaryenSqrtVecF32x4,
    add_vec_f32x4 => BinaryenAddVecF32x4,
    sub_vec_f32x4 => BinaryenSubVecF32x4,
    mul_vec_f32x4 => BinaryenMulVecF32x4,
    div_vec_f32x4 => BinaryenDivVecF32x4,
    min_vec_f32x4 => BinaryenMinVecF32x4,
    max_vec_f32x4 => BinaryenMaxVecF32x4,
    p_min_vec_f32x4 => BinaryenPMinVecF32x4,
    p_max_vec_f32x4 => BinaryenPMaxVecF32x4,
    ceil_vec_f32x4 => BinaryenCeilVecF32x4,
    floor_vec_f32x4 => BinaryenFloorVecF32x4,
    trunc_vec_f32x4 => BinaryenTruncVecF32x4,
    nearest_vec_f32x4 => BinaryenNearestVecF32x4,
    abs_vec_f64x2 => BinaryenAbsVecF64x2,
    neg_vec_f64x2 => BinaryenNegVecF64x2,
    sqrt_vec_f64x2 => BinaryenSqrtVecF64x2,
    add_vec_f64x2 => BinaryenAddVecF64x2,
    sub_vec_f64x2 => BinaryenSubVecF64x2,
    mul_vec_f64x2 => BinaryenMulVecF64x2,
    div_vec_f64x2 => BinaryenDivVecF64x2,
    min_vec_f64x2 => BinaryenMinVecF64x2,
    max_vec_f64x2 => BinaryenMaxVecF64x2,
    p_min_vec_f64x2 => BinaryenPMinVecF64x2,
    p_max_vec_f64x2 => BinaryenPMaxVecF64x2,
    ceil_vec_f64x2 => BinaryenCeilVecF64x2,
    floor_vec_f64x2 => BinaryenFloorVecF64x2,
    trunc_vec_f64x2 => BinaryenTruncVecF64x2,
    nearest_vec_f64x2 => BinaryenNearestVecF64x2,
    ext_add_pairwise_s_vec_i8x16_to_i16x8 => BinaryenExtAddPairwiseSVecI8x16ToI16x8,
    ext_add_pairwise_u_vec_i8x16_to_i16x8 => BinaryenExtAddPairwiseUVecI8x16ToI16x8,
    ext_add_pairwise_s_vec_i16x8_to_i32x4 => BinaryenExtAddPairwiseSVecI16x8ToI32x4,
    ext_add_pairwise_u_vec_i16x8_to_i32x4 => BinaryenExtAddPairwiseUVecI16x8ToI32x4,
    trunc_sat_s_vec_f32x4_to_vec_i32x4 => BinaryenTruncSatSVecF32x4ToVecI32x4,
    trunc_sat_u_vec_f32x4_to_vec_i32x4 => BinaryenTruncSatUVecF32x4ToVecI32x4,
    convert_s_vec_i32x4_to_vec_f32x4 => BinaryenConvertSVecI32x4ToVecF32x4,
    convert_u_vec_i32x4_to_vec_f32x4 => BinaryenConvertUVecI32x4ToVecF32x4,
    load8_splat_vec128 => BinaryenLoad8SplatVec128,
    load16_splat_vec128 => BinaryenLoad16SplatVec128,
    load32_splat_vec128 => BinaryenLoad32SplatVec128,
    load64_splat_vec128 => BinaryenLoad64SplatVec128,
    load8x8s_vec128 => BinaryenLoad8x8SVec128,
    load8x8u_vec128 => BinaryenLoad8x8UVec128,
    load16x4s_vec128 => BinaryenLoad16x4SVec128,
    load16x4u_vec128 => BinaryenLoad16x4UVec128,
    load32x2s_vec128 => BinaryenLoad32x2SVec128,
    load32x2u_vec128 => BinaryenLoad32x2UVec128,
    load32_zero_vec128 => BinaryenLoad32ZeroVec128,
    load64_zero_vec128 => BinaryenLoad64ZeroVec128,
    load8_lane_vec128 => BinaryenLoad8LaneVec128,
    load16_lane_vec128 => BinaryenLoad16LaneVec128,
    load32_lane_vec128 => BinaryenLoad32LaneVec128,
    load64_lane_vec128 => BinaryenLoad64LaneVec128,
    store8_lane_vec128 => BinaryenStore8LaneVec128,
    store16_lane_vec128 => BinaryenStore16LaneVec128,
    store32_lane_vec128 => BinaryenStore32LaneVec128,
    store64_lane_vec128 => BinaryenStore64LaneVec128,
    narrow_s_vec_i16x8_to_vec_i8x16 => BinaryenNarrowSVecI16x8ToVecI8x16,
    narrow_u_vec_i16x8_to_vec_i8x16 => BinaryenNarrowUVecI16x8ToVecI8x16,
    narrow_s_vec_i32x4_to_vec_i16x8 => BinaryenNarrowSVecI32x4ToVecI16x8,
    narrow_u_vec_i32x4_to_vec_i16x8 => BinaryenNarrowUVecI32x4ToVecI16x8,
    extend_low_s_vec_i8x16_to_vec_i16x8 => BinaryenExtendLowSVecI8x16ToVecI16x8,
    extend_high_s_vec_i8x16_to_vec_i16x8 => BinaryenExtendHighSVecI8x16ToVecI16x8,
    extend_low_u_vec_i8x16_to_vec_i16x8 => BinaryenExtendLowUVecI8x16ToVecI16x8,
    extend_high_u_vec_i8x16_to_vec_i16x8 => BinaryenExtendHighUVecI8x16ToVecI16x8,
    extend_low_s_vec_i16x8_to_vec_i32x4 => BinaryenExtendLowSVecI16x8ToVecI32x4,
    extend_high_s_vec_i16x8_to_vec_i32x4 => BinaryenExtendHighSVecI16x8ToVecI32x4,
    extend_low_u_vec_i16x8_to_vec_i32x4 => BinaryenExtendLowUVecI16x8ToVecI32x4,
    extend_high_u_vec_i16x8_to_vec_i32x4 => BinaryenExtendHighUVecI16x8ToVecI32x4,
    extend_low_s_vec_i32x4_to_vec_i64x2 => BinaryenExtendLowSVecI32x4ToVecI64x2,
    extend_high_s_vec_i32x4_to_vec_i64x2 => BinaryenExtendHighSVecI32x4ToVecI64x2,
    extend_low_u_vec_i32x4_to_vec_i64x2 => BinaryenExtendLowUVecI32x4ToVecI64x2,
    extend_high_u_vec_i32x4_to_vec_i64x2 => BinaryenExtendHighUVecI32x4ToVecI64x2,
    convert_low_s_vec_i32x4_to_vec_f64x2 => BinaryenConvertLowSVecI32x4ToVecF64x2,
    convert_low_u_vec_i32x4_to_vec_f64x2 => BinaryenConvertLowUVecI32x4ToVecF64x2,
    trunc_sat_zero_s_vec_f64x2_to_vec_i32x4 => BinaryenTruncSatZeroSVecF64x2ToVecI32x4,
    trunc_sat_zero_u_vec_f64x2_to_vec_i32x4 => BinaryenTruncSatZeroUVecF64x2ToVecI32x4,
    demote_zero_vec_f64x2_to_vec_f32x4 => BinaryenDemoteZeroVecF64x2ToVecF32x4,
    promote_low_vec_f32x4_to_vec_f64x2 => BinaryenPromoteLowVecF32x4ToVecF64x2,
    relaxed_trunc_s_vec_f32x4_to_vec_i32x4 => BinaryenRelaxedTruncSVecF32x4ToVecI32x4,
    relaxed_trunc_u_vec_f32x4_to_vec_i32x4 => BinaryenRelaxedTruncUVecF32x4ToVecI32x4,
    relaxed_trunc_zero_s_vec_f64x2_to_vec_i32x4 => BinaryenRelaxedTruncZeroSVecF64x2ToVecI32x4,
    relaxed_trunc_zero_u_vec_f64x2_to_vec_i32x4 => BinaryenRelaxedTruncZeroUVecF64x2ToVecI32x4,
    swizzle_vec_i8x16 => BinaryenSwizzleVecI8x16,
    relaxed_swizzle_vec_i8x16 => BinaryenRelaxedSwizzleVecI8x16,
    relaxed_min_vec_f32x4 => BinaryenRelaxedMinVecF32x4,
    relaxed_max_vec_f32x4 => BinaryenRelaxedMaxVecF32x4,
    relaxed_min_vec_f64x2 => BinaryenRelaxedMinVecF64x2,
    relaxed_max_vec_f64x2 => BinaryenRelaxedMaxVecF64x2,
    relaxed_q15_mulr_s_vec_i16x8 => BinaryenRelaxedQ15MulrSVecI16x8,
    dot_i8x16i7x16s_to_vec_i16x8 => BinaryenDotI8x16I7x16SToVecI16x8,
    ref_as_non_null => BinaryenRefAsNonNull,
    ref_as_extern_internalize => BinaryenRefAsExternInternalize,
    ref_as_extern_externalize => BinaryenRefAsExternExternalize,
    br_on_null => BinaryenBrOnNull,
    br_on_non_null => BinaryenBrOnNonNull,
    br_on_cast => BinaryenBrOnCast,
    br_on_cast_fail => BinaryenBrOnCastFail,
    string_new_utf8 => BinaryenStringNewUTF8,
    string_new_wtf8 => BinaryenStringNewWTF8,
    string_new_replace => BinaryenStringNewReplace,
    string_new_wtf16 => BinaryenStringNewWTF16,
    string_new_utf8_array => BinaryenStringNewUTF8Array,
    string_new_wtf8_array => BinaryenStringNewWTF8Array,
    string_new_replace_array => BinaryenStringNewReplaceArray,
    string_new_wtf16_array => BinaryenStringNewWTF16Array,
    string_new_from_code_point => BinaryenStringNewFromCodePoint,
    string_measure_utf8 => BinaryenStringMeasureUTF8,
    string_measure_wtf8 => BinaryenStringMeasureWTF8,
    string_measure_wtf16 => BinaryenStringMeasureWTF16,
    string_measure_is_usv => BinaryenStringMeasureIsUSV,
    string_measure_wtf16_view => BinaryenStringMeasureWTF16View,
    string_encode_utf8 => BinaryenStringEncodeUTF8,
    string_encode_wtf8 => BinaryenStringEncodeWTF8,
    string_encode_wtf16 => BinaryenStringEncodeWTF16,
    string_encode_utf8_array => BinaryenStringEncodeUTF8Array,
    string_encode_wtf8_array => BinaryenStringEncodeWTF8Array,
    string_encode_wtf16_array => BinaryenStringEncodeWTF16Array,
    string_as_wtf8 => BinaryenStringAsWTF8,
    string_as_wtf16 => BinaryenStringAsWTF16,
    string_as_iter => BinaryenStringAsIter,
    string_iter_move_advance => BinaryenStringIterMoveAdvance,
    string_iter_move_rewind => BinaryenStringIterMoveRewind,
    string_slice_wtf8 => BinaryenStringSliceWTF8,
    string_slice_wtf16 => BinaryenStringSliceWTF16,
    string_eq_equal => BinaryenStringEqEqual,
    string_eq_compare => BinaryenStringEqCompare
}
