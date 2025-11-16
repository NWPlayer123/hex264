use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:27"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:27"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "139:9"]
    pub struct cli_mmap_t {
        pub file_size: int64_t,
        pub align_mask: ::core::ffi::c_int,
        pub fd: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:9"]
    pub struct x264_cli_csp_t {
        pub name: *const ::core::ffi::c_char,
        pub planes: ::core::ffi::c_int,
        pub width: [::core::ffi::c_float; 4],
        pub height: [::core::ffi::c_float; 4],
        pub mod_width: ::core::ffi::c_int,
        pub mod_height: ::core::ffi::c_int,
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::x264cli_h::hnd_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "130:1"]
        pub fn x264_cli_csp_depth_factor(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "131:1"]
        pub fn x264_cli_pic_alloc(
            pic: *mut cli_pic_t,
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "133:1"]
        pub fn x264_cli_pic_init_noalloc(
            pic: *mut cli_pic_t,
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "134:1"]
        pub fn x264_cli_pic_clean(pic: *mut cli_pic_t);
        #[c2rust::src_loc = "135:1"]
        pub fn x264_cli_pic_plane_size(
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
            plane: ::core::ffi::c_int,
        ) -> int64_t;
        #[c2rust::src_loc = "137:1"]
        pub fn x264_cli_get_csp(csp: ::core::ffi::c_int) -> *const x264_cli_csp_t;
        #[c2rust::src_loc = "153:1"]
        pub fn x264_cli_mmap_init(h: *mut cli_mmap_t, fh: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "154:1"]
        pub fn x264_cli_mmap(
            h: *mut cli_mmap_t,
            offset: int64_t,
            size: int64_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "155:1"]
        pub fn x264_cli_munmap(
            h: *mut cli_mmap_t,
            addr: *mut ::core::ffi::c_void,
            size: int64_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "156:1"]
        pub fn x264_cli_mmap_close(h: *mut cli_mmap_t);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "112:9"]
    pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "149:14"]
        pub static mut stdin: *mut FILE;
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
        #[c2rust::src_loc = "578:1"]
        pub fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "728:1"]
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __stream: *mut FILE,
        ) -> ::core::ffi::c_ulong;
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
#[c2rust::header_src = "/usr/include/sys/stat.h:27"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "215:1"]
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
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
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "64:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "159:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "246:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:27"]
pub mod base_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "271:10"]
        pub fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const X264_CSP_I400: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:27"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_size_t_h::size_t;
use self::base_h::x264_reduce_fraction;
pub use self::bits_stat_h::__S_IFMT;
pub use self::input_h::{
    cli_image_t, cli_input_opt_t, cli_input_t, cli_mmap_t, cli_pic_t, video_info_t,
    x264_cli_csp_depth_factor, x264_cli_csp_t, x264_cli_get_csp, x264_cli_mmap,
    x264_cli_mmap_close, x264_cli_mmap_init, x264_cli_munmap, x264_cli_pic_alloc,
    x264_cli_pic_clean, x264_cli_pic_init_noalloc, x264_cli_pic_plane_size,
};
pub use self::osdep_h::x264_is_regular_file;
use self::stat_h::fstat;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
pub use self::stdio_h::{
    fclose, fgetc, fileno, fopen, fread, fseeko, ftello, sscanf, stdin, SEEK_END, SEEK_SET,
};
use self::stdlib_h::{calloc, free, strtol};
use self::string_h::{memcmp, memset, strchr, strcmp, strncmp};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __int64_t, __mode_t, __nlink_t, __off64_t,
    __off_t, __syscall_slong_t, __time_t, __uid_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::x264_h::{
    X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MAX,
    X264_CSP_NONE, X264_LOG_ERROR,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:9"]
pub struct y4m_hnd_t {
    pub fh: *mut FILE,
    pub next_frame: ::core::ffi::c_int,
    pub seq_header_len: ::core::ffi::c_int,
    pub frame_header_len: ::core::ffi::c_int,
    pub frame_size: int64_t,
    pub plane_size: [int64_t; 3],
    pub bit_depth: ::core::ffi::c_int,
    pub mmap: cli_mmap_t,
    pub use_mmap: ::core::ffi::c_int,
}
static mut slen: size_t = 0;
#[c2rust::src_loc = "44:9"]
pub const Y4M_MAGIC: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"YUV4MPEG2\0") };
#[c2rust::src_loc = "45:9"]
pub const Y4M_FRAME_MAGIC: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"FRAME\0") };
#[c2rust::src_loc = "46:9"]
pub const Y4M_MAX_HEADER: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn parse_csp_and_depth(
    mut csp_name: *mut ::core::ffi::c_char,
    mut bit_depth: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut csp: ::core::ffi::c_int = X264_CSP_MAX;
    if strncmp(
        b"mono\0" as *const u8 as *const ::core::ffi::c_char,
        csp_name,
        4 as size_t,
    ) == 0
    {
        csp = X264_CSP_I400;
    } else if strncmp(
        b"420\0" as *const u8 as *const ::core::ffi::c_char,
        csp_name,
        3 as size_t,
    ) == 0
    {
        csp = X264_CSP_I420;
    } else if strncmp(
        b"422\0" as *const u8 as *const ::core::ffi::c_char,
        csp_name,
        3 as size_t,
    ) == 0
    {
        csp = X264_CSP_I422;
    } else if strncmp(
        b"444\0" as *const u8 as *const ::core::ffi::c_char,
        csp_name,
        3 as size_t,
    ) == 0
        && strncmp(
            b"444alpha\0" as *const u8 as *const ::core::ffi::c_char,
            csp_name,
            8 as size_t,
        ) != 0
    {
        csp = X264_CSP_I444;
    }
    if sscanf(
        csp_name,
        b"mono%d\0" as *const u8 as *const ::core::ffi::c_char,
        bit_depth,
    ) != 1 as ::core::ffi::c_int
        && sscanf(
            csp_name,
            b"%*d%*[pP]%d\0" as *const u8 as *const ::core::ffi::c_char,
            bit_depth,
        ) != 1 as ::core::ffi::c_int
    {
        *bit_depth = 8 as ::core::ffi::c_int;
    }
    return csp;
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> ::core::ffi::c_int {
    let mut h: *mut y4m_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<y4m_hnd_t>() as size_t) as *mut y4m_hnd_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut header: [::core::ffi::c_char; 266] = [0; 266];
    let mut tokend: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut header_end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut colorspace: ::core::ffi::c_int = X264_CSP_NONE;
    let mut alt_colorspace: ::core::ffi::c_int = X264_CSP_NONE;
    let mut alt_bit_depth: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*info).vfr = 0 as ::core::ffi::c_int;
    if strcmp(
        psz_filename,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*h).fh = stdin;
    } else {
        (*h).fh = fopen(
            psz_filename,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
    }
    if (*h).fh.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < Y4M_MAX_HEADER {
        header[i as usize] = fgetc((*h).fh) as ::core::ffi::c_char;
        if header[i as usize] as ::core::ffi::c_int == '\n' as i32 {
            header[(i + 1 as ::core::ffi::c_int) as usize] = 0x20 as ::core::ffi::c_char;
            header[(i + 2 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
            break;
        } else {
            i += 1;
        }
    }
    if strncmp(
        header.as_mut_ptr(),
        b"YUV4MPEG2\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1 as size_t),
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"bad sequence header magic\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if i == 256 as ::core::ffi::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"bad sequence header length\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    header_end = &mut *header
        .as_mut_ptr()
        .offset((i + 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char;
    (*h).seq_header_len = i + 1 as ::core::ffi::c_int;
    let mut tokstart: *mut ::core::ffi::c_char = header
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 10]>() as usize as isize);
    while tokstart < header_end {
        if !(*tokstart as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int) {
            let fresh0 = tokstart;
            tokstart = tokstart.offset(1);
            match *fresh0 as ::core::ffi::c_int {
                87 => {
                    (*info).width = strtol(tokstart, &mut tokend, 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    tokstart = tokend;
                }
                72 => {
                    (*info).height = strtol(tokstart, &mut tokend, 10 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    tokstart = tokend;
                }
                67 => {
                    colorspace = parse_csp_and_depth(tokstart, &mut (*h).bit_depth);
                    tokstart = strchr(tokstart, 0x20 as ::core::ffi::c_int);
                }
                73 => {
                    let fresh1 = tokstart;
                    tokstart = tokstart.offset(1);
                    match *fresh1 as ::core::ffi::c_int {
                        116 => {
                            (*info).interlaced = 1 as ::core::ffi::c_int;
                            (*info).tff = 1 as ::core::ffi::c_int;
                        }
                        98 => {
                            (*info).interlaced = 1 as ::core::ffi::c_int;
                            (*info).tff = 0 as ::core::ffi::c_int;
                        }
                        109 => {
                            (*info).interlaced = 1 as ::core::ffi::c_int;
                        }
                        _ => {}
                    }
                }
                70 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const ::core::ffi::c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as ::core::ffi::c_int
                        && n != 0
                        && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).fps_num = n;
                        (*info).fps_den = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as ::core::ffi::c_int);
                }
                65 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const ::core::ffi::c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as ::core::ffi::c_int
                        && n != 0
                        && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).sar_width = n;
                        (*info).sar_height = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as ::core::ffi::c_int);
                }
                88 => {
                    if strncmp(
                        b"YSCSS=\0" as *const u8 as *const ::core::ffi::c_char,
                        tokstart,
                        6 as size_t,
                    ) == 0
                    {
                        tokstart = tokstart.offset(6 as ::core::ffi::c_int as isize);
                        alt_colorspace = parse_csp_and_depth(tokstart, &mut alt_bit_depth);
                    } else if strncmp(
                        b"COLORRANGE=\0" as *const u8 as *const ::core::ffi::c_char,
                        tokstart,
                        11 as size_t,
                    ) == 0
                    {
                        tokstart = tokstart.offset(11 as ::core::ffi::c_int as isize);
                        if strncmp(
                            b"FULL\0" as *const u8 as *const ::core::ffi::c_char,
                            tokstart,
                            4 as size_t,
                        ) == 0
                        {
                            (*info).fullrange = 1 as ::core::ffi::c_int;
                        } else if strncmp(
                            b"LIMITED\0" as *const u8 as *const ::core::ffi::c_char,
                            tokstart,
                            7 as size_t,
                        ) == 0
                        {
                            (*info).fullrange = 0 as ::core::ffi::c_int;
                        }
                    }
                    tokstart = strchr(tokstart, 0x20 as ::core::ffi::c_int);
                }
                _ => {}
            }
        }
        tokstart = tokstart.offset(1);
    }
    if colorspace == X264_CSP_NONE {
        colorspace = alt_colorspace;
        (*h).bit_depth = alt_bit_depth;
    }
    if colorspace == X264_CSP_NONE {
        colorspace = X264_CSP_I420;
        (*h).bit_depth = 8 as ::core::ffi::c_int;
    }
    if colorspace <= 0 as ::core::ffi::c_int || colorspace >= 0x11 as ::core::ffi::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"colorspace unhandled\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).bit_depth < 8 as ::core::ffi::c_int || (*h).bit_depth > 16 as ::core::ffi::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"unsupported bit depth `%d'\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).bit_depth,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*info).thread_safe = 1 as ::core::ffi::c_int;
    (*info).num_frames = 0 as ::core::ffi::c_int;
    (*info).csp = colorspace;
    if (*h).bit_depth > 8 as ::core::ffi::c_int {
        (*info).csp |= X264_CSP_HIGH_DEPTH;
    }
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*info).csp);
    i = 0 as ::core::ffi::c_int;
    while i < (*csp).planes {
        (*h).plane_size[i as usize] =
            x264_cli_pic_plane_size((*info).csp, (*info).width, (*info).height, i);
        (*h).frame_size += (*h).plane_size[i as usize];
        (*h).plane_size[i as usize] /= x264_cli_csp_depth_factor((*info).csp) as int64_t;
        i += 1;
    }
    if x264_is_regular_file((*h).fh) != 0 {
        let mut init_pos: int64_t = ftello((*h).fh) as int64_t;
        let mut len: size_t = 1 as size_t;
        while len <= Y4M_MAX_HEADER as size_t && fgetc((*h).fh) != '\n' as i32 {
            len = len.wrapping_add(1);
        }
        if len > 256 as size_t || len < ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize
        {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).frame_header_len = len as ::core::ffi::c_int;
        (*h).frame_size = ((*h).frame_size as size_t).wrapping_add(len) as int64_t as int64_t;
        fseeko((*h).fh, 0 as __off64_t, SEEK_END);
        let mut i_size: int64_t = ftello((*h).fh) as int64_t;
        fseeko((*h).fh, init_pos as __off64_t, SEEK_SET);
        (*info).num_frames =
            ((i_size - (*h).seq_header_len as int64_t) / (*h).frame_size) as ::core::ffi::c_int;
        if (*info).num_frames == 0 {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"empty input file\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).bit_depth & 7 as ::core::ffi::c_int == 0 {
            (*h).use_mmap =
                (x264_cli_mmap_init(&mut (*h).mmap, (*h).fh) == 0) as ::core::ffi::c_int;
        }
    }
    *p_handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "249:1"]
unsafe extern "C" fn read_frame_internal(
    mut pic: *mut cli_pic_t,
    mut h: *mut y4m_hnd_t,
    mut bit_depth_uc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pixel_depth: ::core::ffi::c_int = x264_cli_csp_depth_factor((*pic).img.csp);
    let mut i: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as ::core::ffi::c_int;
    let mut header_buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut header: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    if (*h).use_mmap != 0 {
        header = (*pic).img.plane[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_char;
        (*pic).img.plane[0 as ::core::ffi::c_int as usize] = (*pic).img.plane
            [0 as ::core::ffi::c_int as usize]
            .offset((*h).frame_header_len as isize);
        while i <= (*h).frame_header_len
            && *header.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != '\n' as i32
        {
            i += 1;
        }
        if i != (*h).frame_header_len {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        header = header_buf.as_mut_ptr();
        if fread(
            header as *mut ::core::ffi::c_void,
            1 as size_t,
            slen,
            (*h).fh,
        ) as size_t
            != slen
        {
            return -(1 as ::core::ffi::c_int);
        }
        while i <= Y4M_MAX_HEADER && fgetc((*h).fh) != '\n' as i32 {
            i += 1;
        }
        if i > 256 as ::core::ffi::c_int {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if memcmp(
        header as *const ::core::ffi::c_void,
        b"FRAME\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        slen,
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"bad frame header magic\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pic).img.planes {
        if (*h).use_mmap != 0 {
            if i != 0 {
                (*pic).img.plane[i as usize] =
                    (*pic).img.plane[(i - 1 as ::core::ffi::c_int) as usize].offset(
                        (pixel_depth as int64_t
                            * (*h).plane_size[(i - 1 as ::core::ffi::c_int) as usize])
                            as isize,
                    );
            }
        } else if fread(
            (*pic).img.plane[i as usize] as *mut ::core::ffi::c_void,
            pixel_depth as size_t,
            (*h).plane_size[i as usize] as size_t,
            (*h).fh,
        ) as uint64_t
            != (*h).plane_size[i as usize] as uint64_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        if bit_depth_uc != 0 {
            let mut plane: *mut uint16_t = (*pic).img.plane[i as usize] as *mut uint16_t;
            let mut pixel_count: int64_t = (*h).plane_size[i as usize];
            let mut lshift: ::core::ffi::c_int = 16 as ::core::ffi::c_int - (*h).bit_depth;
            let mut j: int64_t = 0 as int64_t;
            while j < pixel_count {
                *plane.offset(j as isize) =
                    ((*plane.offset(j as isize) as ::core::ffi::c_int) << lshift) as uint16_t;
                j += 1;
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        (*pic).img.plane[0 as ::core::ffi::c_int as usize] = x264_cli_mmap(
            &mut (*h).mmap,
            (*h).frame_size * i_frame as int64_t + (*h).seq_header_len as int64_t,
            (*h).frame_size,
        ) as *mut uint8_t;
        if (*pic).img.plane[0 as ::core::ffi::c_int as usize].is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    } else if i_frame > (*h).next_frame {
        if x264_is_regular_file((*h).fh) != 0 {
            fseeko(
                (*h).fh,
                (*h).frame_size as __off64_t * i_frame as __off64_t
                    + (*h).seq_header_len as __off64_t,
                SEEK_SET,
            );
        } else {
            while i_frame > (*h).next_frame {
                if read_frame_internal(pic, h, 0 as ::core::ffi::c_int) != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
                (*h).next_frame += 1;
            }
        }
    }
    if read_frame_internal(pic, h, (*h).bit_depth & 7 as ::core::ffi::c_int) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).next_frame = i_frame + 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "335:1"]
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> ::core::ffi::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        return x264_cli_munmap(
            &mut (*h).mmap,
            (*pic).img.plane[0 as ::core::ffi::c_int as usize]
                .offset(-((*h).frame_header_len as isize)) as *mut ::core::ffi::c_void,
            (*h).frame_size,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "343:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    return if (*h).use_mmap != 0 {
        Some(
            x264_cli_pic_init_noalloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            x264_cli_pic_alloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
    }
    .expect("non-null function pointer")(pic, csp, width, height);
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        memset(
            pic as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<cli_pic_t>() as size_t,
        );
    } else {
        x264_cli_pic_clean(pic);
    };
}
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> ::core::ffi::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if h.is_null() || (*h).fh.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*h).use_mmap != 0 {
        x264_cli_mmap_close(&mut (*h).mmap);
    }
    fclose((*h).fh);
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "370:19"]
pub static mut y4m_input: cli_input_t = unsafe {
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
unsafe extern "C" fn run_static_initializers() {
    slen = (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1 as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
