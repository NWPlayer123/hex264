// =============== BEGIN cpu_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_cpu_name_t {
    pub name: *const ::core::ffi::c_char,
    pub flags: crate::stdlib::uint32_t,
}
#[no_mangle]
pub static mut x264_cpu_names: [crate::src::common::cpu::x264_cpu_name_t; 28] = [
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"MMX2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX | crate::x264_h::X264_CPU_MMX2,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"MMXEXT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX | crate::x264_h::X264_CPU_MMX2,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2Slow\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE2_IS_SLOW,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2Fast\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE2_IS_FAST,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"LZCNT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_LZCNT,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSSE3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4.1\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4.2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"XOP\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_XOP,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"FMA4\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_FMA4,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"FMA3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_FMA3,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"BMI1\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_LZCNT
            | crate::x264_h::X264_CPU_BMI1,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"BMI2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_LZCNT
            | crate::x264_h::X264_CPU_BMI1
            | crate::x264_h::X264_CPU_BMI2,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_FMA3
            | crate::x264_h::X264_CPU_LZCNT
            | crate::x264_h::X264_CPU_BMI1
            | crate::x264_h::X264_CPU_BMI2
            | crate::x264_h::X264_CPU_AVX2,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX512\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX
            | crate::x264_h::X264_CPU_MMX2
            | crate::x264_h::X264_CPU_SSE
            | crate::x264_h::X264_CPU_SSE2
            | crate::x264_h::X264_CPU_SSE3
            | crate::x264_h::X264_CPU_SSSE3
            | crate::x264_h::X264_CPU_SSE4
            | crate::x264_h::X264_CPU_SSE42
            | crate::x264_h::X264_CPU_AVX
            | crate::x264_h::X264_CPU_FMA3
            | crate::x264_h::X264_CPU_LZCNT
            | crate::x264_h::X264_CPU_BMI1
            | crate::x264_h::X264_CPU_BMI2
            | crate::x264_h::X264_CPU_AVX2
            | crate::x264_h::X264_CPU_AVX512,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"Cache32\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_CACHELINE_32,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"Cache64\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_CACHELINE_64,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowAtom\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_ATOM,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowPshufb\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_PSHUFB,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowPalignr\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_PALIGNR,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowShuffle\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_SHUFFLE,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"UnalignedStack\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_STACK_MOD4,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0u32,
    },
];
#[no_mangle]
pub extern "C" fn x264_cpu_detect() -> crate::stdlib::uint32_t {
    return 0u32;
}
#[no_mangle]
pub unsafe extern "C" fn x264_cpu_num_processors() -> ::core::ffi::c_int {
    unsafe {
        let mut p_aff: crate::stdlib::cpu_set_t = crate::stdlib::cpu_set_t { __bits: [0; 16] };
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
