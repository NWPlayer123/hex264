// =============== BEGIN bitstream_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vlc_large_t {
    pub i_bits: crate::stdlib::uint16_t,
    pub i_size: crate::stdlib::uint8_t,
    pub i_next: crate::stdlib::uint8_t,
}

pub type bs_t = crate::src::common::bitstream::bs_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct bs_s {
    pub p_start: *mut crate::stdlib::uint8_t,
    pub p: *mut crate::stdlib::uint8_t,
    pub p_end: *mut crate::stdlib::uint8_t,
    pub cur_bits: crate::stdlib::uintptr_t,
    pub i_left: ::core::ffi::c_int,
    pub i_bits_encoded: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct x264_run_level_t {
    pub last: crate::stdlib::int32_t,
    pub mask: crate::stdlib::int32_t,
    pub level: [crate::src::common::common::dctcoef; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct x264_bitstream_function_t {
    pub nal_escape: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
            *mut crate::stdlib::uint8_t,
        ) -> *mut crate::stdlib::uint8_t,
    >,
    pub cabac_block_residual_internal: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
            crate::stdlib::intptr_t,
            *mut crate::src::common::cabac::x264_cabac_t,
        ) -> (),
    >,
    pub cabac_block_residual_rd_internal: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
            crate::stdlib::intptr_t,
            *mut crate::src::common::cabac::x264_cabac_t,
        ) -> (),
    >,
    pub cabac_block_residual_8x8_rd_internal: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            ::core::ffi::c_int,
            crate::stdlib::intptr_t,
            *mut crate::src::common::cabac::x264_cabac_t,
        ) -> (),
    >,
}

pub const LEVEL_TABLE_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::common::cabac::x264_cabac_t;
pub use crate::stdlib::C2Rust_Unnamed_7;
pub use crate::stdlib::__atomic_wide_counter;

pub use crate::internal::__va_list_tag;
pub use crate::src::common::common::dctcoef;
pub use crate::src::common::common::pixel;
pub use crate::src::common::common::udctcoef;
pub use crate::src::common::common::x264_frame_stat_t;
pub use crate::src::common::common::x264_left_table_t;
pub use crate::src::common::common::x264_lookahead_t;
pub use crate::src::common::common::x264_ratecontrol_t;
pub use crate::src::common::common::x264_slice_header_t;
pub use crate::src::common::common::x264_t;
pub use crate::src::common::common::C2Rust_Unnamed_10;
pub use crate::src::common::common::C2Rust_Unnamed_11;
pub use crate::src::common::common::C2Rust_Unnamed_12;
pub use crate::src::common::common::C2Rust_Unnamed_13;
pub use crate::src::common::common::C2Rust_Unnamed_14;
pub use crate::src::common::common::C2Rust_Unnamed_15;
pub use crate::src::common::common::C2Rust_Unnamed_16;
pub use crate::src::common::common::C2Rust_Unnamed_17;
pub use crate::src::common::common::C2Rust_Unnamed_8;
pub use crate::src::common::common::C2Rust_Unnamed_9;
pub use crate::src::common::common::NALU_OVERHEAD;
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;

pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_mc_functions_t;
pub use crate::src::common::mc::x264_weight_t;
pub use crate::src::common::pixel::x264_pixel_cmp_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x3_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x4_t;
pub use crate::src::common::pixel::x264_pixel_function_t;
pub use crate::src::common::predict::x264_predict8x8_t;
pub use crate::src::common::predict::x264_predict_8x8_filter_t;
pub use crate::src::common::predict::x264_predict_t;
pub use crate::src::common::quant::x264_quant_function_t;
pub use crate::stdlib::pthread_cond_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::pthread_t;

pub use crate::src::common::set::x264_pps_t;
pub use crate::src::common::set::x264_sps_t;
pub use crate::src::common::set::C2Rust_Unnamed_24;
pub use crate::src::common::set::C2Rust_Unnamed_25;
pub use crate::src::common::set::C2Rust_Unnamed_26;
use crate::src::common::threadpool::x264_threadpool_t;
pub use crate::stdlib::__pthread_cond_s;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::int8_t;
pub use crate::stdlib::intptr_t;
use crate::stdlib::memset;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::uintptr_t;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__int8_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::x264_h::x264_hrd_t;
pub use crate::x264_h::x264_nal_t;
pub use crate::x264_h::x264_param_t;
pub use crate::x264_h::x264_sei_payload_t;
pub use crate::x264_h::x264_sei_t;
pub use crate::x264_h::x264_zone_t;
pub use crate::x264_h::C2Rust_Unnamed_0;
pub use crate::x264_h::C2Rust_Unnamed_1;
pub use crate::x264_h::C2Rust_Unnamed_2;
pub use crate::x264_h::C2Rust_Unnamed_3;
pub use crate::x264_h::C2Rust_Unnamed_4;
pub use crate::x264_h::C2Rust_Unnamed_5;

unsafe extern "C" fn nal_escape_c(
    mut dst: *mut crate::stdlib::uint8_t,
    mut src: *mut crate::stdlib::uint8_t,
    mut end: *mut crate::stdlib::uint8_t,
) -> *mut crate::stdlib::uint8_t {
    unsafe {
        if src < end {
            let c2rust_fresh0 = src;
            src = src.offset(1);
            let c2rust_fresh1 = dst;
            dst = dst.offset(1);
            *c2rust_fresh1 = *c2rust_fresh0;
        }
        if src < end {
            let c2rust_fresh2 = src;
            src = src.offset(1);
            let c2rust_fresh3 = dst;
            dst = dst.offset(1);
            *c2rust_fresh3 = *c2rust_fresh2;
        }
        while src < end {
            if *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= 0x3 as ::core::ffi::c_int
                && *dst.offset(-(2 as ::core::ffi::c_int) as isize) == 0
                && *dst.offset(-(1 as ::core::ffi::c_int) as isize) == 0
            {
                let c2rust_fresh4 = dst;
                dst = dst.offset(1);
                *c2rust_fresh4 = 0x3 as crate::stdlib::uint8_t;
            }
            let c2rust_fresh5 = src;
            src = src.offset(1);
            let c2rust_fresh6 = dst;
            dst = dst.offset(1);
            *c2rust_fresh6 = *c2rust_fresh5;
        }
        return dst;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_nal_encode(
    mut h: *mut crate::src::common::common::x264_t,
    mut dst: *mut crate::stdlib::uint8_t,
    mut nal: *mut crate::x264_h::x264_nal_t,
) {
    unsafe {
        let mut src: *mut crate::stdlib::uint8_t = (*nal).p_payload;
        let mut end: *mut crate::stdlib::uint8_t =
            (*nal).p_payload.offset((*nal).i_payload as isize);
        let mut orig_dst: *mut crate::stdlib::uint8_t = dst;
        if (*h).param.b_annexb != 0 {
            if (*nal).b_long_startcode != 0 {
                let c2rust_fresh7 = dst;
                dst = dst.offset(1);
                *c2rust_fresh7 = 0 as crate::stdlib::uint8_t;
            }
            let c2rust_fresh8 = dst;
            dst = dst.offset(1);
            *c2rust_fresh8 = 0 as crate::stdlib::uint8_t;
            let c2rust_fresh9 = dst;
            dst = dst.offset(1);
            *c2rust_fresh9 = 0 as crate::stdlib::uint8_t;
            let c2rust_fresh10 = dst;
            dst = dst.offset(1);
            *c2rust_fresh10 = 0x1 as crate::stdlib::uint8_t;
        } else {
            dst = dst.offset(4 as ::core::ffi::c_int as isize);
        }
        let c2rust_fresh11 = dst;
        dst = dst.offset(1);
        *c2rust_fresh11 = ((0 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            | (*nal).i_ref_idc << 5 as ::core::ffi::c_int
            | (*nal).i_type) as crate::stdlib::uint8_t;
        dst = (*h).bsf.nal_escape.expect("non-null function pointer")(dst, src, end);
        let mut size: ::core::ffi::c_int =
            dst.offset_from(orig_dst) as ::core::ffi::c_long as ::core::ffi::c_int;
        if (*h).param.i_avcintra_class != 0 {
            let mut padding: ::core::ffi::c_int =
                (*nal).i_payload + (*nal).i_padding + crate::src::common::common::NALU_OVERHEAD
                    - size;
            if padding > 0 as ::core::ffi::c_int {
                crate::stdlib::memset(
                    dst as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    padding as crate::__stddef_size_t_h::size_t,
                );
                size += padding;
            }
            (*nal).i_padding = if padding > 0 as ::core::ffi::c_int {
                padding
            } else {
                0 as ::core::ffi::c_int
            };
        }
        if (*h).param.b_annexb == 0 {
            let mut chunk_size: ::core::ffi::c_int = size - 4 as ::core::ffi::c_int;
            *orig_dst.offset(0 as ::core::ffi::c_int as isize) =
                (chunk_size >> 24 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
            *orig_dst.offset(1 as ::core::ffi::c_int as isize) =
                (chunk_size >> 16 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
            *orig_dst.offset(2 as ::core::ffi::c_int as isize) =
                (chunk_size >> 8 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
            *orig_dst.offset(3 as ::core::ffi::c_int as isize) =
                (chunk_size >> 0 as ::core::ffi::c_int) as crate::stdlib::uint8_t;
        }
        (*nal).i_payload = size;
        (*nal).p_payload = orig_dst;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_bitstream_init(
    mut cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::bitstream::x264_bitstream_function_t,
) {
    unsafe {
        crate::stdlib::memset(
            pf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::common::bitstream::x264_bitstream_function_t>()
                as crate::__stddef_size_t_h::size_t,
        );
        (*pf).nal_escape = Some(
            nal_escape_c
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                ) -> *mut crate::stdlib::uint8_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                ) -> *mut crate::stdlib::uint8_t,
            >;
    }
}
