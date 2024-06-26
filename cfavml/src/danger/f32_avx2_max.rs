use core::arch::x86_64::*;
use core::{mem, ptr};

use crate::danger::{copy_avx2_ps_register_to, offsets_avx2_ps, CHUNK_0, CHUNK_1};

#[target_feature(enable = "avx2")]
#[inline]
/// Computes the horizontal maximum of the given vector that is `[f32; DIMS]`.
///
/// # Safety
///
/// `DIMS` **MUST** be a multiple of `64`, otherwise this routine
/// will become immediately UB due to out of bounds pointer accesses.
///
/// This method assumes AVX2 instructions are available, if this method is executed
/// on non-AVX2 enabled systems, it will lead to an `ILLEGAL_INSTRUCTION` error.
pub unsafe fn f32_xconst_avx2_nofma_max_horizontal<const DIMS: usize>(
    arr: &[f32],
) -> f32 {
    debug_assert_eq!(arr.len(), DIMS, "Array length must match DIMS");
    debug_assert_eq!(DIMS % 64, 0, "DIMS must be a multiple of 64");

    let arr = arr.as_ptr();

    let mut acc1 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc2 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc3 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc4 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc5 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc6 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc7 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc8 = _mm256_set1_ps(f32::NEG_INFINITY);

    let mut i = 0;
    while i < DIMS {
        let [x1, x2, x3, x4] = offsets_avx2_ps::<CHUNK_0>(arr.add(i));
        let [x5, x6, x7, x8] = offsets_avx2_ps::<CHUNK_1>(arr.add(i));

        let x1 = _mm256_loadu_ps(x1);
        let x2 = _mm256_loadu_ps(x2);
        let x3 = _mm256_loadu_ps(x3);
        let x4 = _mm256_loadu_ps(x4);
        let x5 = _mm256_loadu_ps(x5);
        let x6 = _mm256_loadu_ps(x6);
        let x7 = _mm256_loadu_ps(x7);
        let x8 = _mm256_loadu_ps(x8);

        acc1 = _mm256_max_ps(acc1, x1);
        acc2 = _mm256_max_ps(acc2, x2);
        acc3 = _mm256_max_ps(acc3, x3);
        acc4 = _mm256_max_ps(acc4, x4);
        acc5 = _mm256_max_ps(acc5, x5);
        acc6 = _mm256_max_ps(acc6, x6);
        acc7 = _mm256_max_ps(acc7, x7);
        acc8 = _mm256_max_ps(acc8, x8);

        i += 64;
    }

    acc1 = _mm256_max_ps(acc1, acc2);
    acc3 = _mm256_max_ps(acc3, acc4);
    acc5 = _mm256_max_ps(acc5, acc6);
    acc7 = _mm256_max_ps(acc7, acc8);

    acc1 = _mm256_max_ps(acc1, acc3);
    acc5 = _mm256_max_ps(acc5, acc7);

    acc1 = _mm256_max_ps(acc1, acc5);

    let unpacked = mem::transmute::<__m256, [f32; 8]>(acc1);

    // This is technically not the full SIMD way of doing this, but it is simpler,
    // and I am not convinced this really has a significant performance impact to warrant
    // the extra work needed to maintain it in the future.
    let mut max = f32::NEG_INFINITY;
    for x in unpacked {
        max = max.max(x);
    }

    max
}

#[target_feature(enable = "avx2")]
#[allow(unused)]
#[inline]
/// Computes the vertical maximum of the given vector that is `[[f32; DIMS]; N]`.
///
/// # Safety
///
/// `DIMS` **MUST** be a multiple of `64`, otherwise this routine
/// will become immediately UB due to out of bounds pointer accesses.
///
/// This method assumes AVX2 instructions are available, if this method is executed
/// on non-AVX2 enabled systems, it will lead to an `ILLEGAL_INSTRUCTION` error.
pub unsafe fn f32_xconst_avx2_nofma_max_vertical<const DIMS: usize>(
    matrix: &[&[f32]],
) -> Vec<f32> {
    debug_assert_eq!(DIMS % 64, 0, "DIMS must be a multiple of 64");

    let mut max_values = vec![0.0; DIMS];
    let max_values_ptr = max_values.as_mut_ptr();

    // We work our way horizontally by taking steps of 64 and finding
    // the max of for each of the lanes vertically through the matrix.
    // TODO: I am unsure how hard this is on the cache or if there is a smarter
    //       way to write this.
    let mut i = 0;
    while i < DIMS {
        let mut acc1 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc2 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc3 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc4 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc5 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc6 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc7 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc8 = _mm256_set1_ps(f32::NEG_INFINITY);

        // Vertical max of the 64 elements.
        for m in 0..matrix.len() {
            let arr = *matrix.get_unchecked(m);
            let arr = arr.as_ptr();

            let [x1, x2, x3, x4] = offsets_avx2_ps::<CHUNK_0>(arr.add(i));
            let [x5, x6, x7, x8] = offsets_avx2_ps::<CHUNK_1>(arr.add(i));

            let x1 = _mm256_loadu_ps(x1);
            let x2 = _mm256_loadu_ps(x2);
            let x3 = _mm256_loadu_ps(x3);
            let x4 = _mm256_loadu_ps(x4);
            let x5 = _mm256_loadu_ps(x5);
            let x6 = _mm256_loadu_ps(x6);
            let x7 = _mm256_loadu_ps(x7);
            let x8 = _mm256_loadu_ps(x8);

            acc1 = _mm256_max_ps(acc1, x1);
            acc2 = _mm256_max_ps(acc2, x2);
            acc3 = _mm256_max_ps(acc3, x3);
            acc4 = _mm256_max_ps(acc4, x4);
            acc5 = _mm256_max_ps(acc5, x5);
            acc6 = _mm256_max_ps(acc6, x6);
            acc7 = _mm256_max_ps(acc7, x7);
            acc8 = _mm256_max_ps(acc8, x8);
        }

        let merged = [acc1, acc2, acc3, acc4, acc5, acc6, acc7, acc8];

        let result = mem::transmute::<[__m256; 8], [f32; 64]>(merged);
        ptr::copy_nonoverlapping(result.as_ptr(), max_values_ptr.add(i), result.len());

        i += 64;
    }

    max_values
}

#[target_feature(enable = "avx2")]
#[inline]
/// Computes the horizontal maximum of the given vector that is `[f32; N]`.
///
/// # Safety
///
/// This method assumes AVX2 instructions are available, if this method is executed
/// on non-AVX2 enabled systems, it will lead to an `ILLEGAL_INSTRUCTION` error.
pub unsafe fn f32_xany_avx2_nofma_max_horizontal(arr: &[f32]) -> f32 {
    let len = arr.len();
    let offset_from = len % 64;

    let mut max = f32::NEG_INFINITY;

    let mut acc1 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc2 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc3 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc4 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc5 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc6 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc7 = _mm256_set1_ps(f32::NEG_INFINITY);
    let mut acc8 = _mm256_set1_ps(f32::NEG_INFINITY);

    let arr_ptr = arr.as_ptr();

    let mut i = 0;
    while i < (len - offset_from) {
        let [x1, x2, x3, x4] = offsets_avx2_ps::<CHUNK_0>(arr_ptr.add(i));
        let [x5, x6, x7, x8] = offsets_avx2_ps::<CHUNK_1>(arr_ptr.add(i));

        let x1 = _mm256_loadu_ps(x1);
        let x2 = _mm256_loadu_ps(x2);
        let x3 = _mm256_loadu_ps(x3);
        let x4 = _mm256_loadu_ps(x4);
        let x5 = _mm256_loadu_ps(x5);
        let x6 = _mm256_loadu_ps(x6);
        let x7 = _mm256_loadu_ps(x7);
        let x8 = _mm256_loadu_ps(x8);

        acc1 = _mm256_max_ps(acc1, x1);
        acc2 = _mm256_max_ps(acc2, x2);
        acc3 = _mm256_max_ps(acc3, x3);
        acc4 = _mm256_max_ps(acc4, x4);
        acc5 = _mm256_max_ps(acc5, x5);
        acc6 = _mm256_max_ps(acc6, x6);
        acc7 = _mm256_max_ps(acc7, x7);
        acc8 = _mm256_max_ps(acc8, x8);

        i += 64;
    }

    if offset_from != 0 {
        let tail = offset_from % 8;

        while i < (len - tail) {
            let x = _mm256_loadu_ps(arr_ptr.add(i));
            acc1 = _mm256_max_ps(acc1, x);

            i += 8;
        }

        for n in i..len {
            let x = *arr.get_unchecked(n);
            max = max.max(x);
        }
    }

    acc1 = _mm256_max_ps(acc1, acc2);
    acc3 = _mm256_max_ps(acc3, acc4);
    acc5 = _mm256_max_ps(acc5, acc6);
    acc7 = _mm256_max_ps(acc7, acc8);

    acc1 = _mm256_max_ps(acc1, acc3);
    acc5 = _mm256_max_ps(acc5, acc7);

    acc1 = _mm256_max_ps(acc1, acc5);

    let unpacked = mem::transmute::<__m256, [f32; 8]>(acc1);

    // This is technically not the full SIMD way of doing this, but it is simpler,
    // and I am not convinced this really has a significant performance impact to warrant
    // the extra work needed to maintain it in the future.
    for x in unpacked {
        max = max.max(x);
    }

    max
}

#[target_feature(enable = "avx2")]
#[allow(unused)]
#[inline]
/// Computes the vertical maximum of the given vector that is `[[f32; N]; N2]`.
///
/// # Safety
///
/// The size of each array in the matrix must be equal otherwise out of bounds
/// access can occur.
///
/// This method assumes AVX2 instructions are available, if this method is executed
/// on non-AVX2 enabled systems, it will lead to an `ILLEGAL_INSTRUCTION` error.
pub unsafe fn f32_xany_avx2_nofma_max_vertical(matrix: &[&[f32]]) -> Vec<f32> {
    let len = matrix[0].len();
    let offset_from = len % 64;

    let mut max_values = vec![0.0; len];
    let max_values_ptr = max_values.as_mut_ptr();

    // We work our way horizontally by taking steps of 64 and finding
    // the max of for each of the lanes vertically through the matrix.
    // TODO: I am unsure how hard this is on the cache or if there is a smarter
    //       way to write this.
    let mut i = 0;
    while i < (len - offset_from) {
        let mut acc1 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc2 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc3 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc4 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc5 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc6 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc7 = _mm256_set1_ps(f32::NEG_INFINITY);
        let mut acc8 = _mm256_set1_ps(f32::NEG_INFINITY);

        // Vertical max of the 64 elements.
        for m in 0..matrix.len() {
            let arr = *matrix.get_unchecked(m);
            debug_assert_eq!(arr.len(), len);

            let arr = arr.as_ptr();

            let [x1, x2, x3, x4] = offsets_avx2_ps::<CHUNK_0>(arr.add(i));
            let [x5, x6, x7, x8] = offsets_avx2_ps::<CHUNK_1>(arr.add(i));

            let x1 = _mm256_loadu_ps(x1);
            let x2 = _mm256_loadu_ps(x2);
            let x3 = _mm256_loadu_ps(x3);
            let x4 = _mm256_loadu_ps(x4);
            let x5 = _mm256_loadu_ps(x5);
            let x6 = _mm256_loadu_ps(x6);
            let x7 = _mm256_loadu_ps(x7);
            let x8 = _mm256_loadu_ps(x8);

            acc1 = _mm256_max_ps(acc1, x1);
            acc2 = _mm256_max_ps(acc2, x2);
            acc3 = _mm256_max_ps(acc3, x3);
            acc4 = _mm256_max_ps(acc4, x4);
            acc5 = _mm256_max_ps(acc5, x5);
            acc6 = _mm256_max_ps(acc6, x6);
            acc7 = _mm256_max_ps(acc7, x7);
            acc8 = _mm256_max_ps(acc8, x8);
        }

        let merged = [acc1, acc2, acc3, acc4, acc5, acc6, acc7, acc8];

        let result = mem::transmute::<[__m256; 8], [f32; 64]>(merged);
        ptr::copy_nonoverlapping(result.as_ptr(), max_values_ptr.add(i), result.len());

        i += 64;
    }

    if offset_from != 0 {
        let tail = offset_from % 8;

        while i < (len - tail) {
            let mut acc = _mm256_set1_ps(f32::NEG_INFINITY);
            for m in 0..matrix.len() {
                let arr = *matrix.get_unchecked(m);
                debug_assert_eq!(arr.len(), len);
                let arr_ptr = arr.as_ptr();
                let x = _mm256_loadu_ps(arr_ptr.add(i));
                acc = _mm256_max_ps(acc, x);
            }
            copy_avx2_ps_register_to(max_values_ptr.add(i), acc);

            i += 8;
        }

        for n in i..len {
            let mut max = f32::NEG_INFINITY;
            for m in 0..matrix.len() {
                let arr = *matrix.get_unchecked(m);
                debug_assert_eq!(arr.len(), len);
                let x = *arr.get_unchecked(n);
                max = max.max(x);
            }
            *max_values.get_unchecked_mut(n) = max;
        }
    }

    max_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_sample_vectors;

    #[test]
    fn test_xconst_nofma_max_horizontal() {
        let (x, _) = get_sample_vectors(512);
        let max = unsafe { f32_xconst_avx2_nofma_max_horizontal::<512>(&x) };
        assert_eq!(max, x.iter().fold(f32::NEG_INFINITY, |acc, v| acc.max(*v)));
    }

    #[test]
    fn test_xconst_nofma_max_vertical() {
        let mut matrix = Vec::new();
        for _ in 0..25 {
            let (x, _) = get_sample_vectors(512);
            matrix.push(x);
        }

        let matrix_view = matrix.iter().map(|v| v.as_ref()).collect::<Vec<&[f32]>>();

        let mut expected_vertical_max = vec![f32::NEG_INFINITY; 512];
        for i in 0..512 {
            let mut max = f32::NEG_INFINITY;
            for arr in matrix.iter() {
                max = max.max(arr[i]);
            }
            expected_vertical_max[i] = max;
        }

        let max = unsafe { f32_xconst_avx2_nofma_max_vertical::<512>(&matrix_view) };
        assert_eq!(max, expected_vertical_max);
    }

    #[test]
    fn test_xany_nofma_max_horizontal() {
        let (x, _) = get_sample_vectors(793);
        let max = unsafe { f32_xany_avx2_nofma_max_horizontal(&x) };
        assert_eq!(max, x.iter().fold(f32::NEG_INFINITY, |acc, v| acc.max(*v)));
    }

    #[test]
    fn test_xany_nofma_max_vertical() {
        let mut matrix = Vec::new();
        for _ in 0..25 {
            let (x, _) = get_sample_vectors(537);
            matrix.push(x);
        }

        let matrix_view = matrix.iter().map(|v| v.as_ref()).collect::<Vec<&[f32]>>();

        let mut expected_vertical_max = vec![f32::NEG_INFINITY; 537];
        for i in 0..537 {
            let mut max = f32::NEG_INFINITY;
            for arr in matrix.iter() {
                max = max.max(arr[i]);
            }
            expected_vertical_max[i] = max;
        }

        let max = unsafe { f32_xany_avx2_nofma_max_vertical(&matrix_view) };
        assert_eq!(max, expected_vertical_max);
    }
}
