#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:28"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/bits/cpu-set.h:28"]
pub mod cpu_set_h {
    #[c2rust::src_loc = "32:1"]
    pub type __cpu_mask = ::core::ffi::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct cpu_set_t {
        pub __bits: [__cpu_mask; 16],
    }
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "117:1"]
        pub fn __sched_cpucount(
            __setsize: size_t,
            __setp: *const cpu_set_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cpu.h:28"]
pub mod cpu_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct x264_cpu_name_t {
        pub name: *const ::core::ffi::c_char,
        pub flags: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/usr/include/sched.h:28"]
pub mod sched_h {
    use super::types_h::__pid_t;
    use super::__stddef_size_t_h::size_t;
    use super::cpu_set_h::cpu_set_t;
    extern "C" {
        #[c2rust::src_loc = "134:1"]
        pub fn sched_getaffinity(
            __pid: __pid_t,
            __cpusetsize: size_t,
            __cpuset: *mut cpu_set_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:28"]
pub mod x264_h {
    #[c2rust::src_loc = "139:9"]
    pub const X264_CPU_MMX: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "140:9"]
    pub const X264_CPU_MMX2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "142:9"]
    pub const X264_CPU_SSE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "143:9"]
    pub const X264_CPU_SSE2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "144:9"]
    pub const X264_CPU_LZCNT: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "145:9"]
    pub const X264_CPU_SSE3: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const X264_CPU_SSSE3: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "147:9"]
    pub const X264_CPU_SSE4: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 7 as ::core::ffi::c_int;
    #[c2rust::src_loc = "148:9"]
    pub const X264_CPU_SSE42: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "149:9"]
    pub const X264_CPU_AVX: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 9 as ::core::ffi::c_int;
    #[c2rust::src_loc = "150:9"]
    pub const X264_CPU_XOP: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "151:9"]
    pub const X264_CPU_FMA4: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "152:9"]
    pub const X264_CPU_FMA3: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 12 as ::core::ffi::c_int;
    #[c2rust::src_loc = "153:9"]
    pub const X264_CPU_BMI1: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 13 as ::core::ffi::c_int;
    #[c2rust::src_loc = "154:9"]
    pub const X264_CPU_BMI2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 14 as ::core::ffi::c_int;
    #[c2rust::src_loc = "155:9"]
    pub const X264_CPU_AVX2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 15 as ::core::ffi::c_int;
    #[c2rust::src_loc = "156:9"]
    pub const X264_CPU_AVX512: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "158:9"]
    pub const X264_CPU_CACHELINE_32: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 17 as ::core::ffi::c_int;
    #[c2rust::src_loc = "159:9"]
    pub const X264_CPU_CACHELINE_64: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 18 as ::core::ffi::c_int;
    #[c2rust::src_loc = "160:9"]
    pub const X264_CPU_SSE2_IS_SLOW: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 19 as ::core::ffi::c_int;
    #[c2rust::src_loc = "161:9"]
    pub const X264_CPU_SSE2_IS_FAST: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 20 as ::core::ffi::c_int;
    #[c2rust::src_loc = "162:9"]
    pub const X264_CPU_SLOW_SHUFFLE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 21 as ::core::ffi::c_int;
    #[c2rust::src_loc = "163:9"]
    pub const X264_CPU_STACK_MOD4: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 22 as ::core::ffi::c_int;
    #[c2rust::src_loc = "164:9"]
    pub const X264_CPU_SLOW_ATOM: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 23 as ::core::ffi::c_int;
    #[c2rust::src_loc = "169:9"]
    pub const X264_CPU_SLOW_PSHUFB: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 24 as ::core::ffi::c_int;
    #[c2rust::src_loc = "170:9"]
    pub const X264_CPU_SLOW_PALIGNR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 25 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__uint32_t, __pid_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::cpu_set_h::{__cpu_mask, cpu_set_t, __sched_cpucount};
pub use self::cpu_h::x264_cpu_name_t;
use self::sched_h::sched_getaffinity;
use self::string_h::memset;
pub use self::x264_h::{
    X264_CPU_MMX, X264_CPU_MMX2, X264_CPU_SSE, X264_CPU_SSE2, X264_CPU_LZCNT,
    X264_CPU_SSE3, X264_CPU_SSSE3, X264_CPU_SSE4, X264_CPU_SSE42, X264_CPU_AVX,
    X264_CPU_XOP, X264_CPU_FMA4, X264_CPU_FMA3, X264_CPU_BMI1, X264_CPU_BMI2,
    X264_CPU_AVX2, X264_CPU_AVX512, X264_CPU_CACHELINE_32, X264_CPU_CACHELINE_64,
    X264_CPU_SSE2_IS_SLOW, X264_CPU_SSE2_IS_FAST, X264_CPU_SLOW_SHUFFLE,
    X264_CPU_STACK_MOD4, X264_CPU_SLOW_ATOM, X264_CPU_SLOW_PSHUFB, X264_CPU_SLOW_PALIGNR,
};
#[no_mangle]
#[c2rust::src_loc = "50:23"]
pub static mut x264_cpu_names: [x264_cpu_name_t; 28] = [
    {
        let mut init = x264_cpu_name_t {
            name: b"MMX2\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"MMXEXT\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2Slow\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE2_IS_SLOW as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE2Fast\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE2_IS_FAST as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"LZCNT\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_LZCNT as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE3\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSSE3\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4.1\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SSE4.2\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"XOP\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_XOP as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"FMA4\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_FMA4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"FMA3\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_FMA3 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"BMI1\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"BMI2\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_LZCNT as uint32_t
                | X264_CPU_BMI1 as uint32_t | X264_CPU_BMI2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX2\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_FMA3 as uint32_t
                | X264_CPU_LZCNT as uint32_t | X264_CPU_BMI1 as uint32_t
                | X264_CPU_BMI2 as uint32_t | X264_CPU_AVX2 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"AVX512\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_MMX as uint32_t | X264_CPU_MMX2 as uint32_t
                | X264_CPU_SSE as uint32_t | X264_CPU_SSE2 as uint32_t
                | X264_CPU_SSE3 as uint32_t | X264_CPU_SSSE3 as uint32_t
                | X264_CPU_SSE4 as uint32_t | X264_CPU_SSE42 as uint32_t
                | X264_CPU_AVX as uint32_t | X264_CPU_FMA3 as uint32_t
                | X264_CPU_LZCNT as uint32_t | X264_CPU_BMI1 as uint32_t
                | X264_CPU_BMI2 as uint32_t | X264_CPU_AVX2 as uint32_t
                | X264_CPU_AVX512 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"Cache32\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_CACHELINE_32 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"Cache64\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_CACHELINE_64 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowAtom\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_SLOW_ATOM as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowPshufb\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_SLOW_PSHUFB as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowPalignr\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_SLOW_PALIGNR as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"SlowShuffle\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_SLOW_SHUFFLE as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"UnalignedStack\0" as *const u8 as *const ::core::ffi::c_char,
            flags: X264_CPU_STACK_MOD4 as uint32_t,
        };
        init
    },
    {
        let mut init = x264_cpu_name_t {
            name: b"\0" as *const u8 as *const ::core::ffi::c_char,
            flags: 0 as uint32_t,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "627:1"]
pub unsafe extern "C" fn x264_cpu_detect() -> uint32_t {
    return 0 as uint32_t;
}
#[no_mangle]
#[c2rust::src_loc = "634:1"]
pub unsafe extern "C" fn x264_cpu_num_processors() -> ::core::ffi::c_int {
    let mut p_aff: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    memset(
        &mut p_aff as *mut cpu_set_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
    );
    if sched_getaffinity(
        0 as __pid_t,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &mut p_aff,
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return __sched_cpucount(::core::mem::size_of::<cpu_set_t>() as size_t, &mut p_aff);
}
