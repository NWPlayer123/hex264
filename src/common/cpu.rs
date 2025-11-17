use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::cpu_h::x264_cpu_name_t;
use crate::cpu_set_h::{__sched_cpucount, cpu_set_t};
use crate::sched_h::sched_getaffinity;
use crate::stdint_uintn_h::uint32_t;
use crate::string_h::memset;
use crate::types_h::__pid_t;
use crate::x264_h::{
    X264_CPU_AVX, X264_CPU_AVX2, X264_CPU_AVX512, X264_CPU_BMI1, X264_CPU_BMI2,
    X264_CPU_CACHELINE_32, X264_CPU_CACHELINE_64, X264_CPU_FMA3, X264_CPU_FMA4, X264_CPU_LZCNT,
    X264_CPU_MMX, X264_CPU_MMX2, X264_CPU_SLOW_ATOM, X264_CPU_SLOW_PALIGNR, X264_CPU_SLOW_PSHUFB,
    X264_CPU_SLOW_SHUFFLE, X264_CPU_SSE, X264_CPU_SSE2, X264_CPU_SSE2_IS_FAST,
    X264_CPU_SSE2_IS_SLOW, X264_CPU_SSE3, X264_CPU_SSE4, X264_CPU_SSE42, X264_CPU_SSSE3,
    X264_CPU_STACK_MOD4, X264_CPU_XOP,
};
#[no_mangle]
#[c2rust::src_loc = "50:23"]
static mut x264_cpu_names: [x264_cpu_name_t; 28] = [
    {
        let mut init = x264_cpu_name_t {
            name: b"MMX2\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"MMXEXT\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t | X264_CPU_SSE as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2Slow\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE2_IS_SLOW as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2Fast\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE2_IS_FAST as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"LZCNT\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_LZCNT as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE3\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSSE3\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4.1\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4.2\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"XOP\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_XOP as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"FMA4\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_FMA4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"FMA3\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_FMA3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"BMI1\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"BMI2\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t
                | X264_CPU_BMI2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX2\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_FMA3 as uint32_t
                | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t
                | X264_CPU_BMI2 as uint32_t
                | X264_CPU_AVX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX512\0" as *const u8 as *const c_char,
            flags: X264_CPU_MMX as uint32_t
                | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t
                | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t
                | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t
                | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t
                | X264_CPU_FMA3 as uint32_t
                | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t
                | X264_CPU_BMI2 as uint32_t
                | X264_CPU_AVX2 as uint32_t
                | X264_CPU_AVX512 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"Cache32\0" as *const u8 as *const c_char,
            flags: X264_CPU_CACHELINE_32 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"Cache64\0" as *const u8 as *const c_char,
            flags: X264_CPU_CACHELINE_64 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowAtom\0" as *const u8 as *const c_char,
            flags: X264_CPU_SLOW_ATOM as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowPshufb\0" as *const u8 as *const c_char,
            flags: X264_CPU_SLOW_PSHUFB as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowPalignr\0" as *const u8 as *const c_char,
            flags: X264_CPU_SLOW_PALIGNR as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowShuffle\0" as *const u8 as *const c_char,
            flags: X264_CPU_SLOW_SHUFFLE as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"UnalignedStack\0" as *const u8 as *const c_char,
            flags: X264_CPU_STACK_MOD4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"\0" as *const u8 as *const c_char,
            flags: 0 as uint32_t,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "627:1"]
unsafe extern "C" fn x264_cpu_detect() -> uint32_t {
    return 0 as uint32_t;
}
#[no_mangle]
#[c2rust::src_loc = "634:1"]
unsafe extern "C" fn x264_cpu_num_processors() -> c_int {
    let mut p_aff: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    memset(
        &mut p_aff as *mut cpu_set_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
    );
    if sched_getaffinity(
        0 as __pid_t,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &mut p_aff,
    ) != 0
    {
        return 1 as c_int;
    }
    return __sched_cpucount(::core::mem::size_of::<cpu_set_t>() as size_t, &mut p_aff);
}
