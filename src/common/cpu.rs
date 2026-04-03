use crate::x264_h::{
    X264_CPU_AVX, X264_CPU_AVX2, X264_CPU_AVX512, X264_CPU_BMI1, X264_CPU_BMI2,
    X264_CPU_CACHELINE_32, X264_CPU_CACHELINE_64, X264_CPU_FMA3, X264_CPU_FMA4, X264_CPU_LZCNT,
    X264_CPU_MMX, X264_CPU_MMX2, X264_CPU_SLOW_ATOM, X264_CPU_SLOW_PALIGNR, X264_CPU_SLOW_PSHUFB,
    X264_CPU_SLOW_SHUFFLE, X264_CPU_SSE, X264_CPU_SSE2, X264_CPU_SSE2_IS_FAST,
    X264_CPU_SSE2_IS_SLOW, X264_CPU_SSE3, X264_CPU_SSE4, X264_CPU_SSE42, X264_CPU_SSSE3,
    X264_CPU_STACK_MOD4, X264_CPU_XOP,
};
pub const X264_CPU_NAMES: &[(&core::ffi::CStr, u32)] = &[
    (c"MMX2", X264_CPU_MMX | X264_CPU_MMX2),
    (c"MMXEXT", X264_CPU_MMX | X264_CPU_MMX2),
    (c"SSE", X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE),
    (
        c"SSE2Slow",
        X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE | X264_CPU_SSE2 | X264_CPU_SSE2_IS_SLOW,
    ),
    (
        c"SSE2",
        X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE | X264_CPU_SSE2,
    ),
    (
        c"SSE2Fast",
        X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE | X264_CPU_SSE2 | X264_CPU_SSE2_IS_FAST,
    ),
    (
        c"LZCNT",
        X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE | X264_CPU_SSE2 | X264_CPU_LZCNT,
    ),
    (
        c"SSE3",
        X264_CPU_MMX | X264_CPU_MMX2 | X264_CPU_SSE | X264_CPU_SSE2 | X264_CPU_SSE3,
    ),
    (
        c"SSSE3",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3,
    ),
    (
        c"SSE4.1",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4,
    ),
    (
        c"SSE4",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4,
    ),
    (
        c"SSE4.2",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42,
    ),
    (
        c"AVX",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX,
    ),
    (
        c"XOP",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_XOP,
    ),
    (
        c"FMA4",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_FMA4,
    ),
    (
        c"FMA3",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_FMA3,
    ),
    (
        c"BMI1",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_LZCNT
            | X264_CPU_BMI1,
    ),
    (
        c"BMI2",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_LZCNT
            | X264_CPU_BMI1
            | X264_CPU_BMI2,
    ),
    (
        c"AVX2",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_FMA3
            | X264_CPU_LZCNT
            | X264_CPU_BMI1
            | X264_CPU_BMI2
            | X264_CPU_AVX2,
    ),
    (
        c"AVX512",
        X264_CPU_MMX
            | X264_CPU_MMX2
            | X264_CPU_SSE
            | X264_CPU_SSE2
            | X264_CPU_SSE3
            | X264_CPU_SSSE3
            | X264_CPU_SSE4
            | X264_CPU_SSE42
            | X264_CPU_AVX
            | X264_CPU_FMA3
            | X264_CPU_LZCNT
            | X264_CPU_BMI1
            | X264_CPU_BMI2
            | X264_CPU_AVX2
            | X264_CPU_AVX512,
    ),
    (c"Cache32", X264_CPU_CACHELINE_32),
    (c"Cache64", X264_CPU_CACHELINE_64),
    (c"SlowAtom", X264_CPU_SLOW_ATOM),
    (c"SlowPshufb", X264_CPU_SLOW_PSHUFB),
    (c"SlowPalignr", X264_CPU_SLOW_PALIGNR),
    (c"SlowShuffle", X264_CPU_SLOW_SHUFFLE),
    (c"UnalignedStack", X264_CPU_STACK_MOD4),
];
pub extern "C" fn x264_cpu_detect() -> crate::stdlib::uint32_t {
    return 0u32;
}
pub unsafe extern "C" fn x264_cpu_num_processors() -> ::core::ffi::c_int {
    unsafe {
        let mut p_aff = crate::stdlib::cpu_set_t { __bits: [0; 16] };
        crate::stdlib::memset(
            &raw mut p_aff as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>(),
        );
        if crate::stdlib::sched_getaffinity(
            0i32,
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>(),
            &raw mut p_aff,
        ) != 0
        {
            return 1i32;
        }
        return crate::stdlib::__sched_cpucount(
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>(),
            &raw mut p_aff,
        );
    }
}
