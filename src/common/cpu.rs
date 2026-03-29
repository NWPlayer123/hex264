// =============== BEGIN cpu_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_cpu_name_t {
    pub name: *const ::core::ffi::c_char,
    pub flags: crate::stdlib::uint32_t,
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::stdlib::__cpu_mask;
pub use crate::stdlib::__sched_cpucount;
pub use crate::stdlib::cpu_set_t;
use crate::stdlib::memset;
use crate::stdlib::sched_getaffinity;
pub use crate::stdlib::uint32_t;

pub use crate::stdlib::__pid_t;
pub use crate::stdlib::__uint32_t;
pub use crate::x264_h::X264_CPU_AVX;
pub use crate::x264_h::X264_CPU_AVX2;
pub use crate::x264_h::X264_CPU_AVX512;
pub use crate::x264_h::X264_CPU_BMI1;
pub use crate::x264_h::X264_CPU_BMI2;
pub use crate::x264_h::X264_CPU_CACHELINE_32;
pub use crate::x264_h::X264_CPU_CACHELINE_64;
pub use crate::x264_h::X264_CPU_FMA3;
pub use crate::x264_h::X264_CPU_FMA4;
pub use crate::x264_h::X264_CPU_LZCNT;
pub use crate::x264_h::X264_CPU_MMX;
pub use crate::x264_h::X264_CPU_MMX2;
pub use crate::x264_h::X264_CPU_SLOW_ATOM;
pub use crate::x264_h::X264_CPU_SLOW_PALIGNR;
pub use crate::x264_h::X264_CPU_SLOW_PSHUFB;
pub use crate::x264_h::X264_CPU_SLOW_SHUFFLE;
pub use crate::x264_h::X264_CPU_SSE;
pub use crate::x264_h::X264_CPU_SSE2;
pub use crate::x264_h::X264_CPU_SSE2_IS_FAST;
pub use crate::x264_h::X264_CPU_SSE2_IS_SLOW;
pub use crate::x264_h::X264_CPU_SSE3;
pub use crate::x264_h::X264_CPU_SSE4;
pub use crate::x264_h::X264_CPU_SSE42;
pub use crate::x264_h::X264_CPU_SSSE3;
pub use crate::x264_h::X264_CPU_STACK_MOD4;
pub use crate::x264_h::X264_CPU_XOP;
#[no_mangle]

pub static mut x264_cpu_names: [crate::src::common::cpu::x264_cpu_name_t; 28] = [
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"MMX2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"MMXEXT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2Slow\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2_IS_SLOW as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE2Fast\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2_IS_FAST as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"LZCNT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_LZCNT as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSSE3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4.1\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SSE4.2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"XOP\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_XOP as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"FMA4\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_FMA4 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"FMA3\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_FMA3 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"BMI1\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_LZCNT as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI1 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"BMI2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_LZCNT as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI1 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI2 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX2\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_FMA3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_LZCNT as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI1 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX2 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"AVX512\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_MMX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_MMX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSSE3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE4 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_SSE42 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_FMA3 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_LZCNT as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI1 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_BMI2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX2 as crate::stdlib::uint32_t
            | crate::x264_h::X264_CPU_AVX512 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"Cache32\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_CACHELINE_32 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"Cache64\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_CACHELINE_64 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowAtom\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_ATOM as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowPshufb\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_PSHUFB as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowPalignr\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_PALIGNR as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"SlowShuffle\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_SLOW_SHUFFLE as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"UnalignedStack\0".as_ptr() as *const ::core::ffi::c_char,
        flags: crate::x264_h::X264_CPU_STACK_MOD4 as crate::stdlib::uint32_t,
    },
    crate::src::common::cpu::x264_cpu_name_t {
        name: b"\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as crate::stdlib::uint32_t,
    },
];
#[no_mangle]

pub unsafe extern "C" fn x264_cpu_detect() -> crate::stdlib::uint32_t {
    unsafe {
        return 0 as crate::stdlib::uint32_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_cpu_num_processors() -> ::core::ffi::c_int {
    unsafe {
        let mut p_aff: crate::stdlib::cpu_set_t = crate::stdlib::cpu_set_t { __bits: [0; 16] };
        crate::stdlib::memset(
            &raw mut p_aff as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>() as crate::__stddef_size_t_h::size_t,
        );
        if crate::stdlib::sched_getaffinity(
            0 as crate::stdlib::__pid_t,
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>() as crate::__stddef_size_t_h::size_t,
            &raw mut p_aff,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        return crate::stdlib::__sched_cpucount(
            ::core::mem::size_of::<crate::stdlib::cpu_set_t>() as crate::__stddef_size_t_h::size_t,
            &raw mut p_aff,
        );
    }
}
