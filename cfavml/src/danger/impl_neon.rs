use core::arch::aarch64::*;
use core::iter::zip;
use core::mem;

use crate::danger::{DenseLane, SimdRegister};
use crate::math::{AutoMath, Math};

/// NEON enabled SIMD operations.
///
/// This requires the `neon` CPU features be enabled.
pub struct Neon;

impl SimdRegister<f32> for Neon {
    type Register = float32x4_t;

    #[inline(always)]
    unsafe fn load(mem: *const f32) -> Self::Register {
        vld1q_f32(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: f32) -> Self::Register {
        vdupq_n_f32(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<f32>>::filled(0.0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vdivq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        vfmaq_f32(acc, l1, l2)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_f32(l1, l2)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> f32 {
        vaddvq_f32(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> f32 {
        vmaxvq_f32(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> f32 {
        vminvq_f32(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut f32, reg: Self::Register) {
        vst1q_f32(mem, reg)
    }
}

impl SimdRegister<f64> for Neon {
    type Register = float64x2_t;

    #[inline(always)]
    unsafe fn load(mem: *const f64) -> Self::Register {
        vld1q_f64(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: f64) -> Self::Register {
        vdupq_n_f64(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<f64>>::filled(0.0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vdivq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        vfmaq_f64(acc, l1, l2)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_f64(l1, l2)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> f64 {
        vaddvq_f64(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> f64 {
        vmaxvq_f64(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> f64 {
        vminvq_f64(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut f64, reg: Self::Register) {
        vst1q_f64(mem, reg)
    }
}

impl SimdRegister<i8> for Neon {
    type Register = int8x16_t;

    #[inline(always)]
    unsafe fn load(mem: *const i8) -> Self::Register {
        vld1q_s8(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: i8) -> Self::Register {
        vdupq_n_s8(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<i8>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_s8(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_s8(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_s8(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i8; 16]>(l1);
        let l2_unpacked = mem::transmute::<_, [i8; 16]>(l2);

        let mut result = [0i8; 16];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<i8>>::mul(l1, l2);
        <Self as SimdRegister<i8>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_s8(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_s8(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<i8>>::mul_dense(l1, l2);
        <Self as SimdRegister<i8>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> i8 {
        vaddvq_s8(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> i8 {
        vmaxvq_s8(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> i8 {
        vminvq_s8(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut i8, reg: Self::Register) {
        vst1q_s8(mem, reg)
    }
}

impl SimdRegister<i16> for Neon {
    type Register = int16x8_t;

    #[inline(always)]
    unsafe fn load(mem: *const i16) -> Self::Register {
        vld1q_s16(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: i16) -> Self::Register {
        vdupq_n_s16(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<i16>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_s16(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_s16(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_s16(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i16; 8]>(l1);
        let l2_unpacked = mem::transmute::<_, [i16; 8]>(l2);

        let mut result = [0i16; 8];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<i16>>::mul(l1, l2);
        <Self as SimdRegister<i16>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_s16(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_s16(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<i16>>::mul_dense(l1, l2);
        <Self as SimdRegister<i16>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> i16 {
        vaddvq_s16(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> i16 {
        vmaxvq_s16(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> i16 {
        vminvq_s16(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut i16, reg: Self::Register) {
        vst1q_s16(mem, reg)
    }
}

impl SimdRegister<i32> for Neon {
    type Register = int32x4_t;

    #[inline(always)]
    unsafe fn load(mem: *const i32) -> Self::Register {
        vld1q_s32(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: i32) -> Self::Register {
        vdupq_n_s32(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<i32>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_s32(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_s32(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_s32(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i32; 4]>(l1);
        let l2_unpacked = mem::transmute::<_, [i32; 4]>(l2);

        let mut result = [0i32; 4];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<i32>>::mul(l1, l2);
        <Self as SimdRegister<i32>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_s32(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_s32(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<i32>>::mul_dense(l1, l2);
        <Self as SimdRegister<i32>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> i32 {
        vaddvq_s32(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> i32 {
        vmaxvq_s32(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> i32 {
        vminvq_s32(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut i32, reg: Self::Register) {
        vst1q_s32(mem, reg)
    }
}

impl SimdRegister<i64> for Neon {
    type Register = int64x2_t;

    #[inline(always)]
    unsafe fn load(mem: *const i64) -> Self::Register {
        vld1q_s64(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: i64) -> Self::Register {
        vdupq_n_s64(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<i64>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_s64(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_s64(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [i64; 2]>(l2);

        let mut result = [0i64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::mul(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [i64; 2]>(l2);

        let mut result = [0i64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<i64>>::mul(l1, l2);
        <Self as SimdRegister<i64>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [i64; 2]>(l2);

        let mut result = [0i64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = core::cmp::max(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [i64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [i64; 2]>(l2);

        let mut result = [0i64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = core::cmp::min(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<i64>>::mul_dense(l1, l2);
        <Self as SimdRegister<i64>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> i64 {
        vaddvq_s64(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> i64 {
        let [a, b] = mem::transmute::<_, [i64; 2]>(reg);
        core::cmp::max(a, b)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> i64 {
        let [a, b] = mem::transmute::<_, [i64; 2]>(reg);
        core::cmp::min(a, b)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut i64, reg: Self::Register) {
        vst1q_s64(mem, reg)
    }
}

impl SimdRegister<u8> for Neon {
    type Register = uint8x16_t;

    #[inline(always)]
    unsafe fn load(mem: *const u8) -> Self::Register {
        vld1q_u8(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: u8) -> Self::Register {
        vdupq_n_u8(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<u8>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_u8(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_u8(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_u8(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u8; 16]>(l1);
        let l2_unpacked = mem::transmute::<_, [u8; 16]>(l2);

        let mut result = [0u8; 16];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<u8>>::mul(l1, l2);
        <Self as SimdRegister<u8>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_u8(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_u8(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<u8>>::mul_dense(l1, l2);
        <Self as SimdRegister<u8>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> u8 {
        vaddvq_u8(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> u8 {
        vmaxvq_u8(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> u8 {
        vminvq_u8(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut u8, reg: Self::Register) {
        vst1q_u8(mem, reg)
    }
}

impl SimdRegister<u16> for Neon {
    type Register = uint16x8_t;

    #[inline(always)]
    unsafe fn load(mem: *const u16) -> Self::Register {
        vld1q_u16(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: u16) -> Self::Register {
        vdupq_n_u16(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<u16>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_u16(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_u16(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_u16(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u16; 8]>(l1);
        let l2_unpacked = mem::transmute::<_, [u16; 8]>(l2);

        let mut result = [0u16; 8];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<u16>>::mul(l1, l2);
        <Self as SimdRegister<u16>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_u16(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_u16(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<u16>>::mul_dense(l1, l2);
        <Self as SimdRegister<u16>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> u16 {
        vaddvq_u16(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> u16 {
        vmaxvq_u16(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> u16 {
        vminvq_u16(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut u16, reg: Self::Register) {
        vst1q_u16(mem, reg)
    }
}

impl SimdRegister<u32> for Neon {
    type Register = uint32x4_t;

    #[inline(always)]
    unsafe fn load(mem: *const u32) -> Self::Register {
        vld1q_u32(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: u32) -> Self::Register {
        vdupq_n_u32(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<u32>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_u32(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_u32(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmulq_u32(l1, l2)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u32; 4]>(l1);
        let l2_unpacked = mem::transmute::<_, [u32; 4]>(l2);

        let mut result = [0u32; 4];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<u32>>::mul(l1, l2);
        <Self as SimdRegister<u32>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vmaxq_u32(l1, l2)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vminq_u32(l1, l2)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<u32>>::mul_dense(l1, l2);
        <Self as SimdRegister<u32>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> u32 {
        vaddvq_u32(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> u32 {
        vmaxvq_u32(reg)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> u32 {
        vminvq_u32(reg)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut u32, reg: Self::Register) {
        vst1q_u32(mem, reg)
    }
}

impl SimdRegister<u64> for Neon {
    type Register = uint64x2_t;

    #[inline(always)]
    unsafe fn load(mem: *const u64) -> Self::Register {
        vld1q_u64(mem)
    }

    #[inline(always)]
    unsafe fn filled(value: u64) -> Self::Register {
        vdupq_n_u64(value)
    }

    #[inline(always)]
    unsafe fn zeroed() -> Self::Register {
        <Self as SimdRegister<u64>>::filled(0)
    }

    #[inline(always)]
    unsafe fn add(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vaddq_u64(l1, l2)
    }

    #[inline(always)]
    unsafe fn sub(l1: Self::Register, l2: Self::Register) -> Self::Register {
        vsubq_u64(l1, l2)
    }

    #[inline(always)]
    unsafe fn mul(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [u64; 2]>(l2);

        let mut result = [0u64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::mul(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn div(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [u64; 2]>(l2);

        let mut result = [0u64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = AutoMath::div(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd(
        l1: Self::Register,
        l2: Self::Register,
        acc: Self::Register,
    ) -> Self::Register {
        let res = <Self as SimdRegister<u64>>::mul(l1, l2);
        <Self as SimdRegister<u64>>::add(res, acc)
    }

    #[inline(always)]
    unsafe fn max(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [u64; 2]>(l2);

        let mut result = [0u64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = core::cmp::max(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn min(l1: Self::Register, l2: Self::Register) -> Self::Register {
        let l1_unpacked = mem::transmute::<_, [u64; 2]>(l1);
        let l2_unpacked = mem::transmute::<_, [u64; 2]>(l2);

        let mut result = [0u64; 2];
        for (idx, (l1, l2)) in zip(l1_unpacked, l2_unpacked).enumerate() {
            result[idx] = core::cmp::min(l1, l2);
        }

        mem::transmute::<_, Self::Register>(result)
    }

    #[inline(always)]
    unsafe fn fmadd_dense(
        l1: DenseLane<Self::Register>,
        l2: DenseLane<Self::Register>,
        acc: DenseLane<Self::Register>,
    ) -> DenseLane<Self::Register> {
        let res = <Self as SimdRegister<u64>>::mul_dense(l1, l2);
        <Self as SimdRegister<u64>>::add_dense(res, acc)
    }

    #[inline(always)]
    unsafe fn sum_to_value(reg: Self::Register) -> u64 {
        vaddvq_u64(reg)
    }

    #[inline(always)]
    unsafe fn max_to_value(reg: Self::Register) -> u64 {
        let [a, b] = mem::transmute::<_, [u64; 2]>(reg);
        core::cmp::max(a, b)
    }

    #[inline(always)]
    unsafe fn min_to_value(reg: Self::Register) -> u64 {
        let [a, b] = mem::transmute::<_, [u64; 2]>(reg);
        core::cmp::min(a, b)
    }

    #[inline(always)]
    unsafe fn write(mem: *mut u64, reg: Self::Register) {
        vst1q_u64(mem, reg)
    }
}
