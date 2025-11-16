use ::c2rust_bitfields;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "3:9"]
    pub const BIT_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:28"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = i8;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = i16;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = i32;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:28"]
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
    use super::types_h::{__off_t, __off64_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:28"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:28"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:28"]
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
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t,
    };
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/stdint.h:28"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
    #[c2rust::src_loc = "117:10"]
    pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
    #[c2rust::src_loc = "118:10"]
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:28"]
pub mod atomic_wide_counter_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "25:9"]
    pub union __atomic_wide_counter {
        pub __value64: ::core::ffi::c_ulonglong,
        pub __value32: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:3"]
    pub struct C2RustUnnamed {
        pub __low: ::core::ffi::c_uint,
        pub __high: ::core::ffi::c_uint,
    }
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:28"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "51:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:8"]
    pub struct __pthread_cond_s {
        pub __wseq: __atomic_wide_counter,
        pub __g1_start: __atomic_wide_counter,
        pub __g_size: [::core::ffi::c_uint; 2],
        pub __g1_orig_size: ::core::ffi::c_uint,
        pub __wrefs: ::core::ffi::c_uint,
        pub __g_signals: [::core::ffi::c_uint; 2],
        pub __unused_initialized_1: ::core::ffi::c_uint,
        pub __unused_initialized_2: ::core::ffi::c_uint,
    }
    use super::atomic_wide_counter_h::__atomic_wide_counter;
}
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:28"]
pub mod struct_mutex_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct __pthread_mutex_s {
        pub __lock: ::core::ffi::c_int,
        pub __count: ::core::ffi::c_uint,
        pub __owner: ::core::ffi::c_int,
        pub __nusers: ::core::ffi::c_uint,
        pub __kind: ::core::ffi::c_int,
        pub __spins: ::core::ffi::c_short,
        pub __elision: ::core::ffi::c_short,
        pub __list: __pthread_list_t,
    }
    use super::thread_shared_types_h::__pthread_list_t;
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:28"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "27:1"]
    pub type pthread_t = ::core::ffi::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:9"]
    pub union pthread_condattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:9"]
    pub union pthread_cond_t {
        pub __data: __pthread_cond_s,
        pub __size: [::core::ffi::c_char; 48],
        pub __align: ::core::ffi::c_longlong,
    }
    use super::struct_mutex_h::__pthread_mutex_s;
    use super::thread_shared_types_h::__pthread_cond_s;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:28"]
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "270:8"]
    pub struct x264_t {
        pub param: x264_param_t,
        pub api: *mut ::core::ffi::c_void,
        pub thread: [*mut x264_t; 129],
        pub lookahead_thread: [*mut x264_t; 16],
        pub b_thread_active: ::core::ffi::c_int,
        pub i_thread_phase: ::core::ffi::c_int,
        pub i_thread_idx: ::core::ffi::c_int,
        pub i_threadslice_start: ::core::ffi::c_int,
        pub i_threadslice_end: ::core::ffi::c_int,
        pub i_threadslice_pass: ::core::ffi::c_int,
        pub threadpool: *mut x264_threadpool_t,
        pub lookaheadpool: *mut x264_threadpool_t,
        pub mutex: pthread_mutex_t,
        pub cv: pthread_cond_t,
        pub out: C2RustUnnamed_18,
        pub nal_buffer: *mut uint8_t,
        pub nal_buffer_size: ::core::ffi::c_int,
        pub reconfig_h: *mut x264_t,
        pub reconfig: ::core::ffi::c_int,
        pub i_frame: ::core::ffi::c_int,
        pub i_frame_num: ::core::ffi::c_int,
        pub i_thread_frames: ::core::ffi::c_int,
        pub i_nal_type: ::core::ffi::c_int,
        pub i_nal_ref_idc: ::core::ffi::c_int,
        pub i_disp_fields: int64_t,
        pub i_disp_fields_last_frame: ::core::ffi::c_int,
        pub i_prev_duration: int64_t,
        pub i_coded_fields: int64_t,
        pub i_cpb_delay: int64_t,
        pub i_coded_fields_lookahead: int64_t,
        pub i_cpb_delay_lookahead: int64_t,
        pub i_cpb_delay_pir_offset: int64_t,
        pub i_cpb_delay_pir_offset_next: int64_t,
        pub b_queued_intra_refresh: ::core::ffi::c_int,
        pub i_last_idr_pts: int64_t,
        pub i_idr_pic_id: ::core::ffi::c_int,
        pub dequant4_mf: [*mut [::core::ffi::c_int; 16]; 4],
        pub dequant8_mf: [*mut [::core::ffi::c_int; 64]; 4],
        pub unquant4_mf: [*mut [::core::ffi::c_int; 16]; 4],
        pub unquant8_mf: [*mut [::core::ffi::c_int; 64]; 4],
        pub quant4_mf: [*mut [udctcoef; 16]; 4],
        pub quant8_mf: [*mut [udctcoef; 64]; 4],
        pub quant4_bias: [*mut [udctcoef; 16]; 4],
        pub quant8_bias: [*mut [udctcoef; 64]; 4],
        pub quant4_bias0: [*mut [udctcoef; 16]; 4],
        pub quant8_bias0: [*mut [udctcoef; 64]; 4],
        pub nr_offset_emergency: *mut [[udctcoef; 64]; 4],
        pub cost_mv: [*mut uint16_t; 82],
        pub cost_mv_fpel: [[*mut uint16_t; 4]; 82],
        pub cost_table: *mut C2RustUnnamed_17,
        pub chroma_qp_table: *const uint8_t,
        pub sh: x264_slice_header_t,
        pub sps: [x264_sps_t; 1],
        pub pps: [x264_pps_t; 1],
        pub b_sh_backup: ::core::ffi::c_int,
        pub sh_backup: x264_slice_header_t,
        pub cabac: x264_cabac_t,
        pub frames: C2RustUnnamed_11,
        pub fenc: *mut x264_frame_t,
        pub fdec: *mut x264_frame_t,
        pub i_ref: [::core::ffi::c_int; 2],
        pub fref: [[*mut x264_frame_t; 19]; 2],
        pub fref_nearest: [*mut x264_frame_t; 2],
        pub b_ref_reorder: [::core::ffi::c_int; 2],
        pub initial_cpb_removal_delay: ::core::ffi::c_int,
        pub initial_cpb_removal_delay_offset: ::core::ffi::c_int,
        pub i_reordered_pts_delay: int64_t,
        pub dct: C2RustUnnamed_10,
        pub mb: C2RustUnnamed_7,
        pub rc: *mut x264_ratecontrol_t,
        pub stat: C2RustUnnamed_6,
        pub nr_offset: *mut [udctcoef; 64],
        pub nr_residual_sum: *mut [uint32_t; 64],
        pub nr_count: *mut uint32_t,
        pub nr_offset_denoise: [[udctcoef; 64]; 4],
        pub nr_residual_sum_buf: [[[uint32_t; 64]; 4]; 2],
        pub nr_count_buf: [[uint32_t; 4]; 2],
        pub luma2chroma_pixel: [uint8_t; 7],
        pub scratch_buffer: *mut ::core::ffi::c_void,
        pub scratch_buffer2: *mut ::core::ffi::c_void,
        pub intra_border_backup: [[*mut pixel; 3]; 5],
        pub deblock_strength: [*mut [[[uint8_t; 4]; 8]; 2]; 2],
        pub predict_16x16: [x264_predict_t; 7],
        pub predict_8x8: [x264_predict8x8_t; 12],
        pub predict_4x4: [x264_predict_t; 12],
        pub predict_chroma: [x264_predict_t; 7],
        pub predict_8x8c: [x264_predict_t; 7],
        pub predict_8x16c: [x264_predict_t; 7],
        pub predict_8x8_filter: x264_predict_8x8_filter_t,
        pub pixf: x264_pixel_function_t,
        pub mc: x264_mc_functions_t,
        pub dctf: x264_dct_function_t,
        pub zigzagf: x264_zigzag_function_t,
        pub zigzagf_interlaced: x264_zigzag_function_t,
        pub zigzagf_progressive: x264_zigzag_function_t,
        pub quantf: x264_quant_function_t,
        pub loopf: x264_deblock_function_t,
        pub bsf: x264_bitstream_function_t,
        pub lookahead: *mut x264_lookahead_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "217:16"]
    pub struct x264_lookahead_t {
        pub b_exit_thread: uint8_t,
        pub b_thread_active: uint8_t,
        pub b_analyse_keyframe: uint8_t,
        pub i_last_keyframe: ::core::ffi::c_int,
        pub i_slicetype_length: ::core::ffi::c_int,
        pub last_nonb: *mut x264_frame_t,
        pub thread_handle: pthread_t,
        pub ifbuf: x264_sync_frame_list_t,
        pub next: x264_sync_frame_list_t,
        pub ofbuf: x264_sync_frame_list_t,
    }
    #[c2rust::src_loc = "94:5"]
    pub type pixel = uint16_t;
    #[c2rust::src_loc = "96:5"]
    pub type dctcoef = int32_t;
    #[c2rust::src_loc = "97:5"]
    pub type udctcoef = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "685:5"]
    pub struct C2RustUnnamed_6 {
        pub i_frame_count: [::core::ffi::c_int; 3],
        pub i_frame_size: [int64_t; 3],
        pub f_frame_qp: [::core::ffi::c_double; 3],
        pub i_consecutive_bframes: [::core::ffi::c_int; 17],
        pub f_ssd_global: [::core::ffi::c_double; 3],
        pub f_psnr_average: [::core::ffi::c_double; 3],
        pub f_psnr_mean_y: [::core::ffi::c_double; 3],
        pub f_psnr_mean_u: [::core::ffi::c_double; 3],
        pub f_psnr_mean_v: [::core::ffi::c_double; 3],
        pub f_ssim_mean_y: [::core::ffi::c_double; 3],
        pub f_frame_duration: [::core::ffi::c_double; 3],
        pub i_mb_count: [[int64_t; 19]; 3],
        pub i_mb_partition: [[int64_t; 17]; 2],
        pub i_mb_count_8x8dct: [int64_t; 2],
        pub i_mb_count_ref: [[[int64_t; 32]; 2]; 2],
        pub i_mb_cbp: [int64_t; 6],
        pub i_mb_pred_mode: [[int64_t; 13]; 4],
        pub i_mb_field: [int64_t; 3],
        pub i_direct_score: [::core::ffi::c_int; 2],
        pub i_direct_frames: [::core::ffi::c_int; 2],
        pub i_wpred: [::core::ffi::c_int; 2],
        pub frame: x264_frame_stat_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "243:9"]
    pub struct x264_frame_stat_t {
        pub i_mv_bits: ::core::ffi::c_int,
        pub i_tex_bits: ::core::ffi::c_int,
        pub i_misc_bits: ::core::ffi::c_int,
        pub i_mb_count: [::core::ffi::c_int; 19],
        pub i_mb_count_i: ::core::ffi::c_int,
        pub i_mb_count_p: ::core::ffi::c_int,
        pub i_mb_count_skip: ::core::ffi::c_int,
        pub i_mb_count_8x8dct: [::core::ffi::c_int; 2],
        pub i_mb_count_ref: [[::core::ffi::c_int; 32]; 2],
        pub i_mb_partition: [::core::ffi::c_int; 17],
        pub i_mb_cbp: [::core::ffi::c_int; 6],
        pub i_mb_pred_mode: [[::core::ffi::c_int; 13]; 4],
        pub i_mb_field: [::core::ffi::c_int; 3],
        pub i_direct_score: [::core::ffi::c_int; 2],
        pub i_ssd: [int64_t; 3],
        pub f_ssim: ::core::ffi::c_double,
        pub i_ssim_cnt: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "438:5"]
    pub struct C2RustUnnamed_7 {
        pub i_mb_width: ::core::ffi::c_int,
        pub i_mb_height: ::core::ffi::c_int,
        pub i_mb_count: ::core::ffi::c_int,
        pub chroma_h_shift: ::core::ffi::c_int,
        pub chroma_v_shift: ::core::ffi::c_int,
        pub i_mb_stride: ::core::ffi::c_int,
        pub i_b8_stride: ::core::ffi::c_int,
        pub i_b4_stride: ::core::ffi::c_int,
        pub left_b8: [::core::ffi::c_int; 2],
        pub left_b4: [::core::ffi::c_int; 2],
        pub i_mb_x: ::core::ffi::c_int,
        pub i_mb_y: ::core::ffi::c_int,
        pub i_mb_xy: ::core::ffi::c_int,
        pub i_b8_xy: ::core::ffi::c_int,
        pub i_b4_xy: ::core::ffi::c_int,
        pub i_me_method: ::core::ffi::c_int,
        pub i_subpel_refine: ::core::ffi::c_int,
        pub b_chroma_me: ::core::ffi::c_int,
        pub b_trellis: ::core::ffi::c_int,
        pub b_noise_reduction: ::core::ffi::c_int,
        pub b_dct_decimate: ::core::ffi::c_int,
        pub i_psy_rd: ::core::ffi::c_int,
        pub i_psy_trellis: ::core::ffi::c_int,
        pub b_interlaced: ::core::ffi::c_int,
        pub b_adaptive_mbaff: ::core::ffi::c_int,
        pub mv_min: [::core::ffi::c_int; 2],
        pub mv_max: [::core::ffi::c_int; 2],
        pub mv_miny_row: [::core::ffi::c_int; 3],
        pub mv_maxy_row: [::core::ffi::c_int; 3],
        pub mv_min_spel: [::core::ffi::c_int; 2],
        pub mv_max_spel: [::core::ffi::c_int; 2],
        pub mv_miny_spel_row: [::core::ffi::c_int; 3],
        pub mv_maxy_spel_row: [::core::ffi::c_int; 3],
        pub mv_limit_fpel: [[int16_t; 2]; 2],
        pub mv_miny_fpel_row: [::core::ffi::c_int; 3],
        pub mv_maxy_fpel_row: [::core::ffi::c_int; 3],
        pub i_neighbour: ::core::ffi::c_uint,
        pub i_neighbour8: [::core::ffi::c_uint; 4],
        pub i_neighbour4: [::core::ffi::c_uint; 16],
        pub i_neighbour_intra: ::core::ffi::c_uint,
        pub i_neighbour_frame: ::core::ffi::c_uint,
        pub i_mb_type_top: ::core::ffi::c_int,
        pub i_mb_type_left: [::core::ffi::c_int; 2],
        pub i_mb_type_topleft: ::core::ffi::c_int,
        pub i_mb_type_topright: ::core::ffi::c_int,
        pub i_mb_prev_xy: ::core::ffi::c_int,
        pub i_mb_left_xy: [::core::ffi::c_int; 2],
        pub i_mb_top_xy: ::core::ffi::c_int,
        pub i_mb_topleft_xy: ::core::ffi::c_int,
        pub i_mb_topright_xy: ::core::ffi::c_int,
        pub i_mb_top_y: ::core::ffi::c_int,
        pub i_mb_topleft_y: ::core::ffi::c_int,
        pub i_mb_topright_y: ::core::ffi::c_int,
        pub left_index_table: *const x264_left_table_t,
        pub i_mb_top_mbpair_xy: ::core::ffi::c_int,
        pub topleft_partition: ::core::ffi::c_int,
        pub b_allow_skip: ::core::ffi::c_int,
        pub field_decoding_flag: ::core::ffi::c_int,
        pub base: *mut uint8_t,
        pub type_0: *mut int8_t,
        pub partition: *mut uint8_t,
        pub qp: *mut int8_t,
        pub cbp: *mut int16_t,
        pub intra4x4_pred_mode: *mut [int8_t; 8],
        pub non_zero_count: *mut [uint8_t; 48],
        pub chroma_pred_mode: *mut int8_t,
        pub mv: [*mut [int16_t; 2]; 2],
        pub mvd: [*mut [[uint8_t; 2]; 8]; 2],
        pub ref_0: [*mut int8_t; 2],
        pub mvr: [[*mut [int16_t; 2]; 32]; 2],
        pub skipbp: *mut int8_t,
        pub mb_transform_size: *mut int8_t,
        pub slice_table: *mut int32_t,
        pub field: *mut uint8_t,
        pub p_weight_buf: [*mut pixel; 16],
        pub i_type: ::core::ffi::c_int,
        pub i_partition: ::core::ffi::c_int,
        pub i_sub_partition: [uint8_t; 4],
        pub b_transform_8x8: ::core::ffi::c_int,
        pub i_cbp_luma: ::core::ffi::c_int,
        pub i_cbp_chroma: ::core::ffi::c_int,
        pub i_intra16x16_pred_mode: ::core::ffi::c_int,
        pub i_chroma_pred_mode: ::core::ffi::c_int,
        pub i_skip_intra: ::core::ffi::c_int,
        pub b_skip_mc: ::core::ffi::c_int,
        pub b_reencode_mb: ::core::ffi::c_int,
        pub ip_offset: ::core::ffi::c_int,
        pub b_deblock_rdo: ::core::ffi::c_int,
        pub b_overflow: ::core::ffi::c_int,
        pub pic: C2RustUnnamed_9,
        pub cache: C2RustUnnamed_8,
        pub i_qp: ::core::ffi::c_int,
        pub i_chroma_qp: ::core::ffi::c_int,
        pub i_last_qp: ::core::ffi::c_int,
        pub i_last_dqp: ::core::ffi::c_int,
        pub b_variable_qp: ::core::ffi::c_int,
        pub b_lossless: ::core::ffi::c_int,
        pub b_direct_auto_read: ::core::ffi::c_int,
        pub b_direct_auto_write: ::core::ffi::c_int,
        pub i_trellis_lambda2: [[::core::ffi::c_int; 2]; 2],
        pub i_psy_rd_lambda: ::core::ffi::c_int,
        pub i_chroma_lambda2_offset: ::core::ffi::c_int,
        pub dist_scale_factor_buf: [[[[int16_t; 4]; 32]; 2]; 2],
        pub dist_scale_factor: *mut [int16_t; 4],
        pub bipred_weight_buf: [[[[int8_t; 4]; 32]; 2]; 2],
        pub bipred_weight: *mut [int8_t; 4],
        pub map_col_to_list0: [int8_t; 18],
        pub ref_blind_dupe: ::core::ffi::c_int,
        pub deblock_ref_table: [int8_t; 34],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "614:9"]
    pub struct C2RustUnnamed_8 {
        pub intra4x4_pred_mode: [int8_t; 40],
        pub non_zero_count: [uint8_t; 120],
        pub ref_0: [[int8_t; 40]; 2],
        pub mv: [[[int16_t; 2]; 40]; 2],
        pub mvd: [[[uint8_t; 2]; 40]; 2],
        pub skip: [int8_t; 40],
        pub direct_mv: [[[int16_t; 2]; 4]; 2],
        pub direct_ref: [[int8_t; 4]; 2],
        pub direct_partition: ::core::ffi::c_int,
        pub pskip_mv: [int16_t; 2],
        pub i_neighbour_transform_size: ::core::ffi::c_int,
        pub i_neighbour_skip: ::core::ffi::c_int,
        pub i_cbp_top: ::core::ffi::c_int,
        pub i_cbp_left: ::core::ffi::c_int,
        pub topright_mv: [[[int16_t; 2]; 3]; 2],
        pub topright_ref: [[int8_t; 3]; 2],
        pub deblock_strength: *mut [[uint8_t; 4]; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "567:9"]
    pub struct C2RustUnnamed_9 {
        pub fenc_buf: [pixel; 768],
        pub fdec_buf: [pixel; 1728],
        pub i4x4_fdec_buf: [pixel; 256],
        pub i8x8_fdec_buf: [pixel; 256],
        pub i8x8_dct_buf: [[dctcoef; 64]; 3],
        pub i4x4_dct_buf: [[dctcoef; 16]; 15],
        pub i4x4_nnz_buf: [uint32_t; 4],
        pub i8x8_nnz_buf: [uint32_t; 4],
        pub fenc_dct8: [[dctcoef; 64]; 4],
        pub fenc_dct4: [[dctcoef; 16]; 16],
        pub fenc_satd_cache: [uint32_t; 32],
        pub fenc_hadamard_cache: [uint64_t; 9],
        pub i4x4_cbp: ::core::ffi::c_int,
        pub i8x8_cbp: ::core::ffi::c_int,
        pub p_fenc: [*mut pixel; 3],
        pub p_fenc_plane: [*mut pixel; 3],
        pub p_fdec: [*mut pixel; 3],
        pub i_fref: [::core::ffi::c_int; 2],
        pub p_fref: [[[*mut pixel; 12]; 32]; 2],
        pub p_fref_w: [*mut pixel; 32],
        pub p_integral: [[*mut uint16_t; 16]; 2],
        pub i_stride: [::core::ffi::c_int; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "233:16"]
    pub struct x264_left_table_t {
        pub intra: [uint8_t; 4],
        pub nnz: [uint8_t; 4],
        pub nnz_chroma: [uint8_t; 4],
        pub mv: [uint8_t; 4],
        pub ref_0: [uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "428:5"]
    pub struct C2RustUnnamed_10 {
        pub luma16x16_dc: [[dctcoef; 16]; 3],
        pub chroma_dc: [[dctcoef; 8]; 2],
        pub luma8x8: [[dctcoef; 64]; 12],
        pub luma4x4: [[dctcoef; 16]; 48],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "375:5"]
    pub struct C2RustUnnamed_11 {
        pub current: *mut *mut x264_frame_t,
        pub unused: [*mut *mut x264_frame_t; 2],
        pub blank_unused: *mut *mut x264_frame_t,
        pub reference: [*mut x264_frame_t; 18],
        pub i_last_keyframe: ::core::ffi::c_int,
        pub i_last_idr: ::core::ffi::c_int,
        pub i_poc_last_open_gop: ::core::ffi::c_int,
        pub i_input: ::core::ffi::c_int,
        pub i_max_dpb: ::core::ffi::c_int,
        pub i_max_ref0: ::core::ffi::c_int,
        pub i_max_ref1: ::core::ffi::c_int,
        pub i_delay: ::core::ffi::c_int,
        pub i_bframe_delay: ::core::ffi::c_int,
        pub i_bframe_delay_time: int64_t,
        pub i_first_pts: int64_t,
        pub i_prev_reordered_pts: [int64_t; 2],
        pub i_largest_pts: int64_t,
        pub i_second_largest_pts: int64_t,
        pub b_have_lowres: ::core::ffi::c_int,
        pub b_have_sub8x8_esa: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:9"]
    pub struct x264_slice_header_t {
        pub sps: *mut x264_sps_t,
        pub pps: *mut x264_pps_t,
        pub i_type: ::core::ffi::c_int,
        pub i_first_mb: ::core::ffi::c_int,
        pub i_last_mb: ::core::ffi::c_int,
        pub i_pps_id: ::core::ffi::c_int,
        pub i_frame_num: ::core::ffi::c_int,
        pub b_mbaff: ::core::ffi::c_int,
        pub b_field_pic: ::core::ffi::c_int,
        pub b_bottom_field: ::core::ffi::c_int,
        pub i_idr_pic_id: ::core::ffi::c_int,
        pub i_poc: ::core::ffi::c_int,
        pub i_delta_poc_bottom: ::core::ffi::c_int,
        pub i_delta_poc: [::core::ffi::c_int; 2],
        pub i_redundant_pic_cnt: ::core::ffi::c_int,
        pub b_direct_spatial_mv_pred: ::core::ffi::c_int,
        pub b_num_ref_idx_override: ::core::ffi::c_int,
        pub i_num_ref_idx_l0_active: ::core::ffi::c_int,
        pub i_num_ref_idx_l1_active: ::core::ffi::c_int,
        pub b_ref_pic_list_reordering: [::core::ffi::c_int; 2],
        pub ref_pic_list_order: [[C2RustUnnamed_13; 16]; 2],
        pub b_weighted_pred: ::core::ffi::c_int,
        pub weight: [[x264_weight_t; 3]; 32],
        pub i_mmco_remove_from_end: ::core::ffi::c_int,
        pub i_mmco_command_count: ::core::ffi::c_int,
        pub mmco: [C2RustUnnamed_12; 16],
        pub i_cabac_init_idc: ::core::ffi::c_int,
        pub i_qp: ::core::ffi::c_int,
        pub i_qp_delta: ::core::ffi::c_int,
        pub b_sp_for_swidth: ::core::ffi::c_int,
        pub i_qs_delta: ::core::ffi::c_int,
        pub i_disable_deblocking_filter_idc: ::core::ffi::c_int,
        pub i_alpha_c0_offset: ::core::ffi::c_int,
        pub i_beta_offset: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "197:5"]
    pub struct C2RustUnnamed_12 {
        pub i_difference_of_pic_nums: ::core::ffi::c_int,
        pub i_poc: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "185:5"]
    pub struct C2RustUnnamed_13 {
        pub idc: ::core::ffi::c_int,
        pub arg: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "353:5"]
    pub struct C2RustUnnamed_17 {
        pub ref_0: [[[uint16_t; 33]; 3]; 82],
        pub i4x4_mode: [[uint16_t; 17]; 82],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "291:5"]
    pub struct C2RustUnnamed_18 {
        pub i_nal: ::core::ffi::c_int,
        pub i_nals_allocated: ::core::ffi::c_int,
        pub nal: *mut x264_nal_t,
        pub i_bitstream: ::core::ffi::c_int,
        pub p_bitstream: *mut uint8_t,
        pub bs: bs_t,
    }
    #[c2rust::src_loc = "58:9"]
    pub const QP_BD_OFFSET: ::core::ffi::c_int = 6 as ::core::ffi::c_int
        * (BIT_DEPTH - 8 as ::core::ffi::c_int);
    #[c2rust::src_loc = "59:9"]
    pub const QP_MAX_SPEC: ::core::ffi::c_int = 51 as ::core::ffi::c_int + QP_BD_OFFSET;
    #[c2rust::src_loc = "60:9"]
    pub const QP_MAX: ::core::ffi::c_int = QP_MAX_SPEC + 18 as ::core::ffi::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const PIXEL_MAX: ::core::ffi::c_int = ((1 as ::core::ffi::c_int) << BIT_DEPTH)
        - 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "66:9"]
    pub const NALU_OVERHEAD: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "67:9"]
    pub const FILLER_OVERHEAD: ::core::ffi::c_int = NALU_OVERHEAD
        + 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "111:9"]
    pub const SIZEOF_PIXEL: ::core::ffi::c_int = ::core::mem::size_of::<pixel>()
        as ::core::ffi::c_int;
    use super::x264_h::{x264_param_t, x264_nal_t};
    use super::threadpool_h::x264_threadpool_t;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t, pthread_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int32_t, int16_t, int8_t};
    use super::set_h::{x264_sps_t, x264_pps_t};
    use super::cabac_h::x264_cabac_t;
    use super::frame_h::{x264_frame_t, x264_deblock_function_t, x264_sync_frame_list_t};
    use super::predict_h::{x264_predict_t, x264_predict8x8_t, x264_predict_8x8_filter_t};
    use super::pixel_h::x264_pixel_function_t;
    use super::mc_h::{x264_mc_functions_t, x264_weight_t};
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::quant_h::x264_quant_function_t;
    use super::bitstream_h::{x264_bitstream_function_t, bs_t};
    use super::internal::BIT_DEPTH;
    extern "C" {
        #[c2rust::src_loc = "231:16"]
        pub type x264_ratecontrol_t;
        #[c2rust::src_loc = "138:1"]
        pub fn x264_10_log(
            h: *mut x264_t,
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "141:1"]
        pub fn x264_10_cavlc_init(h: *mut x264_t);
        #[c2rust::src_loc = "143:1"]
        pub fn x264_10_cabac_init(h: *mut x264_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:28"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "186:9"]
    pub struct x264_sync_frame_list_t {
        pub list: *mut *mut x264_frame_t,
        pub i_max_size: ::core::ffi::c_int,
        pub i_size: ::core::ffi::c_int,
        pub mutex: pthread_mutex_t,
        pub cv_fill: pthread_cond_t,
        pub cv_empty: pthread_cond_t,
    }
    #[c2rust::src_loc = "37:1"]
    pub type x264_frame_t = x264_frame;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct x264_frame {
        pub base: *mut uint8_t,
        pub i_poc: ::core::ffi::c_int,
        pub i_delta_poc: [::core::ffi::c_int; 2],
        pub i_type: ::core::ffi::c_int,
        pub i_forced_type: ::core::ffi::c_int,
        pub i_qpplus1: ::core::ffi::c_int,
        pub i_pts: int64_t,
        pub i_dts: int64_t,
        pub i_reordered_pts: int64_t,
        pub i_duration: int64_t,
        pub f_duration: ::core::ffi::c_float,
        pub i_cpb_duration: int64_t,
        pub i_cpb_delay: int64_t,
        pub i_dpb_output_delay: int64_t,
        pub param: *mut x264_param_t,
        pub i_frame: ::core::ffi::c_int,
        pub i_coded: ::core::ffi::c_int,
        pub i_field_cnt: int64_t,
        pub i_frame_num: ::core::ffi::c_int,
        pub b_kept_as_ref: ::core::ffi::c_int,
        pub i_pic_struct: ::core::ffi::c_int,
        pub b_keyframe: ::core::ffi::c_int,
        pub b_fdec: uint8_t,
        pub b_last_minigop_bframe: uint8_t,
        pub i_bframes: uint8_t,
        pub f_qp_avg_rc: ::core::ffi::c_float,
        pub f_qp_avg_aq: ::core::ffi::c_float,
        pub f_crf_avg: ::core::ffi::c_float,
        pub i_poc_l0ref0: ::core::ffi::c_int,
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 3],
        pub i_width: [::core::ffi::c_int; 3],
        pub i_lines: [::core::ffi::c_int; 3],
        pub i_stride_lowres: ::core::ffi::c_int,
        pub i_width_lowres: ::core::ffi::c_int,
        pub i_lines_lowres: ::core::ffi::c_int,
        pub plane: [*mut pixel; 3],
        pub plane_fld: [*mut pixel; 3],
        pub filtered: [[*mut pixel; 4]; 3],
        pub filtered_fld: [[*mut pixel; 4]; 3],
        pub lowres: [*mut pixel; 4],
        pub integral: *mut uint16_t,
        pub buffer: [*mut pixel; 4],
        pub buffer_fld: [*mut pixel; 4],
        pub buffer_lowres: *mut pixel,
        pub weight: [[x264_weight_t; 3]; 16],
        pub weighted: [*mut pixel; 16],
        pub b_duplicate: ::core::ffi::c_int,
        pub orig: *mut x264_frame,
        pub mb_type: *mut int8_t,
        pub mb_partition: *mut uint8_t,
        pub mv: [*mut [int16_t; 2]; 2],
        pub mv16x16: *mut [int16_t; 2],
        pub lowres_mvs: [[*mut [int16_t; 2]; 17]; 2],
        pub field: *mut uint8_t,
        pub effective_qp: *mut uint8_t,
        pub lowres_costs: [[*mut uint16_t; 18]; 18],
        pub lowres_mv_costs: [[*mut ::core::ffi::c_int; 17]; 2],
        pub ref_0: [*mut int8_t; 2],
        pub i_ref: [::core::ffi::c_int; 2],
        pub ref_poc: [[::core::ffi::c_int; 16]; 2],
        pub inv_ref_poc: [int16_t; 2],
        pub i_cost_est: [[::core::ffi::c_int; 18]; 18],
        pub i_cost_est_aq: [[::core::ffi::c_int; 18]; 18],
        pub i_satd: ::core::ffi::c_int,
        pub i_intra_mbs: [::core::ffi::c_int; 18],
        pub i_row_satds: [[*mut ::core::ffi::c_int; 18]; 18],
        pub i_row_satd: *mut ::core::ffi::c_int,
        pub i_row_bits: *mut ::core::ffi::c_int,
        pub f_row_qp: *mut ::core::ffi::c_float,
        pub f_row_qscale: *mut ::core::ffi::c_float,
        pub f_qp_offset: *mut ::core::ffi::c_float,
        pub f_qp_offset_aq: *mut ::core::ffi::c_float,
        pub b_intra_calculated: ::core::ffi::c_int,
        pub i_intra_cost: *mut uint16_t,
        pub i_propagate_cost: *mut uint16_t,
        pub i_inv_qscale_factor: *mut uint16_t,
        pub b_scenecut: ::core::ffi::c_int,
        pub f_weighted_cost_delta: [::core::ffi::c_float; 18],
        pub i_pixel_sum: [uint32_t; 3],
        pub i_pixel_ssd: [uint64_t; 3],
        pub hrd_timing: x264_hrd_t,
        pub i_planned_type: [uint8_t; 251],
        pub i_planned_satd: [::core::ffi::c_int; 251],
        pub f_planned_cpb_duration: [::core::ffi::c_double; 251],
        pub i_coded_fields_lookahead: int64_t,
        pub i_cpb_delay_lookahead: int64_t,
        pub i_lines_completed: ::core::ffi::c_int,
        pub i_lines_weighted: ::core::ffi::c_int,
        pub i_reference_count: ::core::ffi::c_int,
        pub mutex: pthread_mutex_t,
        pub cv: pthread_cond_t,
        pub i_slice_count: ::core::ffi::c_int,
        pub f_pir_position: ::core::ffi::c_float,
        pub i_pir_start_col: ::core::ffi::c_int,
        pub i_pir_end_col: ::core::ffi::c_int,
        pub i_frames_since_pir: ::core::ffi::c_int,
        pub b_corrupt: ::core::ffi::c_int,
        pub extra_sei: x264_sei_t,
        pub opaque: *mut ::core::ffi::c_void,
        pub mb_info: *mut uint8_t,
        pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:9"]
    pub struct x264_deblock_function_t {
        pub deblock_luma: [x264_deblock_inter_t; 2],
        pub deblock_chroma: [x264_deblock_inter_t; 2],
        pub deblock_h_chroma_420: x264_deblock_inter_t,
        pub deblock_h_chroma_422: x264_deblock_inter_t,
        pub deblock_luma_intra: [x264_deblock_intra_t; 2],
        pub deblock_chroma_intra: [x264_deblock_intra_t; 2],
        pub deblock_h_chroma_420_intra: x264_deblock_intra_t,
        pub deblock_h_chroma_422_intra: x264_deblock_intra_t,
        pub deblock_luma_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_420_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_422_mbaff: x264_deblock_inter_t,
        pub deblock_luma_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_420_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_422_intra_mbaff: x264_deblock_intra_t,
        pub deblock_strength: Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *mut [int8_t; 40],
                *mut [[int16_t; 2]; 40],
                *mut [[uint8_t; 4]; 8],
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
    }
    #[c2rust::src_loc = "197:1"]
    pub type x264_deblock_intra_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "196:1"]
    pub type x264_deblock_inter_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >;
    #[c2rust::src_loc = "32:9"]
    pub const PADH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const PADV: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int8_t, int16_t};
    use super::x264_h::{x264_param_t, x264_hrd_t, x264_sei_t, x264_picture_t};
    use super::common_h::{pixel, x264_t};
    use super::mc_h::x264_weight_t;
    use super::stdint_h::intptr_t;
    extern "C" {
        #[c2rust::src_loc = "222:1"]
        pub fn x264_10_frame_delete(frame: *mut x264_frame_t);
        #[c2rust::src_loc = "225:1"]
        pub fn x264_10_frame_copy_picture(
            h: *mut x264_t,
            dst: *mut x264_frame_t,
            src: *mut x264_picture_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "228:1"]
        pub fn x264_10_frame_expand_border(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
            mb_y: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "230:1"]
        pub fn x264_10_frame_expand_border_filtered(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
            mb_y: ::core::ffi::c_int,
            b_end: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "236:1"]
        pub fn x264_10_frame_expand_border_mod16(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
        );
        #[c2rust::src_loc = "238:1"]
        pub fn x264_10_expand_border_mbpair(
            h: *mut x264_t,
            mb_x: ::core::ffi::c_int,
            mb_y: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "241:1"]
        pub fn x264_10_frame_deblock_row(h: *mut x264_t, mb_y: ::core::ffi::c_int);
        #[c2rust::src_loc = "246:1"]
        pub fn x264_10_frame_filter(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
            mb_y: ::core::ffi::c_int,
            b_end: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "248:1"]
        pub fn x264_10_frame_init_lowres(h: *mut x264_t, frame: *mut x264_frame_t);
        #[c2rust::src_loc = "251:1"]
        pub fn x264_10_deblock_init(
            cpu: uint32_t,
            pf: *mut x264_deblock_function_t,
            b_mbaff: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "254:1"]
        pub fn x264_10_frame_cond_broadcast(
            frame: *mut x264_frame_t,
            i_lines_completed: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "258:1"]
        pub fn x264_10_frame_new_slice(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "261:1"]
        pub fn x264_10_threadslice_cond_broadcast(
            h: *mut x264_t,
            pass: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "263:1"]
        pub fn x264_10_threadslice_cond_wait(h: *mut x264_t, pass: ::core::ffi::c_int);
        #[c2rust::src_loc = "266:10"]
        pub fn x264_10_frame_push(
            list: *mut *mut x264_frame_t,
            frame: *mut x264_frame_t,
        );
        #[c2rust::src_loc = "268:10"]
        pub fn x264_10_frame_pop(list: *mut *mut x264_frame_t) -> *mut x264_frame_t;
        #[c2rust::src_loc = "270:10"]
        pub fn x264_10_frame_unshift(
            list: *mut *mut x264_frame_t,
            frame: *mut x264_frame_t,
        );
        #[c2rust::src_loc = "272:10"]
        pub fn x264_10_frame_shift(list: *mut *mut x264_frame_t) -> *mut x264_frame_t;
        #[c2rust::src_loc = "275:1"]
        pub fn x264_10_frame_push_unused(h: *mut x264_t, frame: *mut x264_frame_t);
        #[c2rust::src_loc = "277:1"]
        pub fn x264_10_frame_push_blank_unused(h: *mut x264_t, frame: *mut x264_frame_t);
        #[c2rust::src_loc = "279:1"]
        pub fn x264_10_frame_pop_blank_unused(h: *mut x264_t) -> *mut x264_frame_t;
        #[c2rust::src_loc = "281:1"]
        pub fn x264_10_weight_scale_plane(
            h: *mut x264_t,
            dst: *mut pixel,
            i_dst_stride: intptr_t,
            src: *mut pixel,
            i_src_stride: intptr_t,
            i_width: ::core::ffi::c_int,
            i_height: ::core::ffi::c_int,
            w: *mut x264_weight_t,
        );
        #[c2rust::src_loc = "284:1"]
        pub fn x264_10_frame_pop_unused(
            h: *mut x264_t,
            b_fdec: ::core::ffi::c_int,
        ) -> *mut x264_frame_t;
        #[c2rust::src_loc = "286:1"]
        pub fn x264_10_frame_delete_list(list: *mut *mut x264_frame_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:28"]
pub mod x264_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "795:16"]
    pub struct x264_sei_t {
        pub num_payloads: ::core::ffi::c_int,
        pub payloads: *mut x264_sei_payload_t,
        pub sei_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "788:16"]
    pub struct x264_sei_payload_t {
        pub payload_size: ::core::ffi::c_int,
        pub payload_type: ::core::ffi::c_int,
        pub payload: *mut uint8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "770:16"]
    pub struct x264_hrd_t {
        pub cpb_initial_arrival_time: ::core::ffi::c_double,
        pub cpb_final_arrival_time: ::core::ffi::c_double,
        pub cpb_removal_time: ::core::ffi::c_double,
        pub dpb_output_time: ::core::ffi::c_double,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "315:16"]
    pub struct x264_param_t {
        pub cpu: uint32_t,
        pub i_threads: ::core::ffi::c_int,
        pub i_lookahead_threads: ::core::ffi::c_int,
        pub b_sliced_threads: ::core::ffi::c_int,
        pub b_deterministic: ::core::ffi::c_int,
        pub b_cpu_independent: ::core::ffi::c_int,
        pub i_sync_lookahead: ::core::ffi::c_int,
        pub i_width: ::core::ffi::c_int,
        pub i_height: ::core::ffi::c_int,
        pub i_csp: ::core::ffi::c_int,
        pub i_bitdepth: ::core::ffi::c_int,
        pub i_level_idc: ::core::ffi::c_int,
        pub i_frame_total: ::core::ffi::c_int,
        pub i_nal_hrd: ::core::ffi::c_int,
        pub vui: C2RustUnnamed_5,
        pub i_frame_reference: ::core::ffi::c_int,
        pub i_dpb_size: ::core::ffi::c_int,
        pub i_keyint_max: ::core::ffi::c_int,
        pub i_keyint_min: ::core::ffi::c_int,
        pub i_scenecut_threshold: ::core::ffi::c_int,
        pub b_intra_refresh: ::core::ffi::c_int,
        pub i_bframe: ::core::ffi::c_int,
        pub i_bframe_adaptive: ::core::ffi::c_int,
        pub i_bframe_bias: ::core::ffi::c_int,
        pub i_bframe_pyramid: ::core::ffi::c_int,
        pub b_open_gop: ::core::ffi::c_int,
        pub b_bluray_compat: ::core::ffi::c_int,
        pub i_avcintra_class: ::core::ffi::c_int,
        pub i_avcintra_flavor: ::core::ffi::c_int,
        pub b_deblocking_filter: ::core::ffi::c_int,
        pub i_deblocking_filter_alphac0: ::core::ffi::c_int,
        pub i_deblocking_filter_beta: ::core::ffi::c_int,
        pub b_cabac: ::core::ffi::c_int,
        pub i_cabac_init_idc: ::core::ffi::c_int,
        pub b_interlaced: ::core::ffi::c_int,
        pub b_constrained_intra: ::core::ffi::c_int,
        pub i_cqm_preset: ::core::ffi::c_int,
        pub psz_cqm_file: *mut ::core::ffi::c_char,
        pub cqm_4iy: [uint8_t; 16],
        pub cqm_4py: [uint8_t; 16],
        pub cqm_4ic: [uint8_t; 16],
        pub cqm_4pc: [uint8_t; 16],
        pub cqm_8iy: [uint8_t; 64],
        pub cqm_8py: [uint8_t; 64],
        pub cqm_8ic: [uint8_t; 64],
        pub cqm_8pc: [uint8_t; 64],
        pub pf_log: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub p_log_private: *mut ::core::ffi::c_void,
        pub i_log_level: ::core::ffi::c_int,
        pub b_full_recon: ::core::ffi::c_int,
        pub psz_dump_yuv: *mut ::core::ffi::c_char,
        pub analyse: C2RustUnnamed_4,
        pub rc: C2RustUnnamed_3,
        pub crop_rect: C2RustUnnamed_2,
        pub i_frame_packing: ::core::ffi::c_int,
        pub mastering_display: C2RustUnnamed_1,
        pub content_light_level: C2RustUnnamed_0,
        pub i_alternative_transfer: ::core::ffi::c_int,
        pub b_aud: ::core::ffi::c_int,
        pub b_repeat_headers: ::core::ffi::c_int,
        pub b_annexb: ::core::ffi::c_int,
        pub i_sps_id: ::core::ffi::c_int,
        pub b_vfr_input: ::core::ffi::c_int,
        pub b_pulldown: ::core::ffi::c_int,
        pub i_fps_num: uint32_t,
        pub i_fps_den: uint32_t,
        pub i_timebase_num: uint32_t,
        pub i_timebase_den: uint32_t,
        pub b_tff: ::core::ffi::c_int,
        pub b_pic_struct: ::core::ffi::c_int,
        pub b_fake_interlaced: ::core::ffi::c_int,
        pub b_stitchable: ::core::ffi::c_int,
        pub b_opencl: ::core::ffi::c_int,
        pub i_opencl_device: ::core::ffi::c_int,
        pub opencl_device_id: *mut ::core::ffi::c_void,
        pub psz_clbin_file: *mut ::core::ffi::c_char,
        pub i_slice_max_size: ::core::ffi::c_int,
        pub i_slice_max_mbs: ::core::ffi::c_int,
        pub i_slice_min_mbs: ::core::ffi::c_int,
        pub i_slice_count: ::core::ffi::c_int,
        pub i_slice_count_max: ::core::ffi::c_int,
        pub param_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub nalu_process: Option<
            unsafe extern "C" fn(
                *mut x264_t,
                *mut x264_nal_t,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:16"]
    pub struct x264_nal_t {
        pub i_ref_idc: ::core::ffi::c_int,
        pub i_type: ::core::ffi::c_int,
        pub b_long_startcode: ::core::ffi::c_int,
        pub i_first_mb: ::core::ffi::c_int,
        pub i_last_mb: ::core::ffi::c_int,
        pub i_payload: ::core::ffi::c_int,
        pub p_payload: *mut uint8_t,
        pub i_padding: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:5"]
    pub struct C2RustUnnamed_0 {
        pub b_cll: ::core::ffi::c_int,
        pub i_max_cll: ::core::ffi::c_int,
        pub i_max_fall: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "501:5"]
    pub struct C2RustUnnamed_1 {
        pub b_mastering_display: ::core::ffi::c_int,
        pub i_green_x: ::core::ffi::c_int,
        pub i_green_y: ::core::ffi::c_int,
        pub i_blue_x: ::core::ffi::c_int,
        pub i_blue_y: ::core::ffi::c_int,
        pub i_red_x: ::core::ffi::c_int,
        pub i_red_y: ::core::ffi::c_int,
        pub i_white_x: ::core::ffi::c_int,
        pub i_white_y: ::core::ffi::c_int,
        pub i_display_max: int64_t,
        pub i_display_min: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:5"]
    pub struct C2RustUnnamed_2 {
        pub i_left: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "443:5"]
    pub struct C2RustUnnamed_3 {
        pub i_rc_method: ::core::ffi::c_int,
        pub i_qp_constant: ::core::ffi::c_int,
        pub i_qp_min: ::core::ffi::c_int,
        pub i_qp_max: ::core::ffi::c_int,
        pub i_qp_step: ::core::ffi::c_int,
        pub i_bitrate: ::core::ffi::c_int,
        pub f_rf_constant: ::core::ffi::c_float,
        pub f_rf_constant_max: ::core::ffi::c_float,
        pub f_rate_tolerance: ::core::ffi::c_float,
        pub i_vbv_max_bitrate: ::core::ffi::c_int,
        pub i_vbv_buffer_size: ::core::ffi::c_int,
        pub f_vbv_buffer_init: ::core::ffi::c_float,
        pub f_ip_factor: ::core::ffi::c_float,
        pub f_pb_factor: ::core::ffi::c_float,
        pub b_filler: ::core::ffi::c_int,
        pub i_aq_mode: ::core::ffi::c_int,
        pub f_aq_strength: ::core::ffi::c_float,
        pub b_mb_tree: ::core::ffi::c_int,
        pub i_lookahead: ::core::ffi::c_int,
        pub b_stat_write: ::core::ffi::c_int,
        pub psz_stat_out: *mut ::core::ffi::c_char,
        pub b_stat_read: ::core::ffi::c_int,
        pub psz_stat_in: *mut ::core::ffi::c_char,
        pub f_qcompress: ::core::ffi::c_float,
        pub f_qblur: ::core::ffi::c_float,
        pub f_complexity_blur: ::core::ffi::c_float,
        pub zones: *mut x264_zone_t,
        pub i_zones: ::core::ffi::c_int,
        pub psz_zones: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:16"]
    pub struct x264_zone_t {
        pub i_start: ::core::ffi::c_int,
        pub i_end: ::core::ffi::c_int,
        pub b_force_qp: ::core::ffi::c_int,
        pub i_qp: ::core::ffi::c_int,
        pub f_bitrate_factor: ::core::ffi::c_float,
        pub param: *mut x264_param_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "406:5"]
    pub struct C2RustUnnamed_4 {
        pub intra: ::core::ffi::c_uint,
        pub inter: ::core::ffi::c_uint,
        pub b_transform_8x8: ::core::ffi::c_int,
        pub i_weighted_pred: ::core::ffi::c_int,
        pub b_weighted_bipred: ::core::ffi::c_int,
        pub i_direct_mv_pred: ::core::ffi::c_int,
        pub i_chroma_qp_offset: ::core::ffi::c_int,
        pub i_me_method: ::core::ffi::c_int,
        pub i_me_range: ::core::ffi::c_int,
        pub i_mv_range: ::core::ffi::c_int,
        pub i_mv_range_thread: ::core::ffi::c_int,
        pub i_subpel_refine: ::core::ffi::c_int,
        pub b_chroma_me: ::core::ffi::c_int,
        pub b_mixed_references: ::core::ffi::c_int,
        pub i_trellis: ::core::ffi::c_int,
        pub b_fast_pskip: ::core::ffi::c_int,
        pub b_dct_decimate: ::core::ffi::c_int,
        pub i_noise_reduction: ::core::ffi::c_int,
        pub f_psy_rd: ::core::ffi::c_float,
        pub f_psy_trellis: ::core::ffi::c_float,
        pub b_psy: ::core::ffi::c_int,
        pub b_mb_info: ::core::ffi::c_int,
        pub b_mb_info_update: ::core::ffi::c_int,
        pub i_luma_deadzone: [::core::ffi::c_int; 2],
        pub b_psnr: ::core::ffi::c_int,
        pub b_ssim: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "342:5"]
    pub struct C2RustUnnamed_5 {
        pub i_sar_height: ::core::ffi::c_int,
        pub i_sar_width: ::core::ffi::c_int,
        pub i_overscan: ::core::ffi::c_int,
        pub i_vidformat: ::core::ffi::c_int,
        pub b_fullrange: ::core::ffi::c_int,
        pub i_colorprim: ::core::ffi::c_int,
        pub i_transfer: ::core::ffi::c_int,
        pub i_colmatrix: ::core::ffi::c_int,
        pub i_chroma_loc: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "86:1"]
    pub type nal_unit_type_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "98:5"]
    pub const NAL_FILLER: nal_unit_type_e = 12;
    #[c2rust::src_loc = "97:5"]
    pub const NAL_AUD: nal_unit_type_e = 9;
    #[c2rust::src_loc = "96:5"]
    pub const NAL_PPS: nal_unit_type_e = 8;
    #[c2rust::src_loc = "95:5"]
    pub const NAL_SPS: nal_unit_type_e = 7;
    #[c2rust::src_loc = "94:5"]
    pub const NAL_SEI: nal_unit_type_e = 6;
    #[c2rust::src_loc = "93:5"]
    pub const NAL_SLICE_IDR: nal_unit_type_e = 5;
    #[c2rust::src_loc = "92:5"]
    pub const NAL_SLICE_DPC: nal_unit_type_e = 4;
    #[c2rust::src_loc = "91:5"]
    pub const NAL_SLICE_DPB: nal_unit_type_e = 3;
    #[c2rust::src_loc = "90:5"]
    pub const NAL_SLICE_DPA: nal_unit_type_e = 2;
    #[c2rust::src_loc = "89:5"]
    pub const NAL_SLICE: nal_unit_type_e = 1;
    #[c2rust::src_loc = "88:5"]
    pub const NAL_UNKNOWN: nal_unit_type_e = 0;
    #[c2rust::src_loc = "101:1"]
    pub type nal_priority_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "106:5"]
    pub const NAL_PRIORITY_HIGHEST: nal_priority_e = 3;
    #[c2rust::src_loc = "105:5"]
    pub const NAL_PRIORITY_HIGH: nal_priority_e = 2;
    #[c2rust::src_loc = "104:5"]
    pub const NAL_PRIORITY_LOW: nal_priority_e = 1;
    #[c2rust::src_loc = "103:5"]
    pub const NAL_PRIORITY_DISPOSABLE: nal_priority_e = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "633:16"]
    pub struct x264_level_t {
        pub level_idc: uint8_t,
        pub mbps: int32_t,
        pub frame_size: int32_t,
        pub dpb: int32_t,
        pub bitrate: int32_t,
        pub cpb: int32_t,
        pub mv_range: uint16_t,
        pub mvs_per_2mb: uint8_t,
        pub slice_rate: uint8_t,
        pub mincr: uint8_t,
        pub bipred8x8: uint8_t,
        pub direct8x8: uint8_t,
        pub frame_only: uint8_t,
    }
    #[c2rust::src_loc = "757:1"]
    pub type pic_struct_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "767:5"]
    pub const PIC_STRUCT_TRIPLE: pic_struct_e = 9;
    #[c2rust::src_loc = "766:5"]
    pub const PIC_STRUCT_DOUBLE: pic_struct_e = 8;
    #[c2rust::src_loc = "765:5"]
    pub const PIC_STRUCT_BOTTOM_TOP_BOTTOM: pic_struct_e = 7;
    #[c2rust::src_loc = "764:5"]
    pub const PIC_STRUCT_TOP_BOTTOM_TOP: pic_struct_e = 6;
    #[c2rust::src_loc = "763:5"]
    pub const PIC_STRUCT_BOTTOM_TOP: pic_struct_e = 5;
    #[c2rust::src_loc = "762:5"]
    pub const PIC_STRUCT_TOP_BOTTOM: pic_struct_e = 4;
    #[c2rust::src_loc = "760:5"]
    pub const PIC_STRUCT_PROGRESSIVE: pic_struct_e = 1;
    #[c2rust::src_loc = "759:5"]
    pub const PIC_STRUCT_AUTO: pic_struct_e = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "803:16"]
    pub struct x264_image_t {
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 4],
        pub plane: [*mut uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "811:16"]
    pub struct x264_image_properties_t {
        pub quant_offsets: *mut ::core::ffi::c_float,
        pub quant_offsets_free: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        >,
        pub mb_info: *mut uint8_t,
        pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub f_ssim: ::core::ffi::c_double,
        pub f_psnr_avg: ::core::ffi::c_double,
        pub f_psnr: [::core::ffi::c_double; 3],
        pub f_crf_avg: ::core::ffi::c_double,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "867:16"]
    pub struct x264_picture_t {
        pub i_type: ::core::ffi::c_int,
        pub i_qpplus1: ::core::ffi::c_int,
        pub i_pic_struct: ::core::ffi::c_int,
        pub b_keyframe: ::core::ffi::c_int,
        pub i_pts: int64_t,
        pub i_dts: int64_t,
        pub param: *mut x264_param_t,
        pub img: x264_image_t,
        pub prop: x264_image_properties_t,
        pub hrd_timing: x264_hrd_t,
        pub extra_sei: x264_sei_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "143:9"]
    pub const X264_CPU_SSE2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "146:9"]
    pub const X264_CPU_SSSE3: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "148:9"]
    pub const X264_CPU_SSE42: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "152:9"]
    pub const X264_CPU_FMA3: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 12 as ::core::ffi::c_int;
    #[c2rust::src_loc = "153:9"]
    pub const X264_CPU_BMI1: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 13 as ::core::ffi::c_int;
    #[c2rust::src_loc = "154:9"]
    pub const X264_CPU_BMI2: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 14 as ::core::ffi::c_int;
    #[c2rust::src_loc = "156:9"]
    pub const X264_CPU_AVX512: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "159:9"]
    pub const X264_CPU_CACHELINE_64: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 18 as ::core::ffi::c_int;
    #[c2rust::src_loc = "160:9"]
    pub const X264_CPU_SSE2_IS_SLOW: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 19 as ::core::ffi::c_int;
    #[c2rust::src_loc = "161:9"]
    pub const X264_CPU_SSE2_IS_FAST: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 20 as ::core::ffi::c_int;
    #[c2rust::src_loc = "196:9"]
    pub const X264_ANALYSE_I4x4: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "197:9"]
    pub const X264_ANALYSE_I8x8: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "198:9"]
    pub const X264_ANALYSE_PSUB16x16: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "199:9"]
    pub const X264_ANALYSE_PSUB8x8: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "200:9"]
    pub const X264_ANALYSE_BSUB16x16: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "202:9"]
    pub const X264_DIRECT_PRED_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "203:9"]
    pub const X264_DIRECT_PRED_SPATIAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "205:9"]
    pub const X264_DIRECT_PRED_AUTO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "206:9"]
    pub const X264_ME_DIA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "207:9"]
    pub const X264_ME_HEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "208:9"]
    pub const X264_ME_UMH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "209:9"]
    pub const X264_ME_ESA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const X264_ME_TESA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "211:9"]
    pub const X264_CQM_FLAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "213:9"]
    pub const X264_CQM_CUSTOM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "214:9"]
    pub const X264_RC_CQP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "215:9"]
    pub const X264_RC_CRF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const X264_RC_ABR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "222:9"]
    pub const X264_B_ADAPT_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "224:9"]
    pub const X264_B_ADAPT_TRELLIS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "225:9"]
    pub const X264_WEIGHTP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "226:9"]
    pub const X264_WEIGHTP_SIMPLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "227:9"]
    pub const X264_WEIGHTP_SMART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "228:9"]
    pub const X264_B_PYRAMID_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "229:9"]
    pub const X264_B_PYRAMID_STRICT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "230:9"]
    pub const X264_B_PYRAMID_NORMAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "231:9"]
    pub const X264_KEYINT_MIN_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "232:9"]
    pub const X264_KEYINT_MAX_INFINITE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 30 as ::core::ffi::c_int;
    #[c2rust::src_loc = "236:9"]
    pub const X264_AVCINTRA_FLAVOR_SONY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xe as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "277:9"]
    pub const X264_TYPE_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "278:9"]
    pub const X264_TYPE_IDR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "279:9"]
    pub const X264_TYPE_I: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "280:9"]
    pub const X264_TYPE_P: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "281:9"]
    pub const X264_TYPE_BREF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "282:9"]
    pub const X264_TYPE_B: ::core::ffi::c_int = 0x5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "283:9"]
    pub const X264_TYPE_KEYFRAME: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "292:9"]
    pub const X264_LOG_DEBUG: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "295:9"]
    pub const X264_THREADS_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "299:9"]
    pub const X264_NAL_HRD_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "300:9"]
    pub const X264_NAL_HRD_VBR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "301:9"]
    pub const X264_NAL_HRD_CBR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint16_t};
    use super::internal::__va_list_tag;
    use super::common_h::x264_t;
    use super::stdint_intn_h::{int64_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "651:36"]
        pub static x264_levels: [x264_level_t; 0];
        #[c2rust::src_loc = "679:10"]
        pub fn x264_param_cleanup(param: *mut x264_param_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:28"]
pub mod mc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "235:16"]
    pub struct x264_weight_t {
        pub cachea: [int16_t; 8],
        pub cacheb: [int16_t; 8],
        pub i_denom: int32_t,
        pub i_scale: int32_t,
        pub i_offset: int32_t,
        pub weightfn: *mut weight_fn_t,
    }
    #[c2rust::src_loc = "234:1"]
    pub type weight_fn_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *const x264_weight_t,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "267:9"]
    pub struct x264_mc_functions_t {
        pub mc_luma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *const x264_weight_t,
            ) -> (),
        >,
        pub get_ref: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut intptr_t,
                *mut *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *const x264_weight_t,
            ) -> *mut pixel,
        >,
        pub mc_chroma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub avg: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >; 12],
        pub copy: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >; 7],
        pub copy_16x16_unaligned: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub store_interleave_chroma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub load_deinterleave_chroma_fenc: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub load_deinterleave_chroma_fdec: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_swap: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_interleave: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_yuyv: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_rgb: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_v210: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut uint32_t,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub hpel_filter: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut int16_t,
            ) -> (),
        >,
        pub prefetch_fenc: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_400: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_420: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_422: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_ref: Option<
            unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> (),
        >,
        pub memcpy_aligned: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub memzero_aligned: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
        >,
        pub integral_init4h: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
        >,
        pub integral_init8h: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
        >,
        pub integral_init4v: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> (),
        >,
        pub integral_init8v: Option<unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ()>,
        pub frame_init_lowres_core: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub weight: *mut weight_fn_t,
        pub offsetadd: *mut weight_fn_t,
        pub offsetsub: *mut weight_fn_t,
        pub weight_cache: Option<
            unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> (),
        >,
        pub mbtree_propagate_cost: Option<
            unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_propagate_list: Option<
            unsafe extern "C" fn(
                *mut x264_t,
                *mut uint16_t,
                *mut [int16_t; 2],
                *mut int16_t,
                *mut uint16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_fix8_pack: Option<
            unsafe extern "C" fn(
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_fix8_unpack: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_float,
                *mut uint16_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
    }
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::common_h::{pixel, x264_t};
    use super::stdint_h::intptr_t;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "343:1"]
        pub fn x264_10_mc_init(
            cpu: uint32_t,
            pf: *mut x264_mc_functions_t,
            cpu_independent: ::core::ffi::c_int,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:28"]
pub mod bitstream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:9"]
    pub struct x264_bitstream_function_t {
        pub nal_escape: Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                *mut uint8_t,
            ) -> *mut uint8_t,
        >,
        pub cabac_block_residual_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
        pub cabac_block_residual_rd_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
        pub cabac_block_residual_8x8_rd_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:9"]
    pub struct x264_run_level_t {
        pub last: int32_t,
        pub mask: int32_t,
        pub level: [dctcoef; 18],
    }
    #[c2rust::src_loc = "39:1"]
    pub type bs_t = bs_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct bs_s {
        pub p_start: *mut uint8_t,
        pub p: *mut uint8_t,
        pub p_end: *mut uint8_t,
        pub cur_bits: uintptr_t,
        pub i_left: ::core::ffi::c_int,
        pub i_bits_encoded: ::core::ffi::c_int,
    }
    #[inline]
    #[c2rust::src_loc = "86:1"]
    pub unsafe extern "C" fn bs_init(
        mut s: *mut bs_t,
        mut p_data: *mut ::core::ffi::c_void,
        mut i_data: ::core::ffi::c_int,
    ) {
        let mut offset: ::core::ffi::c_int = (p_data as intptr_t & 3 as intptr_t)
            as ::core::ffi::c_int;
        (*s).p_start = (p_data as *mut uint8_t).offset(-(offset as isize));
        (*s).p = (*s).p_start;
        (*s).p_end = (p_data as *mut uint8_t).offset(i_data as isize);
        (*s).i_left = WORD_SIZE
            .wrapping_sub(offset as uint64_t)
            .wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
        if offset != 0 {
            (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i)
                as uintptr_t;
            (*s).cur_bits
                >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
        } else {
            (*s).cur_bits = 0 as uintptr_t;
        };
    }
    #[inline]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn bs_pos(mut s: *mut bs_t) -> ::core::ffi::c_int {
        return ((8 as ::core::ffi::c_long
            * (*s).p.offset_from((*s).p_start) as ::core::ffi::c_long) as uint64_t)
            .wrapping_add(WORD_SIZE.wrapping_mul(8 as uint64_t))
            .wrapping_sub((*s).i_left as uint64_t) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "106:1"]
    pub unsafe extern "C" fn bs_flush(mut s: *mut bs_t) {
        (*((*s).p as *mut x264_union32_t)).i = endian_fix32(
            ((*s).cur_bits << ((*s).i_left & 31 as ::core::ffi::c_int)) as uint32_t,
        );
        (*s).p = (*s)
            .p
            .offset(
                WORD_SIZE
                    .wrapping_sub(((*s).i_left >> 3 as ::core::ffi::c_int) as uint64_t)
                    as isize,
            );
        (*s).i_left = WORD_SIZE.wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "113:1"]
    pub unsafe extern "C" fn bs_realign(mut s: *mut bs_t) {
        let mut offset: ::core::ffi::c_int = ((*s).p as intptr_t & 3 as intptr_t)
            as ::core::ffi::c_int;
        if offset != 0 {
            (*s).p = (*s).p.offset(-(offset as isize));
            (*s).i_left = WORD_SIZE
                .wrapping_sub(offset as uint64_t)
                .wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
            (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i)
                as uintptr_t;
            (*s).cur_bits
                >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
        }
    }
    #[inline]
    #[c2rust::src_loc = "125:1"]
    pub unsafe extern "C" fn bs_write(
        mut s: *mut bs_t,
        mut i_count: ::core::ffi::c_int,
        mut i_bits: uint32_t,
    ) {
        if WORD_SIZE == 8 as uint64_t {
            (*s).cur_bits = (*s).cur_bits << i_count | i_bits as uintptr_t;
            (*s).i_left -= i_count;
            if (*s).i_left <= 32 as ::core::ffi::c_int {
                (*((*s).p as *mut x264_union32_t)).i = endian_fix(
                    (*s).cur_bits << (*s).i_left,
                ) as uint32_t;
                (*s).i_left += 32 as ::core::ffi::c_int;
                (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            }
        } else if i_count < (*s).i_left {
            (*s).cur_bits = (*s).cur_bits << i_count | i_bits as uintptr_t;
            (*s).i_left -= i_count;
        } else {
            i_count -= (*s).i_left;
            (*s).cur_bits = (*s).cur_bits << (*s).i_left
                | (i_bits >> i_count) as uintptr_t;
            (*((*s).p as *mut x264_union32_t)).i = endian_fix((*s).cur_bits) as uint32_t;
            (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            (*s).cur_bits = i_bits as uintptr_t;
            (*s).i_left = 32 as ::core::ffi::c_int - i_count;
        };
    }
    #[inline]
    #[c2rust::src_loc = "169:1"]
    pub unsafe extern "C" fn bs_write1(mut s: *mut bs_t, mut i_bit: uint32_t) {
        (*s).cur_bits <<= 1 as ::core::ffi::c_int;
        (*s).cur_bits |= i_bit as uintptr_t;
        (*s).i_left -= 1;
        if (*s).i_left as uint64_t
            == WORD_SIZE.wrapping_mul(8 as uint64_t).wrapping_sub(32 as uint64_t)
        {
            (*((*s).p as *mut x264_union32_t)).i = endian_fix32(
                (*s).cur_bits as uint32_t,
            );
            (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            (*s).i_left = WORD_SIZE.wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
        }
    }
    #[inline]
    #[c2rust::src_loc = "187:1"]
    pub unsafe extern "C" fn bs_align_1(mut s: *mut bs_t) {
        bs_write(
            s,
            (*s).i_left & 7 as ::core::ffi::c_int,
            (((1 as ::core::ffi::c_int) << ((*s).i_left & 7 as ::core::ffi::c_int))
                - 1 as ::core::ffi::c_int) as uint32_t,
        );
        bs_flush(s);
    }
    #[c2rust::src_loc = "201:22"]
    pub static mut x264_ue_size_tab: [uint8_t; 256] = [
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    #[inline]
    #[c2rust::src_loc = "221:1"]
    pub unsafe extern "C" fn bs_write_ue_big(
        mut s: *mut bs_t,
        mut val: ::core::ffi::c_uint,
    ) {
        let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        val = val.wrapping_add(1);
        let mut tmp: ::core::ffi::c_int = val as ::core::ffi::c_int;
        if tmp >= 0x10000 as ::core::ffi::c_int {
            size = 32 as ::core::ffi::c_int;
            tmp >>= 16 as ::core::ffi::c_int;
        }
        if tmp >= 0x100 as ::core::ffi::c_int {
            size += 16 as ::core::ffi::c_int;
            tmp >>= 8 as ::core::ffi::c_int;
        }
        size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
        bs_write(s, size >> 1 as ::core::ffi::c_int, 0 as uint32_t);
        bs_write(
            s,
            (size >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int,
            val as uint32_t,
        );
    }
    #[inline]
    #[c2rust::src_loc = "246:1"]
    pub unsafe extern "C" fn bs_write_se(mut s: *mut bs_t, mut val: ::core::ffi::c_int) {
        let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut tmp: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            - val * 2 as ::core::ffi::c_int;
        if tmp < 0 as ::core::ffi::c_int {
            tmp = val * 2 as ::core::ffi::c_int;
        }
        val = tmp;
        if tmp >= 0x100 as ::core::ffi::c_int {
            size = 16 as ::core::ffi::c_int;
            tmp >>= 8 as ::core::ffi::c_int;
        }
        size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
        bs_write(s, size, val as uint32_t);
    }
    #[inline]
    #[c2rust::src_loc = "272:1"]
    pub unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut bs_t) {
        bs_write1(s, 1 as uint32_t);
        bs_write(s, (*s).i_left & 7 as ::core::ffi::c_int, 0 as uint32_t);
    }
    #[inline(always)]
    #[c2rust::src_loc = "283:1"]
    pub unsafe extern "C" fn bs_size_ue_big(
        mut val: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int {
        if val < 255 as ::core::ffi::c_uint {
            return x264_ue_size_tab[val.wrapping_add(1 as ::core::ffi::c_uint) as usize]
                as ::core::ffi::c_int
        } else {
            return x264_ue_size_tab[(val.wrapping_add(1 as ::core::ffi::c_uint)
                >> 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int
        };
    }
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
    use super::common_h::dctcoef;
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::cabac_h::x264_cabac_t;
    use super::stdint_intn_h::int32_t;
    use super::osdep_h::{WORD_SIZE, endian_fix32, endian_fix};
    use super::base_h::x264_union32_t;
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn x264_10_bitstream_init(cpu: uint32_t, pf: *mut x264_bitstream_function_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:28"]
pub mod cabac_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct x264_cabac_t {
        pub i_low: ::core::ffi::c_int,
        pub i_range: ::core::ffi::c_int,
        pub i_queue: ::core::ffi::c_int,
        pub i_bytes_outstanding: ::core::ffi::c_int,
        pub p_start: *mut uint8_t,
        pub p: *mut uint8_t,
        pub p_end: *mut uint8_t,
        pub f8_bits_encoded: ::core::ffi::c_int,
        pub state: [uint8_t; 1024],
        pub padding: [uint8_t; 12],
    }
    #[inline(always)]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn x264_cabac_pos(
        mut cb: *mut x264_cabac_t,
    ) -> ::core::ffi::c_int {
        return (((*cb).p.offset_from((*cb).p_start) as ::core::ffi::c_long
            + (*cb).i_bytes_outstanding as ::core::ffi::c_long)
            * 8 as ::core::ffi::c_long + (*cb).i_queue as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    }
    use super::stdint_uintn_h::uint8_t;
    use super::common_h::x264_t;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn x264_10_cabac_context_init(
            h: *mut x264_t,
            cb: *mut x264_cabac_t,
            i_slice_type: ::core::ffi::c_int,
            i_qp: ::core::ffi::c_int,
            i_model: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "61:1"]
        pub fn x264_10_cabac_encode_init(
            cb: *mut x264_cabac_t,
            p_data: *mut uint8_t,
            p_end: *mut uint8_t,
        );
        #[c2rust::src_loc = "71:1"]
        pub fn x264_10_cabac_encode_terminal_c(cb: *mut x264_cabac_t);
        #[c2rust::src_loc = "77:1"]
        pub fn x264_10_cabac_encode_flush(h: *mut x264_t, cb: *mut x264_cabac_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:28"]
pub mod quant_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct x264_quant_function_t {
        pub quant_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4x4: Option<
            unsafe extern "C" fn(
                *mut [dctcoef; 16],
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_2x2_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub dequant_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 64],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub dequant_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub dequant_4x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub idct_dequant_2x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [dctcoef; 16],
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub idct_dequant_2x4_dconly: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub optimize_chroma_2x2_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub optimize_chroma_2x4_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub denoise_dct: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut uint32_t,
                *mut udctcoef,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub decimate_score15: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub decimate_score16: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub decimate_score64: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_last: [Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >; 14],
        pub coeff_last4: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_last8: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_level_run: [Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >; 13],
        pub coeff_level_run4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >,
        pub coeff_level_run8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_4x4: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_8x8: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_4x4_psy: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
                *mut dctcoef,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_8x8_psy: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
                *mut dctcoef,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_dc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_chroma_422_dc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
            ) -> ::core::ffi::c_int,
        >,
    }
    use super::common_h::{dctcoef, udctcoef, x264_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint64_t, uint16_t};
    use super::bitstream_h::x264_run_level_t;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn x264_10_quant_init(
            h: *mut x264_t,
            cpu: uint32_t,
            pf: *mut x264_quant_function_t,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:28"]
pub mod dct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:9"]
    pub struct x264_zigzag_function_t {
        pub scan_8x8: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
        pub scan_4x4: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
        pub sub_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
            ) -> ::core::ffi::c_int,
        >,
        pub sub_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
            ) -> ::core::ffi::c_int,
        >,
        pub sub_4x4ac: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
                *mut dctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub interleave_8x8_cavlc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> (),
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:9"]
    pub struct x264_dct_function_t {
        pub sub4x4_dct: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add4x4_idct: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x8_dct: Option<
            unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
        >,
        pub sub8x8_dct_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add8x8_idct: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
        >,
        pub add8x8_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x16_dct_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub sub16x16_dct: Option<
            unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
        >,
        pub add16x16_idct: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
        >,
        pub add16x16_idct_dc: Option<
            unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> (),
        >,
        pub sub8x8_dct8: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add8x8_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub16x16_dct8: Option<
            unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> (),
        >,
        pub add16x16_idct8: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> (),
        >,
        pub dct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub idct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub dct2x4dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> (),
        >,
    }
    use super::common_h::{dctcoef, pixel};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn x264_10_dct_init(cpu: uint32_t, dctf: *mut x264_dct_function_t);
        #[c2rust::src_loc = "75:1"]
        pub fn x264_10_zigzag_init(
            cpu: uint32_t,
            pf_progressive: *mut x264_zigzag_function_t,
            pf_interlaced: *mut x264_zigzag_function_t,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:28"]
pub mod pixel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:9"]
    pub struct x264_pixel_function_t {
        pub sad: [x264_pixel_cmp_t; 8],
        pub ssd: [x264_pixel_cmp_t; 8],
        pub satd: [x264_pixel_cmp_t; 8],
        pub ssim: [x264_pixel_cmp_t; 7],
        pub sa8d: [x264_pixel_cmp_t; 4],
        pub mbcmp: [x264_pixel_cmp_t; 8],
        pub mbcmp_unaligned: [x264_pixel_cmp_t; 8],
        pub fpelcmp: [x264_pixel_cmp_t; 8],
        pub fpelcmp_x3: [x264_pixel_cmp_x3_t; 7],
        pub fpelcmp_x4: [x264_pixel_cmp_x4_t; 7],
        pub sad_aligned: [x264_pixel_cmp_t; 8],
        pub vsad: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub asd8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub sa8d_satd: [Option<
            unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> uint64_t,
        >; 1],
        pub var: [Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
        pub var2: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >; 4],
        pub hadamard_ac: [Option<
            unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
        >; 4],
        pub ssd_nv12_core: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
        >,
        pub ssim_4x4x2_core: Option<
            unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [::core::ffi::c_int; 4],
            ) -> (),
        >,
        pub ssim_end4: Option<
            unsafe extern "C" fn(
                *mut [::core::ffi::c_int; 4],
                *mut [::core::ffi::c_int; 4],
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_float,
        >,
        pub sad_x3: [x264_pixel_cmp_x3_t; 7],
        pub sad_x4: [x264_pixel_cmp_x4_t; 7],
        pub satd_x3: [x264_pixel_cmp_x3_t; 7],
        pub satd_x4: [x264_pixel_cmp_x4_t; 7],
        pub ads: [Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >; 7],
        pub intra_mbcmp_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sa8d_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_satd_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sad_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_mbcmp_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sa8d_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sad_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
    }
    #[c2rust::src_loc = "35:1"]
    pub type x264_pixel_cmp_x4_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            *mut ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "34:1"]
    pub type x264_pixel_cmp_x3_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            *mut ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "33:1"]
    pub type x264_pixel_cmp_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
        ) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "37:1"]
    pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "52:5"]
    pub const PIXEL_2x2: C2RustUnnamed_19 = 11;
    #[c2rust::src_loc = "51:5"]
    pub const PIXEL_2x4: C2RustUnnamed_19 = 10;
    #[c2rust::src_loc = "50:5"]
    pub const PIXEL_2x8: C2RustUnnamed_19 = 9;
    #[c2rust::src_loc = "49:5"]
    pub const PIXEL_4x2: C2RustUnnamed_19 = 8;
    #[c2rust::src_loc = "48:5"]
    pub const PIXEL_4x16: C2RustUnnamed_19 = 7;
    #[c2rust::src_loc = "45:5"]
    pub const PIXEL_4x4: C2RustUnnamed_19 = 6;
    #[c2rust::src_loc = "44:5"]
    pub const PIXEL_4x8: C2RustUnnamed_19 = 5;
    #[c2rust::src_loc = "43:5"]
    pub const PIXEL_8x4: C2RustUnnamed_19 = 4;
    #[c2rust::src_loc = "42:5"]
    pub const PIXEL_8x8: C2RustUnnamed_19 = 3;
    #[c2rust::src_loc = "41:5"]
    pub const PIXEL_8x16: C2RustUnnamed_19 = 2;
    #[c2rust::src_loc = "40:5"]
    pub const PIXEL_16x8: C2RustUnnamed_19 = 1;
    #[c2rust::src_loc = "39:5"]
    pub const PIXEL_16x16: C2RustUnnamed_19 = 0;
    #[c2rust::src_loc = "70:22"]
    pub static mut x264_luma2chroma_pixel: [[uint8_t; 7]; 4] = [
        [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0, 0, 0, 0],
        [
            PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x2 as ::core::ffi::c_int as uint8_t,
            PIXEL_2x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_2x2 as ::core::ffi::c_int as uint8_t,
        ],
        [
            PIXEL_8x16 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x16 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_2x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_2x4 as ::core::ffi::c_int as uint8_t,
        ],
        [
            PIXEL_16x16 as ::core::ffi::c_int as uint8_t,
            PIXEL_16x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x16 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    use super::common_h::{pixel, x264_t};
    use super::stdint_h::intptr_t;
    use super::stdint_uintn_h::{uint64_t, uint16_t, uint8_t, uint32_t};
    use super::stdint_intn_h::int16_t;
    extern "C" {
        #[c2rust::src_loc = "147:1"]
        pub fn x264_10_pixel_init(cpu: uint32_t, pixf: *mut x264_pixel_function_t);
        #[c2rust::src_loc = "149:1"]
        pub fn x264_10_pixel_ssd_nv12(
            pf: *mut x264_pixel_function_t,
            pix1: *mut pixel,
            i_pix1: intptr_t,
            pix2: *mut pixel,
            i_pix2: intptr_t,
            i_width: ::core::ffi::c_int,
            i_height: ::core::ffi::c_int,
            ssd_u: *mut uint64_t,
            ssd_v: *mut uint64_t,
        );
        #[c2rust::src_loc = "152:1"]
        pub fn x264_10_pixel_ssd_wxh(
            pf: *mut x264_pixel_function_t,
            pix1: *mut pixel,
            i_pix1: intptr_t,
            pix2: *mut pixel,
            i_pix2: intptr_t,
            i_width: ::core::ffi::c_int,
            i_height: ::core::ffi::c_int,
        ) -> uint64_t;
        #[c2rust::src_loc = "155:1"]
        pub fn x264_10_pixel_ssim_wxh(
            pf: *mut x264_pixel_function_t,
            pix1: *mut pixel,
            i_pix1: intptr_t,
            pix2: *mut pixel,
            i_pix2: intptr_t,
            i_width: ::core::ffi::c_int,
            i_height: ::core::ffi::c_int,
            buf: *mut ::core::ffi::c_void,
            cnt: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_float;
        #[c2rust::src_loc = "158:1"]
        pub fn x264_10_field_vsad(
            h: *mut x264_t,
            mb_x: ::core::ffi::c_int,
            mb_y: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:28"]
pub mod predict_h {
    #[c2rust::src_loc = "32:1"]
    pub type x264_predict_8x8_filter_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "30:1"]
    pub type x264_predict_t = Option<unsafe extern "C" fn(*mut pixel) -> ()>;
    #[c2rust::src_loc = "31:1"]
    pub type x264_predict8x8_t = Option<
        unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    >;
    #[c2rust::src_loc = "34:1"]
    pub type intra_chroma_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "43:5"]
    pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
    #[c2rust::src_loc = "42:5"]
    pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
    #[c2rust::src_loc = "41:5"]
    pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
    #[c2rust::src_loc = "39:5"]
    pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
    #[c2rust::src_loc = "38:5"]
    pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
    #[c2rust::src_loc = "37:5"]
    pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
    #[c2rust::src_loc = "36:5"]
    pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
    #[c2rust::src_loc = "51:1"]
    pub type intra16x16_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "60:5"]
    pub const I_PRED_16x16_DC_128: intra16x16_pred_e = 6;
    #[c2rust::src_loc = "59:5"]
    pub const I_PRED_16x16_DC_TOP: intra16x16_pred_e = 5;
    #[c2rust::src_loc = "58:5"]
    pub const I_PRED_16x16_DC_LEFT: intra16x16_pred_e = 4;
    #[c2rust::src_loc = "56:5"]
    pub const I_PRED_16x16_P: intra16x16_pred_e = 3;
    #[c2rust::src_loc = "55:5"]
    pub const I_PRED_16x16_DC: intra16x16_pred_e = 2;
    #[c2rust::src_loc = "54:5"]
    pub const I_PRED_16x16_H: intra16x16_pred_e = 1;
    #[c2rust::src_loc = "53:5"]
    pub const I_PRED_16x16_V: intra16x16_pred_e = 0;
    #[c2rust::src_loc = "68:1"]
    pub type intra4x4_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "82:5"]
    pub const I_PRED_4x4_DC_128: intra4x4_pred_e = 11;
    #[c2rust::src_loc = "81:5"]
    pub const I_PRED_4x4_DC_TOP: intra4x4_pred_e = 10;
    #[c2rust::src_loc = "80:5"]
    pub const I_PRED_4x4_DC_LEFT: intra4x4_pred_e = 9;
    #[c2rust::src_loc = "78:5"]
    pub const I_PRED_4x4_HU: intra4x4_pred_e = 8;
    #[c2rust::src_loc = "77:5"]
    pub const I_PRED_4x4_VL: intra4x4_pred_e = 7;
    #[c2rust::src_loc = "76:5"]
    pub const I_PRED_4x4_HD: intra4x4_pred_e = 6;
    #[c2rust::src_loc = "75:5"]
    pub const I_PRED_4x4_VR: intra4x4_pred_e = 5;
    #[c2rust::src_loc = "74:5"]
    pub const I_PRED_4x4_DDR: intra4x4_pred_e = 4;
    #[c2rust::src_loc = "73:5"]
    pub const I_PRED_4x4_DDL: intra4x4_pred_e = 3;
    #[c2rust::src_loc = "72:5"]
    pub const I_PRED_4x4_DC: intra4x4_pred_e = 2;
    #[c2rust::src_loc = "71:5"]
    pub const I_PRED_4x4_H: intra4x4_pred_e = 1;
    #[c2rust::src_loc = "70:5"]
    pub const I_PRED_4x4_V: intra4x4_pred_e = 0;
    #[c2rust::src_loc = "95:1"]
    pub type intra8x8_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "109:5"]
    pub const I_PRED_8x8_DC_128: intra8x8_pred_e = 11;
    #[c2rust::src_loc = "108:5"]
    pub const I_PRED_8x8_DC_TOP: intra8x8_pred_e = 10;
    #[c2rust::src_loc = "107:5"]
    pub const I_PRED_8x8_DC_LEFT: intra8x8_pred_e = 9;
    #[c2rust::src_loc = "105:5"]
    pub const I_PRED_8x8_HU: intra8x8_pred_e = 8;
    #[c2rust::src_loc = "104:5"]
    pub const I_PRED_8x8_VL: intra8x8_pred_e = 7;
    #[c2rust::src_loc = "103:5"]
    pub const I_PRED_8x8_HD: intra8x8_pred_e = 6;
    #[c2rust::src_loc = "102:5"]
    pub const I_PRED_8x8_VR: intra8x8_pred_e = 5;
    #[c2rust::src_loc = "101:5"]
    pub const I_PRED_8x8_DDR: intra8x8_pred_e = 4;
    #[c2rust::src_loc = "100:5"]
    pub const I_PRED_8x8_DDL: intra8x8_pred_e = 3;
    #[c2rust::src_loc = "99:5"]
    pub const I_PRED_8x8_DC: intra8x8_pred_e = 2;
    #[c2rust::src_loc = "98:5"]
    pub const I_PRED_8x8_H: intra8x8_pred_e = 1;
    #[c2rust::src_loc = "97:5"]
    pub const I_PRED_8x8_V: intra8x8_pred_e = 0;
    #[c2rust::src_loc = "45:22"]
    pub static mut x264_mb_chroma_pred_mode_fix: [uint8_t; 7] = [
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_H as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_V as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_P as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "62:22"]
    pub static mut x264_mb_pred_mode16x16_fix: [uint8_t; 7] = [
        I_PRED_16x16_V as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_H as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_P as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "84:21"]
    pub static mut x264_mb_pred_mode4x4_fix: [int8_t; 13] = [
        -(1 as ::core::ffi::c_int) as int8_t,
        I_PRED_4x4_V as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_H as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DDL as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DDR as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_VR as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_HD as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_VL as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_HU as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
    ];
    use super::common_h::pixel;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int8_t;
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn x264_10_predict_16x16_init(cpu: uint32_t, pf: *mut x264_predict_t);
        #[c2rust::src_loc = "152:1"]
        pub fn x264_10_predict_8x8c_init(cpu: uint32_t, pf: *mut x264_predict_t);
        #[c2rust::src_loc = "154:1"]
        pub fn x264_10_predict_8x16c_init(cpu: uint32_t, pf: *mut x264_predict_t);
        #[c2rust::src_loc = "156:1"]
        pub fn x264_10_predict_4x4_init(cpu: uint32_t, pf: *mut x264_predict_t);
        #[c2rust::src_loc = "158:1"]
        pub fn x264_10_predict_8x8_init(
            cpu: uint32_t,
            pf: *mut x264_predict8x8_t,
            predict_filter: *mut x264_predict_8x8_filter_t,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:28"]
pub mod set_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:9"]
    pub struct x264_pps_t {
        pub i_id: ::core::ffi::c_int,
        pub i_sps_id: ::core::ffi::c_int,
        pub b_cabac: ::core::ffi::c_int,
        pub b_pic_order: ::core::ffi::c_int,
        pub i_num_slice_groups: ::core::ffi::c_int,
        pub i_num_ref_idx_l0_default_active: ::core::ffi::c_int,
        pub i_num_ref_idx_l1_default_active: ::core::ffi::c_int,
        pub b_weighted_pred: ::core::ffi::c_int,
        pub b_weighted_bipred: ::core::ffi::c_int,
        pub i_pic_init_qp: ::core::ffi::c_int,
        pub i_pic_init_qs: ::core::ffi::c_int,
        pub i_chroma_qp_index_offset: ::core::ffi::c_int,
        pub b_deblocking_filter_control: ::core::ffi::c_int,
        pub b_constrained_intra_pred: ::core::ffi::c_int,
        pub b_redundant_pic_cnt: ::core::ffi::c_int,
        pub b_transform_8x8_mode: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct x264_sps_t {
        pub i_id: ::core::ffi::c_int,
        pub i_profile_idc: ::core::ffi::c_int,
        pub i_level_idc: ::core::ffi::c_int,
        pub b_constraint_set0: ::core::ffi::c_int,
        pub b_constraint_set1: ::core::ffi::c_int,
        pub b_constraint_set2: ::core::ffi::c_int,
        pub b_constraint_set3: ::core::ffi::c_int,
        pub i_log2_max_frame_num: ::core::ffi::c_int,
        pub i_poc_type: ::core::ffi::c_int,
        pub i_log2_max_poc_lsb: ::core::ffi::c_int,
        pub i_num_ref_frames: ::core::ffi::c_int,
        pub b_gaps_in_frame_num_value_allowed: ::core::ffi::c_int,
        pub i_mb_width: ::core::ffi::c_int,
        pub i_mb_height: ::core::ffi::c_int,
        pub b_frame_mbs_only: ::core::ffi::c_int,
        pub b_mb_adaptive_frame_field: ::core::ffi::c_int,
        pub b_direct8x8_inference: ::core::ffi::c_int,
        pub b_crop: ::core::ffi::c_int,
        pub crop: C2RustUnnamed_16,
        pub b_vui: ::core::ffi::c_int,
        pub vui: C2RustUnnamed_14,
        pub b_qpprime_y_zero_transform_bypass: ::core::ffi::c_int,
        pub i_chroma_format_idc: ::core::ffi::c_int,
        pub b_avcintra_hd: ::core::ffi::c_int,
        pub b_avcintra_4k: ::core::ffi::c_int,
        pub i_cqm_preset: ::core::ffi::c_int,
        pub scaling_list: [*const uint8_t; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:5"]
    pub struct C2RustUnnamed_14 {
        pub b_aspect_ratio_info_present: ::core::ffi::c_int,
        pub i_sar_width: ::core::ffi::c_int,
        pub i_sar_height: ::core::ffi::c_int,
        pub b_overscan_info_present: ::core::ffi::c_int,
        pub b_overscan_info: ::core::ffi::c_int,
        pub b_signal_type_present: ::core::ffi::c_int,
        pub i_vidformat: ::core::ffi::c_int,
        pub b_fullrange: ::core::ffi::c_int,
        pub b_color_description_present: ::core::ffi::c_int,
        pub i_colorprim: ::core::ffi::c_int,
        pub i_transfer: ::core::ffi::c_int,
        pub i_colmatrix: ::core::ffi::c_int,
        pub b_chroma_loc_info_present: ::core::ffi::c_int,
        pub i_chroma_loc_top: ::core::ffi::c_int,
        pub i_chroma_loc_bottom: ::core::ffi::c_int,
        pub b_timing_info_present: ::core::ffi::c_int,
        pub i_num_units_in_tick: uint32_t,
        pub i_time_scale: uint32_t,
        pub b_fixed_frame_rate: ::core::ffi::c_int,
        pub b_nal_hrd_parameters_present: ::core::ffi::c_int,
        pub b_vcl_hrd_parameters_present: ::core::ffi::c_int,
        pub hrd: C2RustUnnamed_15,
        pub b_pic_struct_present: ::core::ffi::c_int,
        pub b_bitstream_restriction: ::core::ffi::c_int,
        pub b_motion_vectors_over_pic_boundaries: ::core::ffi::c_int,
        pub i_max_bytes_per_pic_denom: ::core::ffi::c_int,
        pub i_max_bits_per_mb_denom: ::core::ffi::c_int,
        pub i_log2_max_mv_length_horizontal: ::core::ffi::c_int,
        pub i_log2_max_mv_length_vertical: ::core::ffi::c_int,
        pub i_num_reorder_frames: ::core::ffi::c_int,
        pub i_max_dec_frame_buffering: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:9"]
    pub struct C2RustUnnamed_15 {
        pub i_cpb_cnt: ::core::ffi::c_int,
        pub i_bit_rate_scale: ::core::ffi::c_int,
        pub i_cpb_size_scale: ::core::ffi::c_int,
        pub i_bit_rate_value: ::core::ffi::c_int,
        pub i_cpb_size_value: ::core::ffi::c_int,
        pub i_bit_rate_unscaled: ::core::ffi::c_int,
        pub i_cpb_size_unscaled: ::core::ffi::c_int,
        pub b_cbr_hrd: ::core::ffi::c_int,
        pub i_initial_cpb_removal_delay_length: ::core::ffi::c_int,
        pub i_cpb_removal_delay_length: ::core::ffi::c_int,
        pub i_dpb_output_delay_length: ::core::ffi::c_int,
        pub i_time_offset_length: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "72:5"]
    pub struct C2RustUnnamed_16 {
        pub i_left: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::common_h::x264_t;
    extern "C" {
        #[c2rust::src_loc = "180:1"]
        pub fn x264_10_cqm_init(h: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "182:1"]
        pub fn x264_10_cqm_delete(h: *mut x264_t);
        #[c2rust::src_loc = "184:1"]
        pub fn x264_10_cqm_parse_file(
            h: *mut x264_t,
            filename: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:28"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
        #[c2rust::src_loc = "33:10"]
        pub fn x264_10_threadpool_init(
            p_pool: *mut *mut x264_threadpool_t,
            threads: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "35:10"]
        pub fn x264_10_threadpool_run(
            pool: *mut x264_threadpool_t,
            func: Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
            >,
            arg: *mut ::core::ffi::c_void,
        );
        #[c2rust::src_loc = "37:10"]
        pub fn x264_10_threadpool_wait(
            pool: *mut x264_threadpool_t,
            arg: *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "39:10"]
        pub fn x264_10_threadpool_delete(pool: *mut x264_threadpool_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:28"]
pub mod base_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub union x264_union16_t {
        pub i: uint16_t,
        pub b: [uint8_t; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:9"]
    pub union x264_union32_t {
        pub i: uint32_t,
        pub w: [uint16_t; 2],
        pub b: [uint8_t; 4],
    }
    #[c2rust::src_loc = "93:1"]
    pub type profile_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "100:5"]
    pub const PROFILE_HIGH444_PREDICTIVE: profile_e = 244;
    #[c2rust::src_loc = "99:5"]
    pub const PROFILE_HIGH422: profile_e = 122;
    #[c2rust::src_loc = "98:5"]
    pub const PROFILE_HIGH10: profile_e = 110;
    #[c2rust::src_loc = "97:5"]
    pub const PROFILE_HIGH: profile_e = 100;
    #[c2rust::src_loc = "96:5"]
    pub const PROFILE_MAIN: profile_e = 77;
    #[c2rust::src_loc = "95:5"]
    pub const PROFILE_BASELINE: profile_e = 66;
    #[c2rust::src_loc = "103:1"]
    pub type chroma_format_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "108:5"]
    pub const CHROMA_444: chroma_format_e = 3;
    #[c2rust::src_loc = "107:5"]
    pub const CHROMA_422: chroma_format_e = 2;
    #[c2rust::src_loc = "106:5"]
    pub const CHROMA_420: chroma_format_e = 1;
    #[c2rust::src_loc = "105:5"]
    pub const CHROMA_400: chroma_format_e = 0;
    #[c2rust::src_loc = "111:1"]
    pub type slice_type_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "115:5"]
    pub const SLICE_TYPE_I: slice_type_e = 2;
    #[c2rust::src_loc = "114:5"]
    pub const SLICE_TYPE_B: slice_type_e = 1;
    #[c2rust::src_loc = "113:5"]
    pub const SLICE_TYPE_P: slice_type_e = 0;
    #[c2rust::src_loc = "118:19"]
    pub static mut slice_type_to_char: [::core::ffi::c_char; 3] = [
        'P' as i32 as ::core::ffi::c_char,
        'B' as i32 as ::core::ffi::c_char,
        'I' as i32 as ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "137:9"]
    pub const X264_REF_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "138:9"]
    pub const X264_THREAD_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
    #[c2rust::src_loc = "140:9"]
    pub const X264_LOOKAHEAD_MAX: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
    #[c2rust::src_loc = "144:9"]
    pub const X264_THREAD_HEIGHT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    #[c2rust::src_loc = "151:9"]
    pub const X264_WEIGHTP_FAKE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    #[c2rust::src_loc = "180:22"]
    pub static mut x264_scan8: [uint8_t; 51] = [
        (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min { i_min } else if v > i_max { i_max } else { v };
    }
    #[inline(always)]
    #[c2rust::src_loc = "211:1"]
    pub unsafe extern "C" fn x264_clip3f(
        mut v: ::core::ffi::c_double,
        mut f_min: ::core::ffi::c_double,
        mut f_max: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double {
        return if v < f_min { f_min } else if v > f_max { f_max } else { v };
    }
    use super::stdint_uintn_h::{uint16_t, uint8_t, uint32_t};
    use super::stdint_intn_h::int64_t;
    use super::x264_h::x264_param_t;
    extern "C" {
        #[c2rust::src_loc = "271:10"]
        pub fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
        #[c2rust::src_loc = "275:10"]
        pub fn x264_log_internal(
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "287:1"]
        pub fn x264_param_strdup(
            param: *mut x264_param_t,
            src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
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
    extern "C" {
        #[c2rust::src_loc = "30:10"]
        pub fn x264_cpu_num_processors() -> ::core::ffi::c_int;
        #[c2rust::src_loc = "54:39"]
        pub static x264_cpu_names: [x264_cpu_name_t; 0];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:28"]
pub mod macroblock_h {
    #[c2rust::src_loc = "64:1"]
    pub type mb_class_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:5"]
    pub const X264_MBTYPE_MAX: mb_class_e = 19;
    #[c2rust::src_loc = "86:5"]
    pub const B_SKIP: mb_class_e = 18;
    #[c2rust::src_loc = "85:5"]
    pub const B_8x8: mb_class_e = 17;
    #[c2rust::src_loc = "84:5"]
    pub const B_BI_BI: mb_class_e = 16;
    #[c2rust::src_loc = "83:5"]
    pub const B_BI_L1: mb_class_e = 15;
    #[c2rust::src_loc = "82:5"]
    pub const B_BI_L0: mb_class_e = 14;
    #[c2rust::src_loc = "81:5"]
    pub const B_L1_BI: mb_class_e = 13;
    #[c2rust::src_loc = "80:5"]
    pub const B_L1_L1: mb_class_e = 12;
    #[c2rust::src_loc = "79:5"]
    pub const B_L1_L0: mb_class_e = 11;
    #[c2rust::src_loc = "78:5"]
    pub const B_L0_BI: mb_class_e = 10;
    #[c2rust::src_loc = "77:5"]
    pub const B_L0_L1: mb_class_e = 9;
    #[c2rust::src_loc = "76:5"]
    pub const B_L0_L0: mb_class_e = 8;
    #[c2rust::src_loc = "75:5"]
    pub const B_DIRECT: mb_class_e = 7;
    #[c2rust::src_loc = "73:5"]
    pub const P_SKIP: mb_class_e = 6;
    #[c2rust::src_loc = "72:5"]
    pub const P_8x8: mb_class_e = 5;
    #[c2rust::src_loc = "71:5"]
    pub const P_L0: mb_class_e = 4;
    #[c2rust::src_loc = "69:5"]
    pub const I_PCM: mb_class_e = 3;
    #[c2rust::src_loc = "68:5"]
    pub const I_16x16: mb_class_e = 2;
    #[c2rust::src_loc = "67:5"]
    pub const I_8x8: mb_class_e = 1;
    #[c2rust::src_loc = "66:5"]
    pub const I_4x4: mb_class_e = 0;
    #[c2rust::src_loc = "115:1"]
    pub type mb_partition_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "140:5"]
    pub const X264_PARTTYPE_MAX: mb_partition_e = 17;
    #[c2rust::src_loc = "139:5"]
    pub const D_16x16: mb_partition_e = 16;
    #[c2rust::src_loc = "138:5"]
    pub const D_8x16: mb_partition_e = 15;
    #[c2rust::src_loc = "137:5"]
    pub const D_16x8: mb_partition_e = 14;
    #[c2rust::src_loc = "136:5"]
    pub const D_8x8: mb_partition_e = 13;
    #[c2rust::src_loc = "133:5"]
    pub const D_DIRECT_8x8: mb_partition_e = 12;
    #[c2rust::src_loc = "132:5"]
    pub const D_BI_8x8: mb_partition_e = 11;
    #[c2rust::src_loc = "131:5"]
    pub const D_BI_4x8: mb_partition_e = 10;
    #[c2rust::src_loc = "130:5"]
    pub const D_BI_8x4: mb_partition_e = 9;
    #[c2rust::src_loc = "129:5"]
    pub const D_BI_4x4: mb_partition_e = 8;
    #[c2rust::src_loc = "127:5"]
    pub const D_L1_8x8: mb_partition_e = 7;
    #[c2rust::src_loc = "126:5"]
    pub const D_L1_4x8: mb_partition_e = 6;
    #[c2rust::src_loc = "125:5"]
    pub const D_L1_8x4: mb_partition_e = 5;
    #[c2rust::src_loc = "124:5"]
    pub const D_L1_4x4: mb_partition_e = 4;
    #[c2rust::src_loc = "121:5"]
    pub const D_L0_8x8: mb_partition_e = 3;
    #[c2rust::src_loc = "120:5"]
    pub const D_L0_4x8: mb_partition_e = 2;
    #[c2rust::src_loc = "119:5"]
    pub const D_L0_8x4: mb_partition_e = 1;
    #[c2rust::src_loc = "118:5"]
    pub const D_L0_4x4: mb_partition_e = 0;
    #[c2rust::src_loc = "273:1"]
    pub type cabac_ctx_block_cat_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "288:5"]
    pub const DCT_CHROMAV_8x8: cabac_ctx_block_cat_e = 13;
    #[c2rust::src_loc = "287:5"]
    pub const DCT_CHROMAV_4x4: cabac_ctx_block_cat_e = 12;
    #[c2rust::src_loc = "286:5"]
    pub const DCT_CHROMAV_AC: cabac_ctx_block_cat_e = 11;
    #[c2rust::src_loc = "285:5"]
    pub const DCT_CHROMAV_DC: cabac_ctx_block_cat_e = 10;
    #[c2rust::src_loc = "284:5"]
    pub const DCT_CHROMAU_8x8: cabac_ctx_block_cat_e = 9;
    #[c2rust::src_loc = "283:5"]
    pub const DCT_CHROMAU_4x4: cabac_ctx_block_cat_e = 8;
    #[c2rust::src_loc = "282:5"]
    pub const DCT_CHROMAU_AC: cabac_ctx_block_cat_e = 7;
    #[c2rust::src_loc = "281:5"]
    pub const DCT_CHROMAU_DC: cabac_ctx_block_cat_e = 6;
    #[c2rust::src_loc = "280:5"]
    pub const DCT_LUMA_8x8: cabac_ctx_block_cat_e = 5;
    #[c2rust::src_loc = "279:5"]
    pub const DCT_CHROMA_AC: cabac_ctx_block_cat_e = 4;
    #[c2rust::src_loc = "278:5"]
    pub const DCT_CHROMA_DC: cabac_ctx_block_cat_e = 3;
    #[c2rust::src_loc = "277:5"]
    pub const DCT_LUMA_4x4: cabac_ctx_block_cat_e = 2;
    #[c2rust::src_loc = "276:5"]
    pub const DCT_LUMA_AC: cabac_ctx_block_cat_e = 1;
    #[c2rust::src_loc = "275:5"]
    pub const DCT_LUMA_DC: cabac_ctx_block_cat_e = 0;
    #[c2rust::src_loc = "97:22"]
    pub static mut x264_mb_type_list_table: [[[uint8_t; 2]; 2]; 19] = [
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
            [1 as ::core::ffi::c_int as uint8_t, 1 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
        [
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
            [0 as ::core::ffi::c_int as uint8_t, 0 as ::core::ffi::c_int as uint8_t],
        ],
    ];
    #[c2rust::src_loc = "171:22"]
    pub static mut x264_mb_partition_pixel_table: [uint8_t; 17] = [
        PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
        PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_16x8 as ::core::ffi::c_int as uint8_t,
        PIXEL_8x16 as ::core::ffi::c_int as uint8_t,
        PIXEL_16x16 as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "249:22"]
    pub static mut i_chroma_qp_table: [uint8_t; 106] = [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        (-(12 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(11 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(10 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(9 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(8 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(7 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(6 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(5 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(4 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(3 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(2 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (-(1 as ::core::ffi::c_int) + QP_BD_OFFSET) as uint8_t,
        (0 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (1 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (2 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (3 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (4 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (5 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (6 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (7 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (8 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (9 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (10 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (11 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (12 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (13 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (14 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (15 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (16 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (17 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (18 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (19 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (20 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (21 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (22 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (23 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (24 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (25 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (26 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (27 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (28 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (29 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (29 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (30 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (31 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (32 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (32 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (33 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (34 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (34 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (35 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (35 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (36 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (36 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (37 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (37 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (37 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (38 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (38 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (38 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        (39 as ::core::ffi::c_int + QP_BD_OFFSET) as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    use super::stdint_uintn_h::uint8_t;
    use super::pixel_h::{
        PIXEL_4x4, PIXEL_8x4, PIXEL_4x8, PIXEL_8x8, PIXEL_16x8, PIXEL_8x16, PIXEL_16x16,
    };
    use super::internal::BIT_DEPTH;
    use super::common_h::{QP_BD_OFFSET, x264_t};
    extern "C" {
        #[c2rust::src_loc = "303:1"]
        pub fn x264_10_macroblock_cache_allocate(h: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "305:1"]
        pub fn x264_10_macroblock_cache_free(h: *mut x264_t);
        #[c2rust::src_loc = "309:1"]
        pub fn x264_10_macroblock_thread_allocate(
            h: *mut x264_t,
            b_lookahead: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "311:1"]
        pub fn x264_10_macroblock_thread_free(
            h: *mut x264_t,
            b_lookahead: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "314:1"]
        pub fn x264_10_macroblock_slice_init(h: *mut x264_t);
        #[c2rust::src_loc = "316:1"]
        pub fn x264_10_macroblock_thread_init(h: *mut x264_t);
        #[c2rust::src_loc = "320:1"]
        pub fn x264_10_macroblock_cache_load_interlaced(
            h: *mut x264_t,
            mb_x: ::core::ffi::c_int,
            mb_y: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "322:1"]
        pub fn x264_10_macroblock_deblock_strength(h: *mut x264_t);
        #[c2rust::src_loc = "324:1"]
        pub fn x264_10_macroblock_cache_save(h: *mut x264_t);
        #[c2rust::src_loc = "327:1"]
        pub fn x264_10_macroblock_bipred_init(h: *mut x264_t);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "368:1"]
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "388:1"]
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "735:1"]
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __s: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "802:1"]
        pub fn fseeko(
            __stream: *mut FILE,
            __off: __off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:28"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:28"]
pub mod osdep_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn x264_is_regular_file(
        mut filehandle: *mut FILE,
    ) -> ::core::ffi::c_int {
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
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(filehandle), &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>()
        as uint64_t;
    #[inline(always)]
    #[c2rust::src_loc = "475:1"]
    pub unsafe extern "C" fn endian_fix32(mut x: uint32_t) -> uint32_t {
        return (x << 24 as ::core::ffi::c_int)
            .wrapping_add(x << 8 as ::core::ffi::c_int & 0xff0000 as uint32_t)
            .wrapping_add(x >> 8 as ::core::ffi::c_int & 0xff00 as uint32_t)
            .wrapping_add(x >> 24 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "487:1"]
    pub unsafe extern "C" fn endian_fix64(mut x: uint64_t) -> uint64_t {
        return (endian_fix32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t)
            .wrapping_add(
                (endian_fix32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int,
            );
    }
    #[inline(always)]
    #[c2rust::src_loc = "492:1"]
    pub unsafe extern "C" fn endian_fix(mut x: uintptr_t) -> uintptr_t {
        return if WORD_SIZE == 8 as uint64_t {
            endian_fix64(x as uint64_t) as uintptr_t
        } else {
            endian_fix32(x as uint32_t) as uintptr_t
        };
    }
    use super::FILE_h::FILE;
    use super::struct_stat_h::stat;
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t, __time_t,
    };
    use super::struct_timespec_h::timespec;
    use super::stat_h::fstat;
    use super::stdio_h::fileno;
    use super::bits_stat_h::__S_IFMT;
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    use super::stdint_h::uintptr_t;
}
#[c2rust::header_src = "/usr/include/pthread.h:28"]
pub mod pthread_h {
    use super::pthreadtypes_h::{
        pthread_mutex_t, pthread_mutexattr_t, pthread_cond_t, pthread_condattr_t,
    };
    extern "C" {
        #[c2rust::src_loc = "781:1"]
        pub fn pthread_mutex_init(
            __mutex: *mut pthread_mutex_t,
            __mutexattr: *const pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "786:1"]
        pub fn pthread_mutex_destroy(
            __mutex: *mut pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "794:1"]
        pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "835:1"]
        pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1112:1"]
        pub fn pthread_cond_init(
            __cond: *mut pthread_cond_t,
            __cond_attr: *const pthread_condattr_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1117:1"]
        pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1125:1"]
        pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "47:1"]
        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "141:1"]
        pub fn strcpy(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:28"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "52:22"]
        pub static x264_cqm_jvt4i: [uint8_t; 16];
        #[c2rust::src_loc = "58:22"]
        pub static x264_cqm_avci50_4ic: [uint8_t; 16];
        #[c2rust::src_loc = "59:22"]
        pub static x264_cqm_avci50_p_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "60:22"]
        pub static x264_cqm_avci50_1080i_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "61:22"]
        pub static x264_cqm_avci100_720p_4ic: [uint8_t; 16];
        #[c2rust::src_loc = "62:22"]
        pub static x264_cqm_avci100_720p_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "63:22"]
        pub static x264_cqm_avci100_1080_4ic: [uint8_t; 16];
        #[c2rust::src_loc = "64:22"]
        pub static x264_cqm_avci100_1080i_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "65:22"]
        pub static x264_cqm_avci100_1080p_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "66:22"]
        pub static x264_cqm_avci300_2160p_4iy: [uint8_t; 16];
        #[c2rust::src_loc = "67:22"]
        pub static x264_cqm_avci300_2160p_4ic: [uint8_t; 16];
        #[c2rust::src_loc = "68:22"]
        pub static x264_cqm_avci300_2160p_8iy: [uint8_t; 64];
        #[c2rust::src_loc = "100:16"]
        pub static mut x264_zero: [uint8_t; 1024];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/set.h:28"]
pub mod encoder_set_h {
    use super::set_h::{x264_sps_t, x264_pps_t};
    use super::x264_h::x264_param_t;
    use super::bitstream_h::bs_t;
    use super::common_h::x264_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn x264_10_sps_init(
            sps: *mut x264_sps_t,
            i_id: ::core::ffi::c_int,
            param: *mut x264_param_t,
        );
        #[c2rust::src_loc = "33:1"]
        pub fn x264_10_sps_init_reconfigurable(
            sps: *mut x264_sps_t,
            param: *mut x264_param_t,
        );
        #[c2rust::src_loc = "35:1"]
        pub fn x264_10_sps_init_scaling_list(
            sps: *mut x264_sps_t,
            param: *mut x264_param_t,
        );
        #[c2rust::src_loc = "37:1"]
        pub fn x264_10_sps_write(s: *mut bs_t, sps: *mut x264_sps_t);
        #[c2rust::src_loc = "39:1"]
        pub fn x264_10_pps_init(
            pps: *mut x264_pps_t,
            i_id: ::core::ffi::c_int,
            param: *mut x264_param_t,
            sps: *mut x264_sps_t,
        );
        #[c2rust::src_loc = "41:1"]
        pub fn x264_10_pps_write(
            s: *mut bs_t,
            sps: *mut x264_sps_t,
            pps: *mut x264_pps_t,
        );
        #[c2rust::src_loc = "43:1"]
        pub fn x264_10_sei_recovery_point_write(
            h: *mut x264_t,
            s: *mut bs_t,
            recovery_frame_cnt: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "45:1"]
        pub fn x264_10_sei_version_write(
            h: *mut x264_t,
            s: *mut bs_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "47:1"]
        pub fn x264_10_validate_levels(
            h: *mut x264_t,
            verbose: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "49:1"]
        pub fn x264_10_sei_buffering_period_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "51:1"]
        pub fn x264_10_sei_pic_timing_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "53:1"]
        pub fn x264_10_sei_dec_ref_pic_marking_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "55:1"]
        pub fn x264_10_sei_frame_packing_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "57:1"]
        pub fn x264_10_sei_mastering_display_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "59:1"]
        pub fn x264_10_sei_content_light_level_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "61:1"]
        pub fn x264_10_sei_alternative_transfer_write(h: *mut x264_t, s: *mut bs_t);
        #[c2rust::src_loc = "63:1"]
        pub fn x264_10_sei_avcintra_umid_write(
            h: *mut x264_t,
            s: *mut bs_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "65:1"]
        pub fn x264_10_sei_avcintra_vanc_write(
            h: *mut x264_t,
            s: *mut bs_t,
            len: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "67:1"]
        pub fn x264_10_sei_write(
            s: *mut bs_t,
            payload: *mut uint8_t,
            payload_size: ::core::ffi::c_int,
            payload_type: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn x264_10_filler_write(
            h: *mut x264_t,
            s: *mut bs_t,
            filler: ::core::ffi::c_int,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/analyse.h:28"]
pub mod analyse_h {
    use super::common_h::x264_t;
    use super::frame_h::x264_frame_t;
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn x264_10_analyse_init_costs(h: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "33:1"]
        pub fn x264_10_analyse_free_costs(h: *mut x264_t);
        #[c2rust::src_loc = "35:1"]
        pub fn x264_10_analyse_weight_frame(h: *mut x264_t, end: ::core::ffi::c_int);
        #[c2rust::src_loc = "37:1"]
        pub fn x264_10_macroblock_analyse(h: *mut x264_t);
        #[c2rust::src_loc = "45:1"]
        pub fn x264_10_lookahead_init(
            h: *mut x264_t,
            i_slicetype_length: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "47:1"]
        pub fn x264_10_lookahead_is_empty(h: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "49:1"]
        pub fn x264_10_lookahead_put_frame(h: *mut x264_t, frame: *mut x264_frame_t);
        #[c2rust::src_loc = "51:1"]
        pub fn x264_10_lookahead_get_frames(h: *mut x264_t);
        #[c2rust::src_loc = "53:1"]
        pub fn x264_10_lookahead_delete(h: *mut x264_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/ratecontrol.h:28"]
pub mod ratecontrol_h {
    use super::common_h::x264_t;
    use super::frame_h::x264_frame_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn x264_10_ratecontrol_new(_: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "45:1"]
        pub fn x264_10_ratecontrol_delete(_: *mut x264_t);
        #[c2rust::src_loc = "48:1"]
        pub fn x264_10_ratecontrol_init_reconfigurable(
            h: *mut x264_t,
            b_init: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "53:1"]
        pub fn x264_10_adaptive_quant_frame(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
            quant_offsets: *mut ::core::ffi::c_float,
        );
        #[c2rust::src_loc = "55:1"]
        pub fn x264_10_macroblock_tree_read(
            h: *mut x264_t,
            frame: *mut x264_frame_t,
            quant_offsets: *mut ::core::ffi::c_float,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "57:1"]
        pub fn x264_10_reference_build_list_optimal(
            h: *mut x264_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "59:1"]
        pub fn x264_10_thread_sync_ratecontrol(
            cur: *mut x264_t,
            prev: *mut x264_t,
            next: *mut x264_t,
        );
        #[c2rust::src_loc = "61:1"]
        pub fn x264_10_ratecontrol_zone_init(_: *mut x264_t);
        #[c2rust::src_loc = "63:1"]
        pub fn x264_10_ratecontrol_start(
            _: *mut x264_t,
            i_force_qp: ::core::ffi::c_int,
            overhead: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn x264_10_ratecontrol_set_weights(h: *mut x264_t, frm: *mut x264_frame_t);
        #[c2rust::src_loc = "69:1"]
        pub fn x264_10_ratecontrol_mb(
            _: *mut x264_t,
            bits: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "71:1"]
        pub fn x264_10_ratecontrol_qp(_: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "73:1"]
        pub fn x264_10_ratecontrol_mb_qp(h: *mut x264_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "75:1"]
        pub fn x264_10_ratecontrol_end(
            _: *mut x264_t,
            bits: ::core::ffi::c_int,
            filler: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "77:1"]
        pub fn x264_10_ratecontrol_summary(_: *mut x264_t);
        #[c2rust::src_loc = "81:1"]
        pub fn x264_10_threads_distribute_ratecontrol(h: *mut x264_t);
        #[c2rust::src_loc = "83:1"]
        pub fn x264_10_threads_merge_ratecontrol(h: *mut x264_t);
        #[c2rust::src_loc = "85:1"]
        pub fn x264_10_hrd_fullness(h: *mut x264_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/macroblock.h:28"]
pub mod encoder_macroblock_h {
    use super::common_h::x264_t;
    use super::cabac_h::x264_cabac_t;
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn x264_10_rdo_init();
        #[c2rust::src_loc = "53:1"]
        pub fn x264_10_macroblock_encode(h: *mut x264_t);
        #[c2rust::src_loc = "55:1"]
        pub fn x264_10_macroblock_write_cabac(h: *mut x264_t, cb: *mut x264_cabac_t);
        #[c2rust::src_loc = "57:1"]
        pub fn x264_10_macroblock_write_cavlc(h: *mut x264_t);
        #[c2rust::src_loc = "67:1"]
        pub fn x264_10_cabac_mb_skip(h: *mut x264_t, b_skip: ::core::ffi::c_int);
        #[c2rust::src_loc = "88:1"]
        pub fn x264_10_noise_reduction_update(h: *mut x264_t);
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/limits.h:28"]
pub mod limits_h {
    #[c2rust::src_loc = "50:9"]
    pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
    use super::internal::__INT_MAX__;
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "980:1"]
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:28"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "129:1"]
        pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "177:1"]
        pub fn pow(
            __x: ::core::ffi::c_double,
            __y: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "216:1"]
        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "170:1"]
        pub fn log2f(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:28"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:28"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/assert.h:28"]
pub mod assert_h {
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub use self::internal::{__va_list_tag, BIT_DEPTH, __INT_MAX__};
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t, __uint32_t, __int64_t,
    __uint64_t, __dev_t, __uid_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
    __off64_t, __time_t, __blksize_t, __blkcnt_t, __syscall_slong_t,
};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::struct_timespec_h::timespec;
pub use self::struct_stat_h::stat;
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stdint_h::{intptr_t, uintptr_t, UINT16_MAX, UINT32_MAX};
pub use self::atomic_wide_counter_h::{__atomic_wide_counter, C2RustUnnamed};
pub use self::thread_shared_types_h::{
    __pthread_internal_list, __pthread_list_t, __pthread_cond_s,
};
pub use self::struct_mutex_h::__pthread_mutex_s;
pub use self::pthreadtypes_h::{
    pthread_t, pthread_mutexattr_t, pthread_condattr_t, pthread_mutex_t, pthread_cond_t,
};
pub use self::common_h::{
    x264_t, x264_lookahead_t, pixel, dctcoef, udctcoef, C2RustUnnamed_6,
    x264_frame_stat_t, C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9,
    x264_left_table_t, C2RustUnnamed_10, C2RustUnnamed_11, x264_slice_header_t,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, QP_BD_OFFSET,
    QP_MAX_SPEC, QP_MAX, PIXEL_MAX, NALU_OVERHEAD, FILLER_OVERHEAD, SIZEOF_PIXEL,
    x264_ratecontrol_t, x264_10_log, x264_10_cavlc_init, x264_10_cabac_init,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t, PADH, PADV, x264_10_frame_delete,
    x264_10_frame_copy_picture, x264_10_frame_expand_border,
    x264_10_frame_expand_border_filtered, x264_10_frame_expand_border_mod16,
    x264_10_expand_border_mbpair, x264_10_frame_deblock_row, x264_10_frame_filter,
    x264_10_frame_init_lowres, x264_10_deblock_init, x264_10_frame_cond_broadcast,
    x264_10_frame_new_slice, x264_10_threadslice_cond_broadcast,
    x264_10_threadslice_cond_wait, x264_10_frame_push, x264_10_frame_pop,
    x264_10_frame_unshift, x264_10_frame_shift, x264_10_frame_push_unused,
    x264_10_frame_push_blank_unused, x264_10_frame_pop_blank_unused,
    x264_10_weight_scale_plane, x264_10_frame_pop_unused, x264_10_frame_delete_list,
};
pub use self::x264_h::{
    x264_sei_t, x264_sei_payload_t, x264_hrd_t, x264_param_t, x264_nal_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, x264_zone_t,
    C2RustUnnamed_4, C2RustUnnamed_5, nal_unit_type_e, NAL_FILLER, NAL_AUD, NAL_PPS,
    NAL_SPS, NAL_SEI, NAL_SLICE_IDR, NAL_SLICE_DPC, NAL_SLICE_DPB, NAL_SLICE_DPA,
    NAL_SLICE, NAL_UNKNOWN, nal_priority_e, NAL_PRIORITY_HIGHEST, NAL_PRIORITY_HIGH,
    NAL_PRIORITY_LOW, NAL_PRIORITY_DISPOSABLE, x264_level_t, pic_struct_e,
    PIC_STRUCT_TRIPLE, PIC_STRUCT_DOUBLE, PIC_STRUCT_BOTTOM_TOP_BOTTOM,
    PIC_STRUCT_TOP_BOTTOM_TOP, PIC_STRUCT_BOTTOM_TOP, PIC_STRUCT_TOP_BOTTOM,
    PIC_STRUCT_PROGRESSIVE, PIC_STRUCT_AUTO, x264_image_t, x264_image_properties_t,
    x264_picture_t, X264_CPU_SSE2, X264_CPU_SSSE3, X264_CPU_SSE42, X264_CPU_FMA3,
    X264_CPU_BMI1, X264_CPU_BMI2, X264_CPU_AVX512, X264_CPU_CACHELINE_64,
    X264_CPU_SSE2_IS_SLOW, X264_CPU_SSE2_IS_FAST, X264_ANALYSE_I4x4, X264_ANALYSE_I8x8,
    X264_ANALYSE_PSUB16x16, X264_ANALYSE_PSUB8x8, X264_ANALYSE_BSUB16x16,
    X264_DIRECT_PRED_NONE, X264_DIRECT_PRED_SPATIAL, X264_DIRECT_PRED_AUTO, X264_ME_DIA,
    X264_ME_HEX, X264_ME_UMH, X264_ME_ESA, X264_ME_TESA, X264_CQM_FLAT, X264_CQM_CUSTOM,
    X264_RC_CQP, X264_RC_CRF, X264_RC_ABR, X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS,
    X264_WEIGHTP_NONE, X264_WEIGHTP_SIMPLE, X264_WEIGHTP_SMART, X264_B_PYRAMID_NONE,
    X264_B_PYRAMID_STRICT, X264_B_PYRAMID_NORMAL, X264_KEYINT_MIN_AUTO,
    X264_KEYINT_MAX_INFINITE, X264_AVCINTRA_FLAVOR_SONY, X264_CSP_MASK, X264_CSP_NONE,
    X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_BGR,
    X264_CSP_MAX, X264_CSP_HIGH_DEPTH, X264_TYPE_AUTO, X264_TYPE_IDR, X264_TYPE_I,
    X264_TYPE_P, X264_TYPE_BREF, X264_TYPE_B, X264_TYPE_KEYFRAME, X264_LOG_ERROR,
    X264_LOG_WARNING, X264_LOG_INFO, X264_LOG_DEBUG, X264_THREADS_AUTO,
    X264_NAL_HRD_NONE, X264_NAL_HRD_VBR, X264_NAL_HRD_CBR, x264_levels,
    x264_param_cleanup,
};
pub use self::mc_h::{x264_weight_t, weight_fn_t, x264_mc_functions_t, x264_10_mc_init};
pub use self::bitstream_h::{
    x264_bitstream_function_t, x264_run_level_t, bs_t, bs_s, bs_init, bs_pos, bs_flush,
    bs_realign, bs_write, bs_write1, bs_align_1, x264_ue_size_tab, bs_write_ue_big,
    bs_write_se, bs_rbsp_trailing, bs_size_ue_big, x264_10_bitstream_init,
};
pub use self::cabac_h::{
    x264_cabac_t, x264_cabac_pos, x264_10_cabac_context_init, x264_10_cabac_encode_init,
    x264_10_cabac_encode_terminal_c, x264_10_cabac_encode_flush,
};
pub use self::quant_h::{x264_quant_function_t, x264_10_quant_init};
pub use self::dct_h::{
    x264_zigzag_function_t, x264_dct_function_t, x264_10_dct_init, x264_10_zigzag_init,
};
pub use self::pixel_h::{
    x264_pixel_function_t, x264_pixel_cmp_x4_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_t,
    C2RustUnnamed_19, PIXEL_2x2, PIXEL_2x4, PIXEL_2x8, PIXEL_4x2, PIXEL_4x16, PIXEL_4x4,
    PIXEL_4x8, PIXEL_8x4, PIXEL_8x8, PIXEL_8x16, PIXEL_16x8, PIXEL_16x16,
    x264_luma2chroma_pixel, x264_10_pixel_init, x264_10_pixel_ssd_nv12,
    x264_10_pixel_ssd_wxh, x264_10_pixel_ssim_wxh, x264_10_field_vsad,
};
pub use self::predict_h::{
    x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t, intra_chroma_pred_e,
    I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_DC_LEFT, I_PRED_CHROMA_P,
    I_PRED_CHROMA_V, I_PRED_CHROMA_H, I_PRED_CHROMA_DC, intra16x16_pred_e,
    I_PRED_16x16_DC_128, I_PRED_16x16_DC_TOP, I_PRED_16x16_DC_LEFT, I_PRED_16x16_P,
    I_PRED_16x16_DC, I_PRED_16x16_H, I_PRED_16x16_V, intra4x4_pred_e, I_PRED_4x4_DC_128,
    I_PRED_4x4_DC_TOP, I_PRED_4x4_DC_LEFT, I_PRED_4x4_HU, I_PRED_4x4_VL, I_PRED_4x4_HD,
    I_PRED_4x4_VR, I_PRED_4x4_DDR, I_PRED_4x4_DDL, I_PRED_4x4_DC, I_PRED_4x4_H,
    I_PRED_4x4_V, intra8x8_pred_e, I_PRED_8x8_DC_128, I_PRED_8x8_DC_TOP,
    I_PRED_8x8_DC_LEFT, I_PRED_8x8_HU, I_PRED_8x8_VL, I_PRED_8x8_HD, I_PRED_8x8_VR,
    I_PRED_8x8_DDR, I_PRED_8x8_DDL, I_PRED_8x8_DC, I_PRED_8x8_H, I_PRED_8x8_V,
    x264_mb_chroma_pred_mode_fix, x264_mb_pred_mode16x16_fix, x264_mb_pred_mode4x4_fix,
    x264_10_predict_16x16_init, x264_10_predict_8x8c_init, x264_10_predict_8x16c_init,
    x264_10_predict_4x4_init, x264_10_predict_8x8_init,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
    x264_10_cqm_init, x264_10_cqm_delete, x264_10_cqm_parse_file,
};
use self::threadpool_h::{
    x264_threadpool_t, x264_10_threadpool_init, x264_10_threadpool_run,
    x264_10_threadpool_wait, x264_10_threadpool_delete,
};
pub use self::base_h::{
    x264_union16_t, x264_union32_t, profile_e, PROFILE_HIGH444_PREDICTIVE,
    PROFILE_HIGH422, PROFILE_HIGH10, PROFILE_HIGH, PROFILE_MAIN, PROFILE_BASELINE,
    chroma_format_e, CHROMA_444, CHROMA_422, CHROMA_420, CHROMA_400, slice_type_e,
    SLICE_TYPE_I, SLICE_TYPE_B, SLICE_TYPE_P, slice_type_to_char, X264_REF_MAX,
    X264_THREAD_MAX, X264_LOOKAHEAD_MAX, X264_THREAD_HEIGHT, X264_WEIGHTP_FAKE,
    x264_scan8, x264_clip3, x264_clip3f, x264_reduce_fraction, x264_log_internal,
    x264_malloc, x264_free, x264_param_strdup,
};
pub use self::cpu_h::{x264_cpu_name_t, x264_cpu_num_processors, x264_cpu_names};
pub use self::macroblock_h::{
    mb_class_e, X264_MBTYPE_MAX, B_SKIP, B_8x8, B_BI_BI, B_BI_L1, B_BI_L0, B_L1_BI,
    B_L1_L1, B_L1_L0, B_L0_BI, B_L0_L1, B_L0_L0, B_DIRECT, P_SKIP, P_8x8, P_L0, I_PCM,
    I_16x16, I_8x8, I_4x4, mb_partition_e, X264_PARTTYPE_MAX, D_16x16, D_8x16, D_16x8,
    D_8x8, D_DIRECT_8x8, D_BI_8x8, D_BI_4x8, D_BI_8x4, D_BI_4x4, D_L1_8x8, D_L1_4x8,
    D_L1_8x4, D_L1_4x4, D_L0_8x8, D_L0_4x8, D_L0_8x4, D_L0_4x4, cabac_ctx_block_cat_e,
    DCT_CHROMAV_8x8, DCT_CHROMAV_4x4, DCT_CHROMAV_AC, DCT_CHROMAV_DC, DCT_CHROMAU_8x8,
    DCT_CHROMAU_4x4, DCT_CHROMAU_AC, DCT_CHROMAU_DC, DCT_LUMA_8x8, DCT_CHROMA_AC,
    DCT_CHROMA_DC, DCT_LUMA_4x4, DCT_LUMA_AC, DCT_LUMA_DC, x264_mb_type_list_table,
    x264_mb_partition_pixel_table, i_chroma_qp_table, x264_10_macroblock_cache_allocate,
    x264_10_macroblock_cache_free, x264_10_macroblock_thread_allocate,
    x264_10_macroblock_thread_free, x264_10_macroblock_slice_init,
    x264_10_macroblock_thread_init, x264_10_macroblock_cache_load_interlaced,
    x264_10_macroblock_deblock_strength, x264_10_macroblock_cache_save,
    x264_10_macroblock_bipred_init,
};
pub use self::stdio_h::{
    SEEK_SET, fclose, fopen, sprintf, snprintf, fwrite, fseeko, fileno,
};
use self::stat_h::fstat;
pub use self::osdep_h::{
    x264_is_regular_file, WORD_SIZE, endian_fix32, endian_fix64, endian_fix,
};
use self::pthread_h::{
    pthread_mutex_init, pthread_mutex_destroy, pthread_mutex_lock, pthread_mutex_unlock,
    pthread_cond_init, pthread_cond_destroy, pthread_cond_broadcast,
};
use self::string_h::{memcpy, memmove, memset, strcpy, strcmp, strlen};
use self::tables_h::{
    x264_cqm_jvt4i, x264_cqm_avci50_4ic, x264_cqm_avci50_p_8iy,
    x264_cqm_avci50_1080i_8iy, x264_cqm_avci100_720p_4ic, x264_cqm_avci100_720p_8iy,
    x264_cqm_avci100_1080_4ic, x264_cqm_avci100_1080i_8iy, x264_cqm_avci100_1080p_8iy,
    x264_cqm_avci300_2160p_4iy, x264_cqm_avci300_2160p_4ic, x264_cqm_avci300_2160p_8iy,
    x264_zero,
};
use self::encoder_set_h::{
    x264_10_sps_init, x264_10_sps_init_reconfigurable, x264_10_sps_init_scaling_list,
    x264_10_sps_write, x264_10_pps_init, x264_10_pps_write,
    x264_10_sei_recovery_point_write, x264_10_sei_version_write, x264_10_validate_levels,
    x264_10_sei_buffering_period_write, x264_10_sei_pic_timing_write,
    x264_10_sei_dec_ref_pic_marking_write, x264_10_sei_frame_packing_write,
    x264_10_sei_mastering_display_write, x264_10_sei_content_light_level_write,
    x264_10_sei_alternative_transfer_write, x264_10_sei_avcintra_umid_write,
    x264_10_sei_avcintra_vanc_write, x264_10_sei_write, x264_10_filler_write,
};
use self::analyse_h::{
    x264_10_analyse_init_costs, x264_10_analyse_free_costs, x264_10_analyse_weight_frame,
    x264_10_macroblock_analyse, x264_10_lookahead_init, x264_10_lookahead_is_empty,
    x264_10_lookahead_put_frame, x264_10_lookahead_get_frames, x264_10_lookahead_delete,
};
use self::ratecontrol_h::{
    x264_10_ratecontrol_new, x264_10_ratecontrol_delete,
    x264_10_ratecontrol_init_reconfigurable, x264_10_adaptive_quant_frame,
    x264_10_macroblock_tree_read, x264_10_reference_build_list_optimal,
    x264_10_thread_sync_ratecontrol, x264_10_ratecontrol_zone_init,
    x264_10_ratecontrol_start, x264_10_ratecontrol_set_weights, x264_10_ratecontrol_mb,
    x264_10_ratecontrol_qp, x264_10_ratecontrol_mb_qp, x264_10_ratecontrol_end,
    x264_10_ratecontrol_summary, x264_10_threads_distribute_ratecontrol,
    x264_10_threads_merge_ratecontrol, x264_10_hrd_fullness,
};
use self::encoder_macroblock_h::{
    x264_10_rdo_init, x264_10_macroblock_encode, x264_10_macroblock_write_cabac,
    x264_10_macroblock_write_cavlc, x264_10_cabac_mb_skip, x264_10_noise_reduction_update,
};
pub use self::limits_h::INT_MAX;
use self::stdlib_h::abs;
use self::mathcalls_h::{log10, pow, fabs, log2f};
pub use self::__stddef_null_h::NULL;
pub use self::bits_stat_h::__S_IFMT;
use self::assert_h::__assert_fail;
extern "C" {
    #[c2rust::src_loc = "44:1"]
    pub fn x264_10_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "45:1"]
    pub fn x264_10_macroblock_cache_load_progressive(
        h: *mut x264_t,
        i_mb_x: ::core::ffi::c_int,
        i_mb_y: ::core::ffi::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "729:22"]
pub struct C2RustUnnamed_20 {
    pub fps_num: uint16_t,
    pub fps_den: uint16_t,
    pub interlaced: uint8_t,
    pub frame_size: uint16_t,
    pub cqm_4iy: *const uint8_t,
    pub cqm_4ic: *const uint8_t,
    pub cqm_8iy: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "2677:9"]
pub struct x264_bs_bak_t {
    pub skip: ::core::ffi::c_int,
    pub cabac_prevbyte: uint8_t,
    pub bs: bs_t,
    pub cabac: x264_cabac_t,
    pub stat: x264_frame_stat_t,
    pub last_qp: ::core::ffi::c_int,
    pub last_dqp: ::core::ffi::c_int,
    pub field_decoding_flag: ::core::ffi::c_int,
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn calc_psnr(
    mut sqe: ::core::ffi::c_double,
    mut size: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut mse: ::core::ffi::c_double = sqe
        / ((PIXEL_MAX * PIXEL_MAX) as ::core::ffi::c_double * size);
    if mse <= 0.0000000001f64 {
        return 100 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    return -10.0f64 * log10(mse);
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn calc_ssim_db(
    mut ssim: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut inv_ssim: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double - ssim;
    if inv_ssim <= 0.0000000001f64 {
        return 100 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    return -10.0f64 * log10(inv_ssim);
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn threadpool_wait_all(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).param.i_threads {
        if (*(*h).thread[i as usize]).b_thread_active != 0 {
            (*(*h).thread[i as usize]).b_thread_active = 0 as ::core::ffi::c_int;
            if (x264_10_threadpool_wait(
                (*h).threadpool,
                (*h).thread[i as usize] as *mut ::core::ffi::c_void,
            ) as intptr_t) < 0 as intptr_t
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn frame_dump(mut h: *mut x264_t) {
    let mut f: *mut FILE = fopen(
        (*h).param.psz_dump_yuv,
        b"r+b\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return;
    }
    if (*h).param.b_sliced_threads != 0 {
        threadpool_wait_all(h);
    }
    let mut frame_size: ::core::ffi::c_int = (*h).param.i_height * (*h).param.i_width
        * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                (*h).param.i_height * (*h).param.i_width
                    * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                    >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
            } else {
                0 as ::core::ffi::c_int
            });
    if fseeko(f, (*(*h).fdec).i_frame as __off64_t * frame_size as __off64_t, SEEK_SET)
        == 0
    {
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p
            < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                == CHROMA_444 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            })
        {
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < (*h).param.i_height {
                fwrite(
                    &mut *(*(*(*h).fdec).plane.as_mut_ptr().offset(p as isize))
                        .offset(
                            (y * *(*(*h).fdec).i_stride.as_mut_ptr().offset(p as isize))
                                as isize,
                        ) as *mut pixel as *const ::core::ffi::c_void,
                    SIZEOF_PIXEL as size_t,
                    (*h).param.i_width as size_t,
                    f,
                );
                y += 1;
            }
            p += 1;
        }
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_420 as ::core::ffi::c_int
            || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                == CHROMA_422 as ::core::ffi::c_int
        {
            let mut cw: ::core::ffi::c_int = (*h).param.i_width
                >> 1 as ::core::ffi::c_int;
            let mut ch: ::core::ffi::c_int = (*h).param.i_height
                >> (*h).mb.chroma_v_shift;
            let mut planeu: *mut pixel = x264_malloc(
                (2 as ::core::ffi::c_int
                    * (cw * ch * SIZEOF_PIXEL + 32 as ::core::ffi::c_int)) as int64_t,
            ) as *mut pixel;
            if !planeu.is_null() {
                let mut planev: *mut pixel = planeu
                    .offset((cw * ch) as isize)
                    .offset((32 as ::core::ffi::c_int / SIZEOF_PIXEL) as isize);
                (*h)
                    .mc
                    .plane_copy_deinterleave
                    .expect(
                        "non-null function pointer",
                    )(
                    planeu,
                    cw as intptr_t,
                    planev,
                    cw as intptr_t,
                    (*(*h).fdec).plane[1 as ::core::ffi::c_int as usize],
                    (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    cw,
                    ch,
                );
                fwrite(
                    planeu as *const ::core::ffi::c_void,
                    1 as size_t,
                    (cw * ch * SIZEOF_PIXEL) as size_t,
                    f,
                );
                fwrite(
                    planev as *const ::core::ffi::c_void,
                    1 as size_t,
                    (cw * ch * SIZEOF_PIXEL) as size_t,
                    f,
                );
                x264_free(planeu as *mut ::core::ffi::c_void);
            }
        }
    }
    fclose(f);
}
#[c2rust::src_loc = "122:1"]
unsafe extern "C" fn slice_header_init(
    mut h: *mut x264_t,
    mut sh: *mut x264_slice_header_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
    mut i_idr_pic_id: ::core::ffi::c_int,
    mut i_frame: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    let mut param: *mut x264_param_t = &mut (*h).param;
    (*sh).sps = sps;
    (*sh).pps = pps;
    (*sh).i_first_mb = 0 as ::core::ffi::c_int;
    (*sh).i_last_mb = (*h).mb.i_mb_count - 1 as ::core::ffi::c_int;
    (*sh).i_pps_id = (*pps).i_id;
    (*sh).i_frame_num = i_frame;
    (*sh).b_mbaff = (*h).param.b_interlaced;
    (*sh).b_field_pic = 0 as ::core::ffi::c_int;
    (*sh).b_bottom_field = 0 as ::core::ffi::c_int;
    (*sh).i_idr_pic_id = i_idr_pic_id;
    (*sh).i_poc = 0 as ::core::ffi::c_int;
    (*sh).i_delta_poc_bottom = 0 as ::core::ffi::c_int;
    (*sh).i_delta_poc[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*sh).i_delta_poc[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*sh).i_redundant_pic_cnt = 0 as ::core::ffi::c_int;
    (*h).mb.b_direct_auto_write = ((*h).param.analyse.i_direct_mv_pred
        == X264_DIRECT_PRED_AUTO && (*h).param.i_bframe != 0
        && ((*h).param.rc.b_stat_write != 0 || (*h).param.rc.b_stat_read == 0))
        as ::core::ffi::c_int;
    if (*h).mb.b_direct_auto_read == 0
        && (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int
    {
        if (*(*h)
            .fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .i_poc_l0ref0
            == (*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_poc
        {
            if (*h).mb.b_direct_auto_write != 0 {
                (*sh).b_direct_spatial_mv_pred = ((*h)
                    .stat
                    .i_direct_score[1 as ::core::ffi::c_int as usize]
                    > (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize])
                    as ::core::ffi::c_int;
            } else {
                (*sh).b_direct_spatial_mv_pred = ((*param).analyse.i_direct_mv_pred
                    == X264_DIRECT_PRED_SPATIAL) as ::core::ffi::c_int;
            }
        } else {
            (*h).mb.b_direct_auto_write = 0 as ::core::ffi::c_int;
            (*sh).b_direct_spatial_mv_pred = 1 as ::core::ffi::c_int;
        }
    }
    (*sh).b_num_ref_idx_override = 0 as ::core::ffi::c_int;
    (*sh).i_num_ref_idx_l0_active = 1 as ::core::ffi::c_int;
    (*sh).i_num_ref_idx_l1_active = 1 as ::core::ffi::c_int;
    (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize] = (*h)
        .b_ref_reorder[0 as ::core::ffi::c_int as usize];
    (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize] = (*h)
        .b_ref_reorder[1 as ::core::ffi::c_int as usize];
    let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while list < 2 as ::core::ffi::c_int {
        if (*sh).b_ref_pic_list_reordering[list as usize] != 0 {
            let mut pred_frame_num: ::core::ffi::c_int = i_frame;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).i_ref[list as usize] {
                let mut diff: ::core::ffi::c_int = (*(*h)
                    .fref[list as usize][i as usize])
                    .i_frame_num - pred_frame_num;
                (*sh).ref_pic_list_order[list as usize][i as usize].idc = (diff
                    > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                (*sh).ref_pic_list_order[list as usize][i as usize].arg = abs(diff)
                    - 1 as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << (*sps).i_log2_max_frame_num)
                        - 1 as ::core::ffi::c_int;
                pred_frame_num = (*(*h).fref[list as usize][i as usize]).i_frame_num;
                i += 1;
            }
        }
        list += 1;
    }
    (*sh).i_cabac_init_idc = (*param).i_cabac_init_idc;
    (*sh).i_qp = if i_qp
        < 51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    {
        i_qp
    } else {
        51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    };
    (*sh).i_qp_delta = (*sh).i_qp - (*pps).i_pic_init_qp;
    (*sh).b_sp_for_swidth = 0 as ::core::ffi::c_int;
    (*sh).i_qs_delta = 0 as ::core::ffi::c_int;
    let mut deblock_thresh: ::core::ffi::c_int = i_qp
        + 2 as ::core::ffi::c_int
            * (if (*param).i_deblocking_filter_alphac0
                < (*param).i_deblocking_filter_beta
            {
                (*param).i_deblocking_filter_alphac0
            } else {
                (*param).i_deblocking_filter_beta
            });
    if (*param).b_deblocking_filter != 0
        && ((*h).mb.b_variable_qp != 0 || (15 as ::core::ffi::c_int) < deblock_thresh)
    {
        (*sh).i_disable_deblocking_filter_idc = if (*param).b_sliced_threads != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        (*sh).i_disable_deblocking_filter_idc = 1 as ::core::ffi::c_int;
    }
    (*sh).i_alpha_c0_offset = (*param).i_deblocking_filter_alphac0
        * 2 as ::core::ffi::c_int;
    (*sh).i_beta_offset = (*param).i_deblocking_filter_beta * 2 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn slice_header_write(
    mut s: *mut bs_t,
    mut sh: *mut x264_slice_header_t,
    mut i_nal_ref_idc: ::core::ffi::c_int,
) {
    if (*sh).b_mbaff != 0 {
        let mut first_x: ::core::ffi::c_int = (*sh).i_first_mb % (*(*sh).sps).i_mb_width;
        let mut first_y: ::core::ffi::c_int = (*sh).i_first_mb / (*(*sh).sps).i_mb_width;
        if first_y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"(first_y&1) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"encoder/encoder.c\0" as *const u8 as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 60],
                    [::core::ffi::c_char; 60],
                >(*b"void slice_header_write(bs_t *, x264_slice_header_t *, int)\0")
                    .as_ptr(),
            );
        }
        'c_44275: {
            if first_y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {} else {
                __assert_fail(
                    b"(first_y&1) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"encoder/encoder.c\0" as *const u8 as *const ::core::ffi::c_char,
                    219 as ::core::ffi::c_uint,
                    ::core::mem::transmute::<
                        [u8; 60],
                        [::core::ffi::c_char; 60],
                    >(*b"void slice_header_write(bs_t *, x264_slice_header_t *, int)\0")
                        .as_ptr(),
                );
            }
        };
        bs_write_ue_big(
            s,
            (2 as ::core::ffi::c_int * first_x
                + (*(*sh).sps).i_mb_width * (first_y & !(1 as ::core::ffi::c_int))
                + (first_y & 1 as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    } else {
        bs_write_ue_big(s, (*sh).i_first_mb as ::core::ffi::c_uint);
    }
    bs_write_ue_big(s, ((*sh).i_type + 5 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    bs_write_ue_big(s, (*sh).i_pps_id as ::core::ffi::c_uint);
    bs_write(
        s,
        (*(*sh).sps).i_log2_max_frame_num,
        ((*sh).i_frame_num
            & ((1 as ::core::ffi::c_int) << (*(*sh).sps).i_log2_max_frame_num)
                - 1 as ::core::ffi::c_int) as uint32_t,
    );
    if (*(*sh).sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sh).b_field_pic as uint32_t);
        if (*sh).b_field_pic != 0 {
            bs_write1(s, (*sh).b_bottom_field as uint32_t);
        }
    }
    if (*sh).i_idr_pic_id >= 0 as ::core::ffi::c_int {
        bs_write_ue_big(s, (*sh).i_idr_pic_id as ::core::ffi::c_uint);
    }
    if (*(*sh).sps).i_poc_type == 0 as ::core::ffi::c_int {
        bs_write(
            s,
            (*(*sh).sps).i_log2_max_poc_lsb,
            ((*sh).i_poc
                & ((1 as ::core::ffi::c_int) << (*(*sh).sps).i_log2_max_poc_lsb)
                    - 1 as ::core::ffi::c_int) as uint32_t,
        );
        if (*(*sh).pps).b_pic_order != 0 && (*sh).b_field_pic == 0 {
            bs_write_se(s, (*sh).i_delta_poc_bottom);
        }
    }
    if (*(*sh).pps).b_redundant_pic_cnt != 0 {
        bs_write_ue_big(s, (*sh).i_redundant_pic_cnt as ::core::ffi::c_uint);
    }
    if (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        bs_write1(s, (*sh).b_direct_spatial_mv_pred as uint32_t);
    }
    if (*sh).i_type == SLICE_TYPE_P as ::core::ffi::c_int
        || (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int
    {
        bs_write1(s, (*sh).b_num_ref_idx_override as uint32_t);
        if (*sh).b_num_ref_idx_override != 0 {
            bs_write_ue_big(
                s,
                ((*sh).i_num_ref_idx_l0_active - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            if (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int {
                bs_write_ue_big(
                    s,
                    ((*sh).i_num_ref_idx_l1_active - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
            }
        }
    }
    if (*sh).i_type != SLICE_TYPE_I as ::core::ffi::c_int {
        bs_write1(
            s,
            (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize] as uint32_t,
        );
        if (*sh).b_ref_pic_list_reordering[0 as ::core::ffi::c_int as usize] != 0 {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*sh).i_num_ref_idx_l0_active {
                bs_write_ue_big(
                    s,
                    (*sh)
                        .ref_pic_list_order[0 as ::core::ffi::c_int as usize][i as usize]
                        .idc as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    (*sh)
                        .ref_pic_list_order[0 as ::core::ffi::c_int as usize][i as usize]
                        .arg as ::core::ffi::c_uint,
                );
                i += 1;
            }
            bs_write_ue_big(s, 3 as ::core::ffi::c_uint);
        }
    }
    if (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        bs_write1(
            s,
            (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize] as uint32_t,
        );
        if (*sh).b_ref_pic_list_reordering[1 as ::core::ffi::c_int as usize] != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*sh).i_num_ref_idx_l1_active {
                bs_write_ue_big(
                    s,
                    (*sh)
                        .ref_pic_list_order[1 as ::core::ffi::c_int
                            as usize][i_0 as usize]
                        .idc as ::core::ffi::c_uint,
                );
                bs_write_ue_big(
                    s,
                    (*sh)
                        .ref_pic_list_order[1 as ::core::ffi::c_int
                            as usize][i_0 as usize]
                        .arg as ::core::ffi::c_uint,
                );
                i_0 += 1;
            }
            bs_write_ue_big(s, 3 as ::core::ffi::c_uint);
        }
    }
    (*sh).b_weighted_pred = 0 as ::core::ffi::c_int;
    if (*(*sh).pps).b_weighted_pred != 0
        && (*sh).i_type == SLICE_TYPE_P as ::core::ffi::c_int
    {
        (*sh).b_weighted_pred = (!(*sh)
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            .weightfn
            .is_null()
            || !(*sh)
                .weight[0 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
            || !(*sh)
                .weight[0 as ::core::ffi::c_int
                    as usize][2 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()) as ::core::ffi::c_int;
        bs_write_ue_big(
            s,
            (*sh)
                .weight[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .i_denom as ::core::ffi::c_uint,
        );
        if (*(*sh).sps).i_chroma_format_idc != 0 {
            bs_write_ue_big(
                s,
                (*sh)
                    .weight[0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize]
                    .i_denom as ::core::ffi::c_uint,
            );
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*sh).i_num_ref_idx_l0_active {
            let mut luma_weight_l0_flag: ::core::ffi::c_int = !(*sh)
                .weight[i_1 as usize][0 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null() as ::core::ffi::c_int;
            bs_write1(s, luma_weight_l0_flag as uint32_t);
            if luma_weight_l0_flag != 0 {
                bs_write_se(
                    s,
                    (*sh).weight[i_1 as usize][0 as ::core::ffi::c_int as usize].i_scale
                        as ::core::ffi::c_int,
                );
                bs_write_se(
                    s,
                    (*sh).weight[i_1 as usize][0 as ::core::ffi::c_int as usize].i_offset
                        as ::core::ffi::c_int,
                );
            }
            if (*(*sh).sps).i_chroma_format_idc != 0 {
                let mut chroma_weight_l0_flag: ::core::ffi::c_int = (!(*sh)
                    .weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                    || !(*sh)
                        .weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                        .weightfn
                        .is_null()) as ::core::ffi::c_int;
                bs_write1(s, chroma_weight_l0_flag as uint32_t);
                if chroma_weight_l0_flag != 0 {
                    let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while j < 3 as ::core::ffi::c_int {
                        bs_write_se(
                            s,
                            (*sh).weight[i_1 as usize][j as usize].i_scale
                                as ::core::ffi::c_int,
                        );
                        bs_write_se(
                            s,
                            (*sh).weight[i_1 as usize][j as usize].i_offset
                                as ::core::ffi::c_int,
                        );
                        j += 1;
                    }
                }
            }
            i_1 += 1;
        }
    } else {
        (*(*sh).pps).b_weighted_bipred == 1 as ::core::ffi::c_int
            && (*sh).i_type == SLICE_TYPE_B as ::core::ffi::c_int;
    }
    if i_nal_ref_idc != 0 as ::core::ffi::c_int {
        if (*sh).i_idr_pic_id >= 0 as ::core::ffi::c_int {
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
        } else {
            bs_write1(
                s,
                ((*sh).i_mmco_command_count > 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as uint32_t,
            );
            if (*sh).i_mmco_command_count > 0 as ::core::ffi::c_int {
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < (*sh).i_mmco_command_count {
                    bs_write_ue_big(s, 1 as ::core::ffi::c_uint);
                    bs_write_ue_big(
                        s,
                        ((*sh).mmco[i_2 as usize].i_difference_of_pic_nums
                            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                    );
                    i_2 += 1;
                }
                bs_write_ue_big(s, 0 as ::core::ffi::c_uint);
            }
        }
    }
    if (*(*sh).pps).b_cabac != 0 && (*sh).i_type != SLICE_TYPE_I as ::core::ffi::c_int {
        bs_write_ue_big(s, (*sh).i_cabac_init_idc as ::core::ffi::c_uint);
    }
    bs_write_se(s, (*sh).i_qp_delta);
    if (*(*sh).pps).b_deblocking_filter_control != 0 {
        bs_write_ue_big(s, (*sh).i_disable_deblocking_filter_idc as ::core::ffi::c_uint);
        if (*sh).i_disable_deblocking_filter_idc != 1 as ::core::ffi::c_int {
            bs_write_se(s, (*sh).i_alpha_c0_offset >> 1 as ::core::ffi::c_int);
            bs_write_se(s, (*sh).i_beta_offset >> 1 as ::core::ffi::c_int);
        }
    }
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn bitstream_check_buffer_internal(
    mut h: *mut x264_t,
    mut size: ::core::ffi::c_int,
    mut b_cabac: ::core::ffi::c_int,
    mut i_nal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if b_cabac != 0
        && ((*h).cabac.p_end.offset_from((*h).cabac.p) as ::core::ffi::c_long)
            < size as ::core::ffi::c_long
        || ((*h).out.bs.p_end.offset_from((*h).out.bs.p) as ::core::ffi::c_long)
            < size as ::core::ffi::c_long
    {
        if size > INT_MAX - (*h).out.i_bitstream {
            return -(1 as ::core::ffi::c_int);
        }
        let mut buf_size: ::core::ffi::c_int = (*h).out.i_bitstream + size;
        let mut buf: *mut uint8_t = x264_malloc(buf_size as int64_t) as *mut uint8_t;
        if buf.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        let mut aligned_size: ::core::ffi::c_int = (*h).out.i_bitstream
            & !(15 as ::core::ffi::c_int);
        (*h)
            .mc
            .memcpy_aligned
            .expect(
                "non-null function pointer",
            )(
            buf as *mut ::core::ffi::c_void,
            (*h).out.p_bitstream as *const ::core::ffi::c_void,
            aligned_size as size_t,
        );
        memcpy(
            buf.offset(aligned_size as isize) as *mut ::core::ffi::c_void,
            (*h).out.p_bitstream.offset(aligned_size as isize)
                as *const ::core::ffi::c_void,
            ((*h).out.i_bitstream - aligned_size) as size_t,
        );
        let mut delta: intptr_t = buf.offset_from((*h).out.p_bitstream) as intptr_t;
        (*h).out.bs.p_start = (*h).out.bs.p_start.offset(delta as isize);
        (*h).out.bs.p = (*h).out.bs.p.offset(delta as isize);
        (*h).out.bs.p_end = buf.offset(buf_size as isize);
        (*h).cabac.p_start = (*h).cabac.p_start.offset(delta as isize);
        (*h).cabac.p = (*h).cabac.p.offset(delta as isize);
        (*h).cabac.p_end = buf.offset(buf_size as isize);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i <= i_nal {
            let ref mut fresh2 = (*(*h).out.nal.offset(i as isize)).p_payload;
            *fresh2 = (*fresh2).offset(delta as isize);
            i += 1;
        }
        x264_free((*h).out.p_bitstream as *mut ::core::ffi::c_void);
        (*h).out.p_bitstream = buf;
        (*h).out.i_bitstream = buf_size;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn bitstream_check_buffer(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut max_row_size: ::core::ffi::c_int = ((2500 as ::core::ffi::c_int)
        << (*h).sh.b_mbaff) * (*h).mb.i_mb_width;
    return bitstream_check_buffer_internal(
        h,
        max_row_size,
        (*h).param.b_cabac,
        (*h).out.i_nal,
    );
}
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn bitstream_check_buffer_filler(
    mut h: *mut x264_t,
    mut filler: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    filler += 32 as ::core::ffi::c_int;
    return bitstream_check_buffer_internal(
        h,
        filler,
        0 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
    );
}
#[c2rust::src_loc = "423:1"]
unsafe extern "C" fn validate_parameters(
    mut h: *mut x264_t,
    mut b_open: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*h).param.pf_log.is_none() {
        x264_log_internal(
            X264_LOG_ERROR,
            b"pf_log not set! did you forget to call x264_param_default?\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as ::core::ffi::c_int;
    if (*h).param.i_width <= 0 as ::core::ffi::c_int
        || (*h).param.i_height <= 0 as ::core::ffi::c_int
        || (*h).param.i_width > MAX_RESOLUTION || (*h).param.i_height > MAX_RESOLUTION
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid width x height (%dx%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut i_csp: ::core::ffi::c_int = (*h).param.i_csp & X264_CSP_MASK;
    if i_csp <= X264_CSP_NONE || i_csp >= X264_CSP_MAX {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid CSP (only I400/I420/YV12/NV12/NV21/I422/YV16/NV16/YUYV/UYVY/I444/YV24/BGR/BGRA/RGB supported)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut w_mod: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut h_mod: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << ((*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0)
            as ::core::ffi::c_int;
    if i_csp == X264_CSP_I400 {
        (*h).param.analyse.i_chroma_qp_offset = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_chroma_me = 0 as ::core::ffi::c_int;
        (*h).param.vui.i_colmatrix = 2 as ::core::ffi::c_int;
    } else if i_csp < X264_CSP_I444 {
        w_mod = 2 as ::core::ffi::c_int;
        if i_csp < X264_CSP_I422 {
            h_mod *= 2 as ::core::ffi::c_int;
        }
    }
    if (*h).param.i_width % w_mod != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"width not divisible by %d (%dx%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            w_mod,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.i_height % h_mod != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"height not divisible by %d (%dx%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            h_mod,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.crop_rect.i_left < 0 as ::core::ffi::c_int
        || (*h).param.crop_rect.i_left >= (*h).param.i_width
        || (*h).param.crop_rect.i_right < 0 as ::core::ffi::c_int
        || (*h).param.crop_rect.i_right >= (*h).param.i_width
        || (*h).param.crop_rect.i_top < 0 as ::core::ffi::c_int
        || (*h).param.crop_rect.i_top >= (*h).param.i_height
        || (*h).param.crop_rect.i_bottom < 0 as ::core::ffi::c_int
        || (*h).param.crop_rect.i_bottom >= (*h).param.i_height
        || (*h).param.crop_rect.i_left + (*h).param.crop_rect.i_right
            >= (*h).param.i_width
        || (*h).param.crop_rect.i_top + (*h).param.crop_rect.i_bottom
            >= (*h).param.i_height
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid crop-rect %d,%d,%d,%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).param.crop_rect.i_left,
            (*h).param.crop_rect.i_top,
            (*h).param.crop_rect.i_right,
            (*h).param.crop_rect.i_bottom,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.crop_rect.i_left % w_mod != 0
        || (*h).param.crop_rect.i_right % w_mod != 0
        || (*h).param.crop_rect.i_top % h_mod != 0
        || (*h).param.crop_rect.i_bottom % h_mod != 0
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"crop-rect %d,%d,%d,%d not divisible by %dx%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).param.crop_rect.i_left,
            (*h).param.crop_rect.i_top,
            (*h).param.crop_rect.i_right,
            (*h).param.crop_rect.i_bottom,
            w_mod,
            h_mod,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.vui.i_sar_width <= 0 as ::core::ffi::c_int
        || (*h).param.vui.i_sar_height <= 0 as ::core::ffi::c_int
    {
        (*h).param.vui.i_sar_width = 0 as ::core::ffi::c_int;
        (*h).param.vui.i_sar_height = 0 as ::core::ffi::c_int;
    }
    if (*h).param.i_threads == X264_THREADS_AUTO {
        (*h).param.i_threads = x264_cpu_num_processors()
            * (if (*h).param.b_sliced_threads != 0 {
                2 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            }) / 2 as ::core::ffi::c_int;
        let mut max_threads: ::core::ffi::c_int = if 1 as ::core::ffi::c_int
            > ((*h).param.i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            ((*h).param.i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int
        };
        (*h).param.i_threads = if (*h).param.i_threads < max_threads {
            (*h).param.i_threads
        } else {
            max_threads
        };
    }
    let mut max_sliced_threads: ::core::ffi::c_int = if 1 as ::core::ffi::c_int
        > ((*h).param.i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int
            / 4 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        ((*h).param.i_height + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int
            / 4 as ::core::ffi::c_int
    };
    if (*h).param.i_threads > 1 as ::core::ffi::c_int {
        if (*h).param.b_sliced_threads != 0 {
            (*h).param.i_threads = if (*h).param.i_threads < max_sliced_threads {
                (*h).param.i_threads
            } else {
                max_sliced_threads
            };
        }
    }
    (*h).param.i_threads = x264_clip3(
        (*h).param.i_threads,
        1 as ::core::ffi::c_int,
        X264_THREAD_MAX,
    );
    if (*h).param.i_threads == 1 as ::core::ffi::c_int {
        (*h).param.b_sliced_threads = 0 as ::core::ffi::c_int;
        (*h).param.i_lookahead_threads = 1 as ::core::ffi::c_int;
    }
    (*h).i_thread_frames = if (*h).param.b_sliced_threads != 0 {
        1 as ::core::ffi::c_int
    } else {
        (*h).param.i_threads
    };
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        (*h).param.nalu_process = None;
    }
    if (*h).param.b_opencl != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"OpenCL: not compiled with OpenCL support, disabling\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.b_opencl = 0 as ::core::ffi::c_int;
        if !(*h).param.opencl_device_id.is_null() && (*h).param.i_opencl_device != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"OpenCL: device id and device skip count configured; dropping skip\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            (*h).param.i_opencl_device = 0 as ::core::ffi::c_int;
        }
    }
    (*h).param.i_keyint_max = x264_clip3(
        (*h).param.i_keyint_max,
        1 as ::core::ffi::c_int,
        X264_KEYINT_MAX_INFINITE,
    );
    if (*h).param.i_keyint_max == 1 as ::core::ffi::c_int {
        (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
        (*h).param.analyse.i_weighted_pred = 0 as ::core::ffi::c_int;
        (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
        (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
    }
    if (*h).param.i_frame_packing < -(1 as ::core::ffi::c_int)
        || (*h).param.i_frame_packing > 7 as ::core::ffi::c_int
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"ignoring unknown frame packing value\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_frame_packing = -(1 as ::core::ffi::c_int);
    }
    if (*h).param.i_frame_packing == 7 as ::core::ffi::c_int
        && (((*h).param.i_width - (*h).param.crop_rect.i_left
            - (*h).param.crop_rect.i_right) % 3 as ::core::ffi::c_int != 0
            || ((*h).param.i_height - (*h).param.crop_rect.i_top
                - (*h).param.crop_rect.i_bottom) % 3 as ::core::ffi::c_int != 0)
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"cropped resolution %dx%d not compatible with tile format frame packing\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*h).param.i_width - (*h).param.crop_rect.i_left
                - (*h).param.crop_rect.i_right,
            (*h).param.i_height - (*h).param.crop_rect.i_top
                - (*h).param.crop_rect.i_bottom,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.mastering_display.b_mastering_display != 0 {
        if (*h).param.mastering_display.i_green_x > UINT16_MAX
            || (*h).param.mastering_display.i_green_x < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_green_y > UINT16_MAX
            || (*h).param.mastering_display.i_green_y < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_blue_x > UINT16_MAX
            || (*h).param.mastering_display.i_blue_x < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_blue_y > UINT16_MAX
            || (*h).param.mastering_display.i_blue_y < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_red_x > UINT16_MAX
            || (*h).param.mastering_display.i_red_x < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_red_y > UINT16_MAX
            || (*h).param.mastering_display.i_red_y < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_white_x > UINT16_MAX
            || (*h).param.mastering_display.i_white_x < 0 as ::core::ffi::c_int
            || (*h).param.mastering_display.i_white_y > UINT16_MAX
            || (*h).param.mastering_display.i_white_y < 0 as ::core::ffi::c_int
        {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"mastering display xy coordinates out of range [0,%u]\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                UINT16_MAX,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.mastering_display.i_display_max > UINT32_MAX as int64_t
            || (*h).param.mastering_display.i_display_max < 0 as int64_t
            || (*h).param.mastering_display.i_display_min > UINT32_MAX as int64_t
            || (*h).param.mastering_display.i_display_min < 0 as int64_t
        {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"mastering display brightness out of range [0,%u]\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                UINT32_MAX,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.mastering_display.i_display_min == 50000 as int64_t
            && (*h).param.mastering_display.i_display_max == 50000 as int64_t
        {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"mastering display min and max brightness cannot both be 50000\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*h).param.content_light_level.b_cll != 0
        && ((*h).param.content_light_level.i_max_cll > UINT16_MAX
            || (*h).param.content_light_level.i_max_cll < 0 as ::core::ffi::c_int
            || (*h).param.content_light_level.i_max_fall > UINT16_MAX
            || (*h).param.content_light_level.i_max_fall < 0 as ::core::ffi::c_int)
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"content light levels out of range [0,%u]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            UINT16_MAX,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if b_open != 0 {
        let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        score
            += ((*h).param.analyse.i_me_range == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        score
            += ((*h).param.rc.i_qp_step == 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        score
            += ((*h).param.i_keyint_max == 12 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        score
            += ((*h).param.rc.i_qp_min == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        score
            += ((*h).param.rc.i_qp_max == 31 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        score
            += ((*h).param.rc.f_qcompress as ::core::ffi::c_double == 0.5f64)
                as ::core::ffi::c_int;
        score
            += (fabs((*h).param.rc.f_ip_factor as ::core::ffi::c_double - 1.25f64)
                < 0.01f64) as ::core::ffi::c_int;
        score
            += (fabs((*h).param.rc.f_pb_factor as ::core::ffi::c_double - 1.25f64)
                < 0.01f64) as ::core::ffi::c_int;
        score
            += ((*h).param.analyse.inter == 0 as ::core::ffi::c_uint
                && (*h).param.analyse.i_subpel_refine == 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        if score >= 5 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"broken ffmpeg default settings detected\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"use an encoding preset (e.g. -vpre medium)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"preset usage: -vpre <speed> -vpre <profile>\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"speed presets are listed in x264 --help\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"profile is optional; x264 defaults to high\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*h).param.rc.i_rc_method < 0 as ::core::ffi::c_int
        || (*h).param.rc.i_rc_method > 2 as ::core::ffi::c_int
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"no ratecontrol method specified\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.b_interlaced != 0 {
        (*h).param.b_pic_struct = 1 as ::core::ffi::c_int;
    }
    if (*h).param.i_avcintra_class != 0 {
        if BIT_DEPTH != 10 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"%2d-bit AVC-Intra is not widely compatible\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                BIT_DEPTH,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"10-bit x264 is required to encode AVC-Intra\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut type_0: ::core::ffi::c_int = if (*h).param.i_avcintra_class
            == 480 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else if (*h).param.i_avcintra_class == 300 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else if (*h).param.i_avcintra_class == 200 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int
        } else if (*h).param.i_avcintra_class == 100 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else if (*h).param.i_avcintra_class == 50 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
        if type_0 < 0 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Invalid AVC-Intra class\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        } else if type_0 > 2 as ::core::ffi::c_int
            && (*h).param.i_avcintra_flavor != X264_AVCINTRA_FLAVOR_SONY
        {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"AVC-Intra %d only supported by Sony XAVC flavor\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.i_avcintra_class,
            );
            return -(1 as ::core::ffi::c_int);
        }
        static mut avcintra_lut: [[[C2RustUnnamed_20; 7]; 2]; 5] = unsafe {
            [
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1100 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1100 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 912 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2196 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1820 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci50_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci50_p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2224 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 2224 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 1848 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3692 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 3724 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 4472 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_720p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_720p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 1 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080i_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 8940 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 7444 as uint16_t,
                                cqm_4iy: x264_cqm_jvt4i.as_ptr(),
                                cqm_4ic: x264_cqm_avci100_1080_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci100_1080p_8iy.as_ptr(),
                            };
                            init
                        },
                    ],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 9844 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [C2RustUnnamed_20 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: 0 as *const uint8_t,
                        cqm_4ic: 0 as *const uint8_t,
                        cqm_8iy: 0 as *const uint8_t,
                    }; 7],
                ],
                [
                    [
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 60000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 50 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 30000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 25 as uint16_t,
                                fps_den: 1 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        {
                            let mut init = C2RustUnnamed_20 {
                                fps_num: 24000 as uint16_t,
                                fps_den: 1001 as uint16_t,
                                interlaced: 0 as uint8_t,
                                frame_size: 15700 as uint16_t,
                                cqm_4iy: x264_cqm_avci300_2160p_4iy.as_ptr(),
                                cqm_4ic: x264_cqm_avci300_2160p_4ic.as_ptr(),
                                cqm_8iy: x264_cqm_avci300_2160p_8iy.as_ptr(),
                            };
                            init
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                        C2RustUnnamed_20 {
                            fps_num: 0,
                            fps_den: 0,
                            interlaced: 0,
                            frame_size: 0,
                            cqm_4iy: 0 as *const uint8_t,
                            cqm_4ic: 0 as *const uint8_t,
                            cqm_8iy: 0 as *const uint8_t,
                        },
                    ],
                    [C2RustUnnamed_20 {
                        fps_num: 0,
                        fps_den: 0,
                        interlaced: 0,
                        frame_size: 0,
                        cqm_4iy: 0 as *const uint8_t,
                        cqm_4ic: 0 as *const uint8_t,
                        cqm_8iy: 0 as *const uint8_t,
                    }; 7],
                ],
            ]
        };
        let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        if i_csp >= X264_CSP_I420 && i_csp < X264_CSP_I422 && type_0 == 0 {
            if (*h).param.i_width == 1440 as ::core::ffi::c_int
                && (*h).param.i_height == 1080 as ::core::ffi::c_int
            {
                res = 1 as ::core::ffi::c_int;
            } else if (*h).param.i_width == 960 as ::core::ffi::c_int
                && (*h).param.i_height == 720 as ::core::ffi::c_int
            {
                res = 0 as ::core::ffi::c_int;
            }
        } else if i_csp >= X264_CSP_I422 && i_csp < X264_CSP_I444 && type_0 != 0 {
            if type_0 < 3 as ::core::ffi::c_int {
                if (*h).param.i_width == 1920 as ::core::ffi::c_int
                    && (*h).param.i_height == 1080 as ::core::ffi::c_int
                {
                    res = 1 as ::core::ffi::c_int;
                } else if (*h).param.i_width == 2048 as ::core::ffi::c_int
                    && (*h).param.i_height == 1080 as ::core::ffi::c_int
                {
                    res = 1 as ::core::ffi::c_int;
                } else if (*h).param.i_width == 1280 as ::core::ffi::c_int
                    && (*h).param.i_height == 720 as ::core::ffi::c_int
                {
                    res = 0 as ::core::ffi::c_int;
                }
            } else if (*h).param.i_width == 3840 as ::core::ffi::c_int
                && (*h).param.i_height == 2160 as ::core::ffi::c_int
            {
                res = 0 as ::core::ffi::c_int;
            } else if (*h).param.i_width == 4096 as ::core::ffi::c_int
                && (*h).param.i_height == 2160 as ::core::ffi::c_int
            {
                res = 0 as ::core::ffi::c_int;
            }
        } else {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Invalid colorspace for AVC-Intra %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.i_avcintra_class,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if res < 0 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Resolution %dx%d invalid for AVC-Intra %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.i_width,
                (*h).param.i_height,
                (*h).param.i_avcintra_class,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.nalu_process.is_some() {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"nalu_process is not supported in AVC-Intra mode\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.b_repeat_headers == 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"Separate headers not supported in AVC-Intra mode\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut i: ::core::ffi::c_int = 0;
        let mut fps_num: uint32_t = (*h).param.i_fps_num;
        let mut fps_den: uint32_t = (*h).param.i_fps_den;
        x264_reduce_fraction(&mut fps_num, &mut fps_den);
        i = 0 as ::core::ffi::c_int;
        while i < 7 as ::core::ffi::c_int {
            if avcintra_lut[type_0 as usize][res as usize][i as usize].fps_num
                as uint32_t == fps_num
                && avcintra_lut[type_0 as usize][res as usize][i as usize].fps_den
                    as uint32_t == fps_den
                && avcintra_lut[type_0 as usize][res as usize][i as usize].interlaced
                    as ::core::ffi::c_int == (*h).param.b_interlaced
            {
                break;
            }
            i += 1;
        }
        if i == 7 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"FPS %d/%d%c not compatible with AVC-Intra %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.i_fps_num,
                (*h).param.i_fps_den,
                if (*h).param.b_interlaced != 0 { 'i' as i32 } else { 'p' as i32 },
                (*h).param.i_avcintra_class,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).param.i_keyint_max = 1 as ::core::ffi::c_int;
        (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
        (*h).param.analyse.i_weighted_pred = 0 as ::core::ffi::c_int;
        (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
        (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
        (*h).param.b_bluray_compat = 0 as ::core::ffi::c_int;
        (*h).param.b_vfr_input = 0 as ::core::ffi::c_int;
        (*h).param.b_aud = 1 as ::core::ffi::c_int;
        (*h).param.vui.i_chroma_loc = 0 as ::core::ffi::c_int;
        (*h).param.i_nal_hrd = X264_NAL_HRD_NONE;
        (*h).param.b_deblocking_filter = 0 as ::core::ffi::c_int;
        (*h).param.b_stitchable = 1 as ::core::ffi::c_int;
        (*h).param.b_pic_struct = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_transform_8x8 = 1 as ::core::ffi::c_int;
        (*h).param.analyse.intra = X264_ANALYSE_I8x8;
        (*h).param.analyse.i_chroma_qp_offset = if type_0 > 2 as ::core::ffi::c_int {
            -(4 as ::core::ffi::c_int)
        } else if res != 0 && type_0 != 0 {
            3 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        (*h).param.b_cabac = (type_0 == 0) as ::core::ffi::c_int;
        (*h).param.rc.i_vbv_buffer_size = avcintra_lut[type_0
                as usize][res as usize][i as usize]
            .frame_size as ::core::ffi::c_int;
        (*h).param.rc.i_bitrate = ((*h).param.rc.i_vbv_buffer_size as uint32_t)
            .wrapping_mul(fps_num)
            .wrapping_div(fps_den) as ::core::ffi::c_int;
        (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
        (*h).param.rc.i_rc_method = X264_RC_ABR;
        (*h).param.rc.f_vbv_buffer_init = 1.0f32;
        (*h).param.rc.b_filler = 1 as ::core::ffi::c_int;
        (*h).param.i_cqm_preset = X264_CQM_CUSTOM;
        memcpy(
            (*h).param.cqm_4iy.as_mut_ptr() as *mut ::core::ffi::c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4iy
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            (*h).param.cqm_4ic.as_mut_ptr() as *mut ::core::ffi::c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_4ic
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        memcpy(
            (*h).param.cqm_8iy.as_mut_ptr() as *mut ::core::ffi::c_void,
            avcintra_lut[type_0 as usize][res as usize][i as usize].cqm_8iy
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        );
        if (*h).param.i_avcintra_flavor == X264_AVCINTRA_FLAVOR_SONY {
            (*h).param.i_slice_count = 8 as ::core::ffi::c_int;
            if (*h).param.b_sliced_threads != 0 {
                (*h).param.i_threads = (*h).param.i_slice_count;
            }
        } else {
            (*h).param.i_slice_max_mbs = ((*h).param.i_width + 15 as ::core::ffi::c_int)
                / 16 as ::core::ffi::c_int
                * (((*h).param.i_height + 15 as ::core::ffi::c_int)
                    / 16 as ::core::ffi::c_int) / 10 as ::core::ffi::c_int;
            (*h).param.i_slice_max_size = 0 as ::core::ffi::c_int;
            if (*h).param.b_sliced_threads != 0 {
                if res != 0 {
                    (*h).param.i_threads = if (2 as ::core::ffi::c_int)
                        < (*h).param.i_threads
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (*h).param.i_threads
                    };
                } else {
                    (*h).param.i_threads = if (5 as ::core::ffi::c_int)
                        < (*h).param.i_threads
                    {
                        5 as ::core::ffi::c_int
                    } else {
                        (*h).param.i_threads
                    };
                    if (*h).param.i_threads < 5 as ::core::ffi::c_int {
                        (*h).param.i_threads = 1 as ::core::ffi::c_int;
                    }
                }
            }
            (*h).param.rc.i_qp_min = if (*h).param.rc.i_qp_min
                > 6 as ::core::ffi::c_int
                    * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int
            {
                (*h).param.rc.i_qp_min
            } else {
                6 as ::core::ffi::c_int
                    * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int
            };
        }
        if type_0 != 0 {
            (*h).param.vui.i_sar_height = 1 as ::core::ffi::c_int;
            (*h).param.vui.i_sar_width = (*h).param.vui.i_sar_height;
        } else {
            (*h).param.vui.i_sar_width = 4 as ::core::ffi::c_int;
            (*h).param.vui.i_sar_height = 3 as ::core::ffi::c_int;
        }
    }
    (*h).param.rc.f_rf_constant = x264_clip3f(
        (*h).param.rc.f_rf_constant as ::core::ffi::c_double,
        -QP_BD_OFFSET as ::core::ffi::c_double,
        51 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    (*h).param.rc.f_rf_constant_max = x264_clip3f(
        (*h).param.rc.f_rf_constant_max as ::core::ffi::c_double,
        -QP_BD_OFFSET as ::core::ffi::c_double,
        51 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    (*h).param.rc.i_qp_constant = x264_clip3(
        (*h).param.rc.i_qp_constant,
        -(1 as ::core::ffi::c_int),
        QP_MAX,
    );
    (*h).param.analyse.i_subpel_refine = x264_clip3(
        (*h).param.analyse.i_subpel_refine,
        0 as ::core::ffi::c_int,
        11 as ::core::ffi::c_int,
    );
    (*h).param.rc.f_ip_factor = x264_clip3f(
        (*h).param.rc.f_ip_factor as ::core::ffi::c_double,
        0.01f64,
        10.0f64,
    ) as ::core::ffi::c_float;
    (*h).param.rc.f_pb_factor = x264_clip3f(
        (*h).param.rc.f_pb_factor as ::core::ffi::c_double,
        0.01f64,
        10.0f64,
    ) as ::core::ffi::c_float;
    if (*h).param.rc.i_rc_method == X264_RC_CRF {
        (*h).param.rc.i_qp_constant = ((*h).param.rc.f_rf_constant
            + QP_BD_OFFSET as ::core::ffi::c_float) as ::core::ffi::c_int;
        (*h).param.rc.i_bitrate = 0 as ::core::ffi::c_int;
    }
    if b_open != 0
        && ((*h).param.rc.i_rc_method == X264_RC_CQP
            || (*h).param.rc.i_rc_method == X264_RC_CRF)
        && (*h).param.rc.i_qp_constant == 0 as ::core::ffi::c_int
    {
        (*h).mb.b_lossless = 1 as ::core::ffi::c_int;
        (*h).param.i_cqm_preset = X264_CQM_FLAT;
        (*h).param.psz_cqm_file = 0 as *mut ::core::ffi::c_char;
        (*h).param.rc.i_rc_method = X264_RC_CQP;
        (*h).param.rc.f_ip_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
        (*h).param.rc.f_pb_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
        (*h).param.analyse.b_psnr = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_ssim = 0 as ::core::ffi::c_int;
        (*h).param.analyse.i_chroma_qp_offset = 0 as ::core::ffi::c_int;
        (*h).param.analyse.i_trellis = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_fast_pskip = 0 as ::core::ffi::c_int;
        (*h).param.analyse.i_noise_reduction = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_psy = 0 as ::core::ffi::c_int;
        (*h).param.i_bframe = 0 as ::core::ffi::c_int;
        if (*h).param.b_cabac == 0
            && (*h).param.analyse.i_subpel_refine < 6 as ::core::ffi::c_int
        {
            (*h).param.analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
        }
    }
    if (*h).param.rc.i_rc_method == X264_RC_CQP {
        let mut qp_p: ::core::ffi::c_float = (*h).param.rc.i_qp_constant
            as ::core::ffi::c_float;
        let mut qp_i: ::core::ffi::c_float = qp_p
            - 6 as ::core::ffi::c_int as ::core::ffi::c_float
                * log2f((*h).param.rc.f_ip_factor);
        let mut qp_b: ::core::ffi::c_float = qp_p
            + 6 as ::core::ffi::c_int as ::core::ffi::c_float
                * log2f((*h).param.rc.f_pb_factor);
        if qp_p < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"qp not specified\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).param.rc.i_qp_min = x264_clip3(
            (if qp_p < (if qp_i < qp_b { qp_i } else { qp_b }) {
                qp_p
            } else if qp_i < qp_b {
                qp_i
            } else {
                qp_b
            }) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            QP_MAX,
        );
        (*h).param.rc.i_qp_max = x264_clip3(
            ((if qp_p > (if qp_i > qp_b { qp_i } else { qp_b }) {
                qp_p
            } else {
                (if qp_i > qp_b { qp_i } else { qp_b })
            }) as ::core::ffi::c_double + 0.999f64) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            QP_MAX,
        );
        (*h).param.rc.i_aq_mode = 0 as ::core::ffi::c_int;
        (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
        (*h).param.rc.i_bitrate = 0 as ::core::ffi::c_int;
    }
    (*h).param.rc.i_qp_max = x264_clip3(
        (*h).param.rc.i_qp_max,
        0 as ::core::ffi::c_int,
        QP_MAX,
    );
    (*h).param.rc.i_qp_min = x264_clip3(
        (*h).param.rc.i_qp_min,
        0 as ::core::ffi::c_int,
        (*h).param.rc.i_qp_max,
    );
    (*h).param.rc.i_qp_step = x264_clip3(
        (*h).param.rc.i_qp_step,
        2 as ::core::ffi::c_int,
        QP_MAX,
    );
    (*h).param.rc.i_bitrate = x264_clip3(
        (*h).param.rc.i_bitrate,
        0 as ::core::ffi::c_int,
        2000000 as ::core::ffi::c_int,
    );
    if (*h).param.rc.i_rc_method == X264_RC_ABR && (*h).param.rc.i_bitrate == 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"bitrate not specified\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).param.rc.i_vbv_buffer_size = x264_clip3(
        (*h).param.rc.i_vbv_buffer_size,
        0 as ::core::ffi::c_int,
        2000000 as ::core::ffi::c_int,
    );
    (*h).param.rc.i_vbv_max_bitrate = x264_clip3(
        (*h).param.rc.i_vbv_max_bitrate,
        0 as ::core::ffi::c_int,
        2000000 as ::core::ffi::c_int,
    );
    (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
        (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        2000000 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    if (*h).param.rc.i_vbv_buffer_size != 0 {
        if (*h).param.rc.i_rc_method == X264_RC_CQP {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV is incompatible with constant QP, ignored.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*h).param.rc.i_vbv_max_bitrate = 0 as ::core::ffi::c_int;
            (*h).param.rc.i_vbv_buffer_size = 0 as ::core::ffi::c_int;
        } else if (*h).param.rc.i_vbv_max_bitrate == 0 as ::core::ffi::c_int {
            if (*h).param.rc.i_rc_method == X264_RC_ABR {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"VBV maxrate unspecified, assuming CBR\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
            } else {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"VBV bufsize set but maxrate unspecified, ignored\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*h).param.rc.i_vbv_buffer_size = 0 as ::core::ffi::c_int;
            }
        } else if (*h).param.rc.i_vbv_max_bitrate < (*h).param.rc.i_bitrate
            && (*h).param.rc.i_rc_method == X264_RC_ABR
        {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"max bitrate less than average bitrate, assuming CBR\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*h).param.rc.i_bitrate = (*h).param.rc.i_vbv_max_bitrate;
        }
    } else if (*h).param.rc.i_vbv_max_bitrate != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"VBV maxrate specified, but no bufsize, ignored\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.rc.i_vbv_max_bitrate = 0 as ::core::ffi::c_int;
    }
    (*h).param.i_slice_max_size = if (*h).param.i_slice_max_size
        > 0 as ::core::ffi::c_int
    {
        (*h).param.i_slice_max_size
    } else {
        0 as ::core::ffi::c_int
    };
    (*h).param.i_slice_max_mbs = if (*h).param.i_slice_max_mbs > 0 as ::core::ffi::c_int
    {
        (*h).param.i_slice_max_mbs
    } else {
        0 as ::core::ffi::c_int
    };
    (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs > 0 as ::core::ffi::c_int
    {
        (*h).param.i_slice_min_mbs
    } else {
        0 as ::core::ffi::c_int
    };
    if (*h).param.i_slice_max_mbs != 0 {
        (*h).param.i_slice_min_mbs = if (*h).param.i_slice_min_mbs
            < (*h).param.i_slice_max_mbs / 2 as ::core::ffi::c_int
        {
            (*h).param.i_slice_min_mbs
        } else {
            (*h).param.i_slice_max_mbs / 2 as ::core::ffi::c_int
        };
    } else if (*h).param.i_slice_max_size == 0 {
        (*h).param.i_slice_min_mbs = 0 as ::core::ffi::c_int;
    }
    if (*h).param.b_interlaced != 0 && (*h).param.i_slice_min_mbs != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"interlace + slice-min-mbs is not implemented\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_slice_min_mbs = 0 as ::core::ffi::c_int;
    }
    let mut mb_width: ::core::ffi::c_int = ((*h).param.i_width
        + 15 as ::core::ffi::c_int) / 16 as ::core::ffi::c_int;
    if (*h).param.i_slice_min_mbs > mb_width {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"slice-min-mbs > row mb size (%d) not implemented\n\0" as *const u8
                as *const ::core::ffi::c_char,
            mb_width,
        );
        (*h).param.i_slice_min_mbs = mb_width;
    }
    let mut max_slices: ::core::ffi::c_int = ((*h).param.i_height
        + (((16 as ::core::ffi::c_int) << (*h).param.b_interlaced)
            - 1 as ::core::ffi::c_int))
        / ((16 as ::core::ffi::c_int) << (*h).param.b_interlaced);
    if (*h).param.b_sliced_threads != 0 {
        (*h).param.i_slice_count = x264_clip3(
            (*h).param.i_threads,
            0 as ::core::ffi::c_int,
            max_slices,
        );
    } else {
        (*h).param.i_slice_count = x264_clip3(
            (*h).param.i_slice_count,
            0 as ::core::ffi::c_int,
            max_slices,
        );
        if (*h).param.i_slice_max_mbs != 0 || (*h).param.i_slice_max_size != 0 {
            (*h).param.i_slice_count = 0 as ::core::ffi::c_int;
        }
    }
    if (*h).param.i_slice_count_max > 0 as ::core::ffi::c_int {
        (*h).param.i_slice_count_max = if (*h).param.i_slice_count
            > (*h).param.i_slice_count_max
        {
            (*h).param.i_slice_count
        } else {
            (*h).param.i_slice_count_max
        };
    }
    if (*h).param.b_bluray_compat != 0 {
        (*h).param.i_bframe_pyramid = if (1 as ::core::ffi::c_int)
            < (*h).param.i_bframe_pyramid
        {
            1 as ::core::ffi::c_int
        } else {
            (*h).param.i_bframe_pyramid
        };
        (*h).param.i_bframe = if (*h).param.i_bframe < 3 as ::core::ffi::c_int {
            (*h).param.i_bframe
        } else {
            3 as ::core::ffi::c_int
        };
        (*h).param.b_aud = 1 as ::core::ffi::c_int;
        (*h).param.i_nal_hrd = if (*h).param.i_nal_hrd > 1 as ::core::ffi::c_int {
            (*h).param.i_nal_hrd
        } else {
            1 as ::core::ffi::c_int
        };
        (*h).param.i_slice_max_size = 0 as ::core::ffi::c_int;
        (*h).param.i_slice_max_mbs = 0 as ::core::ffi::c_int;
        (*h).param.b_intra_refresh = 0 as ::core::ffi::c_int;
        (*h).param.i_frame_reference = if (*h).param.i_frame_reference
            < 6 as ::core::ffi::c_int
        {
            (*h).param.i_frame_reference
        } else {
            6 as ::core::ffi::c_int
        };
        (*h).param.i_dpb_size = if (*h).param.i_dpb_size < 6 as ::core::ffi::c_int {
            (*h).param.i_dpb_size
        } else {
            6 as ::core::ffi::c_int
        };
        (*h).param.i_keyint_min = 1 as ::core::ffi::c_int;
        (*h).param.analyse.i_weighted_pred = if (*h).param.analyse.i_weighted_pred
            < 1 as ::core::ffi::c_int
        {
            (*h).param.analyse.i_weighted_pred
        } else {
            1 as ::core::ffi::c_int
        };
        if (*h).param.b_fake_interlaced != 0 {
            (*h).param.b_pic_struct = 1 as ::core::ffi::c_int;
        }
    }
    (*h).param.i_frame_reference = x264_clip3(
        (*h).param.i_frame_reference,
        1 as ::core::ffi::c_int,
        X264_REF_MAX,
    );
    (*h).param.i_dpb_size = x264_clip3(
        (*h).param.i_dpb_size,
        1 as ::core::ffi::c_int,
        X264_REF_MAX,
    );
    if (*h).param.i_scenecut_threshold < 0 as ::core::ffi::c_int {
        (*h).param.i_scenecut_threshold = 0 as ::core::ffi::c_int;
    }
    (*h).param.analyse.i_direct_mv_pred = x264_clip3(
        (*h).param.analyse.i_direct_mv_pred,
        X264_DIRECT_PRED_NONE,
        X264_DIRECT_PRED_AUTO,
    );
    if (*h).param.analyse.i_subpel_refine == 0
        && (*h).param.analyse.i_direct_mv_pred > X264_DIRECT_PRED_SPATIAL
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"subme=0 + direct=temporal is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.analyse.i_direct_mv_pred = X264_DIRECT_PRED_SPATIAL;
    }
    (*h).param.i_bframe = x264_clip3(
        (*h).param.i_bframe,
        0 as ::core::ffi::c_int,
        if (16 as ::core::ffi::c_int) < (*h).param.i_keyint_max - 1 as ::core::ffi::c_int
        {
            16 as ::core::ffi::c_int
        } else {
            (*h).param.i_keyint_max - 1 as ::core::ffi::c_int
        },
    );
    (*h).param.i_bframe_bias = x264_clip3(
        (*h).param.i_bframe_bias,
        -(90 as ::core::ffi::c_int),
        100 as ::core::ffi::c_int,
    );
    if (*h).param.i_bframe <= 1 as ::core::ffi::c_int {
        (*h).param.i_bframe_pyramid = X264_B_PYRAMID_NONE;
    }
    (*h).param.i_bframe_pyramid = x264_clip3(
        (*h).param.i_bframe_pyramid,
        X264_B_PYRAMID_NONE,
        X264_B_PYRAMID_NORMAL,
    );
    (*h).param.i_bframe_adaptive = x264_clip3(
        (*h).param.i_bframe_adaptive,
        X264_B_ADAPT_NONE,
        X264_B_ADAPT_TRELLIS,
    );
    if (*h).param.i_bframe == 0 {
        (*h).param.i_bframe_adaptive = X264_B_ADAPT_NONE;
        (*h).param.analyse.i_direct_mv_pred = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_weighted_bipred = 0 as ::core::ffi::c_int;
        (*h).param.b_open_gop = 0 as ::core::ffi::c_int;
    }
    if (*h).param.b_intra_refresh != 0
        && (*h).param.i_bframe_pyramid == X264_B_PYRAMID_NORMAL
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"b-pyramid normal + intra-refresh is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_bframe_pyramid = X264_B_PYRAMID_STRICT;
    }
    if (*h).param.b_intra_refresh != 0
        && ((*h).param.i_frame_reference > 1 as ::core::ffi::c_int
            || (*h).param.i_dpb_size > 1 as ::core::ffi::c_int)
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"ref > 1 + intra-refresh is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_frame_reference = 1 as ::core::ffi::c_int;
        (*h).param.i_dpb_size = 1 as ::core::ffi::c_int;
    }
    if (*h).param.b_intra_refresh != 0 && (*h).param.b_open_gop != 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"intra-refresh is not compatible with open-gop\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.b_open_gop = 0 as ::core::ffi::c_int;
    }
    if (*h).param.i_fps_num == 0 || (*h).param.i_fps_den == 0 {
        (*h).param.i_fps_num = 25 as uint32_t;
        (*h).param.i_fps_den = 1 as uint32_t;
    }
    let mut fps: ::core::ffi::c_float = (*h).param.i_fps_num as ::core::ffi::c_float
        / (*h).param.i_fps_den as ::core::ffi::c_float;
    if (*h).param.i_keyint_min == X264_KEYINT_MIN_AUTO {
        (*h).param.i_keyint_min = if ((*h).param.i_keyint_max / 10 as ::core::ffi::c_int)
            < fps as ::core::ffi::c_int
        {
            (*h).param.i_keyint_max / 10 as ::core::ffi::c_int
        } else {
            fps as ::core::ffi::c_int
        };
    }
    (*h).param.i_keyint_min = x264_clip3(
        (*h).param.i_keyint_min,
        1 as ::core::ffi::c_int,
        (*h).param.i_keyint_max / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    );
    (*h).param.rc.i_lookahead = x264_clip3(
        (*h).param.rc.i_lookahead,
        0 as ::core::ffi::c_int,
        X264_LOOKAHEAD_MAX,
    );
    let mut maxrate: ::core::ffi::c_int = if (*h).param.rc.i_vbv_max_bitrate
        > (*h).param.rc.i_bitrate
    {
        (*h).param.rc.i_vbv_max_bitrate
    } else {
        (*h).param.rc.i_bitrate
    };
    let mut bufsize: ::core::ffi::c_float = if maxrate != 0 {
        (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_float
            / maxrate as ::core::ffi::c_float
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_float
    };
    (*h).param.rc.i_lookahead = (if ((*h).param.rc.i_lookahead as ::core::ffi::c_float)
        < (if (*h).param.i_keyint_max as ::core::ffi::c_float > bufsize * fps {
            (*h).param.i_keyint_max as ::core::ffi::c_float
        } else {
            bufsize * fps
        })
    {
        (*h).param.rc.i_lookahead as ::core::ffi::c_float
    } else if (*h).param.i_keyint_max as ::core::ffi::c_float > bufsize * fps {
        (*h).param.i_keyint_max as ::core::ffi::c_float
    } else {
        bufsize * fps
    }) as ::core::ffi::c_int;
    if (*h).param.i_timebase_num == 0 || (*h).param.i_timebase_den == 0
        || !((*h).param.b_vfr_input != 0 || (*h).param.b_pulldown != 0)
    {
        (*h).param.i_timebase_num = (*h).param.i_fps_den;
        (*h).param.i_timebase_den = (*h).param.i_fps_num;
    }
    (*h).param.rc.f_qcompress = x264_clip3f(
        (*h).param.rc.f_qcompress as ::core::ffi::c_double,
        0.0f64,
        1.0f64,
    ) as ::core::ffi::c_float;
    if (*h).param.i_keyint_max == 1 as ::core::ffi::c_int
        || (*h).param.rc.f_qcompress == 1 as ::core::ffi::c_int as ::core::ffi::c_float
    {
        (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
    }
    if (*h).param.b_intra_refresh == 0
        && (*h).param.i_keyint_max != X264_KEYINT_MAX_INFINITE
        && (*h).param.rc.i_lookahead == 0 && (*h).param.rc.b_mb_tree != 0
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"lookaheadless mb-tree requires intra refresh or infinite keyint\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        (*h).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
    }
    if b_open != 0 && (*h).param.rc.b_stat_read != 0 {
        (*h).param.rc.i_lookahead = 0 as ::core::ffi::c_int;
    }
    if (*h).param.i_sync_lookahead < 0 as ::core::ffi::c_int {
        (*h).param.i_sync_lookahead = (*h).param.i_bframe + 1 as ::core::ffi::c_int;
    }
    (*h).param.i_sync_lookahead = if (*h).param.i_sync_lookahead
        < 250 as ::core::ffi::c_int
    {
        (*h).param.i_sync_lookahead
    } else {
        250 as ::core::ffi::c_int
    };
    if (*h).param.rc.b_stat_read != 0 || (*h).i_thread_frames == 1 as ::core::ffi::c_int
    {
        (*h).param.i_sync_lookahead = 0 as ::core::ffi::c_int;
    }
    (*h).param.i_deblocking_filter_alphac0 = x264_clip3(
        (*h).param.i_deblocking_filter_alphac0,
        -(6 as ::core::ffi::c_int),
        6 as ::core::ffi::c_int,
    );
    (*h).param.i_deblocking_filter_beta = x264_clip3(
        (*h).param.i_deblocking_filter_beta,
        -(6 as ::core::ffi::c_int),
        6 as ::core::ffi::c_int,
    );
    (*h).param.analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize] = x264_clip3(
        (*h).param.analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
    );
    (*h).param.analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize] = x264_clip3(
        (*h).param.analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
    );
    (*h).param.i_cabac_init_idc = x264_clip3(
        (*h).param.i_cabac_init_idc,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    );
    if (*h).param.i_cqm_preset < X264_CQM_FLAT
        || (*h).param.i_cqm_preset > X264_CQM_CUSTOM
    {
        (*h).param.i_cqm_preset = X264_CQM_FLAT;
    }
    if (*h).param.analyse.i_me_method < X264_ME_DIA
        || (*h).param.analyse.i_me_method > X264_ME_TESA
    {
        (*h).param.analyse.i_me_method = X264_ME_HEX;
    }
    (*h).param.analyse.i_me_range = x264_clip3(
        (*h).param.analyse.i_me_range,
        4 as ::core::ffi::c_int,
        1024 as ::core::ffi::c_int,
    );
    if (*h).param.analyse.i_me_range > 16 as ::core::ffi::c_int
        && (*h).param.analyse.i_me_method <= X264_ME_HEX
    {
        (*h).param.analyse.i_me_range = 16 as ::core::ffi::c_int;
    }
    if (*h).param.analyse.i_me_method == X264_ME_TESA
        && ((*h).mb.b_lossless != 0
            || (*h).param.analyse.i_subpel_refine <= 1 as ::core::ffi::c_int)
    {
        (*h).param.analyse.i_me_method = X264_ME_ESA;
    }
    (*h).param.analyse.b_mixed_references = ((*h).param.analyse.b_mixed_references != 0
        && (*h).param.i_frame_reference > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*h).param.analyse.inter
        &= X264_ANALYSE_PSUB16x16 | X264_ANALYSE_PSUB8x8 | X264_ANALYSE_BSUB16x16
            | X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
    (*h).param.analyse.intra &= X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
    if (*h).param.analyse.inter & X264_ANALYSE_PSUB16x16 == 0 {
        (*h).param.analyse.inter &= !X264_ANALYSE_PSUB8x8;
    }
    if (*h).param.analyse.b_transform_8x8 == 0 {
        (*h).param.analyse.inter &= !X264_ANALYSE_I8x8;
        (*h).param.analyse.intra &= !X264_ANALYSE_I8x8;
    }
    (*h).param.analyse.i_trellis = x264_clip3(
        (*h).param.analyse.i_trellis,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
    );
    (*h).param.rc.i_aq_mode = x264_clip3(
        (*h).param.rc.i_aq_mode,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    );
    (*h).param.rc.f_aq_strength = x264_clip3f(
        (*h).param.rc.f_aq_strength as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        3 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    if (*h).param.rc.f_aq_strength == 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*h).param.rc.i_aq_mode = 0 as ::core::ffi::c_int;
    }
    if (*h).param.i_log_level < X264_LOG_INFO {
        (*h).param.analyse.b_psnr = 0 as ::core::ffi::c_int;
        (*h).param.analyse.b_ssim = 0 as ::core::ffi::c_int;
    }
    if b_open != 0 && ((*h).param.analyse.b_psnr != 0 || (*h).param.analyse.b_ssim != 0)
    {
        let mut s: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        if (*h).param.analyse.b_psy != 0 {
            s = (if (*h).param.analyse.b_psnr != 0 {
                b"psnr\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"ssim\0" as *const u8 as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--%s used with psy on: results will be invalid!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                s,
            );
        } else if (*h).param.rc.i_aq_mode == 0 && (*h).param.analyse.b_ssim != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--ssim used with AQ off: results will be invalid!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            s = b"ssim\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else if (*h).param.rc.i_aq_mode != 0 && (*h).param.analyse.b_psnr != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--psnr used with AQ on: results will be invalid!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            s = b"psnr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        if !s.is_null() {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"--tune %s should be used if attempting to benchmark %s!\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                s,
                s,
            );
        }
    }
    if (*h).param.analyse.b_psy == 0 {
        (*h).param.analyse.f_psy_rd = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
        (*h).param.analyse.f_psy_trellis = 0 as ::core::ffi::c_int
            as ::core::ffi::c_float;
    }
    (*h).param.analyse.f_psy_rd = x264_clip3f(
        (*h).param.analyse.f_psy_rd as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        10 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    (*h).param.analyse.f_psy_trellis = x264_clip3f(
        (*h).param.analyse.f_psy_trellis as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        10 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    (*h).mb.i_psy_rd = if (*h).param.analyse.i_subpel_refine >= 6 as ::core::ffi::c_int {
        (((*h).param.analyse.f_psy_rd
            * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_float) as ::core::ffi::c_double + 0.5f64)
            as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*h).mb.i_psy_trellis = if (*h).param.analyse.i_trellis != 0 {
        (((*h).param.analyse.f_psy_trellis
            / 4 as ::core::ffi::c_int as ::core::ffi::c_float
            * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_float) as ::core::ffi::c_double + 0.5f64)
            as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
        (*h).param.analyse.i_chroma_qp_offset,
        -(32 as ::core::ffi::c_int),
        32 as ::core::ffi::c_int,
    );
    if b_open != 0 && i_csp >= X264_CSP_I444 && i_csp < X264_CSP_BGR
        && (*h).param.analyse.b_psy != 0
    {
        (*h).param.analyse.i_chroma_qp_offset += 6 as ::core::ffi::c_int;
    }
    if b_open != 0 && (*h).mb.i_psy_rd != 0 && (*h).param.i_avcintra_class == 0 {
        (*h).param.analyse.i_chroma_qp_offset
            -= if ((*h).param.analyse.f_psy_rd as ::core::ffi::c_double) < 0.25f64 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
    }
    if b_open != 0 && (*h).mb.i_psy_trellis != 0 && (*h).param.i_avcintra_class == 0 {
        (*h).param.analyse.i_chroma_qp_offset
            -= if ((*h).param.analyse.f_psy_trellis as ::core::ffi::c_double) < 0.25f64 {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
    }
    (*h).param.analyse.i_chroma_qp_offset = x264_clip3(
        (*h).param.analyse.i_chroma_qp_offset,
        -(12 as ::core::ffi::c_int),
        12 as ::core::ffi::c_int,
    );
    if (*h).param.rc.i_aq_mode == 0 && (*h).param.rc.b_mb_tree != 0 {
        (*h).param.rc.i_aq_mode = 1 as ::core::ffi::c_int;
        (*h).param.rc.f_aq_strength = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    (*h).param.analyse.i_noise_reduction = x264_clip3(
        (*h).param.analyse.i_noise_reduction,
        0 as ::core::ffi::c_int,
        (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
    );
    if (*h).param.analyse.i_subpel_refine >= 10 as ::core::ffi::c_int
        && ((*h).param.analyse.i_trellis != 2 as ::core::ffi::c_int
            || (*h).param.rc.i_aq_mode == 0)
    {
        (*h).param.analyse.i_subpel_refine = 9 as ::core::ffi::c_int;
    }
    if b_open != 0 {
        let mut l: *const x264_level_t = x264_levels.as_ptr();
        if (*h).param.i_level_idc < 0 as ::core::ffi::c_int {
            let mut maxrate_bak: ::core::ffi::c_int = (*h).param.rc.i_vbv_max_bitrate;
            if (*h).param.rc.i_rc_method == X264_RC_ABR
                && (*h).param.rc.i_vbv_buffer_size <= 0 as ::core::ffi::c_int
            {
                (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate
                    * 2 as ::core::ffi::c_int;
            }
            x264_10_sps_init(
                (*h).sps.as_mut_ptr(),
                (*h).param.i_sps_id,
                &mut (*h).param,
            );
            loop {
                (*h).param.i_level_idc = (*l).level_idc as ::core::ffi::c_int;
                if !((*l.offset(1 as ::core::ffi::c_int as isize)).level_idc
                    as ::core::ffi::c_int != 0
                    && x264_10_validate_levels(h, 0 as ::core::ffi::c_int) != 0
                    && {
                        let fresh0 = l;
                        l = l.offset(1);
                        !fresh0.is_null()
                    })
                {
                    break;
                }
            }
            (*h).param.rc.i_vbv_max_bitrate = maxrate_bak;
        } else {
            while (*l).level_idc as ::core::ffi::c_int != 0
                && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
            {
                l = l.offset(1);
            }
            if (*l).level_idc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"invalid level_idc: %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*h).param.i_level_idc,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*h).param.analyse.i_mv_range <= 0 as ::core::ffi::c_int {
            (*h).param.analyse.i_mv_range = (*l).mv_range as ::core::ffi::c_int
                >> (*h).param.b_interlaced;
        } else {
            (*h).param.analyse.i_mv_range = x264_clip3(
                (*h).param.analyse.i_mv_range,
                32 as ::core::ffi::c_int,
                8192 as ::core::ffi::c_int >> (*h).param.b_interlaced,
            );
        }
    }
    (*h).param.analyse.i_weighted_pred = x264_clip3(
        (*h).param.analyse.i_weighted_pred,
        X264_WEIGHTP_NONE,
        X264_WEIGHTP_SMART,
    );
    if (*h).param.i_lookahead_threads == X264_THREADS_AUTO {
        if (*h).param.b_sliced_threads != 0 {
            (*h).param.i_lookahead_threads = (*h).param.i_threads;
        } else {
            let mut badapt: ::core::ffi::c_int = ((*h).param.i_bframe_adaptive
                == X264_B_ADAPT_TRELLIS) as ::core::ffi::c_int;
            let mut subme: ::core::ffi::c_int = (if ((*h).param.analyse.i_subpel_refine
                / 3 as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
            {
                (*h).param.analyse.i_subpel_refine / 3 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            })
                + ((*h).param.analyse.i_subpel_refine > 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            let mut bframes: ::core::ffi::c_int = if (((*h).param.i_bframe
                - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int)
                < 3 as ::core::ffi::c_int
            {
                ((*h).param.i_bframe - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            };
            static mut lookahead_thread_div: [[[uint8_t; 4]; 5]; 2] = [
                [
                    [
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        3 as ::core::ffi::c_int as uint8_t,
                        3 as ::core::ffi::c_int as uint8_t,
                        3 as ::core::ffi::c_int as uint8_t,
                        3 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        4 as ::core::ffi::c_int as uint8_t,
                        4 as ::core::ffi::c_int as uint8_t,
                        4 as ::core::ffi::c_int as uint8_t,
                        4 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        12 as ::core::ffi::c_int as uint8_t,
                        12 as ::core::ffi::c_int as uint8_t,
                        12 as ::core::ffi::c_int as uint8_t,
                        12 as ::core::ffi::c_int as uint8_t,
                    ],
                ],
                [
                    [
                        3 as ::core::ffi::c_int as uint8_t,
                        2 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        2 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        4 as ::core::ffi::c_int as uint8_t,
                        3 as ::core::ffi::c_int as uint8_t,
                        2 as ::core::ffi::c_int as uint8_t,
                        1 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        6 as ::core::ffi::c_int as uint8_t,
                        4 as ::core::ffi::c_int as uint8_t,
                        3 as ::core::ffi::c_int as uint8_t,
                        2 as ::core::ffi::c_int as uint8_t,
                    ],
                    [
                        12 as ::core::ffi::c_int as uint8_t,
                        9 as ::core::ffi::c_int as uint8_t,
                        6 as ::core::ffi::c_int as uint8_t,
                        4 as ::core::ffi::c_int as uint8_t,
                    ],
                ],
            ];
            (*h).param.i_lookahead_threads = (*h).param.i_threads
                / lookahead_thread_div[badapt as usize][subme as usize][bframes as usize]
                    as ::core::ffi::c_int;
            (*h).param.i_lookahead_threads = if (*h).param.i_lookahead_threads
                < (*h).param.i_height / 128 as ::core::ffi::c_int
            {
                (*h).param.i_lookahead_threads
            } else {
                (*h).param.i_height / 128 as ::core::ffi::c_int
            };
        }
    }
    (*h).param.i_lookahead_threads = x264_clip3(
        (*h).param.i_lookahead_threads,
        1 as ::core::ffi::c_int,
        if max_sliced_threads < 16 as ::core::ffi::c_int {
            max_sliced_threads
        } else {
            16 as ::core::ffi::c_int
        },
    );
    if (*h).param.b_interlaced != 0 {
        if (*h).param.analyse.i_me_method >= X264_ME_ESA {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlace + me=esa is not implemented\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*h).param.analyse.i_me_method = X264_ME_UMH;
        }
        if (*h).param.analyse.i_weighted_pred > 0 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlace + weightp is not implemented\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*h).param.analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        }
    }
    if (*h).param.analyse.i_weighted_pred == 0 && (*h).param.rc.b_mb_tree != 0
        && (*h).param.analyse.b_psy != 0
    {
        (*h).param.analyse.i_weighted_pred = X264_WEIGHTP_FAKE;
    }
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        let mut r: ::core::ffi::c_int = (*h).param.analyse.i_mv_range_thread;
        let mut r2: ::core::ffi::c_int = 0;
        if r <= 0 as ::core::ffi::c_int {
            let mut max_range: ::core::ffi::c_int = ((*h).param.i_height
                + X264_THREAD_HEIGHT) / (*h).i_thread_frames - X264_THREAD_HEIGHT;
            r = max_range / 2 as ::core::ffi::c_int;
        }
        r = if r > (*h).param.analyse.i_me_range {
            r
        } else {
            (*h).param.analyse.i_me_range
        };
        r = if r < (*h).param.analyse.i_mv_range {
            r
        } else {
            (*h).param.analyse.i_mv_range
        };
        r2 = (r & !(15 as ::core::ffi::c_int))
            + (-X264_THREAD_HEIGHT & 15 as ::core::ffi::c_int);
        if r2 < r {
            r2 += 16 as ::core::ffi::c_int;
        }
        x264_10_log(
            h,
            X264_LOG_DEBUG,
            b"using mv_range_thread = %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            r2,
        );
        (*h).param.analyse.i_mv_range_thread = r2;
    }
    if (*h).param.rc.f_rate_tolerance < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*h).param.rc.f_rate_tolerance = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*h).param.rc.f_qblur < 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        (*h).param.rc.f_qblur = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    if (*h).param.rc.f_complexity_blur < 0 as ::core::ffi::c_int as ::core::ffi::c_float
    {
        (*h).param.rc.f_complexity_blur = 0 as ::core::ffi::c_int
            as ::core::ffi::c_float;
    }
    (*h).param.i_sps_id &= 31 as ::core::ffi::c_int;
    (*h).param.i_nal_hrd = x264_clip3(
        (*h).param.i_nal_hrd,
        X264_NAL_HRD_NONE,
        X264_NAL_HRD_CBR,
    );
    if (*h).param.i_nal_hrd != 0 && (*h).param.rc.i_vbv_buffer_size == 0 {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"NAL HRD parameters require VBV parameters\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_nal_hrd = X264_NAL_HRD_NONE;
    }
    if (*h).param.i_nal_hrd == X264_NAL_HRD_CBR
        && ((*h).param.rc.i_bitrate != (*h).param.rc.i_vbv_max_bitrate
            || (*h).param.rc.i_vbv_max_bitrate == 0)
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"CBR HRD requires constant bitrate\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*h).param.i_nal_hrd = X264_NAL_HRD_VBR;
    }
    if (*h).param.i_nal_hrd == X264_NAL_HRD_CBR {
        (*h).param.rc.b_filler = 1 as ::core::ffi::c_int;
    }
    (*h).param.b_cabac = ((*h).param.b_cabac != 0) as ::core::ffi::c_int;
    (*h).param.b_constrained_intra = ((*h).param.b_constrained_intra != 0)
        as ::core::ffi::c_int;
    (*h).param.b_deblocking_filter = ((*h).param.b_deblocking_filter != 0)
        as ::core::ffi::c_int;
    (*h).param.b_deterministic = ((*h).param.b_deterministic != 0) as ::core::ffi::c_int;
    (*h).param.b_sliced_threads = ((*h).param.b_sliced_threads != 0)
        as ::core::ffi::c_int;
    (*h).param.b_interlaced = ((*h).param.b_interlaced != 0) as ::core::ffi::c_int;
    (*h).param.b_intra_refresh = ((*h).param.b_intra_refresh != 0) as ::core::ffi::c_int;
    (*h).param.b_aud = ((*h).param.b_aud != 0) as ::core::ffi::c_int;
    (*h).param.b_repeat_headers = ((*h).param.b_repeat_headers != 0)
        as ::core::ffi::c_int;
    (*h).param.b_annexb = ((*h).param.b_annexb != 0) as ::core::ffi::c_int;
    (*h).param.b_vfr_input = ((*h).param.b_vfr_input != 0) as ::core::ffi::c_int;
    (*h).param.b_pulldown = ((*h).param.b_pulldown != 0) as ::core::ffi::c_int;
    (*h).param.b_tff = ((*h).param.b_tff != 0) as ::core::ffi::c_int;
    (*h).param.b_pic_struct = ((*h).param.b_pic_struct != 0) as ::core::ffi::c_int;
    (*h).param.b_fake_interlaced = ((*h).param.b_fake_interlaced != 0)
        as ::core::ffi::c_int;
    (*h).param.b_open_gop = ((*h).param.b_open_gop != 0) as ::core::ffi::c_int;
    (*h).param.b_bluray_compat = ((*h).param.b_bluray_compat != 0) as ::core::ffi::c_int;
    (*h).param.b_stitchable = ((*h).param.b_stitchable != 0) as ::core::ffi::c_int;
    (*h).param.b_full_recon = ((*h).param.b_full_recon != 0) as ::core::ffi::c_int;
    (*h).param.b_opencl = ((*h).param.b_opencl != 0) as ::core::ffi::c_int;
    (*h).param.analyse.b_transform_8x8 = ((*h).param.analyse.b_transform_8x8 != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_weighted_bipred = ((*h).param.analyse.b_weighted_bipred != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_chroma_me = ((*h).param.analyse.b_chroma_me != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_mixed_references = ((*h).param.analyse.b_mixed_references != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_fast_pskip = ((*h).param.analyse.b_fast_pskip != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_dct_decimate = ((*h).param.analyse.b_dct_decimate != 0)
        as ::core::ffi::c_int;
    (*h).param.analyse.b_psy = ((*h).param.analyse.b_psy != 0) as ::core::ffi::c_int;
    (*h).param.analyse.b_psnr = ((*h).param.analyse.b_psnr != 0) as ::core::ffi::c_int;
    (*h).param.analyse.b_ssim = ((*h).param.analyse.b_ssim != 0) as ::core::ffi::c_int;
    (*h).param.rc.b_stat_write = ((*h).param.rc.b_stat_write != 0) as ::core::ffi::c_int;
    (*h).param.rc.b_stat_read = ((*h).param.rc.b_stat_read != 0) as ::core::ffi::c_int;
    (*h).param.rc.b_mb_tree = ((*h).param.rc.b_mb_tree != 0) as ::core::ffi::c_int;
    (*h).param.rc.b_filler = ((*h).param.rc.b_filler != 0) as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "467:9"]
pub const MAX_RESOLUTION: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
#[c2rust::src_loc = "1409:1"]
unsafe extern "C" fn mbcmp_init(mut h: *mut x264_t) {
    let mut satd: ::core::ffi::c_int = ((*h).mb.b_lossless == 0
        && (*h).param.analyse.i_subpel_refine > 1 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    memcpy(
        (*h).pixf.mbcmp.as_mut_ptr() as *mut ::core::ffi::c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad_aligned.as_mut_ptr()
        }) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    memcpy(
        (*h).pixf.mbcmp_unaligned.as_mut_ptr() as *mut ::core::ffi::c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad.as_mut_ptr()
        }) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    (*h).pixf.intra_mbcmp_x3_16x16 = if satd != 0 {
        (*h).pixf.intra_satd_x3_16x16
    } else {
        (*h).pixf.intra_sad_x3_16x16
    };
    (*h).pixf.intra_mbcmp_x3_8x16c = if satd != 0 {
        (*h).pixf.intra_satd_x3_8x16c
    } else {
        (*h).pixf.intra_sad_x3_8x16c
    };
    (*h).pixf.intra_mbcmp_x3_8x8c = if satd != 0 {
        (*h).pixf.intra_satd_x3_8x8c
    } else {
        (*h).pixf.intra_sad_x3_8x8c
    };
    (*h).pixf.intra_mbcmp_x3_8x8 = if satd != 0 {
        (*h).pixf.intra_sa8d_x3_8x8
    } else {
        (*h).pixf.intra_sad_x3_8x8
    };
    (*h).pixf.intra_mbcmp_x3_4x4 = if satd != 0 {
        (*h).pixf.intra_satd_x3_4x4
    } else {
        (*h).pixf.intra_sad_x3_4x4
    };
    (*h).pixf.intra_mbcmp_x9_4x4 = if (*h).param.b_cpu_independent != 0
        || (*h).mb.b_lossless != 0
    {
        None
    } else if satd != 0 {
        (*h).pixf.intra_satd_x9_4x4
    } else {
        (*h).pixf.intra_sad_x9_4x4
    };
    (*h).pixf.intra_mbcmp_x9_8x8 = if (*h).param.b_cpu_independent != 0
        || (*h).mb.b_lossless != 0
    {
        None
    } else if satd != 0 {
        (*h).pixf.intra_sa8d_x9_8x8
    } else {
        (*h).pixf.intra_sad_x9_8x8
    };
    satd &= ((*h).param.analyse.i_me_method == X264_ME_TESA) as ::core::ffi::c_int;
    memcpy(
        (*h).pixf.fpelcmp.as_mut_ptr() as *mut ::core::ffi::c_void,
        (if satd != 0 {
            (*h).pixf.satd.as_mut_ptr()
        } else {
            (*h).pixf.sad.as_mut_ptr()
        }) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_pixel_cmp_t; 8]>() as size_t,
    );
    memcpy(
        (*h).pixf.fpelcmp_x3.as_mut_ptr() as *mut ::core::ffi::c_void,
        (if satd != 0 {
            (*h).pixf.satd_x3.as_mut_ptr()
        } else {
            (*h).pixf.sad_x3.as_mut_ptr()
        }) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_pixel_cmp_x3_t; 7]>() as size_t,
    );
    memcpy(
        (*h).pixf.fpelcmp_x4.as_mut_ptr() as *mut ::core::ffi::c_void,
        (if satd != 0 {
            (*h).pixf.satd_x4.as_mut_ptr()
        } else {
            (*h).pixf.sad_x4.as_mut_ptr()
        }) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_pixel_cmp_x4_t; 7]>() as size_t,
    );
}
#[c2rust::src_loc = "1429:1"]
unsafe extern "C" fn chroma_dsp_init(mut h: *mut x264_t) {
    memcpy(
        (*h).luma2chroma_pixel.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*x264_luma2chroma_pixel
            .as_ptr()
            .offset((*(*h).sps.as_mut_ptr()).i_chroma_format_idc as isize))
            .as_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
    );
    match (*(*h).sps.as_mut_ptr()).i_chroma_format_idc {
        0 => {
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_400;
        }
        1 => {
            memcpy(
                (*h).predict_chroma.as_mut_ptr() as *mut ::core::ffi::c_void,
                (*h).predict_8x8c.as_mut_ptr() as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[x264_predict_t; 7]>() as size_t,
            );
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_420;
            (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize] = (*h)
                .loopf
                .deblock_h_chroma_420;
            (*h).loopf.deblock_chroma_intra[0 as ::core::ffi::c_int as usize] = (*h)
                .loopf
                .deblock_h_chroma_420_intra;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_420_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h)
                .loopf
                .deblock_chroma_420_intra_mbaff;
            (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x8c;
            (*h).quantf.coeff_last[DCT_CHROMA_DC as ::core::ffi::c_int as usize] = (*h)
                .quantf
                .coeff_last4;
            (*h).quantf.coeff_level_run[DCT_CHROMA_DC as ::core::ffi::c_int as usize] = (*h)
                .quantf
                .coeff_level_run4;
        }
        2 => {
            memcpy(
                (*h).predict_chroma.as_mut_ptr() as *mut ::core::ffi::c_void,
                (*h).predict_8x16c.as_mut_ptr() as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[x264_predict_t; 7]>() as size_t,
            );
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
            (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize] = (*h)
                .loopf
                .deblock_h_chroma_422;
            (*h).loopf.deblock_chroma_intra[0 as ::core::ffi::c_int as usize] = (*h)
                .loopf
                .deblock_h_chroma_422_intra;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_chroma_422_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h)
                .loopf
                .deblock_chroma_422_intra_mbaff;
            (*h).pixf.intra_mbcmp_x3_chroma = (*h).pixf.intra_mbcmp_x3_8x16c;
            (*h).quantf.coeff_last[DCT_CHROMA_DC as ::core::ffi::c_int as usize] = (*h)
                .quantf
                .coeff_last8;
            (*h).quantf.coeff_level_run[DCT_CHROMA_DC as ::core::ffi::c_int as usize] = (*h)
                .quantf
                .coeff_level_run8;
        }
        3 => {
            (*h).mc.prefetch_fenc = (*h).mc.prefetch_fenc_422;
            (*h).loopf.deblock_chroma_mbaff = (*h).loopf.deblock_luma_mbaff;
            (*h).loopf.deblock_chroma_intra_mbaff = (*h).loopf.deblock_luma_intra_mbaff;
        }
        _ => {}
    };
}
#[c2rust::src_loc = "1468:1"]
unsafe extern "C" fn set_aspect_ratio(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
    mut initial: ::core::ffi::c_int,
) {
    if (*param).vui.i_sar_width > 0 as ::core::ffi::c_int
        && (*param).vui.i_sar_height > 0 as ::core::ffi::c_int
    {
        let mut i_w: uint32_t = (*param).vui.i_sar_width as uint32_t;
        let mut i_h: uint32_t = (*param).vui.i_sar_height as uint32_t;
        let mut old_w: uint32_t = (*h).param.vui.i_sar_width as uint32_t;
        let mut old_h: uint32_t = (*h).param.vui.i_sar_height as uint32_t;
        x264_reduce_fraction(&mut i_w, &mut i_h);
        while i_w > 65535 as uint32_t || i_h > 65535 as uint32_t {
            i_w = i_w.wrapping_div(2 as uint32_t);
            i_h = i_h.wrapping_div(2 as uint32_t);
        }
        x264_reduce_fraction(&mut i_w, &mut i_h);
        if i_w != old_w || i_h != old_h || initial != 0 {
            (*h).param.vui.i_sar_width = 0 as ::core::ffi::c_int;
            (*h).param.vui.i_sar_height = 0 as ::core::ffi::c_int;
            if i_w == 0 as uint32_t || i_h == 0 as uint32_t {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"cannot create valid sample aspect ratio\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                x264_10_log(
                    h,
                    if initial != 0 { X264_LOG_INFO } else { X264_LOG_DEBUG },
                    b"using SAR=%d/%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    i_w,
                    i_h,
                );
                (*h).param.vui.i_sar_width = i_w as ::core::ffi::c_int;
                (*h).param.vui.i_sar_height = i_h as ::core::ffi::c_int;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1507:1"]
pub unsafe extern "C" fn x264_10_encoder_open(
    mut param: *mut x264_param_t,
    mut api: *mut ::core::ffi::c_void,
) -> *mut x264_t {
    let mut temp: ::core::ffi::c_int = 0;
    let mut profile: *const ::core::ffi::c_char = 0 as *const ::core::ffi::c_char;
    let mut level: [::core::ffi::c_char; 16] = [0; 16];
    static mut subsampling: [*const ::core::ffi::c_char; 4] = [
        b"4:0:0\0" as *const u8 as *const ::core::ffi::c_char,
        b"4:2:0\0" as *const u8 as *const ::core::ffi::c_char,
        b"4:2:2\0" as *const u8 as *const ::core::ffi::c_char,
        b"4:4:4\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut current_block: u64;
    let mut h: *mut x264_t = 0 as *mut x264_t;
    let mut buf: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut p: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut i_slicetype_length: ::core::ffi::c_int = 0;
    h = x264_malloc(::core::mem::size_of::<x264_t>() as int64_t) as *mut x264_t;
    if !h.is_null() {
        memset(
            h as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<x264_t>() as size_t,
        );
        memcpy(
            &mut (*h).param as *mut x264_param_t as *mut ::core::ffi::c_void,
            param as *const ::core::ffi::c_void,
            ::core::mem::size_of::<x264_param_t>() as size_t,
        );
        (*h).param.opaque = NULL;
        (*h).param.param_free = None;
        if !(*h).param.psz_cqm_file.is_null() {
            (*h).param.psz_cqm_file = x264_param_strdup(
                &mut (*h).param,
                (*h).param.psz_cqm_file,
            );
            if (*h).param.psz_cqm_file.is_null() {
                current_block = 17708740041508716982;
            } else {
                current_block = 5399440093318478209;
            }
        } else {
            current_block = 5399440093318478209;
        }
        match current_block {
            17708740041508716982 => {}
            _ => {
                if !(*h).param.psz_dump_yuv.is_null() {
                    (*h).param.psz_dump_yuv = x264_param_strdup(
                        &mut (*h).param,
                        (*h).param.psz_dump_yuv,
                    );
                    if (*h).param.psz_dump_yuv.is_null() {
                        current_block = 17708740041508716982;
                    } else {
                        current_block = 8831408221741692167;
                    }
                } else {
                    current_block = 8831408221741692167;
                }
                match current_block {
                    17708740041508716982 => {}
                    _ => {
                        if !(*h).param.rc.psz_stat_out.is_null() {
                            (*h).param.rc.psz_stat_out = x264_param_strdup(
                                &mut (*h).param,
                                (*h).param.rc.psz_stat_out,
                            );
                            if (*h).param.rc.psz_stat_out.is_null() {
                                current_block = 17708740041508716982;
                            } else {
                                current_block = 4808432441040389987;
                            }
                        } else {
                            current_block = 4808432441040389987;
                        }
                        match current_block {
                            17708740041508716982 => {}
                            _ => {
                                if !(*h).param.rc.psz_stat_in.is_null() {
                                    (*h).param.rc.psz_stat_in = x264_param_strdup(
                                        &mut (*h).param,
                                        (*h).param.rc.psz_stat_in,
                                    );
                                    if (*h).param.rc.psz_stat_in.is_null() {
                                        current_block = 17708740041508716982;
                                    } else {
                                        current_block = 10043043949733653460;
                                    }
                                } else {
                                    current_block = 10043043949733653460;
                                }
                                match current_block {
                                    17708740041508716982 => {}
                                    _ => {
                                        if !(*h).param.rc.psz_zones.is_null() {
                                            (*h).param.rc.psz_zones = x264_param_strdup(
                                                &mut (*h).param,
                                                (*h).param.rc.psz_zones,
                                            );
                                            if (*h).param.rc.psz_zones.is_null() {
                                                current_block = 17708740041508716982;
                                            } else {
                                                current_block = 14648156034262866959;
                                            }
                                        } else {
                                            current_block = 14648156034262866959;
                                        }
                                        match current_block {
                                            17708740041508716982 => {}
                                            _ => {
                                                if !(*h).param.psz_clbin_file.is_null() {
                                                    (*h).param.psz_clbin_file = x264_param_strdup(
                                                        &mut (*h).param,
                                                        (*h).param.psz_clbin_file,
                                                    );
                                                    if (*h).param.psz_clbin_file.is_null() {
                                                        current_block = 17708740041508716982;
                                                    } else {
                                                        current_block = 652864300344834934;
                                                    }
                                                } else {
                                                    current_block = 652864300344834934;
                                                }
                                                match current_block {
                                                    17708740041508716982 => {}
                                                    _ => {
                                                        if (*param).param_free.is_some() {
                                                            x264_param_cleanup(param);
                                                            (*param)
                                                                .param_free
                                                                .expect(
                                                                    "non-null function pointer",
                                                                )(param as *mut ::core::ffi::c_void);
                                                        }
                                                        (*h).api = api;
                                                        if !(validate_parameters(h, 1 as ::core::ffi::c_int)
                                                            < 0 as ::core::ffi::c_int)
                                                        {
                                                            if !(*h).param.psz_cqm_file.is_null() {
                                                                if x264_10_cqm_parse_file(h, (*h).param.psz_cqm_file)
                                                                    < 0 as ::core::ffi::c_int
                                                                {
                                                                    current_block = 17708740041508716982;
                                                                } else {
                                                                    current_block = 3934796541983872331;
                                                                }
                                                            } else {
                                                                current_block = 3934796541983872331;
                                                            }
                                                            match current_block {
                                                                17708740041508716982 => {}
                                                                _ => {
                                                                    x264_reduce_fraction(
                                                                        &mut (*h).param.i_fps_num,
                                                                        &mut (*h).param.i_fps_den,
                                                                    );
                                                                    x264_reduce_fraction(
                                                                        &mut (*h).param.i_timebase_num,
                                                                        &mut (*h).param.i_timebase_den,
                                                                    );
                                                                    (*h).i_frame = -(1 as ::core::ffi::c_int);
                                                                    (*h).i_frame_num = 0 as ::core::ffi::c_int;
                                                                    if (*h).param.i_avcintra_class != 0 {
                                                                        (*h).i_idr_pic_id = if (*h).param.i_avcintra_class
                                                                            > 200 as ::core::ffi::c_int
                                                                        {
                                                                            4 as ::core::ffi::c_int
                                                                        } else {
                                                                            5 as ::core::ffi::c_int
                                                                        };
                                                                    } else {
                                                                        (*h).i_idr_pic_id = 0 as ::core::ffi::c_int;
                                                                    }
                                                                    if ((*h).param.i_timebase_den as uint64_t)
                                                                        .wrapping_mul(2 as uint64_t) > UINT32_MAX as uint64_t
                                                                    {
                                                                        x264_10_log(
                                                                            h,
                                                                            X264_LOG_ERROR,
                                                                            b"Effective timebase denominator %u exceeds H.264 maximum\n\0"
                                                                                as *const u8 as *const ::core::ffi::c_char,
                                                                            (*h).param.i_timebase_den,
                                                                        );
                                                                    } else {
                                                                        set_aspect_ratio(
                                                                            h,
                                                                            &mut (*h).param,
                                                                            1 as ::core::ffi::c_int,
                                                                        );
                                                                        x264_10_sps_init(
                                                                            (*h).sps.as_mut_ptr(),
                                                                            (*h).param.i_sps_id,
                                                                            &mut (*h).param,
                                                                        );
                                                                        x264_10_sps_init_scaling_list(
                                                                            (*h).sps.as_mut_ptr(),
                                                                            &mut (*h).param,
                                                                        );
                                                                        x264_10_pps_init(
                                                                            (*h).pps.as_mut_ptr(),
                                                                            (*h).param.i_sps_id,
                                                                            &mut (*h).param,
                                                                            (*h).sps.as_mut_ptr(),
                                                                        );
                                                                        x264_10_validate_levels(h, 1 as ::core::ffi::c_int);
                                                                        (*h).chroma_qp_table = i_chroma_qp_table
                                                                            .as_ptr()
                                                                            .offset(12 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (*(*h).pps.as_mut_ptr()).i_chroma_qp_index_offset as isize,
                                                                            );
                                                                        if !(x264_10_cqm_init(h) < 0 as ::core::ffi::c_int) {
                                                                            (*h).mb.i_mb_width = (*(*h).sps.as_mut_ptr()).i_mb_width;
                                                                            (*h).mb.i_mb_height = (*(*h).sps.as_mut_ptr()).i_mb_height;
                                                                            (*h).mb.i_mb_count = (*h).mb.i_mb_width
                                                                                * (*h).mb.i_mb_height;
                                                                            (*h).mb.chroma_h_shift = ((*(*h).sps.as_mut_ptr())
                                                                                .i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int
                                                                                || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                                                                    == CHROMA_422 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                                                            (*h).mb.chroma_v_shift = ((*(*h).sps.as_mut_ptr())
                                                                                .i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int)
                                                                                as ::core::ffi::c_int;
                                                                            (*h).mb.b_adaptive_mbaff = ((*h).param.b_interlaced != 0
                                                                                && (*h).param.analyse.i_subpel_refine != 0)
                                                                                as ::core::ffi::c_int;
                                                                            if (*h).param.i_bframe_adaptive == X264_B_ADAPT_TRELLIS
                                                                                && (*h).param.rc.b_stat_read == 0
                                                                            {
                                                                                (*h).frames.i_delay = (if (*h).param.i_bframe
                                                                                    > 3 as ::core::ffi::c_int
                                                                                {
                                                                                    (*h).param.i_bframe
                                                                                } else {
                                                                                    3 as ::core::ffi::c_int
                                                                                }) * 4 as ::core::ffi::c_int;
                                                                            } else {
                                                                                (*h).frames.i_delay = (*h).param.i_bframe;
                                                                            }
                                                                            if (*h).param.rc.b_mb_tree != 0
                                                                                || (*h).param.rc.i_vbv_buffer_size != 0
                                                                            {
                                                                                (*h).frames.i_delay = if (*h).frames.i_delay
                                                                                    > (*h).param.rc.i_lookahead
                                                                                {
                                                                                    (*h).frames.i_delay
                                                                                } else {
                                                                                    (*h).param.rc.i_lookahead
                                                                                };
                                                                            }
                                                                            i_slicetype_length = (*h).frames.i_delay;
                                                                            (*h).frames.i_delay
                                                                                += (*h).i_thread_frames - 1 as ::core::ffi::c_int;
                                                                            (*h).frames.i_delay += (*h).param.i_sync_lookahead;
                                                                            (*h).frames.i_delay += (*h).param.b_vfr_input;
                                                                            (*h).frames.i_bframe_delay = if (*h).param.i_bframe != 0 {
                                                                                if (*h).param.i_bframe_pyramid != 0 {
                                                                                    2 as ::core::ffi::c_int
                                                                                } else {
                                                                                    1 as ::core::ffi::c_int
                                                                                }
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            };
                                                                            (*h).frames.i_max_ref0 = (*h).param.i_frame_reference;
                                                                            (*h).frames.i_max_ref1 = if (*(*h).sps.as_mut_ptr())
                                                                                .vui
                                                                                .i_num_reorder_frames < (*h).param.i_frame_reference
                                                                            {
                                                                                (*(*h).sps.as_mut_ptr()).vui.i_num_reorder_frames
                                                                            } else {
                                                                                (*h).param.i_frame_reference
                                                                            };
                                                                            (*h).frames.i_max_dpb = (*(*h).sps.as_mut_ptr())
                                                                                .vui
                                                                                .i_max_dec_frame_buffering;
                                                                            (*h).frames.b_have_lowres = ((*h).param.rc.b_stat_read == 0
                                                                                && ((*h).param.rc.i_rc_method == X264_RC_ABR
                                                                                    || (*h).param.rc.i_rc_method == X264_RC_CRF
                                                                                    || (*h).param.i_bframe_adaptive != 0
                                                                                    || (*h).param.i_scenecut_threshold != 0
                                                                                    || (*h).param.rc.b_mb_tree != 0
                                                                                    || (*h).param.analyse.i_weighted_pred != 0))
                                                                                as ::core::ffi::c_int;
                                                                            (*h).frames.b_have_lowres
                                                                                |= ((*h).param.rc.b_stat_read != 0
                                                                                    && (*h).param.rc.i_vbv_buffer_size
                                                                                        > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                                                                            (*h).frames.b_have_sub8x8_esa = ((*h).param.analyse.inter
                                                                                & X264_ANALYSE_PSUB8x8 != 0) as ::core::ffi::c_int;
                                                                            (*h).frames.i_last_keyframe = -(*h).param.i_keyint_max;
                                                                            (*h).frames.i_last_idr = (*h).frames.i_last_keyframe;
                                                                            (*h).frames.i_input = 0 as ::core::ffi::c_int;
                                                                            (*h).frames.i_second_largest_pts = -(1
                                                                                as ::core::ffi::c_int) as int64_t;
                                                                            (*h).frames.i_largest_pts = (*h)
                                                                                .frames
                                                                                .i_second_largest_pts;
                                                                            (*h).frames.i_poc_last_open_gop = -(1
                                                                                as ::core::ffi::c_int);
                                                                            (*h).cost_table = x264_malloc(
                                                                                ::core::mem::size_of::<C2RustUnnamed_17>() as int64_t,
                                                                            ) as *mut C2RustUnnamed_17;
                                                                            if !(*h).cost_table.is_null() {
                                                                                memset(
                                                                                    (*h).cost_table as *mut ::core::ffi::c_void,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    ::core::mem::size_of::<C2RustUnnamed_17>() as size_t,
                                                                                );
                                                                                (*h).frames.unused[0 as ::core::ffi::c_int as usize] = x264_malloc(
                                                                                    (((*h).frames.i_delay + 3 as ::core::ffi::c_int) as usize)
                                                                                        .wrapping_mul(
                                                                                            ::core::mem::size_of::<*mut x264_frame_t>() as usize,
                                                                                        ) as int64_t,
                                                                                ) as *mut *mut x264_frame_t;
                                                                                if !(*h)
                                                                                    .frames
                                                                                    .unused[0 as ::core::ffi::c_int as usize]
                                                                                    .is_null()
                                                                                {
                                                                                    memset(
                                                                                        (*h).frames.unused[0 as ::core::ffi::c_int as usize]
                                                                                            as *mut ::core::ffi::c_void,
                                                                                        0 as ::core::ffi::c_int,
                                                                                        (((*h).frames.i_delay + 3 as ::core::ffi::c_int) as size_t)
                                                                                            .wrapping_mul(
                                                                                                ::core::mem::size_of::<*mut x264_frame_t>() as size_t,
                                                                                            ),
                                                                                    );
                                                                                    (*h).frames.unused[1 as ::core::ffi::c_int as usize] = x264_malloc(
                                                                                        (((*h).i_thread_frames + 16 as ::core::ffi::c_int
                                                                                            + 4 as ::core::ffi::c_int) as usize)
                                                                                            .wrapping_mul(
                                                                                                ::core::mem::size_of::<*mut x264_frame_t>() as usize,
                                                                                            ) as int64_t,
                                                                                    ) as *mut *mut x264_frame_t;
                                                                                    if !(*h)
                                                                                        .frames
                                                                                        .unused[1 as ::core::ffi::c_int as usize]
                                                                                        .is_null()
                                                                                    {
                                                                                        memset(
                                                                                            (*h).frames.unused[1 as ::core::ffi::c_int as usize]
                                                                                                as *mut ::core::ffi::c_void,
                                                                                            0 as ::core::ffi::c_int,
                                                                                            (((*h).i_thread_frames + 16 as ::core::ffi::c_int
                                                                                                + 4 as ::core::ffi::c_int) as size_t)
                                                                                                .wrapping_mul(
                                                                                                    ::core::mem::size_of::<*mut x264_frame_t>() as size_t,
                                                                                                ),
                                                                                        );
                                                                                        (*h).frames.current = x264_malloc(
                                                                                            (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                + (*h).i_thread_frames + 3 as ::core::ffi::c_int) as usize)
                                                                                                .wrapping_mul(
                                                                                                    ::core::mem::size_of::<*mut x264_frame_t>() as usize,
                                                                                                ) as int64_t,
                                                                                        ) as *mut *mut x264_frame_t;
                                                                                        if !(*h).frames.current.is_null() {
                                                                                            memset(
                                                                                                (*h).frames.current as *mut ::core::ffi::c_void,
                                                                                                0 as ::core::ffi::c_int,
                                                                                                (((*h).param.i_sync_lookahead + (*h).param.i_bframe
                                                                                                    + (*h).i_thread_frames + 3 as ::core::ffi::c_int) as size_t)
                                                                                                    .wrapping_mul(
                                                                                                        ::core::mem::size_of::<*mut x264_frame_t>() as size_t,
                                                                                                    ),
                                                                                            );
                                                                                            if (*h).param.analyse.i_weighted_pred
                                                                                                > 0 as ::core::ffi::c_int
                                                                                            {
                                                                                                (*h).frames.blank_unused = x264_malloc(
                                                                                                    (((*h).i_thread_frames * 4 as ::core::ffi::c_int) as usize)
                                                                                                        .wrapping_mul(
                                                                                                            ::core::mem::size_of::<*mut x264_frame_t>() as usize,
                                                                                                        ) as int64_t,
                                                                                                ) as *mut *mut x264_frame_t;
                                                                                                if (*h).frames.blank_unused.is_null() {
                                                                                                    current_block = 17708740041508716982;
                                                                                                } else {
                                                                                                    memset(
                                                                                                        (*h).frames.blank_unused as *mut ::core::ffi::c_void,
                                                                                                        0 as ::core::ffi::c_int,
                                                                                                        (((*h).i_thread_frames * 4 as ::core::ffi::c_int) as size_t)
                                                                                                            .wrapping_mul(
                                                                                                                ::core::mem::size_of::<*mut x264_frame_t>() as size_t,
                                                                                                            ),
                                                                                                    );
                                                                                                    current_block = 6406431739208918833;
                                                                                                }
                                                                                            } else {
                                                                                                current_block = 6406431739208918833;
                                                                                            }
                                                                                            match current_block {
                                                                                                17708740041508716982 => {}
                                                                                                _ => {
                                                                                                    (*h).i_ref[1 as ::core::ffi::c_int as usize] = 0
                                                                                                        as ::core::ffi::c_int;
                                                                                                    (*h).i_ref[0 as ::core::ffi::c_int as usize] = (*h)
                                                                                                        .i_ref[1 as ::core::ffi::c_int as usize];
                                                                                                    (*h).i_disp_fields = 0 as int64_t;
                                                                                                    (*h).i_coded_fields = (*h).i_disp_fields;
                                                                                                    (*h).i_cpb_delay = (*h).i_coded_fields;
                                                                                                    (*h).i_prev_duration = ((*h).param.i_fps_den as uint64_t)
                                                                                                        .wrapping_mul(
                                                                                                            (*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t,
                                                                                                        )
                                                                                                        .wrapping_div(
                                                                                                            ((*h).param.i_fps_num as uint64_t)
                                                                                                                .wrapping_mul(
                                                                                                                    (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as uint64_t,
                                                                                                                ),
                                                                                                        ) as int64_t;
                                                                                                    (*h).i_disp_fields_last_frame = -(1 as ::core::ffi::c_int);
                                                                                                    x264_10_rdo_init();
                                                                                                    (*h).param.cpu = ((*h).param.cpu as ::core::ffi::c_uint
                                                                                                        & !X264_CPU_AVX512) as uint32_t;
                                                                                                    x264_10_predict_16x16_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_16x16.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_predict_8x8c_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_8x8c.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_predict_8x16c_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_8x16c.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_predict_8x8_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_8x8.as_mut_ptr(),
                                                                                                        &mut (*h).predict_8x8_filter,
                                                                                                    );
                                                                                                    x264_10_predict_4x4_init(
                                                                                                        (*h).param.cpu,
                                                                                                        (*h).predict_4x4.as_mut_ptr(),
                                                                                                    );
                                                                                                    x264_10_pixel_init((*h).param.cpu, &mut (*h).pixf);
                                                                                                    x264_10_dct_init((*h).param.cpu, &mut (*h).dctf);
                                                                                                    x264_10_zigzag_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).zigzagf_progressive,
                                                                                                        &mut (*h).zigzagf_interlaced,
                                                                                                    );
                                                                                                    memcpy(
                                                                                                        &mut (*h).zigzagf as *mut x264_zigzag_function_t
                                                                                                            as *mut ::core::ffi::c_void,
                                                                                                        (if (*h).param.b_interlaced != 0 {
                                                                                                            &mut (*h).zigzagf_interlaced as *mut x264_zigzag_function_t
                                                                                                        } else {
                                                                                                            &mut (*h).zigzagf_progressive as *mut x264_zigzag_function_t
                                                                                                        }) as *const ::core::ffi::c_void,
                                                                                                        ::core::mem::size_of::<x264_zigzag_function_t>() as size_t,
                                                                                                    );
                                                                                                    x264_10_mc_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).mc,
                                                                                                        (*h).param.b_cpu_independent,
                                                                                                    );
                                                                                                    x264_10_quant_init(h, (*h).param.cpu, &mut (*h).quantf);
                                                                                                    x264_10_deblock_init(
                                                                                                        (*h).param.cpu,
                                                                                                        &mut (*h).loopf,
                                                                                                        (*h).param.b_interlaced,
                                                                                                    );
                                                                                                    x264_10_bitstream_init((*h).param.cpu, &mut (*h).bsf);
                                                                                                    if (*h).param.b_cabac != 0 {
                                                                                                        x264_10_cabac_init(h);
                                                                                                    } else {
                                                                                                        x264_10_cavlc_init(h);
                                                                                                    }
                                                                                                    mbcmp_init(h);
                                                                                                    chroma_dsp_init(h);
                                                                                                    p = buf
                                                                                                        .as_mut_ptr()
                                                                                                        .offset(
                                                                                                            sprintf(
                                                                                                                buf.as_mut_ptr(),
                                                                                                                b"using cpu capabilities:\0" as *const u8
                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                            ) as isize,
                                                                                                        );
                                                                                                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                    while (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                        != 0
                                                                                                    {
                                                                                                        if !(strcmp(
                                                                                                            (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                            b"SSE\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                        ) == 0
                                                                                                            && (*h).param.cpu
                                                                                                                & (1 as uint32_t) << 3 as ::core::ffi::c_int != 0)
                                                                                                        {
                                                                                                            if !(strcmp(
                                                                                                                (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                b"SSE2\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                            ) == 0
                                                                                                                && (*h).param.cpu
                                                                                                                    & (X264_CPU_SSE2_IS_FAST as uint32_t
                                                                                                                        | X264_CPU_SSE2_IS_SLOW as uint32_t) != 0)
                                                                                                            {
                                                                                                                if !(strcmp(
                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                    b"SSE3\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                ) == 0
                                                                                                                    && ((*h).param.cpu & X264_CPU_SSSE3 as uint32_t != 0
                                                                                                                        || (*h).param.cpu & X264_CPU_CACHELINE_64 as uint32_t == 0))
                                                                                                                {
                                                                                                                    if !(strcmp(
                                                                                                                        (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                        b"SSE4.1\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                    ) == 0 && (*h).param.cpu & X264_CPU_SSE42 as uint32_t != 0)
                                                                                                                    {
                                                                                                                        if !(strcmp(
                                                                                                                            (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                            b"LZCNT\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                        ) == 0 && (*h).param.cpu & X264_CPU_BMI1 as uint32_t != 0)
                                                                                                                        {
                                                                                                                            if !(strcmp(
                                                                                                                                (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                b"BMI1\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                            ) == 0 && (*h).param.cpu & X264_CPU_BMI2 as uint32_t != 0)
                                                                                                                            {
                                                                                                                                if !(strcmp(
                                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                    b"FMA4\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                ) == 0 && (*h).param.cpu & X264_CPU_FMA3 as uint32_t != 0)
                                                                                                                                {
                                                                                                                                    if (*h).param.cpu
                                                                                                                                        & (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                        == (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                        && (i == 0
                                                                                                                                            || (*x264_cpu_names.as_ptr().offset(i as isize)).flags
                                                                                                                                                != (*x264_cpu_names
                                                                                                                                                    .as_ptr()
                                                                                                                                                    .offset((i - 1 as ::core::ffi::c_int) as isize))
                                                                                                                                                    .flags)
                                                                                                                                    {
                                                                                                                                        p = p
                                                                                                                                            .offset(
                                                                                                                                                sprintf(
                                                                                                                                                    p,
                                                                                                                                                    b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                    (*x264_cpu_names.as_ptr().offset(i as isize)).name,
                                                                                                                                                ) as isize,
                                                                                                                                            );
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        i += 1;
                                                                                                    }
                                                                                                    if (*h).param.cpu == 0 {
                                                                                                        p = p
                                                                                                            .offset(
                                                                                                                sprintf(
                                                                                                                    p,
                                                                                                                    b" none!\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                ) as isize,
                                                                                                            );
                                                                                                    }
                                                                                                    x264_10_log(
                                                                                                        h,
                                                                                                        X264_LOG_INFO,
                                                                                                        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                        buf.as_mut_ptr(),
                                                                                                    );
                                                                                                    if !(x264_10_analyse_init_costs(h) != 0) {
                                                                                                        temp = 392 as ::core::ffi::c_int;
                                                                                                        if (temp as ::core::ffi::c_uint).leading_zeros() as i32
                                                                                                            != 23 as ::core::ffi::c_int
                                                                                                        {
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"CLZ test failed: x264 has been miscompiled!\n\0"
                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                            );
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"Are you attempting to run an SSE4a/LZCNT-targeted build on a CPU that\n\0"
                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                            );
                                                                                                            x264_10_log(
                                                                                                                h,
                                                                                                                X264_LOG_ERROR,
                                                                                                                b"doesn't support it?\n\0" as *const u8
                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                            );
                                                                                                        } else {
                                                                                                            (*h).out.i_nal = 0 as ::core::ffi::c_int;
                                                                                                            (*h).out.i_bitstream = x264_clip3f(
                                                                                                                ((*h).param.i_width * (*h).param.i_height
                                                                                                                    * 4 as ::core::ffi::c_int) as ::core::ffi::c_double
                                                                                                                    * (if (*h).param.rc.i_rc_method == X264_RC_ABR {
                                                                                                                        pow(
                                                                                                                            0.95f64,
                                                                                                                            (*h).param.rc.i_qp_min as ::core::ffi::c_double,
                                                                                                                        )
                                                                                                                    } else {
                                                                                                                        pow(
                                                                                                                            0.95f64,
                                                                                                                            (*h).param.rc.i_qp_constant as ::core::ffi::c_double,
                                                                                                                        )
                                                                                                                            * (if 1 as ::core::ffi::c_int as ::core::ffi::c_float
                                                                                                                                > (*h).param.rc.f_ip_factor
                                                                                                                            {
                                                                                                                                1 as ::core::ffi::c_int as ::core::ffi::c_float
                                                                                                                            } else {
                                                                                                                                (*h).param.rc.f_ip_factor
                                                                                                                            }) as ::core::ffi::c_double
                                                                                                                    }),
                                                                                                                1000000 as ::core::ffi::c_int as ::core::ffi::c_double,
                                                                                                                (INT_MAX / 3 as ::core::ffi::c_int) as ::core::ffi::c_double,
                                                                                                            ) as ::core::ffi::c_int;
                                                                                                            (*h).nal_buffer_size = (*h).out.i_bitstream
                                                                                                                * 3 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                                                                                                + 4 as ::core::ffi::c_int + 64 as ::core::ffi::c_int;
                                                                                                            (*h).nal_buffer = x264_malloc(
                                                                                                                (*h).nal_buffer_size as int64_t,
                                                                                                            ) as *mut uint8_t;
                                                                                                            if !(*h).nal_buffer.is_null() {
                                                                                                                (*h).reconfig_h = x264_malloc(
                                                                                                                    ::core::mem::size_of::<x264_t>() as int64_t,
                                                                                                                ) as *mut x264_t;
                                                                                                                if !(*h).reconfig_h.is_null() {
                                                                                                                    if !((*h).param.i_threads > 1 as ::core::ffi::c_int
                                                                                                                        && x264_10_threadpool_init(
                                                                                                                            &mut (*h).threadpool,
                                                                                                                            (*h).param.i_threads,
                                                                                                                        ) != 0)
                                                                                                                    {
                                                                                                                        if !((*h).param.i_lookahead_threads
                                                                                                                            > 1 as ::core::ffi::c_int
                                                                                                                            && x264_10_threadpool_init(
                                                                                                                                &mut (*h).lookaheadpool,
                                                                                                                                (*h).param.i_lookahead_threads,
                                                                                                                            ) != 0)
                                                                                                                        {
                                                                                                                            (*h).thread[0 as ::core::ffi::c_int as usize] = h;
                                                                                                                            let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                                                                                                                            loop {
                                                                                                                                if !(i_0
                                                                                                                                    < (*h).param.i_threads
                                                                                                                                        + ((*h).param.i_sync_lookahead != 0) as ::core::ffi::c_int)
                                                                                                                                {
                                                                                                                                    current_block = 4338462691184853296;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                (*h).thread[i_0 as usize] = x264_malloc(
                                                                                                                                    ::core::mem::size_of::<x264_t>() as int64_t,
                                                                                                                                ) as *mut x264_t;
                                                                                                                                if (*h).thread[i_0 as usize].is_null() {
                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                i_0 += 1;
                                                                                                                            }
                                                                                                                            match current_block {
                                                                                                                                17708740041508716982 => {}
                                                                                                                                _ => {
                                                                                                                                    if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int
                                                                                                                                    {
                                                                                                                                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                        loop {
                                                                                                                                            if !(i_1 < (*h).param.i_lookahead_threads) {
                                                                                                                                                current_block = 11735322225073324345;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            (*h).lookahead_thread[i_1 as usize] = x264_malloc(
                                                                                                                                                ::core::mem::size_of::<x264_t>() as int64_t,
                                                                                                                                            ) as *mut x264_t;
                                                                                                                                            if (*h).lookahead_thread[i_1 as usize].is_null() {
                                                                                                                                                current_block = 17708740041508716982;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                            *(*h).lookahead_thread[i_1 as usize] = *h;
                                                                                                                                            i_1 += 1;
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        current_block = 11735322225073324345;
                                                                                                                                    }
                                                                                                                                    match current_block {
                                                                                                                                        17708740041508716982 => {}
                                                                                                                                        _ => {
                                                                                                                                            *(*h).reconfig_h = *h;
                                                                                                                                            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                            loop {
                                                                                                                                                if !(i_2 < (*h).param.i_threads) {
                                                                                                                                                    current_block = 9702083122263515018;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                let mut init_nal_count: ::core::ffi::c_int = (*h)
                                                                                                                                                    .param
                                                                                                                                                    .i_slice_count + 3 as ::core::ffi::c_int;
                                                                                                                                                let mut allocate_threadlocal_data: ::core::ffi::c_int = ((*h)
                                                                                                                                                    .param
                                                                                                                                                    .b_sliced_threads == 0 || i_2 == 0) as ::core::ffi::c_int;
                                                                                                                                                if i_2 > 0 as ::core::ffi::c_int {
                                                                                                                                                    *(*h).thread[i_2 as usize] = *h;
                                                                                                                                                }
                                                                                                                                                if pthread_mutex_init(
                                                                                                                                                    &mut (**(*h).thread.as_mut_ptr().offset(i_2 as isize))
                                                                                                                                                        .mutex,
                                                                                                                                                    0 as *const pthread_mutexattr_t,
                                                                                                                                                ) != 0
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                if pthread_cond_init(
                                                                                                                                                    &mut (**(*h).thread.as_mut_ptr().offset(i_2 as isize)).cv,
                                                                                                                                                    0 as *const pthread_condattr_t,
                                                                                                                                                ) != 0
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                if allocate_threadlocal_data != 0 {
                                                                                                                                                    (*(*h).thread[i_2 as usize]).fdec = x264_10_frame_pop_unused(
                                                                                                                                                        h,
                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                    );
                                                                                                                                                    if (*(*h).thread[i_2 as usize]).fdec.is_null() {
                                                                                                                                                        current_block = 17708740041508716982;
                                                                                                                                                        break;
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    (*(*h).thread[i_2 as usize]).fdec = (*(*h)
                                                                                                                                                        .thread[0 as ::core::ffi::c_int as usize])
                                                                                                                                                        .fdec;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.p_bitstream = x264_malloc(
                                                                                                                                                    (*h).out.i_bitstream as int64_t,
                                                                                                                                                ) as *mut uint8_t;
                                                                                                                                                if (*(*h).thread[i_2 as usize]).out.p_bitstream.is_null() {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.nal = x264_malloc(
                                                                                                                                                    (init_nal_count as usize)
                                                                                                                                                        .wrapping_mul(::core::mem::size_of::<x264_nal_t>() as usize)
                                                                                                                                                        as int64_t,
                                                                                                                                                ) as *mut x264_nal_t;
                                                                                                                                                if (*(*h).thread[i_2 as usize]).out.nal.is_null() {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                (*(*h).thread[i_2 as usize]).out.i_nals_allocated = init_nal_count;
                                                                                                                                                if allocate_threadlocal_data != 0
                                                                                                                                                    && x264_10_macroblock_cache_allocate(
                                                                                                                                                        (*h).thread[i_2 as usize],
                                                                                                                                                    ) < 0 as ::core::ffi::c_int
                                                                                                                                                {
                                                                                                                                                    current_block = 17708740041508716982;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                i_2 += 1;
                                                                                                                                            }
                                                                                                                                            match current_block {
                                                                                                                                                17708740041508716982 => {}
                                                                                                                                                _ => {
                                                                                                                                                    if !(x264_10_lookahead_init(h, i_slicetype_length) != 0) {
                                                                                                                                                        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                                                                                                                        loop {
                                                                                                                                                            if !(i_3 < (*h).param.i_threads) {
                                                                                                                                                                current_block = 9812798724717783973;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            if x264_10_macroblock_thread_allocate(
                                                                                                                                                                (*h).thread[i_3 as usize],
                                                                                                                                                                0 as ::core::ffi::c_int,
                                                                                                                                                            ) < 0 as ::core::ffi::c_int
                                                                                                                                                            {
                                                                                                                                                                current_block = 17708740041508716982;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            i_3 += 1;
                                                                                                                                                        }
                                                                                                                                                        match current_block {
                                                                                                                                                            17708740041508716982 => {}
                                                                                                                                                            _ => {
                                                                                                                                                                if !(x264_10_ratecontrol_new(h) < 0 as ::core::ffi::c_int) {
                                                                                                                                                                    if (*h).param.i_nal_hrd != 0 {
                                                                                                                                                                        x264_10_log(
                                                                                                                                                                            h,
                                                                                                                                                                            X264_LOG_DEBUG,
                                                                                                                                                                            b"HRD bitrate: %i bits/sec\n\0" as *const u8
                                                                                                                                                                                as *const ::core::ffi::c_char,
                                                                                                                                                                            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled,
                                                                                                                                                                        );
                                                                                                                                                                        x264_10_log(
                                                                                                                                                                            h,
                                                                                                                                                                            X264_LOG_DEBUG,
                                                                                                                                                                            b"CPB size: %i bits\n\0" as *const u8
                                                                                                                                                                                as *const ::core::ffi::c_char,
                                                                                                                                                                            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled,
                                                                                                                                                                        );
                                                                                                                                                                    }
                                                                                                                                                                    if !(*h).param.psz_dump_yuv.is_null() {
                                                                                                                                                                        let mut f: *mut FILE = fopen(
                                                                                                                                                                            (*h).param.psz_dump_yuv,
                                                                                                                                                                            b"w\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                        ) as *mut FILE;
                                                                                                                                                                        if f.is_null() {
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_ERROR,
                                                                                                                                                                                b"dump_yuv: can't write to %s\n\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*h).param.psz_dump_yuv,
                                                                                                                                                                            );
                                                                                                                                                                            current_block = 17708740041508716982;
                                                                                                                                                                        } else if x264_is_regular_file(f) == 0 {
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_ERROR,
                                                                                                                                                                                b"dump_yuv: incompatible with non-regular file %s\n\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                (*h).param.psz_dump_yuv,
                                                                                                                                                                            );
                                                                                                                                                                            fclose(f);
                                                                                                                                                                            current_block = 17708740041508716982;
                                                                                                                                                                        } else {
                                                                                                                                                                            fclose(f);
                                                                                                                                                                            current_block = 11844752514624976770;
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        current_block = 11844752514624976770;
                                                                                                                                                                    }
                                                                                                                                                                    match current_block {
                                                                                                                                                                        17708740041508716982 => {}
                                                                                                                                                                        _ => {
                                                                                                                                                                            profile = if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_BASELINE as ::core::ffi::c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"Constrained Baseline\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_MAIN as ::core::ffi::c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"Main\0" as *const u8 as *const ::core::ffi::c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH as ::core::ffi::c_int
                                                                                                                                                                            {
                                                                                                                                                                                b"High\0" as *const u8 as *const ::core::ffi::c_char
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH10 as ::core::ffi::c_int
                                                                                                                                                                            {
                                                                                                                                                                                if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                    b"High 10 Intra\0" as *const u8
                                                                                                                                                                                        as *const ::core::ffi::c_char
                                                                                                                                                                                } else {
                                                                                                                                                                                    b"High 10\0" as *const u8 as *const ::core::ffi::c_char
                                                                                                                                                                                }
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                == PROFILE_HIGH422 as ::core::ffi::c_int
                                                                                                                                                                            {
                                                                                                                                                                                if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                    b"High 4:2:2 Intra\0" as *const u8
                                                                                                                                                                                        as *const ::core::ffi::c_char
                                                                                                                                                                                } else {
                                                                                                                                                                                    b"High 4:2:2\0" as *const u8 as *const ::core::ffi::c_char
                                                                                                                                                                                }
                                                                                                                                                                            } else if (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0 {
                                                                                                                                                                                b"High 4:4:4 Intra\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char
                                                                                                                                                                            } else {
                                                                                                                                                                                b"High 4:4:4 Predictive\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char
                                                                                                                                                                            };
                                                                                                                                                                            level = [0; 16];
                                                                                                                                                                            if (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                == 9 as ::core::ffi::c_int
                                                                                                                                                                                || (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                    == 11 as ::core::ffi::c_int
                                                                                                                                                                                    && (*(*h).sps.as_mut_ptr()).b_constraint_set3 != 0
                                                                                                                                                                                    && ((*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                        == PROFILE_BASELINE as ::core::ffi::c_int
                                                                                                                                                                                        || (*(*h).sps.as_mut_ptr()).i_profile_idc
                                                                                                                                                                                            == PROFILE_MAIN as ::core::ffi::c_int)
                                                                                                                                                                            {
                                                                                                                                                                                strcpy(
                                                                                                                                                                                    level.as_mut_ptr(),
                                                                                                                                                                                    b"1b\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                );
                                                                                                                                                                            } else {
                                                                                                                                                                                snprintf(
                                                                                                                                                                                    level.as_mut_ptr(),
                                                                                                                                                                                    ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                                                                                                                                                                        as size_t,
                                                                                                                                                                                    b"%d.%d\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                        / 10 as ::core::ffi::c_int,
                                                                                                                                                                                    (*(*h).sps.as_mut_ptr()).i_level_idc
                                                                                                                                                                                        % 10 as ::core::ffi::c_int,
                                                                                                                                                                                );
                                                                                                                                                                            }
                                                                                                                                                                            x264_10_log(
                                                                                                                                                                                h,
                                                                                                                                                                                X264_LOG_INFO,
                                                                                                                                                                                b"profile %s, level %s, %s, %d-bit\n\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                profile,
                                                                                                                                                                                level.as_mut_ptr(),
                                                                                                                                                                                subsampling[(*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                                                                                                                                                                    as usize],
                                                                                                                                                                                BIT_DEPTH,
                                                                                                                                                                            );
                                                                                                                                                                            return h;
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    x264_free(h as *mut ::core::ffi::c_void);
    return 0 as *mut x264_t;
}
#[c2rust::src_loc = "1862:1"]
unsafe extern "C" fn encoder_try_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
    mut rc_reconfig: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *rc_reconfig = 0 as ::core::ffi::c_int;
    set_aspect_ratio(h, param, 0 as ::core::ffi::c_int);
    (*h).param.i_frame_reference = (*param).i_frame_reference;
    (*h).param.i_bframe_bias = (*param).i_bframe_bias;
    if (*h).param.i_scenecut_threshold != 0 {
        (*h).param.i_scenecut_threshold = (*param).i_scenecut_threshold;
    }
    (*h).param.b_deblocking_filter = (*param).b_deblocking_filter;
    (*h).param.i_deblocking_filter_alphac0 = (*param).i_deblocking_filter_alphac0;
    (*h).param.i_deblocking_filter_beta = (*param).i_deblocking_filter_beta;
    (*h).param.i_frame_packing = (*param).i_frame_packing;
    (*h).param.mastering_display = (*param).mastering_display;
    (*h).param.content_light_level = (*param).content_light_level;
    (*h).param.i_alternative_transfer = (*param).i_alternative_transfer;
    (*h).param.analyse.inter = (*param).analyse.inter;
    (*h).param.analyse.intra = (*param).analyse.intra;
    (*h).param.analyse.i_direct_mv_pred = (*param).analyse.i_direct_mv_pred;
    if (*h).param.analyse.i_me_method < X264_ME_ESA
        || (*param).analyse.i_me_range < (*h).param.analyse.i_me_range
    {
        (*h).param.analyse.i_me_range = (*param).analyse.i_me_range;
    }
    (*h).param.analyse.i_noise_reduction = (*param).analyse.i_noise_reduction;
    if (*h).param.analyse.i_subpel_refine != 0 {
        (*h).param.analyse.i_subpel_refine = (*param).analyse.i_subpel_refine;
    }
    (*h).param.analyse.i_trellis = (*param).analyse.i_trellis;
    (*h).param.analyse.b_chroma_me = (*param).analyse.b_chroma_me;
    (*h).param.analyse.b_dct_decimate = (*param).analyse.b_dct_decimate;
    (*h).param.analyse.b_fast_pskip = (*param).analyse.b_fast_pskip;
    (*h).param.analyse.b_mixed_references = (*param).analyse.b_mixed_references;
    (*h).param.analyse.f_psy_rd = (*param).analyse.f_psy_rd;
    (*h).param.analyse.f_psy_trellis = (*param).analyse.f_psy_trellis;
    (*h).param.crop_rect = (*param).crop_rect;
    if (*h).param.analyse.i_me_method >= X264_ME_ESA
        || (*param).analyse.i_me_method < X264_ME_ESA
    {
        (*h).param.analyse.i_me_method = (*param).analyse.i_me_method;
    }
    if (*h).param.analyse.i_me_method >= X264_ME_ESA
        && (*h).frames.b_have_sub8x8_esa == 0
    {
        (*h).param.analyse.inter &= !X264_ANALYSE_PSUB8x8;
    }
    if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
        (*h).param.analyse.b_transform_8x8 = (*param).analyse.b_transform_8x8;
    }
    if (*h).frames.i_max_ref1 > 1 as ::core::ffi::c_int {
        (*h).param.i_bframe_pyramid = (*param).i_bframe_pyramid;
    }
    (*h).param.i_slice_max_size = (*param).i_slice_max_size;
    (*h).param.i_slice_max_mbs = (*param).i_slice_max_mbs;
    (*h).param.i_slice_min_mbs = (*param).i_slice_min_mbs;
    (*h).param.i_slice_count = (*param).i_slice_count;
    (*h).param.i_slice_count_max = (*param).i_slice_count_max;
    (*h).param.b_tff = (*param).b_tff;
    if (*h).param.rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
        && (*h).param.rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
        && (*param).rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
        && (*param).rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
    {
        *rc_reconfig
            |= ((*h).param.rc.i_vbv_max_bitrate != (*param).rc.i_vbv_max_bitrate)
                as ::core::ffi::c_int;
        *rc_reconfig
            |= ((*h).param.rc.i_vbv_buffer_size != (*param).rc.i_vbv_buffer_size)
                as ::core::ffi::c_int;
        *rc_reconfig
            |= ((*h).param.rc.i_bitrate != (*param).rc.i_bitrate) as ::core::ffi::c_int;
        (*h).param.rc.i_vbv_max_bitrate = (*param).rc.i_vbv_max_bitrate;
        (*h).param.rc.i_vbv_buffer_size = (*param).rc.i_vbv_buffer_size;
        (*h).param.rc.i_bitrate = (*param).rc.i_bitrate;
    }
    *rc_reconfig
        |= ((*h).param.rc.f_rf_constant != (*param).rc.f_rf_constant)
            as ::core::ffi::c_int;
    *rc_reconfig
        |= ((*h).param.rc.f_rf_constant_max != (*param).rc.f_rf_constant_max)
            as ::core::ffi::c_int;
    (*h).param.rc.f_rf_constant = (*param).rc.f_rf_constant;
    (*h).param.rc.f_rf_constant_max = (*param).rc.f_rf_constant_max;
    return validate_parameters(h, 0 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1932:1"]
pub unsafe extern "C" fn x264_10_encoder_reconfig_apply(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut rc_reconfig: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = encoder_try_reconfig(h, param, &mut rc_reconfig);
    mbcmp_init(h);
    if ret == 0 {
        x264_10_sps_init_reconfigurable((*h).sps.as_mut_ptr(), &mut (*h).param);
    }
    if ret == 0 && rc_reconfig != 0 {
        x264_10_ratecontrol_init_reconfigurable(h, 0 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1955:1"]
pub unsafe extern "C" fn x264_10_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    h = (*h)
        .thread[(*(*h).thread[0 as ::core::ffi::c_int as usize]).i_thread_phase
        as usize];
    let mut param_save: x264_param_t = (*(*h).reconfig_h).param;
    (*(*h).reconfig_h).param = (*h).param;
    let mut rc_reconfig: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = encoder_try_reconfig(
        (*h).reconfig_h,
        param,
        &mut rc_reconfig,
    );
    if ret == 0 {
        (*h).reconfig = 1 as ::core::ffi::c_int;
    } else {
        (*(*h).reconfig_h).param = param_save;
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1974:1"]
pub unsafe extern "C" fn x264_10_encoder_parameters(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) {
    memcpy(
        param as *mut ::core::ffi::c_void,
        &mut (**(*h).thread.as_mut_ptr().offset((*h).i_thread_phase as isize)).param
            as *mut x264_param_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<x264_param_t>() as size_t,
    );
    (*param).opaque = NULL;
}
#[c2rust::src_loc = "1981:1"]
unsafe extern "C" fn nal_start(
    mut h: *mut x264_t,
    mut i_type: ::core::ffi::c_int,
    mut i_ref_idc: ::core::ffi::c_int,
) {
    let mut nal: *mut x264_nal_t = &mut *(*h).out.nal.offset((*h).out.i_nal as isize)
        as *mut x264_nal_t;
    (*nal).i_ref_idc = i_ref_idc;
    (*nal).i_type = i_type;
    (*nal).b_long_startcode = 1 as ::core::ffi::c_int;
    (*nal).i_payload = 0 as ::core::ffi::c_int;
    (*nal).p_payload = &mut *(*h)
        .out
        .p_bitstream
        .offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut bs_t,
                ) -> ::core::ffi::c_int)(&mut (*h).out.bs) / 8 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t;
    (*nal).i_padding = 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "1995:1"]
unsafe extern "C" fn nal_check_buffer(mut h: *mut x264_t) -> ::core::ffi::c_int {
    if (*h).out.i_nal >= (*h).out.i_nals_allocated {
        let mut new_out: *mut x264_nal_t = x264_malloc(
            (::core::mem::size_of::<x264_nal_t>() as usize)
                .wrapping_mul(
                    ((*h).out.i_nals_allocated * 2 as ::core::ffi::c_int) as usize,
                ) as int64_t,
        ) as *mut x264_nal_t;
        if new_out.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        memcpy(
            new_out as *mut ::core::ffi::c_void,
            (*h).out.nal as *const ::core::ffi::c_void,
            (::core::mem::size_of::<x264_nal_t>() as size_t)
                .wrapping_mul((*h).out.i_nals_allocated as size_t),
        );
        x264_free((*h).out.nal as *mut ::core::ffi::c_void);
        (*h).out.nal = new_out;
        (*h).out.i_nals_allocated *= 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "2010:1"]
unsafe extern "C" fn nal_end(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut nal: *mut x264_nal_t = &mut *(*h).out.nal.offset((*h).out.i_nal as isize)
        as *mut x264_nal_t;
    let mut end: *mut uint8_t = &mut *(*h)
        .out
        .p_bitstream
        .offset(
            ((bs_pos
                as unsafe extern "C" fn(
                    *mut bs_t,
                ) -> ::core::ffi::c_int)(&mut (*h).out.bs) / 8 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t;
    (*nal).i_payload = end.offset_from((*nal).p_payload) as ::core::ffi::c_long
        as ::core::ffi::c_int;
    memset(end as *mut ::core::ffi::c_void, 0xff as ::core::ffi::c_int, 64 as size_t);
    if (*h).param.nalu_process.is_some() {
        (*h)
            .param
            .nalu_process
            .expect(
                "non-null function pointer",
            )((*h).api as *mut x264_t, nal, (*(*h).fenc).opaque);
    }
    (*h).out.i_nal += 1;
    return nal_check_buffer(h);
}
#[c2rust::src_loc = "2025:1"]
unsafe extern "C" fn check_encapsulated_buffer(
    mut h: *mut x264_t,
    mut h0: *mut x264_t,
    mut start: ::core::ffi::c_int,
    mut previous_nal_size: int64_t,
    mut necessary_size: int64_t,
) -> ::core::ffi::c_int {
    if ((*h0).nal_buffer_size as int64_t) < necessary_size {
        necessary_size *= 2 as int64_t;
        if necessary_size > INT_MAX as int64_t {
            return -(1 as ::core::ffi::c_int);
        }
        let mut buf: *mut uint8_t = x264_malloc(necessary_size) as *mut uint8_t;
        if buf.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if previous_nal_size != 0 {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                (*h0).nal_buffer as *const ::core::ffi::c_void,
                previous_nal_size as size_t,
            );
        }
        let mut delta: intptr_t = buf.offset_from((*h0).nal_buffer) as intptr_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < start {
            let ref mut fresh1 = (*(*h).out.nal.offset(i as isize)).p_payload;
            *fresh1 = (*fresh1).offset(delta as isize);
            i += 1;
        }
        x264_free((*h0).nal_buffer as *mut ::core::ffi::c_void);
        (*h0).nal_buffer = buf;
        (*h0).nal_buffer_size = necessary_size as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "2051:1"]
unsafe extern "C" fn encoder_encapsulate_nals(
    mut h: *mut x264_t,
    mut start: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h0: *mut x264_t = (*h).thread[0 as ::core::ffi::c_int as usize];
    let mut nal_size: int64_t = 0 as int64_t;
    let mut previous_nal_size: int64_t = 0 as int64_t;
    if (*h).param.nalu_process.is_some() {
        let mut i: ::core::ffi::c_int = start;
        while i < (*h).out.i_nal {
            nal_size += (*(*h).out.nal.offset(i as isize)).i_payload as int64_t;
            i += 1;
        }
        if nal_size > INT_MAX as int64_t {
            return -(1 as ::core::ffi::c_int);
        }
        return nal_size as ::core::ffi::c_int;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < start {
        previous_nal_size += (*(*h).out.nal.offset(i_0 as isize)).i_payload as int64_t;
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = start;
    while i_1 < (*h).out.i_nal {
        nal_size += (*(*h).out.nal.offset(i_1 as isize)).i_payload as int64_t;
        i_1 += 1;
    }
    let mut necessary_size: int64_t = previous_nal_size
        + nal_size * 3 as int64_t / 2 as int64_t
        + ((*h).out.i_nal * 4 as ::core::ffi::c_int) as int64_t + 4 as int64_t
        + 64 as int64_t;
    let mut i_2: ::core::ffi::c_int = start;
    while i_2 < (*h).out.i_nal {
        necessary_size += (*(*h).out.nal.offset(i_2 as isize)).i_padding as int64_t;
        i_2 += 1;
    }
    if check_encapsulated_buffer(h, h0, start, previous_nal_size, necessary_size) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    let mut nal_buffer: *mut uint8_t = (*h0)
        .nal_buffer
        .offset(previous_nal_size as isize);
    let mut i_3: ::core::ffi::c_int = start;
    while i_3 < (*h).out.i_nal {
        (*(*h).out.nal.offset(i_3 as isize)).b_long_startcode = (i_3 == 0
            || (*(*h).out.nal.offset(i_3 as isize)).i_type
                == NAL_SPS as ::core::ffi::c_int
            || (*(*h).out.nal.offset(i_3 as isize)).i_type
                == NAL_PPS as ::core::ffi::c_int || (*h).param.i_avcintra_class != 0)
            as ::core::ffi::c_int;
        x264_10_nal_encode(h, nal_buffer, &mut *(*h).out.nal.offset(i_3 as isize));
        nal_buffer = nal_buffer
            .offset((*(*h).out.nal.offset(i_3 as isize)).i_payload as isize);
        i_3 += 1;
    }
    return nal_buffer.offset_from((*h0).nal_buffer.offset(previous_nal_size as isize))
        as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2096:1"]
pub unsafe extern "C" fn x264_10_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut frame_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*h).out.i_nal = 0 as ::core::ffi::c_int;
    bs_init(
        &mut (*h).out.bs,
        (*h).out.p_bitstream as *mut ::core::ffi::c_void,
        (*h).out.i_bitstream,
    );
    nal_start(
        h,
        NAL_SPS as ::core::ffi::c_int,
        NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
    );
    x264_10_sps_write(&mut (*h).out.bs, (*h).sps.as_mut_ptr());
    if nal_end(h) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    nal_start(
        h,
        NAL_PPS as ::core::ffi::c_int,
        NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
    );
    x264_10_pps_write(&mut (*h).out.bs, (*h).sps.as_mut_ptr(), (*h).pps.as_mut_ptr());
    if nal_end(h) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    nal_start(
        h,
        NAL_SEI as ::core::ffi::c_int,
        NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
    );
    if x264_10_sei_version_write(h, &mut (*h).out.bs) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if nal_end(h) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    frame_size = encoder_encapsulate_nals(h, 0 as ::core::ffi::c_int);
    if frame_size < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    *pi_nal = (*h).out.i_nal;
    *pp_nal = &mut *(*h).out.nal.offset(0 as ::core::ffi::c_int as isize)
        as *mut x264_nal_t;
    (*h).out.i_nal = 0 as ::core::ffi::c_int;
    return frame_size;
}
#[inline]
#[c2rust::src_loc = "2138:1"]
unsafe extern "C" fn reference_check_reorder(mut h: *mut x264_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*h).frames.reference[i as usize].is_null() {
        if (*(*h).frames.reference[i as usize]).b_corrupt != 0 {
            (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1
                as ::core::ffi::c_int;
            return;
        }
        i += 1;
    }
    let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while list
        <= ((*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int) as ::core::ffi::c_int
    {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).i_ref[list as usize] - 1 as ::core::ffi::c_int {
            let mut framenum_diff: ::core::ffi::c_int = (*(*h)
                .fref[list as usize][(i_0 + 1 as ::core::ffi::c_int) as usize])
                .i_frame_num - (*(*h).fref[list as usize][i_0 as usize]).i_frame_num;
            let mut poc_diff: ::core::ffi::c_int = (*(*h)
                .fref[list as usize][(i_0 + 1 as ::core::ffi::c_int) as usize])
                .i_poc - (*(*h).fref[list as usize][i_0 as usize]).i_poc;
            if if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
                (framenum_diff > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } else if list == 1 as ::core::ffi::c_int {
                (poc_diff < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } else {
                (poc_diff > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } != 0
            {
                (*h).b_ref_reorder[list as usize] = 1 as ::core::ffi::c_int;
                return;
            }
            i_0 += 1;
        }
        list += 1;
    }
}
#[c2rust::src_loc = "2163:1"]
unsafe extern "C" fn weighted_reference_duplicate(
    mut h: *mut x264_t,
    mut i_ref: ::core::ffi::c_int,
    mut w: *const x264_weight_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = (*h).i_ref[0 as ::core::ffi::c_int as usize];
    let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut newframe: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if i <= 1 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.analyse.i_weighted_pred != X264_WEIGHTP_SMART {
        return -(1 as ::core::ffi::c_int);
    }
    if BIT_DEPTH > 8 as ::core::ffi::c_int
        && w != x264_zero.as_mut_ptr() as *const x264_weight_t
    {
        return -(1 as ::core::ffi::c_int);
    }
    newframe = x264_10_frame_pop_blank_unused(h);
    if newframe.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *newframe = *(*h).fref[0 as ::core::ffi::c_int as usize][i_ref as usize];
    (*newframe).i_reference_count = 1 as ::core::ffi::c_int;
    (*newframe).orig = (*h).fref[0 as ::core::ffi::c_int as usize][i_ref as usize]
        as *mut x264_frame;
    (*newframe).b_duplicate = 1 as ::core::ffi::c_int;
    memcpy(
        (*(*(*h).fenc).weight.as_mut_ptr().offset(j as isize)).as_mut_ptr()
            as *mut ::core::ffi::c_void,
        w as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[x264_weight_t; 3]>() as size_t,
    );
    (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    if (*h).i_ref[0 as ::core::ffi::c_int as usize] < X264_REF_MAX {
        (*h).i_ref[0 as ::core::ffi::c_int as usize] += 1;
    }
    (*h)
        .fref[0 as ::core::ffi::c_int
        as usize][(X264_REF_MAX - 1 as ::core::ffi::c_int) as usize] = 0
        as *mut x264_frame_t;
    x264_10_frame_unshift(
        &mut *(*(*h).fref.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(j as isize),
        newframe,
    );
    return j;
}
#[c2rust::src_loc = "2202:1"]
unsafe extern "C" fn weighted_pred_init(mut h: *mut x264_t) {
    let mut i_ref: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_ref < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
        (*(*h).fenc).weighted[i_ref as usize] = (*(*h)
            .fref[0 as ::core::ffi::c_int as usize][i_ref as usize])
            .filtered[0 as ::core::ffi::c_int
            as usize][0 as ::core::ffi::c_int as usize];
        i_ref += 1;
    }
    (*(*h).fenc).i_lines_weighted = 0 as ::core::ffi::c_int;
    let mut i_ref_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_ref_0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).sh.b_mbaff {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            (*h).sh.weight[i_ref_0 as usize][i as usize].weightfn = 0
                as *mut weight_fn_t;
            i += 1;
        }
        i_ref_0 += 1;
    }
    if (*h).sh.i_type != SLICE_TYPE_P as ::core::ffi::c_int
        || (*h).param.analyse.i_weighted_pred <= 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut i_padv: ::core::ffi::c_int = PADV << (*h).param.b_interlaced;
    let mut denom: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut weightplane: [::core::ffi::c_int; 2] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut buffer_next: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 3 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            if !(*(*h).fenc).weight[j as usize][i_0 as usize].weightfn.is_null() {
                (*h).sh.weight[j as usize][i_0 as usize] = (*(*h).fenc)
                    .weight[j as usize][i_0 as usize];
                if (*h).sh.weight[j as usize][i_0 as usize].i_scale
                    == (1 as int32_t) << (*h).sh.weight[j as usize][i_0 as usize].i_denom
                    && (*h).sh.weight[j as usize][i_0 as usize].i_offset == 0 as int32_t
                {
                    (*h).sh.weight[j as usize][i_0 as usize].weightfn = 0
                        as *mut weight_fn_t;
                } else {
                    if weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] == 0 {
                        weightplane[(i_0 != 0) as ::core::ffi::c_int as usize] = 1
                            as ::core::ffi::c_int;
                        denom = (*h).sh.weight[j as usize][i_0 as usize].i_denom
                            as ::core::ffi::c_int;
                        (*h)
                            .sh
                            .weight[0 as ::core::ffi::c_int
                                as usize][(i_0 != 0) as ::core::ffi::c_int as usize]
                            .i_denom = denom as int32_t;
                        if x264_clip3(
                            denom,
                            0 as ::core::ffi::c_int,
                            7 as ::core::ffi::c_int,
                        ) == denom
                        {} else {
                            __assert_fail(
                                b"x264_clip3( denom, 0, 7 ) == denom\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                2240 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 34],
                                    [::core::ffi::c_char; 34],
                                >(*b"void weighted_pred_init(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                        'c_49115: {
                            if x264_clip3(
                                denom,
                                0 as ::core::ffi::c_int,
                                7 as ::core::ffi::c_int,
                            ) == denom
                            {} else {
                                __assert_fail(
                                    b"x264_clip3( denom, 0, 7 ) == denom\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"encoder/encoder.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    2240 as ::core::ffi::c_uint,
                                    ::core::mem::transmute::<
                                        [u8; 34],
                                        [::core::ffi::c_char; 34],
                                    >(*b"void weighted_pred_init(x264_t *)\0")
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                    if (*h).sh.weight[j as usize][i_0 as usize].i_denom
                        == denom as int32_t
                    {} else {
                        __assert_fail(
                            b"h->sh.weight[j][i].i_denom == denom\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"encoder/encoder.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            2243 as ::core::ffi::c_uint,
                            ::core::mem::transmute::<
                                [u8; 34],
                                [::core::ffi::c_char; 34],
                            >(*b"void weighted_pred_init(x264_t *)\0")
                                .as_ptr(),
                        );
                    }
                    'c_49045: {
                        if (*h).sh.weight[j as usize][i_0 as usize].i_denom
                            == denom as int32_t
                        {} else {
                            __assert_fail(
                                b"h->sh.weight[j][i].i_denom == denom\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/encoder.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                2243 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 34],
                                    [::core::ffi::c_char; 34],
                                >(*b"void weighted_pred_init(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                    };
                    if i_0 == 0 {
                        let fresh3 = buffer_next;
                        buffer_next = buffer_next + 1;
                        (*(*h).fenc).weighted[j as usize] = (*h)
                            .mb
                            .p_weight_buf[fresh3 as usize]
                            .offset(
                                ((*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize]
                                    * i_padv) as isize,
                            )
                            .offset(
                                (if 32 as ::core::ffi::c_int
                                    > 64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                {
                                    32 as ::core::ffi::c_int
                                } else {
                                    64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                }) as isize,
                            );
                        if (*h).param.i_threads == 1 as ::core::ffi::c_int {
                            let mut src: *mut pixel = (*(*h)
                                .fref[0 as ::core::ffi::c_int as usize][j as usize])
                                .filtered[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize]
                                .offset(
                                    -(((*(*h)
                                        .fref[0 as ::core::ffi::c_int as usize][j as usize])
                                        .i_stride[0 as ::core::ffi::c_int as usize] * i_padv)
                                        as isize),
                                )
                                .offset(
                                    -((if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize),
                                );
                            let mut dst: *mut pixel = (*(*h).fenc)
                                .weighted[j as usize]
                                .offset(
                                    -(((*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize]
                                        * i_padv) as isize),
                                )
                                .offset(
                                    -((if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize),
                                );
                            let mut stride: ::core::ffi::c_int = (*(*h).fenc)
                                .i_stride[0 as ::core::ffi::c_int as usize];
                            let mut width: ::core::ffi::c_int = (*(*h).fenc)
                                .i_width[0 as ::core::ffi::c_int as usize]
                                + ((if 32 as ::core::ffi::c_int
                                    > 64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                {
                                    32 as ::core::ffi::c_int
                                } else {
                                    64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                }) + PADH);
                            let mut height: ::core::ffi::c_int = (*(*h).fenc)
                                .i_lines[0 as ::core::ffi::c_int as usize]
                                + i_padv * 2 as ::core::ffi::c_int;
                            x264_10_weight_scale_plane(
                                h,
                                dst,
                                stride as intptr_t,
                                src,
                                stride as intptr_t,
                                width,
                                height,
                                &mut *(*(*h).sh.weight.as_mut_ptr().offset(j as isize))
                                    .as_mut_ptr()
                                    .offset(0 as ::core::ffi::c_int as isize),
                            );
                            (*(*h).fenc).i_lines_weighted = height;
                        }
                    }
                }
            }
            j += 1;
        }
        i_0 += 1;
    }
    if weightplane[1 as ::core::ffi::c_int as usize] != 0 {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            if !(*h)
                .sh
                .weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
                && (*h)
                    .sh
                    .weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
            {
                (*h).sh.weight[i_1 as usize][2 as ::core::ffi::c_int as usize].i_scale = ((1
                    as ::core::ffi::c_int)
                    << (*h)
                        .sh
                        .weight[0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize]
                        .i_denom) as int32_t;
                (*h)
                    .sh
                    .weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                    .i_offset = 0 as ::core::ffi::c_int as int32_t;
            } else if !(*h)
                .sh
                .weight[i_1 as usize][2 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
                && (*h)
                    .sh
                    .weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
            {
                (*h).sh.weight[i_1 as usize][1 as ::core::ffi::c_int as usize].i_scale = ((1
                    as ::core::ffi::c_int)
                    << (*h)
                        .sh
                        .weight[0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize]
                        .i_denom) as int32_t;
                (*h)
                    .sh
                    .weight[i_1 as usize][1 as ::core::ffi::c_int as usize]
                    .i_offset = 0 as ::core::ffi::c_int as int32_t;
            }
            i_1 += 1;
        }
    }
    if weightplane[0 as ::core::ffi::c_int as usize] == 0 {
        (*h)
            .sh
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            .i_denom = 0 as ::core::ffi::c_int as int32_t;
    }
    if weightplane[1 as ::core::ffi::c_int as usize] == 0 {
        (*h)
            .sh
            .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            .i_denom = 0 as ::core::ffi::c_int as int32_t;
    }
    (*h)
        .sh
        .weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
        .i_denom = (*h)
        .sh
        .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
        .i_denom;
}
#[inline]
#[c2rust::src_loc = "2286:1"]
unsafe extern "C" fn reference_distance(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) -> ::core::ffi::c_int {
    if (*h).param.i_frame_packing == 5 as ::core::ffi::c_int {
        return abs(
            ((*(*h).fenc).i_frame & !(1 as ::core::ffi::c_int))
                - ((*frame).i_frame & !(1 as ::core::ffi::c_int)),
        )
            + ((*(*h).fenc).i_frame & 1 as ::core::ffi::c_int
                != (*frame).i_frame & 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        return abs((*(*h).fenc).i_frame - (*frame).i_frame)
    };
}
#[inline]
#[c2rust::src_loc = "2295:1"]
unsafe extern "C" fn reference_build_list(
    mut h: *mut x264_t,
    mut i_poc: ::core::ffi::c_int,
) {
    let mut b_ok: ::core::ffi::c_int = 0;
    (*h).i_ref[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] = (*h)
        .i_ref[0 as ::core::ffi::c_int as usize];
    (*h).i_ref[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] = (*h)
        .i_ref[1 as ::core::ffi::c_int as usize];
    if (*h).sh.i_type == SLICE_TYPE_I as ::core::ffi::c_int {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*h).frames.reference[i as usize].is_null() {
        if !((*(*h).frames.reference[i as usize]).b_corrupt != 0) {
            if (*(*h).frames.reference[i as usize]).i_poc < i_poc {
                let fresh4 = (*h).i_ref[0 as ::core::ffi::c_int as usize];
                (*h).i_ref[0 as ::core::ffi::c_int as usize] = (*h)
                    .i_ref[0 as ::core::ffi::c_int as usize] + 1;
                (*h).fref[0 as ::core::ffi::c_int as usize][fresh4 as usize] = (*h)
                    .frames
                    .reference[i as usize];
            } else if (*(*h).frames.reference[i as usize]).i_poc > i_poc {
                let fresh5 = (*h).i_ref[1 as ::core::ffi::c_int as usize];
                (*h).i_ref[1 as ::core::ffi::c_int as usize] = (*h)
                    .i_ref[1 as ::core::ffi::c_int as usize] + 1;
                (*h).fref[1 as ::core::ffi::c_int as usize][fresh5 as usize] = (*h)
                    .frames
                    .reference[i as usize];
            }
        }
        i += 1;
    }
    if (*h).sh.i_mmco_remove_from_end != 0 {
        loop {
            b_ok = 1 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0
                < (*h).i_ref[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int
            {
                if (*(*h).fref[0 as ::core::ffi::c_int as usize][i_0 as usize]).i_frame
                    < (*(*h)
                        .fref[0 as ::core::ffi::c_int
                        as usize][(i_0 + 1 as ::core::ffi::c_int) as usize])
                        .i_frame
                {
                    let mut t: *mut x264_frame_t = (*h)
                        .fref[0 as ::core::ffi::c_int as usize][i_0 as usize];
                    (*h).fref[0 as ::core::ffi::c_int as usize][i_0 as usize] = (*h)
                        .fref[0 as ::core::ffi::c_int
                        as usize][(i_0 + 1 as ::core::ffi::c_int) as usize];
                    (*h)
                        .fref[0 as ::core::ffi::c_int
                        as usize][(i_0 + 1 as ::core::ffi::c_int) as usize] = t;
                    b_ok = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    i_0 += 1;
                }
            }
            if !(b_ok == 0) {
                break;
            }
        }
        let mut i_1: ::core::ffi::c_int = (*h).i_ref[0 as ::core::ffi::c_int as usize]
            - 1 as ::core::ffi::c_int;
        while i_1
            >= (*h).i_ref[0 as ::core::ffi::c_int as usize]
                - (*h).sh.i_mmco_remove_from_end
        {
            let mut diff: ::core::ffi::c_int = (*h).i_frame_num
                - (*(*h).fref[0 as ::core::ffi::c_int as usize][i_1 as usize])
                    .i_frame_num;
            (*h).sh.mmco[(*h).sh.i_mmco_command_count as usize].i_poc = (*(*h)
                .fref[0 as ::core::ffi::c_int as usize][i_1 as usize])
                .i_poc;
            let fresh6 = (*h).sh.i_mmco_command_count;
            (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
            (*h).sh.mmco[fresh6 as usize].i_difference_of_pic_nums = diff;
            i_1 -= 1;
        }
    }
    let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while list < 2 as ::core::ffi::c_int {
        (*h).fref_nearest[list as usize] = (*h)
            .fref[list as usize][0 as ::core::ffi::c_int as usize];
        loop {
            b_ok = 1 as ::core::ffi::c_int;
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < (*h).i_ref[list as usize] - 1 as ::core::ffi::c_int {
                if if list != 0 {
                    ((*(*h)
                        .fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize])
                        .i_poc < (*(*h).fref_nearest[list as usize]).i_poc)
                        as ::core::ffi::c_int
                } else {
                    ((*(*h)
                        .fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize])
                        .i_poc > (*(*h).fref_nearest[list as usize]).i_poc)
                        as ::core::ffi::c_int
                } != 0
                {
                    (*h).fref_nearest[list as usize] = (*h)
                        .fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize];
                }
                if reference_distance(h, (*h).fref[list as usize][i_2 as usize])
                    > reference_distance(
                        h,
                        (*h)
                            .fref[list
                            as usize][(i_2 + 1 as ::core::ffi::c_int) as usize],
                    )
                {
                    let mut t_0: *mut x264_frame_t = (*h)
                        .fref[list as usize][i_2 as usize];
                    (*h).fref[list as usize][i_2 as usize] = (*h)
                        .fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize];
                    (*h).fref[list as usize][(i_2 + 1 as ::core::ffi::c_int) as usize] = t_0;
                    b_ok = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    i_2 += 1;
                }
            }
            if !(b_ok == 0) {
                break;
            }
        }
        list += 1;
    }
    reference_check_reorder(h);
    (*h).i_ref[1 as ::core::ffi::c_int as usize] = if (*h)
        .i_ref[1 as ::core::ffi::c_int as usize] < (*h).frames.i_max_ref1
    {
        (*h).i_ref[1 as ::core::ffi::c_int as usize]
    } else {
        (*h).frames.i_max_ref1
    };
    (*h).i_ref[0 as ::core::ffi::c_int as usize] = if (*h)
        .i_ref[0 as ::core::ffi::c_int as usize] < (*h).frames.i_max_ref0
    {
        (*h).i_ref[0 as ::core::ffi::c_int as usize]
    } else {
        (*h).frames.i_max_ref0
    };
    (*h).i_ref[0 as ::core::ffi::c_int as usize] = if (*h)
        .i_ref[0 as ::core::ffi::c_int as usize] < (*h).param.i_frame_reference
    {
        (*h).i_ref[0 as ::core::ffi::c_int as usize]
    } else {
        (*h).param.i_frame_reference
    };
    if ((*(*h).fenc).i_type == X264_TYPE_B || (*(*h).fenc).i_type == X264_TYPE_BREF)
        && (*h).param.b_bluray_compat != 0
    {
        (*h).i_ref[0 as ::core::ffi::c_int as usize] = if (*h)
            .i_ref[0 as ::core::ffi::c_int as usize]
            < ((*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_type == 0x5 as ::core::ffi::c_int
                || (*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .i_type == 0x4 as ::core::ffi::c_int) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        {
            (*h).i_ref[0 as ::core::ffi::c_int as usize]
        } else {
            ((*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_type == 0x5 as ::core::ffi::c_int
                || (*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .i_type == 0x4 as ::core::ffi::c_int) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
        };
    }
    if (*(*h).fenc).i_type == X264_TYPE_P {
        let mut idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE {
            let mut w: [x264_weight_t; 3] = [x264_weight_t {
                cachea: [0; 8],
                cacheb: [0; 8],
                i_denom: 0,
                i_scale: 0,
                i_offset: 0,
                weightfn: 0 as *mut weight_fn_t,
            }; 3];
            w[2 as ::core::ffi::c_int as usize].weightfn = 0 as *mut weight_fn_t;
            w[1 as ::core::ffi::c_int as usize].weightfn = w[2 as ::core::ffi::c_int
                    as usize]
                .weightfn;
            if (*h).param.rc.b_stat_read != 0 {
                x264_10_ratecontrol_set_weights(h, (*h).fenc);
            }
            if (*(*h).fenc)
                .weight[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
            {
                (*(*h).fenc)
                    .weight[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .i_denom = 0 as ::core::ffi::c_int as int32_t;
                w[0 as ::core::ffi::c_int as usize].i_scale = 1 as ::core::ffi::c_int
                    as int32_t;
                w[0 as ::core::ffi::c_int as usize].i_denom = 0 as ::core::ffi::c_int
                    as int32_t;
                w[0 as ::core::ffi::c_int as usize].i_offset = -(1 as ::core::ffi::c_int)
                    as int32_t;
                (*h)
                    .mc
                    .weight_cache
                    .expect(
                        "non-null function pointer",
                    )(h, &mut *w.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize));
                idx = weighted_reference_duplicate(
                    h,
                    0 as ::core::ffi::c_int,
                    w.as_mut_ptr(),
                );
            } else {
                if (*(*h).fenc)
                    .weight[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .i_scale
                    == (1 as int32_t)
                        << (*(*h).fenc)
                            .weight[0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int as usize]
                            .i_denom
                {
                    (*(*h).fenc)
                        .weight[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize]
                        .i_scale = 1 as ::core::ffi::c_int as int32_t;
                    (*(*h).fenc)
                        .weight[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize]
                        .i_denom = 0 as ::core::ffi::c_int as int32_t;
                    (*(*h).fenc)
                        .weight[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize]
                        .i_offset = (*(*h).fenc)
                        .weight[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize]
                        .i_offset;
                    (*h)
                        .mc
                        .weight_cache
                        .expect(
                            "non-null function pointer",
                        )(
                        h,
                        &mut *(*(*(*h).fenc)
                            .weight
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize),
                    );
                }
                weighted_reference_duplicate(
                    h,
                    0 as ::core::ffi::c_int,
                    x264_zero.as_mut_ptr() as *const x264_weight_t,
                );
                if (*(*h).fenc)
                    .weight[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .i_offset > -(128 as int32_t)
                {
                    w[0 as ::core::ffi::c_int as usize] = (*(*h).fenc)
                        .weight[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize];
                    w[0 as ::core::ffi::c_int as usize].i_offset -= 1;
                    (*h)
                        .mc
                        .weight_cache
                        .expect(
                            "non-null function pointer",
                        )(
                        h,
                        &mut *w.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize),
                    );
                    idx = weighted_reference_duplicate(
                        h,
                        0 as ::core::ffi::c_int,
                        w.as_mut_ptr(),
                    );
                }
            }
        }
        (*h).mb.ref_blind_dupe = idx;
    }
    if (*h).i_ref[0 as ::core::ffi::c_int as usize]
        + (*h).i_ref[1 as ::core::ffi::c_int as usize] <= 16 as ::core::ffi::c_int
    {} else {
        __assert_fail(
            b"h->i_ref[0] + h->i_ref[1] <= X264_REF_MAX\0" as *const u8
                as *const ::core::ffi::c_char,
            b"encoder/encoder.c\0" as *const u8 as *const ::core::ffi::c_char,
            2408 as ::core::ffi::c_uint,
            ::core::mem::transmute::<
                [u8; 41],
                [::core::ffi::c_char; 41],
            >(*b"void reference_build_list(x264_t *, int)\0")
                .as_ptr(),
        );
    }
    'c_49572: {
        if (*h).i_ref[0 as ::core::ffi::c_int as usize]
            + (*h).i_ref[1 as ::core::ffi::c_int as usize] <= 16 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"h->i_ref[0] + h->i_ref[1] <= X264_REF_MAX\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"encoder/encoder.c\0" as *const u8 as *const ::core::ffi::c_char,
                2408 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 41],
                    [::core::ffi::c_char; 41],
                >(*b"void reference_build_list(x264_t *, int)\0")
                    .as_ptr(),
            );
        }
    };
    (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] = (*h)
        .i_ref[0 as ::core::ffi::c_int as usize];
    (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] = (*h)
        .i_ref[1 as ::core::ffi::c_int as usize];
}
#[c2rust::src_loc = "2413:1"]
unsafe extern "C" fn fdec_filter_row(
    mut h: *mut x264_t,
    mut mb_y: ::core::ffi::c_int,
    mut pass: ::core::ffi::c_int,
) {
    let mut b_hpel: ::core::ffi::c_int = (*(*h).fdec).b_kept_as_ref;
    let mut b_deblock: ::core::ffi::c_int = ((*h).sh.i_disable_deblocking_filter_idc
        != 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut b_end: ::core::ffi::c_int = (mb_y == (*h).i_threadslice_end)
        as ::core::ffi::c_int;
    let mut b_measure_quality: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut min_y: ::core::ffi::c_int = mb_y
        - ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff);
    let mut b_start: ::core::ffi::c_int = (min_y == (*h).i_threadslice_start)
        as ::core::ffi::c_int;
    let mut minpix_y: ::core::ffi::c_int = min_y * 16 as ::core::ffi::c_int
        - 4 as ::core::ffi::c_int * (b_start == 0) as ::core::ffi::c_int;
    let mut maxpix_y: ::core::ffi::c_int = mb_y * 16 as ::core::ffi::c_int
        - 4 as ::core::ffi::c_int * (b_end == 0) as ::core::ffi::c_int;
    b_deblock
        &= (b_hpel != 0 || (*h).param.b_full_recon != 0
            || !(*h).param.psz_dump_yuv.is_null()) as ::core::ffi::c_int;
    if (*h).param.b_sliced_threads != 0 {
        match pass {
            1 => {
                b_deblock &= ((*h).param.b_full_recon == 0) as ::core::ffi::c_int;
                b_hpel
                    &= !(b_start != 0 && min_y > 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                b_measure_quality = 0 as ::core::ffi::c_int;
            }
            2 => {
                b_deblock = 0 as ::core::ffi::c_int;
                b_measure_quality = 0 as ::core::ffi::c_int;
            }
            0 | _ => {
                b_deblock &= (*h).param.b_full_recon;
                b_hpel = 0 as ::core::ffi::c_int;
            }
        }
    }
    if mb_y & (*h).sh.b_mbaff != 0 {
        return;
    }
    if min_y < (*h).i_threadslice_start {
        return;
    }
    if b_deblock != 0 {
        let mut y: ::core::ffi::c_int = min_y;
        while y < mb_y {
            x264_10_frame_deblock_row(h, y);
            y += (1 as ::core::ffi::c_int) << (*h).sh.b_mbaff;
        }
    }
    if (*h).param.b_interlaced != 0
        && ((*h).param.b_sliced_threads == 0 || pass == 1 as ::core::ffi::c_int)
    {
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p < (*(*h).fdec).i_plane {
            let mut i: ::core::ffi::c_int = minpix_y
                >> ((*h).mb.chroma_v_shift != 0 && p != 0) as ::core::ffi::c_int;
            while i
                < maxpix_y
                    >> ((*h).mb.chroma_v_shift != 0 && p != 0) as ::core::ffi::c_int
            {
                memcpy(
                    (*(*h).fdec)
                        .plane_fld[p as usize]
                        .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                        as *mut ::core::ffi::c_void,
                    (*(*h).fdec)
                        .plane[p as usize]
                        .offset((i * (*(*h).fdec).i_stride[p as usize]) as isize)
                        as *const ::core::ffi::c_void,
                    ((*h).mb.i_mb_width * 16 as ::core::ffi::c_int * SIZEOF_PIXEL)
                        as size_t,
                );
                i += 1;
            }
            p += 1;
        }
    }
    if (*(*h).fdec).b_kept_as_ref != 0
        && ((*h).param.b_sliced_threads == 0 || pass == 1 as ::core::ffi::c_int)
    {
        x264_10_frame_expand_border(h, (*h).fdec, min_y);
    }
    if b_hpel != 0 {
        let mut end: ::core::ffi::c_int = (mb_y == (*h).mb.i_mb_height)
            as ::core::ffi::c_int;
        if (*h).param.analyse.i_subpel_refine != 0 {
            x264_10_frame_filter(h, (*h).fdec, min_y, end);
            x264_10_frame_expand_border_filtered(h, (*h).fdec, min_y, end);
        }
    }
    if (*h).sh.b_mbaff != 0 && pass == 0 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 3 as ::core::ffi::c_int {
            let mut t: *mut pixel = (*h)
                .intra_border_backup[0 as ::core::ffi::c_int as usize][i_0 as usize];
            (*h).intra_border_backup[0 as ::core::ffi::c_int as usize][i_0 as usize] = (*h)
                .intra_border_backup[3 as ::core::ffi::c_int as usize][i_0 as usize];
            (*h).intra_border_backup[3 as ::core::ffi::c_int as usize][i_0 as usize] = t;
            let mut t_0: *mut pixel = (*h)
                .intra_border_backup[1 as ::core::ffi::c_int as usize][i_0 as usize];
            (*h).intra_border_backup[1 as ::core::ffi::c_int as usize][i_0 as usize] = (*h)
                .intra_border_backup[4 as ::core::ffi::c_int as usize][i_0 as usize];
            (*h).intra_border_backup[4 as ::core::ffi::c_int as usize][i_0 as usize] = t_0;
            i_0 += 1;
        }
    }
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int && (*(*h).fdec).b_kept_as_ref != 0
    {
        x264_10_frame_cond_broadcast(
            (*h).fdec,
            mb_y * 16 as ::core::ffi::c_int
                + (if b_end != 0 {
                    10000 as ::core::ffi::c_int
                } else {
                    -(X264_THREAD_HEIGHT << (*h).sh.b_mbaff)
                }),
        );
    }
    if b_measure_quality != 0 {
        maxpix_y = if maxpix_y < (*h).param.i_height {
            maxpix_y
        } else {
            (*h).param.i_height
        };
        if (*h).param.analyse.b_psnr != 0 {
            let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_0
                < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            {
                (*h).stat.frame.i_ssd[p_0 as usize] = ((*h)
                    .stat
                    .frame
                    .i_ssd[p_0 as usize] as uint64_t)
                    .wrapping_add(
                        x264_10_pixel_ssd_wxh(
                            &mut (*h).pixf,
                            (*(*h).fdec)
                                .plane[p_0 as usize]
                                .offset(
                                    (minpix_y * (*(*h).fdec).i_stride[p_0 as usize]) as isize,
                                ),
                            (*(*h).fdec).i_stride[p_0 as usize] as intptr_t,
                            (*(*h).fenc)
                                .plane[p_0 as usize]
                                .offset(
                                    (minpix_y * (*(*h).fenc).i_stride[p_0 as usize]) as isize,
                                ),
                            (*(*h).fenc).i_stride[p_0 as usize] as intptr_t,
                            (*h).param.i_width,
                            maxpix_y - minpix_y,
                        ),
                    ) as int64_t as int64_t;
                p_0 += 1;
            }
            if !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                == CHROMA_444 as ::core::ffi::c_int)
            {
                let mut ssd_u: uint64_t = 0;
                let mut ssd_v: uint64_t = 0;
                let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
                x264_10_pixel_ssd_nv12(
                    &mut (*h).pixf,
                    (*(*h).fdec)
                        .plane[1 as ::core::ffi::c_int as usize]
                        .offset(
                            ((minpix_y >> v_shift)
                                * (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    (*(*h).fenc)
                        .plane[1 as ::core::ffi::c_int as usize]
                        .offset(
                            ((minpix_y >> v_shift)
                                * (*(*h).fenc).i_stride[1 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fenc).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    (*h).param.i_width >> 1 as ::core::ffi::c_int,
                    maxpix_y - minpix_y >> v_shift,
                    &mut ssd_u,
                    &mut ssd_v,
                );
                (*h).stat.frame.i_ssd[1 as ::core::ffi::c_int as usize] = ((*h)
                    .stat
                    .frame
                    .i_ssd[1 as ::core::ffi::c_int as usize] as uint64_t)
                    .wrapping_add(ssd_u) as int64_t as int64_t;
                (*h).stat.frame.i_ssd[2 as ::core::ffi::c_int as usize] = ((*h)
                    .stat
                    .frame
                    .i_ssd[2 as ::core::ffi::c_int as usize] as uint64_t)
                    .wrapping_add(ssd_v) as int64_t as int64_t;
            }
        }
        if (*h).param.analyse.b_ssim != 0 {
            let mut ssim_cnt: ::core::ffi::c_int = 0;
            minpix_y
                += if b_start != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    -(6 as ::core::ffi::c_int)
                };
            (*h).stat.frame.f_ssim
                += x264_10_pixel_ssim_wxh(
                    &mut (*h).pixf,
                    (*(*h).fdec)
                        .plane[0 as ::core::ffi::c_int as usize]
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(
                            (minpix_y
                                * (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    (*(*h).fenc)
                        .plane[0 as ::core::ffi::c_int as usize]
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(
                            (minpix_y
                                * (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize])
                                as isize,
                        ),
                    (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    (*h).param.i_width - 2 as ::core::ffi::c_int,
                    maxpix_y - minpix_y,
                    (*h).scratch_buffer,
                    &mut ssim_cnt,
                ) as ::core::ffi::c_double;
            (*h).stat.frame.i_ssim_cnt += ssim_cnt;
        }
    }
}
#[inline]
#[c2rust::src_loc = "2533:1"]
unsafe extern "C" fn reference_update(mut h: *mut x264_t) -> ::core::ffi::c_int {
    if (*(*h).fdec).b_kept_as_ref == 0 {
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            x264_10_frame_push_unused(h, (*h).fdec);
            (*h).fdec = x264_10_frame_pop_unused(h, 1 as ::core::ffi::c_int);
            if (*h).fdec.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).sh.i_mmco_command_count {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[j as usize].is_null() {
            if (*(*h).frames.reference[j as usize]).i_poc
                == (*h).sh.mmco[i as usize].i_poc
            {
                x264_10_frame_push_unused(
                    h,
                    x264_10_frame_shift(
                        &mut *(*h).frames.reference.as_mut_ptr().offset(j as isize),
                    ),
                );
            }
            j += 1;
        }
        i += 1;
    }
    x264_10_frame_push((*h).frames.reference.as_mut_ptr(), (*h).fdec);
    if !(*h)
        .frames
        .reference[(*(*h).sps.as_mut_ptr()).i_num_ref_frames as usize]
        .is_null()
    {
        x264_10_frame_push_unused(
            h,
            x264_10_frame_shift((*h).frames.reference.as_mut_ptr()),
        );
    }
    (*h).fdec = x264_10_frame_pop_unused(h, 1 as ::core::ffi::c_int);
    if (*h).fdec.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "2563:1"]
unsafe extern "C" fn reference_reset(mut h: *mut x264_t) {
    while !(*h).frames.reference[0 as ::core::ffi::c_int as usize].is_null() {
        x264_10_frame_push_unused(
            h,
            x264_10_frame_pop((*h).frames.reference.as_mut_ptr()),
        );
    }
    (*(*h).fenc).i_poc = 0 as ::core::ffi::c_int;
    (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
}
#[inline]
#[c2rust::src_loc = "2571:1"]
unsafe extern "C" fn reference_hierarchy_reset(mut h: *mut x264_t) {
    let mut ref_0: ::core::ffi::c_int = 0;
    let mut b_hasdelayframe: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*(*h).frames.current.offset(i as isize)).is_null()
        && (**(*h).frames.current.offset(i as isize)).i_type == X264_TYPE_B
    {
        b_hasdelayframe
            |= ((**(*h).frames.current.offset(i as isize)).i_coded
                != (**(*h).frames.current.offset(i as isize)).i_frame
                    + (*(*h).sps.as_mut_ptr()).vui.i_num_reorder_frames)
                as ::core::ffi::c_int;
        i += 1;
    }
    if (*h).param.i_bframe_pyramid != X264_B_PYRAMID_STRICT && b_hasdelayframe == 0
        && (*h).frames.i_poc_last_open_gop == -(1 as ::core::ffi::c_int)
    {
        return;
    }
    ref_0 = 0 as ::core::ffi::c_int;
    while !(*h).frames.reference[ref_0 as usize].is_null() {
        if (*h).param.i_bframe_pyramid == X264_B_PYRAMID_STRICT
            && (*(*h).frames.reference[ref_0 as usize]).i_type == X264_TYPE_BREF
            || (*(*h).frames.reference[ref_0 as usize]).i_poc
                < (*h).frames.i_poc_last_open_gop
                && (*h).sh.i_type != SLICE_TYPE_B as ::core::ffi::c_int
        {
            let mut diff: ::core::ffi::c_int = (*h).i_frame_num
                - (*(*h).frames.reference[ref_0 as usize]).i_frame_num;
            (*h)
                .sh
                .mmco[(*h).sh.i_mmco_command_count as usize]
                .i_difference_of_pic_nums = diff;
            let fresh7 = (*h).sh.i_mmco_command_count;
            (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_command_count + 1;
            (*h).sh.mmco[fresh7 as usize].i_poc = (*(*h)
                .frames
                .reference[ref_0 as usize])
                .i_poc;
            x264_10_frame_push_unused(
                h,
                x264_10_frame_shift(
                    &mut *(*h).frames.reference.as_mut_ptr().offset(ref_0 as isize),
                ),
            );
            (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = 1
                as ::core::ffi::c_int;
            ref_0 -= 1;
        }
        ref_0 += 1;
    }
    if (*h).param.i_bframe_pyramid != 0 {
        (*h).sh.i_mmco_remove_from_end = if ref_0 + 2 as ::core::ffi::c_int
            - (*h).frames.i_max_dpb > 0 as ::core::ffi::c_int
        {
            ref_0 + 2 as ::core::ffi::c_int - (*h).frames.i_max_dpb
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[inline]
#[c2rust::src_loc = "2608:1"]
unsafe extern "C" fn slice_init(
    mut h: *mut x264_t,
    mut i_nal_type: ::core::ffi::c_int,
    mut i_global_qp: ::core::ffi::c_int,
) {
    if i_nal_type == NAL_SLICE_IDR as ::core::ffi::c_int {
        slice_header_init(
            h,
            &mut (*h).sh,
            (*h).sps.as_mut_ptr(),
            (*h).pps.as_mut_ptr(),
            (*h).i_idr_pic_id,
            (*h).i_frame_num,
            i_global_qp,
        );
        if (*h).param.i_avcintra_class != 0 {
            match (*h).i_idr_pic_id {
                5 => {
                    (*h).i_idr_pic_id = 3 as ::core::ffi::c_int;
                }
                3 => {
                    (*h).i_idr_pic_id = 4 as ::core::ffi::c_int;
                }
                4 | _ => {
                    (*h).i_idr_pic_id = 5 as ::core::ffi::c_int;
                }
            }
        } else {
            (*h).i_idr_pic_id ^= 1 as ::core::ffi::c_int;
        }
    } else {
        slice_header_init(
            h,
            &mut (*h).sh,
            (*h).sps.as_mut_ptr(),
            (*h).pps.as_mut_ptr(),
            -(1 as ::core::ffi::c_int),
            (*h).i_frame_num,
            i_global_qp,
        );
        (*h).sh.i_num_ref_idx_l0_active = if (*h).i_ref[0 as ::core::ffi::c_int as usize]
            <= 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (*h).i_ref[0 as ::core::ffi::c_int as usize]
        };
        (*h).sh.i_num_ref_idx_l1_active = if (*h).i_ref[1 as ::core::ffi::c_int as usize]
            <= 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (*h).i_ref[1 as ::core::ffi::c_int as usize]
        };
        if (*h).sh.i_num_ref_idx_l0_active
            != (*(*h).pps.as_mut_ptr()).i_num_ref_idx_l0_default_active
            || (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
                && (*h).sh.i_num_ref_idx_l1_active
                    != (*(*h).pps.as_mut_ptr()).i_num_ref_idx_l1_default_active
        {
            (*h).sh.b_num_ref_idx_override = 1 as ::core::ffi::c_int;
        }
    }
    if (*(*h).fenc).i_type == X264_TYPE_BREF && (*h).param.b_bluray_compat != 0
        && (*h).sh.i_mmco_command_count != 0
    {
        (*h).b_sh_backup = 1 as ::core::ffi::c_int;
        (*h).sh_backup = (*h).sh;
    }
    (*(*h).fdec).i_frame_num = (*h).sh.i_frame_num;
    if (*(*h).sps.as_mut_ptr()).i_poc_type == 0 as ::core::ffi::c_int {
        (*h).sh.i_poc = (*(*h).fdec).i_poc;
        if (*h).param.b_interlaced != 0 {
            (*h).sh.i_delta_poc_bottom = if (*h).param.b_tff != 0 {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
            (*h).sh.i_poc
                += ((*h).sh.i_delta_poc_bottom == -(1 as ::core::ffi::c_int))
                    as ::core::ffi::c_int;
        } else {
            (*h).sh.i_delta_poc_bottom = 0 as ::core::ffi::c_int;
        }
        (*(*h).fdec).i_delta_poc[0 as ::core::ffi::c_int as usize] = ((*h)
            .sh
            .i_delta_poc_bottom == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
        (*(*h).fdec).i_delta_poc[1 as ::core::ffi::c_int as usize] = ((*h)
            .sh
            .i_delta_poc_bottom == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    x264_10_macroblock_slice_init(h);
}
#[inline(always)]
#[c2rust::src_loc = "2689:1"]
unsafe extern "C" fn bitstream_backup(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut i_skip: ::core::ffi::c_int,
    mut full: ::core::ffi::c_int,
) {
    if full != 0 {
        (*bak).stat = (*h).stat.frame;
        (*bak).last_qp = (*h).mb.i_last_qp;
        (*bak).last_dqp = (*h).mb.i_last_dqp;
        (*bak).field_decoding_flag = (*h).mb.field_decoding_flag;
    } else {
        (*bak).stat.i_mv_bits = (*h).stat.frame.i_mv_bits;
        (*bak).stat.i_tex_bits = (*h).stat.frame.i_tex_bits;
    }
    if (*h).param.b_cabac != 0 {
        if full != 0 {
            memcpy(
                &mut (*bak).cabac as *mut x264_cabac_t as *mut ::core::ffi::c_void,
                &mut (*h).cabac as *mut x264_cabac_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<x264_cabac_t>() as size_t,
            );
        } else {
            memcpy(
                &mut (*bak).cabac as *mut x264_cabac_t as *mut ::core::ffi::c_void,
                &mut (*h).cabac as *mut x264_cabac_t as *const ::core::ffi::c_void,
                64 as size_t,
            );
        }
        (*bak).cabac_prevbyte = *(*h)
            .cabac
            .p
            .offset(-(1 as ::core::ffi::c_int) as isize);
    } else {
        (*bak).bs = (*h).out.bs;
        (*bak).skip = i_skip;
    };
}
#[inline(always)]
#[c2rust::src_loc = "2723:1"]
unsafe extern "C" fn bitstream_restore(
    mut h: *mut x264_t,
    mut bak: *mut x264_bs_bak_t,
    mut skip: *mut ::core::ffi::c_int,
    mut full: ::core::ffi::c_int,
) {
    if full != 0 {
        (*h).stat.frame = (*bak).stat;
        (*h).mb.i_last_qp = (*bak).last_qp;
        (*h).mb.i_last_dqp = (*bak).last_dqp;
        (*h).mb.field_decoding_flag = (*bak).field_decoding_flag;
    } else {
        (*h).stat.frame.i_mv_bits = (*bak).stat.i_mv_bits;
        (*h).stat.frame.i_tex_bits = (*bak).stat.i_tex_bits;
    }
    if (*h).param.b_cabac != 0 {
        if full != 0 {
            memcpy(
                &mut (*h).cabac as *mut x264_cabac_t as *mut ::core::ffi::c_void,
                &mut (*bak).cabac as *mut x264_cabac_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<x264_cabac_t>() as size_t,
            );
        } else {
            memcpy(
                &mut (*h).cabac as *mut x264_cabac_t as *mut ::core::ffi::c_void,
                &mut (*bak).cabac as *mut x264_cabac_t as *const ::core::ffi::c_void,
                64 as size_t,
            );
        }
        *(*h).cabac.p.offset(-(1 as ::core::ffi::c_int) as isize) = (*bak)
            .cabac_prevbyte;
    } else {
        (*h).out.bs = (*bak).bs;
        *skip = (*bak).skip;
    };
}
#[c2rust::src_loc = "2752:1"]
unsafe extern "C" fn slice_write(mut h: *mut x264_t) -> intptr_t {
    let mut i_skip: ::core::ffi::c_int = 0;
    let mut mb_xy: ::core::ffi::c_int = 0;
    let mut i_mb_x: ::core::ffi::c_int = 0;
    let mut i_mb_y: ::core::ffi::c_int = 0;
    let mut overhead_guess: ::core::ffi::c_int = NALU_OVERHEAD
        - ((*h).param.b_annexb != 0 && (*h).out.i_nal != 0) as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int + (*h).param.b_cabac + 5 as ::core::ffi::c_int;
    let mut slice_max_size: ::core::ffi::c_int = if (*h).param.i_slice_max_size
        > 0 as ::core::ffi::c_int
    {
        ((*h).param.i_slice_max_size - overhead_guess) * 8 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut back_up_bitstream_cavlc: ::core::ffi::c_int = ((*h).param.b_cabac == 0
        && (*(*h).sps.as_mut_ptr()).i_profile_idc < PROFILE_HIGH as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut back_up_bitstream: ::core::ffi::c_int = (slice_max_size != 0
        || back_up_bitstream_cavlc != 0) as ::core::ffi::c_int;
    let mut starting_bits: ::core::ffi::c_int = bs_pos(&mut (*h).out.bs);
    let mut b_deblock: ::core::ffi::c_int = ((*h).sh.i_disable_deblocking_filter_idc
        != 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut b_hpel: ::core::ffi::c_int = (*(*h).fdec).b_kept_as_ref;
    let mut orig_last_mb: ::core::ffi::c_int = (*h).sh.i_last_mb;
    let mut thread_last_mb: ::core::ffi::c_int = (*h).i_threadslice_end
        * (*h).mb.i_mb_width - 1 as ::core::ffi::c_int;
    let mut last_emu_check: *mut uint8_t = 0 as *mut uint8_t;
    let mut bs_bak: [x264_bs_bak_t; 4] = [x264_bs_bak_t {
        skip: 0,
        cabac_prevbyte: 0,
        bs: bs_s {
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        },
        cabac: x264_cabac_t {
            i_low: 0,
            i_range: 0,
            i_queue: 0,
            i_bytes_outstanding: 0,
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            f8_bits_encoded: 0,
            state: [0; 1024],
            padding: [0; 12],
        },
        stat: x264_frame_stat_t {
            i_mv_bits: 0,
            i_tex_bits: 0,
            i_misc_bits: 0,
            i_mb_count: [0; 19],
            i_mb_count_i: 0,
            i_mb_count_p: 0,
            i_mb_count_skip: 0,
            i_mb_count_8x8dct: [0; 2],
            i_mb_count_ref: [[0; 32]; 2],
            i_mb_partition: [0; 17],
            i_mb_cbp: [0; 6],
            i_mb_pred_mode: [[0; 13]; 4],
            i_mb_field: [0; 3],
            i_direct_score: [0; 2],
            i_ssd: [0; 3],
            f_ssim: 0.,
            i_ssim_cnt: 0,
        },
        last_qp: 0,
        last_dqp: 0,
        field_decoding_flag: 0,
    }; 4];
    b_deblock
        &= (b_hpel != 0 || (*h).param.b_full_recon != 0
            || !(*h).param.psz_dump_yuv.is_null()) as ::core::ffi::c_int;
    bs_realign(&mut (*h).out.bs);
    nal_start(h, (*h).i_nal_type, (*h).i_nal_ref_idc);
    (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_first_mb = (*h).sh.i_first_mb;
    x264_10_macroblock_thread_init(h);
    (*h).mb.i_mb_xy = (*h).sh.i_first_mb;
    (*h).sh.i_qp = x264_10_ratecontrol_mb_qp(h);
    (*h).sh.i_qp = if (*h).sh.i_qp
        < 51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    {
        (*h).sh.i_qp
    } else {
        51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    };
    (*h).sh.i_qp_delta = (*h).sh.i_qp - (*(*h).pps.as_mut_ptr()).i_pic_init_qp;
    slice_header_write(&mut (*h).out.bs, &mut (*h).sh, (*h).i_nal_ref_idc);
    if (*h).param.b_cabac != 0 {
        bs_align_1(&mut (*h).out.bs);
        x264_10_cabac_context_init(
            h,
            &mut (*h).cabac,
            (*h).sh.i_type,
            x264_clip3(
                (*h).sh.i_qp - QP_BD_OFFSET,
                0 as ::core::ffi::c_int,
                51 as ::core::ffi::c_int,
            ),
            (*h).sh.i_cabac_init_idc,
        );
        x264_10_cabac_encode_init(&mut (*h).cabac, (*h).out.bs.p, (*h).out.bs.p_end);
        last_emu_check = (*h).cabac.p;
    } else {
        last_emu_check = (*h).out.bs.p;
    }
    (*h).mb.i_last_qp = (*h).sh.i_qp;
    (*h).mb.i_last_dqp = 0 as ::core::ffi::c_int;
    (*h).mb.field_decoding_flag = 0 as ::core::ffi::c_int;
    i_mb_y = (*h).sh.i_first_mb / (*h).mb.i_mb_width;
    i_mb_x = (*h).sh.i_first_mb % (*h).mb.i_mb_width;
    i_skip = 0 as ::core::ffi::c_int;
    loop {
        mb_xy = i_mb_x + i_mb_y * (*h).mb.i_mb_width;
        let mut mb_spos: ::core::ffi::c_int = bs_pos(&mut (*h).out.bs)
            + x264_cabac_pos(&mut (*h).cabac);
        if i_mb_x == 0 as ::core::ffi::c_int {
            if bitstream_check_buffer(h) != 0 {
                return -(1 as ::core::ffi::c_int) as intptr_t;
            }
            if i_mb_y & (*h).sh.b_mbaff == 0 && (*h).param.rc.i_vbv_buffer_size != 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_ROW_VBV as isize),
                    i_skip,
                    1 as ::core::ffi::c_int,
                );
            }
            if (*h).mb.b_reencode_mb == 0 {
                fdec_filter_row(h, i_mb_y, 0 as ::core::ffi::c_int);
            }
        }
        if back_up_bitstream != 0 {
            if back_up_bitstream_cavlc != 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_CAVLC_OVERFLOW as isize),
                    i_skip,
                    0 as ::core::ffi::c_int,
                );
            }
            if slice_max_size != 0 && i_mb_y & (*h).sh.b_mbaff == 0 {
                bitstream_backup(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MAX_SIZE as isize),
                    i_skip,
                    0 as ::core::ffi::c_int,
                );
                if thread_last_mb + 1 as ::core::ffi::c_int - mb_xy
                    == (*h).param.i_slice_min_mbs
                {
                    bitstream_backup(
                        h,
                        &mut *bs_bak.as_mut_ptr().offset(BS_BAK_SLICE_MIN_MBS as isize),
                        i_skip,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        }
        if (*h).param.b_interlaced != 0 {
            if (*h).mb.b_adaptive_mbaff != 0 {
                if i_mb_y & 1 as ::core::ffi::c_int == 0 {
                    (*h).mb.b_interlaced = x264_10_field_vsad(h, i_mb_x, i_mb_y);
                    memcpy(
                        &mut (*h).zigzagf as *mut x264_zigzag_function_t
                            as *mut ::core::ffi::c_void,
                        (if (*h).mb.b_interlaced != 0 {
                            &mut (*h).zigzagf_interlaced as *mut x264_zigzag_function_t
                        } else {
                            &mut (*h).zigzagf_progressive as *mut x264_zigzag_function_t
                        }) as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<x264_zigzag_function_t>() as size_t,
                    );
                    if (*h).mb.b_interlaced == 0
                        && i_mb_y + 2 as ::core::ffi::c_int == (*h).mb.i_mb_height
                    {
                        x264_10_expand_border_mbpair(h, i_mb_x, i_mb_y);
                    }
                }
            }
            *(*h).mb.field.offset(mb_xy as isize) = (*h).mb.b_interlaced as uint8_t;
        }
        if (*h).sh.b_mbaff != 0 {
            x264_10_macroblock_cache_load_interlaced(h, i_mb_x, i_mb_y);
        } else {
            x264_10_macroblock_cache_load_progressive(h, i_mb_x, i_mb_y);
        }
        x264_10_macroblock_analyse(h);
        loop {
            x264_10_macroblock_encode(h);
            if (*h).param.b_cabac != 0 {
                if mb_xy > (*h).sh.i_first_mb
                    && !((*h).sh.b_mbaff != 0 && i_mb_y & 1 as ::core::ffi::c_int != 0)
                {
                    x264_10_cabac_encode_terminal_c(&mut (*h).cabac);
                }
                if (*h).mb.i_type == P_SKIP as ::core::ffi::c_int
                    || (*h).mb.i_type == B_SKIP as ::core::ffi::c_int
                {
                    x264_10_cabac_mb_skip(h, 1 as ::core::ffi::c_int);
                } else {
                    if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int {
                        x264_10_cabac_mb_skip(h, 0 as ::core::ffi::c_int);
                    }
                    x264_10_macroblock_write_cabac(h, &mut (*h).cabac);
                }
                break;
            } else if (*h).mb.i_type == P_SKIP as ::core::ffi::c_int
                || (*h).mb.i_type == B_SKIP as ::core::ffi::c_int
            {
                i_skip += 1;
                break;
            } else {
                if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int {
                    bs_write_ue_big(&mut (*h).out.bs, i_skip as ::core::ffi::c_uint);
                    i_skip = 0 as ::core::ffi::c_int;
                }
                x264_10_macroblock_write_cavlc(h);
                if !((*h).mb.b_overflow != 0) {
                    break;
                }
                (*h).mb.i_qp += 1;
                (*h).mb.i_chroma_qp = *(*h).chroma_qp_table.offset((*h).mb.i_qp as isize)
                    as ::core::ffi::c_int;
                (*h).mb.i_skip_intra = 0 as ::core::ffi::c_int;
                (*h).mb.b_skip_mc = 0 as ::core::ffi::c_int;
                (*h).mb.b_overflow = 0 as ::core::ffi::c_int;
                bitstream_restore(
                    h,
                    &mut *bs_bak.as_mut_ptr().offset(BS_BAK_CAVLC_OVERFLOW as isize),
                    &mut i_skip,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        let mut total_bits: ::core::ffi::c_int = bs_pos(&mut (*h).out.bs)
            + x264_cabac_pos(&mut (*h).cabac);
        let mut mb_size: ::core::ffi::c_int = total_bits - mb_spos;
        if slice_max_size != 0
            && ((*h).sh.b_mbaff == 0 || i_mb_y & 1 as ::core::ffi::c_int != 0)
        {
            if (*h).param.b_cabac == 0 {
                total_bits += bs_size_ue_big(i_skip as ::core::ffi::c_uint);
            }
            let mut end: *mut uint8_t = if (*h).param.b_cabac != 0 {
                (*h).cabac.p
            } else {
                (*h).out.bs.p
            };
            while last_emu_check < end.offset(-(2 as ::core::ffi::c_int as isize)) {
                if *last_emu_check.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && *last_emu_check.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && *last_emu_check.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int <= 3 as ::core::ffi::c_int
                {
                    slice_max_size -= 8 as ::core::ffi::c_int;
                    last_emu_check = last_emu_check.offset(1);
                }
                last_emu_check = last_emu_check.offset(1);
            }
            if total_bits - starting_bits > slice_max_size && (*h).mb.b_reencode_mb == 0
            {
                if x264_10_frame_new_slice(h, (*h).fdec) == 0 {
                    if mb_xy <= thread_last_mb
                        && thread_last_mb + 1 as ::core::ffi::c_int - mb_xy
                            < (*h).param.i_slice_min_mbs
                    {
                        if thread_last_mb - (*h).param.i_slice_min_mbs
                            < (*h).sh.i_first_mb + (*h).param.i_slice_min_mbs
                        {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"slice-max-size violated (frame %d, cause: slice-min-mbs)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                (*h).i_frame,
                            );
                            slice_max_size = 0 as ::core::ffi::c_int;
                        } else {
                            bitstream_restore(
                                h,
                                &mut *bs_bak
                                    .as_mut_ptr()
                                    .offset(BS_BAK_SLICE_MIN_MBS as isize),
                                &mut i_skip,
                                0 as ::core::ffi::c_int,
                            );
                            (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
                            (*h).sh.i_last_mb = thread_last_mb
                                - (*h).param.i_slice_min_mbs;
                            break;
                        }
                    } else if mb_xy - (*h).sh.b_mbaff * (*h).mb.i_mb_stride
                        != (*h).sh.i_first_mb
                    {
                        bitstream_restore(
                            h,
                            &mut *bs_bak
                                .as_mut_ptr()
                                .offset(BS_BAK_SLICE_MAX_SIZE as isize),
                            &mut i_skip,
                            0 as ::core::ffi::c_int,
                        );
                        (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
                        if (*h).sh.b_mbaff != 0 {
                            if i_mb_x != 0 {
                                (*h).sh.i_last_mb = mb_xy - 1 as ::core::ffi::c_int
                                    + (*h).mb.i_mb_stride
                                        * (i_mb_y & 1 as ::core::ffi::c_int == 0)
                                            as ::core::ffi::c_int;
                            } else {
                                (*h).sh.i_last_mb = (i_mb_y - 2 as ::core::ffi::c_int
                                    + (i_mb_y & 1 as ::core::ffi::c_int == 0)
                                        as ::core::ffi::c_int) * (*h).mb.i_mb_stride
                                    + (*h).mb.i_mb_width - 1 as ::core::ffi::c_int;
                            }
                        } else {
                            (*h).sh.i_last_mb = mb_xy - 1 as ::core::ffi::c_int;
                        }
                        break;
                    } else {
                        (*h).sh.i_last_mb = mb_xy;
                    }
                } else {
                    slice_max_size = 0 as ::core::ffi::c_int;
                }
            }
        }
        (*h).mb.b_reencode_mb = 0 as ::core::ffi::c_int;
        x264_10_macroblock_cache_save(h);
        if x264_10_ratecontrol_mb(h, mb_size) < 0 as ::core::ffi::c_int {
            bitstream_restore(
                h,
                &mut *bs_bak.as_mut_ptr().offset(BS_BAK_ROW_VBV as isize),
                &mut i_skip,
                1 as ::core::ffi::c_int,
            );
            (*h).mb.b_reencode_mb = 1 as ::core::ffi::c_int;
            i_mb_x = 0 as ::core::ffi::c_int;
            i_mb_y = i_mb_y - (*h).sh.b_mbaff;
            (*h).mb.i_mb_prev_xy = i_mb_y * (*h).mb.i_mb_stride
                - 1 as ::core::ffi::c_int;
            (*h).sh.i_last_mb = orig_last_mb;
        } else {
            (*h).stat.frame.i_mb_count[(*h).mb.i_type as usize] += 1;
            let mut b_intra: ::core::ffi::c_int = ((*h).mb.i_type
                == I_4x4 as ::core::ffi::c_int
                || (*h).mb.i_type == I_8x8 as ::core::ffi::c_int
                || (*h).mb.i_type == I_16x16 as ::core::ffi::c_int
                || (*h).mb.i_type == I_PCM as ::core::ffi::c_int) as ::core::ffi::c_int;
            let mut b_skip: ::core::ffi::c_int = ((*h).mb.i_type
                == P_SKIP as ::core::ffi::c_int
                || (*h).mb.i_type == B_SKIP as ::core::ffi::c_int) as ::core::ffi::c_int;
            if (*h).param.i_log_level >= X264_LOG_INFO || (*h).param.rc.b_stat_write != 0
            {
                if b_intra == 0 && b_skip == 0
                    && !((*h).mb.i_type == B_DIRECT as ::core::ffi::c_int)
                {
                    if (*h).mb.i_partition != D_8x8 as ::core::ffi::c_int {
                        (*h).stat.frame.i_mb_partition[(*h).mb.i_partition as usize]
                            += 4 as ::core::ffi::c_int;
                    } else {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < 4 as ::core::ffi::c_int {
                            (*h)
                                .stat
                                .frame
                                .i_mb_partition[(*h).mb.i_sub_partition[i as usize]
                                as usize] += 1;
                            i += 1;
                        }
                    }
                    if (*h).param.i_frame_reference > 1 as ::core::ffi::c_int {
                        let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_list
                            <= ((*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                        {
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_0 < 4 as ::core::ffi::c_int {
                                let mut i_ref: ::core::ffi::c_int = (*h)
                                    .mb
                                    .cache
                                    .ref_0[i_list
                                    as usize][x264_scan8[(4 as ::core::ffi::c_int * i_0)
                                    as usize] as usize] as ::core::ffi::c_int;
                                if i_ref >= 0 as ::core::ffi::c_int {
                                    (*h)
                                        .stat
                                        .frame
                                        .i_mb_count_ref[i_list as usize][i_ref as usize] += 1;
                                }
                                i_0 += 1;
                            }
                            i_list += 1;
                        }
                    }
                }
            }
            if (*h).param.i_log_level >= X264_LOG_INFO {
                if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_1 < 4 as ::core::ffi::c_int {
                            if (*h).mb.i_cbp_luma & (1 as ::core::ffi::c_int) << i_1 != 0
                            {
                                let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while p < 3 as ::core::ffi::c_int {
                                    let mut s8: ::core::ffi::c_int = i_1
                                        * 4 as ::core::ffi::c_int + p * 16 as ::core::ffi::c_int;
                                    let mut nnz8x8: ::core::ffi::c_int = (*(&mut *(*h)
                                        .mb
                                        .cache
                                        .non_zero_count
                                        .as_mut_ptr()
                                        .offset(
                                            (*x264_scan8.as_ptr().offset(s8 as isize)
                                                as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                                        ) as *mut uint8_t as *mut x264_union16_t))
                                        .i as ::core::ffi::c_int
                                        | (*(&mut *(*h)
                                            .mb
                                            .cache
                                            .non_zero_count
                                            .as_mut_ptr()
                                            .offset(
                                                (*x264_scan8.as_ptr().offset(s8 as isize)
                                                    as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                                            ) as *mut uint8_t as *mut x264_union16_t))
                                            .i as ::core::ffi::c_int;
                                    (*h)
                                        .stat
                                        .frame
                                        .i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                                        + p * 2 as ::core::ffi::c_int) as usize]
                                        += (nnz8x8 != 0) as ::core::ffi::c_int;
                                    p += 1;
                                }
                            }
                            i_1 += 1;
                        }
                    } else {
                        let mut cbpsum: ::core::ffi::c_int = ((*h).mb.i_cbp_luma
                            & 1 as ::core::ffi::c_int)
                            + ((*h).mb.i_cbp_luma >> 1 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int)
                            + ((*h).mb.i_cbp_luma >> 2 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int)
                            + ((*h).mb.i_cbp_luma >> 3 as ::core::ffi::c_int);
                        (*h)
                            .stat
                            .frame
                            .i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int) as usize] += cbpsum;
                        (*h)
                            .stat
                            .frame
                            .i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int) as usize]
                            += ((*h).mb.i_cbp_chroma != 0) as ::core::ffi::c_int;
                        (*h)
                            .stat
                            .frame
                            .i_mb_cbp[((b_intra == 0) as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int) as usize]
                            += (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int;
                    }
                }
                if (*h).mb.i_cbp_luma != 0 && b_intra == 0 {
                    (*h).stat.frame.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize]
                        += 1;
                    (*h).stat.frame.i_mb_count_8x8dct[1 as ::core::ffi::c_int as usize]
                        += (*h).mb.b_transform_8x8;
                }
                if b_intra != 0 && (*h).mb.i_type != I_PCM as ::core::ffi::c_int {
                    if (*h).mb.i_type == I_16x16 as ::core::ffi::c_int {
                        (*h)
                            .stat
                            .frame
                            .i_mb_pred_mode[0 as ::core::ffi::c_int
                            as usize][(*h).mb.i_intra16x16_pred_mode as usize] += 1;
                    } else if (*h).mb.i_type == I_8x8 as ::core::ffi::c_int {
                        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_2 < 16 as ::core::ffi::c_int {
                            (*h)
                                .stat
                                .frame
                                .i_mb_pred_mode[1 as ::core::ffi::c_int
                                as usize][(*h)
                                .mb
                                .cache
                                .intra4x4_pred_mode[x264_scan8[i_2 as usize] as usize]
                                as usize] += 1;
                            i_2 += 4 as ::core::ffi::c_int;
                        }
                    } else {
                        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_3 < 16 as ::core::ffi::c_int {
                            (*h)
                                .stat
                                .frame
                                .i_mb_pred_mode[2 as ::core::ffi::c_int
                                as usize][(*h)
                                .mb
                                .cache
                                .intra4x4_pred_mode[x264_scan8[i_3 as usize] as usize]
                                as usize] += 1;
                            i_3 += 1;
                        }
                    }
                    (*h)
                        .stat
                        .frame
                        .i_mb_pred_mode[3 as ::core::ffi::c_int
                        as usize][x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode
                        as usize] as usize] += 1;
                }
                (*h)
                    .stat
                    .frame
                    .i_mb_field[(if b_intra != 0 {
                    0 as ::core::ffi::c_int
                } else if b_skip != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as usize] += (*h).mb.b_interlaced;
            }
            if b_deblock != 0 {
                x264_10_macroblock_deblock_strength(h);
            }
            if mb_xy == (*h).sh.i_last_mb {
                break;
            }
            if (*h).sh.b_mbaff != 0 {
                i_mb_x += i_mb_y & 1 as ::core::ffi::c_int;
                i_mb_y ^= (i_mb_x < (*h).mb.i_mb_width) as ::core::ffi::c_int;
            } else {
                i_mb_x += 1;
            }
            if i_mb_x == (*h).mb.i_mb_width {
                i_mb_y += 1;
                i_mb_x = 0 as ::core::ffi::c_int;
            }
        }
    }
    if (*h).sh.i_last_mb < (*h).sh.i_first_mb {
        return 0 as intptr_t;
    }
    (*(*h).out.nal.offset((*h).out.i_nal as isize)).i_last_mb = (*h).sh.i_last_mb;
    if (*h).param.b_cabac != 0 {
        x264_10_cabac_encode_flush(h, &mut (*h).cabac);
        (*h).out.bs.p = (*h).cabac.p;
    } else {
        if i_skip > 0 as ::core::ffi::c_int {
            bs_write_ue_big(&mut (*h).out.bs, i_skip as ::core::ffi::c_uint);
        }
        bs_rbsp_trailing(&mut (*h).out.bs);
        bs_flush(&mut (*h).out.bs);
    }
    if nal_end(h) != 0 {
        return -(1 as ::core::ffi::c_int) as intptr_t;
    }
    if (*h).sh.i_last_mb
        == (*h).i_threadslice_end * (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
    {
        (*h).stat.frame.i_misc_bits = bs_pos(&mut (*h).out.bs)
            + (*h).out.i_nal * NALU_OVERHEAD * 8 as ::core::ffi::c_int
            - (*h).stat.frame.i_tex_bits - (*h).stat.frame.i_mv_bits;
        fdec_filter_row(h, (*h).i_threadslice_end, 0 as ::core::ffi::c_int);
        if (*h).param.b_sliced_threads != 0 {
            x264_10_threadslice_cond_broadcast(h, 1 as ::core::ffi::c_int);
            let mut mb_y: ::core::ffi::c_int = (*h).i_threadslice_start;
            while mb_y <= (*h).i_threadslice_end {
                fdec_filter_row(h, mb_y, 1 as ::core::ffi::c_int);
                mb_y += 1;
            }
            x264_10_threadslice_cond_broadcast(h, 2 as ::core::ffi::c_int);
            if (*h).i_thread_idx > 0 as ::core::ffi::c_int {
                x264_10_threadslice_cond_wait(
                    (*h).thread[((*h).i_thread_idx - 1 as ::core::ffi::c_int) as usize],
                    2 as ::core::ffi::c_int,
                );
                fdec_filter_row(
                    h,
                    (*h).i_threadslice_start
                        + ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff),
                    2 as ::core::ffi::c_int,
                );
            }
        }
        if (*(*h).fdec).mb_info_free.is_some()
            && ((*h).param.b_sliced_threads == 0
                || (*h).i_thread_idx == (*h).param.i_threads - 1 as ::core::ffi::c_int)
        {
            (*(*h).fdec)
                .mb_info_free
                .expect(
                    "non-null function pointer",
                )((*(*h).fdec).mb_info as *mut ::core::ffi::c_void);
            (*(*h).fdec).mb_info = 0 as *mut uint8_t;
            (*(*h).fdec).mb_info_free = None;
        }
    }
    return 0 as intptr_t;
}
#[c2rust::src_loc = "2770:9"]
pub const BS_BAK_SLICE_MAX_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[c2rust::src_loc = "2771:9"]
pub const BS_BAK_CAVLC_OVERFLOW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[c2rust::src_loc = "2772:9"]
pub const BS_BAK_SLICE_MIN_MBS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[c2rust::src_loc = "2773:9"]
pub const BS_BAK_ROW_VBV: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[c2rust::src_loc = "3132:1"]
unsafe extern "C" fn thread_sync_context(mut dst: *mut x264_t, mut src: *mut x264_t) {
    if dst == src {
        return;
    }
    let mut f: *mut *mut x264_frame_t = (*src).frames.reference.as_mut_ptr();
    while !(*f).is_null() {
        (**f).i_reference_count += 1;
        f = f.offset(1);
    }
    let mut f_0: *mut *mut x264_frame_t = (*dst).frames.reference.as_mut_ptr();
    while !(*f_0).is_null() {
        x264_10_frame_push_unused(src, *f_0);
        f_0 = f_0.offset(1);
    }
    (*(*src).fdec).i_reference_count += 1;
    x264_10_frame_push_unused(src, (*dst).fdec);
    memcpy(
        &mut (*dst).i_frame as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        &mut (*src).i_frame as *mut ::core::ffi::c_int as *const ::core::ffi::c_void,
        (28560 as size_t).wrapping_sub(2420 as size_t),
    );
    (*dst).param = (*src).param;
    (*dst).stat = (*src).stat;
    (*dst).pixf = (*src).pixf;
    (*dst).reconfig = (*src).reconfig;
}
#[c2rust::src_loc = "3153:1"]
unsafe extern "C" fn thread_sync_stat(mut dst: *mut x264_t, mut src: *mut x264_t) {
    if dst != src {
        memcpy(
            &mut (*dst).stat as *mut C2RustUnnamed_6 as *mut ::core::ffi::c_void,
            &mut (*src).stat as *mut C2RustUnnamed_6 as *const ::core::ffi::c_void,
            (51424 as size_t).wrapping_sub(48840 as size_t),
        );
    }
}
#[c2rust::src_loc = "3159:1"]
unsafe extern "C" fn slices_write(mut h: *mut x264_t) -> *mut ::core::ffi::c_void {
    let mut current_block: u64;
    let mut i_slice_num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last_thread_mb: ::core::ffi::c_int = (*h).sh.i_last_mb;
    let mut round_bias: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
        0 as ::core::ffi::c_int
    } else {
        (*h).param.i_slice_count / 2 as ::core::ffi::c_int
    };
    memset(
        &mut (*h).stat.frame as *mut x264_frame_stat_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<x264_frame_stat_t>() as size_t,
    );
    (*h).mb.b_reencode_mb = 0 as ::core::ffi::c_int;
    loop {
        if !((*h).sh.i_first_mb + (*h).sh.b_mbaff * (*h).mb.i_mb_stride
            <= last_thread_mb)
        {
            current_block = 5634871135123216486;
            break;
        }
        (*h).sh.i_last_mb = last_thread_mb;
        if i_slice_num == 0 || x264_10_frame_new_slice(h, (*h).fdec) == 0 {
            if (*h).param.i_slice_max_mbs != 0 {
                if (*h).sh.b_mbaff != 0 {
                    let mut last_mbaff: ::core::ffi::c_int = 2 as ::core::ffi::c_int
                        * ((*h).sh.i_first_mb % (*h).mb.i_mb_width)
                        + (*h).mb.i_mb_width * ((*h).sh.i_first_mb / (*h).mb.i_mb_width)
                        + (*h).param.i_slice_max_mbs - 1 as ::core::ffi::c_int;
                    let mut last_x: ::core::ffi::c_int = last_mbaff
                        % (2 as ::core::ffi::c_int * (*h).mb.i_mb_width)
                        / 2 as ::core::ffi::c_int;
                    let mut last_y: ::core::ffi::c_int = last_mbaff
                        / (2 as ::core::ffi::c_int * (*h).mb.i_mb_width)
                        * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                    (*h).sh.i_last_mb = last_x + (*h).mb.i_mb_stride * last_y;
                } else {
                    (*h).sh.i_last_mb = (*h).sh.i_first_mb + (*h).param.i_slice_max_mbs
                        - 1 as ::core::ffi::c_int;
                    if (*h).sh.i_last_mb < last_thread_mb
                        && last_thread_mb - (*h).sh.i_last_mb
                            < (*h).param.i_slice_min_mbs
                    {
                        (*h).sh.i_last_mb = last_thread_mb - (*h).param.i_slice_min_mbs;
                    }
                }
                i_slice_num += 1;
            } else if (*h).param.i_slice_count != 0 && (*h).param.b_sliced_threads == 0 {
                let mut height: ::core::ffi::c_int = (*h).mb.i_mb_height
                    >> (*h).param.b_interlaced;
                let mut width: ::core::ffi::c_int = (*h).mb.i_mb_width
                    << (*h).param.b_interlaced;
                i_slice_num += 1;
                (*h).sh.i_last_mb = (height * i_slice_num + round_bias)
                    / (*h).param.i_slice_count * width - 1 as ::core::ffi::c_int;
            }
        }
        (*h).sh.i_last_mb = if (*h).sh.i_last_mb < last_thread_mb {
            (*h).sh.i_last_mb
        } else {
            last_thread_mb
        };
        if slice_write(h) != 0 {
            current_block = 15078433425792239898;
            break;
        }
        (*h).sh.i_first_mb = (*h).sh.i_last_mb + 1 as ::core::ffi::c_int;
        if (*h).sh.b_mbaff != 0 && (*h).sh.i_first_mb % (*h).mb.i_mb_width != 0 {
            (*h).sh.i_first_mb -= (*h).mb.i_mb_stride;
        }
    }
    match current_block {
        5634871135123216486 => return 0 as *mut ::core::ffi::c_void,
        _ => {
            if (*h).param.b_sliced_threads != 0 {
                x264_10_threadslice_cond_broadcast(h, 2 as ::core::ffi::c_int);
            }
            return -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
        }
    };
}
#[c2rust::src_loc = "3219:1"]
unsafe extern "C" fn threaded_slices_write(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut round_bias: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
        0 as ::core::ffi::c_int
    } else {
        (*h).param.i_slice_count / 2 as ::core::ffi::c_int
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).param.i_threads {
        let mut t: *mut x264_t = (*h).thread[i as usize];
        if i != 0 {
            (*t).param = (*h).param;
            memcpy(
                &mut (*t).i_frame as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                &mut (*h).i_frame as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_void,
                (48832 as size_t).wrapping_sub(2420 as size_t),
            );
        }
        let mut height: ::core::ffi::c_int = (*h).mb.i_mb_height
            >> (*h).param.b_interlaced;
        (*t).i_threadslice_start = (height * i + round_bias) / (*h).param.i_threads
            << (*h).param.b_interlaced;
        (*t).i_threadslice_end = (height * (i + 1 as ::core::ffi::c_int) + round_bias)
            / (*h).param.i_threads << (*h).param.b_interlaced;
        (*t).sh.i_first_mb = (*t).i_threadslice_start * (*h).mb.i_mb_width;
        (*t).sh.i_last_mb = (*t).i_threadslice_end * (*h).mb.i_mb_width
            - 1 as ::core::ffi::c_int;
        i += 1;
    }
    x264_10_analyse_weight_frame(
        h,
        (*h).mb.i_mb_height * 16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
    );
    x264_10_threads_distribute_ratecontrol(h);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*h).param.i_threads {
        (*(*h).thread[i_0 as usize]).i_thread_idx = i_0;
        (*(*h).thread[i_0 as usize]).b_thread_active = 1 as ::core::ffi::c_int;
        x264_10_threadslice_cond_broadcast(
            (*h).thread[i_0 as usize],
            0 as ::core::ffi::c_int,
        );
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*h).param.i_threads {
        x264_10_threadpool_run(
            (*h).threadpool,
            ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
                >,
            >(
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void,
                    >,
                    *mut ::core::ffi::c_void,
                >(
                    Some(
                        slices_write
                            as unsafe extern "C" fn(
                                *mut x264_t,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                ),
            ),
            (*h).thread[i_1 as usize] as *mut ::core::ffi::c_void,
        );
        i_1 += 1;
    }
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < (*h).param.i_threads {
        x264_10_threadslice_cond_wait(
            (*h).thread[i_2 as usize],
            1 as ::core::ffi::c_int,
        );
        i_2 += 1;
    }
    x264_10_threads_merge_ratecontrol(h);
    let mut i_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i_3 < (*h).param.i_threads {
        let mut t_0: *mut x264_t = (*h).thread[i_3 as usize];
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*t_0).out.i_nal {
            *(*h).out.nal.offset((*h).out.i_nal as isize) = *(*t_0)
                .out
                .nal
                .offset(j as isize);
            (*h).out.i_nal += 1;
            nal_check_buffer(h);
            j += 1;
        }
        let mut j_0: size_t = 0 as size_t;
        while j_0
            < (52112 as usize)
                .wrapping_sub(51424 as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
        {
            *(&mut (*h).stat.frame as *mut x264_frame_stat_t as *mut ::core::ffi::c_int)
                .offset(j_0 as isize)
                += *(&mut (*t_0).stat.frame as *mut x264_frame_stat_t
                    as *mut ::core::ffi::c_int)
                    .offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_1 < 3 as ::core::ffi::c_int {
            (*h).stat.frame.i_ssd[j_1 as usize] += (*t_0).stat.frame.i_ssd[j_1 as usize];
            j_1 += 1;
        }
        (*h).stat.frame.f_ssim += (*t_0).stat.frame.f_ssim;
        (*h).stat.frame.i_ssim_cnt += (*t_0).stat.frame.i_ssim_cnt;
        i_3 += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3280:1"]
pub unsafe extern "C" fn x264_10_encoder_intra_refresh(mut h: *mut x264_t) {
    h = (*h).thread[(*h).i_thread_phase as usize];
    (*h).b_queued_intra_refresh = 1 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3286:1"]
pub unsafe extern "C" fn x264_10_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> ::core::ffi::c_int {
    if (*h).param.i_bframe != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"x264_encoder_invalidate_reference is not supported with B-frames enabled\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).param.b_intra_refresh != 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"x264_encoder_invalidate_reference is not supported with intra refresh enabled\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    h = (*h).thread[(*h).i_thread_phase as usize];
    if pts >= (*h).i_last_idr_pts {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if pts <= (*(*h).frames.reference[i as usize]).i_pts {
                (*(*h).frames.reference[i as usize]).b_corrupt = 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if pts <= (*(*h).fdec).i_pts {
            (*(*h).fdec).b_corrupt = 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "3323:1"]
pub unsafe extern "C" fn x264_10_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut thread_current: *mut x264_t = 0 as *mut x264_t;
    let mut thread_prev: *mut x264_t = 0 as *mut x264_t;
    let mut thread_oldest: *mut x264_t = 0 as *mut x264_t;
    let mut i_nal_type: ::core::ffi::c_int = 0;
    let mut i_nal_ref_idc: ::core::ffi::c_int = 0;
    let mut i_global_qp: ::core::ffi::c_int = 0;
    let mut overhead: ::core::ffi::c_int = NALU_OVERHEAD;
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        thread_prev = (*h).thread[(*h).i_thread_phase as usize];
        (*h).i_thread_phase = ((*h).i_thread_phase + 1 as ::core::ffi::c_int)
            % (*h).i_thread_frames;
        thread_current = (*h).thread[(*h).i_thread_phase as usize];
        thread_oldest = (*h)
            .thread[(((*h).i_thread_phase + 1 as ::core::ffi::c_int)
            % (*h).i_thread_frames) as usize];
        thread_sync_context(thread_current, thread_prev);
        x264_10_thread_sync_ratecontrol(thread_current, thread_prev, thread_oldest);
        h = thread_current;
    } else {
        thread_oldest = h;
        thread_current = thread_oldest;
    }
    (*h).i_cpb_delay_pir_offset = (*h).i_cpb_delay_pir_offset_next;
    *pi_nal = 0 as ::core::ffi::c_int;
    *pp_nal = 0 as *mut x264_nal_t;
    if !pic_in.is_null() {
        if (*(*h).lookahead).b_exit_thread != 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"lookahead thread is already stopped\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut fenc: *mut x264_frame_t = x264_10_frame_pop_unused(
            h,
            0 as ::core::ffi::c_int,
        );
        if fenc.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if x264_10_frame_copy_picture(h, fenc, pic_in) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        if (*h).param.i_width != 16 as ::core::ffi::c_int * (*h).mb.i_mb_width
            || (*h).param.i_height != 16 as ::core::ffi::c_int * (*h).mb.i_mb_height
        {
            x264_10_frame_expand_border_mod16(h, fenc);
        }
        let fresh8 = (*h).frames.i_input;
        (*h).frames.i_input = (*h).frames.i_input + 1;
        (*fenc).i_frame = fresh8;
        if (*fenc).i_frame == 0 as ::core::ffi::c_int {
            (*h).frames.i_first_pts = (*fenc).i_pts;
        }
        if (*h).frames.i_bframe_delay != 0
            && (*fenc).i_frame == (*h).frames.i_bframe_delay
        {
            (*h).frames.i_bframe_delay_time = (*fenc).i_pts - (*h).frames.i_first_pts;
        }
        if (*h).param.b_vfr_input != 0 && (*fenc).i_pts <= (*h).frames.i_largest_pts {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"non-strictly-monotonic PTS\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*h).frames.i_second_largest_pts = (*h).frames.i_largest_pts;
        (*h).frames.i_largest_pts = (*fenc).i_pts;
        if (*fenc).i_pic_struct < PIC_STRUCT_AUTO as ::core::ffi::c_int
            || (*fenc).i_pic_struct > PIC_STRUCT_TRIPLE as ::core::ffi::c_int
        {
            (*fenc).i_pic_struct = PIC_STRUCT_AUTO as ::core::ffi::c_int;
        }
        if (*fenc).i_pic_struct == PIC_STRUCT_AUTO as ::core::ffi::c_int {
            let mut b_interlaced: ::core::ffi::c_int = if !(*fenc).param.is_null() {
                (*(*fenc).param).b_interlaced
            } else {
                (*h).param.b_interlaced
            };
            if b_interlaced != 0 {
                let mut b_tff: ::core::ffi::c_int = if !(*fenc).param.is_null() {
                    (*(*fenc).param).b_tff
                } else {
                    (*h).param.b_tff
                };
                (*fenc).i_pic_struct = if b_tff != 0 {
                    PIC_STRUCT_TOP_BOTTOM as ::core::ffi::c_int
                } else {
                    PIC_STRUCT_BOTTOM_TOP as ::core::ffi::c_int
                };
            } else {
                (*fenc).i_pic_struct = PIC_STRUCT_PROGRESSIVE as ::core::ffi::c_int;
            }
        }
        if (*h).param.rc.b_mb_tree != 0 && (*h).param.rc.b_stat_read != 0 {
            if x264_10_macroblock_tree_read(h, fenc, (*pic_in).prop.quant_offsets) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            x264_10_adaptive_quant_frame(h, fenc, (*pic_in).prop.quant_offsets);
        }
        if (*pic_in).prop.quant_offsets_free.is_some() {
            (*pic_in)
                .prop
                .quant_offsets_free
                .expect(
                    "non-null function pointer",
                )((*pic_in).prop.quant_offsets as *mut ::core::ffi::c_void);
        }
        if (*h).frames.b_have_lowres != 0 {
            x264_10_frame_init_lowres(h, fenc);
        }
        x264_10_lookahead_put_frame(h, fenc);
        if (*h).frames.i_input
            <= (*h).frames.i_delay + 1 as ::core::ffi::c_int - (*h).i_thread_frames
        {
            (*pic_out).i_type = X264_TYPE_AUTO;
            return 0 as ::core::ffi::c_int;
        }
    } else {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        ::core::ptr::write_volatile(
            &mut (*(*h).lookahead).b_exit_thread as *mut uint8_t,
            1 as uint8_t,
        );
        pthread_cond_broadcast(&mut (*(*h).lookahead).ifbuf.cv_fill);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    }
    (*h).i_frame += 1;
    if (*(*h).frames.current.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        x264_10_lookahead_get_frames(h);
    }
    if (*(*h).frames.current.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && x264_10_lookahead_is_empty(h) != 0
    {
        return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
    }
    (*h).fenc = x264_10_frame_shift((*h).frames.current);
    if (*h).param.b_sliced_threads != 0 {
        if threadpool_wait_all(h) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*h).i_frame == 0 as ::core::ffi::c_int {
        (*h).i_reordered_pts_delay = (*(*h).fenc).i_reordered_pts;
    }
    if (*h).reconfig != 0 {
        x264_10_encoder_reconfig_apply(h, &mut (*(*h).reconfig_h).param);
        (*h).reconfig = 0 as ::core::ffi::c_int;
    }
    if !(*(*h).fenc).param.is_null() {
        x264_10_encoder_reconfig_apply(h, (*(*h).fenc).param);
        if (*(*(*h).fenc).param).param_free.is_some() {
            x264_param_cleanup((*(*h).fenc).param);
            (*(*(*h).fenc).param)
                .param_free
                .expect(
                    "non-null function pointer",
                )((*(*h).fenc).param as *mut ::core::ffi::c_void);
            (*(*h).fenc).param = 0 as *mut x264_param_t;
        }
    }
    x264_10_ratecontrol_zone_init(h);
    if reference_update(h) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*(*h).fdec).i_lines_completed = -(1 as ::core::ffi::c_int);
    if !((*(*h).fenc).i_type == X264_TYPE_I || (*(*h).fenc).i_type == X264_TYPE_IDR
        || (*(*h).fenc).i_type == X264_TYPE_KEYFRAME)
    {
        let mut valid_refs_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*h).frames.reference[i as usize].is_null() {
            if (*(*h).frames.reference[i as usize]).b_corrupt == 0 {
                valid_refs_left += 1;
            }
            i += 1;
        }
        if valid_refs_left == 0 {
            (*(*h).fenc).b_keyframe = 1 as ::core::ffi::c_int;
            (*(*h).fenc).i_type = X264_TYPE_IDR;
        }
    }
    if (*(*h).fenc).b_keyframe != 0 {
        (*h).frames.i_last_keyframe = (*(*h).fenc).i_frame;
        if (*(*h).fenc).i_type == X264_TYPE_IDR {
            (*h).i_frame_num = 0 as ::core::ffi::c_int;
            (*h).frames.i_last_idr = (*(*h).fenc).i_frame;
        }
    }
    (*h).sh.i_mmco_remove_from_end = 0 as ::core::ffi::c_int;
    (*h).sh.i_mmco_command_count = (*h).sh.i_mmco_remove_from_end;
    (*h).b_ref_reorder[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    (*h).b_ref_reorder[0 as ::core::ffi::c_int as usize] = (*h)
        .b_ref_reorder[1 as ::core::ffi::c_int as usize];
    (*(*h).fenc).i_poc = 2 as ::core::ffi::c_int
        * ((*(*h).fenc).i_frame
            - (if (*h).frames.i_last_idr > 0 as ::core::ffi::c_int {
                (*h).frames.i_last_idr
            } else {
                0 as ::core::ffi::c_int
            }));
    (*(*h).fdec).i_poc = (*(*h).fenc).i_poc;
    if (*(*h).fenc).i_type == X264_TYPE_IDR {
        i_nal_type = NAL_SLICE_IDR as ::core::ffi::c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGHEST as ::core::ffi::c_int;
        (*h).sh.i_type = SLICE_TYPE_I as ::core::ffi::c_int;
        reference_reset(h);
        (*h).frames.i_poc_last_open_gop = -(1 as ::core::ffi::c_int);
    } else if (*(*h).fenc).i_type == X264_TYPE_I {
        i_nal_type = NAL_SLICE as ::core::ffi::c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGH as ::core::ffi::c_int;
        (*h).sh.i_type = SLICE_TYPE_I as ::core::ffi::c_int;
        reference_hierarchy_reset(h);
        if (*h).param.b_open_gop != 0 {
            (*h).frames.i_poc_last_open_gop = if (*(*h).fenc).b_keyframe != 0 {
                (*(*h).fenc).i_poc
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    } else if (*(*h).fenc).i_type == X264_TYPE_P {
        i_nal_type = NAL_SLICE as ::core::ffi::c_int;
        i_nal_ref_idc = NAL_PRIORITY_HIGH as ::core::ffi::c_int;
        (*h).sh.i_type = SLICE_TYPE_P as ::core::ffi::c_int;
        reference_hierarchy_reset(h);
        (*h).frames.i_poc_last_open_gop = -(1 as ::core::ffi::c_int);
    } else if (*(*h).fenc).i_type == X264_TYPE_BREF {
        i_nal_type = NAL_SLICE as ::core::ffi::c_int;
        i_nal_ref_idc = if (*h).param.i_bframe_pyramid == X264_B_PYRAMID_STRICT {
            NAL_PRIORITY_LOW as ::core::ffi::c_int
        } else {
            NAL_PRIORITY_HIGH as ::core::ffi::c_int
        };
        (*h).sh.i_type = SLICE_TYPE_B as ::core::ffi::c_int;
        reference_hierarchy_reset(h);
    } else {
        i_nal_type = NAL_SLICE as ::core::ffi::c_int;
        i_nal_ref_idc = NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int;
        (*h).sh.i_type = SLICE_TYPE_B as ::core::ffi::c_int;
    }
    (*(*h).fdec).i_type = (*(*h).fenc).i_type;
    (*(*h).fdec).i_frame = (*(*h).fenc).i_frame;
    (*(*h).fdec).b_kept_as_ref = (i_nal_ref_idc
        != NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int
        && (*h).param.i_keyint_max > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*(*h).fenc).b_kept_as_ref = (*(*h).fdec).b_kept_as_ref;
    (*(*h).fdec).mb_info = (*(*h).fenc).mb_info;
    (*(*h).fdec).mb_info_free = (*(*h).fenc).mb_info_free;
    (*(*h).fenc).mb_info = 0 as *mut uint8_t;
    (*(*h).fenc).mb_info_free = None;
    (*(*h).fdec).i_pts = (*(*h).fenc).i_pts;
    if (*h).frames.i_bframe_delay != 0 {
        let mut prev_reordered_pts: *mut int64_t = (*thread_current)
            .frames
            .i_prev_reordered_pts
            .as_mut_ptr();
        (*(*h).fdec).i_dts = if (*h).i_frame > (*h).frames.i_bframe_delay {
            *prev_reordered_pts
                .offset(
                    (((*h).i_frame - (*h).frames.i_bframe_delay)
                        % (*h).frames.i_bframe_delay) as isize,
                )
        } else {
            (*(*h).fenc).i_reordered_pts - (*h).frames.i_bframe_delay_time
        };
        *prev_reordered_pts
            .offset(((*h).i_frame % (*h).frames.i_bframe_delay) as isize) = (*(*h).fenc)
            .i_reordered_pts;
    } else {
        (*(*h).fdec).i_dts = (*(*h).fenc).i_reordered_pts;
    }
    if (*(*h).fenc).i_type == X264_TYPE_IDR {
        (*h).i_last_idr_pts = (*(*h).fdec).i_pts;
    }
    reference_build_list(h, (*(*h).fdec).i_poc);
    if (*h).param.b_sliced_threads != 0 {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).param.i_threads {
            bs_init(
                &mut (**(*h).thread.as_mut_ptr().offset(i_0 as isize)).out.bs,
                (*(*h).thread[i_0 as usize]).out.p_bitstream as *mut ::core::ffi::c_void,
                (*(*h).thread[i_0 as usize]).out.i_bitstream,
            );
            (*(*h).thread[i_0 as usize]).out.i_nal = 0 as ::core::ffi::c_int;
            i_0 += 1;
        }
    } else {
        bs_init(
            &mut (*h).out.bs,
            (*h).out.p_bitstream as *mut ::core::ffi::c_void,
            (*h).out.i_bitstream,
        );
        (*h).out.i_nal = 0 as ::core::ffi::c_int;
    }
    if (*h).param.b_aud != 0 {
        let mut pic_type: ::core::ffi::c_int = 0;
        if (*h).sh.i_type == SLICE_TYPE_I as ::core::ffi::c_int {
            pic_type = 0 as ::core::ffi::c_int;
        } else if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
            pic_type = 1 as ::core::ffi::c_int;
        } else if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
            pic_type = 2 as ::core::ffi::c_int;
        } else {
            pic_type = 7 as ::core::ffi::c_int;
        }
        nal_start(
            h,
            NAL_AUD as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        bs_write(&mut (*h).out.bs, 3 as ::core::ffi::c_int, pic_type as uint32_t);
        bs_rbsp_trailing(&mut (*h).out.bs);
        bs_flush(&mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload + NALU_OVERHEAD;
    }
    (*h).i_nal_type = i_nal_type;
    (*h).i_nal_ref_idc = i_nal_ref_idc;
    if (*h).param.b_intra_refresh != 0 {
        if (*(*h).fenc).i_type == X264_TYPE_I || (*(*h).fenc).i_type == X264_TYPE_IDR
            || (*(*h).fenc).i_type == X264_TYPE_KEYFRAME
        {
            (*(*h).fdec).i_frames_since_pir = 0 as ::core::ffi::c_int;
            (*h).b_queued_intra_refresh = 0 as ::core::ffi::c_int;
            (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
        } else if (*(*h).fenc).i_type == X264_TYPE_P {
            let mut pocdiff: ::core::ffi::c_int = ((*(*h).fdec).i_poc
                - (*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .i_poc) / 2 as ::core::ffi::c_int;
            let mut increment: ::core::ffi::c_float = if ((*h).mb.i_mb_width
                as ::core::ffi::c_float
                - 1 as ::core::ffi::c_int as ::core::ffi::c_float)
                / (*h).param.i_keyint_max as ::core::ffi::c_float
                > 1 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                ((*h).mb.i_mb_width as ::core::ffi::c_float
                    - 1 as ::core::ffi::c_int as ::core::ffi::c_float)
                    / (*h).param.i_keyint_max as ::core::ffi::c_float
            } else {
                1 as ::core::ffi::c_int as ::core::ffi::c_float
            };
            (*(*h).fdec).f_pir_position = (*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .f_pir_position;
            (*(*h).fdec).i_frames_since_pir = (*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_frames_since_pir + pocdiff;
            if (*(*h).fdec).i_frames_since_pir >= (*h).param.i_keyint_max
                || (*h).b_queued_intra_refresh != 0
                    && (*(*h).fdec).f_pir_position as ::core::ffi::c_double + 0.5f64
                        >= (*h).mb.i_mb_width as ::core::ffi::c_double
            {
                (*(*h).fdec).f_pir_position = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_float;
                (*(*h).fdec).i_frames_since_pir = 0 as ::core::ffi::c_int;
                (*h).b_queued_intra_refresh = 0 as ::core::ffi::c_int;
                (*(*h).fenc).b_keyframe = 1 as ::core::ffi::c_int;
            }
            (*(*h).fdec).i_pir_start_col = ((*(*h).fdec).f_pir_position
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
            (*(*h).fdec).f_pir_position += increment * pocdiff as ::core::ffi::c_float;
            (*(*h).fdec).i_pir_end_col = ((*(*h).fdec).f_pir_position
                as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
            if (*(*h).fdec).i_pir_end_col >= (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
            {
                (*(*h).fdec).f_pir_position = (*h).mb.i_mb_width as ::core::ffi::c_float;
                (*(*h).fdec).i_pir_end_col = (*h).mb.i_mb_width
                    - 1 as ::core::ffi::c_int;
            }
        }
    }
    if (*(*h).fenc).b_keyframe != 0 {
        if (*h).param.b_repeat_headers != 0 {
            nal_start(
                h,
                NAL_SPS as ::core::ffi::c_int,
                NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
            );
            x264_10_sps_write(&mut (*h).out.bs, (*h).sps.as_mut_ptr());
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.i_avcintra_class != 0 {
                (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding = 256 as ::core::ffi::c_int
                    - bs_pos(&mut (*h).out.bs) / 8 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * NALU_OVERHEAD;
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                        .i_padding + NALU_OVERHEAD;
            nal_start(
                h,
                NAL_PPS as ::core::ffi::c_int,
                NAL_PRIORITY_HIGHEST as ::core::ffi::c_int,
            );
            x264_10_pps_write(
                &mut (*h).out.bs,
                (*h).sps.as_mut_ptr(),
                (*h).pps.as_mut_ptr(),
            );
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).param.i_avcintra_class != 0 {
                let mut total_len: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
                if (*h).param.i_avcintra_flavor == X264_AVCINTRA_FLAVOR_SONY {
                    total_len
                        += if (*h).param.i_height >= 1080 as ::core::ffi::c_int {
                            18 as ::core::ffi::c_int * 512 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int * 512 as ::core::ffi::c_int
                        };
                }
                (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding = total_len
                    - (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                        .i_payload - NALU_OVERHEAD;
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (*(*h)
                        .out
                        .nal
                        .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                        .i_padding + NALU_OVERHEAD;
        }
        if (*h).i_thread_frames == 1 as ::core::ffi::c_int
            && (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
        {
            x264_10_hrd_fullness(h);
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_sei_buffering_period_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*(*h).fenc).extra_sei.num_payloads {
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_sei_write(
            &mut (*h).out.bs,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_size,
            (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload_type,
        );
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        if (*(*h).fenc).extra_sei.sei_free.is_some() {
            (*(*h).fenc)
                .extra_sei
                .sei_free
                .expect(
                    "non-null function pointer",
                )(
                (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize)).payload
                    as *mut ::core::ffi::c_void,
            );
            let ref mut fresh9 = (*(*(*h).fenc).extra_sei.payloads.offset(i_1 as isize))
                .payload;
            *fresh9 = 0 as *mut uint8_t;
        }
        i_1 += 1;
    }
    if (*(*h).fenc).extra_sei.sei_free.is_some() {
        (*(*h).fenc)
            .extra_sei
            .sei_free
            .expect(
                "non-null function pointer",
            )((*(*h).fenc).extra_sei.payloads as *mut ::core::ffi::c_void);
        (*(*h).fenc).extra_sei.payloads = 0 as *mut x264_sei_payload_t;
        (*(*h).fenc).extra_sei.sei_free = None;
    }
    if (*(*h).fenc).b_keyframe != 0 {
        if (*h).param.b_repeat_headers != 0
            && (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int
            && (*h).param.i_avcintra_class == 0
        {
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            if x264_10_sei_version_write(h, &mut (*h).out.bs) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
        if (*(*h).fenc).i_type != X264_TYPE_IDR {
            let mut time_to_recovery: ::core::ffi::c_int = if (*h).param.b_open_gop != 0
            {
                0 as ::core::ffi::c_int
            } else {
                (if ((*h).mb.i_mb_width - 1 as ::core::ffi::c_int)
                    < (*h).param.i_keyint_max
                {
                    (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
                } else {
                    (*h).param.i_keyint_max
                }) + (*h).param.i_bframe - 1 as ::core::ffi::c_int
            };
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_sei_recovery_point_write(h, &mut (*h).out.bs, time_to_recovery);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
        if (*h).param.mastering_display.b_mastering_display != 0 {
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_sei_mastering_display_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
        if (*h).param.content_light_level.b_cll != 0 {
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_sei_content_light_level_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
        if (*h).param.i_alternative_transfer != 2 as ::core::ffi::c_int {
            nal_start(
                h,
                NAL_SEI as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_sei_alternative_transfer_write(h, &mut (*h).out.bs);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            overhead
                += (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_payload
                    + (NALU_OVERHEAD
                        - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                            && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int);
        }
    }
    if (*h).param.i_frame_packing >= 0 as ::core::ffi::c_int
        && ((*(*h).fenc).b_keyframe != 0
            || (*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
    {
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_sei_frame_packing_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
    }
    if (*(*h).sps.as_mut_ptr()).vui.b_pic_struct_present != 0
        || (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
    {
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_sei_pic_timing_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
    }
    if !((*(*h).fenc).i_type == X264_TYPE_B || (*(*h).fenc).i_type == X264_TYPE_BREF)
        && (*h).b_sh_backup != 0
    {
        (*h).b_sh_backup = 0 as ::core::ffi::c_int;
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_sei_dec_ref_pic_marking_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
    }
    if (*(*h).fenc).b_keyframe != 0 && (*h).param.b_intra_refresh != 0 {
        (*h).i_cpb_delay_pir_offset_next = (*(*h).fenc).i_cpb_delay;
    }
    if (*h).param.i_avcintra_class != 0
        && (*h).param.i_avcintra_flavor != X264_AVCINTRA_FLAVOR_SONY
    {
        nal_start(
            h,
            NAL_FILLER as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_filler_write(h, &mut (*h).out.bs, 0 as ::core::ffi::c_int);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload + NALU_OVERHEAD;
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        if x264_10_sei_avcintra_umid_write(h, &mut (*h).out.bs) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
        let mut unpadded_len: ::core::ffi::c_int = 0;
        let mut total_len_0: ::core::ffi::c_int = 0;
        if (*h).param.i_height == 1080 as ::core::ffi::c_int {
            unpadded_len = 5780 as ::core::ffi::c_int;
            total_len_0 = 17 as ::core::ffi::c_int * 512 as ::core::ffi::c_int;
        } else {
            unpadded_len = 2900 as ::core::ffi::c_int;
            total_len_0 = 9 as ::core::ffi::c_int * 512 as ::core::ffi::c_int;
        }
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        if x264_10_sei_avcintra_vanc_write(h, &mut (*h).out.bs, unpadded_len)
            < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        (*(*h).out.nal.offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
            .i_padding = total_len_0
            - (*(*h).out.nal.offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
            - (NALU_OVERHEAD
                - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                    && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int);
        overhead
            += (*(*h)
                .out
                .nal
                .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                .i_payload
                + (*(*h)
                    .out
                    .nal
                    .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize))
                    .i_padding
                + (NALU_OVERHEAD
                    - ((*h).param.b_annexb != 0 && (*h).param.i_avcintra_class == 0
                        && (*h).out.i_nal - 1 as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int);
    }
    x264_10_ratecontrol_start(
        h,
        (*(*h).fenc).i_qpplus1,
        overhead * 8 as ::core::ffi::c_int,
    );
    i_global_qp = x264_10_ratecontrol_qp(h);
    (*(*h).fdec).i_qpplus1 = i_global_qp + 1 as ::core::ffi::c_int;
    (*pic_out).i_qpplus1 = (*(*h).fdec).i_qpplus1;
    if (*h).param.rc.b_stat_read != 0
        && (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int
    {
        x264_10_reference_build_list_optimal(h);
        reference_check_reorder(h);
    }
    if (*h).i_ref[0 as ::core::ffi::c_int as usize] != 0 {
        (*(*h).fdec).i_poc_l0ref0 = (*(*h)
            .fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .i_poc;
    }
    slice_init(h, i_nal_type, i_global_qp);
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        x264_10_macroblock_bipred_init(h);
    }
    weighted_pred_init(h);
    if i_nal_ref_idc != NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int {
        (*h).i_frame_num += 1;
    }
    (*h).i_threadslice_start = 0 as ::core::ffi::c_int;
    (*h).i_threadslice_end = (*h).mb.i_mb_height;
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        x264_10_threadpool_run(
            (*h).threadpool,
            ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
                >,
            >(
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(*mut x264_t) -> *mut ::core::ffi::c_void,
                    >,
                    *mut ::core::ffi::c_void,
                >(
                    Some(
                        slices_write
                            as unsafe extern "C" fn(
                                *mut x264_t,
                            ) -> *mut ::core::ffi::c_void,
                    ),
                ),
            ),
            h as *mut ::core::ffi::c_void,
        );
        (*h).b_thread_active = 1 as ::core::ffi::c_int;
    } else if (*h).param.b_sliced_threads != 0 {
        if threaded_slices_write(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
    } else if slices_write(h) as intptr_t != 0 {
        return -(1 as ::core::ffi::c_int)
    }
    return encoder_frame_end(thread_oldest, thread_current, pp_nal, pi_nal, pic_out);
}
#[c2rust::src_loc = "3904:1"]
unsafe extern "C" fn encoder_frame_end(
    mut h: *mut x264_t,
    mut thread_current: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_out: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut psz_message: [::core::ffi::c_char; 80] = [0; 80];
    if (*h).param.b_sliced_threads == 0 && (*h).b_thread_active != 0 {
        (*h).b_thread_active = 0 as ::core::ffi::c_int;
        if x264_10_threadpool_wait((*h).threadpool, h as *mut ::core::ffi::c_void)
            as intptr_t != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*h).out.i_nal == 0 {
        (*pic_out).i_type = X264_TYPE_AUTO;
        return 0 as ::core::ffi::c_int;
    }
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int && (*(*h).fenc).b_keyframe != 0
        && (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0
    {
        x264_10_hrd_fullness(h);
        nal_start(
            h,
            NAL_SEI as ::core::ffi::c_int,
            NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
        );
        x264_10_sei_buffering_period_write(h, &mut (*h).out.bs);
        if nal_end(h) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (*(*h).out.nal.offset(idx as isize)).i_type
            == NAL_AUD as ::core::ffi::c_int
            || (*(*h).out.nal.offset(idx as isize)).i_type
                == NAL_SPS as ::core::ffi::c_int
            || (*(*h).out.nal.offset(idx as isize)).i_type
                == NAL_PPS as ::core::ffi::c_int
        {
            idx += 1;
        }
        let mut nal_tmp: x264_nal_t = *(*h)
            .out
            .nal
            .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize);
        memmove(
            &mut *(*h).out.nal.offset((idx + 1 as ::core::ffi::c_int) as isize)
                as *mut x264_nal_t as *mut ::core::ffi::c_void,
            &mut *(*h).out.nal.offset(idx as isize) as *mut x264_nal_t
                as *const ::core::ffi::c_void,
            (((*h).out.i_nal - idx - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<x264_nal_t>() as size_t),
        );
        *(*h).out.nal.offset(idx as isize) = nal_tmp;
    }
    let mut frame_size: ::core::ffi::c_int = encoder_encapsulate_nals(
        h,
        0 as ::core::ffi::c_int,
    );
    if frame_size < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic_out).i_type = (*(*h).fenc).i_type;
    (*pic_out).b_keyframe = (*(*h).fenc).b_keyframe;
    (*pic_out).i_pic_struct = (*(*h).fenc).i_pic_struct;
    (*pic_out).i_pts = (*(*h).fdec).i_pts;
    (*pic_out).i_dts = (*(*h).fdec).i_dts;
    if (*pic_out).i_pts < (*pic_out).i_dts {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"invalid DTS: PTS is less than DTS\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*pic_out).opaque = (*(*h).fenc).opaque;
    (*pic_out).img.i_csp = (*(*h).fdec).i_csp;
    (*pic_out).img.i_csp |= X264_CSP_HIGH_DEPTH;
    (*pic_out).img.i_plane = (*(*h).fdec).i_plane;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pic_out).img.i_plane {
        (*pic_out).img.i_stride[i as usize] = (*(*h).fdec).i_stride[i as usize]
            * SIZEOF_PIXEL;
        (*pic_out).img.plane[i as usize] = (*(*h).fdec).plane[i as usize]
            as *mut uint8_t;
        i += 1;
    }
    x264_10_frame_push_unused(thread_current, (*h).fenc);
    let mut filler: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if x264_10_ratecontrol_end(h, frame_size * 8 as ::core::ffi::c_int, &mut filler)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic_out).hrd_timing = (*(*h).fenc).hrd_timing;
    (*pic_out).prop.f_crf_avg = (*(*h).fdec).f_crf_avg as ::core::ffi::c_double;
    if (*h).param.i_avcintra_class != 0 {
        if check_encapsulated_buffer(
            h,
            (*h).thread[0 as ::core::ffi::c_int as usize],
            (*h).out.i_nal,
            frame_size as int64_t,
            frame_size as int64_t + filler as int64_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        let mut nal: *mut x264_nal_t = &mut *(*h)
            .out
            .nal
            .offset(((*h).out.i_nal - 1 as ::core::ffi::c_int) as isize)
            as *mut x264_nal_t;
        memset(
            (*nal).p_payload.offset((*nal).i_payload as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            filler as size_t,
        );
        (*nal).i_payload += filler;
        (*nal).i_padding = filler;
        frame_size += filler;
        if (*h).param.b_annexb == 0 {
            let mut nal_data: *mut uint8_t = (*nal).p_payload;
            let mut chunk_size: ::core::ffi::c_int = (*nal).i_payload
                - 4 as ::core::ffi::c_int;
            *nal_data.offset(0 as ::core::ffi::c_int as isize) = (chunk_size
                >> 24 as ::core::ffi::c_int) as uint8_t;
            *nal_data.offset(1 as ::core::ffi::c_int as isize) = (chunk_size
                >> 16 as ::core::ffi::c_int) as uint8_t;
            *nal_data.offset(2 as ::core::ffi::c_int as isize) = (chunk_size
                >> 8 as ::core::ffi::c_int) as uint8_t;
            *nal_data.offset(3 as ::core::ffi::c_int as isize) = (chunk_size
                >> 0 as ::core::ffi::c_int) as uint8_t;
        }
    } else {
        while filler > 0 as ::core::ffi::c_int {
            let mut f: ::core::ffi::c_int = 0;
            let mut overhead: ::core::ffi::c_int = FILLER_OVERHEAD - (*h).param.b_annexb;
            if (*h).param.i_slice_max_size != 0 && filler > (*h).param.i_slice_max_size {
                let mut next_size: ::core::ffi::c_int = filler
                    - (*h).param.i_slice_max_size;
                let mut overflow: ::core::ffi::c_int = if overhead - next_size
                    > 0 as ::core::ffi::c_int
                {
                    overhead - next_size
                } else {
                    0 as ::core::ffi::c_int
                };
                f = (*h).param.i_slice_max_size - overhead - overflow;
            } else {
                f = if 0 as ::core::ffi::c_int > filler - overhead {
                    0 as ::core::ffi::c_int
                } else {
                    filler - overhead
                };
            }
            if bitstream_check_buffer_filler(h, f) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            nal_start(
                h,
                NAL_FILLER as ::core::ffi::c_int,
                NAL_PRIORITY_DISPOSABLE as ::core::ffi::c_int,
            );
            x264_10_filler_write(h, &mut (*h).out.bs, f);
            if nal_end(h) != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            let mut total_size: ::core::ffi::c_int = encoder_encapsulate_nals(
                h,
                (*h).out.i_nal - 1 as ::core::ffi::c_int,
            );
            if total_size < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            frame_size += total_size;
            filler -= total_size;
        }
    }
    *pi_nal = (*h).out.i_nal;
    *pp_nal = (*h).out.nal;
    (*h).out.i_nal = 0 as ::core::ffi::c_int;
    x264_10_noise_reduction_update(h);
    thread_sync_stat(h, (*h).thread[0 as ::core::ffi::c_int as usize]);
    (*h).stat.i_frame_count[(*h).sh.i_type as usize] += 1;
    (*h).stat.i_frame_size[(*h).sh.i_type as usize] += frame_size as int64_t;
    (*h).stat.f_frame_qp[(*h).sh.i_type as usize]
        += (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < X264_MBTYPE_MAX as ::core::ffi::c_int {
        (*h).stat.i_mb_count[(*h).sh.i_type as usize][i_0 as usize]
            += (*h).stat.frame.i_mb_count[i_0 as usize] as int64_t;
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < 2 as ::core::ffi::c_int {
        (*h).stat.i_mb_count_8x8dct[i_1 as usize]
            += (*h).stat.frame.i_mb_count_8x8dct[i_1 as usize] as int64_t;
        i_1 += 1;
    }
    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_2 < 6 as ::core::ffi::c_int {
        (*h).stat.i_mb_cbp[i_2 as usize]
            += (*h).stat.frame.i_mb_cbp[i_2 as usize] as int64_t;
        i_2 += 1;
    }
    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_3 < 4 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 13 as ::core::ffi::c_int {
            (*h).stat.i_mb_pred_mode[i_3 as usize][j as usize]
                += (*h).stat.frame.i_mb_pred_mode[i_3 as usize][j as usize] as int64_t;
            j += 1;
        }
        i_3 += 1;
    }
    if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int {
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < X264_PARTTYPE_MAX as ::core::ffi::c_int {
            (*h).stat.i_mb_partition[(*h).sh.i_type as usize][i_4 as usize]
                += (*h).stat.frame.i_mb_partition[i_4 as usize] as int64_t;
            i_4 += 1;
        }
        let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_list < 2 as ::core::ffi::c_int {
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 < X264_REF_MAX * 2 as ::core::ffi::c_int {
                (*h)
                    .stat
                    .i_mb_count_ref[(*h).sh.i_type
                    as usize][i_list as usize][i_5 as usize]
                    += (*h).stat.frame.i_mb_count_ref[i_list as usize][i_5 as usize]
                        as int64_t;
                i_5 += 1;
            }
            i_list += 1;
        }
    }
    let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_6 < 3 as ::core::ffi::c_int {
        (*h).stat.i_mb_field[i_6 as usize]
            += (*h).stat.frame.i_mb_field[i_6 as usize] as int64_t;
        i_6 += 1;
    }
    if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int
        && (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
    {
        (*h).stat.i_wpred[0 as ::core::ffi::c_int as usize]
            += !(*h)
                .sh
                .weight[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null() as ::core::ffi::c_int;
        (*h).stat.i_wpred[1 as ::core::ffi::c_int as usize]
            += (!(*h)
                .sh
                .weight[0 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
                || !(*h)
                    .sh
                    .weight[0 as ::core::ffi::c_int
                        as usize][2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()) as ::core::ffi::c_int;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        (*h).stat.i_direct_frames[(*h).sh.b_direct_spatial_mv_pred as usize] += 1;
        if (*h).mb.b_direct_auto_write != 0 {
            if (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize]
                + (*h).stat.i_direct_score[1 as ::core::ffi::c_int as usize]
                > (*h).mb.i_mb_count
            {
                let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_7 < 2 as ::core::ffi::c_int {
                    (*h).stat.i_direct_score[i_7 as usize] = (*h)
                        .stat
                        .i_direct_score[i_7 as usize] * 9 as ::core::ffi::c_int
                        / 10 as ::core::ffi::c_int;
                    i_7 += 1;
                }
            }
            let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_8 < 2 as ::core::ffi::c_int {
                (*h).stat.i_direct_score[i_8 as usize]
                    += (*h).stat.frame.i_direct_score[i_8 as usize];
                i_8 += 1;
            }
        }
    } else {
        (*h).stat.i_consecutive_bframes[(*(*h).fenc).i_bframes as usize] += 1;
    }
    psz_message[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut dur: ::core::ffi::c_double = (*(*h).fenc).f_duration
        as ::core::ffi::c_double;
    (*h).stat.f_frame_duration[(*h).sh.i_type as usize] += dur;
    if (*h).param.analyse.b_psnr != 0 {
        let mut ssd: [int64_t; 3] = [
            (*h).stat.frame.i_ssd[0 as ::core::ffi::c_int as usize],
            (*h).stat.frame.i_ssd[1 as ::core::ffi::c_int as usize],
            (*h).stat.frame.i_ssd[2 as ::core::ffi::c_int as usize],
        ];
        let mut luma_size: ::core::ffi::c_int = (*h).param.i_width * (*h).param.i_height;
        let mut chroma_size: ::core::ffi::c_int = if (*(*h).sps.as_mut_ptr())
            .i_chroma_format_idc != 0
        {
            luma_size >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
        } else {
            0 as ::core::ffi::c_int
        };
        (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize] = calc_psnr(
            ssd[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
            luma_size as ::core::ffi::c_double,
        );
        (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize] = calc_psnr(
            ssd[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
            chroma_size as ::core::ffi::c_double,
        );
        (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize] = calc_psnr(
            ssd[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double,
            chroma_size as ::core::ffi::c_double,
        );
        (*pic_out).prop.f_psnr_avg = calc_psnr(
            (ssd[0 as ::core::ffi::c_int as usize]
                + ssd[1 as ::core::ffi::c_int as usize]
                + ssd[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double,
            (luma_size + chroma_size * 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        );
        (*h).stat.f_ssd_global[(*h).sh.i_type as usize]
            += dur
                * (ssd[0 as ::core::ffi::c_int as usize]
                    + ssd[1 as ::core::ffi::c_int as usize]
                    + ssd[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double;
        (*h).stat.f_psnr_average[(*h).sh.i_type as usize]
            += dur * (*pic_out).prop.f_psnr_avg;
        (*h).stat.f_psnr_mean_y[(*h).sh.i_type as usize]
            += dur * (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize];
        (*h).stat.f_psnr_mean_u[(*h).sh.i_type as usize]
            += dur * (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize];
        (*h).stat.f_psnr_mean_v[(*h).sh.i_type as usize]
            += dur * (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize];
        snprintf(
            psz_message.as_mut_ptr(),
            80 as size_t,
            b" PSNR Y:%5.2f U:%5.2f V:%5.2f\0" as *const u8
                as *const ::core::ffi::c_char,
            (*pic_out).prop.f_psnr[0 as ::core::ffi::c_int as usize],
            (*pic_out).prop.f_psnr[1 as ::core::ffi::c_int as usize],
            (*pic_out).prop.f_psnr[2 as ::core::ffi::c_int as usize],
        );
    }
    if (*h).param.analyse.b_ssim != 0 {
        (*pic_out).prop.f_ssim = (*h).stat.frame.f_ssim
            / (*h).stat.frame.i_ssim_cnt as ::core::ffi::c_double;
        (*h).stat.f_ssim_mean_y[(*h).sh.i_type as usize] += (*pic_out).prop.f_ssim * dur;
        let mut msg_len: ::core::ffi::c_int = strlen(psz_message.as_mut_ptr())
            as ::core::ffi::c_int;
        snprintf(
            psz_message.as_mut_ptr().offset(msg_len as isize),
            (80 as ::core::ffi::c_int - msg_len) as size_t,
            b" SSIM Y:%.5f\0" as *const u8 as *const ::core::ffi::c_char,
            (*pic_out).prop.f_ssim,
        );
    }
    psz_message[79 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    x264_10_log(
        h,
        X264_LOG_DEBUG,
        b"frame=%4d QP=%.2f NAL=%d Slice:%c Poc:%-3d I:%-4d P:%-4d SKIP:%-4d size=%d bytes%s\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        (*h).i_frame,
        (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double,
        (*h).i_nal_ref_idc,
        if (*h).sh.i_type == SLICE_TYPE_I as ::core::ffi::c_int {
            'I' as i32
        } else if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
            'P' as i32
        } else {
            'B' as i32
        },
        (*(*h).fdec).i_poc,
        (*h).stat.frame.i_mb_count_i,
        (*h).stat.frame.i_mb_count_p,
        (*h).stat.frame.i_mb_count_skip,
        frame_size,
        psz_message.as_mut_ptr(),
    );
    thread_sync_stat((*h).thread[0 as ::core::ffi::c_int as usize], h);
    thread_sync_stat(thread_current, h);
    let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_9 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
        if !(*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize].is_null()
            && (*(*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize]).b_duplicate
                != 0
        {
            x264_10_frame_push_blank_unused(
                h,
                (*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize],
            );
            (*h).fref[0 as ::core::ffi::c_int as usize][i_9 as usize] = 0
                as *mut x264_frame_t;
        }
        i_9 += 1;
    }
    if !(*h).param.psz_dump_yuv.is_null() {
        frame_dump(h);
    }
    return frame_size;
}
#[c2rust::src_loc = "4182:1"]
unsafe extern "C" fn print_intra(
    mut i_mb_count: *mut int64_t,
    mut i_count: ::core::ffi::c_double,
    mut b_print_pcm: ::core::ffi::c_int,
    mut intra: *mut ::core::ffi::c_char,
) {
    intra = intra
        .offset(
            sprintf(
                intra,
                b"I16..4%s: %4.1f%% %4.1f%% %4.1f%%\0" as *const u8
                    as *const ::core::ffi::c_char,
                if b_print_pcm != 0 {
                    b"..PCM\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                *i_mb_count.offset(I_16x16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double / i_count,
                *i_mb_count.offset(I_8x8 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double / i_count,
                *i_mb_count.offset(I_4x4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_double / i_count,
            ) as isize,
        );
    if b_print_pcm != 0 {
        sprintf(
            intra,
            b" %4.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
            *i_mb_count.offset(I_PCM as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double / i_count,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "4196:1"]
pub unsafe extern "C" fn x264_10_encoder_close(mut h: *mut x264_t) {
    let mut i_yuv_size: int64_t = ((*h).param.i_width * (*h).param.i_height
        + 2 as ::core::ffi::c_int
            * (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                (*h).param.i_width * (*h).param.i_height
                    >> (*h).mb.chroma_h_shift + (*h).mb.chroma_v_shift
            } else {
                0 as ::core::ffi::c_int
            })) as int64_t;
    let mut i_mb_count_size: [[int64_t; 7]; 2] = [
        [0 as ::core::ffi::c_int as int64_t, 0, 0, 0, 0, 0, 0],
        [0; 7],
    ];
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut b_print_pcm: ::core::ffi::c_int = ((*h)
        .stat
        .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
        as usize][I_PCM as ::core::ffi::c_int as usize] != 0
        || (*h)
            .stat
            .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
            as usize][I_PCM as ::core::ffi::c_int as usize] != 0
        || (*h)
            .stat
            .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
            as usize][I_PCM as ::core::ffi::c_int as usize] != 0) as ::core::ffi::c_int;
    x264_10_lookahead_delete(h);
    if (*h).param.b_sliced_threads != 0 {
        threadpool_wait_all(h);
    }
    if (*h).param.i_threads > 1 as ::core::ffi::c_int {
        x264_10_threadpool_delete((*h).threadpool);
    }
    if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int {
        x264_10_threadpool_delete((*h).lookaheadpool);
    }
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).i_thread_frames {
            if (*(*h).thread[i as usize]).b_thread_active != 0 {
                if (*(*(*h).thread[i as usize]).fenc).i_reference_count
                    == 1 as ::core::ffi::c_int
                {} else {
                    __assert_fail(
                        b"h->thread[i]->fenc->i_reference_count == 1\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"encoder/encoder.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        4223 as ::core::ffi::c_uint,
                        ::core::mem::transmute::<
                            [u8; 37],
                            [::core::ffi::c_char; 37],
                        >(*b"void x264_10_encoder_close(x264_t *)\0")
                            .as_ptr(),
                    );
                }
                'c_64771: {
                    if (*(*(*h).thread[i as usize]).fenc).i_reference_count
                        == 1 as ::core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"h->thread[i]->fenc->i_reference_count == 1\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"encoder/encoder.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            4223 as ::core::ffi::c_uint,
                            ::core::mem::transmute::<
                                [u8; 37],
                                [::core::ffi::c_char; 37],
                            >(*b"void x264_10_encoder_close(x264_t *)\0")
                                .as_ptr(),
                        );
                    }
                };
                x264_10_frame_delete((*(*h).thread[i as usize]).fenc);
            }
            i += 1;
        }
        let mut thread_prev: *mut x264_t = (*h).thread[(*h).i_thread_phase as usize];
        x264_10_thread_sync_ratecontrol(h, thread_prev, h);
        x264_10_thread_sync_ratecontrol(thread_prev, thread_prev, h);
        (*h).i_frame = (*thread_prev).i_frame + 1 as ::core::ffi::c_int
            - (*h).i_thread_frames;
    }
    (*h).i_frame += 1;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 3 as ::core::ffi::c_int {
        static mut slice_order: [uint8_t; 3] = [
            SLICE_TYPE_I as ::core::ffi::c_int as uint8_t,
            SLICE_TYPE_P as ::core::ffi::c_int as uint8_t,
            SLICE_TYPE_B as ::core::ffi::c_int as uint8_t,
        ];
        let mut i_slice: ::core::ffi::c_int = slice_order[i_0 as usize]
            as ::core::ffi::c_int;
        if (*h).stat.i_frame_count[i_slice as usize] > 0 as ::core::ffi::c_int {
            let mut i_count: ::core::ffi::c_int = (*h)
                .stat
                .i_frame_count[i_slice as usize];
            let mut dur: ::core::ffi::c_double = (*h)
                .stat
                .f_frame_duration[i_slice as usize];
            if (*h).param.analyse.b_psnr != 0 {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f  PSNR Mean Y:%5.2f U:%5.2f V:%5.2f Avg:%5.2f Global:%5.2f\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    slice_type_to_char[i_slice as usize] as ::core::ffi::c_int,
                    i_count,
                    (*h).stat.f_frame_qp[i_slice as usize]
                        / i_count as ::core::ffi::c_double,
                    (*h).stat.i_frame_size[i_slice as usize] as ::core::ffi::c_double
                        / i_count as ::core::ffi::c_double,
                    (*h).stat.f_psnr_mean_y[i_slice as usize] / dur,
                    (*h).stat.f_psnr_mean_u[i_slice as usize] / dur,
                    (*h).stat.f_psnr_mean_v[i_slice as usize] / dur,
                    (*h).stat.f_psnr_average[i_slice as usize] / dur,
                    calc_psnr(
                        (*h).stat.f_ssd_global[i_slice as usize],
                        dur * i_yuv_size as ::core::ffi::c_double,
                    ),
                );
            } else {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"frame %c:%-5d Avg QP:%5.2f  size:%6.0f\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    slice_type_to_char[i_slice as usize] as ::core::ffi::c_int,
                    i_count,
                    (*h).stat.f_frame_qp[i_slice as usize]
                        / i_count as ::core::ffi::c_double,
                    (*h).stat.i_frame_size[i_slice as usize] as ::core::ffi::c_double
                        / i_count as ::core::ffi::c_double,
                );
            }
        }
        i_0 += 1;
    }
    if (*h).param.i_bframe != 0
        && (*h).stat.i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize] != 0
    {
        let mut p: *mut ::core::ffi::c_char = buf.as_mut_ptr();
        let mut den: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 <= (*h).param.i_bframe {
            den
                += (i_1 + 1 as ::core::ffi::c_int)
                    * (*h).stat.i_consecutive_bframes[i_1 as usize];
            i_1 += 1;
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 <= (*h).param.i_bframe {
            p = p
                .offset(
                    sprintf(
                        p,
                        b" %4.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                        100.0f64
                            * (i_2 + 1 as ::core::ffi::c_int) as ::core::ffi::c_double
                            * (*h).stat.i_consecutive_bframes[i_2 as usize]
                                as ::core::ffi::c_double / den as ::core::ffi::c_double,
                    ) as isize,
                );
            i_2 += 1;
        }
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"consecutive B-frames:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            buf.as_mut_ptr(),
        );
    }
    let mut i_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_type < 2 as ::core::ffi::c_int {
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < X264_PARTTYPE_MAX as ::core::ffi::c_int {
            if !(i_3 == D_DIRECT_8x8 as ::core::ffi::c_int) {
                i_mb_count_size[i_type
                    as usize][x264_mb_partition_pixel_table[i_3 as usize] as usize]
                    += (*h).stat.i_mb_partition[i_type as usize][i_3 as usize];
            }
            i_3 += 1;
        }
        i_type += 1;
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_I as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
    {
        let mut i_mb_count: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_I as ::core::ffi::c_int as isize))
            .as_mut_ptr();
        let mut i_count_0: ::core::ffi::c_double = (*h)
            .stat
            .i_frame_count[SLICE_TYPE_I as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double * (*h).mb.i_mb_count as ::core::ffi::c_double
            / 100.0f64;
        print_intra(i_mb_count, i_count_0, b_print_pcm, buf.as_mut_ptr());
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb I  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            buf.as_mut_ptr(),
        );
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
    {
        let mut i_mb_count_0: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_P as ::core::ffi::c_int as isize))
            .as_mut_ptr();
        let mut i_count_1: ::core::ffi::c_double = (*h)
            .stat
            .i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double * (*h).mb.i_mb_count as ::core::ffi::c_double
            / 100.0f64;
        let mut i_mb_size: *mut int64_t = (*i_mb_count_size
            .as_mut_ptr()
            .offset(SLICE_TYPE_P as ::core::ffi::c_int as isize))
            .as_mut_ptr();
        print_intra(i_mb_count_0, i_count_1, b_print_pcm, buf.as_mut_ptr());
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb P  %s  P16..4: %4.1f%% %4.1f%% %4.1f%% %4.1f%% %4.1f%%    skip:%4.1f%%\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            buf.as_mut_ptr(),
            *i_mb_size.offset(PIXEL_16x16 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            (*i_mb_size.offset(PIXEL_16x8 as ::core::ffi::c_int as isize)
                + *i_mb_size.offset(PIXEL_8x16 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double
                / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            *i_mb_size.offset(PIXEL_8x8 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            (*i_mb_size.offset(PIXEL_8x4 as ::core::ffi::c_int as isize)
                + *i_mb_size.offset(PIXEL_4x8 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double
                / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            *i_mb_size.offset(PIXEL_4x4 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / (i_count_1 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            *i_mb_count_0.offset(P_SKIP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double / i_count_1,
        );
    }
    if (*h).stat.i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
    {
        let mut i_mb_count_1: *mut int64_t = (*(*h)
            .stat
            .i_mb_count
            .as_mut_ptr()
            .offset(SLICE_TYPE_B as ::core::ffi::c_int as isize))
            .as_mut_ptr();
        let mut i_count_2: ::core::ffi::c_double = (*h)
            .stat
            .i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize]
            as ::core::ffi::c_double * (*h).mb.i_mb_count as ::core::ffi::c_double
            / 100.0f64;
        let mut i_mb_list_count: ::core::ffi::c_double = 0.;
        let mut i_mb_size_0: *mut int64_t = (*i_mb_count_size
            .as_mut_ptr()
            .offset(SLICE_TYPE_B as ::core::ffi::c_int as isize))
            .as_mut_ptr();
        let mut list_count: [int64_t; 3] = [0 as ::core::ffi::c_int as int64_t, 0, 0];
        print_intra(i_mb_count_1, i_count_2, b_print_pcm, buf.as_mut_ptr());
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < X264_PARTTYPE_MAX as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 2 as ::core::ffi::c_int {
                let mut l0: ::core::ffi::c_int = x264_mb_type_list_table[i_4
                    as usize][0 as ::core::ffi::c_int as usize][j as usize]
                    as ::core::ffi::c_int;
                let mut l1: ::core::ffi::c_int = x264_mb_type_list_table[i_4
                    as usize][1 as ::core::ffi::c_int as usize][j as usize]
                    as ::core::ffi::c_int;
                if l0 != 0 || l1 != 0 {
                    list_count[(l1 + l0 * l1) as usize]
                        += (*h)
                            .stat
                            .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                            as usize][i_4 as usize] * 2 as int64_t;
                }
                j += 1;
            }
            i_4 += 1;
        }
        list_count[0 as ::core::ffi::c_int as usize]
            += (*h)
                .stat
                .i_mb_partition[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][D_L0_8x8 as ::core::ffi::c_int as usize];
        list_count[1 as ::core::ffi::c_int as usize]
            += (*h)
                .stat
                .i_mb_partition[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][D_L1_8x8 as ::core::ffi::c_int as usize];
        list_count[2 as ::core::ffi::c_int as usize]
            += (*h)
                .stat
                .i_mb_partition[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][D_BI_8x8 as ::core::ffi::c_int as usize];
        *i_mb_count_1.offset(B_DIRECT as ::core::ffi::c_int as isize)
            += ((*h)
                .stat
                .i_mb_partition[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][D_DIRECT_8x8 as ::core::ffi::c_int as usize] + 2 as int64_t)
                / 4 as int64_t;
        i_mb_list_count = (list_count[0 as ::core::ffi::c_int as usize]
            + list_count[1 as ::core::ffi::c_int as usize]
            + list_count[2 as ::core::ffi::c_int as usize]) as ::core::ffi::c_double
            / 100.0f64;
        sprintf(
            buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
            b"  B16..8: %4.1f%% %4.1f%% %4.1f%%  direct:%4.1f%%  skip:%4.1f%%\0"
                as *const u8 as *const ::core::ffi::c_char,
            *i_mb_size_0.offset(PIXEL_16x16 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            (*i_mb_size_0.offset(PIXEL_16x8 as ::core::ffi::c_int as isize)
                + *i_mb_size_0.offset(PIXEL_8x16 as ::core::ffi::c_int as isize))
                as ::core::ffi::c_double
                / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            *i_mb_size_0.offset(PIXEL_8x8 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double
                / (i_count_2 * 4 as ::core::ffi::c_int as ::core::ffi::c_double),
            *i_mb_count_1.offset(B_DIRECT as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double / i_count_2,
            *i_mb_count_1.offset(B_SKIP as ::core::ffi::c_int as isize)
                as ::core::ffi::c_double / i_count_2,
        );
        if i_mb_list_count != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            sprintf(
                buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
                b"  L0:%4.1f%% L1:%4.1f%% BI:%4.1f%%\0" as *const u8
                    as *const ::core::ffi::c_char,
                list_count[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    / i_mb_list_count,
                list_count[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    / i_mb_list_count,
                list_count[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    / i_mb_list_count,
            );
        }
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"mb B  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            buf.as_mut_ptr(),
        );
    }
    x264_10_ratecontrol_summary(h);
    if (*h).stat.i_frame_count[SLICE_TYPE_I as ::core::ffi::c_int as usize]
        + (*h).stat.i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
        + (*h).stat.i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
    {
        let mut i_i8x8: int64_t = (*h)
            .stat
            .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
            as usize][I_8x8 as ::core::ffi::c_int as usize]
            + (*h)
                .stat
                .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                as usize][I_8x8 as ::core::ffi::c_int as usize]
            + (*h)
                .stat
                .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][I_8x8 as ::core::ffi::c_int as usize];
        let mut i_intra: int64_t = i_i8x8
            + ((*h)
                .stat
                .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
                as usize][I_4x4 as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                    as usize][I_4x4 as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                    as usize][I_4x4 as ::core::ffi::c_int as usize])
            + ((*h)
                .stat
                .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
                as usize][I_16x16 as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                    as usize][I_16x16 as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                    as usize][I_16x16 as ::core::ffi::c_int as usize]);
        let mut i_all_intra: int64_t = i_intra
            + ((*h)
                .stat
                .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
                as usize][I_PCM as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                    as usize][I_PCM as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                    as usize][I_PCM as ::core::ffi::c_int as usize]);
        let mut i_skip: int64_t = (*h)
            .stat
            .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
            as usize][P_SKIP as ::core::ffi::c_int as usize]
            + (*h)
                .stat
                .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                as usize][P_SKIP as ::core::ffi::c_int as usize]
            + (*h)
                .stat
                .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                as usize][P_SKIP as ::core::ffi::c_int as usize]
            + ((*h)
                .stat
                .i_mb_count[SLICE_TYPE_I as ::core::ffi::c_int
                as usize][B_SKIP as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_P as ::core::ffi::c_int
                    as usize][B_SKIP as ::core::ffi::c_int as usize]
                + (*h)
                    .stat
                    .i_mb_count[SLICE_TYPE_B as ::core::ffi::c_int
                    as usize][B_SKIP as ::core::ffi::c_int as usize]);
        let i_count_3: ::core::ffi::c_int = (*h)
            .stat
            .i_frame_count[SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize];
        let mut i_mb_count_2: int64_t = i_count_3 as int64_t
            * (*h).mb.i_mb_count as int64_t;
        let mut i_inter: int64_t = i_mb_count_2 - i_skip - i_all_intra;
        let duration: ::core::ffi::c_double = (*h)
            .stat
            .f_frame_duration[SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.f_frame_duration[SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.f_frame_duration[SLICE_TYPE_B as ::core::ffi::c_int as usize];
        let mut f_bitrate: ::core::ffi::c_float = (((*h)
            .stat
            .i_frame_size[SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_B as ::core::ffi::c_int as usize])
            as ::core::ffi::c_double / duration
            / 125 as ::core::ffi::c_int as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        if (*h).param.b_interlaced != 0 {
            let mut fieldstats: *mut ::core::ffi::c_char = buf.as_mut_ptr();
            *fieldstats.offset(0 as ::core::ffi::c_int as isize) = 0
                as ::core::ffi::c_char;
            if i_inter != 0 {
                fieldstats = fieldstats
                    .offset(
                        sprintf(
                            fieldstats,
                            b" inter:%.1f%%\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*h).stat.i_mb_field[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_double * 100.0f64
                                / i_inter as ::core::ffi::c_double,
                        ) as isize,
                    );
            }
            if i_skip != 0 {
                fieldstats = fieldstats
                    .offset(
                        sprintf(
                            fieldstats,
                            b" skip:%.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                            (*h).stat.i_mb_field[2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_double * 100.0f64
                                / i_skip as ::core::ffi::c_double,
                        ) as isize,
                    );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"field mbs: intra: %.1f%%%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).stat.i_mb_field[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / i_all_intra as ::core::ffi::c_double,
                buf.as_mut_ptr(),
            );
        }
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
            buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            if (*h).stat.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize] != 0 {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter:%.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                    100.0f64
                        * (*h).stat.i_mb_count_8x8dct[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
                        / (*h).stat.i_mb_count_8x8dct[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"8x8 transform intra:%.1f%%%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                100.0f64 * i_i8x8 as ::core::ffi::c_double
                    / (if i_intra > 1 as int64_t { i_intra } else { 1 as int64_t })
                        as ::core::ffi::c_double,
                buf.as_mut_ptr(),
            );
        }
        if ((*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
            || (*h).stat.i_direct_frames[0 as ::core::ffi::c_int as usize] != 0
                && (*h).stat.i_direct_frames[1 as ::core::ffi::c_int as usize] != 0)
            && (*h).stat.i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize] != 0
        {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"direct mvs  spatial:%.1f%% temporal:%.1f%%\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).stat.i_direct_frames[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (*h)
                        .stat
                        .i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                (*h).stat.i_direct_frames[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (*h)
                        .stat
                        .i_frame_count[SLICE_TYPE_B as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
            );
        }
        buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            let mut csize: ::core::ffi::c_int = if (*(*h).sps.as_mut_ptr())
                .i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            if i_mb_count_2 != i_all_intra {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter: %.1f%% %.1f%% %.1f%%\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*h).stat.i_mb_cbp[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * 4 as int64_t)
                            as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * csize as int64_t)
                            as ::core::ffi::c_double,
                    (*h).stat.i_mb_cbp[5 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * csize as int64_t)
                            as ::core::ffi::c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"coded y,%s,%s intra: %.1f%% %.1f%% %.1f%%%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as ::core::ffi::c_int
                {
                    b"u\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"uvDC\0" as *const u8 as *const ::core::ffi::c_char
                },
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as ::core::ffi::c_int
                {
                    b"v\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"uvAC\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*h).stat.i_mb_cbp[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (i_all_intra * 4 as int64_t) as ::core::ffi::c_double,
                (*h).stat.i_mb_cbp[2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (i_all_intra * csize as int64_t) as ::core::ffi::c_double,
                (*h).stat.i_mb_cbp[4 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (i_all_intra * csize as int64_t) as ::core::ffi::c_double,
                buf.as_mut_ptr(),
            );
        } else {
            if i_mb_count_2 != i_all_intra {
                sprintf(
                    buf.as_mut_ptr(),
                    b" inter: %.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                    (*h).stat.i_mb_cbp[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / ((i_mb_count_2 - i_all_intra) * 4 as int64_t)
                            as ::core::ffi::c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"coded y intra: %.1f%%%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).stat.i_mb_cbp[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (i_all_intra * 4 as int64_t) as ::core::ffi::c_double,
                buf.as_mut_ptr(),
            );
        }
        let mut fixed_pred_modes: [[int64_t; 9]; 4] = [
            [0 as ::core::ffi::c_int as int64_t, 0, 0, 0, 0, 0, 0, 0, 0],
            [0; 9],
            [0; 9],
            [0; 9],
        ];
        let mut sum_pred_modes: [int64_t; 4] = [
            0 as ::core::ffi::c_int as int64_t,
            0,
            0,
            0,
        ];
        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_5 <= I_PRED_16x16_DC_128 as ::core::ffi::c_int {
            fixed_pred_modes[0 as ::core::ffi::c_int
                as usize][x264_mb_pred_mode16x16_fix[i_5 as usize] as usize]
                += (*h)
                    .stat
                    .i_mb_pred_mode[0 as ::core::ffi::c_int as usize][i_5 as usize];
            sum_pred_modes[0 as ::core::ffi::c_int as usize]
                += (*h)
                    .stat
                    .i_mb_pred_mode[0 as ::core::ffi::c_int as usize][i_5 as usize];
            i_5 += 1;
        }
        if sum_pred_modes[0 as ::core::ffi::c_int as usize] != 0 {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"i16 v,h,dc,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                fixed_pred_modes[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[0 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[0 as ::core::ffi::c_int
                    as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[0 as ::core::ffi::c_int
                    as usize][3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
            );
        }
        let mut i_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_6 <= 2 as ::core::ffi::c_int {
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 <= I_PRED_8x8_DC_128 as ::core::ffi::c_int {
                fixed_pred_modes[i_6
                    as usize][x264_mb_pred_mode4x4_fix[(j_0 + 1 as ::core::ffi::c_int)
                    as usize] as usize]
                    += (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                sum_pred_modes[i_6 as usize]
                    += (*h).stat.i_mb_pred_mode[i_6 as usize][j_0 as usize];
                j_0 += 1;
            }
            if sum_pred_modes[i_6 as usize] != 0 {
                x264_10_log(
                    h,
                    X264_LOG_INFO,
                    b"i%d v,h,dc,ddl,ddr,vr,hd,vl,hu: %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (3 as ::core::ffi::c_int - i_6) * 4 as ::core::ffi::c_int,
                    fixed_pred_modes[i_6 as usize][0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][4 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][5 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][6 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][7 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                    fixed_pred_modes[i_6 as usize][8 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / sum_pred_modes[i_6 as usize] as ::core::ffi::c_double,
                );
            }
            i_6 += 1;
        }
        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_7 <= I_PRED_CHROMA_DC_128 as ::core::ffi::c_int {
            fixed_pred_modes[3 as ::core::ffi::c_int
                as usize][x264_mb_chroma_pred_mode_fix[i_7 as usize] as usize]
                += (*h)
                    .stat
                    .i_mb_pred_mode[3 as ::core::ffi::c_int as usize][i_7 as usize];
            sum_pred_modes[3 as ::core::ffi::c_int as usize]
                += (*h)
                    .stat
                    .i_mb_pred_mode[3 as ::core::ffi::c_int as usize][i_7 as usize];
            i_7 += 1;
        }
        if sum_pred_modes[3 as ::core::ffi::c_int as usize] != 0
            && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                == CHROMA_444 as ::core::ffi::c_int)
        {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"i8c dc,h,v,p: %2.0f%% %2.0f%% %2.0f%% %2.0f%%\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                fixed_pred_modes[3 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[3 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[3 as ::core::ffi::c_int
                    as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                fixed_pred_modes[3 as ::core::ffi::c_int
                    as usize][3 as ::core::ffi::c_int as usize] as ::core::ffi::c_double
                    * 100.0f64
                    / sum_pred_modes[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
            );
        }
        if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
            && (*h).stat.i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                > 0 as ::core::ffi::c_int
        {
            buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                sprintf(
                    buf.as_mut_ptr(),
                    b" UV:%.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                    (*h).stat.i_wpred[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double * 100.0f64
                        / (*h)
                            .stat
                            .i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double,
                );
            }
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"Weighted P-Frames: Y:%.1f%%%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).stat.i_wpred[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_double * 100.0f64
                    / (*h)
                        .stat
                        .i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_double,
                buf.as_mut_ptr(),
            );
        }
        let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_list < 2 as ::core::ffi::c_int {
            let mut i_slice_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_slice_0 < 2 as ::core::ffi::c_int {
                let mut p_0: *mut ::core::ffi::c_char = buf.as_mut_ptr();
                let mut i_den: int64_t = 0 as int64_t;
                let mut i_max: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_8 < X264_REF_MAX * 2 as ::core::ffi::c_int {
                    if (*h)
                        .stat
                        .i_mb_count_ref[i_slice_0
                        as usize][i_list as usize][i_8 as usize] != 0
                    {
                        i_den
                            += (*h)
                                .stat
                                .i_mb_count_ref[i_slice_0
                                as usize][i_list as usize][i_8 as usize];
                        i_max = i_8;
                    }
                    i_8 += 1;
                }
                if !(i_max == 0 as ::core::ffi::c_int) {
                    let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_9 <= i_max {
                        p_0 = p_0
                            .offset(
                                sprintf(
                                    p_0,
                                    b" %4.1f%%\0" as *const u8 as *const ::core::ffi::c_char,
                                    100.0f64
                                        * (*h)
                                            .stat
                                            .i_mb_count_ref[i_slice_0
                                            as usize][i_list as usize][i_9 as usize]
                                            as ::core::ffi::c_double / i_den as ::core::ffi::c_double,
                                ) as isize,
                            );
                        i_9 += 1;
                    }
                    x264_10_log(
                        h,
                        X264_LOG_INFO,
                        b"ref %c L%d:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            [u8; 3],
                            [::core::ffi::c_char; 3],
                        >(*b"PB\0")[i_slice_0 as usize] as ::core::ffi::c_int,
                        i_list,
                        buf.as_mut_ptr(),
                    );
                }
                i_slice_0 += 1;
            }
            i_list += 1;
        }
        if (*h).param.analyse.b_ssim != 0 {
            let mut ssim: ::core::ffi::c_float = (((*h)
                .stat
                .f_ssim_mean_y[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                + (*h).stat.f_ssim_mean_y[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                + (*h).stat.f_ssim_mean_y[SLICE_TYPE_B as ::core::ffi::c_int as usize])
                / duration) as ::core::ffi::c_float;
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"SSIM Mean Y:%.7f (%6.3fdb)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                ssim as ::core::ffi::c_double,
                calc_ssim_db(ssim as ::core::ffi::c_double),
            );
        }
        if (*h).param.analyse.b_psnr != 0 {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"PSNR Mean Y:%6.3f U:%6.3f V:%6.3f Avg:%6.3f Global:%6.3f kb/s:%.2f\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                ((*h).stat.f_psnr_mean_y[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_y[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_y[SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_mean_u[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_u[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_u[SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_mean_v[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_v[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_mean_v[SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration,
                ((*h).stat.f_psnr_average[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_average[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                    + (*h)
                        .stat
                        .f_psnr_average[SLICE_TYPE_B as ::core::ffi::c_int as usize])
                    / duration,
                calc_psnr(
                    (*h).stat.f_ssd_global[SLICE_TYPE_I as ::core::ffi::c_int as usize]
                        + (*h)
                            .stat
                            .f_ssd_global[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        + (*h)
                            .stat
                            .f_ssd_global[SLICE_TYPE_B as ::core::ffi::c_int as usize],
                    duration * i_yuv_size as ::core::ffi::c_double,
                ),
                f_bitrate as ::core::ffi::c_double,
            );
        } else {
            x264_10_log(
                h,
                X264_LOG_INFO,
                b"kb/s:%.2f\n\0" as *const u8 as *const ::core::ffi::c_char,
                f_bitrate as ::core::ffi::c_double,
            );
        }
    }
    x264_10_ratecontrol_delete(h);
    x264_param_cleanup(&mut (*h).param);
    x264_10_cqm_delete(h);
    x264_free((*h).nal_buffer as *mut ::core::ffi::c_void);
    x264_free((*h).reconfig_h as *mut ::core::ffi::c_void);
    x264_10_analyse_free_costs(h);
    x264_free((*h).cost_table as *mut ::core::ffi::c_void);
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        h = (*h).thread[(*h).i_thread_phase as usize];
    }
    x264_10_frame_delete_list((*h).frames.unused[0 as ::core::ffi::c_int as usize]);
    x264_10_frame_delete_list((*h).frames.unused[1 as ::core::ffi::c_int as usize]);
    x264_10_frame_delete_list((*h).frames.current);
    x264_10_frame_delete_list((*h).frames.blank_unused);
    h = (*h).thread[0 as ::core::ffi::c_int as usize];
    let mut i_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_10 < (*h).i_thread_frames {
        if (*(*h).thread[i_10 as usize]).b_thread_active != 0 {
            let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_1
                < (*(*h).thread[i_10 as usize]).i_ref[0 as ::core::ffi::c_int as usize]
            {
                if !(*(*h).thread[i_10 as usize])
                    .fref[0 as ::core::ffi::c_int as usize][j_1 as usize]
                    .is_null()
                    && (*(*(*h).thread[i_10 as usize])
                        .fref[0 as ::core::ffi::c_int as usize][j_1 as usize])
                        .b_duplicate != 0
                {
                    x264_10_frame_delete(
                        (*(*h).thread[i_10 as usize])
                            .fref[0 as ::core::ffi::c_int as usize][j_1 as usize],
                    );
                }
                j_1 += 1;
            }
        }
        i_10 += 1;
    }
    if (*h).param.i_lookahead_threads > 1 as ::core::ffi::c_int {
        let mut i_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_11 < (*h).param.i_lookahead_threads {
            x264_free((*h).lookahead_thread[i_11 as usize] as *mut ::core::ffi::c_void);
            i_11 += 1;
        }
    }
    let mut i_12: ::core::ffi::c_int = (*h).param.i_threads - 1 as ::core::ffi::c_int;
    while i_12 >= 0 as ::core::ffi::c_int {
        let mut frame: *mut *mut x264_frame_t = 0 as *mut *mut x264_frame_t;
        if (*h).param.b_sliced_threads == 0 || i_12 == 0 as ::core::ffi::c_int {
            frame = (**(*h).thread.as_mut_ptr().offset(i_12 as isize))
                .frames
                .reference
                .as_mut_ptr();
            while !(*frame).is_null() {
                if (**frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"(*frame)->i_reference_count > 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"encoder/encoder.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        4552 as ::core::ffi::c_uint,
                        ::core::mem::transmute::<
                            [u8; 37],
                            [::core::ffi::c_char; 37],
                        >(*b"void x264_10_encoder_close(x264_t *)\0")
                            .as_ptr(),
                    );
                }
                'c_61212: {
                    if (**frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
                        __assert_fail(
                            b"(*frame)->i_reference_count > 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"encoder/encoder.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            4552 as ::core::ffi::c_uint,
                            ::core::mem::transmute::<
                                [u8; 37],
                                [::core::ffi::c_char; 37],
                            >(*b"void x264_10_encoder_close(x264_t *)\0")
                                .as_ptr(),
                        );
                    }
                };
                (**frame).i_reference_count -= 1;
                if (**frame).i_reference_count == 0 as ::core::ffi::c_int {
                    x264_10_frame_delete(*frame);
                }
                frame = frame.offset(1);
            }
            frame = &mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).fdec;
            if !(*frame).is_null() {
                if (**frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"(*frame)->i_reference_count > 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"encoder/encoder.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        4560 as ::core::ffi::c_uint,
                        ::core::mem::transmute::<
                            [u8; 37],
                            [::core::ffi::c_char; 37],
                        >(*b"void x264_10_encoder_close(x264_t *)\0")
                            .as_ptr(),
                    );
                }
                'c_61120: {
                    if (**frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
                        __assert_fail(
                            b"(*frame)->i_reference_count > 0\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"encoder/encoder.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            4560 as ::core::ffi::c_uint,
                            ::core::mem::transmute::<
                                [u8; 37],
                                [::core::ffi::c_char; 37],
                            >(*b"void x264_10_encoder_close(x264_t *)\0")
                                .as_ptr(),
                        );
                    }
                };
                (**frame).i_reference_count -= 1;
                if (**frame).i_reference_count == 0 as ::core::ffi::c_int {
                    x264_10_frame_delete(*frame);
                }
            }
            x264_10_macroblock_cache_free((*h).thread[i_12 as usize]);
        }
        x264_10_macroblock_thread_free(
            (*h).thread[i_12 as usize],
            0 as ::core::ffi::c_int,
        );
        x264_free(
            (*(*h).thread[i_12 as usize]).out.p_bitstream as *mut ::core::ffi::c_void,
        );
        x264_free((*(*h).thread[i_12 as usize]).out.nal as *mut ::core::ffi::c_void);
        pthread_mutex_destroy(
            &mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).mutex,
        );
        pthread_cond_destroy(&mut (**(*h).thread.as_mut_ptr().offset(i_12 as isize)).cv);
        x264_free((*h).thread[i_12 as usize] as *mut ::core::ffi::c_void);
        i_12 -= 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "4579:1"]
pub unsafe extern "C" fn x264_10_encoder_delayed_frames(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut delayed_frames: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).i_thread_frames {
            delayed_frames += (*(*h).thread[i as usize]).b_thread_active;
            i += 1;
        }
        h = (*h).thread[(*h).i_thread_phase as usize];
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*(*h).frames.current.offset(i_0 as isize)).is_null() {
        delayed_frames += 1;
        i_0 += 1;
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    delayed_frames
        += (*(*h).lookahead).ifbuf.i_size + (*(*h).lookahead).next.i_size
            + (*(*h).lookahead).ofbuf.i_size;
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    return delayed_frames;
}
#[no_mangle]
#[c2rust::src_loc = "4600:1"]
pub unsafe extern "C" fn x264_10_encoder_maximum_delayed_frames(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    return (*h).frames.i_delay;
}
