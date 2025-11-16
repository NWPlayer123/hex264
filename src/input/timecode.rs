use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "175:1"]
    pub type __blksize_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "197:1"]
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:8"]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:26"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:26"]
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:26"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    #[inline]
    #[c2rust::src_loc = "50:1"]
    pub unsafe extern "C" fn gcd(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
        loop {
            let mut c: int64_t = a.wrapping_rem(b) as int64_t;
            if c == 0 {
                return b;
            }
            a = b;
            b = c as uint64_t;
        }
    }
    #[inline]
    #[c2rust::src_loc = "62:1"]
    pub unsafe extern "C" fn lcm(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
        return a.wrapping_div(gcd(a, b)).wrapping_mul(b);
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_cli_log(
            name: *const ::core::ffi::c_char,
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:26"]
pub mod input_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:9"]
    pub struct cli_input_opt_t {
        pub index_file: *mut ::core::ffi::c_char,
        pub format: *mut ::core::ffi::c_char,
        pub resolution: *mut ::core::ffi::c_char,
        pub colorspace: *mut ::core::ffi::c_char,
        pub bit_depth: ::core::ffi::c_int,
        pub timebase: *mut ::core::ffi::c_char,
        pub seek: ::core::ffi::c_int,
        pub progress: ::core::ffi::c_int,
        pub output_csp: ::core::ffi::c_int,
        pub output_range: ::core::ffi::c_int,
        pub input_range: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct video_info_t {
        pub csp: ::core::ffi::c_int,
        pub fps_num: uint32_t,
        pub fps_den: uint32_t,
        pub fullrange: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub interlaced: ::core::ffi::c_int,
        pub num_frames: ::core::ffi::c_int,
        pub sar_width: uint32_t,
        pub sar_height: uint32_t,
        pub tff: ::core::ffi::c_int,
        pub thread_safe: ::core::ffi::c_int,
        pub timebase_num: uint32_t,
        pub timebase_den: uint32_t,
        pub vfr: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct cli_image_t {
        pub csp: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub planes: ::core::ffi::c_int,
        pub plane: [*mut uint8_t; 4],
        pub stride: [::core::ffi::c_int; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct cli_pic_t {
        pub img: cli_image_t,
        pub pts: int64_t,
        pub duration: int64_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:9"]
    pub struct cli_input_t {
        pub open_file: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                *mut hnd_t,
                *mut video_info_t,
                *mut cli_input_opt_t,
            ) -> ::core::ffi::c_int,
        >,
        pub picture_alloc: Option<
            unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub read_frame: Option<
            unsafe extern "C" fn(*mut cli_pic_t, hnd_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub release_frame:
            Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int>,
        pub picture_clean: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
        pub close_file: Option<unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int>,
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "111:20"]
        pub static mut cli_input: cli_input_t;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::types_h::__off64_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "654:1"]
        pub fn fgets(
            __s: *mut ::core::ffi::c_char,
            __n: ::core::ffi::c_int,
            __stream: *mut FILE,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "802:1"]
        pub fn fseeko(
            __stream: *mut FILE,
            __off: __off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "805:1"]
        pub fn ftello(__stream: *mut FILE) -> __off64_t;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:26"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "219:1"]
        pub fn strtoul(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:26"]
pub mod osdep_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn x264_is_regular_file(mut filehandle: *mut FILE) -> ::core::ffi::c_int {
        let mut file_stat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(filehandle), &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    use super::bits_stat_h::__S_IFMT;
    use super::stat_h::fstat;
    use super::stdio_h::fileno;
    use super::struct_stat_h::stat;
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __time_t, __uid_t,
    };
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/stdint.h:26"]
pub mod stdint_h {
    #[c2rust::src_loc = "118:10"]
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:26"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "129:1"]
        pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "177:1"]
        pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
            -> ::core::ffi::c_double;
        #[c2rust::src_loc = "216:1"]
        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "219:1"]
        pub fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "355:1"]
        pub fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:26"]
pub mod x264_h {
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:26"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::bits_stat_h::__S_IFMT;
pub use self::input_h::{
    cli_image_t, cli_input, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t,
};
use self::mathcalls_h::{fabs, floor, log10, pow, round};
pub use self::osdep_h::x264_is_regular_file;
use self::stat_h::fstat;
pub use self::stdint_h::UINT32_MAX;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
pub use self::stdio_h::{fclose, fgets, fileno, fopen, fseeko, ftello, sscanf, SEEK_SET};
use self::stdlib_h::{free, malloc, strtoul};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __int64_t, __mode_t, __nlink_t, __off64_t,
    __off_t, __syscall_slong_t, __time_t, __uid_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::x264_h::{X264_LOG_ERROR, X264_LOG_INFO};
pub use self::x264cli_h::{gcd, hnd_t, lcm, x264_cli_log};
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:9"]
pub struct timecode_hnd_t {
    pub input: cli_input_t,
    pub p_handle: hnd_t,
    pub auto_timebase_num: ::core::ffi::c_int,
    pub auto_timebase_den: ::core::ffi::c_int,
    pub timebase_num: uint64_t,
    pub timebase_den: uint64_t,
    pub stored_pts_num: ::core::ffi::c_int,
    pub pts: *mut int64_t,
    pub assume_fps: ::core::ffi::c_double,
    pub last_timecode: ::core::ffi::c_double,
}
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn sigexp10(
    mut value: ::core::ffi::c_double,
    mut exponent: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    *exponent = pow(
        10 as ::core::ffi::c_int as ::core::ffi::c_double,
        floor(log10(value)),
    );
    return value / *exponent;
}
#[c2rust::src_loc = "51:9"]
pub const DOUBLE_EPSILON: ::core::ffi::c_double = 5e-6f64;
#[c2rust::src_loc = "52:9"]
pub const MKV_TIMEBASE_DEN: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn correct_fps(
    mut fps: ::core::ffi::c_double,
    mut h: *mut timecode_hnd_t,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut fps_num: uint64_t = 0;
    let mut fps_den: uint64_t = 0;
    let mut exponent: ::core::ffi::c_double = 0.;
    let mut fps_sig: ::core::ffi::c_double = sigexp10(fps, &mut exponent);
    loop {
        fps_den = (i as uint64_t).wrapping_mul((*h).timebase_num);
        fps_num = (round(fps_den as ::core::ffi::c_double * fps_sig) * exponent) as uint64_t;
        if fps_num > 4294967295 as uint64_t {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"tcfile fps correction failed.\n                  Specify an appropriate timebase manually or remake tcfile.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
        }
        if fabs(
            fps_num as ::core::ffi::c_double / fps_den as ::core::ffi::c_double / exponent
                - fps_sig,
        ) < DOUBLE_EPSILON
        {
            break;
        }
        i += 1;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = if (*h).timebase_den != 0 {
            lcm((*h).timebase_den, fps_num)
        } else {
            fps_num
        };
        if (*h).timebase_den > UINT32_MAX as uint64_t {
            (*h).auto_timebase_den = 0 as ::core::ffi::c_int;
        }
    }
    return fps_num as ::core::ffi::c_double / fps_den as ::core::ffi::c_double;
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn try_mkv_timebase_den(
    mut fpss: *mut ::core::ffi::c_double,
    mut h: *mut timecode_hnd_t,
    mut loop_num: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*h).timebase_num = 0 as uint64_t;
    (*h).timebase_den = MKV_TIMEBASE_DEN as uint64_t;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while num < loop_num {
        let mut fps_den: uint64_t = 0;
        let mut exponent: ::core::ffi::c_double = 0.;
        let mut fps_sig: ::core::ffi::c_double =
            sigexp10(*fpss.offset(num as isize), &mut exponent);
        fps_den =
            (round(MKV_TIMEBASE_DEN as ::core::ffi::c_double / fps_sig) / exponent) as uint64_t;
        (*h).timebase_num = if fps_den != 0 && (*h).timebase_num != 0 {
            gcd((*h).timebase_num, fps_den)
        } else {
            fps_den
        };
        if (*h).timebase_num > 4294967295 as uint64_t || (*h).timebase_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"automatic timebase generation failed.\n                  Specify timebase manually.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        num += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn parse_tcfile(
    mut tcfile_in: *mut FILE,
    mut h: *mut timecode_hnd_t,
    mut info: *mut video_info_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut buff: [::core::ffi::c_char; 256] = [0; 256];
    let mut ret: ::core::ffi::c_int = 0;
    let mut tcfv: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    let mut seq_num: ::core::ffi::c_int = 0;
    let mut timecodes_num: ::core::ffi::c_int = 0;
    let mut timecodes: *mut ::core::ffi::c_double = 0 as *mut ::core::ffi::c_double;
    let mut fpss: *mut ::core::ffi::c_double = 0 as *mut ::core::ffi::c_double;
    ret = (!fgets(
        buff.as_mut_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
        tcfile_in,
    )
    .is_null()
        && (sscanf(
            buff.as_mut_ptr(),
            b"# timecode format v%d\0" as *const u8 as *const ::core::ffi::c_char,
            &mut tcfv as *mut ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int
            || sscanf(
                buff.as_mut_ptr(),
                b"# timestamp format v%d\0" as *const u8 as *const ::core::ffi::c_char,
                &mut tcfv as *mut ::core::ffi::c_int,
            ) == 1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
    if ret == 0 || tcfv != 1 as ::core::ffi::c_int && tcfv != 2 as ::core::ffi::c_int {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"unsupported timecode format\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if tcfv == 1 as ::core::ffi::c_int {
        let mut file_pos: int64_t = 0;
        let mut assume_fps: ::core::ffi::c_double = 0.;
        let mut seq_fps: ::core::ffi::c_double = 0.;
        let mut start: ::core::ffi::c_int = 0;
        let mut end: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut prev_start: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut prev_end: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        (*h).assume_fps = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        num = 2 as ::core::ffi::c_int;
        while !fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\n' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\r' as i32
            {
                num += 1;
            } else {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"assume %lf\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut (*h).assume_fps as *mut ::core::ffi::c_double,
                ) != 1 as ::core::ffi::c_int
                    && sscanf(
                        buff.as_mut_ptr(),
                        b"Assume %lf\0" as *const u8 as *const ::core::ffi::c_char,
                        &mut (*h).assume_fps as *mut ::core::ffi::c_double,
                    ) != 1 as ::core::ffi::c_int
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"tcfile parsing error: assumed fps not found\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                break;
            }
        }
        if (*h).assume_fps <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"invalid assumed fps %.6f\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*h).assume_fps,
            );
            return -(1 as ::core::ffi::c_int);
        }
        file_pos = ftello(tcfile_in) as int64_t;
        (*h).stored_pts_num = 0 as ::core::ffi::c_int;
        seq_num = 0 as ::core::ffi::c_int;
        while !fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\n' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\r' as i32
            {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"# TDecimate Mode 3:  Last Frame = %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &mut end as *mut ::core::ffi::c_int,
                ) == 1 as ::core::ffi::c_int
                {
                    (*h).stored_pts_num = end + 1 as ::core::ffi::c_int;
                }
            } else {
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%d,%d,%lf\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut start as *mut ::core::ffi::c_int,
                    &mut end as *mut ::core::ffi::c_int,
                    &mut seq_fps as *mut ::core::ffi::c_double,
                );
                if ret != 3 as ::core::ffi::c_int && ret != -(1 as ::core::ffi::c_int) {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                if start > end
                    || start <= prev_start
                    || end <= prev_end
                    || seq_fps <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile at line %d: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        num,
                        buff.as_mut_ptr(),
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                prev_start = start;
                prev_end = end;
                if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                    seq_num += 1;
                }
            }
            num += 1;
        }
        if (*h).stored_pts_num == 0 {
            (*h).stored_pts_num = end + 2 as ::core::ffi::c_int;
        }
        timecodes_num = (*h).stored_pts_num;
        fseeko(tcfile_in, file_pos as __off64_t, SEEK_SET);
        timecodes = malloc(
            (timecodes_num as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        ) as *mut ::core::ffi::c_double;
        if timecodes.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
            fpss = malloc(
                ((seq_num + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            ) as *mut ::core::ffi::c_double;
            if fpss.is_null() {
                current_block = 15792084957793291482;
            } else {
                current_block = 13678349939556791712;
            }
        } else {
            current_block = 13678349939556791712;
        }
        match current_block {
            15792084957793291482 => {}
            _ => {
                assume_fps = correct_fps((*h).assume_fps, h);
                if assume_fps < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                    current_block = 15792084957793291482;
                } else {
                    *timecodes.offset(0 as ::core::ffi::c_int as isize) =
                        0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    seq_num = 0 as ::core::ffi::c_int;
                    num = seq_num;
                    loop {
                        if !(num < timecodes_num - 1 as ::core::ffi::c_int
                            && !fgets(
                                buff.as_mut_ptr(),
                                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                                    as ::core::ffi::c_int,
                                tcfile_in,
                            )
                            .is_null())
                        {
                            current_block = 13660591889533726445;
                            break;
                        }
                        if buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == '#' as i32
                            || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                == '\n' as i32
                            || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                == '\r' as i32
                        {
                            continue;
                        }
                        ret = sscanf(
                            buff.as_mut_ptr(),
                            b"%d,%d,%lf\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut start as *mut ::core::ffi::c_int,
                            &mut end as *mut ::core::ffi::c_int,
                            &mut seq_fps as *mut ::core::ffi::c_double,
                        );
                        if ret != 3 as ::core::ffi::c_int {
                            end = timecodes_num - 1 as ::core::ffi::c_int;
                            start = end;
                        }
                        while num < start && num < timecodes_num - 1 as ::core::ffi::c_int {
                            *timecodes.offset((num + 1 as ::core::ffi::c_int) as isize) = *timecodes
                                .offset(num as isize)
                                + 1 as ::core::ffi::c_int as ::core::ffi::c_double / assume_fps;
                            num += 1;
                        }
                        if !(num < timecodes_num - 1 as ::core::ffi::c_int) {
                            continue;
                        }
                        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                            let fresh0 = seq_num;
                            seq_num = seq_num + 1;
                            *fpss.offset(fresh0 as isize) = seq_fps;
                        }
                        seq_fps = correct_fps(seq_fps, h);
                        if seq_fps < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                            current_block = 15792084957793291482;
                            break;
                        }
                        num = start;
                        while num <= end && num < timecodes_num - 1 as ::core::ffi::c_int {
                            *timecodes.offset((num + 1 as ::core::ffi::c_int) as isize) = *timecodes
                                .offset(num as isize)
                                + 1 as ::core::ffi::c_int as ::core::ffi::c_double / seq_fps;
                            num += 1;
                        }
                    }
                    match current_block {
                        15792084957793291482 => {}
                        _ => {
                            while num < timecodes_num - 1 as ::core::ffi::c_int {
                                *timecodes.offset((num + 1 as ::core::ffi::c_int) as isize) =
                                    *timecodes.offset(num as isize)
                                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                                            / assume_fps;
                                num += 1;
                            }
                            if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                                *fpss.offset(seq_num as isize) = (*h).assume_fps;
                            }
                            if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                                let mut exponent: ::core::ffi::c_double = 0.;
                                let mut assume_fps_sig: ::core::ffi::c_double = 0.;
                                let mut seq_fps_sig: ::core::ffi::c_double = 0.;
                                if try_mkv_timebase_den(fpss, h, seq_num + 1 as ::core::ffi::c_int)
                                    < 0 as ::core::ffi::c_int
                                {
                                    current_block = 15792084957793291482;
                                } else {
                                    fseeko(tcfile_in, file_pos as __off64_t, SEEK_SET);
                                    assume_fps_sig = sigexp10((*h).assume_fps, &mut exponent);
                                    assume_fps = MKV_TIMEBASE_DEN as ::core::ffi::c_double
                                        / (round(
                                            MKV_TIMEBASE_DEN as ::core::ffi::c_double
                                                / assume_fps_sig,
                                        ) / exponent);
                                    num = 0 as ::core::ffi::c_int;
                                    while num < timecodes_num - 1 as ::core::ffi::c_int
                                        && !fgets(
                                            buff.as_mut_ptr(),
                                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                                                as ::core::ffi::c_int,
                                            tcfile_in,
                                        )
                                        .is_null()
                                    {
                                        if buff[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            == '#' as i32
                                            || buff[0 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                == '\n' as i32
                                            || buff[0 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                == '\r' as i32
                                        {
                                            continue;
                                        }
                                        ret = sscanf(
                                            buff.as_mut_ptr(),
                                            b"%d,%d,%lf\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            &mut start as *mut ::core::ffi::c_int,
                                            &mut end as *mut ::core::ffi::c_int,
                                            &mut seq_fps as *mut ::core::ffi::c_double,
                                        );
                                        if ret != 3 as ::core::ffi::c_int {
                                            end = timecodes_num - 1 as ::core::ffi::c_int;
                                            start = end;
                                        }
                                        seq_fps_sig = sigexp10(seq_fps, &mut exponent);
                                        seq_fps = MKV_TIMEBASE_DEN as ::core::ffi::c_double
                                            / (round(
                                                MKV_TIMEBASE_DEN as ::core::ffi::c_double
                                                    / seq_fps_sig,
                                            ) / exponent);
                                        while num < start
                                            && num < timecodes_num - 1 as ::core::ffi::c_int
                                        {
                                            *timecodes
                                                .offset((num + 1 as ::core::ffi::c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                        / assume_fps;
                                            num += 1;
                                        }
                                        num = start;
                                        while num <= end
                                            && num < timecodes_num - 1 as ::core::ffi::c_int
                                        {
                                            *timecodes
                                                .offset((num + 1 as ::core::ffi::c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                        / seq_fps;
                                            num += 1;
                                        }
                                    }
                                    while num < timecodes_num - 1 as ::core::ffi::c_int {
                                        *timecodes
                                            .offset((num + 1 as ::core::ffi::c_int) as isize) =
                                            *timecodes.offset(num as isize)
                                                + 1 as ::core::ffi::c_int as ::core::ffi::c_double
                                                    / assume_fps;
                                        num += 1;
                                    }
                                    current_block = 496303045384785551;
                                }
                            } else {
                                current_block = 496303045384785551;
                            }
                            match current_block {
                                15792084957793291482 => {}
                                _ => {
                                    if !fpss.is_null() {
                                        free(fpss as *mut ::core::ffi::c_void);
                                        fpss = 0 as *mut ::core::ffi::c_double;
                                    }
                                    (*h).assume_fps = assume_fps;
                                    (*h).last_timecode = *timecodes
                                        .offset((timecodes_num - 1 as ::core::ffi::c_int) as isize);
                                    current_block = 12129449210080749085;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut file_pos_0: int64_t = ftello(tcfile_in) as int64_t;
        (*h).stored_pts_num = 0 as ::core::ffi::c_int;
        while !fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\n' as i32
                || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\r' as i32
            {
                if (*h).stored_pts_num == 0 {
                    file_pos_0 = ftello(tcfile_in) as int64_t;
                }
            } else {
                (*h).stored_pts_num += 1;
            }
        }
        timecodes_num = (*h).stored_pts_num;
        if timecodes_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"input tcfile doesn't have any timecodes!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        fseeko(tcfile_in, file_pos_0 as __off64_t, SEEK_SET);
        timecodes = malloc(
            (timecodes_num as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
        ) as *mut ::core::ffi::c_double;
        if timecodes.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        num = 0 as ::core::ffi::c_int;
        if !fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            tcfile_in,
        )
        .is_null()
        {
            ret = sscanf(
                buff.as_mut_ptr(),
                b"%lf\0" as *const u8 as *const ::core::ffi::c_char,
                &mut *timecodes.offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            );
            *timecodes.offset(0 as ::core::ffi::c_int as isize) *= 1e-3f64;
            if ret != 1 as ::core::ffi::c_int {
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid input tcfile for frame 0\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            num = 1 as ::core::ffi::c_int;
            while num < timecodes_num
                && !fgets(
                    buff.as_mut_ptr(),
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
                    tcfile_in,
                )
                .is_null()
            {
                if buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
                    || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\n' as i32
                    || buff[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\r' as i32
                {
                    continue;
                }
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%lf\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut *timecodes.offset(num as isize) as *mut ::core::ffi::c_double,
                );
                *timecodes.offset(num as isize) *= 1e-3f64;
                if ret != 1 as ::core::ffi::c_int
                    || *timecodes.offset(num as isize)
                        <= *timecodes.offset((num - 1 as ::core::ffi::c_int) as isize)
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile for frame %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        num,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                num += 1;
            }
        }
        if num < timecodes_num {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"failed to read input tcfile for frame %d\0" as *const u8
                    as *const ::core::ffi::c_char,
                num,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if timecodes_num == 1 as ::core::ffi::c_int {
            (*h).timebase_den = (*info).fps_num as uint64_t;
            current_block = 6938158527927677584;
        } else if (*h).auto_timebase_den != 0 {
            fpss = malloc(
                ((timecodes_num - 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as size_t),
            ) as *mut ::core::ffi::c_double;
            if fpss.is_null() {
                current_block = 15792084957793291482;
            } else {
                num = 0 as ::core::ffi::c_int;
                while num < timecodes_num - 1 as ::core::ffi::c_int {
                    *fpss.offset(num as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        / (*timecodes.offset((num + 1 as ::core::ffi::c_int) as isize)
                            - *timecodes.offset(num as isize));
                    if (*h).auto_timebase_den != 0 {
                        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        let mut fps_num: uint64_t = 0;
                        let mut fps_den: uint64_t = 0;
                        let mut exponent_0: ::core::ffi::c_double = 0.;
                        let mut fps_sig: ::core::ffi::c_double =
                            sigexp10(*fpss.offset(num as isize), &mut exponent_0);
                        loop {
                            fps_den = (i as uint64_t).wrapping_mul((*h).timebase_num);
                            fps_num = (round(fps_den as ::core::ffi::c_double * fps_sig)
                                * exponent_0) as uint64_t;
                            if fps_num > UINT32_MAX as uint64_t
                                || fabs(
                                    fps_num as ::core::ffi::c_double
                                        / fps_den as ::core::ffi::c_double
                                        / exponent_0
                                        - fps_sig,
                                ) < DOUBLE_EPSILON
                            {
                                break;
                            }
                            i += 1;
                        }
                        (*h).timebase_den = if fps_num != 0 && (*h).timebase_den != 0 {
                            lcm((*h).timebase_den, fps_num)
                        } else {
                            fps_num
                        };
                        if (*h).timebase_den > UINT32_MAX as uint64_t {
                            (*h).auto_timebase_den = 0 as ::core::ffi::c_int;
                        }
                    }
                    num += 1;
                }
                if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                    if try_mkv_timebase_den(fpss, h, timecodes_num - 1 as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        current_block = 15792084957793291482;
                    } else {
                        current_block = 10783567741412653655;
                    }
                } else {
                    current_block = 10783567741412653655;
                }
                match current_block {
                    15792084957793291482 => {}
                    _ => {
                        free(fpss as *mut ::core::ffi::c_void);
                        fpss = 0 as *mut ::core::ffi::c_double;
                        current_block = 6938158527927677584;
                    }
                }
            }
        } else {
            current_block = 6938158527927677584;
        }
        match current_block {
            15792084957793291482 => {}
            _ => {
                if timecodes_num > 1 as ::core::ffi::c_int {
                    (*h).assume_fps = 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        / (*timecodes.offset((timecodes_num - 1 as ::core::ffi::c_int) as isize)
                            - *timecodes
                                .offset((timecodes_num - 2 as ::core::ffi::c_int) as isize));
                } else {
                    (*h).assume_fps = (*info).fps_num as ::core::ffi::c_double
                        / (*info).fps_den as ::core::ffi::c_double;
                }
                (*h).last_timecode =
                    *timecodes.offset((timecodes_num - 1 as ::core::ffi::c_int) as isize);
                current_block = 12129449210080749085;
            }
        }
    }
    match current_block {
        12129449210080749085 => {
            if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                let mut i_0: uint64_t = gcd((*h).timebase_num, (*h).timebase_den);
                (*h).timebase_num = (*h).timebase_num.wrapping_div(i_0);
                (*h).timebase_den = (*h).timebase_den.wrapping_div(i_0);
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_INFO,
                    b"automatic timebase generation %lu/%lu\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*h).timebase_num,
                    (*h).timebase_den,
                );
            } else if (*h).timebase_den > 4294967295 as uint64_t || (*h).timebase_den == 0 {
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"automatic timebase generation failed.\n                  Specify an appropriate timebase manually.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*h).pts = malloc(
                ((*h).stored_pts_num as size_t)
                    .wrapping_mul(::core::mem::size_of::<int64_t>() as size_t),
            ) as *mut int64_t;
            if !(*h).pts.is_null() {
                num = 0 as ::core::ffi::c_int;
                while num < (*h).stored_pts_num {
                    *(*h).pts.offset(num as isize) = (*timecodes.offset(num as isize)
                        * ((*h).timebase_den as ::core::ffi::c_double
                            / (*h).timebase_num as ::core::ffi::c_double)
                        + 0.5f64) as int64_t;
                    if num > 0 as ::core::ffi::c_int
                        && *(*h).pts.offset(num as isize)
                            <= *(*h).pts.offset((num - 1 as ::core::ffi::c_int) as isize)
                    {
                        x264_cli_log(
                            b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                            X264_LOG_ERROR,
                            b"invalid timebase or timecode for frame %d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            num,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    num += 1;
                }
                free(timecodes as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    if !timecodes.is_null() {
        free(timecodes as *mut ::core::ffi::c_void);
    }
    if !fpss.is_null() {
        free(fpss as *mut ::core::ffi::c_void);
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "344:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tcfile_in: *mut FILE = 0 as *mut FILE;
    let mut h: *mut timecode_hnd_t =
        malloc(::core::mem::size_of::<timecode_hnd_t>() as size_t) as *mut timecode_hnd_t;
    if h.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).pts = 0 as *mut int64_t;
    if !(*opt).timebase.is_null() {
        ret = sscanf(
            (*opt).timebase,
            b"%lu/%lu\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*h).timebase_num as *mut uint64_t,
            &mut (*h).timebase_den as *mut uint64_t,
        );
        if ret == 1 as ::core::ffi::c_int {
            (*h).timebase_num = strtoul(
                (*opt).timebase,
                0 as *mut *mut ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            ) as uint64_t;
            (*h).timebase_den = 0 as uint64_t;
        }
        if (*h).timebase_num > 4294967295 as uint64_t || (*h).timebase_den > 4294967295 as uint64_t
        {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"timebase you specified exceeds H.264 maximum\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*h).auto_timebase_num = (ret == 0) as ::core::ffi::c_int;
    (*h).auto_timebase_den = (ret < 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*h).auto_timebase_num != 0 {
        (*h).timebase_num = (*info).fps_den as uint64_t;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = 0 as uint64_t;
    }
    tcfile_in = fopen(
        psz_filename,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if tcfile_in.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"can't open `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            psz_filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if x264_is_regular_file(tcfile_in) == 0 {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"tcfile input incompatible with non-regular file `%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            psz_filename,
        );
        fclose(tcfile_in);
        return -(1 as ::core::ffi::c_int);
    }
    if parse_tcfile(tcfile_in, h, info) < 0 as ::core::ffi::c_int {
        if !(*h).pts.is_null() {
            free((*h).pts as *mut ::core::ffi::c_void);
        }
        fclose(tcfile_in);
        return -(1 as ::core::ffi::c_int);
    }
    fclose(tcfile_in);
    (*info).timebase_num = (*h).timebase_num as uint32_t;
    (*info).timebase_den = (*h).timebase_den as uint32_t;
    (*info).vfr = 1 as ::core::ffi::c_int;
    *p_handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn get_frame_pts(
    mut h: *mut timecode_hnd_t,
    mut frame: ::core::ffi::c_int,
    mut real_frame: ::core::ffi::c_int,
) -> int64_t {
    if frame < (*h).stored_pts_num {
        return *(*h).pts.offset(frame as isize);
    } else {
        if !(*h).pts.is_null() && real_frame != 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_INFO,
                b"input timecode file missing data for frame %d and later\n                 assuming constant fps %.6f\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                frame,
                (*h).assume_fps,
            );
            free((*h).pts as *mut ::core::ffi::c_void);
            (*h).pts = 0 as *mut int64_t;
        }
        let mut timecode: ::core::ffi::c_double =
            (*h).last_timecode + 1 as ::core::ffi::c_int as ::core::ffi::c_double / (*h).assume_fps;
        if real_frame != 0 {
            (*h).last_timecode = timecode;
        }
        return (timecode
            * ((*h).timebase_den as ::core::ffi::c_double
                / (*h).timebase_num as ::core::ffi::c_double)
            + 0.5f64) as int64_t;
    };
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if (*h).input.read_frame.expect("non-null function pointer")(pic, (*h).p_handle, frame) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic).pts = get_frame_pts(h, frame, 1 as ::core::ffi::c_int);
    (*pic).duration =
        get_frame_pts(h, frame + 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) - (*pic).pts;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> ::core::ffi::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if (*h).input.release_frame.is_some() {
        return (*h).input.release_frame.expect("non-null function pointer")(pic, (*h).p_handle);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "437:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    return (*h).input.picture_alloc.expect("non-null function pointer")(
        pic,
        (*h).p_handle,
        csp,
        width,
        height,
    );
}
#[c2rust::src_loc = "443:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    (*h).input.picture_clean.expect("non-null function pointer")(pic, (*h).p_handle);
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> ::core::ffi::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if !(*h).pts.is_null() {
        free((*h).pts as *mut ::core::ffi::c_void);
    }
    (*h).input.close_file.expect("non-null function pointer")((*h).p_handle);
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "459:19"]
pub static mut timecode_input: cli_input_t = unsafe {
    {
        let mut init = cli_input_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut hnd_t,
                        *mut video_info_t,
                        *mut cli_input_opt_t,
                    ) -> ::core::ffi::c_int,
            ),
            picture_alloc: Some(
                picture_alloc
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            read_frame: Some(
                read_frame
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            release_frame: Some(
                release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int,
            ),
            picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
            close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int),
        };
        init
    }
};
