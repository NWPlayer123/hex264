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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:32"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:32"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
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
#[c2rust::header_src = "/usr/include/stdint.h:32"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:32"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "27:1"]
    pub type pthread_t = ::core::ffi::c_ulong;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:32"]
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
        pub out: C2RustUnnamed_19,
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
        pub cost_table: *mut C2RustUnnamed_18,
        pub chroma_qp_table: *const uint8_t,
        pub sh: x264_slice_header_t,
        pub sps: [x264_sps_t; 1],
        pub pps: [x264_pps_t; 1],
        pub b_sh_backup: ::core::ffi::c_int,
        pub sh_backup: x264_slice_header_t,
        pub cabac: x264_cabac_t,
        pub frames: C2RustUnnamed_12,
        pub fenc: *mut x264_frame_t,
        pub fdec: *mut x264_frame_t,
        pub i_ref: [::core::ffi::c_int; 2],
        pub fref: [[*mut x264_frame_t; 19]; 2],
        pub fref_nearest: [*mut x264_frame_t; 2],
        pub b_ref_reorder: [::core::ffi::c_int; 2],
        pub initial_cpb_removal_delay: ::core::ffi::c_int,
        pub initial_cpb_removal_delay_offset: ::core::ffi::c_int,
        pub i_reordered_pts_delay: int64_t,
        pub dct: C2RustUnnamed_11,
        pub mb: C2RustUnnamed_8,
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
    pub struct C2RustUnnamed_8 {
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
        pub pic: C2RustUnnamed_10,
        pub cache: C2RustUnnamed_9,
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
    pub struct C2RustUnnamed_9 {
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
    pub struct C2RustUnnamed_10 {
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
    pub struct C2RustUnnamed_11 {
        pub luma16x16_dc: [[dctcoef; 16]; 3],
        pub chroma_dc: [[dctcoef; 8]; 2],
        pub luma8x8: [[dctcoef; 64]; 12],
        pub luma4x4: [[dctcoef; 16]; 48],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "375:5"]
    pub struct C2RustUnnamed_12 {
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
        pub ref_pic_list_order: [[C2RustUnnamed_14; 16]; 2],
        pub b_weighted_pred: ::core::ffi::c_int,
        pub weight: [[x264_weight_t; 3]; 32],
        pub i_mmco_remove_from_end: ::core::ffi::c_int,
        pub i_mmco_command_count: ::core::ffi::c_int,
        pub mmco: [C2RustUnnamed_13; 16],
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
    pub struct C2RustUnnamed_13 {
        pub i_difference_of_pic_nums: ::core::ffi::c_int,
        pub i_poc: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "185:5"]
    pub struct C2RustUnnamed_14 {
        pub idc: ::core::ffi::c_int,
        pub arg: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "353:5"]
    pub struct C2RustUnnamed_18 {
        pub ref_0: [[[uint16_t; 33]; 3]; 82],
        pub i4x4_mode: [[uint16_t; 17]; 82],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "291:5"]
    pub struct C2RustUnnamed_19 {
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
    #[c2rust::src_loc = "570:9"]
    pub const FENC_STRIDE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    use super::x264_h::{x264_param_t, x264_nal_t};
    use super::threadpool_h::x264_threadpool_t;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t, pthread_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int32_t, int16_t, int8_t};
    use super::set_h::{x264_sps_t, x264_pps_t};
    use super::cabac_h::x264_cabac_t;
    use super::frame_h::{x264_frame_t, x264_deblock_function_t, x264_sync_frame_list_t};
    use super::x264_ratecontrol_t;
    use super::predict_h::{x264_predict_t, x264_predict8x8_t, x264_predict_8x8_filter_t};
    use super::pixel_h::x264_pixel_function_t;
    use super::mc_h::{x264_mc_functions_t, x264_weight_t};
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::quant_h::x264_quant_function_t;
    use super::bitstream_h::{x264_bitstream_function_t, bs_t};
    use super::internal::BIT_DEPTH;
    extern "C" {
        #[c2rust::src_loc = "138:1"]
        pub fn x264_10_log(
            h: *mut x264_t,
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:32"]
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
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int8_t, int16_t};
    use super::x264_h::{x264_param_t, x264_hrd_t, x264_sei_t};
    use super::common_h::pixel;
    use super::mc_h::x264_weight_t;
    use super::stdint_h::intptr_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:32"]
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
    #[c2rust::src_loc = "205:9"]
    pub const X264_DIRECT_PRED_AUTO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "214:9"]
    pub const X264_RC_CQP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "215:9"]
    pub const X264_RC_CRF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const X264_RC_ABR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const X264_QP_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "218:9"]
    pub const X264_AQ_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "220:9"]
    pub const X264_AQ_AUTOVARIANCE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "221:9"]
    pub const X264_AQ_AUTOVARIANCE_BIASED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "222:9"]
    pub const X264_B_ADAPT_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "224:9"]
    pub const X264_B_ADAPT_TRELLIS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "226:9"]
    pub const X264_WEIGHTP_SIMPLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "232:9"]
    pub const X264_KEYINT_MAX_INFINITE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 30 as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "301:9"]
    pub const X264_NAL_HRD_CBR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint16_t};
    use super::internal::__va_list_tag;
    use super::common_h::x264_t;
    use super::stdint_intn_h::{int64_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "651:36"]
        pub static x264_levels: [x264_level_t; 0];
        #[c2rust::src_loc = "672:10"]
        pub fn x264_param_parse(
            _: *mut x264_param_t,
            name: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "679:10"]
        pub fn x264_param_cleanup(param: *mut x264_param_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:32"]
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
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:32"]
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
    use super::stdint_uintn_h::uint8_t;
    use super::common_h::dctcoef;
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::cabac_h::x264_cabac_t;
    use super::stdint_intn_h::int32_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:32"]
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
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:32"]
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
    use super::common_h::{dctcoef, udctcoef};
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint64_t, uint16_t};
    use super::bitstream_h::x264_run_level_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:32"]
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
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:32"]
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
    pub type C2RustUnnamed_20 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "52:5"]
    pub const PIXEL_2x2: C2RustUnnamed_20 = 11;
    #[c2rust::src_loc = "51:5"]
    pub const PIXEL_2x4: C2RustUnnamed_20 = 10;
    #[c2rust::src_loc = "50:5"]
    pub const PIXEL_2x8: C2RustUnnamed_20 = 9;
    #[c2rust::src_loc = "49:5"]
    pub const PIXEL_4x2: C2RustUnnamed_20 = 8;
    #[c2rust::src_loc = "48:5"]
    pub const PIXEL_4x16: C2RustUnnamed_20 = 7;
    #[c2rust::src_loc = "45:5"]
    pub const PIXEL_4x4: C2RustUnnamed_20 = 6;
    #[c2rust::src_loc = "44:5"]
    pub const PIXEL_4x8: C2RustUnnamed_20 = 5;
    #[c2rust::src_loc = "43:5"]
    pub const PIXEL_8x4: C2RustUnnamed_20 = 4;
    #[c2rust::src_loc = "42:5"]
    pub const PIXEL_8x8: C2RustUnnamed_20 = 3;
    #[c2rust::src_loc = "41:5"]
    pub const PIXEL_8x16: C2RustUnnamed_20 = 2;
    #[c2rust::src_loc = "40:5"]
    pub const PIXEL_16x8: C2RustUnnamed_20 = 1;
    #[c2rust::src_loc = "39:5"]
    pub const PIXEL_16x16: C2RustUnnamed_20 = 0;
    use super::common_h::pixel;
    use super::stdint_h::intptr_t;
    use super::stdint_uintn_h::{uint64_t, uint16_t};
    use super::stdint_intn_h::int16_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:32"]
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
    use super::common_h::pixel;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:32"]
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
        pub crop: C2RustUnnamed_17,
        pub b_vui: ::core::ffi::c_int,
        pub vui: C2RustUnnamed_15,
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
    pub struct C2RustUnnamed_15 {
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
        pub hrd: C2RustUnnamed_16,
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
    pub struct C2RustUnnamed_16 {
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
    pub struct C2RustUnnamed_17 {
        pub i_left: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:32"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:32"]
pub mod base_h {
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
    #[inline(always)]
    #[c2rust::src_loc = "218:1"]
    pub unsafe extern "C" fn x264_exp2fix8(
        mut x: ::core::ffi::c_float,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = (x * (-64.0f32 / 6.0f32) + 512.5f32)
            as ::core::ffi::c_int;
        if i < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if i > 1023 as ::core::ffi::c_int {
            return 0xffff as ::core::ffi::c_int;
        }
        return (x264_exp2_lut[(i & 63 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int + 256 as ::core::ffi::c_int)
            << (i >> 6 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    }
    #[inline(always)]
    #[c2rust::src_loc = "226:1"]
    pub unsafe extern "C" fn x264_log2(mut x: uint32_t) -> ::core::ffi::c_float {
        let mut lz: ::core::ffi::c_int = x.leading_zeros() as i32;
        return x264_log2_lut[(x << lz >> 24 as ::core::ffi::c_int & 0x7f as uint32_t)
            as usize] + x264_log2_lz_lut[lz as usize];
    }
    use super::tables_h::{x264_exp2_lut, x264_log2_lut, x264_log2_lz_lut};
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
    use super::stdint_intn_h::int64_t;
    use super::x264_h::x264_param_t;
    extern "C" {
        #[c2rust::src_loc = "272:10"]
        pub fn x264_reduce_fraction64(n: *mut uint64_t, d: *mut uint64_t);
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "283:10"]
        pub fn x264_slurp_file(
            filename: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "291:10"]
        pub fn x264_param2string(
            p: *mut x264_param_t,
            b_res: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:32"]
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
    use super::common_h::x264_t;
    use super::frame_h::x264_frame_t;
    extern "C" {
        #[c2rust::src_loc = "330:1"]
        pub fn x264_10_prefetch_fenc(
            h: *mut x264_t,
            fenc: *mut x264_frame_t,
            i_mb_x: ::core::ffi::c_int,
            i_mb_y: ::core::ffi::c_int,
        );
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "160:1"]
        pub fn rename(
            __old: *const ::core::ffi::c_char,
            __new: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "360:1"]
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "368:1"]
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "728:1"]
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __stream: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "735:1"]
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __s: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:32"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:32"]
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
}
#[c2rust::header_src = "/usr/include/string.h:0"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
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
        #[c2rust::src_loc = "149:1"]
        pub fn strcat(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
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
        #[c2rust::src_loc = "293:1"]
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "350:1"]
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "366:1"]
        pub fn strtok_r(
            __s: *mut ::core::ffi::c_char,
            __delim: *const ::core::ffi::c_char,
            __save_ptr: *mut *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:32"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "38:22"]
        pub static x264_exp2_lut: [uint8_t; 64];
        #[c2rust::src_loc = "39:22"]
        pub static x264_log2_lut: [::core::ffi::c_float; 128];
        #[c2rust::src_loc = "40:22"]
        pub static x264_log2_lz_lut: [::core::ffi::c_float; 32];
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/limits.h:32"]
pub mod limits_h {
    #[c2rust::src_loc = "50:9"]
    pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
    use super::internal::__INT_MAX__;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/ratecontrol.h:32"]
pub mod ratecontrol_h {
    use super::common_h::x264_t;
    use super::x264_h::x264_param_t;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub fn x264_10_encoder_reconfig_apply(
            h: *mut x264_t,
            param: *mut x264_param_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "79:1"]
        pub fn x264_10_rc_analyse_slice(h: *mut x264_t) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "980:1"]
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:32"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "117:1"]
        pub fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "126:1"]
        pub fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "170:1"]
        pub fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "177:1"]
        pub fn pow(
            __x: ::core::ffi::c_double,
            __y: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "180:1"]
        pub fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "213:1"]
        pub fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "216:1"]
        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "170:1"]
        pub fn log2f(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
        #[c2rust::src_loc = "177:1"]
        pub fn powf(
            __x: ::core::ffi::c_float,
            __y: ::core::ffi::c_float,
        ) -> ::core::ffi::c_float;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:32"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:32"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/assert.h:32"]
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
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::atomic_wide_counter_h::{__atomic_wide_counter, C2RustUnnamed};
pub use self::thread_shared_types_h::{
    __pthread_internal_list, __pthread_list_t, __pthread_cond_s,
};
pub use self::struct_mutex_h::__pthread_mutex_s;
pub use self::pthreadtypes_h::{pthread_t, pthread_mutex_t, pthread_cond_t};
pub use self::common_h::{
    x264_t, x264_lookahead_t, pixel, dctcoef, udctcoef, C2RustUnnamed_6,
    x264_frame_stat_t, C2RustUnnamed_8, C2RustUnnamed_9, C2RustUnnamed_10,
    x264_left_table_t, C2RustUnnamed_11, C2RustUnnamed_12, x264_slice_header_t,
    C2RustUnnamed_13, C2RustUnnamed_14, C2RustUnnamed_18, C2RustUnnamed_19, QP_BD_OFFSET,
    QP_MAX_SPEC, QP_MAX, FENC_STRIDE, x264_10_log,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t,
};
pub use self::x264_h::{
    x264_sei_t, x264_sei_payload_t, x264_hrd_t, x264_param_t, x264_nal_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, x264_zone_t,
    C2RustUnnamed_4, C2RustUnnamed_5, x264_level_t, X264_DIRECT_PRED_AUTO, X264_RC_CQP,
    X264_RC_CRF, X264_RC_ABR, X264_QP_AUTO, X264_AQ_NONE, X264_AQ_AUTOVARIANCE,
    X264_AQ_AUTOVARIANCE_BIASED, X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS,
    X264_WEIGHTP_SIMPLE, X264_KEYINT_MAX_INFINITE, X264_TYPE_AUTO, X264_TYPE_IDR,
    X264_TYPE_I, X264_TYPE_P, X264_TYPE_BREF, X264_TYPE_B, X264_TYPE_KEYFRAME,
    X264_LOG_ERROR, X264_LOG_WARNING, X264_LOG_INFO, X264_LOG_DEBUG, X264_NAL_HRD_CBR,
    x264_levels, x264_param_parse, x264_param_cleanup,
};
pub use self::mc_h::{x264_weight_t, weight_fn_t, x264_mc_functions_t};
pub use self::bitstream_h::{x264_bitstream_function_t, x264_run_level_t, bs_t, bs_s};
pub use self::cabac_h::x264_cabac_t;
pub use self::quant_h::x264_quant_function_t;
pub use self::dct_h::{x264_zigzag_function_t, x264_dct_function_t};
pub use self::pixel_h::{
    x264_pixel_function_t, x264_pixel_cmp_x4_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_t,
    C2RustUnnamed_20, PIXEL_2x2, PIXEL_2x4, PIXEL_2x8, PIXEL_4x2, PIXEL_4x16, PIXEL_4x4,
    PIXEL_4x8, PIXEL_8x4, PIXEL_8x8, PIXEL_8x16, PIXEL_16x8, PIXEL_16x16,
};
pub use self::predict_h::{x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_15, C2RustUnnamed_16, C2RustUnnamed_17,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::base_h::{
    profile_e, PROFILE_HIGH444_PREDICTIVE, PROFILE_HIGH422, PROFILE_HIGH10, PROFILE_HIGH,
    PROFILE_MAIN, PROFILE_BASELINE, chroma_format_e, CHROMA_444, CHROMA_422, CHROMA_420,
    CHROMA_400, slice_type_e, SLICE_TYPE_I, SLICE_TYPE_B, SLICE_TYPE_P,
    slice_type_to_char, x264_clip3, x264_clip3f, x264_exp2fix8, x264_log2,
    x264_reduce_fraction64, x264_malloc, x264_free, x264_slurp_file, x264_param2string,
};
pub use self::macroblock_h::{
    mb_class_e, X264_MBTYPE_MAX, B_SKIP, B_8x8, B_BI_BI, B_BI_L1, B_BI_L0, B_L1_BI,
    B_L1_L1, B_L1_L0, B_L0_BI, B_L0_L1, B_L0_L0, B_DIRECT, P_SKIP, P_8x8, P_L0, I_PCM,
    I_16x16, I_8x8, I_4x4, x264_10_prefetch_fenc,
};
use self::stdio_h::{
    rename, fclose, fopen, fprintf, sprintf, sscanf, fread, fwrite, fileno,
};
use self::stat_h::fstat;
pub use self::osdep_h::x264_is_regular_file;
use self::string_h::{
    memcpy, memset, strcpy, strcat, strcmp, strncmp, strchr, strcspn, strstr, strtok_r,
    strlen,
};
use self::tables_h::{x264_exp2_lut, x264_log2_lut, x264_log2_lz_lut};
pub use self::limits_h::INT_MAX;
use self::ratecontrol_h::{x264_10_encoder_reconfig_apply, x264_10_rc_analyse_slice};
use self::stdlib_h::abs;
use self::mathcalls_h::{exp, log, log2, pow, sqrt, ceil, fabs, log2f, powf};
pub use self::__stddef_null_h::NULL;
pub use self::bits_stat_h::__S_IFMT;
use self::assert_h::__assert_fail;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "72:8"]
pub struct x264_ratecontrol_t {
    pub b_abr: ::core::ffi::c_int,
    pub b_2pass: ::core::ffi::c_int,
    pub b_vbv: ::core::ffi::c_int,
    pub b_vbv_min_rate: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_double,
    pub bitrate: ::core::ffi::c_double,
    pub rate_tolerance: ::core::ffi::c_double,
    pub qcompress: ::core::ffi::c_double,
    pub nmb: ::core::ffi::c_int,
    pub qp_constant: [::core::ffi::c_int; 3],
    pub rce: *mut ratecontrol_entry_t,
    pub qpm: ::core::ffi::c_float,
    pub qpa_rc: ::core::ffi::c_float,
    pub qpa_rc_prev: ::core::ffi::c_float,
    pub qpa_aq: ::core::ffi::c_int,
    pub qpa_aq_prev: ::core::ffi::c_int,
    pub qp_novbv: ::core::ffi::c_float,
    pub buffer_size: ::core::ffi::c_double,
    pub buffer_fill_final: int64_t,
    pub buffer_fill_final_min: int64_t,
    pub buffer_fill: ::core::ffi::c_double,
    pub buffer_rate: ::core::ffi::c_double,
    pub vbv_max_rate: ::core::ffi::c_double,
    pub pred: *mut predictor_t,
    pub single_frame_vbv: ::core::ffi::c_int,
    pub rate_factor_max_increment: ::core::ffi::c_float,
    pub last_satd: ::core::ffi::c_int,
    pub last_rceq: ::core::ffi::c_double,
    pub cplxr_sum: ::core::ffi::c_double,
    pub expected_bits_sum: ::core::ffi::c_double,
    pub filler_bits_sum: int64_t,
    pub wanted_bits_window: ::core::ffi::c_double,
    pub cbr_decay: ::core::ffi::c_double,
    pub short_term_cplxsum: ::core::ffi::c_double,
    pub short_term_cplxcount: ::core::ffi::c_double,
    pub rate_factor_constant: ::core::ffi::c_double,
    pub ip_offset: ::core::ffi::c_double,
    pub pb_offset: ::core::ffi::c_double,
    pub p_stat_file_out: *mut FILE,
    pub psz_stat_file_tmpname: *mut ::core::ffi::c_char,
    pub p_mbtree_stat_file_out: *mut FILE,
    pub psz_mbtree_stat_file_tmpname: *mut ::core::ffi::c_char,
    pub psz_mbtree_stat_file_name: *mut ::core::ffi::c_char,
    pub p_mbtree_stat_file_in: *mut FILE,
    pub num_entries: ::core::ffi::c_int,
    pub entry: *mut ratecontrol_entry_t,
    pub entry_out: *mut *mut ratecontrol_entry_t,
    pub last_qscale: ::core::ffi::c_double,
    pub last_qscale_for: [::core::ffi::c_double; 3],
    pub last_non_b_pict_type: ::core::ffi::c_int,
    pub accum_p_qp: ::core::ffi::c_double,
    pub accum_p_norm: ::core::ffi::c_double,
    pub last_accum_p_norm: ::core::ffi::c_double,
    pub lmin: [::core::ffi::c_double; 3],
    pub lmax: [::core::ffi::c_double; 3],
    pub lstep: ::core::ffi::c_double,
    pub mbtree: C2RustUnnamed_7,
    pub frame_size_estimated: ::core::ffi::c_float,
    pub bits_so_far: ::core::ffi::c_float,
    pub frame_size_maximum: ::core::ffi::c_double,
    pub frame_size_planned: ::core::ffi::c_double,
    pub slice_size_planned: ::core::ffi::c_double,
    pub row_pred: *mut predictor_t,
    pub row_preds: [[predictor_t; 2]; 3],
    pub pred_b_from_p: *mut predictor_t,
    pub bframes: ::core::ffi::c_int,
    pub bframe_bits: ::core::ffi::c_int,
    pub i_zones: ::core::ffi::c_int,
    pub zones: *mut x264_zone_t,
    pub prev_zone: *mut x264_zone_t,
    pub initial_cpb_removal_delay: ::core::ffi::c_int,
    pub initial_cpb_removal_delay_offset: ::core::ffi::c_int,
    pub nrt_first_access_unit: ::core::ffi::c_double,
    pub previous_cpb_final_arrival_time: ::core::ffi::c_double,
    pub hrd_multiply_denom: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "63:9"]
pub struct predictor_t {
    pub coeff_min: ::core::ffi::c_float,
    pub coeff: ::core::ffi::c_float,
    pub count: ::core::ffi::c_float,
    pub decay: ::core::ffi::c_float,
    pub offset: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "140:5"]
pub struct C2RustUnnamed_7 {
    pub qp_buffer: [*mut uint16_t; 2],
    pub qpbuf_pos: ::core::ffi::c_int,
    pub src_mb_count: ::core::ffi::c_int,
    pub rescale_enabled: ::core::ffi::c_int,
    pub scale_buffer: [*mut ::core::ffi::c_float; 2],
    pub filtersize: [::core::ffi::c_int; 2],
    pub coeffs: [*mut ::core::ffi::c_float; 2],
    pub pos: [*mut ::core::ffi::c_int; 2],
    pub srcdim: [::core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "36:9"]
pub struct ratecontrol_entry_t {
    pub pict_type: ::core::ffi::c_int,
    pub frame_type: ::core::ffi::c_int,
    pub kept_as_ref: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_double,
    pub mv_bits: ::core::ffi::c_int,
    pub tex_bits: ::core::ffi::c_int,
    pub misc_bits: ::core::ffi::c_int,
    pub expected_bits: ::core::ffi::c_double,
    pub expected_vbv: ::core::ffi::c_double,
    pub new_qscale: ::core::ffi::c_double,
    pub new_qp: ::core::ffi::c_float,
    pub i_count: ::core::ffi::c_int,
    pub p_count: ::core::ffi::c_int,
    pub s_count: ::core::ffi::c_int,
    pub blurred_complexity: ::core::ffi::c_float,
    pub direct_mode: ::core::ffi::c_char,
    pub weight: [[int16_t; 2]; 3],
    pub i_weight_denom: [int16_t; 2],
    pub refcount: [::core::ffi::c_int; 16],
    pub refs: ::core::ffi::c_int,
    pub i_duration: int64_t,
    pub i_cpb_duration: int64_t,
    pub out_num: ::core::ffi::c_int,
}
#[inline]
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn qp2qscale(mut qp: ::core::ffi::c_float) -> ::core::ffi::c_float {
    return 0.85f32
        * powf(2.0f32, (qp - (12.0f32 + QP_BD_OFFSET as ::core::ffi::c_float)) / 6.0f32);
}
#[inline]
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn qscale2qp(
    mut qscale: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return 12.0f32 + QP_BD_OFFSET as ::core::ffi::c_float
        + 6.0f32 * log2f(qscale / 0.85f32);
}
#[inline]
#[c2rust::src_loc = "216:1"]
unsafe extern "C" fn qscale2bits(
    mut rce: *mut ratecontrol_entry_t,
    mut qscale: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if qscale < 0.1f64 {
        qscale = 0.1f64;
    }
    return ((*rce).tex_bits as ::core::ffi::c_double + 0.1f64)
        * pow((*rce).qscale / qscale, 1.1f64)
        + (*rce).mv_bits as ::core::ffi::c_double
            * pow(
                (if (*rce).qscale > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                    (*rce).qscale
                } else {
                    1 as ::core::ffi::c_int as ::core::ffi::c_double
                })
                    / (if qscale > 1 as ::core::ffi::c_int as ::core::ffi::c_double {
                        qscale
                    } else {
                        1 as ::core::ffi::c_int as ::core::ffi::c_double
                    }),
                0.5f64,
            ) + (*rce).misc_bits as ::core::ffi::c_double;
}
#[inline(always)]
#[c2rust::src_loc = "225:1"]
unsafe extern "C" fn ac_energy_var(
    mut sum_ssd: uint64_t,
    mut shift: ::core::ffi::c_int,
    mut frame: *mut x264_frame_t,
    mut i: ::core::ffi::c_int,
    mut b_store: ::core::ffi::c_int,
) -> uint32_t {
    let mut sum: uint32_t = sum_ssd as uint32_t;
    let mut ssd: uint32_t = (sum_ssd >> 32 as ::core::ffi::c_int) as uint32_t;
    if b_store != 0 {
        (*frame).i_pixel_sum[i as usize] = (*frame)
            .i_pixel_sum[i as usize]
            .wrapping_add(sum);
        (*frame).i_pixel_ssd[i as usize] = (*frame)
            .i_pixel_ssd[i as usize]
            .wrapping_add(ssd as uint64_t);
    }
    return (ssd as uint64_t)
        .wrapping_sub((sum as uint64_t).wrapping_mul(sum as uint64_t) >> shift)
        as uint32_t;
}
#[inline(always)]
#[c2rust::src_loc = "237:1"]
unsafe extern "C" fn ac_energy_plane(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut frame: *mut x264_frame_t,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_field: ::core::ffi::c_int,
    mut b_store: ::core::ffi::c_int,
) -> uint32_t {
    let mut height: ::core::ffi::c_int = if b_chroma != 0 {
        16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as ::core::ffi::c_int
    };
    let mut stride: ::core::ffi::c_int = (*frame).i_stride[i as usize];
    let mut offset: ::core::ffi::c_int = if b_field != 0 {
        16 as ::core::ffi::c_int * mb_x
            + height * (mb_y & !(1 as ::core::ffi::c_int)) * stride
            + (mb_y & 1 as ::core::ffi::c_int) * stride
    } else {
        16 as ::core::ffi::c_int * mb_x + height * mb_y * stride
    };
    stride <<= b_field;
    if b_chroma != 0 {
        let mut pix: [pixel; 256] = [0; 256];
        let mut chromapix: ::core::ffi::c_int = (*h)
            .luma2chroma_pixel[PIXEL_16x16 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int;
        let mut shift: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            - (*h).mb.chroma_v_shift;
        (*h)
            .mc
            .load_deinterleave_chroma_fenc
            .expect(
                "non-null function pointer",
            )(
            pix.as_mut_ptr(),
            (*frame).plane[1 as ::core::ffi::c_int as usize].offset(offset as isize),
            stride as intptr_t,
            height,
        );
        return ac_energy_var(
                (*h)
                    .pixf
                    .var[chromapix as usize]
                    .expect(
                        "non-null function pointer",
                    )(pix.as_mut_ptr(), FENC_STRIDE as intptr_t),
                shift,
                frame,
                1 as ::core::ffi::c_int,
                b_store,
            )
            .wrapping_add(
                ac_energy_var(
                    (*h)
                        .pixf
                        .var[chromapix as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        pix
                            .as_mut_ptr()
                            .offset((FENC_STRIDE / 2 as ::core::ffi::c_int) as isize),
                        FENC_STRIDE as intptr_t,
                    ),
                    shift,
                    frame,
                    2 as ::core::ffi::c_int,
                    b_store,
                ),
            );
    } else {
        return ac_energy_var(
            (*h)
                .pixf
                .var[PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*frame).plane[i as usize].offset(offset as isize),
                stride as intptr_t,
            ),
            8 as ::core::ffi::c_int,
            frame,
            i,
            b_store,
        )
    };
}
#[inline(never)]
#[c2rust::src_loc = "260:1"]
unsafe extern "C" fn ac_energy_mb(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut frame: *mut x264_frame_t,
) -> uint32_t {
    let mut var: uint32_t = 0;
    x264_10_prefetch_fenc(h, frame, mb_x, mb_y);
    if (*h).mb.b_adaptive_mbaff != 0 {
        let mut var_interlaced: uint32_t = 0;
        let mut var_progressive: uint32_t = 0;
        var_interlaced = ac_energy_plane(
            h,
            mb_x,
            mb_y,
            frame,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        var_progressive = ac_energy_plane(
            h,
            mb_x,
            mb_y,
            frame,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as ::core::ffi::c_int
        {
            var_interlaced = var_interlaced
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ),
                );
            var_progressive = var_progressive
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ),
                );
            var_interlaced = var_interlaced
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ),
                );
            var_progressive = var_progressive
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ),
                );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            var_interlaced = var_interlaced
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ),
                );
            var_progressive = var_progressive
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ),
                );
        }
        var = if var_interlaced < var_progressive {
            var_interlaced
        } else {
            var_progressive
        };
    } else {
        var = ac_energy_plane(
            h,
            mb_x,
            mb_y,
            frame,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*h).param.b_interlaced,
            1 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as ::core::ffi::c_int
        {
            var = var
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        (*h).param.b_interlaced,
                        1 as ::core::ffi::c_int,
                    ),
                );
            var = var
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        (*h).param.b_interlaced,
                        1 as ::core::ffi::c_int,
                    ),
                );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            var = var
                .wrapping_add(
                    ac_energy_plane(
                        h,
                        mb_x,
                        mb_y,
                        frame,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        (*h).param.b_interlaced,
                        1 as ::core::ffi::c_int,
                    ),
                );
        }
    }
    return var;
}
#[no_mangle]
#[c2rust::src_loc = "304:1"]
pub unsafe extern "C" fn x264_10_adaptive_quant_frame(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut quant_offsets: *mut ::core::ffi::c_float,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        (*frame).i_pixel_sum[i as usize] = 0 as uint32_t;
        (*frame).i_pixel_ssd[i as usize] = 0 as uint64_t;
        i += 1;
    }
    if (*h).param.rc.i_aq_mode == X264_AQ_NONE
        || (*h).param.rc.f_aq_strength == 0 as ::core::ffi::c_int as ::core::ffi::c_float
    {
        if (*h).param.rc.i_aq_mode != 0
            && (*h).param.rc.f_aq_strength
                == 0 as ::core::ffi::c_int as ::core::ffi::c_float
        {
            if !quant_offsets.is_null() {
                let mut mb_xy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_xy < (*h).mb.i_mb_count {
                    let ref mut fresh6 = *(*frame).f_qp_offset_aq.offset(mb_xy as isize);
                    *fresh6 = *quant_offsets.offset(mb_xy as isize);
                    *(*frame).f_qp_offset.offset(mb_xy as isize) = *fresh6;
                    mb_xy += 1;
                }
                if (*h).frames.b_have_lowres != 0 {
                    let mut mb_xy_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while mb_xy_0 < (*h).mb.i_mb_count {
                        *(*frame).i_inv_qscale_factor.offset(mb_xy_0 as isize) = x264_exp2fix8(
                            *(*frame).f_qp_offset.offset(mb_xy_0 as isize),
                        ) as uint16_t;
                        mb_xy_0 += 1;
                    }
                }
            } else {
                memset(
                    (*frame).f_qp_offset as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ((*h).mb.i_mb_count as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                        ),
                );
                memset(
                    (*frame).f_qp_offset_aq as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ((*h).mb.i_mb_count as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                        ),
                );
                if (*h).frames.b_have_lowres != 0 {
                    let mut mb_xy_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while mb_xy_1 < (*h).mb.i_mb_count {
                        *(*frame).i_inv_qscale_factor.offset(mb_xy_1 as isize) = 256
                            as uint16_t;
                        mb_xy_1 += 1;
                    }
                }
            }
        }
        if (*h).param.analyse.i_weighted_pred != 0 {
            let mut mb_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while mb_y < (*h).mb.i_mb_height {
                let mut mb_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_x < (*h).mb.i_mb_width {
                    ac_energy_mb(h, mb_x, mb_y, frame);
                    mb_x += 1;
                }
                mb_y += 1;
            }
        } else {
            return
        }
    } else {
        let mut strength: ::core::ffi::c_float = 0.;
        let mut avg_adj: ::core::ffi::c_float = 0.0f32;
        let mut bias_strength: ::core::ffi::c_float = 0.0f32;
        if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE
            || (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE_BIASED
        {
            let mut bit_depth_correction: ::core::ffi::c_float = 1.0f32
                / ((1 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int * (BIT_DEPTH - 8 as ::core::ffi::c_int))
                    as ::core::ffi::c_float;
            let mut avg_adj_pow2: ::core::ffi::c_float = 0.0f32;
            let mut mb_y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while mb_y_0 < (*h).mb.i_mb_height {
                let mut mb_x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while mb_x_0 < (*h).mb.i_mb_width {
                    let mut energy: uint32_t = ac_energy_mb(h, mb_x_0, mb_y_0, frame);
                    let mut qp_adj: ::core::ffi::c_float = powf(
                        energy as ::core::ffi::c_float * bit_depth_correction
                            + 1 as ::core::ffi::c_int as ::core::ffi::c_float,
                        0.125f32,
                    );
                    *(*frame)
                        .f_qp_offset
                        .offset((mb_x_0 + mb_y_0 * (*h).mb.i_mb_stride) as isize) = qp_adj;
                    avg_adj += qp_adj;
                    avg_adj_pow2 += qp_adj * qp_adj;
                    mb_x_0 += 1;
                }
                mb_y_0 += 1;
            }
            avg_adj /= (*h).mb.i_mb_count as ::core::ffi::c_float;
            avg_adj_pow2 /= (*h).mb.i_mb_count as ::core::ffi::c_float;
            strength = (*h).param.rc.f_aq_strength * avg_adj;
            avg_adj = avg_adj - 0.5f32 * (avg_adj_pow2 - 14.0f32) / avg_adj;
            bias_strength = (*h).param.rc.f_aq_strength;
        } else {
            strength = (*h).param.rc.f_aq_strength * 1.0397f32;
        }
        let mut mb_y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while mb_y_1 < (*h).mb.i_mb_height {
            let mut mb_x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while mb_x_1 < (*h).mb.i_mb_width {
                let mut qp_adj_0: ::core::ffi::c_float = 0.;
                let mut mb_xy_2: ::core::ffi::c_int = mb_x_1
                    + mb_y_1 * (*h).mb.i_mb_stride;
                if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE_BIASED {
                    qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                    qp_adj_0 = strength * (qp_adj_0 - avg_adj)
                        + bias_strength * (1.0f32 - 14.0f32 / (qp_adj_0 * qp_adj_0));
                } else if (*h).param.rc.i_aq_mode == X264_AQ_AUTOVARIANCE {
                    qp_adj_0 = *(*frame).f_qp_offset.offset(mb_xy_2 as isize);
                    qp_adj_0 = strength * (qp_adj_0 - avg_adj);
                } else {
                    let mut energy_0: uint32_t = ac_energy_mb(h, mb_x_1, mb_y_1, frame);
                    qp_adj_0 = strength
                        * (x264_log2(
                            (if energy_0 > 1 as uint32_t {
                                energy_0
                            } else {
                                1 as uint32_t
                            }),
                        )
                            - (14.427f32
                                + (2 as ::core::ffi::c_int
                                    * (BIT_DEPTH - 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_float));
                }
                if !quant_offsets.is_null() {
                    qp_adj_0 += *quant_offsets.offset(mb_xy_2 as isize);
                }
                let ref mut fresh7 = *(*frame).f_qp_offset_aq.offset(mb_xy_2 as isize);
                *fresh7 = qp_adj_0;
                *(*frame).f_qp_offset.offset(mb_xy_2 as isize) = *fresh7;
                if (*h).frames.b_have_lowres != 0 {
                    *(*frame).i_inv_qscale_factor.offset(mb_xy_2 as isize) = x264_exp2fix8(
                        qp_adj_0,
                    ) as uint16_t;
                }
                mb_x_1 += 1;
            }
            mb_y_1 += 1;
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 3 as ::core::ffi::c_int {
        let mut ssd: uint64_t = (*frame).i_pixel_ssd[i_0 as usize];
        let mut sum: uint64_t = (*frame).i_pixel_sum[i_0 as usize] as uint64_t;
        let mut width: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb.i_mb_width
            >> (i_0 != 0 && (*h).mb.chroma_h_shift != 0) as ::core::ffi::c_int;
        let mut height: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (*h).mb.i_mb_height
            >> (i_0 != 0 && (*h).mb.chroma_v_shift != 0) as ::core::ffi::c_int;
        (*frame).i_pixel_ssd[i_0 as usize] = ssd
            .wrapping_sub(
                sum
                    .wrapping_mul(sum)
                    .wrapping_add((width * height / 2 as ::core::ffi::c_int) as uint64_t)
                    .wrapping_div((width * height) as uint64_t),
            );
        i_0 += 1;
    }
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn macroblock_tree_rescale_init(
    mut h: *mut x264_t,
    mut rc: *mut x264_ratecontrol_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut srcdim: [::core::ffi::c_float; 2] = [
        (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_float
            / 16.0f32,
        (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_float
            / 16.0f32,
    ];
    let mut dstdim: [::core::ffi::c_float; 2] = [
        (*h).param.i_width as ::core::ffi::c_float / 16.0f32,
        (*h).param.i_height as ::core::ffi::c_float / 16.0f32,
    ];
    let mut srcdimi: [::core::ffi::c_int; 2] = [
        ceil(srcdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
            as ::core::ffi::c_int,
        ceil(srcdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
            as ::core::ffi::c_int,
    ];
    let mut dstdimi: [::core::ffi::c_int; 2] = [
        ceil(dstdim[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
            as ::core::ffi::c_int,
        ceil(dstdim[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double)
            as ::core::ffi::c_int,
    ];
    if (*h).param.b_interlaced != 0 || (*h).param.b_fake_interlaced != 0 {
        srcdimi[1 as ::core::ffi::c_int as usize] = srcdimi[1 as ::core::ffi::c_int
            as usize] + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
        dstdimi[1 as ::core::ffi::c_int as usize] = dstdimi[1 as ::core::ffi::c_int
            as usize] + 1 as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
    }
    (*rc).mbtree.src_mb_count = srcdimi[0 as ::core::ffi::c_int as usize]
        * srcdimi[1 as ::core::ffi::c_int as usize];
    (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize] = x264_malloc(
        ((*rc).mbtree.src_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as int64_t,
    ) as *mut uint16_t;
    if !(*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize].is_null() {
        if (*h).param.i_bframe_pyramid != 0 && (*h).param.rc.b_stat_read != 0 {
            (*rc).mbtree.qp_buffer[1 as ::core::ffi::c_int as usize] = x264_malloc(
                ((*rc).mbtree.src_mb_count as usize)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as int64_t,
            ) as *mut uint16_t;
            if (*rc).mbtree.qp_buffer[1 as ::core::ffi::c_int as usize].is_null() {
                current_block = 9112284078615121668;
            } else {
                current_block = 5399440093318478209;
            }
        } else {
            current_block = 5399440093318478209;
        }
        match current_block {
            9112284078615121668 => {}
            _ => {
                (*rc).mbtree.qpbuf_pos = -(1 as ::core::ffi::c_int);
                if srcdimi[0 as ::core::ffi::c_int as usize]
                    == dstdimi[0 as ::core::ffi::c_int as usize]
                    && srcdimi[1 as ::core::ffi::c_int as usize]
                        == dstdimi[1 as ::core::ffi::c_int as usize]
                {
                    return 0 as ::core::ffi::c_int;
                }
                (*rc).mbtree.rescale_enabled = 1 as ::core::ffi::c_int;
                (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize] = x264_malloc(
                    ((srcdimi[0 as ::core::ffi::c_int as usize]
                        * srcdimi[1 as ::core::ffi::c_int as usize]) as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                        ) as int64_t,
                ) as *mut ::core::ffi::c_float;
                if !(*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize].is_null()
                {
                    (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize] = x264_malloc(
                        ((dstdimi[0 as ::core::ffi::c_int as usize]
                            * srcdimi[1 as ::core::ffi::c_int as usize]) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                            ) as int64_t,
                    ) as *mut ::core::ffi::c_float;
                    if !(*rc)
                        .mbtree
                        .scale_buffer[1 as ::core::ffi::c_int as usize]
                        .is_null()
                    {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        loop {
                            if !(i < 2 as ::core::ffi::c_int) {
                                current_block = 3160140712158701372;
                                break;
                            }
                            if srcdim[i as usize] > dstdim[i as usize] {
                                (*rc).mbtree.filtersize[i as usize] = 1
                                    as ::core::ffi::c_int
                                    + (2 as ::core::ffi::c_int * srcdimi[i as usize]
                                        + dstdimi[i as usize] - 1 as ::core::ffi::c_int)
                                        / dstdimi[i as usize];
                            } else {
                                (*rc).mbtree.filtersize[i as usize] = 3
                                    as ::core::ffi::c_int;
                            }
                            (*rc).mbtree.coeffs[i as usize] = x264_malloc(
                                (((*rc).mbtree.filtersize[i as usize] * dstdimi[i as usize])
                                    as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                                    ) as int64_t,
                            ) as *mut ::core::ffi::c_float;
                            if (*rc).mbtree.coeffs[i as usize].is_null() {
                                current_block = 9112284078615121668;
                                break;
                            }
                            (*rc).mbtree.pos[i as usize] = x264_malloc(
                                (dstdimi[i as usize] as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                    ) as int64_t,
                            ) as *mut ::core::ffi::c_int;
                            if (*rc).mbtree.pos[i as usize].is_null() {
                                current_block = 9112284078615121668;
                                break;
                            }
                            let mut inc: ::core::ffi::c_float = srcdim[i as usize]
                                / dstdim[i as usize];
                            let mut dmul: ::core::ffi::c_float = if inc > 1.0f32 {
                                dstdim[i as usize] / srcdim[i as usize]
                            } else {
                                1.0f32
                            };
                            let mut dstinsrc: ::core::ffi::c_float = 0.5f32 * inc
                                - 0.5f32;
                            let mut filtersize: ::core::ffi::c_int = (*rc)
                                .mbtree
                                .filtersize[i as usize];
                            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while j < dstdimi[i as usize] {
                                let mut pos: ::core::ffi::c_int = (dstinsrc
                                    - (filtersize as ::core::ffi::c_float - 2.0f32) * 0.5f32)
                                    as ::core::ffi::c_int;
                                let mut sum: ::core::ffi::c_float = 0.0f32;
                                *(*rc).mbtree.pos[i as usize].offset(j as isize) = pos;
                                let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while k < filtersize {
                                    let mut d: ::core::ffi::c_float = (fabs(
                                        ((pos + k) as ::core::ffi::c_float - dstinsrc)
                                            as ::core::ffi::c_double,
                                    ) * dmul as ::core::ffi::c_double) as ::core::ffi::c_float;
                                    let mut coeff: ::core::ffi::c_float = if 1.0f32 - d
                                        > 0 as ::core::ffi::c_int as ::core::ffi::c_float
                                    {
                                        1.0f32 - d
                                    } else {
                                        0 as ::core::ffi::c_int as ::core::ffi::c_float
                                    };
                                    *(*rc)
                                        .mbtree
                                        .coeffs[i as usize]
                                        .offset((j * filtersize + k) as isize) = coeff;
                                    sum += coeff;
                                    k += 1;
                                }
                                sum = 1.0f32 / sum;
                                let mut k_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while k_0 < filtersize {
                                    *(*rc)
                                        .mbtree
                                        .coeffs[i as usize]
                                        .offset((j * filtersize + k_0) as isize) *= sum;
                                    k_0 += 1;
                                }
                                dstinsrc += inc;
                                j += 1;
                            }
                            i += 1;
                        }
                        match current_block {
                            9112284078615121668 => {}
                            _ => {
                                (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] = srcdimi[0
                                    as ::core::ffi::c_int as usize];
                                (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] = srcdimi[1
                                    as ::core::ffi::c_int as usize];
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn macroblock_tree_rescale_destroy(mut rc: *mut x264_ratecontrol_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        x264_free((*rc).mbtree.qp_buffer[i as usize] as *mut ::core::ffi::c_void);
        x264_free((*rc).mbtree.scale_buffer[i as usize] as *mut ::core::ffi::c_void);
        x264_free((*rc).mbtree.coeffs[i as usize] as *mut ::core::ffi::c_void);
        x264_free((*rc).mbtree.pos[i as usize] as *mut ::core::ffi::c_void);
        i += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "501:1"]
unsafe extern "C" fn tapfilter(
    mut src: *mut ::core::ffi::c_float,
    mut pos: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut coeff: *mut ::core::ffi::c_float,
    mut filtersize: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut sum: ::core::ffi::c_float = 0.0f32;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < filtersize {
        sum
            += *src
                .offset(
                    (x264_clip3(
                        pos,
                        0 as ::core::ffi::c_int,
                        max - 1 as ::core::ffi::c_int,
                    ) * stride) as isize,
                ) * *coeff.offset(i as isize);
        i += 1;
        pos += 1;
    }
    return sum;
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn macroblock_tree_rescale(
    mut h: *mut x264_t,
    mut rc: *mut x264_ratecontrol_t,
    mut dst: *mut ::core::ffi::c_float,
) {
    let mut input: *mut ::core::ffi::c_float = 0 as *mut ::core::ffi::c_float;
    let mut output: *mut ::core::ffi::c_float = 0 as *mut ::core::ffi::c_float;
    let mut filtersize: ::core::ffi::c_int = 0;
    let mut stride: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    input = (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize];
    output = (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize];
    filtersize = (*rc).mbtree.filtersize[0 as ::core::ffi::c_int as usize];
    stride = (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize];
    height = (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize];
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut coeff: *mut ::core::ffi::c_float = (*rc)
            .mbtree
            .coeffs[0 as ::core::ffi::c_int as usize];
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < (*h).mb.i_mb_width {
            *output.offset(x as isize) = tapfilter(
                input,
                *(*rc).mbtree.pos[0 as ::core::ffi::c_int as usize].offset(x as isize),
                stride,
                1 as ::core::ffi::c_int,
                coeff,
                filtersize,
            );
            x += 1;
            coeff = coeff.offset(filtersize as isize);
        }
        y += 1;
        input = input.offset(stride as isize);
        output = output.offset((*h).mb.i_mb_width as isize);
    }
    input = (*rc).mbtree.scale_buffer[1 as ::core::ffi::c_int as usize];
    output = dst;
    filtersize = (*rc).mbtree.filtersize[1 as ::core::ffi::c_int as usize];
    stride = (*h).mb.i_mb_width;
    height = (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize];
    let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x_0 < (*h).mb.i_mb_width {
        let mut coeff_0: *mut ::core::ffi::c_float = (*rc)
            .mbtree
            .coeffs[1 as ::core::ffi::c_int as usize];
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < (*h).mb.i_mb_height {
            *output.offset((y_0 * stride) as isize) = tapfilter(
                input,
                *(*rc).mbtree.pos[1 as ::core::ffi::c_int as usize].offset(y_0 as isize),
                height,
                stride,
                coeff_0,
                filtersize,
            );
            y_0 += 1;
            coeff_0 = coeff_0.offset(filtersize as isize);
        }
        x_0 += 1;
        input = input.offset(1);
        output = output.offset(1);
    }
}
#[no_mangle]
#[c2rust::src_loc = "541:1"]
pub unsafe extern "C" fn x264_10_macroblock_tree_read(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut quant_offsets: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i_type_actual: uint8_t = (*(*rc).entry.offset((*frame).i_frame as isize))
        .pict_type as uint8_t;
    if (*(*rc).entry.offset((*frame).i_frame as isize)).kept_as_ref != 0 {
        let mut i_type: uint8_t = 0;
        if (*rc).mbtree.qpbuf_pos < 0 as ::core::ffi::c_int {
            current_block = 12675440807659640239;
        } else {
            current_block = 11812396948646013369;
        }
        loop {
            match current_block {
                12675440807659640239 => {
                    (*rc).mbtree.qpbuf_pos += 1;
                    if fread(
                        &mut i_type as *mut uint8_t as *mut ::core::ffi::c_void,
                        1 as size_t,
                        1 as size_t,
                        (*rc).p_mbtree_stat_file_in,
                    ) == 0
                    {
                        current_block = 5482255857212657476;
                        break;
                    }
                    if fread(
                        (*rc).mbtree.qp_buffer[(*rc).mbtree.qpbuf_pos as usize]
                            as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<uint16_t>() as size_t,
                        (*rc).mbtree.src_mb_count as size_t,
                        (*rc).p_mbtree_stat_file_in,
                    )
                        != (*rc).mbtree.src_mb_count as ::core::ffi::c_uint
                            as ::core::ffi::c_ulong
                    {
                        current_block = 5482255857212657476;
                        break;
                    }
                    if i_type as ::core::ffi::c_int
                        != i_type_actual as ::core::ffi::c_int
                        && (*rc).mbtree.qpbuf_pos == 1 as ::core::ffi::c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"MB-tree frametype %d doesn't match actual frametype %d.\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            i_type as ::core::ffi::c_int,
                            i_type_actual as ::core::ffi::c_int,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    if i_type as ::core::ffi::c_int
                        != i_type_actual as ::core::ffi::c_int
                    {
                        current_block = 12675440807659640239;
                    } else {
                        current_block = 11812396948646013369;
                    }
                }
                _ => {
                    let mut dst: *mut ::core::ffi::c_float = if (*rc)
                        .mbtree
                        .rescale_enabled != 0
                    {
                        (*rc).mbtree.scale_buffer[0 as ::core::ffi::c_int as usize]
                    } else {
                        (*frame).f_qp_offset
                    };
                    (*h)
                        .mc
                        .mbtree_fix8_unpack
                        .expect(
                            "non-null function pointer",
                        )(
                        dst,
                        (*rc).mbtree.qp_buffer[(*rc).mbtree.qpbuf_pos as usize],
                        (*rc).mbtree.src_mb_count,
                    );
                    if (*rc).mbtree.rescale_enabled != 0 {
                        macroblock_tree_rescale(h, rc, (*frame).f_qp_offset);
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < (*h).mb.i_mb_count {
                            *(*frame).i_inv_qscale_factor.offset(i as isize) = x264_exp2fix8(
                                *(*frame).f_qp_offset.offset(i as isize),
                            ) as uint16_t;
                            i += 1;
                        }
                    }
                    (*rc).mbtree.qpbuf_pos -= 1;
                    current_block = 5689001924483802034;
                    break;
                }
            }
        }
        match current_block {
            5689001924483802034 => {}
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"Incomplete MB-tree stats file.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
    } else {
        x264_10_adaptive_quant_frame(h, frame, quant_offsets);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "585:1"]
pub unsafe extern "C" fn x264_10_reference_build_list_optimal(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut rce: *mut ratecontrol_entry_t = (*(*h).rc).rce;
    let mut frames: [*mut x264_frame_t; 16] = [0 as *mut x264_frame_t; 16];
    let mut weights: [[x264_weight_t; 3]; 16] = [[x264_weight_t {
        cachea: [0; 8],
        cacheb: [0; 8],
        i_denom: 0,
        i_scale: 0,
        i_offset: 0,
        weightfn: 0 as *mut weight_fn_t,
    }; 3]; 16];
    let mut refcount: [::core::ffi::c_int; 16] = [0; 16];
    if (*rce).refs != (*h).i_ref[0 as ::core::ffi::c_int as usize] {
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        frames.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*(*h).fref.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr()
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut x264_frame_t; 16]>() as size_t,
    );
    memcpy(
        refcount.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*rce).refcount.as_mut_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_int; 16]>() as size_t,
    );
    memcpy(
        weights.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*(*h).fenc).weight.as_mut_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 16]>() as size_t,
    );
    memset(
        &mut *(*(*(*h).fenc)
            .weight
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize) as *mut x264_weight_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 15]>() as size_t,
    );
    let mut ref_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while ref_0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
        let mut max: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut bestref: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            if refcount[i as usize] > max {
                max = refcount[i as usize];
                bestref = i;
            }
            i += 1;
        }
        refcount[bestref as usize] = -(1 as ::core::ffi::c_int);
        (*h).fref[0 as ::core::ffi::c_int as usize][ref_0 as usize] = frames[bestref
            as usize];
        memcpy(
            (*(*(*h).fenc).weight.as_mut_ptr().offset(ref_0 as isize)).as_mut_ptr()
                as *mut ::core::ffi::c_void,
            (*weights.as_mut_ptr().offset(bestref as isize)).as_mut_ptr()
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[x264_weight_t; 3]>() as size_t,
        );
        ref_0 += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "622:1"]
unsafe extern "C" fn strcat_filename(
    mut input: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut output: *mut ::core::ffi::c_char = x264_malloc(
        strlen(input).wrapping_add(strlen(suffix)).wrapping_add(1 as size_t) as int64_t,
    ) as *mut ::core::ffi::c_char;
    if output.is_null() {
        return 0 as *mut ::core::ffi::c_char;
    }
    strcpy(output, input);
    strcat(output, suffix);
    return output;
}
#[no_mangle]
#[c2rust::src_loc = "632:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_init_reconfigurable(
    mut h: *mut x264_t,
    mut b_init: ::core::ffi::c_int,
) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if b_init == 0 && (*rc).b_2pass != 0 {
        return;
    }
    if (*h).param.rc.i_rc_method == X264_RC_CRF {
        let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
            * (if (*h).param.i_bframe != 0 {
                120 as ::core::ffi::c_int
            } else {
                80 as ::core::ffi::c_int
            })) as ::core::ffi::c_double;
        let mut mbtree_offset: ::core::ffi::c_double = if (*h).param.rc.b_mb_tree != 0 {
            (1.0f64 - (*h).param.rc.f_qcompress as ::core::ffi::c_double) * 13.5f64
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        };
        (*rc).rate_factor_constant = pow(
            base_cplx,
            1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rc).qcompress,
        )
            / qp2qscale(
                ((*h).param.rc.f_rf_constant as ::core::ffi::c_double + mbtree_offset
                    + QP_BD_OFFSET as ::core::ffi::c_double) as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
    }
    if (*h).param.rc.i_vbv_max_bitrate > 0 as ::core::ffi::c_int
        && (*h).param.rc.i_vbv_buffer_size > 0 as ::core::ffi::c_int
    {
        if (*rc).b_vbv_min_rate != 0 {
            (*h).param.rc.i_vbv_max_bitrate = (*h).param.rc.i_bitrate;
        }
        if (*h).param.rc.i_vbv_buffer_size
            < ((*h).param.rc.i_vbv_max_bitrate as ::core::ffi::c_double / (*rc).fps)
                as ::core::ffi::c_int
        {
            (*h).param.rc.i_vbv_buffer_size = ((*h).param.rc.i_vbv_max_bitrate
                as ::core::ffi::c_double / (*rc).fps) as ::core::ffi::c_int;
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV buffer size cannot be smaller than one frame, using %d kbit\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*h).param.rc.i_vbv_buffer_size,
            );
        }
        let mut kilobit_size: ::core::ffi::c_int = if (*h).param.i_avcintra_class != 0 {
            1024 as ::core::ffi::c_int
        } else {
            1000 as ::core::ffi::c_int
        };
        let mut vbv_buffer_size: ::core::ffi::c_int = (*h).param.rc.i_vbv_buffer_size
            * kilobit_size;
        let mut vbv_max_bitrate: ::core::ffi::c_int = (*h).param.rc.i_vbv_max_bitrate
            * kilobit_size;
        if (*h).param.i_nal_hrd != 0 && b_init != 0 {
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_cnt = 1 as ::core::ffi::c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.b_cbr_hrd = ((*h).param.i_nal_hrd
                == X264_NAL_HRD_CBR) as ::core::ffi::c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_time_offset_length = 0
                as ::core::ffi::c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale = x264_clip3(
                (vbv_max_bitrate as ::core::ffi::c_uint).trailing_zeros() as i32
                    - BR_SHIFT,
                0 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_value = vbv_max_bitrate
                >> (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale + BR_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled = (*(*h)
                .sps
                .as_mut_ptr())
                .vui
                .hrd
                .i_bit_rate_value
                << (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_scale + BR_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale = x264_clip3(
                (vbv_buffer_size as ::core::ffi::c_uint).trailing_zeros() as i32
                    - CPB_SHIFT,
                0 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_value = vbv_buffer_size
                >> (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale + CPB_SHIFT;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled = (*(*h)
                .sps
                .as_mut_ptr())
                .vui
                .hrd
                .i_cpb_size_value
                << (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_scale + CPB_SHIFT;
            let mut max_cpb_output_delay: ::core::ffi::c_int = (if ((*h)
                .param
                .i_keyint_max as ::core::ffi::c_double * 0.5f64
                * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                    as ::core::ffi::c_double)
                < 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                (*h).param.i_keyint_max as ::core::ffi::c_double * 0.5f64
                    * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                        as ::core::ffi::c_double
            } else {
                2147483647 as ::core::ffi::c_int as ::core::ffi::c_double
            }) as ::core::ffi::c_int;
            let mut max_dpb_output_delay: ::core::ffi::c_int = ((*(*h).sps.as_mut_ptr())
                .vui
                .i_max_dec_frame_buffering as ::core::ffi::c_double * MAX_DURATION
                * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                    as ::core::ffi::c_double) as ::core::ffi::c_int;
            let mut max_delay: ::core::ffi::c_int = (90000.0f64
                * (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled
                    as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled
                    as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int;
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_initial_cpb_removal_delay_length = 2
                as ::core::ffi::c_int
                + x264_clip3(
                    32 as ::core::ffi::c_int
                        - (max_delay as ::core::ffi::c_uint).leading_zeros() as i32,
                    4 as ::core::ffi::c_int,
                    22 as ::core::ffi::c_int,
                );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_removal_delay_length = x264_clip3(
                32 as ::core::ffi::c_int
                    - (max_cpb_output_delay as ::core::ffi::c_uint).leading_zeros()
                        as i32,
                4 as ::core::ffi::c_int,
                31 as ::core::ffi::c_int,
            );
            (*(*h).sps.as_mut_ptr()).vui.hrd.i_dpb_output_delay_length = x264_clip3(
                32 as ::core::ffi::c_int
                    - (max_dpb_output_delay as ::core::ffi::c_uint).leading_zeros()
                        as i32,
                4 as ::core::ffi::c_int,
                31 as ::core::ffi::c_int,
            );
            vbv_buffer_size = (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled;
            vbv_max_bitrate = (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled;
        } else if (*h).param.i_nal_hrd != 0 && b_init == 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV parameters cannot be changed when NAL HRD is in use\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled = vbv_max_bitrate;
        (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled = vbv_buffer_size;
        if (*rc).b_vbv_min_rate != 0 {
            (*rc).bitrate = (*h).param.rc.i_bitrate as ::core::ffi::c_double
                * kilobit_size as ::core::ffi::c_double;
        }
        (*rc).buffer_rate = vbv_max_bitrate as ::core::ffi::c_double / (*rc).fps;
        (*rc).vbv_max_rate = vbv_max_bitrate as ::core::ffi::c_double;
        (*rc).buffer_size = vbv_buffer_size as ::core::ffi::c_double;
        (*rc).single_frame_vbv = ((*rc).buffer_rate * 1.1f64 > (*rc).buffer_size)
            as ::core::ffi::c_int;
        if (*rc).b_abr != 0 && (*h).param.rc.i_rc_method == X264_RC_ABR {
            (*rc).cbr_decay = 1.0f64
                - (*rc).buffer_rate / (*rc).buffer_size * 0.5f64
                    * (if 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        > 1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                    {
                        0 as ::core::ffi::c_int as ::core::ffi::c_double
                    } else {
                        1.5f64 - (*rc).buffer_rate * (*rc).fps / (*rc).bitrate
                    });
        }
        if (*h).param.rc.i_rc_method == X264_RC_CRF
            && (*h).param.rc.f_rf_constant_max != 0.
        {
            (*rc).rate_factor_max_increment = (*h).param.rc.f_rf_constant_max
                - (*h).param.rc.f_rf_constant;
            if (*rc).rate_factor_max_increment
                <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"CRF max must be greater than CRF\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*rc).rate_factor_max_increment = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_float;
            }
        }
        if b_init != 0 {
            if (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double > 1.0f64 {
                (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                    ((*h).param.rc.f_vbv_buffer_init
                        / (*h).param.rc.i_vbv_buffer_size as ::core::ffi::c_float)
                        as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ) as ::core::ffi::c_float;
            }
            (*h).param.rc.f_vbv_buffer_init = x264_clip3f(
                if (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                    > (*rc).buffer_rate / (*rc).buffer_size
                {
                    (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                } else {
                    (*rc).buffer_rate / (*rc).buffer_size
                },
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*rc).buffer_fill_final_min = ((*rc).buffer_size
                * (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double
                * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double)
                as int64_t;
            (*rc).buffer_fill_final = (*rc).buffer_fill_final_min;
            (*rc).b_vbv = 1 as ::core::ffi::c_int;
            (*rc).b_vbv_min_rate = ((*rc).b_2pass == 0
                && (*h).param.rc.i_rc_method == X264_RC_ABR
                && (*h).param.rc.i_vbv_max_bitrate <= (*h).param.rc.i_bitrate)
                as ::core::ffi::c_int;
        }
    }
}
#[c2rust::src_loc = "673:21"]
pub const BR_SHIFT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[c2rust::src_loc = "674:21"]
pub const CPB_SHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[c2rust::src_loc = "688:21"]
pub const MAX_DURATION: ::core::ffi::c_double = 0.5f64;
#[no_mangle]
#[c2rust::src_loc = "744:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_new(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut w: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut num_preds: ::core::ffi::c_int = 0;
    static mut pred_coeff_table: [::core::ffi::c_float; 3] = [
        1.0f64 as ::core::ffi::c_float,
        1.0f64 as ::core::ffi::c_float,
        1.5f64 as ::core::ffi::c_float,
    ];
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = 0 as *mut x264_ratecontrol_t;
    (*h).rc = x264_malloc(
        ((*h).param.i_threads as usize)
            .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>() as usize)
            as int64_t,
    ) as *mut x264_ratecontrol_t;
    if !(*h).rc.is_null() {
        memset(
            (*h).rc as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*h).param.i_threads as size_t)
                .wrapping_mul(::core::mem::size_of::<x264_ratecontrol_t>() as size_t),
        );
        rc = (*h).rc;
        (*rc).b_abr = ((*h).param.rc.i_rc_method != X264_RC_CQP
            && (*h).param.rc.b_stat_read == 0) as ::core::ffi::c_int;
        (*rc).b_2pass = ((*h).param.rc.i_rc_method == X264_RC_ABR
            && (*h).param.rc.b_stat_read != 0) as ::core::ffi::c_int;
        if (*h).param.i_fps_num > 0 as uint32_t && (*h).param.i_fps_den > 0 as uint32_t {
            (*rc).fps = ((*h).param.i_fps_num as ::core::ffi::c_float
                / (*h).param.i_fps_den as ::core::ffi::c_float) as ::core::ffi::c_double;
        } else {
            (*rc).fps = 25.0f64;
        }
        if (*h).param.rc.b_mb_tree != 0 {
            (*h).param.rc.f_pb_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
            (*rc).qcompress = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            (*rc).qcompress = (*h).param.rc.f_qcompress as ::core::ffi::c_double;
        }
        (*rc).bitrate = (*h).param.rc.i_bitrate as ::core::ffi::c_double
            * (if (*h).param.i_avcintra_class != 0 { 1024.0f64 } else { 1000.0f64 });
        (*rc).rate_tolerance = (*h).param.rc.f_rate_tolerance as ::core::ffi::c_double;
        (*rc).nmb = (*h).mb.i_mb_count;
        (*rc).last_non_b_pict_type = -(1 as ::core::ffi::c_int);
        (*rc).cbr_decay = 1.0f64;
        if (*h).param.rc.i_rc_method != X264_RC_ABR && (*h).param.rc.b_stat_read != 0 {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"CRF/CQP is incompatible with 2pass.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        x264_10_ratecontrol_init_reconfigurable(h, 1 as ::core::ffi::c_int);
        if (*h).param.i_nal_hrd != 0 {
            let mut denom: uint64_t = ((*(*h).sps.as_mut_ptr())
                .vui
                .hrd
                .i_bit_rate_unscaled as uint64_t)
                .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
            let mut num: uint64_t = 90000 as uint64_t;
            x264_reduce_fraction64(&mut num, &mut denom);
            (*rc).hrd_multiply_denom = (90000 as uint64_t).wrapping_div(num);
            let mut bits_required: ::core::ffi::c_double = log2(
                num as ::core::ffi::c_double,
            ) + log2((*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double)
                + log2(
                    (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled
                        as ::core::ffi::c_double,
                );
            if bits_required >= 63 as ::core::ffi::c_int as ::core::ffi::c_double {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"HRD with very large timescale and bufsize not supported\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        if (*rc).rate_tolerance < 0.01f64 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"bitrate tolerance too small, using .01\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*rc).rate_tolerance = 0.01f64;
        }
        (*h).mb.b_variable_qp = ((*rc).b_vbv != 0 || (*h).param.rc.i_aq_mode != 0)
            as ::core::ffi::c_int;
        if (*rc).b_abr != 0 {
            (*rc).accum_p_norm = 0.01f64;
            (*rc).accum_p_qp = ((if (*h).param.rc.i_rc_method == X264_RC_CRF {
                (*h).param.rc.f_rf_constant
            } else {
                24 as ::core::ffi::c_int as ::core::ffi::c_float
            }) + QP_BD_OFFSET as ::core::ffi::c_float) as ::core::ffi::c_double
                * (*rc).accum_p_norm;
            (*rc).cplxr_sum = 0.01f64 * pow(7.0e5f64, (*rc).qcompress)
                * pow((*h).mb.i_mb_count as ::core::ffi::c_double, 0.5f64);
            (*rc).wanted_bits_window = 1.0f64 * (*rc).bitrate / (*rc).fps;
            (*rc).last_non_b_pict_type = SLICE_TYPE_I as ::core::ffi::c_int;
        }
        (*rc).ip_offset = 6.0f64
            * log2f((*h).param.rc.f_ip_factor) as ::core::ffi::c_double;
        (*rc).pb_offset = 6.0f64
            * log2f((*h).param.rc.f_pb_factor) as ::core::ffi::c_double;
        (*rc).qp_constant[SLICE_TYPE_P as ::core::ffi::c_int as usize] = (*h)
            .param
            .rc
            .i_qp_constant;
        (*rc).qp_constant[SLICE_TYPE_I as ::core::ffi::c_int as usize] = x264_clip3(
            ((*h).param.rc.i_qp_constant as ::core::ffi::c_double - (*rc).ip_offset
                + 0.5f64) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            QP_MAX,
        );
        (*rc).qp_constant[SLICE_TYPE_B as ::core::ffi::c_int as usize] = x264_clip3(
            ((*h).param.rc.i_qp_constant as ::core::ffi::c_double + (*rc).pb_offset
                + 0.5f64) as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            QP_MAX,
        );
        (*h).mb.ip_offset = ((*rc).ip_offset + 0.5f64) as ::core::ffi::c_int;
        (*rc).lstep = pow(
            2 as ::core::ffi::c_int as ::core::ffi::c_double,
            (*h).param.rc.i_qp_step as ::core::ffi::c_double / 6.0f64,
        );
        (*rc).last_qscale = qp2qscale(
            (26 as ::core::ffi::c_int + QP_BD_OFFSET) as ::core::ffi::c_float,
        ) as ::core::ffi::c_double;
        num_preds = (*h).param.b_sliced_threads * (*h).param.i_threads
            + 1 as ::core::ffi::c_int;
        (*rc).pred = x264_malloc(
            (5 as usize)
                .wrapping_mul(::core::mem::size_of::<predictor_t>() as usize)
                .wrapping_mul(num_preds as usize) as int64_t,
        ) as *mut predictor_t;
        if !(*rc).pred.is_null() {
            (*rc).pred_b_from_p = x264_malloc(
                ::core::mem::size_of::<predictor_t>() as int64_t,
            ) as *mut predictor_t;
            if !(*rc).pred_b_from_p.is_null() {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 3 as ::core::ffi::c_int {
                    (*rc).last_qscale_for[i as usize] = qp2qscale(
                        (if (*h).param.rc.i_rc_method == X264_RC_CRF {
                            (*h).param.rc.f_rf_constant
                        } else {
                            24 as ::core::ffi::c_int as ::core::ffi::c_float
                        }) + QP_BD_OFFSET as ::core::ffi::c_float,
                    ) as ::core::ffi::c_double;
                    (*rc).lmin[i as usize] = qp2qscale(
                        (*h).param.rc.i_qp_min as ::core::ffi::c_float,
                    ) as ::core::ffi::c_double;
                    (*rc).lmax[i as usize] = qp2qscale(
                        (*h).param.rc.i_qp_max as ::core::ffi::c_float,
                    ) as ::core::ffi::c_double;
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < num_preds {
                        (*(*rc).pred.offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .coeff_min = pred_coeff_table[i as usize]
                            / 2 as ::core::ffi::c_int as ::core::ffi::c_float;
                        (*(*rc).pred.offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .coeff = pred_coeff_table[i as usize];
                        (*(*rc).pred.offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .count = 1.0f32;
                        (*(*rc).pred.offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .decay = 0.5f32;
                        (*(*rc).pred.offset((i + j * 5 as ::core::ffi::c_int) as isize))
                            .offset = 0.0f32;
                        j += 1;
                    }
                    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j_0 < 2 as ::core::ffi::c_int {
                        (*rc).row_preds[i as usize][j_0 as usize].coeff_min = (0.25f64
                            / 4 as ::core::ffi::c_int as ::core::ffi::c_double)
                            as ::core::ffi::c_float;
                        (*rc).row_preds[i as usize][j_0 as usize].coeff = 0.25f32;
                        (*rc).row_preds[i as usize][j_0 as usize].count = 1.0f32;
                        (*rc).row_preds[i as usize][j_0 as usize].decay = 0.5f32;
                        (*rc).row_preds[i as usize][j_0 as usize].offset = 0.0f32;
                        j_0 += 1;
                    }
                    i += 1;
                }
                (*(*rc).pred_b_from_p).coeff_min = (0.5f64
                    / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_float;
                (*(*rc).pred_b_from_p).coeff = 0.5f32;
                (*(*rc).pred_b_from_p).count = 1.0f32;
                (*(*rc).pred_b_from_p).decay = 0.5f32;
                (*(*rc).pred_b_from_p).offset = 0.0f32;
                if parse_zones(h) < 0 as ::core::ffi::c_int {
                    x264_10_log(
                        h,
                        X264_LOG_ERROR,
                        b"failed to parse zones\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                if (*h).param.rc.b_stat_read != 0 {
                    let mut p: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
                    let mut stats_in: *mut ::core::ffi::c_char = 0
                        as *mut ::core::ffi::c_char;
                    let mut stats_buf: *mut ::core::ffi::c_char = 0
                        as *mut ::core::ffi::c_char;
                    if !(*h).param.rc.psz_stat_in.is_null() {} else {
                        __assert_fail(
                            b"h->param.rc.psz_stat_in\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"encoder/ratecontrol.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            874 as ::core::ffi::c_uint,
                            ::core::mem::transmute::<
                                [u8; 38],
                                [::core::ffi::c_char; 38],
                            >(*b"int x264_10_ratecontrol_new(x264_t *)\0")
                                .as_ptr(),
                        );
                    }
                    'c_35168: {
                        if !(*h).param.rc.psz_stat_in.is_null() {} else {
                            __assert_fail(
                                b"h->param.rc.psz_stat_in\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/ratecontrol.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                874 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 38],
                                    [::core::ffi::c_char; 38],
                                >(*b"int x264_10_ratecontrol_new(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                    };
                    stats_in = x264_slurp_file((*h).param.rc.psz_stat_in);
                    stats_buf = stats_in;
                    if stats_buf.is_null() {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"ratecontrol_init: can't open stats file\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    if (*h).param.rc.b_mb_tree != 0 {
                        let mut mbtree_stats_in: *mut ::core::ffi::c_char = strcat_filename(
                            (*h).param.rc.psz_stat_in,
                            b".mbtree\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        if mbtree_stats_in.is_null() {
                            return -(1 as ::core::ffi::c_int);
                        }
                        (*rc).p_mbtree_stat_file_in = fopen(
                            mbtree_stats_in,
                            b"rb\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as *mut FILE;
                        x264_free(mbtree_stats_in as *mut ::core::ffi::c_void);
                        if (*rc).p_mbtree_stat_file_in.is_null() {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"ratecontrol_init: can't open mbtree stats file\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                    }
                    if strncmp(
                        stats_buf,
                        b"#options:\0" as *const u8 as *const ::core::ffi::c_char,
                        9 as size_t,
                    ) != 0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"options list in stats file not valid\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    let mut res_factor: ::core::ffi::c_float = 0.;
                    let mut res_factor_bits: ::core::ffi::c_float = 0.;
                    let mut i_0: ::core::ffi::c_int = 0;
                    let mut j_1: ::core::ffi::c_int = 0;
                    let mut k: uint32_t = 0;
                    let mut l: uint32_t = 0;
                    let mut opts: *mut ::core::ffi::c_char = stats_buf;
                    stats_in = strchr(stats_buf, '\n' as i32);
                    if stats_in.is_null() {
                        return -(1 as ::core::ffi::c_int);
                    }
                    *stats_in = '\0' as i32 as ::core::ffi::c_char;
                    stats_in = stats_in.offset(1);
                    if sscanf(
                        opts,
                        b"#options: %dx%d\0" as *const u8 as *const ::core::ffi::c_char,
                        &mut i_0 as *mut ::core::ffi::c_int,
                        &mut j_1 as *mut ::core::ffi::c_int,
                    ) != 2 as ::core::ffi::c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"resolution specified in stats file not valid\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    } else if (*h).param.rc.b_mb_tree != 0 {
                        (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] = i_0;
                        (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] = j_1;
                    }
                    res_factor = (*h).param.i_width as ::core::ffi::c_float
                        * (*h).param.i_height as ::core::ffi::c_float
                        / (i_0 * j_1) as ::core::ffi::c_float;
                    res_factor_bits = powf(res_factor, 0.7f32);
                    p = strstr(
                        opts,
                        b"timebase=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if p.is_null()
                        || sscanf(
                            p,
                            b"timebase=%u/%u\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &mut k as *mut uint32_t,
                            &mut l as *mut uint32_t,
                        ) != 2 as ::core::ffi::c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"timebase specified in stats file not valid\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    if k != (*h).param.i_timebase_num || l != (*h).param.i_timebase_den {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"timebase mismatch with 1st pass (%u/%u vs %u/%u)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.i_timebase_num,
                            (*h).param.i_timebase_den,
                            k,
                            l,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"bitdepth=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bitdepth=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && 10 as ::core::ffi::c_int != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bitdepth setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            10 as ::core::ffi::c_int,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"weightp=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"weightp=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0
                        && (if 0 as ::core::ffi::c_int
                            > (*h).param.analyse.i_weighted_pred
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            (*h).param.analyse.i_weighted_pred
                        }) != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different weightp setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            if 0 as ::core::ffi::c_int
                                > (*h).param.analyse.i_weighted_pred
                            {
                                0 as ::core::ffi::c_int
                            } else {
                                (*h).param.analyse.i_weighted_pred
                            },
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"bframes=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bframes=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.i_bframe != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bframes setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.i_bframe,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"b_pyramid=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"b_pyramid=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.i_bframe_pyramid != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different b_pyramid setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.i_bframe_pyramid,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"intra_refresh=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"intra_refresh=%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.b_intra_refresh != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different intra_refresh setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.b_intra_refresh,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"open_gop=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"open_gop=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.b_open_gop != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different open_gop setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.b_open_gop,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"bluray_compat=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"bluray_compat=%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.b_bluray_compat != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different bluray_compat setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.b_bluray_compat,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"mbtree=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"mbtree=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && (*h).param.rc.b_mb_tree != i_0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"different mbtree setting than first pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.rc.b_mb_tree,
                            i_0,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = strstr(
                        opts,
                        b"interlaced=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null() {
                        let mut current: *mut ::core::ffi::c_char = (if (*h)
                            .param
                            .b_interlaced != 0
                        {
                            if (*h).param.b_tff != 0 {
                                b"tff\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"bff\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else if (*h).param.b_fake_interlaced != 0 {
                            b"fake\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"0\0" as *const u8 as *const ::core::ffi::c_char
                        }) as *mut ::core::ffi::c_char;
                        let mut buf: [::core::ffi::c_char; 5] = [0; 5];
                        sscanf(
                            p,
                            b"interlaced=%4s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            buf.as_mut_ptr(),
                        );
                        if strcmp(current, buf.as_mut_ptr()) != 0 {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"different interlaced setting than first pass (%s vs %s)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                current,
                                buf.as_mut_ptr(),
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                    }
                    p = strstr(
                        opts,
                        b"keyint=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null() {
                        p = p.offset(7 as ::core::ffi::c_int as isize);
                        let mut buf_0: [::core::ffi::c_char; 13] = ::core::mem::transmute::<
                            [u8; 13],
                            [::core::ffi::c_char; 13],
                        >(*b"infinite \0\0\0\0");
                        if (*h).param.i_keyint_max != X264_KEYINT_MAX_INFINITE {
                            sprintf(
                                buf_0.as_mut_ptr(),
                                b"%d \0" as *const u8 as *const ::core::ffi::c_char,
                                (*h).param.i_keyint_max,
                            );
                        }
                        if strncmp(p, buf_0.as_mut_ptr(), strlen(buf_0.as_mut_ptr()))
                            != 0
                        {
                            x264_10_log(
                                h,
                                X264_LOG_ERROR,
                                b"different keyint setting than first pass (%.*s vs %.*s)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                strlen(buf_0.as_mut_ptr()).wrapping_sub(1 as size_t),
                                buf_0.as_mut_ptr(),
                                strcspn(
                                    p,
                                    b" \0" as *const u8 as *const ::core::ffi::c_char,
                                ),
                                p,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                    }
                    if !strstr(
                            opts,
                            b"qp=0\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null() && (*h).param.rc.i_rc_method == X264_RC_ABR
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"1st pass was lossless, bitrate prediction will be inaccurate\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if strstr(
                            opts,
                            b"direct=3\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        && (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"direct=auto not used on the first pass\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        (*h).mb.b_direct_auto_write = 1 as ::core::ffi::c_int;
                    }
                    p = strstr(
                        opts,
                        b"b_adapt=\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !p.is_null()
                        && sscanf(
                            p,
                            b"b_adapt=%d\0" as *const u8 as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0 && i_0 >= X264_B_ADAPT_NONE && i_0 <= X264_B_ADAPT_TRELLIS
                    {
                        (*h).param.i_bframe_adaptive = i_0;
                    } else if (*h).param.i_bframe != 0 {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"b_adapt method specified in stats file not valid\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    if ((*h).param.rc.b_mb_tree != 0
                        || (*h).param.rc.i_vbv_buffer_size != 0)
                        && {
                            p = strstr(
                                opts,
                                b"rc_lookahead=\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            !p.is_null()
                        }
                        && sscanf(
                            p,
                            b"rc_lookahead=%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &mut i_0 as *mut ::core::ffi::c_int,
                        ) != 0
                    {
                        (*h).param.rc.i_lookahead = i_0;
                    }
                    p = stats_in;
                    let mut num_entries: ::core::ffi::c_int = 0;
                    num_entries = -(1 as ::core::ffi::c_int);
                    while !p.is_null() {
                        p = strchr(
                            p.offset(1 as ::core::ffi::c_int as isize),
                            ';' as i32,
                        );
                        num_entries += 1;
                    }
                    if num_entries == 0 {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"empty stats file\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    (*rc).num_entries = num_entries;
                    if (*h).param.i_frame_total < (*rc).num_entries
                        && (*h).param.i_frame_total > 0 as ::core::ffi::c_int
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"2nd pass has fewer frames than 1st pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.i_frame_total,
                            (*rc).num_entries,
                        );
                    }
                    if (*h).param.i_frame_total > (*rc).num_entries {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"2nd pass has more frames than 1st pass (%d vs %d)\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            (*h).param.i_frame_total,
                            (*rc).num_entries,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    (*rc).entry = x264_malloc(
                        ((*rc).num_entries as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<ratecontrol_entry_t>() as usize,
                            ) as int64_t,
                    ) as *mut ratecontrol_entry_t;
                    if (*rc).entry.is_null() {
                        current_block = 4515208850251936372;
                    } else {
                        memset(
                            (*rc).entry as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ((*rc).num_entries as size_t)
                                .wrapping_mul(
                                    ::core::mem::size_of::<ratecontrol_entry_t>() as size_t,
                                ),
                        );
                        (*rc).entry_out = x264_malloc(
                            ((*rc).num_entries as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ratecontrol_entry_t>() as usize,
                                ) as int64_t,
                        ) as *mut *mut ratecontrol_entry_t;
                        if (*rc).entry_out.is_null() {
                            current_block = 4515208850251936372;
                        } else {
                            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_1 < (*rc).num_entries {
                                let mut rce: *mut ratecontrol_entry_t = &mut *(*rc)
                                    .entry
                                    .offset(i_1 as isize) as *mut ratecontrol_entry_t;
                                (*rce).pict_type = SLICE_TYPE_P as ::core::ffi::c_int;
                                (*rce).new_qscale = qp2qscale(
                                    (20 as ::core::ffi::c_int + QP_BD_OFFSET)
                                        as ::core::ffi::c_float,
                                ) as ::core::ffi::c_double;
                                (*rce).qscale = (*rce).new_qscale;
                                (*rce).misc_bits = (*rc).nmb + 10 as ::core::ffi::c_int;
                                (*rce).new_qp = 0 as ::core::ffi::c_int
                                    as ::core::ffi::c_float;
                                let ref mut fresh0 = *(*rc).entry_out.offset(i_1 as isize);
                                *fresh0 = rce;
                                i_1 += 1;
                            }
                            p = stats_in;
                            let mut total_qp_aq: ::core::ffi::c_double = 0
                                as ::core::ffi::c_int as ::core::ffi::c_double;
                            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_2 < (*rc).num_entries {
                                let mut rce_0: *mut ratecontrol_entry_t = 0
                                    as *mut ratecontrol_entry_t;
                                let mut frame_number: ::core::ffi::c_int = 0
                                    as ::core::ffi::c_int;
                                let mut frame_out_number: ::core::ffi::c_int = 0
                                    as ::core::ffi::c_int;
                                let mut pict_type: ::core::ffi::c_char = 0
                                    as ::core::ffi::c_char;
                                let mut e: ::core::ffi::c_int = 0;
                                let mut next: *mut ::core::ffi::c_char = 0
                                    as *mut ::core::ffi::c_char;
                                let mut qp_rc: ::core::ffi::c_float = 0.;
                                let mut qp_aq: ::core::ffi::c_float = 0.;
                                let mut ref_0: ::core::ffi::c_int = 0;
                                next = strchr(p, ';' as i32);
                                if !next.is_null() {
                                    let fresh1 = next;
                                    next = next.offset(1);
                                    *fresh1 = 0 as ::core::ffi::c_char;
                                }
                                e = sscanf(
                                    p,
                                    b" in:%d out:%d \0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &mut frame_number as *mut ::core::ffi::c_int,
                                    &mut frame_out_number as *mut ::core::ffi::c_int,
                                );
                                if frame_number < 0 as ::core::ffi::c_int
                                    || frame_number >= (*rc).num_entries
                                {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"bad frame number (%d) at stats line %d\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        frame_number,
                                        i_2,
                                    );
                                    return -(1 as ::core::ffi::c_int);
                                }
                                if frame_out_number < 0 as ::core::ffi::c_int
                                    || frame_out_number >= (*rc).num_entries
                                {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"bad frame output number (%d) at stats line %d\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        frame_out_number,
                                        i_2,
                                    );
                                    return -(1 as ::core::ffi::c_int);
                                }
                                rce_0 = &mut *(*rc).entry.offset(frame_number as isize)
                                    as *mut ratecontrol_entry_t;
                                let ref mut fresh2 = *(*rc)
                                    .entry_out
                                    .offset(frame_out_number as isize);
                                *fresh2 = rce_0;
                                (*rce_0).direct_mode = 0 as ::core::ffi::c_char;
                                e
                                    += sscanf(
                                        p,
                                        b" in:%*d out:%*d type:%c dur:%ld cpbdur:%ld q:%f aq:%f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        &mut pict_type as *mut ::core::ffi::c_char,
                                        &mut (*rce_0).i_duration as *mut int64_t,
                                        &mut (*rce_0).i_cpb_duration as *mut int64_t,
                                        &mut qp_rc as *mut ::core::ffi::c_float,
                                        &mut qp_aq as *mut ::core::ffi::c_float,
                                        &mut (*rce_0).tex_bits as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).mv_bits as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).misc_bits as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).i_count as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).p_count as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).s_count as *mut ::core::ffi::c_int,
                                        &mut (*rce_0).direct_mode as *mut ::core::ffi::c_char,
                                    );
                                (*rce_0).tex_bits = ((*rce_0).tex_bits
                                    as ::core::ffi::c_float * res_factor_bits)
                                    as ::core::ffi::c_int;
                                (*rce_0).mv_bits = ((*rce_0).mv_bits as ::core::ffi::c_float
                                    * res_factor_bits) as ::core::ffi::c_int;
                                (*rce_0).misc_bits = ((*rce_0).misc_bits
                                    as ::core::ffi::c_float * res_factor_bits)
                                    as ::core::ffi::c_int;
                                (*rce_0).i_count = ((*rce_0).i_count as ::core::ffi::c_float
                                    * res_factor) as ::core::ffi::c_int;
                                (*rce_0).p_count = ((*rce_0).p_count as ::core::ffi::c_float
                                    * res_factor) as ::core::ffi::c_int;
                                (*rce_0).s_count = ((*rce_0).s_count as ::core::ffi::c_float
                                    * res_factor) as ::core::ffi::c_int;
                                p = strstr(
                                    p,
                                    b"ref:\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                                if !p.is_null() {
                                    p = p.offset(4 as ::core::ffi::c_int as isize);
                                    ref_0 = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(ref_0 < 16 as ::core::ffi::c_int) {
                                            current_block = 9728093949049737828;
                                            break;
                                        }
                                        if sscanf(
                                            p,
                                            b" %d\0" as *const u8 as *const ::core::ffi::c_char,
                                            &mut *(*rce_0).refcount.as_mut_ptr().offset(ref_0 as isize)
                                                as *mut ::core::ffi::c_int,
                                        ) != 1 as ::core::ffi::c_int
                                        {
                                            current_block = 9728093949049737828;
                                            break;
                                        }
                                        p = strchr(
                                            p.offset(1 as ::core::ffi::c_int as isize),
                                            ' ' as i32,
                                        );
                                        if p.is_null() {
                                            current_block = 12194954648860098588;
                                            break;
                                        }
                                        ref_0 += 1;
                                    }
                                    match current_block {
                                        12194954648860098588 => {}
                                        _ => {
                                            (*rce_0).refs = ref_0;
                                            (*rce_0).i_weight_denom[1 as ::core::ffi::c_int as usize] = -(1
                                                as ::core::ffi::c_int) as int16_t;
                                            (*rce_0).i_weight_denom[0 as ::core::ffi::c_int as usize] = (*rce_0)
                                                .i_weight_denom[1 as ::core::ffi::c_int as usize];
                                            w = strchr(p, 'w' as i32);
                                            if !w.is_null() {
                                                let mut count: ::core::ffi::c_int = sscanf(
                                                    w,
                                                    b"w:%hd,%hd,%hd,%hd,%hd,%hd,%hd,%hd\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    &mut *(*rce_0)
                                                        .i_weight_denom
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*rce_0)
                                                        .i_weight_denom
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(2 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                    &mut *(*(*rce_0)
                                                        .weight
                                                        .as_mut_ptr()
                                                        .offset(2 as ::core::ffi::c_int as isize))
                                                        .as_mut_ptr()
                                                        .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t,
                                                );
                                                if count == 3 as ::core::ffi::c_int {
                                                    (*rce_0).i_weight_denom[1 as ::core::ffi::c_int as usize] = -(1
                                                        as ::core::ffi::c_int) as int16_t;
                                                } else if count != 8 as ::core::ffi::c_int {
                                                    (*rce_0).i_weight_denom[1 as ::core::ffi::c_int as usize] = -(1
                                                        as ::core::ffi::c_int) as int16_t;
                                                    (*rce_0).i_weight_denom[0 as ::core::ffi::c_int as usize] = (*rce_0)
                                                        .i_weight_denom[1 as ::core::ffi::c_int as usize];
                                                }
                                            }
                                            if pict_type as ::core::ffi::c_int != 'b' as i32 {
                                                (*rce_0).kept_as_ref = 1 as ::core::ffi::c_int;
                                            }
                                            match pict_type as ::core::ffi::c_int {
                                                73 => {
                                                    (*rce_0).frame_type = X264_TYPE_IDR;
                                                    (*rce_0).pict_type = SLICE_TYPE_I as ::core::ffi::c_int;
                                                }
                                                105 => {
                                                    (*rce_0).frame_type = X264_TYPE_I;
                                                    (*rce_0).pict_type = SLICE_TYPE_I as ::core::ffi::c_int;
                                                }
                                                80 => {
                                                    (*rce_0).frame_type = X264_TYPE_P;
                                                    (*rce_0).pict_type = SLICE_TYPE_P as ::core::ffi::c_int;
                                                }
                                                66 => {
                                                    (*rce_0).frame_type = X264_TYPE_BREF;
                                                    (*rce_0).pict_type = SLICE_TYPE_B as ::core::ffi::c_int;
                                                }
                                                98 => {
                                                    (*rce_0).frame_type = X264_TYPE_B;
                                                    (*rce_0).pict_type = SLICE_TYPE_B as ::core::ffi::c_int;
                                                }
                                                _ => {
                                                    e = -(1 as ::core::ffi::c_int);
                                                }
                                            }
                                            if !(e < 14 as ::core::ffi::c_int) {
                                                (*rce_0).qscale = qp2qscale(qp_rc) as ::core::ffi::c_double;
                                                total_qp_aq += qp_aq as ::core::ffi::c_double;
                                                p = next;
                                                i_2 += 1;
                                                continue;
                                            }
                                        }
                                    }
                                }
                                x264_10_log(
                                    h,
                                    X264_LOG_ERROR,
                                    b"statistics are damaged at line %d, parser out=%d\n\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    i_2,
                                    e,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                            if (*h).param.b_stitchable == 0 {
                                (*(*h).pps.as_mut_ptr()).i_pic_init_qp = if ((total_qp_aq
                                    / (*rc).num_entries as ::core::ffi::c_double + 0.5f64)
                                    as ::core::ffi::c_int)
                                    < 51 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                {
                                    (total_qp_aq / (*rc).num_entries as ::core::ffi::c_double
                                        + 0.5f64) as ::core::ffi::c_int
                                } else {
                                    51 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                };
                            }
                            x264_free(stats_buf as *mut ::core::ffi::c_void);
                            if (*h).param.rc.i_rc_method == X264_RC_ABR {
                                if init_pass2(h) < 0 as ::core::ffi::c_int {
                                    return -(1 as ::core::ffi::c_int);
                                }
                            }
                            current_block = 2756754640271984560;
                        }
                    }
                } else {
                    current_block = 2756754640271984560;
                }
                match current_block {
                    4515208850251936372 => {}
                    _ => {
                        if (*h).param.rc.b_stat_write != 0 {
                            let mut p_0: *mut ::core::ffi::c_char = 0
                                as *mut ::core::ffi::c_char;
                            (*rc).psz_stat_file_tmpname = strcat_filename(
                                (*h).param.rc.psz_stat_out,
                                b".temp\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                            );
                            if (*rc).psz_stat_file_tmpname.is_null() {
                                return -(1 as ::core::ffi::c_int);
                            }
                            (*rc).p_stat_file_out = fopen(
                                (*rc).psz_stat_file_tmpname,
                                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
                            ) as *mut FILE;
                            if (*rc).p_stat_file_out.is_null() {
                                x264_10_log(
                                    h,
                                    X264_LOG_ERROR,
                                    b"ratecontrol_init: can't open stats file\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                            p_0 = x264_param2string(
                                &mut (*h).param,
                                1 as ::core::ffi::c_int,
                            );
                            if !p_0.is_null() {
                                fprintf(
                                    (*rc).p_stat_file_out,
                                    b"#options: %s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    p_0,
                                );
                            }
                            x264_free(p_0 as *mut ::core::ffi::c_void);
                            if (*h).param.rc.b_mb_tree != 0
                                && (*h).param.rc.b_stat_read == 0
                            {
                                (*rc).psz_mbtree_stat_file_tmpname = strcat_filename(
                                    (*h).param.rc.psz_stat_out,
                                    b".mbtree.temp\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                (*rc).psz_mbtree_stat_file_name = strcat_filename(
                                    (*h).param.rc.psz_stat_out,
                                    b".mbtree\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                if (*rc).psz_mbtree_stat_file_tmpname.is_null()
                                    || (*rc).psz_mbtree_stat_file_name.is_null()
                                {
                                    return -(1 as ::core::ffi::c_int);
                                }
                                (*rc).p_mbtree_stat_file_out = fopen(
                                    (*rc).psz_mbtree_stat_file_tmpname,
                                    b"wb\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as *mut FILE;
                                if (*rc).p_mbtree_stat_file_out.is_null() {
                                    x264_10_log(
                                        h,
                                        X264_LOG_ERROR,
                                        b"ratecontrol_init: can't open mbtree stats file\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    return -(1 as ::core::ffi::c_int);
                                }
                            }
                        }
                        if (*h).param.rc.b_mb_tree != 0
                            && ((*h).param.rc.b_stat_read != 0
                                || (*h).param.rc.b_stat_write != 0)
                        {
                            if (*h).param.rc.b_stat_read == 0 {
                                (*rc).mbtree.srcdim[0 as ::core::ffi::c_int as usize] = (*h)
                                    .param
                                    .i_width;
                                (*rc).mbtree.srcdim[1 as ::core::ffi::c_int as usize] = (*h)
                                    .param
                                    .i_height;
                            }
                            if macroblock_tree_rescale_init(h, rc)
                                < 0 as ::core::ffi::c_int
                            {
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_3 < (*h).param.i_threads {
                            (*(*h).thread[i_3 as usize]).rc = rc.offset(i_3 as isize);
                            if i_3 != 0 {
                                *rc.offset(i_3 as isize) = *rc
                                    .offset(0 as ::core::ffi::c_int as isize);
                                (*(*h).thread[i_3 as usize]).param = (*h).param;
                                (*(*h).thread[i_3 as usize]).mb.b_variable_qp = (*h)
                                    .mb
                                    .b_variable_qp;
                                (*(*h).thread[i_3 as usize]).mb.ip_offset = (*h)
                                    .mb
                                    .ip_offset;
                            }
                            i_3 += 1;
                        }
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "1219:1"]
unsafe extern "C" fn parse_zone(
    mut h: *mut x264_t,
    mut z: *mut x264_zone_t,
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tok: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut saveptr: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    (*z).param = 0 as *mut x264_param_t;
    (*z).f_bitrate_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
    if 3 as ::core::ffi::c_int
        <= sscanf(
            p,
            b"%d,%d,q=%d%n\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*z).i_start as *mut ::core::ffi::c_int,
            &mut (*z).i_end as *mut ::core::ffi::c_int,
            &mut (*z).i_qp as *mut ::core::ffi::c_int,
            &mut len as *mut ::core::ffi::c_int,
        )
    {
        (*z).b_force_qp = 1 as ::core::ffi::c_int;
    } else if 3 as ::core::ffi::c_int
        <= sscanf(
            p,
            b"%d,%d,b=%f%n\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*z).i_start as *mut ::core::ffi::c_int,
            &mut (*z).i_end as *mut ::core::ffi::c_int,
            &mut (*z).f_bitrate_factor as *mut ::core::ffi::c_float,
            &mut len as *mut ::core::ffi::c_int,
        )
    {
        (*z).b_force_qp = 0 as ::core::ffi::c_int;
    } else if 2 as ::core::ffi::c_int
        <= sscanf(
            p,
            b"%d,%d%n\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*z).i_start as *mut ::core::ffi::c_int,
            &mut (*z).i_end as *mut ::core::ffi::c_int,
            &mut len as *mut ::core::ffi::c_int,
        )
    {
        (*z).b_force_qp = 0 as ::core::ffi::c_int;
    } else {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"invalid zone: \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            p,
        );
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(len as isize);
    if *p == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*z).param = x264_malloc(::core::mem::size_of::<x264_param_t>() as int64_t)
        as *mut x264_param_t;
    if (*z).param.is_null() {
        return -(1 as ::core::ffi::c_int)
    } else {
        memcpy(
            (*z).param as *mut ::core::ffi::c_void,
            &mut (*h).param as *mut x264_param_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<x264_param_t>() as size_t,
        );
        (*(*z).param).opaque = NULL;
        (*(*z).param).param_free = Some(
            x264_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        loop {
            tok = strtok_r(
                p,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                &mut saveptr,
            );
            if tok.is_null() {
                break;
            }
            let mut val: *mut ::core::ffi::c_char = strchr(tok, '=' as i32);
            if !val.is_null() {
                *val = '\0' as i32 as ::core::ffi::c_char;
                val = val.offset(1);
            }
            if x264_param_parse((*z).param as *mut x264_param_t, tok, val) != 0 {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"invalid zone param: %s = %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    tok,
                    val,
                );
                return -(1 as ::core::ffi::c_int);
            }
            p = 0 as *mut ::core::ffi::c_char;
        }
        return 0 as ::core::ffi::c_int;
    };
}
#[c2rust::src_loc = "1263:1"]
unsafe extern "C" fn parse_zones(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if !(*h).param.rc.psz_zones.is_null() && (*h).param.rc.i_zones == 0 {
        let mut psz_zones: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        psz_zones = x264_malloc(
            strlen((*h).param.rc.psz_zones).wrapping_add(1 as size_t) as int64_t,
        ) as *mut ::core::ffi::c_char;
        if psz_zones.is_null() {
            current_block = 12230949131536263527;
        } else {
            strcpy(psz_zones, (*h).param.rc.psz_zones);
            (*h).param.rc.i_zones = 1 as ::core::ffi::c_int;
            p = psz_zones;
            while *p != 0 {
                (*h).param.rc.i_zones
                    += (*p as ::core::ffi::c_int == '/' as i32) as ::core::ffi::c_int;
                p = p.offset(1);
            }
            (*h).param.rc.zones = x264_malloc(
                ((*h).param.rc.i_zones as usize)
                    .wrapping_mul(::core::mem::size_of::<x264_zone_t>() as usize)
                    as int64_t,
            ) as *mut x264_zone_t;
            if (*h).param.rc.zones.is_null() {
                current_block = 12230949131536263527;
            } else {
                p = psz_zones;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*h).param.rc.i_zones {
                    let mut i_tok: ::core::ffi::c_int = strcspn(
                        p,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as ::core::ffi::c_int;
                    *p.offset(i_tok as isize) = 0 as ::core::ffi::c_char;
                    if parse_zone(h, &mut *(*h).param.rc.zones.offset(i as isize), p)
                        != 0
                    {
                        x264_free(psz_zones as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    p = p.offset((i_tok + 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
                x264_free(psz_zones as *mut ::core::ffi::c_void);
                current_block = 11584701595673473500;
            }
        }
    } else {
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 => {
            if (*h).param.rc.i_zones > 0 as ::core::ffi::c_int {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < (*h).param.rc.i_zones {
                    let mut z: x264_zone_t = *(*h).param.rc.zones.offset(i_0 as isize);
                    if z.i_start < 0 as ::core::ffi::c_int || z.i_start > z.i_end {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"invalid zone: start=%d end=%d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            z.i_start,
                            z.i_end,
                        );
                        return -(1 as ::core::ffi::c_int);
                    } else if z.b_force_qp == 0
                        && z.f_bitrate_factor
                            <= 0 as ::core::ffi::c_int as ::core::ffi::c_float
                    {
                        x264_10_log(
                            h,
                            X264_LOG_ERROR,
                            b"invalid zone: bitrate_factor=%f\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            z.f_bitrate_factor as ::core::ffi::c_double,
                        );
                        return -(1 as ::core::ffi::c_int);
                    }
                    i_0 += 1;
                }
                (*rc).i_zones = (*h).param.rc.i_zones + 1 as ::core::ffi::c_int;
                (*rc).zones = x264_malloc(
                    ((*rc).i_zones as usize)
                        .wrapping_mul(::core::mem::size_of::<x264_zone_t>() as usize)
                        as int64_t,
                ) as *mut x264_zone_t;
                if (*rc).zones.is_null() {
                    current_block = 12230949131536263527;
                } else {
                    memcpy(
                        (*rc).zones.offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        (*h).param.rc.zones as *const ::core::ffi::c_void,
                        (((*rc).i_zones - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<x264_zone_t>() as size_t,
                            ),
                    );
                    (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).i_start = 0
                        as ::core::ffi::c_int;
                    (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).i_end = INT_MAX;
                    (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).b_force_qp = 0
                        as ::core::ffi::c_int;
                    (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize))
                        .f_bitrate_factor = 1 as ::core::ffi::c_int
                        as ::core::ffi::c_float;
                    let ref mut fresh3 = (*(*rc)
                        .zones
                        .offset(0 as ::core::ffi::c_int as isize))
                        .param;
                    *fresh3 = x264_malloc(
                        ::core::mem::size_of::<x264_param_t>() as int64_t,
                    ) as *mut x264_param_t;
                    if (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize))
                        .param
                        .is_null()
                    {
                        current_block = 12230949131536263527;
                    } else {
                        memcpy(
                            (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                                as *mut ::core::ffi::c_void,
                            &mut (*h).param as *mut x264_param_t
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<x264_param_t>() as size_t,
                        );
                        let ref mut fresh4 = (*(*(*rc)
                            .zones
                            .offset(0 as ::core::ffi::c_int as isize))
                            .param)
                            .opaque;
                        *fresh4 = NULL;
                        let mut i_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        while i_1 < (*rc).i_zones {
                            if (*(*rc).zones.offset(i_1 as isize)).param.is_null() {
                                let ref mut fresh5 = (*(*rc).zones.offset(i_1 as isize))
                                    .param;
                                *fresh5 = (*(*rc)
                                    .zones
                                    .offset(0 as ::core::ffi::c_int as isize))
                                    .param;
                            }
                            i_1 += 1;
                        }
                        current_block = 2122094917359643297;
                    }
                }
            } else {
                current_block = 2122094917359643297;
            }
            match current_block {
                12230949131536263527 => {}
                _ => return 0 as ::core::ffi::c_int,
            }
        }
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "1333:1"]
unsafe extern "C" fn get_zone(
    mut h: *mut x264_t,
    mut frame_num: ::core::ffi::c_int,
) -> *mut x264_zone_t {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i: ::core::ffi::c_int = (*rc).i_zones - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut z: *mut x264_zone_t = &mut *(*rc).zones.offset(i as isize)
            as *mut x264_zone_t;
        if frame_num >= (*z).i_start && frame_num <= (*z).i_end {
            return z;
        }
        i -= 1;
    }
    return 0 as *mut x264_zone_t;
}
#[no_mangle]
#[c2rust::src_loc = "1345:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_summary(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if (*rc).b_abr != 0 && (*h).param.rc.i_rc_method == X264_RC_ABR
        && (*rc).cbr_decay > 0.9999f64
    {
        let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
            * (if (*h).param.i_bframe != 0 {
                120 as ::core::ffi::c_int
            } else {
                80 as ::core::ffi::c_int
            })) as ::core::ffi::c_double;
        let mut mbtree_offset: ::core::ffi::c_double = if (*h).param.rc.b_mb_tree != 0 {
            (1.0f64 - (*h).param.rc.f_qcompress as ::core::ffi::c_double) * 13.5f64
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        };
        x264_10_log(
            h,
            X264_LOG_INFO,
            b"final ratefactor: %.2f\n\0" as *const u8 as *const ::core::ffi::c_char,
            qscale2qp(
                (pow(
                    base_cplx,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rc).qcompress,
                ) * (*rc).cplxr_sum / (*rc).wanted_bits_window) as ::core::ffi::c_float,
            ) as ::core::ffi::c_double - mbtree_offset
                - QP_BD_OFFSET as ::core::ffi::c_double,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1358:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_delete(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut b_regular_file: ::core::ffi::c_int = 0;
    if !(*rc).p_stat_file_out.is_null() {
        b_regular_file = x264_is_regular_file((*rc).p_stat_file_out);
        fclose((*rc).p_stat_file_out);
        if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
            if rename((*rc).psz_stat_file_tmpname, (*h).param.rc.psz_stat_out)
                != 0 as ::core::ffi::c_int
            {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"failed to rename \"%s\" to \"%s\"\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*rc).psz_stat_file_tmpname,
                    (*h).param.rc.psz_stat_out,
                );
            }
        }
        x264_free((*rc).psz_stat_file_tmpname as *mut ::core::ffi::c_void);
    }
    if !(*rc).p_mbtree_stat_file_out.is_null() {
        b_regular_file = x264_is_regular_file((*rc).p_mbtree_stat_file_out);
        fclose((*rc).p_mbtree_stat_file_out);
        if (*h).i_frame >= (*rc).num_entries && b_regular_file != 0 {
            if rename(
                (*rc).psz_mbtree_stat_file_tmpname,
                (*rc).psz_mbtree_stat_file_name,
            ) != 0 as ::core::ffi::c_int
            {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"failed to rename \"%s\" to \"%s\"\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*rc).psz_mbtree_stat_file_tmpname,
                    (*rc).psz_mbtree_stat_file_name,
                );
            }
        }
        x264_free((*rc).psz_mbtree_stat_file_tmpname as *mut ::core::ffi::c_void);
        x264_free((*rc).psz_mbtree_stat_file_name as *mut ::core::ffi::c_void);
    }
    if !(*rc).p_mbtree_stat_file_in.is_null() {
        fclose((*rc).p_mbtree_stat_file_in);
    }
    x264_free((*rc).pred as *mut ::core::ffi::c_void);
    x264_free((*rc).pred_b_from_p as *mut ::core::ffi::c_void);
    x264_free((*rc).entry as *mut ::core::ffi::c_void);
    x264_free((*rc).entry_out as *mut ::core::ffi::c_void);
    macroblock_tree_rescale_destroy(rc);
    if !(*rc).zones.is_null() {
        x264_param_cleanup(
            (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                as *mut x264_param_t,
        );
        x264_free(
            (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                as *mut ::core::ffi::c_void,
        );
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < (*rc).i_zones {
            if (*(*rc).zones.offset(i as isize)).param
                != (*(*rc).zones.offset(0 as ::core::ffi::c_int as isize)).param
                && (*(*(*rc).zones.offset(i as isize)).param).param_free.is_some()
            {
                x264_param_cleanup(
                    (*(*rc).zones.offset(i as isize)).param as *mut x264_param_t,
                );
                (*(*(*rc).zones.offset(i as isize)).param)
                    .param_free
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*rc).zones.offset(i as isize)).param as *mut ::core::ffi::c_void,
                );
            }
            i += 1;
        }
        x264_free((*rc).zones as *mut ::core::ffi::c_void);
    }
    x264_free(rc as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "1410:1"]
unsafe extern "C" fn accum_p_qp_update(
    mut h: *mut x264_t,
    mut qp: ::core::ffi::c_float,
) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    (*rc).accum_p_qp *= 0.95f64;
    (*rc).accum_p_norm *= 0.95f64;
    (*rc).accum_p_norm += 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*h).sh.i_type == SLICE_TYPE_I as ::core::ffi::c_int {
        (*rc).accum_p_qp += qp as ::core::ffi::c_double + (*rc).ip_offset;
    } else {
        (*rc).accum_p_qp += qp as ::core::ffi::c_double;
    };
}
#[no_mangle]
#[c2rust::src_loc = "1422:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_zone_init(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut zone: *mut x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
    if !zone.is_null()
        && ((*rc).prev_zone.is_null() || (*zone).param != (*(*rc).prev_zone).param)
    {
        x264_10_encoder_reconfig_apply(h, (*zone).param as *mut x264_param_t);
    }
    (*rc).prev_zone = zone;
}
#[no_mangle]
#[c2rust::src_loc = "1432:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_start(
    mut h: *mut x264_t,
    mut i_force_qp: ::core::ffi::c_int,
    mut overhead: ::core::ffi::c_int,
) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rce: *mut ratecontrol_entry_t = 0 as *mut ratecontrol_entry_t;
    let mut zone: *mut x264_zone_t = get_zone(h, (*(*h).fenc).i_frame);
    let mut q: ::core::ffi::c_float = 0.;
    if (*h).param.rc.b_stat_read != 0 {
        let mut frame: ::core::ffi::c_int = (*(*h).fenc).i_frame;
        if frame >= 0 as ::core::ffi::c_int && frame < (*rc).num_entries {} else {
            __assert_fail(
                b"frame >= 0 && frame < rc->num_entries\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"encoder/ratecontrol.c\0" as *const u8 as *const ::core::ffi::c_char,
                1444 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 51],
                    [::core::ffi::c_char; 51],
                >(*b"void x264_10_ratecontrol_start(x264_t *, int, int)\0")
                    .as_ptr(),
            );
        }
        'c_45562: {
            if frame >= 0 as ::core::ffi::c_int && frame < (*rc).num_entries {} else {
                __assert_fail(
                    b"frame >= 0 && frame < rc->num_entries\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"encoder/ratecontrol.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1444 as ::core::ffi::c_uint,
                    ::core::mem::transmute::<
                        [u8; 51],
                        [::core::ffi::c_char; 51],
                    >(*b"void x264_10_ratecontrol_start(x264_t *, int, int)\0")
                        .as_ptr(),
                );
            }
        };
        (*rc).rce = &mut *(*rc).entry.offset(frame as isize) as *mut ratecontrol_entry_t;
        rce = (*rc).rce;
        if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
            && (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_AUTO
        {
            (*h).sh.b_direct_spatial_mv_pred = ((*rce).direct_mode as ::core::ffi::c_int
                == 's' as i32) as ::core::ffi::c_int;
            (*h).mb.b_direct_auto_read = ((*rce).direct_mode as ::core::ffi::c_int
                == 's' as i32 || (*rce).direct_mode as ::core::ffi::c_int == 't' as i32)
                as ::core::ffi::c_int;
        }
    }
    if (*rc).b_vbv != 0 {
        memset(
            (*(*h).fdec).i_row_bits as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*h).mb.i_mb_height as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        );
        memset(
            (*(*h).fdec).f_row_qp as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*h).mb.i_mb_height as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        );
        memset(
            (*(*h).fdec).f_row_qscale as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*h).mb.i_mb_height as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        );
        (*rc).row_pred = (*(*rc).row_preds.as_mut_ptr().offset((*h).sh.i_type as isize))
            .as_mut_ptr();
        (*rc).buffer_rate = (*(*h).fenc).i_cpb_duration as ::core::ffi::c_double
            * (*rc).vbv_max_rate
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as ::core::ffi::c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
        update_vbv_plan(h, overhead);
        let mut l: *const x264_level_t = x264_levels.as_ptr();
        while (*l).level_idc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
        {
            l = l.offset(1);
        }
        let mut mincr: ::core::ffi::c_int = (*l).mincr as ::core::ffi::c_int;
        if (*h).param.b_bluray_compat != 0 {
            mincr = 4 as ::core::ffi::c_int;
        }
        if (*(*h).sps.as_mut_ptr()).i_profile_idc > PROFILE_HIGH as ::core::ffi::c_int {
            (*rc).frame_size_maximum = 1e9f64;
        } else if (*h).i_frame == 0 as ::core::ffi::c_int {
            let mut fr: ::core::ffi::c_double = 1.0f64
                / (if (*h).param.i_level_idc >= 60 as ::core::ffi::c_int {
                    300 as ::core::ffi::c_int
                } else {
                    172 as ::core::ffi::c_int
                }) as ::core::ffi::c_double;
            let mut pic_size_in_mbs: ::core::ffi::c_int = (*h).mb.i_mb_width
                * (*h).mb.i_mb_height;
            (*rc).frame_size_maximum = (384 as ::core::ffi::c_int * BIT_DEPTH)
                as ::core::ffi::c_double
                * (if pic_size_in_mbs as ::core::ffi::c_double
                    > fr * (*l).mbps as ::core::ffi::c_double
                {
                    pic_size_in_mbs as ::core::ffi::c_double
                } else {
                    fr * (*l).mbps as ::core::ffi::c_double
                }) / mincr as ::core::ffi::c_double;
        } else {
            (*rc).frame_size_maximum = (384 as ::core::ffi::c_int * BIT_DEPTH)
                as ::core::ffi::c_double
                * ((*(*h).fenc).i_cpb_duration as ::core::ffi::c_double
                    * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                        as ::core::ffi::c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double)
                * (*l).mbps as ::core::ffi::c_double / mincr as ::core::ffi::c_double;
        }
    }
    if (*h).sh.i_type != SLICE_TYPE_B as ::core::ffi::c_int {
        (*rc).bframes = (*(*h).fenc).i_bframes as ::core::ffi::c_int;
    }
    if (*rc).b_abr != 0 {
        q = qscale2qp(rate_estimate_qscale(h));
    } else if (*rc).b_2pass != 0 {
        (*rce).new_qscale = rate_estimate_qscale(h) as ::core::ffi::c_double;
        q = qscale2qp((*rce).new_qscale as ::core::ffi::c_float);
    } else {
        if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
            && (*(*h).fdec).b_kept_as_ref != 0
        {
            q = (((*rc).qp_constant[SLICE_TYPE_B as ::core::ffi::c_int as usize]
                + (*rc).qp_constant[SLICE_TYPE_P as ::core::ffi::c_int as usize])
                / 2 as ::core::ffi::c_int) as ::core::ffi::c_float;
        } else {
            q = (*rc).qp_constant[(*h).sh.i_type as usize] as ::core::ffi::c_float;
        }
        if !zone.is_null() {
            if (*zone).b_force_qp != 0 {
                q
                    += ((*zone).i_qp
                        - (*rc).qp_constant[SLICE_TYPE_P as ::core::ffi::c_int as usize])
                        as ::core::ffi::c_float;
            } else {
                q
                    -= 6 as ::core::ffi::c_int as ::core::ffi::c_float
                        * log2f((*zone).f_bitrate_factor);
            }
        }
    }
    if i_force_qp != X264_QP_AUTO {
        q = (i_force_qp - 1 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    q = x264_clip3f(
        q as ::core::ffi::c_double,
        (*h).param.rc.i_qp_min as ::core::ffi::c_double,
        (*h).param.rc.i_qp_max as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    (*rc).qpa_aq_prev = 0 as ::core::ffi::c_int;
    (*rc).qpa_aq = (*rc).qpa_aq_prev;
    (*rc).qpa_rc_prev = (*rc).qpa_aq as ::core::ffi::c_float;
    (*rc).qpa_rc = (*rc).qpa_rc_prev;
    (*rc).qpm = q;
    (*(*h).fdec).f_qp_avg_aq = (*rc).qpm;
    (*(*h).fdec).f_qp_avg_rc = (*(*h).fdec).f_qp_avg_aq;
    if !rce.is_null() {
        (*rce).new_qp = q;
    }
    accum_p_qp_update(h, (*rc).qpm);
    if (*h).sh.i_type != SLICE_TYPE_B as ::core::ffi::c_int {
        (*rc).last_non_b_pict_type = (*h).sh.i_type;
    }
}
#[c2rust::src_loc = "1540:1"]
unsafe extern "C" fn predict_row_size(
    mut h: *mut x264_t,
    mut y: ::core::ffi::c_int,
    mut qscale: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut pred_s: ::core::ffi::c_float = predict_size(
        &mut *(*rc).row_pred.offset(0 as ::core::ffi::c_int as isize),
        qscale,
        *(*(*h).fdec).i_row_satd.offset(y as isize) as ::core::ffi::c_float,
    );
    if (*h).sh.i_type == SLICE_TYPE_I as ::core::ffi::c_int
        || qscale
            >= *(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .f_row_qscale
                .offset(y as isize)
    {
        if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int
            && (*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_type == (*(*h).fdec).i_type
            && *(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .f_row_qscale
                .offset(y as isize) > 0 as ::core::ffi::c_int as ::core::ffi::c_float
            && *(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_row_satd
                .offset(y as isize) > 0 as ::core::ffi::c_int
            && abs(
                *(*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .i_row_satd
                    .offset(y as isize) - *(*(*h).fdec).i_row_satd.offset(y as isize),
            ) < *(*(*h).fdec).i_row_satd.offset(y as isize) / 2 as ::core::ffi::c_int
        {
            let mut pred_t: ::core::ffi::c_float = (*(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .i_row_bits
                .offset(y as isize) * *(*(*h).fdec).i_row_satd.offset(y as isize)
                / *(*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .i_row_satd
                    .offset(y as isize)) as ::core::ffi::c_float
                * *(*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qscale
                    .offset(y as isize) / qscale;
            return (pred_s + pred_t) * 0.5f32;
        }
        return pred_s;
    } else {
        let mut pred_intra: ::core::ffi::c_float = predict_size(
            &mut *(*rc).row_pred.offset(1 as ::core::ffi::c_int as isize),
            qscale,
            *(*(*h).fdec)
                .i_row_satds[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .offset(y as isize) as ::core::ffi::c_float,
        );
        return pred_intra + pred_s;
    };
}
#[c2rust::src_loc = "1569:1"]
unsafe extern "C" fn row_bits_so_far(
    mut h: *mut x264_t,
    mut y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = (*h).i_threadslice_start;
    while i <= y {
        bits += *(*(*h).fdec).i_row_bits.offset(i as isize);
        i += 1;
    }
    return bits;
}
#[c2rust::src_loc = "1577:1"]
unsafe extern "C" fn predict_row_size_to_end(
    mut h: *mut x264_t,
    mut y: ::core::ffi::c_int,
    mut qp: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    let mut qscale: ::core::ffi::c_float = qp2qscale(qp);
    let mut bits: ::core::ffi::c_float = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    let mut i: ::core::ffi::c_int = y + 1 as ::core::ffi::c_int;
    while i < (*h).i_threadslice_end {
        bits += predict_row_size(h, i, qscale);
        i += 1;
    }
    return bits;
}
#[no_mangle]
#[c2rust::src_loc = "1590:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_mb(
    mut h: *mut x264_t,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let y: ::core::ffi::c_int = (*h).mb.i_mb_y;
    *(*(*h).fdec).i_row_bits.offset(y as isize) += bits;
    (*rc).qpa_aq += (*h).mb.i_qp;
    if (*h).mb.i_mb_x != (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (*rc).qpa_rc += (*rc).qpm * (*h).mb.i_mb_width as ::core::ffi::c_float;
    if (*rc).b_vbv == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut qscale: ::core::ffi::c_float = qp2qscale((*rc).qpm);
    *(*(*h).fdec).f_row_qp.offset(y as isize) = (*rc).qpm;
    *(*(*h).fdec).f_row_qscale.offset(y as isize) = qscale;
    update_predictor(
        &mut *(*rc).row_pred.offset(0 as ::core::ffi::c_int as isize),
        qscale,
        *(*(*h).fdec).i_row_satd.offset(y as isize) as ::core::ffi::c_float,
        *(*(*h).fdec).i_row_bits.offset(y as isize) as ::core::ffi::c_float,
    );
    if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int
        && (*rc).qpm
            < *(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .f_row_qp
                .offset(y as isize)
    {
        update_predictor(
            &mut *(*rc).row_pred.offset(1 as ::core::ffi::c_int as isize),
            qscale,
            *(*(*h).fdec)
                .i_row_satds[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .offset(y as isize) as ::core::ffi::c_float,
            *(*(*h).fdec).i_row_bits.offset(y as isize) as ::core::ffi::c_float,
        );
    }
    if (*h).sh.b_mbaff != 0 && y & 1 as ::core::ffi::c_int == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut can_reencode_row: ::core::ffi::c_int = ((*h).sh.i_first_mb
        <= ((*h).mb.i_mb_y - (*h).sh.b_mbaff) * (*h).mb.i_mb_stride)
        as ::core::ffi::c_int;
    let mut prev_row_qp: ::core::ffi::c_float = *(*(*h).fdec)
        .f_row_qp
        .offset(y as isize);
    let mut qp_absolute_max: ::core::ffi::c_float = (*h).param.rc.i_qp_max
        as ::core::ffi::c_float;
    if (*rc).rate_factor_max_increment != 0. {
        qp_absolute_max = if qp_absolute_max
            < (*rc).qp_novbv + (*rc).rate_factor_max_increment
        {
            qp_absolute_max
        } else {
            (*rc).qp_novbv + (*rc).rate_factor_max_increment
        };
    }
    let mut qp_max: ::core::ffi::c_float = if (prev_row_qp
        + (*h).param.rc.i_qp_step as ::core::ffi::c_float) < qp_absolute_max
    {
        prev_row_qp + (*h).param.rc.i_qp_step as ::core::ffi::c_float
    } else {
        qp_absolute_max
    };
    let mut qp_min: ::core::ffi::c_float = if prev_row_qp
        - (*h).param.rc.i_qp_step as ::core::ffi::c_float
        > (*h).param.rc.i_qp_min as ::core::ffi::c_float
    {
        prev_row_qp - (*h).param.rc.i_qp_step as ::core::ffi::c_float
    } else {
        (*h).param.rc.i_qp_min as ::core::ffi::c_float
    };
    let mut step_size: ::core::ffi::c_float = 0.5f32;
    let mut slice_size_planned: ::core::ffi::c_float = (if (*h).param.b_sliced_threads
        != 0
    {
        (*rc).slice_size_planned
    } else {
        (*rc).frame_size_planned
    }) as ::core::ffi::c_float;
    let mut bits_so_far: ::core::ffi::c_float = row_bits_so_far(h, y)
        as ::core::ffi::c_float;
    ::core::ptr::write_volatile(
        &mut (*rc).bits_so_far as *mut ::core::ffi::c_float,
        bits_so_far,
    );
    let mut max_frame_error: ::core::ffi::c_float = x264_clip3f(
        1.0f64 / (*h).mb.i_mb_height as ::core::ffi::c_double,
        0.05f64,
        0.25f64,
    ) as ::core::ffi::c_float;
    let mut max_frame_size: ::core::ffi::c_float = ((*rc).frame_size_maximum
        - (*rc).frame_size_maximum * max_frame_error as ::core::ffi::c_double)
        as ::core::ffi::c_float;
    max_frame_size = (if (max_frame_size as ::core::ffi::c_double)
        < (*rc).buffer_fill
            - (*rc).buffer_rate * max_frame_error as ::core::ffi::c_double
    {
        max_frame_size as ::core::ffi::c_double
    } else {
        (*rc).buffer_fill - (*rc).buffer_rate * max_frame_error as ::core::ffi::c_double
    }) as ::core::ffi::c_float;
    let mut size_of_other_slices: ::core::ffi::c_float = 0 as ::core::ffi::c_int
        as ::core::ffi::c_float;
    if (*h).param.b_sliced_threads != 0 {
        let mut bits_so_far_of_other_slices: ::core::ffi::c_float = 0
            as ::core::ffi::c_int as ::core::ffi::c_float;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            if h != (*h).thread[i as usize] {
                size_of_other_slices
                    += (*(*(*h).thread[i as usize]).rc).frame_size_estimated;
                bits_so_far_of_other_slices
                    += (*(*(*h).thread[i as usize]).rc).bits_so_far;
            }
            i += 1;
        }
        let mut weight: ::core::ffi::c_float = x264_clip3f(
            ((bits_so_far_of_other_slices + (*rc).frame_size_estimated)
                / (size_of_other_slices + (*rc).frame_size_estimated))
                as ::core::ffi::c_double,
            0.0f64,
            1.0f64,
        ) as ::core::ffi::c_float;
        let mut frame_size_planned: ::core::ffi::c_float = ((*rc).frame_size_planned
            - (*rc).frame_size_planned * max_frame_error as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        let mut size_of_other_slices_planned: ::core::ffi::c_float = ((if frame_size_planned
            < max_frame_size
        {
            frame_size_planned
        } else {
            max_frame_size
        }) as ::core::ffi::c_double - (*rc).slice_size_planned) as ::core::ffi::c_float;
        size_of_other_slices_planned = if size_of_other_slices_planned
            > bits_so_far_of_other_slices
        {
            size_of_other_slices_planned
        } else {
            bits_so_far_of_other_slices
        };
        size_of_other_slices = (size_of_other_slices - size_of_other_slices_planned)
            * weight + size_of_other_slices_planned;
    }
    if y < (*h).i_threadslice_end - 1 as ::core::ffi::c_int {
        if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
            qp_min = if qp_min
                > (if *(*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as ::core::ffi::c_int) as isize)
                    > *(*(*h)
                        .fref[1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                {
                    *(*(*h)
                        .fref[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                } else {
                    *(*(*h)
                        .fref[1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize])
                        .f_row_qp
                        .offset((y + 1 as ::core::ffi::c_int) as isize)
                })
            {
                qp_min
            } else if *(*(*h)
                .fref[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize])
                .f_row_qp
                .offset((y + 1 as ::core::ffi::c_int) as isize)
                > *(*(*h)
                    .fref[1 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as ::core::ffi::c_int) as isize)
            {
                *(*(*h)
                    .fref[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as ::core::ffi::c_int) as isize)
            } else {
                *(*(*h)
                    .fref[1 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize])
                    .f_row_qp
                    .offset((y + 1 as ::core::ffi::c_int) as isize)
            };
            (*rc).qpm = if (*rc).qpm > qp_min { (*rc).qpm } else { qp_min };
        }
        let mut buffer_left_planned: ::core::ffi::c_float = ((*rc).buffer_fill
            - (*rc).frame_size_planned) as ::core::ffi::c_float;
        buffer_left_planned = if buffer_left_planned > 0.0f32 {
            buffer_left_planned
        } else {
            0.0f32
        };
        let mut rc_tol: ::core::ffi::c_float = ((buffer_left_planned
            / (*h).param.i_threads as ::core::ffi::c_float) as ::core::ffi::c_double
            * (*rc).rate_tolerance) as ::core::ffi::c_float;
        let mut b1: ::core::ffi::c_float = bits_so_far
            + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        let mut trust_coeff: ::core::ffi::c_float = x264_clip3f(
            (bits_so_far / slice_size_planned) as ::core::ffi::c_double,
            0.0f64,
            1.0f64,
        ) as ::core::ffi::c_float;
        if trust_coeff < 0.05f32 {
            qp_absolute_max = prev_row_qp;
            qp_max = qp_absolute_max;
        }
        if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int {
            rc_tol *= 0.5f32;
        }
        if (*rc).b_vbv_min_rate == 0 {
            qp_min = if qp_min > (*rc).qp_novbv { qp_min } else { (*rc).qp_novbv };
        }
        while (*rc).qpm < qp_max
            && (b1 as ::core::ffi::c_double
                > (*rc).frame_size_planned + rc_tol as ::core::ffi::c_double
                || b1 as ::core::ffi::c_double > (*rc).frame_size_planned
                    && (*rc).qpm < (*rc).qp_novbv
                || b1 as ::core::ffi::c_double
                    > (*rc).buffer_fill
                        - (buffer_left_planned * 0.5f32) as ::core::ffi::c_double)
        {
            (*rc).qpm += step_size;
            b1 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm)
                + size_of_other_slices;
        }
        let mut b_max: ::core::ffi::c_float = (b1 as ::core::ffi::c_double
            + (((*rc).buffer_fill - (*rc).buffer_size + (*rc).buffer_rate) * 0.90f64
                - b1 as ::core::ffi::c_double) * trust_coeff as ::core::ffi::c_double)
            as ::core::ffi::c_float;
        (*rc).qpm -= step_size;
        let mut b2: ::core::ffi::c_float = bits_so_far
            + predict_row_size_to_end(h, y, (*rc).qpm) + size_of_other_slices;
        while (*rc).qpm > qp_min && (*rc).qpm < prev_row_qp
            && ((*rc).qpm
                > *(*(*h).fdec).f_row_qp.offset(0 as ::core::ffi::c_int as isize)
                || (*rc).single_frame_vbv != 0) && b2 < max_frame_size
            && ((b2 as ::core::ffi::c_double) < (*rc).frame_size_planned * 0.8f64
                || b2 < b_max)
        {
            b1 = b2;
            (*rc).qpm -= step_size;
            b2 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm)
                + size_of_other_slices;
        }
        (*rc).qpm += step_size;
        while (*rc).qpm < qp_absolute_max && b1 > max_frame_size {
            (*rc).qpm += step_size;
            b1 = bits_so_far + predict_row_size_to_end(h, y, (*rc).qpm)
                + size_of_other_slices;
        }
        ::core::ptr::write_volatile(
            &mut (*rc).frame_size_estimated as *mut ::core::ffi::c_float,
            b1 - size_of_other_slices,
        );
        if (*rc).qpm > qp_max && prev_row_qp < qp_max && can_reencode_row != 0 {
            (*rc).qpm = x264_clip3f(
                ((prev_row_qp + (*rc).qpm) * 0.5f32) as ::core::ffi::c_double,
                (prev_row_qp + 1.0f32) as ::core::ffi::c_double,
                qp_max as ::core::ffi::c_double,
            ) as ::core::ffi::c_float;
            (*rc).qpa_rc = (*rc).qpa_rc_prev;
            (*rc).qpa_aq = (*rc).qpa_aq_prev;
            *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as ::core::ffi::c_int;
            *(*(*h).fdec).i_row_bits.offset((y - (*h).sh.b_mbaff) as isize) = 0
                as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        ::core::ptr::write_volatile(
            &mut (*rc).frame_size_estimated as *mut ::core::ffi::c_float,
            bits_so_far,
        );
        if (*rc).qpm < qp_max && can_reencode_row != 0
            && (bits_so_far + size_of_other_slices) as ::core::ffi::c_double
                > (if (*rc).frame_size_maximum < (*rc).buffer_fill {
                    (*rc).frame_size_maximum
                } else {
                    (*rc).buffer_fill
                })
        {
            (*rc).qpm = qp_max;
            (*rc).qpa_rc = (*rc).qpa_rc_prev;
            (*rc).qpa_aq = (*rc).qpa_aq_prev;
            *(*(*h).fdec).i_row_bits.offset(y as isize) = 0 as ::core::ffi::c_int;
            *(*(*h).fdec).i_row_bits.offset((y - (*h).sh.b_mbaff) as isize) = 0
                as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*rc).qpa_rc_prev = (*rc).qpa_rc;
    (*rc).qpa_aq_prev = (*rc).qpa_aq;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1748:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_qp(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    return x264_clip3(
        ((*(*h).rc).qpm + 0.5f32) as ::core::ffi::c_int,
        (*h).param.rc.i_qp_min,
        (*h).param.rc.i_qp_max,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1754:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_mb_qp(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut qp: ::core::ffi::c_float = (*(*h).rc).qpm;
    if (*h).param.rc.i_aq_mode != 0 {
        let mut qp_offset: ::core::ffi::c_float = if (*(*h).fdec).b_kept_as_ref != 0 {
            *(*(*h).fenc).f_qp_offset.offset((*h).mb.i_mb_xy as isize)
        } else {
            *(*(*h).fenc).f_qp_offset_aq.offset((*h).mb.i_mb_xy as isize)
        };
        if qp > QP_MAX_SPEC as ::core::ffi::c_float {
            qp_offset
                *= (QP_MAX as ::core::ffi::c_float - qp)
                    / (QP_MAX - QP_MAX_SPEC) as ::core::ffi::c_float;
        }
        qp += qp_offset;
    }
    return x264_clip3(
        (qp + 0.5f32) as ::core::ffi::c_int,
        (*h).param.rc.i_qp_min,
        (*h).param.rc.i_qp_max,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1771:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_slice_type(
    mut h: *mut x264_t,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    if (*h).param.rc.b_stat_read != 0 {
        if frame_num >= (*rc).num_entries {
            (*h).param.rc.i_qp_constant = (if (*h)
                .stat
                .i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                == 0 as ::core::ffi::c_int
            {
                (24 as ::core::ffi::c_int + QP_BD_OFFSET) as ::core::ffi::c_double
            } else {
                1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + (*h).stat.f_frame_qp[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                        / (*h)
                            .stat
                            .i_frame_count[SLICE_TYPE_P as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_double
            }) as ::core::ffi::c_int;
            (*rc).qp_constant[SLICE_TYPE_P as ::core::ffi::c_int as usize] = x264_clip3(
                (*h).param.rc.i_qp_constant,
                0 as ::core::ffi::c_int,
                QP_MAX,
            );
            (*rc).qp_constant[SLICE_TYPE_I as ::core::ffi::c_int as usize] = x264_clip3(
                (qscale2qp(
                    qp2qscale((*h).param.rc.i_qp_constant as ::core::ffi::c_float)
                        / (*h).param.rc.f_ip_factor,
                ) as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                QP_MAX,
            );
            (*rc).qp_constant[SLICE_TYPE_B as ::core::ffi::c_int as usize] = x264_clip3(
                (qscale2qp(
                    qp2qscale((*h).param.rc.i_qp_constant as ::core::ffi::c_float)
                        * (*h).param.rc.f_pb_factor,
                ) as ::core::ffi::c_double + 0.5f64) as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                QP_MAX,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"2nd pass has more frames than 1st pass (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*rc).num_entries,
            );
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"continuing anyway, at constant QP=%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.rc.i_qp_constant,
            );
            if (*h).param.i_bframe_adaptive != 0 {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"disabling adaptive B-frames\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).param.i_threads {
                (*(*(*h).thread[i as usize]).rc).b_abr = 0 as ::core::ffi::c_int;
                (*(*(*h).thread[i as usize]).rc).b_2pass = 0 as ::core::ffi::c_int;
                (*(*h).thread[i as usize]).param.rc.i_rc_method = X264_RC_CQP;
                (*(*h).thread[i as usize]).param.rc.b_stat_read = 0
                    as ::core::ffi::c_int;
                (*(*h).thread[i as usize]).param.i_bframe_adaptive = 0
                    as ::core::ffi::c_int;
                (*(*h).thread[i as usize]).param.i_scenecut_threshold = 0
                    as ::core::ffi::c_int;
                (*(*h).thread[i as usize]).param.rc.b_mb_tree = 0 as ::core::ffi::c_int;
                if (*(*h).thread[i as usize]).param.i_bframe > 1 as ::core::ffi::c_int {
                    (*(*h).thread[i as usize]).param.i_bframe = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            return X264_TYPE_AUTO;
        }
        return (*(*rc).entry.offset(frame_num as isize)).frame_type;
    } else {
        return X264_TYPE_AUTO
    };
}
#[no_mangle]
#[c2rust::src_loc = "1812:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_set_weights(
    mut h: *mut x264_t,
    mut frm: *mut x264_frame_t,
) {
    let mut rce: *mut ratecontrol_entry_t = &mut *(*(*h).rc)
        .entry
        .offset((*frm).i_frame as isize) as *mut ratecontrol_entry_t;
    if (*h).param.analyse.i_weighted_pred <= 0 as ::core::ffi::c_int {
        return;
    }
    if (*rce).i_weight_denom[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        >= 0 as ::core::ffi::c_int
    {
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            .i_scale = (*rce)
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            .i_denom = (*rce).i_weight_denom[0 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            .i_offset = (*rce)
            .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            as int32_t;
        (*h)
            .mc
            .weight_cache
            .expect(
                "non-null function pointer",
            )(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize),
        );
    }
    if (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        >= 0 as ::core::ffi::c_int
    {
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            .i_scale = (*rce)
            .weight[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            .i_denom = (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            .i_offset = (*rce)
            .weight[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            as int32_t;
        (*h)
            .mc
            .weight_cache
            .expect(
                "non-null function pointer",
            )(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize),
        );
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
            .i_scale = (*rce)
            .weight[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
            .i_denom = (*rce).i_weight_denom[1 as ::core::ffi::c_int as usize]
            as int32_t;
        (*frm)
            .weight[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize]
            .i_offset = (*rce)
            .weight[2 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            as int32_t;
        (*h)
            .mc
            .weight_cache
            .expect(
                "non-null function pointer",
            )(
            h,
            &mut *(*(*frm).weight.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize),
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "1829:1"]
pub unsafe extern "C" fn x264_10_ratecontrol_end(
    mut h: *mut x264_t,
    mut bits: ::core::ffi::c_int,
    mut filler: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut mbs: *const ::core::ffi::c_int = (*h).stat.frame.i_mb_count.as_mut_ptr();
    (*h).stat.frame.i_mb_count_skip = *mbs.offset(P_SKIP as ::core::ffi::c_int as isize)
        + *mbs.offset(B_SKIP as ::core::ffi::c_int as isize);
    (*h).stat.frame.i_mb_count_i = *mbs.offset(I_16x16 as ::core::ffi::c_int as isize)
        + *mbs.offset(I_8x8 as ::core::ffi::c_int as isize)
        + *mbs.offset(I_4x4 as ::core::ffi::c_int as isize)
        + *mbs.offset(I_PCM as ::core::ffi::c_int as isize);
    (*h).stat.frame.i_mb_count_p = *mbs.offset(P_L0 as ::core::ffi::c_int as isize)
        + *mbs.offset(P_8x8 as ::core::ffi::c_int as isize);
    let mut i: ::core::ffi::c_int = B_DIRECT as ::core::ffi::c_int;
    while i <= B_8x8 as ::core::ffi::c_int {
        (*h).stat.frame.i_mb_count_p += *mbs.offset(i as isize);
        i += 1;
    }
    (*rc).qpa_rc /= (*h).mb.i_mb_count as ::core::ffi::c_float;
    (*(*h).fdec).f_qp_avg_rc = (*rc).qpa_rc;
    (*(*h).fdec).f_qp_avg_aq = (*rc).qpa_aq as ::core::ffi::c_float
        / (*h).mb.i_mb_count as ::core::ffi::c_float;
    (*(*h).fdec).f_crf_avg = (*h).param.rc.f_rf_constant + (*(*h).fdec).f_qp_avg_rc
        - (*rc).qp_novbv;
    if (*h).param.rc.b_stat_write != 0 {
        let mut c_type: ::core::ffi::c_char = (if (*h).sh.i_type
            == SLICE_TYPE_I as ::core::ffi::c_int
        {
            if (*(*h).fenc).i_poc == 0 as ::core::ffi::c_int {
                'I' as i32
            } else {
                'i' as i32
            }
        } else if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
            'P' as i32
        } else if (*(*h).fenc).b_kept_as_ref != 0 {
            'B' as i32
        } else {
            'b' as i32
        }) as ::core::ffi::c_char;
        let mut dir_frame: ::core::ffi::c_int = (*h)
            .stat
            .frame
            .i_direct_score[1 as ::core::ffi::c_int as usize]
            - (*h).stat.frame.i_direct_score[0 as ::core::ffi::c_int as usize];
        let mut dir_avg: ::core::ffi::c_int = (*h)
            .stat
            .i_direct_score[1 as ::core::ffi::c_int as usize]
            - (*h).stat.i_direct_score[0 as ::core::ffi::c_int as usize];
        let mut c_direct: ::core::ffi::c_char = (if (*h).mb.b_direct_auto_write != 0 {
            if dir_frame > 0 as ::core::ffi::c_int {
                's' as i32
            } else if dir_frame < 0 as ::core::ffi::c_int {
                't' as i32
            } else if dir_avg > 0 as ::core::ffi::c_int {
                's' as i32
            } else if dir_avg < 0 as ::core::ffi::c_int {
                't' as i32
            } else {
                '-' as i32
            }
        } else {
            '-' as i32
        }) as ::core::ffi::c_char;
        if fprintf(
            (*rc).p_stat_file_out,
            b"in:%d out:%d type:%c dur:%ld cpbdur:%ld q:%.2f aq:%.2f tex:%d mv:%d misc:%d imb:%d pmb:%d smb:%d d:%c ref:\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*(*h).fenc).i_frame,
            (*h).i_frame,
            c_type as ::core::ffi::c_int,
            (*(*h).fenc).i_duration,
            (*(*h).fenc).i_cpb_duration,
            (*rc).qpa_rc as ::core::ffi::c_double,
            (*(*h).fdec).f_qp_avg_aq as ::core::ffi::c_double,
            (*h).stat.frame.i_tex_bits,
            (*h).stat.frame.i_mv_bits,
            (*h).stat.frame.i_misc_bits,
            (*h).stat.frame.i_mb_count_i,
            (*h).stat.frame.i_mb_count_p,
            (*h).stat.frame.i_mb_count_skip,
            c_direct as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            current_block = 13175027128378642164;
        } else {
            let mut use_old_stats: ::core::ffi::c_int = ((*h).param.rc.b_stat_read != 0
                && (*(*rc).rce).refs > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                if !(i_0
                    < (if use_old_stats != 0 {
                        (*(*rc).rce).refs
                    } else {
                        (*h).i_ref[0 as ::core::ffi::c_int as usize]
                    }))
                {
                    current_block = 8236137900636309791;
                    break;
                }
                let mut refcount: ::core::ffi::c_int = if use_old_stats != 0 {
                    (*(*rc).rce).refcount[i_0 as usize]
                } else if (*h).param.b_interlaced != 0 {
                    (*h)
                        .stat
                        .frame
                        .i_mb_count_ref[0 as ::core::ffi::c_int
                        as usize][(i_0 * 2 as ::core::ffi::c_int) as usize]
                        + (*h)
                            .stat
                            .frame
                            .i_mb_count_ref[0 as ::core::ffi::c_int
                            as usize][(i_0 * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as usize]
                } else {
                    (*h)
                        .stat
                        .frame
                        .i_mb_count_ref[0 as ::core::ffi::c_int as usize][i_0 as usize]
                };
                if fprintf(
                    (*rc).p_stat_file_out,
                    b"%d \0" as *const u8 as *const ::core::ffi::c_char,
                    refcount,
                ) < 0 as ::core::ffi::c_int
                {
                    current_block = 13175027128378642164;
                    break;
                }
                i_0 += 1;
            }
            match current_block {
                13175027128378642164 => {}
                _ => {
                    if (*h).param.analyse.i_weighted_pred >= X264_WEIGHTP_SIMPLE
                        && !(*h)
                            .sh
                            .weight[0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int as usize]
                            .weightfn
                            .is_null()
                    {
                        if fprintf(
                            (*rc).p_stat_file_out,
                            b"w:%d,%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
                            (*h)
                                .sh
                                .weight[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize]
                                .i_denom,
                            (*h)
                                .sh
                                .weight[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize]
                                .i_scale,
                            (*h)
                                .sh
                                .weight[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize]
                                .i_offset,
                        ) < 0 as ::core::ffi::c_int
                        {
                            current_block = 13175027128378642164;
                        } else if !(*h)
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
                                .is_null()
                        {
                            if fprintf(
                                (*rc).p_stat_file_out,
                                b",%d,%d,%d,%d,%d \0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize]
                                    .i_denom,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize]
                                    .i_scale,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize]
                                    .i_offset,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][2 as ::core::ffi::c_int as usize]
                                    .i_scale,
                                (*h)
                                    .sh
                                    .weight[0 as ::core::ffi::c_int
                                        as usize][2 as ::core::ffi::c_int as usize]
                                    .i_offset,
                            ) < 0 as ::core::ffi::c_int
                            {
                                current_block = 13175027128378642164;
                            } else {
                                current_block = 12147880666119273379;
                            }
                        } else if fprintf(
                            (*rc).p_stat_file_out,
                            b" \0" as *const u8 as *const ::core::ffi::c_char,
                        ) < 0 as ::core::ffi::c_int
                        {
                            current_block = 13175027128378642164;
                        } else {
                            current_block = 12147880666119273379;
                        }
                    } else {
                        current_block = 12147880666119273379;
                    }
                    match current_block {
                        13175027128378642164 => {}
                        _ => {
                            if fprintf(
                                (*rc).p_stat_file_out,
                                b";\n\0" as *const u8 as *const ::core::ffi::c_char,
                            ) < 0 as ::core::ffi::c_int
                            {
                                current_block = 13175027128378642164;
                            } else if (*h).param.rc.b_mb_tree != 0
                                && (*(*h).fenc).b_kept_as_ref != 0
                                && (*h).param.rc.b_stat_read == 0
                            {
                                let mut i_type: uint8_t = (*h).sh.i_type as uint8_t;
                                (*h)
                                    .mc
                                    .mbtree_fix8_pack
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize],
                                    (*(*h).fenc).f_qp_offset,
                                    (*h).mb.i_mb_count,
                                );
                                if fwrite(
                                    &mut i_type as *mut uint8_t as *const ::core::ffi::c_void,
                                    1 as size_t,
                                    1 as size_t,
                                    (*rc).p_mbtree_stat_file_out,
                                ) < 1 as ::core::ffi::c_ulong
                                {
                                    current_block = 13175027128378642164;
                                } else if fwrite(
                                    (*rc).mbtree.qp_buffer[0 as ::core::ffi::c_int as usize]
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<uint16_t>() as size_t,
                                    (*h).mb.i_mb_count as size_t,
                                    (*rc).p_mbtree_stat_file_out,
                                )
                                    < (*h).mb.i_mb_count as ::core::ffi::c_uint
                                        as ::core::ffi::c_ulong
                                {
                                    current_block = 13175027128378642164;
                                } else {
                                    current_block = 15125582407903384992;
                                }
                            } else {
                                current_block = 15125582407903384992;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            15125582407903384992 => {}
            _ => {
                x264_10_log(
                    h,
                    X264_LOG_ERROR,
                    b"ratecontrol_end: stats file could not be written to\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    if (*rc).b_abr != 0 {
        if (*h).sh.i_type != SLICE_TYPE_B as ::core::ffi::c_int {
            (*rc).cplxr_sum
                += (bits as ::core::ffi::c_float * qp2qscale((*rc).qpa_rc))
                    as ::core::ffi::c_double / (*rc).last_rceq;
        } else {
            (*rc).cplxr_sum
                += (bits as ::core::ffi::c_float * qp2qscale((*rc).qpa_rc))
                    as ::core::ffi::c_double
                    / ((*rc).last_rceq
                        * (*h).param.rc.f_pb_factor as ::core::ffi::c_double);
        }
        (*rc).cplxr_sum *= (*rc).cbr_decay;
        (*rc).wanted_bits_window
            += (*(*h).fenc).f_duration as ::core::ffi::c_double * (*rc).bitrate;
        (*rc).wanted_bits_window *= (*rc).cbr_decay;
    }
    if (*rc).b_2pass != 0 {
        (*rc).expected_bits_sum
            += qscale2bits(
                (*rc).rce,
                qp2qscale((*(*rc).rce).new_qp) as ::core::ffi::c_double,
            );
    }
    if (*h).mb.b_variable_qp != 0 {
        if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
            (*rc).bframe_bits += bits;
            if (*(*h).fenc).b_last_minigop_bframe != 0 {
                update_predictor(
                    (*rc).pred_b_from_p,
                    qp2qscale((*rc).qpa_rc),
                    (*(*h)
                        .fref[1 as ::core::ffi::c_int
                        as usize][((*h).i_ref[1 as ::core::ffi::c_int as usize]
                        - 1 as ::core::ffi::c_int) as usize])
                        .i_satd as ::core::ffi::c_float,
                    ((*rc).bframe_bits / (*rc).bframes) as ::core::ffi::c_float,
                );
                (*rc).bframe_bits = 0 as ::core::ffi::c_int;
            }
        }
    }
    *filler = update_vbv(h, bits);
    (*rc).filler_bits_sum += (*filler * 8 as ::core::ffi::c_int) as int64_t;
    if (*(*h).sps.as_mut_ptr()).vui.b_nal_hrd_parameters_present != 0 {
        if (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int {
            (*(*h).fenc).hrd_timing.cpb_initial_arrival_time = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
            (*rc).initial_cpb_removal_delay_offset = (*h)
                .initial_cpb_removal_delay_offset;
            (*rc).nrt_first_access_unit = (*rc).initial_cpb_removal_delay
                as ::core::ffi::c_double
                / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit;
        } else {
            (*(*h).fenc).hrd_timing.cpb_removal_time = (*rc).nrt_first_access_unit
                + ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset)
                    as ::core::ffi::c_double
                    * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                        as ::core::ffi::c_double
                    / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
            if (*(*h).fenc).b_keyframe != 0 {
                (*rc).nrt_first_access_unit = (*(*h).fenc).hrd_timing.cpb_removal_time;
                (*rc).initial_cpb_removal_delay = (*h).initial_cpb_removal_delay;
                (*rc).initial_cpb_removal_delay_offset = (*h)
                    .initial_cpb_removal_delay_offset;
            }
            let mut cpb_earliest_arrival_time: ::core::ffi::c_double = (*(*h).fenc)
                .hrd_timing
                .cpb_removal_time
                - (*rc).initial_cpb_removal_delay as ::core::ffi::c_double
                    / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
            if (*(*h).fenc).b_keyframe == 0 {
                cpb_earliest_arrival_time
                    -= (*rc).initial_cpb_removal_delay_offset as ::core::ffi::c_double
                        / 90000 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            if (*(*h).sps.as_mut_ptr()).vui.hrd.b_cbr_hrd != 0 {
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time = (*rc)
                    .previous_cpb_final_arrival_time;
            } else {
                (*(*h).fenc).hrd_timing.cpb_initial_arrival_time = if (*rc)
                    .previous_cpb_final_arrival_time > cpb_earliest_arrival_time
                {
                    (*rc).previous_cpb_final_arrival_time
                } else {
                    cpb_earliest_arrival_time
                };
            }
        }
        let mut filler_bits: ::core::ffi::c_int = if *filler != 0 {
            (if 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
                > *filler
            {
                5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int - (*h).param.b_annexb
            } else {
                *filler
            }) * 8 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        (*rc).previous_cpb_final_arrival_time = (*(*h).fenc)
            .hrd_timing
            .cpb_initial_arrival_time
            + (bits + filler_bits) as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled
                    as ::core::ffi::c_double;
        (*(*h).fenc).hrd_timing.cpb_final_arrival_time = (*rc)
            .previous_cpb_final_arrival_time;
        (*(*h).fenc).hrd_timing.dpb_output_time = (*(*h).fenc).i_dpb_output_delay
            as ::core::ffi::c_double
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as ::core::ffi::c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double
            + (*(*h).fenc).hrd_timing.cpb_removal_time;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "2003:1"]
unsafe extern "C" fn get_qscale(
    mut h: *mut x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut rate_factor: ::core::ffi::c_double,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut zone: *mut x264_zone_t = get_zone(h, frame_num);
    let mut q: ::core::ffi::c_double = 0.;
    if (*h).param.rc.b_mb_tree != 0 {
        let mut timescale: ::core::ffi::c_double = (*(*h).sps.as_mut_ptr())
            .vui
            .i_num_units_in_tick as ::core::ffi::c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
        q = pow(
            (0.04f32
                / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_float) as ::core::ffi::c_double
                / x264_clip3f(
                    (*rce).i_duration as ::core::ffi::c_double * timescale,
                    (0.01f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                    (1.00f32
                        / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                            as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_float) as ::core::ffi::c_double,
                ),
            (1 as ::core::ffi::c_int as ::core::ffi::c_float - (*h).param.rc.f_qcompress)
                as ::core::ffi::c_double,
        );
    } else {
        q = pow(
            (*rce).blurred_complexity as ::core::ffi::c_double,
            1 as ::core::ffi::c_int as ::core::ffi::c_double - (*rcc).qcompress,
        );
    }
    if q.is_finite() as i32 == 0
        || (*rce).tex_bits + (*rce).mv_bits == 0 as ::core::ffi::c_int
    {
        q = (*rcc).last_qscale_for[(*rce).pict_type as usize];
    } else {
        (*rcc).last_rceq = q;
        q /= rate_factor;
        (*rcc).last_qscale = q;
    }
    if !zone.is_null() {
        if (*zone).b_force_qp != 0 {
            q = qp2qscale((*zone).i_qp as ::core::ffi::c_float) as ::core::ffi::c_double;
        } else {
            q /= (*zone).f_bitrate_factor as ::core::ffi::c_double;
        }
    }
    return q;
}
#[c2rust::src_loc = "2037:1"]
unsafe extern "C" fn get_diff_limited_q(
    mut h: *mut x264_t,
    mut rce: *mut ratecontrol_entry_t,
    mut q: ::core::ffi::c_double,
    mut frame_num: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let pict_type: ::core::ffi::c_int = (*rce).pict_type;
    let mut zone: *mut x264_zone_t = get_zone(h, frame_num);
    if pict_type == SLICE_TYPE_I as ::core::ffi::c_int {
        let mut iq: ::core::ffi::c_double = q;
        let mut pq: ::core::ffi::c_double = qp2qscale(
            ((*rcc).accum_p_qp / (*rcc).accum_p_norm) as ::core::ffi::c_float,
        ) as ::core::ffi::c_double;
        let mut ip_factor: ::core::ffi::c_double = (*h).param.rc.f_ip_factor
            as ::core::ffi::c_double;
        if (*rcc).accum_p_norm <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            q = iq;
        } else if (*rcc).accum_p_norm >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            q = pq / ip_factor;
        } else {
            q = (*rcc).accum_p_norm * pq / ip_factor
                + (1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - (*rcc).accum_p_norm) * iq;
        }
    } else if pict_type == SLICE_TYPE_B as ::core::ffi::c_int {
        q = (*rcc).last_qscale_for[(*rcc).last_non_b_pict_type as usize];
        if (*rce).kept_as_ref == 0 {
            q *= (*h).param.rc.f_pb_factor as ::core::ffi::c_double;
        }
    } else if pict_type == SLICE_TYPE_P as ::core::ffi::c_int
        && (*rcc).last_non_b_pict_type == SLICE_TYPE_P as ::core::ffi::c_int
        && (*rce).tex_bits == 0 as ::core::ffi::c_int
    {
        q = (*rcc).last_qscale_for[SLICE_TYPE_P as ::core::ffi::c_int as usize];
    }
    if (*rcc).last_non_b_pict_type == pict_type
        && (pict_type != SLICE_TYPE_I as ::core::ffi::c_int
            || (*rcc).last_accum_p_norm
                < 1 as ::core::ffi::c_int as ::core::ffi::c_double)
    {
        let mut last_q: ::core::ffi::c_double = (*rcc)
            .last_qscale_for[pict_type as usize];
        let mut max_qscale: ::core::ffi::c_double = last_q * (*rcc).lstep;
        let mut min_qscale: ::core::ffi::c_double = last_q / (*rcc).lstep;
        if q > max_qscale {
            q = max_qscale;
        } else if q < min_qscale {
            q = min_qscale;
        }
    }
    (*rcc).last_qscale_for[pict_type as usize] = q;
    if pict_type != SLICE_TYPE_B as ::core::ffi::c_int {
        (*rcc).last_non_b_pict_type = pict_type;
    }
    if pict_type == SLICE_TYPE_I as ::core::ffi::c_int {
        (*rcc).last_accum_p_norm = (*rcc).accum_p_norm;
        (*rcc).accum_p_norm = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*rcc).accum_p_qp = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if pict_type == SLICE_TYPE_P as ::core::ffi::c_int {
        let mut mask: ::core::ffi::c_float = (1 as ::core::ffi::c_int
            as ::core::ffi::c_double
            - pow(
                ((*rce).i_count as ::core::ffi::c_float
                    / (*rcc).nmb as ::core::ffi::c_float) as ::core::ffi::c_double,
                2 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as ::core::ffi::c_float;
        (*rcc).accum_p_qp = mask as ::core::ffi::c_double
            * (qscale2qp(q as ::core::ffi::c_float) as ::core::ffi::c_double
                + (*rcc).accum_p_qp);
        (*rcc).accum_p_norm = mask as ::core::ffi::c_double
            * (1 as ::core::ffi::c_int as ::core::ffi::c_double + (*rcc).accum_p_norm);
    }
    if !zone.is_null() {
        if (*zone).b_force_qp != 0 {
            q = qp2qscale((*zone).i_qp as ::core::ffi::c_float) as ::core::ffi::c_double;
        } else {
            q /= (*zone).f_bitrate_factor as ::core::ffi::c_double;
        }
    }
    return q;
}
#[c2rust::src_loc = "2109:1"]
unsafe extern "C" fn predict_size(
    mut p: *mut predictor_t,
    mut q: ::core::ffi::c_float,
    mut var: ::core::ffi::c_float,
) -> ::core::ffi::c_float {
    return ((*p).coeff * var + (*p).offset) / (q * (*p).count);
}
#[c2rust::src_loc = "2114:1"]
unsafe extern "C" fn update_predictor(
    mut p: *mut predictor_t,
    mut q: ::core::ffi::c_float,
    mut var: ::core::ffi::c_float,
    mut bits: ::core::ffi::c_float,
) {
    let mut range: ::core::ffi::c_float = 1.5f32;
    if var < 10 as ::core::ffi::c_int as ::core::ffi::c_float {
        return;
    }
    let mut old_coeff: ::core::ffi::c_float = (*p).coeff / (*p).count;
    let mut old_offset: ::core::ffi::c_float = (*p).offset / (*p).count;
    let mut new_coeff: ::core::ffi::c_float = if (bits * q - old_offset) / var
        > (*p).coeff_min
    {
        (bits * q - old_offset) / var
    } else {
        (*p).coeff_min
    };
    let mut new_coeff_clipped: ::core::ffi::c_float = x264_clip3f(
        new_coeff as ::core::ffi::c_double,
        (old_coeff / range) as ::core::ffi::c_double,
        (old_coeff * range) as ::core::ffi::c_double,
    ) as ::core::ffi::c_float;
    let mut new_offset: ::core::ffi::c_float = bits * q - new_coeff_clipped * var;
    if new_offset >= 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        new_coeff = new_coeff_clipped;
    } else {
        new_offset = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    }
    (*p).count *= (*p).decay;
    (*p).coeff *= (*p).decay;
    (*p).offset *= (*p).decay;
    (*p).count += 1.;
    (*p).coeff += new_coeff;
    (*p).offset += new_offset;
}
#[c2rust::src_loc = "2137:1"]
unsafe extern "C" fn update_vbv(
    mut h: *mut x264_t,
    mut bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut filler: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bitrate: ::core::ffi::c_int = (*(*h).sps.as_mut_ptr())
        .vui
        .hrd
        .i_bit_rate_unscaled;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rct: *mut x264_ratecontrol_t = (*(*h)
        .thread[0 as ::core::ffi::c_int as usize])
        .rc;
    let mut buffer_size: int64_t = (*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled
        as int64_t * (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t;
    if (*rcc).last_satd >= (*h).mb.i_mb_count {
        update_predictor(
            &mut *(*rct).pred.offset((*h).sh.i_type as isize),
            qp2qscale((*rcc).qpa_rc),
            (*rcc).last_satd as ::core::ffi::c_float,
            bits as ::core::ffi::c_float,
        );
    }
    if (*rcc).b_vbv == 0 {
        return filler;
    }
    let mut buffer_diff: uint64_t = (bits as uint64_t)
        .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
    (*rct).buffer_fill_final = ((*rct).buffer_fill_final as uint64_t)
        .wrapping_sub(buffer_diff) as int64_t as int64_t;
    (*rct).buffer_fill_final_min = ((*rct).buffer_fill_final_min as uint64_t)
        .wrapping_sub(buffer_diff) as int64_t as int64_t;
    if (*rct).buffer_fill_final_min < 0 as int64_t {
        let mut underflow: ::core::ffi::c_double = (*rct).buffer_fill_final_min
            as ::core::ffi::c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
        if (*rcc).rate_factor_max_increment != 0.
            && (*rcc).qpm >= (*rcc).qp_novbv + (*rcc).rate_factor_max_increment
        {
            x264_10_log(
                h,
                X264_LOG_DEBUG,
                b"VBV underflow due to CRF-max (frame %d, %.0f bits)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).i_frame,
                underflow,
            );
        } else {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV underflow (frame %d, %.0f bits)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).i_frame,
                underflow,
            );
        }
        (*rct).buffer_fill_final_min = 0 as int64_t;
        (*rct).buffer_fill_final = (*rct).buffer_fill_final_min;
    }
    if (*h).param.i_avcintra_class != 0 {
        buffer_diff = buffer_size as uint64_t;
    } else {
        buffer_diff = (bitrate as uint64_t)
            .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as uint64_t)
            .wrapping_mul((*(*h).fenc).i_cpb_duration as uint64_t);
    }
    (*rct).buffer_fill_final = ((*rct).buffer_fill_final as uint64_t)
        .wrapping_add(buffer_diff) as int64_t as int64_t;
    (*rct).buffer_fill_final_min = ((*rct).buffer_fill_final_min as uint64_t)
        .wrapping_add(buffer_diff) as int64_t as int64_t;
    if (*rct).buffer_fill_final > buffer_size {
        if (*h).param.rc.b_filler != 0 {
            let mut scale: int64_t = (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t
                * 8 as int64_t;
            filler = (((*rct).buffer_fill_final - buffer_size + scale - 1 as int64_t)
                / scale) as ::core::ffi::c_int;
            bits = if (*h).param.i_avcintra_class != 0 {
                filler * 8 as ::core::ffi::c_int
            } else {
                (if 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    - (*h).param.b_annexb > filler
                {
                    5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        - (*h).param.b_annexb
                } else {
                    filler
                }) * 8 as ::core::ffi::c_int
            };
            buffer_diff = (bits as uint64_t)
                .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
            (*rct).buffer_fill_final = ((*rct).buffer_fill_final as uint64_t)
                .wrapping_sub(buffer_diff) as int64_t as int64_t;
            (*rct).buffer_fill_final_min = ((*rct).buffer_fill_final_min as uint64_t)
                .wrapping_sub(buffer_diff) as int64_t as int64_t;
        } else {
            (*rct).buffer_fill_final = if (*rct).buffer_fill_final < buffer_size {
                (*rct).buffer_fill_final
            } else {
                buffer_size
            };
            (*rct).buffer_fill_final_min = if (*rct).buffer_fill_final_min < buffer_size
            {
                (*rct).buffer_fill_final_min
            } else {
                buffer_size
            };
        }
    }
    return filler;
}
#[no_mangle]
#[c2rust::src_loc = "2194:1"]
pub unsafe extern "C" fn x264_10_hrd_fullness(mut h: *mut x264_t) {
    let mut rct: *mut x264_ratecontrol_t = (*(*h)
        .thread[0 as ::core::ffi::c_int as usize])
        .rc;
    let mut denom: uint64_t = ((*(*h).sps.as_mut_ptr()).vui.hrd.i_bit_rate_unscaled
        as uint64_t)
        .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t)
        .wrapping_div((*rct).hrd_multiply_denom);
    let mut cpb_state: uint64_t = (*rct).buffer_fill_final as uint64_t;
    let mut cpb_size: uint64_t = ((*(*h).sps.as_mut_ptr()).vui.hrd.i_cpb_size_unscaled
        as uint64_t)
        .wrapping_mul((*(*h).sps.as_mut_ptr()).vui.i_time_scale as uint64_t);
    let mut multiply_factor: uint64_t = (90000 as uint64_t)
        .wrapping_div((*rct).hrd_multiply_denom);
    if (*rct).buffer_fill_final < 0 as int64_t
        || (*rct).buffer_fill_final > cpb_size as int64_t
    {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"CPB %s: %.0f bits in a %.0f-bit buffer\n\0" as *const u8
                as *const ::core::ffi::c_char,
            if (*rct).buffer_fill_final < 0 as int64_t {
                b"underflow\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"overflow\0" as *const u8 as *const ::core::ffi::c_char
            },
            (*rct).buffer_fill_final as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double,
            cpb_size as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double,
        );
    }
    (*h).initial_cpb_removal_delay = multiply_factor
        .wrapping_mul(cpb_state)
        .wrapping_div(denom) as ::core::ffi::c_int;
    (*h).initial_cpb_removal_delay_offset = multiply_factor
        .wrapping_mul(cpb_size)
        .wrapping_div(denom)
        .wrapping_sub((*h).initial_cpb_removal_delay as uint64_t) as ::core::ffi::c_int;
    let mut decoder_buffer_fill: int64_t = ((*h).initial_cpb_removal_delay as uint64_t)
        .wrapping_mul(denom)
        .wrapping_div(multiply_factor) as int64_t;
    (*rct).buffer_fill_final_min = if (*rct).buffer_fill_final_min < decoder_buffer_fill
    {
        (*rct).buffer_fill_final_min
    } else {
        decoder_buffer_fill
    };
}
#[c2rust::src_loc = "2217:1"]
unsafe extern "C" fn update_vbv_plan(
    mut h: *mut x264_t,
    mut overhead: ::core::ffi::c_int,
) {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    (*rcc).buffer_fill = ((*(*(*h).thread[0 as ::core::ffi::c_int as usize]).rc)
        .buffer_fill_final_min / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as int64_t)
        as ::core::ffi::c_double;
    if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = rcc
            .offset_from((*(*h).thread[0 as ::core::ffi::c_int as usize]).rc)
            as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < (*h).i_thread_frames {
            let mut t: *mut x264_t = (*h)
                .thread[((j + i) % (*h).i_thread_frames) as usize];
            let mut bits: ::core::ffi::c_double = (*(*t).rc).frame_size_planned;
            if !((*t).b_thread_active == 0) {
                bits = if bits > (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                {
                    bits
                } else {
                    (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                };
                (*rcc).buffer_fill -= bits;
                (*rcc).buffer_fill = if (*rcc).buffer_fill
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    (*rcc).buffer_fill
                } else {
                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                };
                (*rcc).buffer_fill += (*(*t).rc).buffer_rate;
                (*rcc).buffer_fill = if (*rcc).buffer_fill < (*rcc).buffer_size {
                    (*rcc).buffer_fill
                } else {
                    (*rcc).buffer_size
                };
            }
            i += 1;
        }
    }
    (*rcc).buffer_fill = if (*rcc).buffer_fill < (*rcc).buffer_size {
        (*rcc).buffer_fill
    } else {
        (*rcc).buffer_size
    };
    (*rcc).buffer_fill -= overhead as ::core::ffi::c_double;
}
#[c2rust::src_loc = "2242:1"]
unsafe extern "C" fn clip_qscale(
    mut h: *mut x264_t,
    mut pict_type: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut lmin: ::core::ffi::c_double = (*rcc).lmin[pict_type as usize];
    let mut lmax: ::core::ffi::c_double = (*rcc).lmax[pict_type as usize];
    if (*rcc).rate_factor_max_increment != 0. {
        lmax = if lmax
            < qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment)
                as ::core::ffi::c_double
        {
            lmax
        } else {
            qp2qscale((*rcc).qp_novbv + (*rcc).rate_factor_max_increment)
                as ::core::ffi::c_double
        };
    }
    if lmin == lmax {
        return lmin
    } else if (*rcc).b_2pass != 0 {
        let mut min2: ::core::ffi::c_double = log(lmin);
        let mut max2: ::core::ffi::c_double = log(lmax);
        q = (log(q) - min2) / (max2 - min2) - 0.5f64;
        q = 1.0f64
            / (1.0f64 + exp(-(4 as ::core::ffi::c_int) as ::core::ffi::c_double * q));
        q = q * (max2 - min2) + min2;
        return exp(q);
    } else {
        return x264_clip3f(q, lmin, lmax)
    };
}
#[c2rust::src_loc = "2266:1"]
unsafe extern "C" fn vbv_pass1(
    mut h: *mut x264_t,
    mut pict_type: ::core::ffi::c_int,
    mut q: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    if (*rcc).b_vbv != 0 && (*rcc).last_satd > 0 as ::core::ffi::c_int {
        let mut q0: ::core::ffi::c_double = q;
        let mut fenc_cpb_duration: ::core::ffi::c_double = (*(*h).fenc).i_cpb_duration
            as ::core::ffi::c_double
            * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick as ::core::ffi::c_double
            / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
        if (*h).param.rc.i_lookahead != 0 {
            let mut terminate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iterations: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while iterations < 1000 as ::core::ffi::c_int
                && terminate != 3 as ::core::ffi::c_int
            {
                let mut frame_q: [::core::ffi::c_double; 3] = [0.; 3];
                let mut cur_bits: ::core::ffi::c_double = predict_size(
                    &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                    q as ::core::ffi::c_float,
                    (*rcc).last_satd as ::core::ffi::c_float,
                ) as ::core::ffi::c_double;
                let mut buffer_fill_cur: ::core::ffi::c_double = (*rcc).buffer_fill
                    - cur_bits;
                let mut target_fill: ::core::ffi::c_double = 0.;
                let mut total_duration: ::core::ffi::c_double = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                let mut last_duration: ::core::ffi::c_double = fenc_cpb_duration;
                frame_q[0 as ::core::ffi::c_int as usize] = if (*h).sh.i_type
                    == SLICE_TYPE_I as ::core::ffi::c_int
                {
                    q * (*h).param.rc.f_ip_factor as ::core::ffi::c_double
                } else {
                    q
                };
                frame_q[1 as ::core::ffi::c_int as usize] = frame_q[0
                    as ::core::ffi::c_int as usize]
                    * (*h).param.rc.f_pb_factor as ::core::ffi::c_double;
                frame_q[2 as ::core::ffi::c_int as usize] = frame_q[0
                    as ::core::ffi::c_int as usize]
                    / (*h).param.rc.f_ip_factor as ::core::ffi::c_double;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while buffer_fill_cur >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && buffer_fill_cur <= (*rcc).buffer_size
                {
                    total_duration += last_duration;
                    buffer_fill_cur += (*rcc).vbv_max_rate * last_duration;
                    let mut i_type: ::core::ffi::c_int = (*(*h).fenc)
                        .i_planned_type[j as usize] as ::core::ffi::c_int;
                    let mut i_satd: ::core::ffi::c_int = (*(*h).fenc)
                        .i_planned_satd[j as usize];
                    if i_type == X264_TYPE_AUTO {
                        break;
                    }
                    i_type = if i_type == X264_TYPE_I || i_type == X264_TYPE_IDR
                        || i_type == X264_TYPE_KEYFRAME
                    {
                        SLICE_TYPE_I as ::core::ffi::c_int
                    } else if i_type == X264_TYPE_B || i_type == X264_TYPE_BREF {
                        SLICE_TYPE_B as ::core::ffi::c_int
                    } else {
                        SLICE_TYPE_P as ::core::ffi::c_int
                    };
                    cur_bits = predict_size(
                        &mut *(*rcc).pred.offset(i_type as isize),
                        frame_q[i_type as usize] as ::core::ffi::c_float,
                        i_satd as ::core::ffi::c_float,
                    ) as ::core::ffi::c_double;
                    buffer_fill_cur -= cur_bits;
                    last_duration = (*(*h).fenc).f_planned_cpb_duration[j as usize];
                    j += 1;
                }
                target_fill = if (*rcc).buffer_fill
                    + total_duration * (*rcc).vbv_max_rate * 0.5f64
                    < (*rcc).buffer_size * 0.5f64
                {
                    (*rcc).buffer_fill + total_duration * (*rcc).vbv_max_rate * 0.5f64
                } else {
                    (*rcc).buffer_size * 0.5f64
                };
                if buffer_fill_cur < target_fill {
                    q *= 1.01f64;
                    terminate |= 1 as ::core::ffi::c_int;
                } else {
                    target_fill = x264_clip3f(
                        (*rcc).buffer_fill
                            - total_duration * (*rcc).vbv_max_rate * 0.5f64,
                        (*rcc).buffer_size * 0.8f64,
                        (*rcc).buffer_size,
                    );
                    if !((*rcc).b_vbv_min_rate != 0 && buffer_fill_cur > target_fill) {
                        break;
                    }
                    q /= 1.01f64;
                    terminate |= 2 as ::core::ffi::c_int;
                }
                iterations += 1;
            }
        } else {
            if (pict_type == SLICE_TYPE_P as ::core::ffi::c_int
                || pict_type == SLICE_TYPE_I as ::core::ffi::c_int
                    && (*rcc).last_non_b_pict_type == SLICE_TYPE_I as ::core::ffi::c_int)
                && (*rcc).buffer_fill / (*rcc).buffer_size < 0.5f64
            {
                q
                    /= x264_clip3f(
                        2.0f64 * (*rcc).buffer_fill / (*rcc).buffer_size,
                        0.5f64,
                        1.0f64,
                    );
            }
            let mut bits: ::core::ffi::c_double = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q as ::core::ffi::c_float,
                (*rcc).last_satd as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
            let mut max_fill_factor: ::core::ffi::c_double = (if (*h)
                .param
                .rc
                .i_vbv_buffer_size as ::core::ffi::c_double
                >= (5 as ::core::ffi::c_int * (*h).param.rc.i_vbv_max_bitrate)
                    as ::core::ffi::c_double / (*rcc).fps
            {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as ::core::ffi::c_double;
            let mut min_fill_factor: ::core::ffi::c_double = (if (*rcc).single_frame_vbv
                != 0
            {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as ::core::ffi::c_double;
            if bits > (*rcc).buffer_fill / max_fill_factor {
                let mut qf: ::core::ffi::c_double = x264_clip3f(
                    (*rcc).buffer_fill / (max_fill_factor * bits),
                    0.2f64,
                    1.0f64,
                );
                q /= qf;
                bits *= qf;
            }
            if bits < (*rcc).buffer_rate / min_fill_factor {
                let mut qf_0: ::core::ffi::c_double = x264_clip3f(
                    bits * min_fill_factor / (*rcc).buffer_rate,
                    0.001f64,
                    1.0f64,
                );
                q *= qf_0;
            }
            q = if q0 > q { q0 } else { q };
        }
        if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int
            && (*rcc).single_frame_vbv == 0
        {
            let mut nb: ::core::ffi::c_int = (*rcc).bframes;
            let mut bits_0: ::core::ffi::c_double = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q as ::core::ffi::c_float,
                (*rcc).last_satd as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
            let mut pbbits: ::core::ffi::c_double = bits_0;
            let mut bbits: ::core::ffi::c_double = predict_size(
                (*rcc).pred_b_from_p,
                (q * (*h).param.rc.f_pb_factor as ::core::ffi::c_double)
                    as ::core::ffi::c_float,
                (*rcc).last_satd as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
            let mut space: ::core::ffi::c_double = 0.;
            let mut bframe_cpb_duration: ::core::ffi::c_double = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            let mut minigop_cpb_duration: ::core::ffi::c_double = 0.;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < nb {
                bframe_cpb_duration += (*(*h).fenc).f_planned_cpb_duration[i as usize];
                i += 1;
            }
            if bbits * nb as ::core::ffi::c_double
                > bframe_cpb_duration * (*rcc).vbv_max_rate
            {
                nb = 0 as ::core::ffi::c_int;
                bframe_cpb_duration = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            pbbits += nb as ::core::ffi::c_double * bbits;
            minigop_cpb_duration = bframe_cpb_duration + fenc_cpb_duration;
            space = (*rcc).buffer_fill + minigop_cpb_duration * (*rcc).vbv_max_rate
                - (*rcc).buffer_size;
            if pbbits < space {
                q
                    *= if pbbits / space > bits_0 / (0.5f64 * (*rcc).buffer_size) {
                        pbbits / space
                    } else {
                        bits_0 / (0.5f64 * (*rcc).buffer_size)
                    };
            }
            q = if q0 / 2 as ::core::ffi::c_int as ::core::ffi::c_double > q {
                q0 / 2 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                q
            };
        }
        if (*rcc).b_vbv_min_rate == 0 {
            q = if q0 > q { q0 } else { q };
        }
    }
    return clip_qscale(h, pict_type, q);
}
#[c2rust::src_loc = "2400:1"]
unsafe extern "C" fn rate_estimate_qscale(mut h: *mut x264_t) -> ::core::ffi::c_float {
    let mut q: ::core::ffi::c_float = 0.;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut rce: ratecontrol_entry_t = {
        let mut init = ratecontrol_entry_t {
            pict_type: 0 as ::core::ffi::c_int,
            frame_type: 0,
            kept_as_ref: 0,
            qscale: 0.,
            mv_bits: 0,
            tex_bits: 0,
            misc_bits: 0,
            expected_bits: 0.,
            expected_vbv: 0.,
            new_qscale: 0.,
            new_qp: 0.,
            i_count: 0,
            p_count: 0,
            s_count: 0,
            blurred_complexity: 0.,
            direct_mode: 0,
            weight: [[0; 2]; 3],
            i_weight_denom: [0; 2],
            refcount: [0; 16],
            refs: 0,
            i_duration: 0,
            i_cpb_duration: 0,
            out_num: 0,
        };
        init
    };
    let mut pict_type: ::core::ffi::c_int = (*h).sh.i_type;
    let mut total_bits: int64_t = 8 as int64_t
        * ((*h).stat.i_frame_size[SLICE_TYPE_I as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_P as ::core::ffi::c_int as usize]
            + (*h).stat.i_frame_size[SLICE_TYPE_B as ::core::ffi::c_int as usize])
        - (*rcc).filler_bits_sum;
    if (*rcc).b_2pass != 0 {
        rce = *(*rcc).rce;
        if pict_type != rce.pict_type {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"slice=%c but 2pass stats say %c\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                slice_type_to_char[pict_type as usize] as ::core::ffi::c_int,
                slice_type_to_char[rce.pict_type as usize] as ::core::ffi::c_int,
            );
        }
    }
    if pict_type == SLICE_TYPE_B as ::core::ffi::c_int {
        let mut i0: ::core::ffi::c_int = ((*(*h)
            .fref_nearest[0 as ::core::ffi::c_int as usize])
            .i_type == X264_TYPE_I
            || (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                == X264_TYPE_IDR
            || (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
                == X264_TYPE_KEYFRAME) as ::core::ffi::c_int;
        let mut i1: ::core::ffi::c_int = ((*(*h)
            .fref_nearest[1 as ::core::ffi::c_int as usize])
            .i_type == X264_TYPE_I
            || (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                == X264_TYPE_IDR
            || (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
                == X264_TYPE_KEYFRAME) as ::core::ffi::c_int;
        let mut dt0: ::core::ffi::c_int = abs(
            (*(*h).fenc).i_poc
                - (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_poc,
        );
        let mut dt1: ::core::ffi::c_int = abs(
            (*(*h).fenc).i_poc
                - (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_poc,
        );
        let mut q0: ::core::ffi::c_float = (*(*h)
            .fref_nearest[0 as ::core::ffi::c_int as usize])
            .f_qp_avg_rc;
        let mut q1: ::core::ffi::c_float = (*(*h)
            .fref_nearest[1 as ::core::ffi::c_int as usize])
            .f_qp_avg_rc;
        if (*(*h).fref_nearest[0 as ::core::ffi::c_int as usize]).i_type
            == X264_TYPE_BREF
        {
            q0 = (q0 as ::core::ffi::c_double
                - (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_float;
        }
        if (*(*h).fref_nearest[1 as ::core::ffi::c_int as usize]).i_type
            == X264_TYPE_BREF
        {
            q1 = (q1 as ::core::ffi::c_double
                - (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_float;
        }
        if i0 != 0 && i1 != 0 {
            q = (((q0 + q1) / 2 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double + (*rcc).ip_offset) as ::core::ffi::c_float;
        } else if i0 != 0 {
            q = q1;
        } else if i1 != 0 {
            q = q0;
        } else {
            q = (q0 * dt1 as ::core::ffi::c_float + q1 * dt0 as ::core::ffi::c_float)
                / (dt0 + dt1) as ::core::ffi::c_float;
        }
        if (*(*h).fenc).b_kept_as_ref != 0 {
            q = (q as ::core::ffi::c_double
                + (*rcc).pb_offset / 2 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_float;
        } else {
            q = (q as ::core::ffi::c_double + (*rcc).pb_offset) as ::core::ffi::c_float;
        }
        (*rcc).qp_novbv = q;
        q = qp2qscale(q);
        if (*rcc).b_2pass != 0 {
            (*rcc).frame_size_planned = qscale2bits(
                &mut rce,
                q as ::core::ffi::c_double,
            );
        } else {
            (*rcc).frame_size_planned = predict_size(
                (*rcc).pred_b_from_p,
                q,
                (*(*h)
                    .fref[1 as ::core::ffi::c_int
                    as usize][((*h).i_ref[1 as ::core::ffi::c_int as usize]
                    - 1 as ::core::ffi::c_int) as usize])
                    .i_satd as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
        }
        if (*rcc).b_vbv != 0 {
            let mut frame_size_maximum: ::core::ffi::c_double = if (*rcc)
                .frame_size_maximum
                < (if (*rcc).buffer_fill > 0.001f64 {
                    (*rcc).buffer_fill
                } else {
                    0.001f64
                })
            {
                (*rcc).frame_size_maximum
            } else if (*rcc).buffer_fill > 0.001f64 {
                (*rcc).buffer_fill
            } else {
                0.001f64
            };
            if (*rcc).frame_size_planned > frame_size_maximum {
                q = (q as ::core::ffi::c_double
                    * ((*rcc).frame_size_planned / frame_size_maximum))
                    as ::core::ffi::c_float;
                (*rcc).frame_size_planned = frame_size_maximum;
            }
        }
        ::core::ptr::write_volatile(
            &mut (*rcc).frame_size_estimated as *mut ::core::ffi::c_float,
            (*rcc).frame_size_planned as ::core::ffi::c_float,
        );
        if (*rcc).b_vbv != 0 {
            (*rcc).last_satd = x264_10_rc_analyse_slice(h);
        }
        return q;
    } else {
        let mut abr_buffer: ::core::ffi::c_double = 2 as ::core::ffi::c_int
            as ::core::ffi::c_double * (*rcc).rate_tolerance * (*rcc).bitrate;
        let mut predicted_bits: ::core::ffi::c_double = total_bits
            as ::core::ffi::c_double;
        if (*h).i_thread_frames > 1 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = rcc
                .offset_from((*(*h).thread[0 as ::core::ffi::c_int as usize]).rc)
                as ::core::ffi::c_long as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i < (*h).i_thread_frames {
                let mut t: *mut x264_t = (*h)
                    .thread[((j + i) % (*h).i_thread_frames) as usize];
                let mut bits: ::core::ffi::c_double = (*(*t).rc).frame_size_planned;
                if !((*t).b_thread_active == 0) {
                    bits = if bits
                        > (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                    {
                        bits
                    } else {
                        (*(*t).rc).frame_size_estimated as ::core::ffi::c_double
                    };
                    predicted_bits += bits;
                }
                i += 1;
            }
        }
        if (*rcc).b_2pass != 0 {
            let mut lmin: ::core::ffi::c_double = (*rcc).lmin[pict_type as usize];
            let mut lmax: ::core::ffi::c_double = (*rcc).lmax[pict_type as usize];
            let mut diff: ::core::ffi::c_double = 0.;
            if (*rcc).num_entries > (*h).i_frame {
                let mut final_bits: ::core::ffi::c_double = (**(*rcc)
                    .entry_out
                    .offset(((*rcc).num_entries - 1 as ::core::ffi::c_int) as isize))
                    .expected_bits;
                let mut video_pos: ::core::ffi::c_double = rce.expected_bits
                    / final_bits;
                let mut scale_factor: ::core::ffi::c_double = sqrt(
                    (1 as ::core::ffi::c_int as ::core::ffi::c_double - video_pos)
                        * (*rcc).num_entries as ::core::ffi::c_double,
                );
                abr_buffer
                    *= 0.5f64
                        * (if scale_factor > 0.5f64 { scale_factor } else { 0.5f64 });
            }
            diff = predicted_bits - rce.expected_bits;
            q = rce.new_qscale as ::core::ffi::c_float;
            q = (q as ::core::ffi::c_double
                / x264_clip3f(
                    (abr_buffer - diff) / abr_buffer,
                    0.5f64,
                    2 as ::core::ffi::c_int as ::core::ffi::c_double,
                )) as ::core::ffi::c_float;
            if (*h).i_frame as ::core::ffi::c_double >= (*rcc).fps
                && (*rcc).expected_bits_sum
                    >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                let mut cur_time: ::core::ffi::c_double = (*h).i_frame
                    as ::core::ffi::c_double
                    / (*rcc).num_entries as ::core::ffi::c_double;
                let mut w: ::core::ffi::c_double = x264_clip3f(
                    cur_time * 100 as ::core::ffi::c_int as ::core::ffi::c_double,
                    0.0f64,
                    1.0f64,
                );
                q = (q as ::core::ffi::c_double
                    * pow(
                        total_bits as ::core::ffi::c_double / (*rcc).expected_bits_sum,
                        w,
                    )) as ::core::ffi::c_float;
            }
            (*rcc).qp_novbv = qscale2qp(q);
            if (*rcc).b_vbv != 0 {
                let mut expected_size: ::core::ffi::c_double = qscale2bits(
                    &mut rce,
                    q as ::core::ffi::c_double,
                );
                let mut expected_vbv: ::core::ffi::c_double = (*rcc).buffer_fill
                    + (*rcc).buffer_rate - expected_size;
                let mut expected_fullness: ::core::ffi::c_double = rce.expected_vbv
                    / (*rcc).buffer_size;
                let mut qmax: ::core::ffi::c_double = q as ::core::ffi::c_double
                    * (2 as ::core::ffi::c_int as ::core::ffi::c_double
                        - expected_fullness);
                let mut size_constraint: ::core::ffi::c_double = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double + expected_fullness;
                qmax = if qmax > rce.new_qscale { qmax } else { rce.new_qscale };
                if expected_fullness < 0.05f64 {
                    qmax = lmax;
                }
                qmax = if qmax < lmax { qmax } else { lmax };
                while expected_vbv < rce.expected_vbv / size_constraint
                    && (q as ::core::ffi::c_double) < qmax
                    || expected_vbv < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && (q as ::core::ffi::c_double) < lmax
                {
                    q = (q as ::core::ffi::c_double * 1.05f64) as ::core::ffi::c_float;
                    expected_size = qscale2bits(&mut rce, q as ::core::ffi::c_double);
                    expected_vbv = (*rcc).buffer_fill + (*rcc).buffer_rate
                        - expected_size;
                }
                (*rcc).last_satd = x264_10_rc_analyse_slice(h);
            }
            q = x264_clip3f(q as ::core::ffi::c_double, lmin, lmax)
                as ::core::ffi::c_float;
        } else {
            let mut wanted_bits: ::core::ffi::c_double = 0.;
            let mut overflow: ::core::ffi::c_double = 1 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*rcc).last_satd = x264_10_rc_analyse_slice(h);
            (*rcc).short_term_cplxsum *= 0.5f64;
            (*rcc).short_term_cplxcount *= 0.5f64;
            (*rcc).short_term_cplxsum
                += (*rcc).last_satd as ::core::ffi::c_double
                    / (x264_clip3f(
                        (*(*h).fenc).f_duration as ::core::ffi::c_double,
                        (0.01f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float) as ::core::ffi::c_double,
                        (1.00f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float) as ::core::ffi::c_double,
                    )
                        / (0.04f32
                            / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                                as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_float) as ::core::ffi::c_double);
            (*rcc).short_term_cplxcount += 1.;
            rce.tex_bits = (*rcc).last_satd;
            rce.blurred_complexity = ((*rcc).short_term_cplxsum
                / (*rcc).short_term_cplxcount) as ::core::ffi::c_float;
            rce.mv_bits = 0 as ::core::ffi::c_int;
            rce.p_count = (*rcc).nmb;
            rce.i_count = 0 as ::core::ffi::c_int;
            rce.s_count = 0 as ::core::ffi::c_int;
            rce.qscale = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            rce.pict_type = pict_type;
            rce.i_duration = (*(*h).fenc).i_duration;
            if (*h).param.rc.i_rc_method == X264_RC_CRF {
                q = get_qscale(
                    h,
                    &mut rce,
                    (*rcc).rate_factor_constant,
                    (*(*h).fenc).i_frame,
                ) as ::core::ffi::c_float;
            } else {
                q = get_qscale(
                    h,
                    &mut rce,
                    (*rcc).wanted_bits_window / (*rcc).cplxr_sum,
                    (*(*h).fenc).i_frame,
                ) as ::core::ffi::c_float;
                if (*rcc).b_vbv_min_rate == 0 && (*rcc).last_satd != 0 {
                    let mut i_frame_done: ::core::ffi::c_int = (*h).i_frame;
                    let mut time_done: ::core::ffi::c_double = i_frame_done
                        as ::core::ffi::c_double / (*rcc).fps;
                    if (*h).param.b_vfr_input != 0
                        && i_frame_done > 0 as ::core::ffi::c_int
                    {
                        time_done = ((*(*h).fenc).i_reordered_pts
                            - (*h).i_reordered_pts_delay) as ::core::ffi::c_double
                            * (*h).param.i_timebase_num as ::core::ffi::c_double
                            / (*h).param.i_timebase_den as ::core::ffi::c_double;
                    }
                    wanted_bits = time_done * (*rcc).bitrate;
                    if wanted_bits > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                        abr_buffer
                            *= if 1 as ::core::ffi::c_int as ::core::ffi::c_double
                                > sqrt(time_done)
                            {
                                1 as ::core::ffi::c_int as ::core::ffi::c_double
                            } else {
                                sqrt(time_done)
                            };
                        overflow = x264_clip3f(
                            1.0f64 + (predicted_bits - wanted_bits) / abr_buffer,
                            0.5f64,
                            2 as ::core::ffi::c_int as ::core::ffi::c_double,
                        );
                        q = (q as ::core::ffi::c_double * overflow)
                            as ::core::ffi::c_float;
                    }
                }
            }
            if pict_type == SLICE_TYPE_I as ::core::ffi::c_int
                && (*h).param.i_keyint_max > 1 as ::core::ffi::c_int
                && (*rcc).last_non_b_pict_type != SLICE_TYPE_I as ::core::ffi::c_int
            {
                q = qp2qscale(
                    ((*rcc).accum_p_qp / (*rcc).accum_p_norm) as ::core::ffi::c_float,
                );
                q /= (*h).param.rc.f_ip_factor;
            } else if (*h).i_frame > 0 as ::core::ffi::c_int {
                if (*h).param.rc.i_rc_method != X264_RC_CRF {
                    let mut lmin_0: ::core::ffi::c_double = (*rcc)
                        .last_qscale_for[pict_type as usize] / (*rcc).lstep;
                    let mut lmax_0: ::core::ffi::c_double = (*rcc)
                        .last_qscale_for[pict_type as usize] * (*rcc).lstep;
                    if overflow > 1.1f64 && (*h).i_frame > 3 as ::core::ffi::c_int {
                        lmax_0 *= (*rcc).lstep;
                    } else if overflow < 0.9f64 {
                        lmin_0 /= (*rcc).lstep;
                    }
                    q = x264_clip3f(q as ::core::ffi::c_double, lmin_0, lmax_0)
                        as ::core::ffi::c_float;
                }
            } else if (*h).param.rc.i_rc_method == X264_RC_CRF
                && (*rcc).qcompress != 1 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                q = qp2qscale(
                    (if (*h).param.rc.i_rc_method == X264_RC_CRF {
                        (*h).param.rc.f_rf_constant
                    } else {
                        24 as ::core::ffi::c_int as ::core::ffi::c_float
                    }) + QP_BD_OFFSET as ::core::ffi::c_float,
                ) / (*h).param.rc.f_ip_factor;
            }
            (*rcc).qp_novbv = qscale2qp(q);
            q = vbv_pass1(h, pict_type, q as ::core::ffi::c_double)
                as ::core::ffi::c_float;
        }
        (*rcc).last_qscale = q as ::core::ffi::c_double;
        (*rcc).last_qscale_for[pict_type as usize] = (*rcc).last_qscale;
        if !((*rcc).b_2pass != 0 && (*rcc).b_vbv == 0)
            && (*(*h).fenc).i_frame == 0 as ::core::ffi::c_int
        {
            (*rcc).last_qscale_for[SLICE_TYPE_P as ::core::ffi::c_int as usize] = (q
                * (*h).param.rc.f_ip_factor) as ::core::ffi::c_double;
        }
        if (*rcc).b_2pass != 0 {
            (*rcc).frame_size_planned = qscale2bits(
                &mut rce,
                q as ::core::ffi::c_double,
            );
        } else {
            (*rcc).frame_size_planned = predict_size(
                &mut *(*rcc).pred.offset((*h).sh.i_type as isize),
                q,
                (*rcc).last_satd as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
        }
        if (*rcc).b_vbv != 0 {
            let mut frame_size_maximum_0: ::core::ffi::c_double = if (*rcc)
                .frame_size_maximum
                < (if (*rcc).buffer_fill > 0.001f64 {
                    (*rcc).buffer_fill
                } else {
                    0.001f64
                })
            {
                (*rcc).frame_size_maximum
            } else if (*rcc).buffer_fill > 0.001f64 {
                (*rcc).buffer_fill
            } else {
                0.001f64
            };
            if (*rcc).frame_size_planned > frame_size_maximum_0 {
                q = (q as ::core::ffi::c_double
                    * ((*rcc).frame_size_planned / frame_size_maximum_0))
                    as ::core::ffi::c_float;
                (*rcc).frame_size_planned = frame_size_maximum_0;
            }
            if (*rcc).single_frame_vbv != 0 {
                (*rcc).frame_size_planned = if (*rcc).buffer_rate < frame_size_maximum_0
                {
                    (*rcc).buffer_rate
                } else {
                    frame_size_maximum_0
                };
            }
        }
        ::core::ptr::write_volatile(
            &mut (*rcc).frame_size_estimated as *mut ::core::ffi::c_float,
            (*rcc).frame_size_planned as ::core::ffi::c_float,
        );
        return q;
    };
}
#[c2rust::src_loc = "2665:1"]
unsafe extern "C" fn threads_normalize_predictors(mut h: *mut x264_t) {
    let mut totalsize: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).param.i_threads {
        totalsize += (*(*(*h).thread[i as usize]).rc).slice_size_planned;
        i += 1;
    }
    let mut factor: ::core::ffi::c_double = (*(*h).rc).frame_size_planned / totalsize;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*h).param.i_threads {
        (*(*(*h).thread[i_0 as usize]).rc).slice_size_planned *= factor;
        i_0 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "2675:1"]
pub unsafe extern "C" fn x264_10_threads_distribute_ratecontrol(mut h: *mut x264_t) {
    let mut row: ::core::ffi::c_int = 0;
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut qscale: ::core::ffi::c_float = qp2qscale((*rc).qpm);
    if (*h).i_frame == 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).param.i_threads {
            let mut t: *mut x264_t = (*h).thread[i as usize];
            if t != h {
                memcpy(
                    (*(*t).rc).row_preds.as_mut_ptr() as *mut ::core::ffi::c_void,
                    (*rc).row_preds.as_mut_ptr() as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[[predictor_t; 2]; 3]>() as size_t,
                );
            }
            i += 1;
        }
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*h).param.i_threads {
        let mut t_0: *mut x264_t = (*h).thread[i_0 as usize];
        if t_0 != h {
            memcpy(
                (*t_0).rc as *mut ::core::ffi::c_void,
                rc as *const ::core::ffi::c_void,
                576 as size_t,
            );
        }
        (*(*t_0).rc).row_pred = (*(*(*t_0).rc)
            .row_preds
            .as_mut_ptr()
            .offset((*h).sh.i_type as isize))
            .as_mut_ptr();
        if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
            let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            row = (*t_0).i_threadslice_start;
            while row < (*t_0).i_threadslice_end {
                size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                row += 1;
            }
            (*(*t_0).rc).slice_size_planned = predict_size(
                &mut *(*rc)
                    .pred
                    .offset(
                        ((*h).sh.i_type
                            + (i_0 + 1 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                            as isize,
                    ),
                qscale,
                size as ::core::ffi::c_float,
            ) as ::core::ffi::c_double;
        } else {
            (*(*t_0).rc).slice_size_planned = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
        }
        i_0 += 1;
    }
    if (*rc).b_vbv != 0 && (*rc).frame_size_planned != 0. {
        threads_normalize_predictors(h);
        if (*rc).single_frame_vbv != 0 {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < (*h).param.i_threads {
                let mut t_1: *mut x264_t = (*h).thread[i_1 as usize];
                let mut max_frame_error: ::core::ffi::c_float = x264_clip3f(
                    1.0f64
                        / ((*t_1).i_threadslice_end - (*t_1).i_threadslice_start)
                            as ::core::ffi::c_double,
                    0.05f64,
                    0.25f64,
                ) as ::core::ffi::c_float;
                (*(*t_1).rc).slice_size_planned
                    += (2 as ::core::ffi::c_int as ::core::ffi::c_float
                        * max_frame_error) as ::core::ffi::c_double
                        * (*rc).frame_size_planned;
                i_1 += 1;
            }
            threads_normalize_predictors(h);
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < (*h).param.i_threads {
            ::core::ptr::write_volatile(
                &mut (*(*(*h).thread[i_2 as usize]).rc).frame_size_estimated
                    as *mut ::core::ffi::c_float,
                (*(*(*h).thread[i_2 as usize]).rc).slice_size_planned
                    as ::core::ffi::c_float,
            );
            i_2 += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "2729:1"]
pub unsafe extern "C" fn x264_10_threads_merge_ratecontrol(mut h: *mut x264_t) {
    let mut rc: *mut x264_ratecontrol_t = (*h).rc;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).param.i_threads {
        let mut t: *mut x264_t = (*h).thread[i as usize];
        let mut rct: *mut x264_ratecontrol_t = (*(*h).thread[i as usize]).rc;
        if (*h).param.rc.i_vbv_buffer_size != 0 {
            let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut row: ::core::ffi::c_int = (*t).i_threadslice_start;
            while row < (*t).i_threadslice_end {
                size += *(*(*h).fdec).i_row_satd.offset(row as isize);
                row += 1;
            }
            let mut bits: ::core::ffi::c_int = (*t).stat.frame.i_mv_bits
                + (*t).stat.frame.i_tex_bits + (*t).stat.frame.i_misc_bits;
            let mut mb_count: ::core::ffi::c_int = ((*t).i_threadslice_end
                - (*t).i_threadslice_start) * (*h).mb.i_mb_width;
            update_predictor(
                &mut *(*rc)
                    .pred
                    .offset(
                        ((*h).sh.i_type
                            + (i + 1 as ::core::ffi::c_int) * 5 as ::core::ffi::c_int)
                            as isize,
                    ),
                qp2qscale((*rct).qpa_rc / mb_count as ::core::ffi::c_float),
                size as ::core::ffi::c_float,
                bits as ::core::ffi::c_float,
            );
        }
        if !(i == 0) {
            (*rc).qpa_rc += (*rct).qpa_rc;
            (*rc).qpa_aq += (*rct).qpa_aq;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "2754:1"]
pub unsafe extern "C" fn x264_10_thread_sync_ratecontrol(
    mut cur: *mut x264_t,
    mut prev: *mut x264_t,
    mut next: *mut x264_t,
) {
    if cur != prev {
        memcpy(
            &mut (*(*cur).rc).accum_p_qp as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).accum_p_qp as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).accum_p_norm as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).accum_p_norm as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_satd as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).last_satd as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_rceq as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).last_rceq as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_qscale_for as *mut [::core::ffi::c_double; 3]
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).last_qscale_for as *mut [::core::ffi::c_double; 3]
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_double; 3]>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).last_non_b_pict_type as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).last_non_b_pict_type as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).short_term_cplxsum as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).short_term_cplxsum as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).short_term_cplxcount as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).short_term_cplxcount as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).bframes as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).bframes as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).prev_zone as *mut *mut x264_zone_t
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).prev_zone as *mut *mut x264_zone_t
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut x264_zone_t>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).mbtree.qpbuf_pos as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).mbtree.qpbuf_pos as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).bitrate as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).bitrate as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).buffer_size as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).buffer_size as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).buffer_rate as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).buffer_rate as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).vbv_max_rate as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).vbv_max_rate as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).single_frame_vbv as *mut ::core::ffi::c_int
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).single_frame_vbv as *mut ::core::ffi::c_int
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).cbr_decay as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).cbr_decay as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).rate_factor_constant as *mut ::core::ffi::c_double
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).rate_factor_constant as *mut ::core::ffi::c_double
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        );
        memcpy(
            &mut (*(*cur).rc).rate_factor_max_increment as *mut ::core::ffi::c_float
                as *mut ::core::ffi::c_void,
            &mut (*(*prev).rc).rate_factor_max_increment as *mut ::core::ffi::c_float
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        );
    }
    if cur != next {
        (*(*next).rc).cplxr_sum = (*(*cur).rc).cplxr_sum;
        (*(*next).rc).expected_bits_sum = (*(*cur).rc).expected_bits_sum;
        (*(*next).rc).filler_bits_sum = (*(*cur).rc).filler_bits_sum;
        (*(*next).rc).wanted_bits_window = (*(*cur).rc).wanted_bits_window;
        (*(*next).rc).bframe_bits = (*(*cur).rc).bframe_bits;
        (*(*next).rc).initial_cpb_removal_delay = (*(*cur).rc).initial_cpb_removal_delay;
        (*(*next).rc).initial_cpb_removal_delay_offset = (*(*cur).rc)
            .initial_cpb_removal_delay_offset;
        (*(*next).rc).nrt_first_access_unit = (*(*cur).rc).nrt_first_access_unit;
        (*(*next).rc).previous_cpb_final_arrival_time = (*(*cur).rc)
            .previous_cpb_final_arrival_time;
    }
}
#[c2rust::src_loc = "2805:1"]
unsafe extern "C" fn find_underflow(
    mut h: *mut x264_t,
    mut fills: *mut ::core::ffi::c_double,
    mut t0: *mut ::core::ffi::c_int,
    mut t1: *mut ::core::ffi::c_int,
    mut over: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let buffer_min: ::core::ffi::c_double = 0.1f64 * (*rcc).buffer_size;
    let buffer_max: ::core::ffi::c_double = 0.9f64 * (*rcc).buffer_size;
    let mut fill: ::core::ffi::c_double = *fills
        .offset((*t0 - 1 as ::core::ffi::c_int) as isize);
    let mut parity: ::core::ffi::c_double = if over != 0 { 1.0f64 } else { -1.0f64 };
    let mut start: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut end: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = *t0;
    while i < (*rcc).num_entries {
        fill
            += ((**(*rcc).entry_out.offset(i as isize)).i_cpb_duration
                as ::core::ffi::c_double * (*rcc).vbv_max_rate
                * (*(*h).sps.as_mut_ptr()).vui.i_num_units_in_tick
                    as ::core::ffi::c_double
                / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double
                - qscale2bits(
                    *(*rcc).entry_out.offset(i as isize),
                    (**(*rcc).entry_out.offset(i as isize)).new_qscale,
                )) * parity;
        fill = x264_clip3f(
            fill,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            (*rcc).buffer_size,
        );
        *fills.offset(i as isize) = fill;
        if fill <= buffer_min || i == 0 as ::core::ffi::c_int {
            if end >= 0 as ::core::ffi::c_int {
                break;
            }
            start = i;
        } else if fill >= buffer_max && start >= 0 as ::core::ffi::c_int {
            end = i;
        }
        i += 1;
    }
    *t0 = start;
    *t1 = end;
    return (start >= 0 as ::core::ffi::c_int && end >= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[c2rust::src_loc = "2836:1"]
unsafe extern "C" fn fix_underflow(
    mut h: *mut x264_t,
    mut t0: ::core::ffi::c_int,
    mut t1: ::core::ffi::c_int,
    mut adjustment: ::core::ffi::c_double,
    mut qscale_min: ::core::ffi::c_double,
    mut qscale_max: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut qscale_orig: ::core::ffi::c_double = 0.;
    let mut qscale_new: ::core::ffi::c_double = 0.;
    let mut adjusted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if t0 > 0 as ::core::ffi::c_int {
        t0 += 1;
    }
    let mut i: ::core::ffi::c_int = t0;
    while i <= t1 {
        qscale_orig = (**(*rcc).entry_out.offset(i as isize)).new_qscale;
        qscale_orig = x264_clip3f(qscale_orig, qscale_min, qscale_max);
        qscale_new = qscale_orig * adjustment;
        qscale_new = x264_clip3f(qscale_new, qscale_min, qscale_max);
        (**(*rcc).entry_out.offset(i as isize)).new_qscale = qscale_new;
        adjusted = (adjusted != 0 || qscale_new != qscale_orig) as ::core::ffi::c_int;
        i += 1;
    }
    return adjusted;
}
#[c2rust::src_loc = "2855:1"]
unsafe extern "C" fn count_expected_bits(mut h: *mut x264_t) -> ::core::ffi::c_double {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut expected_bits: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*rcc).num_entries {
        let mut rce: *mut ratecontrol_entry_t = *(*rcc).entry_out.offset(i as isize);
        (*rce).expected_bits = expected_bits;
        expected_bits += qscale2bits(rce, (*rce).new_qscale);
        i += 1;
    }
    return expected_bits;
}
#[c2rust::src_loc = "2868:1"]
unsafe extern "C" fn vbv_pass2(
    mut h: *mut x264_t,
    mut all_available_bits: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut fills: *mut ::core::ffi::c_double = 0 as *mut ::core::ffi::c_double;
    let mut expected_bits: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut adjustment: ::core::ffi::c_double = 0.;
    let mut prev_bits: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut t0: ::core::ffi::c_int = 0;
    let mut t1: ::core::ffi::c_int = 0;
    let mut qscale_min: ::core::ffi::c_double = qp2qscale(
        (*h).param.rc.i_qp_min as ::core::ffi::c_float,
    ) as ::core::ffi::c_double;
    let mut qscale_max: ::core::ffi::c_double = qp2qscale(
        (*h).param.rc.i_qp_max as ::core::ffi::c_float,
    ) as ::core::ffi::c_double;
    let mut iterations: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut adj_min: ::core::ffi::c_int = 0;
    let mut adj_max: ::core::ffi::c_int = 0;
    fills = x264_malloc(
        (((*rcc).num_entries + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
            as int64_t,
    ) as *mut ::core::ffi::c_double;
    if fills.is_null() {
        return -(1 as ::core::ffi::c_int)
    } else {
        fills = fills.offset(1);
        loop {
            iterations += 1;
            prev_bits = expected_bits;
            if expected_bits != 0. {
                adjustment = if (if expected_bits / all_available_bits < 0.999f64 {
                    expected_bits / all_available_bits
                } else {
                    0.999f64
                }) > 0.9f64
                {
                    if expected_bits / all_available_bits < 0.999f64 {
                        expected_bits / all_available_bits
                    } else {
                        0.999f64
                    }
                } else {
                    0.9f64
                };
                *fills.offset(-(1 as ::core::ffi::c_int) as isize) = (*rcc).buffer_size
                    * (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double;
                t0 = 0 as ::core::ffi::c_int;
                adj_min = 1 as ::core::ffi::c_int;
                while adj_min != 0
                    && find_underflow(
                        h,
                        fills,
                        &mut t0,
                        &mut t1,
                        1 as ::core::ffi::c_int,
                    ) != 0
                {
                    adj_min = fix_underflow(
                        h,
                        t0,
                        t1,
                        adjustment,
                        qscale_min,
                        qscale_max,
                    );
                    t0 = t1;
                }
            }
            *fills.offset(-(1 as ::core::ffi::c_int) as isize) = (*rcc).buffer_size
                * (1.0f64 - (*h).param.rc.f_vbv_buffer_init as ::core::ffi::c_double);
            t0 = 0 as ::core::ffi::c_int;
            adj_max = 1 as ::core::ffi::c_int;
            while adj_max != 0
                && find_underflow(h, fills, &mut t0, &mut t1, 0 as ::core::ffi::c_int)
                    != 0
            {
                adj_max = fix_underflow(h, t0, t1, 1.001f64, qscale_min, qscale_max);
            }
            expected_bits = count_expected_bits(h);
            if !(expected_bits < 0.995f64 * all_available_bits
                && (expected_bits + 0.5f64) as int64_t > (prev_bits + 0.5f64) as int64_t)
            {
                break;
            }
        }
        if adj_max == 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"vbv-maxrate issue, qpmax or vbv-maxrate too low\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*rcc).num_entries {
            (**(*rcc).entry_out.offset(i as isize)).expected_vbv = (*rcc).buffer_size
                - *fills.offset(i as isize);
            i += 1;
        }
        x264_free(
            fills.offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
        );
        return 0 as ::core::ffi::c_int;
    };
}
#[c2rust::src_loc = "2932:1"]
unsafe extern "C" fn init_pass2(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rcc: *mut x264_ratecontrol_t = (*h).rc;
    let mut all_const_bits: uint64_t = 0 as uint64_t;
    let mut timescale: ::core::ffi::c_double = (*(*h).sps.as_mut_ptr())
        .vui
        .i_num_units_in_tick as ::core::ffi::c_double
        / (*(*h).sps.as_mut_ptr()).vui.i_time_scale as ::core::ffi::c_double;
    let mut duration: ::core::ffi::c_double = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*rcc).num_entries {
        duration
            += (*(*rcc).entry.offset(i as isize)).i_duration as ::core::ffi::c_double;
        i += 1;
    }
    duration *= timescale;
    let mut all_available_bits: uint64_t = ((*h).param.rc.i_bitrate
        as ::core::ffi::c_double * 1000.0f64 * duration) as uint64_t;
    let mut rate_factor: ::core::ffi::c_double = 0.;
    let mut step_mult: ::core::ffi::c_double = 0.;
    let mut qblur: ::core::ffi::c_double = (*h).param.rc.f_qblur
        as ::core::ffi::c_double;
    let mut cplxblur: ::core::ffi::c_double = (*h).param.rc.f_complexity_blur
        as ::core::ffi::c_double;
    let filter_size: ::core::ffi::c_int = (qblur
        * 4 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int
        | 1 as ::core::ffi::c_int;
    let mut expected_bits: ::core::ffi::c_double = 0.;
    let mut qscale: *mut ::core::ffi::c_double = 0 as *mut ::core::ffi::c_double;
    let mut blurred_qscale: *mut ::core::ffi::c_double = 0 as *mut ::core::ffi::c_double;
    let mut base_cplx: ::core::ffi::c_double = ((*h).mb.i_mb_count
        * (if (*h).param.i_bframe != 0 {
            120 as ::core::ffi::c_int
        } else {
            80 as ::core::ffi::c_int
        })) as ::core::ffi::c_double;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*rcc).num_entries {
        let mut rce: *mut ratecontrol_entry_t = &mut *(*rcc).entry.offset(i_0 as isize)
            as *mut ratecontrol_entry_t;
        all_const_bits = all_const_bits.wrapping_add((*rce).misc_bits as uint64_t);
        i_0 += 1;
    }
    if all_available_bits < all_const_bits {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"requested bitrate is too low. estimated minimum is %d kbps\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (all_const_bits as ::core::ffi::c_double * (*rcc).fps
                / ((*rcc).num_entries as ::core::ffi::c_double * 1000.0f64))
                as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < (*rcc).num_entries {
        let mut rce_0: *mut ratecontrol_entry_t = &mut *(*rcc).entry.offset(i_1 as isize)
            as *mut ratecontrol_entry_t;
        let mut weight_sum: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        let mut cplx_sum: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        let mut weight: ::core::ffi::c_double = 1.0f64;
        let mut gaussian_weight: ::core::ffi::c_double = 0.;
        let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while (j as ::core::ffi::c_double)
            < cplxblur * 2 as ::core::ffi::c_int as ::core::ffi::c_double
            && j < (*rcc).num_entries - i_1
        {
            let mut rcj: *mut ratecontrol_entry_t = &mut *(*rcc)
                .entry
                .offset((i_1 + j) as isize) as *mut ratecontrol_entry_t;
            let mut frame_duration: ::core::ffi::c_double = x264_clip3f(
                (*rcj).i_duration as ::core::ffi::c_double * timescale,
                (0.01f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
                (1.00f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
            )
                / (0.04f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double;
            weight
                *= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - pow(
                        ((*rcj).i_count as ::core::ffi::c_float
                            / (*rcc).nmb as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        2 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
            if weight < 0.0001f64 {
                break;
            }
            gaussian_weight = weight * exp((-j * j) as ::core::ffi::c_double / 200.0f64);
            weight_sum += gaussian_weight;
            cplx_sum
                += gaussian_weight
                    * (qscale2bits(rcj, 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        - (*rcj).misc_bits as ::core::ffi::c_double) / frame_duration;
            j += 1;
        }
        weight = 1.0f64;
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 as ::core::ffi::c_double
            <= cplxblur * 2 as ::core::ffi::c_int as ::core::ffi::c_double && j_0 <= i_1
        {
            let mut rcj_0: *mut ratecontrol_entry_t = &mut *(*rcc)
                .entry
                .offset((i_1 - j_0) as isize) as *mut ratecontrol_entry_t;
            let mut frame_duration_0: ::core::ffi::c_double = x264_clip3f(
                (*rcj_0).i_duration as ::core::ffi::c_double * timescale,
                (0.01f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
                (1.00f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double,
            )
                / (0.04f32
                    / (((*h).param.i_frame_packing == 5 as ::core::ffi::c_int)
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float) as ::core::ffi::c_double;
            gaussian_weight = weight
                * exp((-j_0 * j_0) as ::core::ffi::c_double / 200.0f64);
            weight_sum += gaussian_weight;
            cplx_sum
                += gaussian_weight
                    * (qscale2bits(
                        rcj_0,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                    ) - (*rcj_0).misc_bits as ::core::ffi::c_double) / frame_duration_0;
            weight
                *= 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    - pow(
                        ((*rcj_0).i_count as ::core::ffi::c_float
                            / (*rcc).nmb as ::core::ffi::c_float)
                            as ::core::ffi::c_double,
                        2 as ::core::ffi::c_int as ::core::ffi::c_double,
                    );
            if weight < 0.0001f64 {
                break;
            }
            j_0 += 1;
        }
        (*rce_0).blurred_complexity = (cplx_sum / weight_sum) as ::core::ffi::c_float;
        i_1 += 1;
    }
    qscale = x264_malloc(
        (::core::mem::size_of::<::core::ffi::c_double>() as usize)
            .wrapping_mul((*rcc).num_entries as usize) as int64_t,
    ) as *mut ::core::ffi::c_double;
    if !qscale.is_null() {
        if filter_size > 1 as ::core::ffi::c_int {
            blurred_qscale = x264_malloc(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*rcc).num_entries as usize) as int64_t,
            ) as *mut ::core::ffi::c_double;
            if blurred_qscale.is_null() {
                current_block = 8692108595190266206;
            } else {
                current_block = 12199444798915819164;
            }
        } else {
            blurred_qscale = qscale;
            current_block = 12199444798915819164;
        }
        match current_block {
            8692108595190266206 => {}
            _ => {
                expected_bits = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < (*rcc).num_entries {
                    let mut q: ::core::ffi::c_double = get_qscale(
                        h,
                        &mut *(*rcc).entry.offset(i_2 as isize),
                        1.0f64,
                        i_2,
                    );
                    expected_bits
                        += qscale2bits(&mut *(*rcc).entry.offset(i_2 as isize), q);
                    (*rcc)
                        .last_qscale_for[(*(*rcc).entry.offset(i_2 as isize)).pict_type
                        as usize] = q;
                    i_2 += 1;
                }
                step_mult = all_available_bits as ::core::ffi::c_double / expected_bits;
                rate_factor = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                let mut step: ::core::ffi::c_double = 1E4f64 * step_mult;
                while step > 1E-7f64 * step_mult {
                    expected_bits = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    rate_factor += step;
                    (*rcc).last_non_b_pict_type = -(1 as ::core::ffi::c_int);
                    (*rcc).last_accum_p_norm = 1 as ::core::ffi::c_int
                        as ::core::ffi::c_double;
                    (*rcc).accum_p_norm = 0 as ::core::ffi::c_int
                        as ::core::ffi::c_double;
                    (*rcc).last_qscale_for[2 as ::core::ffi::c_int as usize] = pow(
                        base_cplx,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double
                            - (*rcc).qcompress,
                    ) / rate_factor;
                    (*rcc).last_qscale_for[1 as ::core::ffi::c_int as usize] = (*rcc)
                        .last_qscale_for[2 as ::core::ffi::c_int as usize];
                    (*rcc).last_qscale_for[0 as ::core::ffi::c_int as usize] = (*rcc)
                        .last_qscale_for[1 as ::core::ffi::c_int as usize];
                    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_3 < (*rcc).num_entries {
                        *qscale.offset(i_3 as isize) = get_qscale(
                            h,
                            &mut *(*rcc).entry.offset(i_3 as isize),
                            rate_factor,
                            -(1 as ::core::ffi::c_int),
                        );
                        (*rcc)
                            .last_qscale_for[(*(*rcc).entry.offset(i_3 as isize))
                            .pict_type as usize] = *qscale.offset(i_3 as isize);
                        i_3 += 1;
                    }
                    let mut i_4: ::core::ffi::c_int = (*rcc).num_entries
                        - 1 as ::core::ffi::c_int;
                    while i_4 >= 0 as ::core::ffi::c_int {
                        *qscale.offset(i_4 as isize) = get_diff_limited_q(
                            h,
                            &mut *(*rcc).entry.offset(i_4 as isize),
                            *qscale.offset(i_4 as isize),
                            i_4,
                        );
                        if *qscale.offset(i_4 as isize)
                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {} else {
                            __assert_fail(
                                b"qscale[i] >= 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/ratecontrol.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                3050 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 25],
                                    [::core::ffi::c_char; 25],
                                >(*b"int init_pass2(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                        'c_31398: {
                            if *qscale.offset(i_4 as isize)
                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {} else {
                                __assert_fail(
                                    b"qscale[i] >= 0\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"encoder/ratecontrol.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    3050 as ::core::ffi::c_uint,
                                    ::core::mem::transmute::<
                                        [u8; 25],
                                        [::core::ffi::c_char; 25],
                                    >(*b"int init_pass2(x264_t *)\0")
                                        .as_ptr(),
                                );
                            }
                        };
                        i_4 -= 1;
                    }
                    if filter_size > 1 as ::core::ffi::c_int {
                        if filter_size % 2 as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"filter_size%2 == 1\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/ratecontrol.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                3056 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 25],
                                    [::core::ffi::c_char; 25],
                                >(*b"int init_pass2(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                        'c_31348: {
                            if filter_size % 2 as ::core::ffi::c_int
                                == 1 as ::core::ffi::c_int
                            {} else {
                                __assert_fail(
                                    b"filter_size%2 == 1\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"encoder/ratecontrol.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    3056 as ::core::ffi::c_uint,
                                    ::core::mem::transmute::<
                                        [u8; 25],
                                        [::core::ffi::c_char; 25],
                                    >(*b"int init_pass2(x264_t *)\0")
                                        .as_ptr(),
                                );
                            }
                        };
                        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_5 < (*rcc).num_entries {
                            let mut rce_1: *mut ratecontrol_entry_t = &mut *(*rcc)
                                .entry
                                .offset(i_5 as isize) as *mut ratecontrol_entry_t;
                            let mut q_0: ::core::ffi::c_double = 0.0f64;
                            let mut sum: ::core::ffi::c_double = 0.0f64;
                            let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while j_1 < filter_size {
                                let mut idx: ::core::ffi::c_int = i_5 + j_1
                                    - filter_size / 2 as ::core::ffi::c_int;
                                let mut d: ::core::ffi::c_double = (idx - i_5)
                                    as ::core::ffi::c_double;
                                let mut coeff: ::core::ffi::c_double = if qblur
                                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    1.0f64
                                } else {
                                    exp(-d * d / (qblur * qblur))
                                };
                                if !(idx < 0 as ::core::ffi::c_int
                                    || idx >= (*rcc).num_entries)
                                {
                                    if !((*rce_1).pict_type
                                        != (*(*rcc).entry.offset(idx as isize)).pict_type)
                                    {
                                        q_0 += *qscale.offset(idx as isize) * coeff;
                                        sum += coeff;
                                    }
                                }
                                j_1 += 1;
                            }
                            *blurred_qscale.offset(i_5 as isize) = q_0 / sum;
                            i_5 += 1;
                        }
                    }
                    let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_6 < (*rcc).num_entries {
                        let mut rce_2: *mut ratecontrol_entry_t = &mut *(*rcc)
                            .entry
                            .offset(i_6 as isize) as *mut ratecontrol_entry_t;
                        (*rce_2).new_qscale = clip_qscale(
                            h,
                            (*rce_2).pict_type,
                            *blurred_qscale.offset(i_6 as isize),
                        );
                        if (*rce_2).new_qscale
                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {} else {
                            __assert_fail(
                                b"rce->new_qscale >= 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"encoder/ratecontrol.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                3083 as ::core::ffi::c_uint,
                                ::core::mem::transmute::<
                                    [u8; 25],
                                    [::core::ffi::c_char; 25],
                                >(*b"int init_pass2(x264_t *)\0")
                                    .as_ptr(),
                            );
                        }
                        'c_30953: {
                            if (*rce_2).new_qscale
                                >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                            {} else {
                                __assert_fail(
                                    b"rce->new_qscale >= 0\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"encoder/ratecontrol.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    3083 as ::core::ffi::c_uint,
                                    ::core::mem::transmute::<
                                        [u8; 25],
                                        [::core::ffi::c_char; 25],
                                    >(*b"int init_pass2(x264_t *)\0")
                                        .as_ptr(),
                                );
                            }
                        };
                        expected_bits += qscale2bits(rce_2, (*rce_2).new_qscale);
                        i_6 += 1;
                    }
                    if expected_bits > all_available_bits as ::core::ffi::c_double {
                        rate_factor -= step;
                    }
                    step *= 0.5f64;
                }
                x264_free(qscale as *mut ::core::ffi::c_void);
                if filter_size > 1 as ::core::ffi::c_int {
                    x264_free(blurred_qscale as *mut ::core::ffi::c_void);
                }
                if (*rcc).b_vbv != 0 {
                    if vbv_pass2(h, all_available_bits as ::core::ffi::c_double) != 0 {
                        return -(1 as ::core::ffi::c_int);
                    }
                }
                expected_bits = count_expected_bits(h);
                if fabs(
                    expected_bits / all_available_bits as ::core::ffi::c_double - 1.0f64,
                ) > 0.01f64
                {
                    let mut avgq: ::core::ffi::c_double = 0 as ::core::ffi::c_int
                        as ::core::ffi::c_double;
                    let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_7 < (*rcc).num_entries {
                        avgq += (*(*rcc).entry.offset(i_7 as isize)).new_qscale;
                        i_7 += 1;
                    }
                    avgq = qscale2qp(
                        (avgq / (*rcc).num_entries as ::core::ffi::c_double)
                            as ::core::ffi::c_float,
                    ) as ::core::ffi::c_double;
                    if expected_bits > all_available_bits as ::core::ffi::c_double
                        || (*rcc).b_vbv == 0
                    {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"Error: 2pass curve failed to converge\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    x264_10_log(
                        h,
                        X264_LOG_WARNING,
                        b"target: %.2f kbit/s, expected: %.2f kbit/s, avg QP: %.4f\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*h).param.rc.i_bitrate as ::core::ffi::c_float
                            as ::core::ffi::c_double,
                        expected_bits * (*rcc).fps
                            / ((*rcc).num_entries as ::core::ffi::c_double * 1000.0f64),
                        avgq,
                    );
                    if expected_bits < all_available_bits as ::core::ffi::c_double
                        && avgq
                            < ((*h).param.rc.i_qp_min + 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_double
                    {
                        if (*h).param.rc.i_qp_min > 0 as ::core::ffi::c_int {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try reducing target bitrate or reducing qp_min (currently %d)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                (*h).param.rc.i_qp_min,
                            );
                        } else {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try reducing target bitrate\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    } else if expected_bits > all_available_bits as ::core::ffi::c_double
                        && avgq
                            > ((*h).param.rc.i_qp_max - 2 as ::core::ffi::c_int)
                                as ::core::ffi::c_double
                    {
                        if (*h).param.rc.i_qp_max < QP_MAX {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try increasing target bitrate or increasing qp_max (currently %d)\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                (*h).param.rc.i_qp_max,
                            );
                        } else {
                            x264_10_log(
                                h,
                                X264_LOG_WARNING,
                                b"try increasing target bitrate\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    } else if !((*rcc).b_2pass != 0 && (*rcc).b_vbv != 0) {
                        x264_10_log(
                            h,
                            X264_LOG_WARNING,
                            b"internal error\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
