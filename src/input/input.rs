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
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint64_t, __uint8_t};
}
#[c2rust::header_src = "/usr/include/stdint.h:27"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "216:11"]
    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:27"]
pub mod input_h {
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
    #[c2rust::src_loc = "117:9"]
    pub struct x264_cli_csp_t {
        pub name: *const ::core::ffi::c_char,
        pub planes: ::core::ffi::c_int,
        pub width: [::core::ffi::c_float; 4],
        pub height: [::core::ffi::c_float; 4],
        pub mod_width: ::core::ffi::c_int,
        pub mod_height: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "139:9"]
    pub struct cli_mmap_t {
        pub file_size: int64_t,
        pub align_mask: ::core::ffi::c_int,
        pub fd: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "114:9"]
    pub const X264_CSP_CLI_MAX: ::core::ffi::c_int = X264_CSP_MAX;
    #[c2rust::src_loc = "115:9"]
    pub const X264_CSP_OTHER: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    use super::x264_h::X264_CSP_MAX;
}
#[c2rust::header_src = "/usr/include/bits/confname.h:27"]
pub mod confname_h {
    #[c2rust::src_loc = "133:5"]
    pub const _SC_PAGESIZE: C2RustUnnamed = 30;
    #[c2rust::src_loc = "71:1"]
    pub type C2RustUnnamed = ::core::ffi::c_uint;
    #[c2rust::src_loc = "534:5"]
    pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
    #[c2rust::src_loc = "531:5"]
    pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
    #[c2rust::src_loc = "528:5"]
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
    #[c2rust::src_loc = "526:5"]
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
    #[c2rust::src_loc = "523:5"]
    pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
    #[c2rust::src_loc = "520:5"]
    pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
    #[c2rust::src_loc = "518:5"]
    pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
    #[c2rust::src_loc = "516:5"]
    pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
    #[c2rust::src_loc = "514:5"]
    pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
    #[c2rust::src_loc = "511:5"]
    pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
    #[c2rust::src_loc = "508:5"]
    pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
    #[c2rust::src_loc = "506:5"]
    pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
    #[c2rust::src_loc = "504:5"]
    pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
    #[c2rust::src_loc = "502:5"]
    pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
    #[c2rust::src_loc = "499:5"]
    pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
    #[c2rust::src_loc = "497:5"]
    pub const _SC_IPV6: C2RustUnnamed = 235;
    #[c2rust::src_loc = "493:5"]
    pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
    #[c2rust::src_loc = "491:5"]
    pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
    #[c2rust::src_loc = "489:5"]
    pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
    #[c2rust::src_loc = "487:5"]
    pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
    #[c2rust::src_loc = "485:5"]
    pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
    #[c2rust::src_loc = "483:5"]
    pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
    #[c2rust::src_loc = "481:5"]
    pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
    #[c2rust::src_loc = "479:5"]
    pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
    #[c2rust::src_loc = "477:5"]
    pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
    #[c2rust::src_loc = "475:5"]
    pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
    #[c2rust::src_loc = "473:5"]
    pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
    #[c2rust::src_loc = "471:5"]
    pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
    #[c2rust::src_loc = "469:5"]
    pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
    #[c2rust::src_loc = "467:5"]
    pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
    #[c2rust::src_loc = "465:5"]
    pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
    #[c2rust::src_loc = "462:5"]
    pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
    #[c2rust::src_loc = "460:5"]
    pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
    #[c2rust::src_loc = "458:5"]
    pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
    #[c2rust::src_loc = "456:5"]
    pub const _SC_TRACE: C2RustUnnamed = 181;
    #[c2rust::src_loc = "454:5"]
    pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
    #[c2rust::src_loc = "451:5"]
    pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
    #[c2rust::src_loc = "449:5"]
    pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
    #[c2rust::src_loc = "447:5"]
    pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
    #[c2rust::src_loc = "445:5"]
    pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
    #[c2rust::src_loc = "442:5"]
    pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
    #[c2rust::src_loc = "440:5"]
    pub const _SC_STREAMS: C2RustUnnamed = 174;
    #[c2rust::src_loc = "438:5"]
    pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
    #[c2rust::src_loc = "436:5"]
    pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
    #[c2rust::src_loc = "434:5"]
    pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
    #[c2rust::src_loc = "432:5"]
    pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
    #[c2rust::src_loc = "430:5"]
    pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
    #[c2rust::src_loc = "428:5"]
    pub const _SC_2_PBS: C2RustUnnamed = 168;
    #[c2rust::src_loc = "426:5"]
    pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
    #[c2rust::src_loc = "424:5"]
    pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
    #[c2rust::src_loc = "422:5"]
    pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
    #[c2rust::src_loc = "420:5"]
    pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
    #[c2rust::src_loc = "418:5"]
    pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
    #[c2rust::src_loc = "416:5"]
    pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
    #[c2rust::src_loc = "414:5"]
    pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
    #[c2rust::src_loc = "412:5"]
    pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
    #[c2rust::src_loc = "410:5"]
    pub const _SC_SPAWN: C2RustUnnamed = 159;
    #[c2rust::src_loc = "408:5"]
    pub const _SC_SIGNALS: C2RustUnnamed = 158;
    #[c2rust::src_loc = "406:5"]
    pub const _SC_SHELL: C2RustUnnamed = 157;
    #[c2rust::src_loc = "404:5"]
    pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
    #[c2rust::src_loc = "402:5"]
    pub const _SC_REGEXP: C2RustUnnamed = 155;
    #[c2rust::src_loc = "400:5"]
    pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
    #[c2rust::src_loc = "398:5"]
    pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
    #[c2rust::src_loc = "396:5"]
    pub const _SC_NETWORKING: C2RustUnnamed = 152;
    #[c2rust::src_loc = "394:5"]
    pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
    #[c2rust::src_loc = "392:5"]
    pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
    #[c2rust::src_loc = "390:5"]
    pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
    #[c2rust::src_loc = "388:5"]
    pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
    #[c2rust::src_loc = "386:5"]
    pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
    #[c2rust::src_loc = "384:5"]
    pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
    #[c2rust::src_loc = "382:5"]
    pub const _SC_PIPE: C2RustUnnamed = 145;
    #[c2rust::src_loc = "380:5"]
    pub const _SC_FIFO: C2RustUnnamed = 144;
    #[c2rust::src_loc = "378:5"]
    pub const _SC_FD_MGMT: C2RustUnnamed = 143;
    #[c2rust::src_loc = "376:5"]
    pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
    #[c2rust::src_loc = "374:5"]
    pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
    #[c2rust::src_loc = "372:5"]
    pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
    #[c2rust::src_loc = "370:5"]
    pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
    #[c2rust::src_loc = "368:5"]
    pub const _SC_CPUTIME: C2RustUnnamed = 138;
    #[c2rust::src_loc = "366:5"]
    pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
    #[c2rust::src_loc = "364:5"]
    pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
    #[c2rust::src_loc = "362:5"]
    pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
    #[c2rust::src_loc = "360:5"]
    pub const _SC_BASE: C2RustUnnamed = 134;
    #[c2rust::src_loc = "358:5"]
    pub const _SC_BARRIERS: C2RustUnnamed = 133;
    #[c2rust::src_loc = "356:5"]
    pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
    #[c2rust::src_loc = "353:5"]
    pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
    #[c2rust::src_loc = "351:5"]
    pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
    #[c2rust::src_loc = "349:5"]
    pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
    #[c2rust::src_loc = "346:5"]
    pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
    #[c2rust::src_loc = "344:5"]
    pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
    #[c2rust::src_loc = "342:5"]
    pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
    #[c2rust::src_loc = "340:5"]
    pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
    #[c2rust::src_loc = "337:5"]
    pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
    #[c2rust::src_loc = "335:5"]
    pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
    #[c2rust::src_loc = "333:5"]
    pub const _SC_NL_NMAX: C2RustUnnamed = 122;
    #[c2rust::src_loc = "331:5"]
    pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
    #[c2rust::src_loc = "329:5"]
    pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
    #[c2rust::src_loc = "327:5"]
    pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
    #[c2rust::src_loc = "324:5"]
    pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
    #[c2rust::src_loc = "322:5"]
    pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
    #[c2rust::src_loc = "320:5"]
    pub const _SC_UINT_MAX: C2RustUnnamed = 116;
    #[c2rust::src_loc = "318:5"]
    pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
    #[c2rust::src_loc = "316:5"]
    pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
    #[c2rust::src_loc = "314:5"]
    pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
    #[c2rust::src_loc = "312:5"]
    pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
    #[c2rust::src_loc = "310:5"]
    pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
    #[c2rust::src_loc = "308:5"]
    pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
    #[c2rust::src_loc = "306:5"]
    pub const _SC_NZERO: C2RustUnnamed = 109;
    #[c2rust::src_loc = "304:5"]
    pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
    #[c2rust::src_loc = "302:5"]
    pub const _SC_WORD_BIT: C2RustUnnamed = 107;
    #[c2rust::src_loc = "300:5"]
    pub const _SC_LONG_BIT: C2RustUnnamed = 106;
    #[c2rust::src_loc = "298:5"]
    pub const _SC_INT_MIN: C2RustUnnamed = 105;
    #[c2rust::src_loc = "296:5"]
    pub const _SC_INT_MAX: C2RustUnnamed = 104;
    #[c2rust::src_loc = "294:5"]
    pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
    #[c2rust::src_loc = "292:5"]
    pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
    #[c2rust::src_loc = "290:5"]
    pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
    #[c2rust::src_loc = "287:5"]
    pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
    #[c2rust::src_loc = "285:5"]
    pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
    #[c2rust::src_loc = "283:5"]
    pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
    #[c2rust::src_loc = "280:5"]
    pub const _SC_2_UPE: C2RustUnnamed = 97;
    #[c2rust::src_loc = "278:5"]
    pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
    #[c2rust::src_loc = "276:5"]
    pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
    #[c2rust::src_loc = "273:5"]
    pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
    #[c2rust::src_loc = "271:5"]
    pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
    #[c2rust::src_loc = "269:5"]
    pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
    #[c2rust::src_loc = "267:5"]
    pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
    #[c2rust::src_loc = "265:5"]
    pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
    #[c2rust::src_loc = "263:5"]
    pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
    #[c2rust::src_loc = "260:5"]
    pub const _SC_PASS_MAX: C2RustUnnamed = 88;
    #[c2rust::src_loc = "258:5"]
    pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
    #[c2rust::src_loc = "256:5"]
    pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
    #[c2rust::src_loc = "254:5"]
    pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
    #[c2rust::src_loc = "252:5"]
    pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
    #[c2rust::src_loc = "250:5"]
    pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
    #[c2rust::src_loc = "247:5"]
    pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
    #[c2rust::src_loc = "245:5"]
    pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
    #[c2rust::src_loc = "243:5"]
    pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
    #[c2rust::src_loc = "241:5"]
    pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
    #[c2rust::src_loc = "239:5"]
    pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
    #[c2rust::src_loc = "237:5"]
    pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
    #[c2rust::src_loc = "235:5"]
    pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
    #[c2rust::src_loc = "233:5"]
    pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
    #[c2rust::src_loc = "231:5"]
    pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
    #[c2rust::src_loc = "229:5"]
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
    #[c2rust::src_loc = "227:5"]
    pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
    #[c2rust::src_loc = "225:5"]
    pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
    #[c2rust::src_loc = "223:5"]
    pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
    #[c2rust::src_loc = "221:5"]
    pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
    #[c2rust::src_loc = "219:5"]
    pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
    #[c2rust::src_loc = "217:5"]
    pub const _SC_THREADS: C2RustUnnamed = 67;
    #[c2rust::src_loc = "213:5"]
    pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
    #[c2rust::src_loc = "211:5"]
    pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
    #[c2rust::src_loc = "209:5"]
    pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
    #[c2rust::src_loc = "207:5"]
    pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
    #[c2rust::src_loc = "205:5"]
    pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
    #[c2rust::src_loc = "203:5"]
    pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
    #[c2rust::src_loc = "201:5"]
    pub const _SC_IOV_MAX: C2RustUnnamed = 60;
    #[c2rust::src_loc = "199:5"]
    pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
    #[c2rust::src_loc = "197:5"]
    pub const _SC_SELECT: C2RustUnnamed = 59;
    #[c2rust::src_loc = "195:5"]
    pub const _SC_POLL: C2RustUnnamed = 58;
    #[c2rust::src_loc = "193:5"]
    pub const _SC_PII_OSI: C2RustUnnamed = 57;
    #[c2rust::src_loc = "191:5"]
    pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
    #[c2rust::src_loc = "189:5"]
    pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
    #[c2rust::src_loc = "187:5"]
    pub const _SC_PII_XTI: C2RustUnnamed = 54;
    #[c2rust::src_loc = "185:5"]
    pub const _SC_PII: C2RustUnnamed = 53;
    #[c2rust::src_loc = "182:5"]
    pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
    #[c2rust::src_loc = "180:5"]
    pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
    #[c2rust::src_loc = "178:5"]
    pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
    #[c2rust::src_loc = "176:5"]
    pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
    #[c2rust::src_loc = "174:5"]
    pub const _SC_2_C_DEV: C2RustUnnamed = 48;
    #[c2rust::src_loc = "172:5"]
    pub const _SC_2_C_BIND: C2RustUnnamed = 47;
    #[c2rust::src_loc = "170:5"]
    pub const _SC_2_VERSION: C2RustUnnamed = 46;
    #[c2rust::src_loc = "167:5"]
    pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
    #[c2rust::src_loc = "165:5"]
    pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
    #[c2rust::src_loc = "163:5"]
    pub const _SC_LINE_MAX: C2RustUnnamed = 43;
    #[c2rust::src_loc = "161:5"]
    pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
    #[c2rust::src_loc = "159:5"]
    pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
    #[c2rust::src_loc = "157:5"]
    pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
    #[c2rust::src_loc = "155:5"]
    pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
    #[c2rust::src_loc = "153:5"]
    pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
    #[c2rust::src_loc = "151:5"]
    pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
    #[c2rust::src_loc = "149:5"]
    pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
    #[c2rust::src_loc = "144:5"]
    pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
    #[c2rust::src_loc = "142:5"]
    pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
    #[c2rust::src_loc = "140:5"]
    pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
    #[c2rust::src_loc = "138:5"]
    pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
    #[c2rust::src_loc = "136:5"]
    pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
    #[c2rust::src_loc = "131:5"]
    pub const _SC_VERSION: C2RustUnnamed = 29;
    #[c2rust::src_loc = "129:5"]
    pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
    #[c2rust::src_loc = "127:5"]
    pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
    #[c2rust::src_loc = "125:5"]
    pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
    #[c2rust::src_loc = "123:5"]
    pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
    #[c2rust::src_loc = "121:5"]
    pub const _SC_AIO_MAX: C2RustUnnamed = 24;
    #[c2rust::src_loc = "119:5"]
    pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
    #[c2rust::src_loc = "117:5"]
    pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
    #[c2rust::src_loc = "115:5"]
    pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
    #[c2rust::src_loc = "113:5"]
    pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
    #[c2rust::src_loc = "111:5"]
    pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
    #[c2rust::src_loc = "109:5"]
    pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
    #[c2rust::src_loc = "107:5"]
    pub const _SC_MEMLOCK: C2RustUnnamed = 17;
    #[c2rust::src_loc = "105:5"]
    pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
    #[c2rust::src_loc = "103:5"]
    pub const _SC_FSYNC: C2RustUnnamed = 15;
    #[c2rust::src_loc = "101:5"]
    pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
    #[c2rust::src_loc = "99:5"]
    pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
    #[c2rust::src_loc = "97:5"]
    pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
    #[c2rust::src_loc = "95:5"]
    pub const _SC_TIMERS: C2RustUnnamed = 11;
    #[c2rust::src_loc = "93:5"]
    pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
    #[c2rust::src_loc = "91:5"]
    pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
    #[c2rust::src_loc = "89:5"]
    pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
    #[c2rust::src_loc = "87:5"]
    pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
    #[c2rust::src_loc = "85:5"]
    pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
    #[c2rust::src_loc = "83:5"]
    pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
    #[c2rust::src_loc = "81:5"]
    pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
    #[c2rust::src_loc = "79:5"]
    pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
    #[c2rust::src_loc = "77:5"]
    pub const _SC_CLK_TCK: C2RustUnnamed = 2;
    #[c2rust::src_loc = "75:5"]
    pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
    #[c2rust::src_loc = "73:5"]
    pub const _SC_ARG_MAX: C2RustUnnamed = 0;
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
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
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:27"]
pub mod base_h {
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/sys/mman.h:27"]
pub mod mman_h {
    #[c2rust::src_loc = "44:9"]
    pub const MAP_FAILED: *mut ::core::ffi::c_void =
        -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "61:1"]
        pub fn mmap(
            __addr: *mut ::core::ffi::c_void,
            __len: size_t,
            __prot: ::core::ffi::c_int,
            __flags: ::core::ffi::c_int,
            __fd: ::core::ffi::c_int,
            __offset: __off64_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "76:1"]
        pub fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "94:1"]
        pub fn madvise(
            __addr: *mut ::core::ffi::c_void,
            __len: size_t,
            __advice: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "266:9"]
    pub const X264_CSP_V210: ::core::ffi::c_int = 0xb as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
pub mod osdep_h {
    #[c2rust::src_loc = "317:9"]
    pub const NATIVE_ALIGN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/unistd.h:27"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "640:1"]
        pub fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/mman-linux.h:27"]
pub mod mman_linux_h {
    #[c2rust::src_loc = "32:9"]
    pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "43:9"]
    pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const MAP_FIXED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "91:10"]
    pub const MADV_WILLNEED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::base_h::{x264_free, x264_malloc};
pub use self::confname_h::{
    C2RustUnnamed, _SC_2_CHAR_TERM, _SC_2_C_BIND, _SC_2_C_DEV, _SC_2_C_VERSION, _SC_2_FORT_DEV,
    _SC_2_FORT_RUN, _SC_2_LOCALEDEF, _SC_2_PBS, _SC_2_PBS_ACCOUNTING, _SC_2_PBS_CHECKPOINT,
    _SC_2_PBS_LOCATE, _SC_2_PBS_MESSAGE, _SC_2_PBS_TRACK, _SC_2_SW_DEV, _SC_2_UPE, _SC_2_VERSION,
    _SC_ADVISORY_INFO, _SC_AIO_LISTIO_MAX, _SC_AIO_MAX, _SC_AIO_PRIO_DELTA_MAX, _SC_ARG_MAX,
    _SC_ASYNCHRONOUS_IO, _SC_ATEXIT_MAX, _SC_AVPHYS_PAGES, _SC_BARRIERS, _SC_BASE, _SC_BC_BASE_MAX,
    _SC_BC_DIM_MAX, _SC_BC_SCALE_MAX, _SC_BC_STRING_MAX, _SC_CHARCLASS_NAME_MAX, _SC_CHAR_BIT,
    _SC_CHAR_MAX, _SC_CHAR_MIN, _SC_CHILD_MAX, _SC_CLK_TCK, _SC_CLOCK_SELECTION,
    _SC_COLL_WEIGHTS_MAX, _SC_CPUTIME, _SC_C_LANG_SUPPORT, _SC_C_LANG_SUPPORT_R,
    _SC_DELAYTIMER_MAX, _SC_DEVICE_IO, _SC_DEVICE_SPECIFIC, _SC_DEVICE_SPECIFIC_R,
    _SC_EQUIV_CLASS_MAX, _SC_EXPR_NEST_MAX, _SC_FD_MGMT, _SC_FIFO, _SC_FILE_ATTRIBUTES,
    _SC_FILE_LOCKING, _SC_FILE_SYSTEM, _SC_FSYNC, _SC_GETGR_R_SIZE_MAX, _SC_GETPW_R_SIZE_MAX,
    _SC_HOST_NAME_MAX, _SC_INT_MAX, _SC_INT_MIN, _SC_IOV_MAX, _SC_IPV6, _SC_JOB_CONTROL,
    _SC_LEVEL1_DCACHE_ASSOC, _SC_LEVEL1_DCACHE_LINESIZE, _SC_LEVEL1_DCACHE_SIZE,
    _SC_LEVEL1_ICACHE_ASSOC, _SC_LEVEL1_ICACHE_LINESIZE, _SC_LEVEL1_ICACHE_SIZE,
    _SC_LEVEL2_CACHE_ASSOC, _SC_LEVEL2_CACHE_LINESIZE, _SC_LEVEL2_CACHE_SIZE,
    _SC_LEVEL3_CACHE_ASSOC, _SC_LEVEL3_CACHE_LINESIZE, _SC_LEVEL3_CACHE_SIZE,
    _SC_LEVEL4_CACHE_ASSOC, _SC_LEVEL4_CACHE_LINESIZE, _SC_LEVEL4_CACHE_SIZE, _SC_LINE_MAX,
    _SC_LOGIN_NAME_MAX, _SC_LONG_BIT, _SC_MAPPED_FILES, _SC_MB_LEN_MAX, _SC_MEMLOCK,
    _SC_MEMLOCK_RANGE, _SC_MEMORY_PROTECTION, _SC_MESSAGE_PASSING, _SC_MINSIGSTKSZ,
    _SC_MONOTONIC_CLOCK, _SC_MQ_OPEN_MAX, _SC_MQ_PRIO_MAX, _SC_MULTI_PROCESS, _SC_NETWORKING,
    _SC_NGROUPS_MAX, _SC_NL_ARGMAX, _SC_NL_LANGMAX, _SC_NL_MSGMAX, _SC_NL_NMAX, _SC_NL_SETMAX,
    _SC_NL_TEXTMAX, _SC_NPROCESSORS_CONF, _SC_NPROCESSORS_ONLN, _SC_NZERO, _SC_OPEN_MAX,
    _SC_PAGESIZE, _SC_PASS_MAX, _SC_PHYS_PAGES, _SC_PII, _SC_PII_INTERNET, _SC_PII_INTERNET_DGRAM,
    _SC_PII_INTERNET_STREAM, _SC_PII_OSI, _SC_PII_OSI_CLTS, _SC_PII_OSI_COTS, _SC_PII_OSI_M,
    _SC_PII_SOCKET, _SC_PII_XTI, _SC_PIPE, _SC_POLL, _SC_PRIORITIZED_IO, _SC_PRIORITY_SCHEDULING,
    _SC_RAW_SOCKETS, _SC_READER_WRITER_LOCKS, _SC_REALTIME_SIGNALS, _SC_REGEXP, _SC_REGEX_VERSION,
    _SC_RE_DUP_MAX, _SC_RTSIG_MAX, _SC_SAVED_IDS, _SC_SCHAR_MAX, _SC_SCHAR_MIN, _SC_SELECT,
    _SC_SEMAPHORES, _SC_SEM_NSEMS_MAX, _SC_SEM_VALUE_MAX, _SC_SHARED_MEMORY_OBJECTS, _SC_SHELL,
    _SC_SHRT_MAX, _SC_SHRT_MIN, _SC_SIGNALS, _SC_SIGQUEUE_MAX, _SC_SIGSTKSZ, _SC_SINGLE_PROCESS,
    _SC_SPAWN, _SC_SPIN_LOCKS, _SC_SPORADIC_SERVER, _SC_SSIZE_MAX, _SC_SS_REPL_MAX, _SC_STREAMS,
    _SC_STREAM_MAX, _SC_SYMLOOP_MAX, _SC_SYNCHRONIZED_IO, _SC_SYSTEM_DATABASE,
    _SC_SYSTEM_DATABASE_R, _SC_THREADS, _SC_THREAD_ATTR_STACKADDR, _SC_THREAD_ATTR_STACKSIZE,
    _SC_THREAD_CPUTIME, _SC_THREAD_DESTRUCTOR_ITERATIONS, _SC_THREAD_KEYS_MAX,
    _SC_THREAD_PRIORITY_SCHEDULING, _SC_THREAD_PRIO_INHERIT, _SC_THREAD_PRIO_PROTECT,
    _SC_THREAD_PROCESS_SHARED, _SC_THREAD_ROBUST_PRIO_INHERIT, _SC_THREAD_ROBUST_PRIO_PROTECT,
    _SC_THREAD_SAFE_FUNCTIONS, _SC_THREAD_SPORADIC_SERVER, _SC_THREAD_STACK_MIN,
    _SC_THREAD_THREADS_MAX, _SC_TIMEOUTS, _SC_TIMERS, _SC_TIMER_MAX, _SC_TRACE,
    _SC_TRACE_EVENT_FILTER, _SC_TRACE_EVENT_NAME_MAX, _SC_TRACE_INHERIT, _SC_TRACE_LOG,
    _SC_TRACE_NAME_MAX, _SC_TRACE_SYS_MAX, _SC_TRACE_USER_EVENT_MAX, _SC_TTY_NAME_MAX,
    _SC_TYPED_MEMORY_OBJECTS, _SC_TZNAME_MAX, _SC_T_IOV_MAX, _SC_UCHAR_MAX, _SC_UINT_MAX,
    _SC_UIO_MAXIOV, _SC_ULONG_MAX, _SC_USER_GROUPS, _SC_USER_GROUPS_R, _SC_USHRT_MAX,
    _SC_V6_ILP32_OFF32, _SC_V6_ILP32_OFFBIG, _SC_V6_LP64_OFF64, _SC_V6_LPBIG_OFFBIG,
    _SC_V7_ILP32_OFF32, _SC_V7_ILP32_OFFBIG, _SC_V7_LP64_OFF64, _SC_V7_LPBIG_OFFBIG, _SC_VERSION,
    _SC_WORD_BIT, _SC_XBS5_ILP32_OFF32, _SC_XBS5_ILP32_OFFBIG, _SC_XBS5_LP64_OFF64,
    _SC_XBS5_LPBIG_OFFBIG, _SC_XOPEN_CRYPT, _SC_XOPEN_ENH_I18N, _SC_XOPEN_LEGACY,
    _SC_XOPEN_REALTIME, _SC_XOPEN_REALTIME_THREADS, _SC_XOPEN_SHM, _SC_XOPEN_STREAMS,
    _SC_XOPEN_UNIX, _SC_XOPEN_VERSION, _SC_XOPEN_XCU_VERSION, _SC_XOPEN_XPG2, _SC_XOPEN_XPG3,
    _SC_XOPEN_XPG4,
};
pub use self::input_h::{
    cli_image_t, cli_mmap_t, cli_pic_t, x264_cli_csp_t, X264_CSP_CLI_MAX, X264_CSP_OTHER,
};
pub use self::mman_h::{madvise, mmap, munmap, MAP_FAILED};
pub use self::mman_linux_h::{MADV_WILLNEED, MAP_FIXED, MAP_PRIVATE, PROT_READ};
pub use self::osdep_h::NATIVE_ALIGN;
use self::stat_h::fstat;
pub use self::stdint_h::{intptr_t, SIZE_MAX};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint64_t, uint8_t};
use self::stdio_h::fileno;
use self::string_h::memset;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __int64_t, __mode_t, __nlink_t, __off64_t,
    __off_t, __syscall_slong_t, __time_t, __uid_t, __uint64_t, __uint8_t,
};
use self::unistd_h::sysconf;
pub use self::x264_h::{
    X264_CSP_HIGH_DEPTH, X264_CSP_MASK, X264_CSP_MAX, X264_CSP_NONE, X264_CSP_V210,
};
pub use self::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "36:22"]
pub static mut x264_cli_csps: [x264_cli_csp_t; 17] = [
    x264_cli_csp_t {
        name: 0 as *const ::core::ffi::c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i400\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i420\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 2 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv12\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 2 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv12\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 2 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 2 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv21\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 2 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 2 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i422\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv16\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.5f64 as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv16\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 2 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
                0.,
            ],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yuyv\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [2 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"uyvy\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [2 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 2 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    x264_cli_csp_t {
        name: 0 as *const ::core::ffi::c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i444\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv24\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 3 as ::core::ffi::c_int,
            width: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            height: [
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                1 as ::core::ffi::c_int as ::core::ffi::c_float,
                0.,
            ],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"bgr\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [3 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"bgra\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [4 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"rgb\0" as *const u8 as *const ::core::ffi::c_char,
            planes: 1 as ::core::ffi::c_int,
            width: [3 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            height: [1 as ::core::ffi::c_int as ::core::ffi::c_float, 0., 0., 0.],
            mod_width: 1 as ::core::ffi::c_int,
            mod_height: 1 as ::core::ffi::c_int,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn x264_cli_csp_is_invalid(
    mut csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut csp_mask: ::core::ffi::c_int = csp & X264_CSP_MASK;
    return (csp_mask <= X264_CSP_NONE
        || csp_mask >= X264_CSP_CLI_MAX
        || csp_mask == X264_CSP_V210
        || csp & X264_CSP_OTHER != 0) as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn x264_cli_csp_depth_factor(
    mut csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return if csp & X264_CSP_HIGH_DEPTH != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn x264_cli_pic_plane_size(
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut plane: ::core::ffi::c_int,
) -> int64_t {
    let mut csp_mask: ::core::ffi::c_int = csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid(csp) != 0
        || plane < 0 as ::core::ffi::c_int
        || plane >= x264_cli_csps[csp_mask as usize].planes
    {
        return 0 as int64_t;
    }
    let mut size: int64_t = width as int64_t * height as int64_t;
    size = (size as ::core::ffi::c_float
        * (x264_cli_csps[csp_mask as usize].width[plane as usize]
            * x264_cli_csps[csp_mask as usize].height[plane as usize])) as int64_t;
    size *= x264_cli_csp_depth_factor(csp) as int64_t;
    return size;
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
pub unsafe extern "C" fn x264_cli_pic_size(
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> int64_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as int64_t;
    }
    let mut size: int64_t = 0 as int64_t;
    let mut csp_mask: ::core::ffi::c_int = csp & X264_CSP_MASK;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < x264_cli_csps[csp_mask as usize].planes {
        size += x264_cli_pic_plane_size(csp, width, height, i);
        i += 1;
    }
    return size;
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn cli_pic_init_internal(
    mut pic: *mut cli_pic_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut alloc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
    let mut csp_mask: ::core::ffi::c_int = csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid(csp) != 0 {
        (*pic).img.planes = 0 as ::core::ffi::c_int;
    } else {
        (*pic).img.planes = x264_cli_csps[csp_mask as usize].planes;
    }
    (*pic).img.csp = csp;
    (*pic).img.width = width;
    (*pic).img.height = height;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pic).img.planes {
        let mut stride: ::core::ffi::c_int = (width as ::core::ffi::c_float
            * x264_cli_csps[csp_mask as usize].width[i as usize])
            as ::core::ffi::c_int;
        stride *= x264_cli_csp_depth_factor(csp);
        stride = stride + (align - 1 as ::core::ffi::c_int) & !(align - 1 as ::core::ffi::c_int);
        (*pic).img.stride[i as usize] = stride;
        if alloc != 0 {
            let mut size: int64_t = (height as ::core::ffi::c_float
                * x264_cli_csps[csp_mask as usize].height[i as usize])
                as int64_t
                * stride as int64_t;
            (*pic).img.plane[i as usize] = x264_malloc(size) as *mut uint8_t;
            if (*pic).img.plane[i as usize].is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "120:1"]
pub unsafe extern "C" fn x264_cli_pic_alloc(
    mut pic: *mut cli_pic_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn x264_cli_pic_alloc_aligned(
    mut pic: *mut cli_pic_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        NATIVE_ALIGN,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn x264_cli_pic_init_noalloc(
    mut pic: *mut cli_pic_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn x264_cli_pic_clean(mut pic: *mut cli_pic_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pic).img.planes {
        x264_free((*pic).img.plane[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn x264_cli_get_csp(mut csp: ::core::ffi::c_int) -> *const x264_cli_csp_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as *const x264_cli_csp_t;
    }
    return x264_cli_csps
        .as_ptr()
        .offset((csp & X264_CSP_MASK) as isize);
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn x264_cli_mmap_init(
    mut h: *mut cli_mmap_t,
    mut fh: *mut FILE,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = fileno(fh);
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
    if fstat(fd, &mut file_stat) == 0 {
        (*h).file_size = file_stat.st_size as int64_t;
        (*h).align_mask = (sysconf(_SC_PAGESIZE as ::core::ffi::c_int) - 1 as ::core::ffi::c_long)
            as ::core::ffi::c_int;
        (*h).fd = fd;
        return ((*h).align_mask < 0 as ::core::ffi::c_int || fd < 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "183:9"]
pub const MMAP_PADDING: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "185:1"]
pub unsafe extern "C" fn x264_cli_mmap(
    mut h: *mut cli_mmap_t,
    mut offset: int64_t,
    mut size: int64_t,
) -> *mut ::core::ffi::c_void {
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    let mut align: ::core::ffi::c_int = (offset & (*h).align_mask as int64_t) as ::core::ffi::c_int;
    if offset < 0 as int64_t
        || size < 0 as int64_t
        || size as uint64_t
            > (SIZE_MAX as uint64_t)
                .wrapping_sub(MMAP_PADDING as uint64_t)
                .wrapping_sub(align as uint64_t)
    {
        return NULL;
    }
    offset -= align as int64_t;
    size += align as int64_t;
    let mut padded_size: size_t = (size + MMAP_PADDING as int64_t) as size_t;
    base = mmap(
        NULL,
        padded_size,
        PROT_READ,
        MAP_PRIVATE,
        (*h).fd,
        offset as __off64_t,
    ) as *mut uint8_t;
    if base != MAP_FAILED as *mut uint8_t {
        madvise(
            base as *mut ::core::ffi::c_void,
            size as size_t,
            MADV_WILLNEED,
        );
        let mut aligned_size: size_t =
            padded_size.wrapping_sub(1 as size_t) & !(*h).align_mask as size_t;
        if (offset as size_t).wrapping_add(aligned_size) >= (*h).file_size as size_t {
            mmap(
                base.offset(aligned_size as isize) as *mut ::core::ffi::c_void,
                padded_size.wrapping_sub(aligned_size),
                PROT_READ,
                MAP_PRIVATE | MAP_FIXED,
                (*h).fd,
                offset as __off64_t + size as __off64_t - 1 as __off64_t
                    & !(*h).align_mask as __off64_t,
            );
        }
        return base.offset(align as isize) as *mut ::core::ffi::c_void;
    }
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "254:1"]
pub unsafe extern "C" fn x264_cli_munmap(
    mut h: *mut cli_mmap_t,
    mut addr: *mut ::core::ffi::c_void,
    mut size: int64_t,
) -> ::core::ffi::c_int {
    let mut base: *mut ::core::ffi::c_void =
        (addr as intptr_t & !(*h).align_mask as intptr_t) as *mut ::core::ffi::c_void;
    if size < 0 as int64_t
        || size as ::core::ffi::c_ulong
            > SIZE_MAX
                .wrapping_sub(MMAP_PADDING as ::core::ffi::c_ulong)
                .wrapping_sub((addr as intptr_t - base as intptr_t) as ::core::ffi::c_ulong)
    {
        return -(1 as ::core::ffi::c_int);
    }
    return munmap(
        base,
        (size + MMAP_PADDING as int64_t + addr as int64_t - base as int64_t) as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "269:1"]
pub unsafe extern "C" fn x264_cli_mmap_close(mut h: *mut cli_mmap_t) {}
