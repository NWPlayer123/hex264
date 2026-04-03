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
pub const LEVEL_TABLE_SIZE: ::core::ffi::c_int = 128i32;
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
            if *src.offset(0isize) as ::core::ffi::c_int <= 0x3i32
                && *dst.offset(-2isize) == 0
                && *dst.offset(-1isize) == 0
            {
                let c2rust_fresh4 = dst;
                dst = dst.offset(1);
                *c2rust_fresh4 = 0x3u8;
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
        let mut src = (*nal).p_payload;
        let mut end = (*nal).p_payload.offset((*nal).i_payload as isize);
        let mut orig_dst = dst;
        if (*h).param.annexb {
            if (*nal).long_startcode {
                let c2rust_fresh7 = dst;
                dst = dst.offset(1);
                *c2rust_fresh7 = 0u8;
            }
            let c2rust_fresh8 = dst;
            dst = dst.offset(1);
            *c2rust_fresh8 = 0u8;
            let c2rust_fresh9 = dst;
            dst = dst.offset(1);
            *c2rust_fresh9 = 0u8;
            let c2rust_fresh10 = dst;
            dst = dst.offset(1);
            *c2rust_fresh10 = 0x1u8;
        } else {
            dst = dst.offset(4isize);
        }
        let c2rust_fresh11 = dst;
        dst = dst.offset(1);
        *c2rust_fresh11 =
            ((0i32) << 7i32 | (*nal).i_ref_idc << 5i32 | (*nal).i_type) as crate::stdlib::uint8_t;
        dst = (*h).bsf.nal_escape.expect("non-null function pointer")(dst, src, end);
        let mut size = dst.offset_from(orig_dst) as ::core::ffi::c_int;
        if (*h).param.i_avcintra_class != 0 {
            let mut padding =
                (*nal).i_payload + (*nal).i_padding + crate::src::common::common::NALU_OVERHEAD
                    - size;
            if padding > 0i32 {
                crate::stdlib::memset(
                    dst as *mut ::core::ffi::c_void,
                    0i32,
                    padding as crate::__stddef_size_t_h::size_t,
                );
                size += padding;
            }
            (*nal).i_padding = if padding > 0i32 { padding } else { 0i32 };
        }
        if !(*h).param.annexb {
            let mut chunk_size = size - 4i32;
            *orig_dst.offset(0isize) = (chunk_size >> 24i32) as crate::stdlib::uint8_t;
            *orig_dst.offset(1isize) = (chunk_size >> 16i32) as crate::stdlib::uint8_t;
            *orig_dst.offset(2isize) = (chunk_size >> 8i32) as crate::stdlib::uint8_t;
            *orig_dst.offset(3isize) = (chunk_size >> 0i32) as crate::stdlib::uint8_t;
        }
        (*nal).i_payload = size;
        (*nal).p_payload = orig_dst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_bitstream_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::bitstream::x264_bitstream_function_t,
) {
    unsafe {
        crate::stdlib::memset(
            pf as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::src::common::bitstream::x264_bitstream_function_t>(),
        );
        (*pf).nal_escape = Some(
            nal_escape_c
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                    *mut crate::stdlib::uint8_t,
                ) -> *mut crate::stdlib::uint8_t,
        );
    }
}
