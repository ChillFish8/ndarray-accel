//! Vector based min/max/sum/norm operations
//!
//! These exported methods are safe to call and select the fastest available instruction set
//! to use at runtime.
//!
//! Both `xconst` and `xany` variants of each operation are provided.
//!
//! The following arithmetic operations are provided:
//!
//! - The squared L2 norm of the vector
//! 
//! - Sum vector horizontally
//! - Min vector horizontally
//! - Max vector horizontally
//!
//! - Min vector vertically
//! - Max vector vertically
//!
use crate::danger::*;


macro_rules! export_safe_horizontal_op {
    (
        description = $desc:expr,
        ty = $t:ty,
        const_name = $const_name:ident,
        any_name = $any_name:ident,
        $avx512_const_name:ident,
        $avx2_const_name:ident,
        $neon_const_name:ident,
        $fallback_const_name:ident,
        $avx512_any_name:ident,
        $avx2_any_name:ident,
        $neon_any_name:ident,
        $fallback_any_name:ident,
    ) => {      
        #[doc = concat!("`", stringify!($t), "` ", $desc)]
        pub fn $const_name<const DIMS: usize>(a: &[$t]) -> $t {            
            unsafe {
                #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
                if std::arch::is_x86_feature_detected!("avx512f") {
                    return $avx512_const_name::<DIMS>(a);
                }
                        
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                if std::arch::is_x86_feature_detected!("avx2") {
                    return $avx2_const_name::<DIMS>(a);
                }
        
                #[cfg(target_arch = "aarch64")]
                if std::arch::is_aarch64_feature_detected!("neon") {
                    return $neon_const_name::<DIMS>(a);
                }
                
                $fallback_const_name::<DIMS>(a)
            }
        }
        
        #[doc = concat!("`", stringify!($t), "` ", $desc)]
        pub fn $any_name(a: &[$t]) -> $t{            
            unsafe {
                #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
                if std::arch::is_x86_feature_detected!("avx512f") {
                    return $avx512_any_name(a);
                }
                        
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                if std::arch::is_x86_feature_detected!("avx2") {
                    return $avx2_any_name(a);
                }
        
                #[cfg(target_arch = "aarch64")]
                if std::arch::is_aarch64_feature_detected!("neon") {
                    return $neon_any_name(a);
                }
                
                $fallback_any_name(a)
            }
        }
    };
}


macro_rules! export_safe_vertical_op {
    (
        description = $desc:expr,
        ty = $t:ty,
        const_name = $const_name:ident,
        any_name = $any_name:ident,
        $avx512_const_name:ident,
        $avx2_const_name:ident,
        $neon_const_name:ident,
        $fallback_const_name:ident,
        $avx512_any_name:ident,
        $avx2_any_name:ident,
        $neon_any_name:ident,
        $fallback_any_name:ident,
    ) => {      
        #[doc = concat!("`", stringify!($t), "` ", $desc)]
        pub fn $const_name<const DIMS: usize>(a: &[$t], b: &[$t], result: &mut [$t]) {
            assert_eq!(a.len(), b.len(), "Input vector a and b do not match in size");
            assert_eq!(a.len(), result.len(), "Input vectors and result vector size do not match");
            
            unsafe {
                #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
                if std::arch::is_x86_feature_detected!("avx512f") {
                    return $avx512_const_name::<DIMS>(a, b, result);
                }
                        
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                if std::arch::is_x86_feature_detected!("avx2") {
                    return $avx2_const_name::<DIMS>(a, b, result);
                }
        
                #[cfg(target_arch = "aarch64")]
                if std::arch::is_aarch64_feature_detected!("neon") {
                    return $neon_const_name::<DIMS>(a, b, result);
                }
                
                $fallback_const_name::<DIMS>(a, b, result)
            }
        }
        
        #[doc = concat!("`", stringify!($t), "` ", $desc)]
        pub fn $any_name(a: &[$t], b: &[$t], result: &mut [$t]) {
            assert_eq!(a.len(), b.len(), "Input vector a and b do not match in size");
            assert_eq!(a.len(), result.len(), "Input vectors and result vector size do not match");
            
            unsafe {
                #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
                if std::arch::is_x86_feature_detected!("avx512f") {
                    return $avx512_any_name(a, b, result);
                }
                        
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                if std::arch::is_x86_feature_detected!("avx2") {
                    return $avx2_any_name(a, b, result);
                }
        
                #[cfg(target_arch = "aarch64")]
                if std::arch::is_aarch64_feature_detected!("neon") {
                    return $neon_any_name(a, b, result);
                }
                
                $fallback_any_name(a, b, result)
            }
        }
    };
}

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = f32,
    const_name = f32_xconst_sum,
    any_name = f32_xany_sum,    
    f32_xconst_avx512_nofma_sum,
    f32_xconst_avx2_nofma_sum,
    f32_xconst_neon_nofma_sum,
    f32_xconst_fallback_nofma_sum,
    f32_xany_avx512_nofma_sum,
    f32_xany_avx2_nofma_sum,
    f32_xany_neon_nofma_sum,
    f32_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = f32,
    const_name = f32_xconst_max_horizontal,
    any_name = f32_xany_max_horizontal,    
    f32_xconst_avx512_nofma_max_horizontal,
    f32_xconst_avx2_nofma_max_horizontal,
    f32_xconst_neon_nofma_max_horizontal,
    f32_xconst_fallback_nofma_max_horizontal,
    f32_xany_avx512_nofma_max_horizontal,
    f32_xany_avx2_nofma_max_horizontal,
    f32_xany_neon_nofma_max_horizontal,
    f32_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = f32,
    const_name = f32_xconst_min_horizontal,
    any_name = f32_xany_min_horizontal,    
    f32_xconst_avx512_nofma_min_horizontal,
    f32_xconst_avx2_nofma_min_horizontal,
    f32_xconst_neon_nofma_min_horizontal,
    f32_xconst_fallback_nofma_min_horizontal,
    f32_xany_avx512_nofma_min_horizontal,
    f32_xany_avx2_nofma_min_horizontal,
    f32_xany_neon_nofma_min_horizontal,
    f32_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = f64,
    const_name = f64_xconst_sum,
    any_name = f64_xany_sum,    
    f64_xconst_avx512_nofma_sum,
    f64_xconst_avx2_nofma_sum,
    f64_xconst_neon_nofma_sum,
    f64_xconst_fallback_nofma_sum,
    f64_xany_avx512_nofma_sum,
    f64_xany_avx2_nofma_sum,
    f64_xany_neon_nofma_sum,
    f64_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = f64,
    const_name = f64_xconst_max_horizontal,
    any_name = f64_xany_max_horizontal,    
    f64_xconst_avx512_nofma_max_horizontal,
    f64_xconst_avx2_nofma_max_horizontal,
    f64_xconst_neon_nofma_max_horizontal,
    f64_xconst_fallback_nofma_max_horizontal,
    f64_xany_avx512_nofma_max_horizontal,
    f64_xany_avx2_nofma_max_horizontal,
    f64_xany_neon_nofma_max_horizontal,
    f64_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = f64,
    const_name = f64_xconst_min_horizontal,
    any_name = f64_xany_min_horizontal,    
    f64_xconst_avx512_nofma_min_horizontal,
    f64_xconst_avx2_nofma_min_horizontal,
    f64_xconst_neon_nofma_min_horizontal,
    f64_xconst_fallback_nofma_min_horizontal,
    f64_xany_avx512_nofma_min_horizontal,
    f64_xany_avx2_nofma_min_horizontal,
    f64_xany_neon_nofma_min_horizontal,
    f64_xany_fallback_nofma_min_horizontal,    
);


export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = u8,
    const_name = u8_xconst_sum,
    any_name = u8_xany_sum,    
    u8_xconst_avx512_nofma_sum,
    u8_xconst_avx2_nofma_sum,
    u8_xconst_neon_nofma_sum,
    u8_xconst_fallback_nofma_sum,
    u8_xany_avx512_nofma_sum,
    u8_xany_avx2_nofma_sum,
    u8_xany_neon_nofma_sum,
    u8_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = u8,
    const_name = u8_xconst_max_horizontal,
    any_name = u8_xany_max_horizontal,    
    u8_xconst_avx512_nofma_max_horizontal,
    u8_xconst_avx2_nofma_max_horizontal,
    u8_xconst_neon_nofma_max_horizontal,
    u8_xconst_fallback_nofma_max_horizontal,
    u8_xany_avx512_nofma_max_horizontal,
    u8_xany_avx2_nofma_max_horizontal,
    u8_xany_neon_nofma_max_horizontal,
    u8_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = u8,
    const_name = u8_xconst_min_horizontal,
    any_name = u8_xany_min_horizontal,    
    u8_xconst_avx512_nofma_min_horizontal,
    u8_xconst_avx2_nofma_min_horizontal,
    u8_xconst_neon_nofma_min_horizontal,
    u8_xconst_fallback_nofma_min_horizontal,
    u8_xany_avx512_nofma_min_horizontal,
    u8_xany_avx2_nofma_min_horizontal,
    u8_xany_neon_nofma_min_horizontal,
    u8_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = u16,
    const_name = u16_xconst_sum,
    any_name = u16_xany_sum,    
    u16_xconst_avx512_nofma_sum,
    u16_xconst_avx2_nofma_sum,
    u16_xconst_neon_nofma_sum,
    u16_xconst_fallback_nofma_sum,
    u16_xany_avx512_nofma_sum,
    u16_xany_avx2_nofma_sum,
    u16_xany_neon_nofma_sum,
    u16_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = u16,
    const_name = u16_xconst_max_horizontal,
    any_name = u16_xany_max_horizontal,    
    u16_xconst_avx512_nofma_max_horizontal,
    u16_xconst_avx2_nofma_max_horizontal,
    u16_xconst_neon_nofma_max_horizontal,
    u16_xconst_fallback_nofma_max_horizontal,
    u16_xany_avx512_nofma_max_horizontal,
    u16_xany_avx2_nofma_max_horizontal,
    u16_xany_neon_nofma_max_horizontal,
    u16_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = u16,
    const_name = u16_xconst_min_horizontal,
    any_name = u16_xany_min_horizontal,    
    u16_xconst_avx512_nofma_min_horizontal,
    u16_xconst_avx2_nofma_min_horizontal,
    u16_xconst_neon_nofma_min_horizontal,
    u16_xconst_fallback_nofma_min_horizontal,
    u16_xany_avx512_nofma_min_horizontal,
    u16_xany_avx2_nofma_min_horizontal,
    u16_xany_neon_nofma_min_horizontal,
    u16_xany_fallback_nofma_min_horizontal,    
);


export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = u32,
    const_name = u32_xconst_sum,
    any_name = u32_xany_sum,    
    u32_xconst_avx512_nofma_sum,
    u32_xconst_avx2_nofma_sum,
    u32_xconst_neon_nofma_sum,
    u32_xconst_fallback_nofma_sum,
    u32_xany_avx512_nofma_sum,
    u32_xany_avx2_nofma_sum,
    u32_xany_neon_nofma_sum,
    u32_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = u32,
    const_name = u32_xconst_max_horizontal,
    any_name = u32_xany_max_horizontal,    
    u32_xconst_avx512_nofma_max_horizontal,
    u32_xconst_avx2_nofma_max_horizontal,
    u32_xconst_neon_nofma_max_horizontal,
    u32_xconst_fallback_nofma_max_horizontal,
    u32_xany_avx512_nofma_max_horizontal,
    u32_xany_avx2_nofma_max_horizontal,
    u32_xany_neon_nofma_max_horizontal,
    u32_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = u32,
    const_name = u32_xconst_min_horizontal,
    any_name = u32_xany_min_horizontal,    
    u32_xconst_avx512_nofma_min_horizontal,
    u32_xconst_avx2_nofma_min_horizontal,
    u32_xconst_neon_nofma_min_horizontal,
    u32_xconst_fallback_nofma_min_horizontal,
    u32_xany_avx512_nofma_min_horizontal,
    u32_xany_avx2_nofma_min_horizontal,
    u32_xany_neon_nofma_min_horizontal,
    u32_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = u64,
    const_name = u64_xconst_sum,
    any_name = u64_xany_sum,    
    u64_xconst_avx512_nofma_sum,
    u64_xconst_avx2_nofma_sum,
    u64_xconst_neon_nofma_sum,
    u64_xconst_fallback_nofma_sum,
    u64_xany_avx512_nofma_sum,
    u64_xany_avx2_nofma_sum,
    u64_xany_neon_nofma_sum,
    u64_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = u64,
    const_name = u64_xconst_max_horizontal,
    any_name = u64_xany_max_horizontal,    
    u64_xconst_avx512_nofma_max_horizontal,
    u64_xconst_avx2_nofma_max_horizontal,
    u64_xconst_neon_nofma_max_horizontal,
    u64_xconst_fallback_nofma_max_horizontal,
    u64_xany_avx512_nofma_max_horizontal,
    u64_xany_avx2_nofma_max_horizontal,
    u64_xany_neon_nofma_max_horizontal,
    u64_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = u64,
    const_name = u64_xconst_min_horizontal,
    any_name = u64_xany_min_horizontal,    
    u64_xconst_avx512_nofma_min_horizontal,
    u64_xconst_avx2_nofma_min_horizontal,
    u64_xconst_neon_nofma_min_horizontal,
    u64_xconst_fallback_nofma_min_horizontal,
    u64_xany_avx512_nofma_min_horizontal,
    u64_xany_avx2_nofma_min_horizontal,
    u64_xany_neon_nofma_min_horizontal,
    u64_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = i8,
    const_name = i8_xconst_sum,
    any_name = i8_xany_sum,    
    i8_xconst_avx512_nofma_sum,
    i8_xconst_avx2_nofma_sum,
    i8_xconst_neon_nofma_sum,
    i8_xconst_fallback_nofma_sum,
    i8_xany_avx512_nofma_sum,
    i8_xany_avx2_nofma_sum,
    i8_xany_neon_nofma_sum,
    i8_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = i8,
    const_name = i8_xconst_max_horizontal,
    any_name = i8_xany_max_horizontal,    
    i8_xconst_avx512_nofma_max_horizontal,
    i8_xconst_avx2_nofma_max_horizontal,
    i8_xconst_neon_nofma_max_horizontal,
    i8_xconst_fallback_nofma_max_horizontal,
    i8_xany_avx512_nofma_max_horizontal,
    i8_xany_avx2_nofma_max_horizontal,
    i8_xany_neon_nofma_max_horizontal,
    i8_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = i8,
    const_name = i8_xconst_min_horizontal,
    any_name = i8_xany_min_horizontal,    
    i8_xconst_avx512_nofma_min_horizontal,
    i8_xconst_avx2_nofma_min_horizontal,
    i8_xconst_neon_nofma_min_horizontal,
    i8_xconst_fallback_nofma_min_horizontal,
    i8_xany_avx512_nofma_min_horizontal,
    i8_xany_avx2_nofma_min_horizontal,
    i8_xany_neon_nofma_min_horizontal,
    i8_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = i16,
    const_name = i16_xconst_sum,
    any_name = i16_xany_sum,    
    i16_xconst_avx512_nofma_sum,
    i16_xconst_avx2_nofma_sum,
    i16_xconst_neon_nofma_sum,
    i16_xconst_fallback_nofma_sum,
    i16_xany_avx512_nofma_sum,
    i16_xany_avx2_nofma_sum,
    i16_xany_neon_nofma_sum,
    i16_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = i16,
    const_name = i16_xconst_max_horizontal,
    any_name = i16_xany_max_horizontal,    
    i16_xconst_avx512_nofma_max_horizontal,
    i16_xconst_avx2_nofma_max_horizontal,
    i16_xconst_neon_nofma_max_horizontal,
    i16_xconst_fallback_nofma_max_horizontal,
    i16_xany_avx512_nofma_max_horizontal,
    i16_xany_avx2_nofma_max_horizontal,
    i16_xany_neon_nofma_max_horizontal,
    i16_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = i16,
    const_name = i16_xconst_min_horizontal,
    any_name = i16_xany_min_horizontal,    
    i16_xconst_avx512_nofma_min_horizontal,
    i16_xconst_avx2_nofma_min_horizontal,
    i16_xconst_neon_nofma_min_horizontal,
    i16_xconst_fallback_nofma_min_horizontal,
    i16_xany_avx512_nofma_min_horizontal,
    i16_xany_avx2_nofma_min_horizontal,
    i16_xany_neon_nofma_min_horizontal,
    i16_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = i32,
    const_name = i32_xconst_sum,
    any_name = i32_xany_sum,    
    i32_xconst_avx512_nofma_sum,
    i32_xconst_avx2_nofma_sum,
    i32_xconst_neon_nofma_sum,
    i32_xconst_fallback_nofma_sum,
    i32_xany_avx512_nofma_sum,
    i32_xany_avx2_nofma_sum,
    i32_xany_neon_nofma_sum,
    i32_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = i32,
    const_name = i32_xconst_max_horizontal,
    any_name = i32_xany_max_horizontal,    
    i32_xconst_avx512_nofma_max_horizontal,
    i32_xconst_avx2_nofma_max_horizontal,
    i32_xconst_neon_nofma_max_horizontal,
    i32_xconst_fallback_nofma_max_horizontal,
    i32_xany_avx512_nofma_max_horizontal,
    i32_xany_avx2_nofma_max_horizontal,
    i32_xany_neon_nofma_max_horizontal,
    i32_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = i32,
    const_name = i32_xconst_min_horizontal,
    any_name = i32_xany_min_horizontal,    
    i32_xconst_avx512_nofma_min_horizontal,
    i32_xconst_avx2_nofma_min_horizontal,
    i32_xconst_neon_nofma_min_horizontal,
    i32_xconst_fallback_nofma_min_horizontal,
    i32_xany_avx512_nofma_min_horizontal,
    i32_xany_avx2_nofma_min_horizontal,
    i32_xany_neon_nofma_min_horizontal,
    i32_xany_fallback_nofma_min_horizontal,    
);

export_safe_horizontal_op!(
    description = "Performs a horizontal sum of all elements in vector `a`",
    ty = i64,
    const_name = i64_xconst_sum,
    any_name = i64_xany_sum,    
    i64_xconst_avx512_nofma_sum,
    i64_xconst_avx2_nofma_sum,
    i64_xconst_neon_nofma_sum,
    i64_xconst_fallback_nofma_sum,
    i64_xany_avx512_nofma_sum,
    i64_xany_avx2_nofma_sum,
    i64_xany_neon_nofma_sum,
    i64_xany_fallback_nofma_sum,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal max of all elements in vector `a`",
    ty = i64,
    const_name = i64_xconst_max_horizontal,
    any_name = i64_xany_max_horizontal,    
    i64_xconst_avx512_nofma_max_horizontal,
    i64_xconst_avx2_nofma_max_horizontal,
    i64_xconst_neon_nofma_max_horizontal,
    i64_xconst_fallback_nofma_max_horizontal,
    i64_xany_avx512_nofma_max_horizontal,
    i64_xany_avx2_nofma_max_horizontal,
    i64_xany_neon_nofma_max_horizontal,
    i64_xany_fallback_nofma_max_horizontal,    
);
export_safe_horizontal_op!(
    description = "Performs a horizontal min of all elements in vector `a`",
    ty = i64,
    const_name = i64_xconst_min_horizontal,
    any_name = i64_xany_min_horizontal,    
    i64_xconst_avx512_nofma_min_horizontal,
    i64_xconst_avx2_nofma_min_horizontal,
    i64_xconst_neon_nofma_min_horizontal,
    i64_xconst_fallback_nofma_min_horizontal,
    i64_xany_avx512_nofma_min_horizontal,
    i64_xany_avx2_nofma_min_horizontal,
    i64_xany_neon_nofma_min_horizontal,
    i64_xany_fallback_nofma_min_horizontal,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = f32,
    const_name = f32_xconst_max_vertical,
    any_name = f32_xany_max_vertical,    
    f32_xconst_avx512_nofma_max_vertical,
    f32_xconst_avx2_nofma_max_vertical,
    f32_xconst_neon_nofma_max_vertical,
    f32_xconst_fallback_nofma_max_vertical,
    f32_xany_avx512_nofma_max_vertical,
    f32_xany_avx2_nofma_max_vertical,
    f32_xany_neon_nofma_max_vertical,
    f32_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = f32,
    const_name = f32_xconst_min_vertical,
    any_name = f32_xany_min_vertical,    
    f32_xconst_avx512_nofma_min_vertical,
    f32_xconst_avx2_nofma_min_vertical,
    f32_xconst_neon_nofma_min_vertical,
    f32_xconst_fallback_nofma_min_vertical,
    f32_xany_avx512_nofma_min_vertical,
    f32_xany_avx2_nofma_min_vertical,
    f32_xany_neon_nofma_min_vertical,
    f32_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = f64,
    const_name = f64_xconst_max_vertical,
    any_name = f64_xany_max_vertical,    
    f64_xconst_avx512_nofma_max_vertical,
    f64_xconst_avx2_nofma_max_vertical,
    f64_xconst_neon_nofma_max_vertical,
    f64_xconst_fallback_nofma_max_vertical,
    f64_xany_avx512_nofma_max_vertical,
    f64_xany_avx2_nofma_max_vertical,
    f64_xany_neon_nofma_max_vertical,
    f64_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = f64,
    const_name = f64_xconst_min_vertical,
    any_name = f64_xany_min_vertical,    
    f64_xconst_avx512_nofma_min_vertical,
    f64_xconst_avx2_nofma_min_vertical,
    f64_xconst_neon_nofma_min_vertical,
    f64_xconst_fallback_nofma_min_vertical,
    f64_xany_avx512_nofma_min_vertical,
    f64_xany_avx2_nofma_min_vertical,
    f64_xany_neon_nofma_min_vertical,
    f64_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = u8,
    const_name = u8_xconst_max_vertical,
    any_name = u8_xany_max_vertical,    
    u8_xconst_avx512_nofma_max_vertical,
    u8_xconst_avx2_nofma_max_vertical,
    u8_xconst_neon_nofma_max_vertical,
    u8_xconst_fallback_nofma_max_vertical,
    u8_xany_avx512_nofma_max_vertical,
    u8_xany_avx2_nofma_max_vertical,
    u8_xany_neon_nofma_max_vertical,
    u8_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = u8,
    const_name = u8_xconst_min_vertical,
    any_name = u8_xany_min_vertical,    
    u8_xconst_avx512_nofma_min_vertical,
    u8_xconst_avx2_nofma_min_vertical,
    u8_xconst_neon_nofma_min_vertical,
    u8_xconst_fallback_nofma_min_vertical,
    u8_xany_avx512_nofma_min_vertical,
    u8_xany_avx2_nofma_min_vertical,
    u8_xany_neon_nofma_min_vertical,
    u8_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = u16,
    const_name = u16_xconst_max_vertical,
    any_name = u16_xany_max_vertical,    
    u16_xconst_avx512_nofma_max_vertical,
    u16_xconst_avx2_nofma_max_vertical,
    u16_xconst_neon_nofma_max_vertical,
    u16_xconst_fallback_nofma_max_vertical,
    u16_xany_avx512_nofma_max_vertical,
    u16_xany_avx2_nofma_max_vertical,
    u16_xany_neon_nofma_max_vertical,
    u16_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = u16,
    const_name = u16_xconst_min_vertical,
    any_name = u16_xany_min_vertical,    
    u16_xconst_avx512_nofma_min_vertical,
    u16_xconst_avx2_nofma_min_vertical,
    u16_xconst_neon_nofma_min_vertical,
    u16_xconst_fallback_nofma_min_vertical,
    u16_xany_avx512_nofma_min_vertical,
    u16_xany_avx2_nofma_min_vertical,
    u16_xany_neon_nofma_min_vertical,
    u16_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = u32,
    const_name = u32_xconst_max_vertical,
    any_name = u32_xany_max_vertical,    
    u32_xconst_avx512_nofma_max_vertical,
    u32_xconst_avx2_nofma_max_vertical,
    u32_xconst_neon_nofma_max_vertical,
    u32_xconst_fallback_nofma_max_vertical,
    u32_xany_avx512_nofma_max_vertical,
    u32_xany_avx2_nofma_max_vertical,
    u32_xany_neon_nofma_max_vertical,
    u32_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = u32,
    const_name = u32_xconst_min_vertical,
    any_name = u32_xany_min_vertical,    
    u32_xconst_avx512_nofma_min_vertical,
    u32_xconst_avx2_nofma_min_vertical,
    u32_xconst_neon_nofma_min_vertical,
    u32_xconst_fallback_nofma_min_vertical,
    u32_xany_avx512_nofma_min_vertical,
    u32_xany_avx2_nofma_min_vertical,
    u32_xany_neon_nofma_min_vertical,
    u32_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = u64,
    const_name = u64_xconst_max_vertical,
    any_name = u64_xany_max_vertical,    
    u64_xconst_avx512_nofma_max_vertical,
    u64_xconst_avx2_nofma_max_vertical,
    u64_xconst_neon_nofma_max_vertical,
    u64_xconst_fallback_nofma_max_vertical,
    u64_xany_avx512_nofma_max_vertical,
    u64_xany_avx2_nofma_max_vertical,
    u64_xany_neon_nofma_max_vertical,
    u64_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = u64,
    const_name = u64_xconst_min_vertical,
    any_name = u64_xany_min_vertical,    
    u64_xconst_avx512_nofma_min_vertical,
    u64_xconst_avx2_nofma_min_vertical,
    u64_xconst_neon_nofma_min_vertical,
    u64_xconst_fallback_nofma_min_vertical,
    u64_xany_avx512_nofma_min_vertical,
    u64_xany_avx2_nofma_min_vertical,
    u64_xany_neon_nofma_min_vertical,
    u64_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = i8,
    const_name = i8_xconst_max_vertical,
    any_name = i8_xany_max_vertical,    
    i8_xconst_avx512_nofma_max_vertical,
    i8_xconst_avx2_nofma_max_vertical,
    i8_xconst_neon_nofma_max_vertical,
    i8_xconst_fallback_nofma_max_vertical,
    i8_xany_avx512_nofma_max_vertical,
    i8_xany_avx2_nofma_max_vertical,
    i8_xany_neon_nofma_max_vertical,
    i8_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = i8,
    const_name = i8_xconst_min_vertical,
    any_name = i8_xany_min_vertical,    
    i8_xconst_avx512_nofma_min_vertical,
    i8_xconst_avx2_nofma_min_vertical,
    i8_xconst_neon_nofma_min_vertical,
    i8_xconst_fallback_nofma_min_vertical,
    i8_xany_avx512_nofma_min_vertical,
    i8_xany_avx2_nofma_min_vertical,
    i8_xany_neon_nofma_min_vertical,
    i8_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = i16,
    const_name = i16_xconst_max_vertical,
    any_name = i16_xany_max_vertical,    
    i16_xconst_avx512_nofma_max_vertical,
    i16_xconst_avx2_nofma_max_vertical,
    i16_xconst_neon_nofma_max_vertical,
    i16_xconst_fallback_nofma_max_vertical,
    i16_xany_avx512_nofma_max_vertical,
    i16_xany_avx2_nofma_max_vertical,
    i16_xany_neon_nofma_max_vertical,
    i16_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = i16,
    const_name = i16_xconst_min_vertical,
    any_name = i16_xany_min_vertical,    
    i16_xconst_avx512_nofma_min_vertical,
    i16_xconst_avx2_nofma_min_vertical,
    i16_xconst_neon_nofma_min_vertical,
    i16_xconst_fallback_nofma_min_vertical,
    i16_xany_avx512_nofma_min_vertical,
    i16_xany_avx2_nofma_min_vertical,
    i16_xany_neon_nofma_min_vertical,
    i16_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = i32,
    const_name = i32_xconst_max_vertical,
    any_name = i32_xany_max_vertical,    
    i32_xconst_avx512_nofma_max_vertical,
    i32_xconst_avx2_nofma_max_vertical,
    i32_xconst_neon_nofma_max_vertical,
    i32_xconst_fallback_nofma_max_vertical,
    i32_xany_avx512_nofma_max_vertical,
    i32_xany_avx2_nofma_max_vertical,
    i32_xany_neon_nofma_max_vertical,
    i32_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = i32,
    const_name = i32_xconst_min_vertical,
    any_name = i32_xany_min_vertical,    
    i32_xconst_avx512_nofma_min_vertical,
    i32_xconst_avx2_nofma_min_vertical,
    i32_xconst_neon_nofma_min_vertical,
    i32_xconst_fallback_nofma_min_vertical,
    i32_xany_avx512_nofma_min_vertical,
    i32_xany_avx2_nofma_min_vertical,
    i32_xany_neon_nofma_min_vertical,
    i32_xany_fallback_nofma_min_vertical,    
);

export_safe_vertical_op!(
    description = "Performs a vertical max of each respective element of vectors `a` and `b`",
    ty = i64,
    const_name = i64_xconst_max_vertical,
    any_name = i64_xany_max_vertical,    
    i64_xconst_avx512_nofma_max_vertical,
    i64_xconst_avx2_nofma_max_vertical,
    i64_xconst_neon_nofma_max_vertical,
    i64_xconst_fallback_nofma_max_vertical,
    i64_xany_avx512_nofma_max_vertical,
    i64_xany_avx2_nofma_max_vertical,
    i64_xany_neon_nofma_max_vertical,
    i64_xany_fallback_nofma_max_vertical,    
);
export_safe_vertical_op!(
    description = "Performs a vertical min of each respective element in vectors `a` and `b`",
    ty = i64,
    const_name = i64_xconst_min_vertical,
    any_name = i64_xany_min_vertical,    
    i64_xconst_avx512_nofma_min_vertical,
    i64_xconst_avx2_nofma_min_vertical,
    i64_xconst_neon_nofma_min_vertical,
    i64_xconst_fallback_nofma_min_vertical,
    i64_xany_avx512_nofma_min_vertical,
    i64_xany_avx2_nofma_min_vertical,
    i64_xany_neon_nofma_min_vertical,
    i64_xany_fallback_nofma_min_vertical,    
);