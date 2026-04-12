// =============== BEGIN frame_h ================
pub const PADH: ::core::ffi::c_int = 32i32;
pub const PADV: ::core::ffi::c_int = 32i32;
pub type x264_frame_t = crate::src::common::frame::x264_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_frame {
    pub base: *mut crate::stdlib::uint8_t,
    pub i_poc: ::core::ffi::c_int,
    pub i_delta_poc: [::core::ffi::c_int; 2],
    pub i_type: ::core::ffi::c_int,
    pub i_forced_type: ::core::ffi::c_int,
    pub i_qpplus1: ::core::ffi::c_int,
    pub i_pts: crate::stdlib::int64_t,
    pub i_dts: crate::stdlib::int64_t,
    pub i_reordered_pts: crate::stdlib::int64_t,
    pub i_duration: crate::stdlib::int64_t,
    pub f_duration: ::core::ffi::c_float,
    pub i_cpb_duration: crate::stdlib::int64_t,
    pub i_cpb_delay: crate::stdlib::int64_t,
    pub i_dpb_output_delay: crate::stdlib::int64_t,
    pub param: *mut crate::x264_h::x264_param_t,
    pub i_frame: ::core::ffi::c_int,
    pub i_coded: ::core::ffi::c_int,
    pub i_field_cnt: crate::stdlib::int64_t,
    pub i_frame_num: ::core::ffi::c_int,
    pub kept_as_ref: bool,
    pub i_pic_struct: ::core::ffi::c_int,
    pub keyframe: bool,
    pub b_fdec: crate::stdlib::uint8_t,
    pub b_last_minigop_bframe: crate::stdlib::uint8_t,
    pub i_bframes: crate::stdlib::uint8_t,
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
    pub plane: [*mut crate::src::common::common::pixel; 3],
    pub plane_fld: [*mut crate::src::common::common::pixel; 3],
    pub filtered: [[*mut crate::src::common::common::pixel; 4]; 3],
    pub filtered_fld: [[*mut crate::src::common::common::pixel; 4]; 3],
    pub lowres: [*mut crate::src::common::common::pixel; 4],
    pub integral: *mut crate::stdlib::uint16_t,
    pub buffer: [*mut crate::src::common::common::pixel; 4],
    pub buffer_fld: [*mut crate::src::common::common::pixel; 4],
    pub buffer_lowres: *mut crate::src::common::common::pixel,
    pub weight: [[crate::src::common::mc::x264_weight_t; 3]; 16],
    pub weighted: [*mut crate::src::common::common::pixel; 16],
    pub duplicate: bool,
    pub orig: *mut crate::src::common::frame::x264_frame,
    pub mb_type: *mut crate::stdlib::int8_t,
    pub mb_partition: *mut crate::stdlib::uint8_t,
    pub mv: [*mut [crate::stdlib::int16_t; 2]; 2],
    pub mv16x16: *mut [crate::stdlib::int16_t; 2],
    pub lowres_mvs: [[*mut [crate::stdlib::int16_t; 2]; 17]; 2],
    pub field: *mut crate::stdlib::uint8_t,
    pub effective_qp: *mut crate::stdlib::uint8_t,
    pub lowres_costs: [[*mut crate::stdlib::uint16_t; 18]; 18],
    pub lowres_mv_costs: [[*mut ::core::ffi::c_int; 17]; 2],
    pub ref_0: [*mut crate::stdlib::int8_t; 2],
    pub i_ref: [::core::ffi::c_int; 2],
    pub ref_poc: [[::core::ffi::c_int; 16]; 2],
    pub inv_ref_poc: [crate::stdlib::int16_t; 2],
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
    pub intra_calculated: bool,
    pub i_intra_cost: *mut crate::stdlib::uint16_t,
    pub i_propagate_cost: *mut crate::stdlib::uint16_t,
    pub i_inv_qscale_factor: *mut crate::stdlib::uint16_t,
    pub scenecut: bool,
    pub f_weighted_cost_delta: [::core::ffi::c_float; 18],
    pub i_pixel_sum: [crate::stdlib::uint32_t; 3],
    pub i_pixel_ssd: [crate::stdlib::uint64_t; 3],
    pub hrd_timing: crate::x264_h::x264_hrd_t,
    pub i_planned_type: [crate::stdlib::uint8_t; 251],
    pub i_planned_satd: [::core::ffi::c_int; 251],
    pub f_planned_cpb_duration: [::core::ffi::c_double; 251],
    pub i_coded_fields_lookahead: crate::stdlib::int64_t,
    pub i_cpb_delay_lookahead: crate::stdlib::int64_t,
    pub i_lines_completed: ::core::ffi::c_int,
    pub i_lines_weighted: ::core::ffi::c_int,
    pub i_reference_count: ::core::ffi::c_int,
    pub mutex: crate::stdlib::pthread_mutex_t,
    pub cv: crate::stdlib::pthread_cond_t,
    pub i_slice_count: ::core::ffi::c_int,
    pub f_pir_position: ::core::ffi::c_float,
    pub i_pir_start_col: ::core::ffi::c_int,
    pub i_pir_end_col: ::core::ffi::c_int,
    pub i_frames_since_pir: ::core::ffi::c_int,
    pub corrupt: bool,
    pub extra_sei: crate::x264_h::x264_sei_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub mb_info: *mut crate::stdlib::uint8_t,
    pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub const LOWRES_COST_MASK: ::core::ffi::c_int = ((1i32) << 14i32) - 1i32;
pub const LOWRES_COST_SHIFT: ::core::ffi::c_int = 14i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sync_frame_list_t {
    pub list: *mut *mut crate::src::common::frame::x264_frame_t,
    pub i_max_size: ::core::ffi::c_int,
    pub i_size: ::core::ffi::c_int,
    pub mutex: crate::stdlib::pthread_mutex_t,
    pub cv_fill: crate::stdlib::pthread_cond_t,
    pub cv_empty: crate::stdlib::pthread_cond_t,
}
pub type x264_deblock_inter_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut crate::stdlib::int8_t,
    ) -> (),
>;
pub type x264_deblock_intra_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_deblock_function_t {
    pub deblock_luma: [crate::src::common::frame::x264_deblock_inter_t; 2],
    pub deblock_chroma: [crate::src::common::frame::x264_deblock_inter_t; 2],
    pub deblock_h_chroma_420: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_h_chroma_422: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_luma_intra: [crate::src::common::frame::x264_deblock_intra_t; 2],
    pub deblock_chroma_intra: [crate::src::common::frame::x264_deblock_intra_t; 2],
    pub deblock_h_chroma_420_intra: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_h_chroma_422_intra: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_luma_mbaff: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_chroma_mbaff: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_chroma_420_mbaff: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_chroma_422_mbaff: crate::src::common::frame::x264_deblock_inter_t,
    pub deblock_luma_intra_mbaff: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_chroma_intra_mbaff: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_chroma_420_intra_mbaff: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_chroma_422_intra_mbaff: crate::src::common::frame::x264_deblock_intra_t,
    pub deblock_strength: Option<
        unsafe extern "C" fn(
            *mut crate::stdlib::uint8_t,
            *mut [crate::stdlib::int8_t; 40],
            *mut [[crate::stdlib::int16_t; 2]; 40],
            *mut [[crate::stdlib::uint8_t; 4]; 8],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
}
pub mod osdep_h {
    use std::sync::atomic::{AtomicI32, Ordering};
    #[inline(always)]
    pub unsafe extern "C" fn x264_pthread_fetch_and_add(
        mut val: *mut ::core::ffi::c_int,
        mut add: ::core::ffi::c_int,
        mut _mutex: *mut crate::stdlib::pthread_mutex_t,
    ) -> ::core::ffi::c_int {
        unsafe { (*AtomicI32::from_ptr(val)).fetch_add(add, Ordering::SeqCst) }
    }
}
use crate::src::common::frame::osdep_h::x264_pthread_fetch_and_add;
unsafe extern "C" fn align_stride(
    mut x: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut disalign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    x = (x + (align - 1i32)) & !(align - 1i32);
    if x & (disalign - 1i32) == 0 {
        x += align;
    }
    x
}
unsafe extern "C" fn align_plane_size(
    mut x: ::core::ffi::c_int,
    mut disalign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if x & (disalign - 1i32) == 0 {
        x += (if 128i32 > 64i32 { 128i32 } else { 64i32 })
            / crate::src::common::common::SIZEOF_PIXEL;
    }
    x
}
unsafe extern "C" fn frame_internal_csp(
    mut external_csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut csp = external_csp & crate::x264_h::X264_CSP_MASK;
    if csp == crate::x264_h::X264_CSP_I400 {
        return crate::x264_h::X264_CSP_I400;
    }
    if csp >= crate::x264_h::X264_CSP_I420 && csp < crate::x264_h::X264_CSP_I422 {
        return crate::x264_h::X264_CSP_NV12;
    }
    if csp >= crate::x264_h::X264_CSP_I422 && csp < crate::x264_h::X264_CSP_I444 {
        return crate::x264_h::X264_CSP_NV16;
    }
    if csp >= crate::x264_h::X264_CSP_I444 && csp <= crate::x264_h::X264_CSP_RGB {
        return crate::x264_h::X264_CSP_I444;
    }
    crate::x264_h::X264_CSP_NONE
}
unsafe extern "C" fn frame_new(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_fdec: ::core::ffi::c_int,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        let mut i_csp = frame_internal_csp((*h).param.i_csp);
        let mut i_mb_count = (*h).mb.i_mb_count;
        let mut i_padv =
            crate::src::common::frame::PADV << (*h).param.interlaced as ::core::ffi::c_int;
        let mut align = crate::osdep_h::NATIVE_ALIGN / crate::src::common::common::SIZEOF_PIXEL;
        if (*h).param.cpu & crate::x264_h::X264_CPU_CACHELINE_64 != 0
            || (*h).param.cpu & crate::x264_h::X264_CPU_AVX512 != 0
        {
            align = 64i32 / crate::src::common::common::SIZEOF_PIXEL;
        } else if (*h).param.cpu & crate::x264_h::X264_CPU_CACHELINE_32 != 0
            || (*h).param.cpu & crate::x264_h::X264_CPU_AVX != 0
        {
            align = 32i32 / crate::src::common::common::SIZEOF_PIXEL;
        } else {
            align = 16i32 / crate::src::common::common::SIZEOF_PIXEL;
        }
        let mut disalign = ((1i32) << 10i32) / crate::src::common::common::SIZEOF_PIXEL;
        let mut frame = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::src::common::frame::x264_frame_t,
        >() as crate::stdlib::int64_t)
            as *mut crate::src::common::frame::x264_frame_t;
        if !frame.is_null() {
            let mut prealloc_idx = 0;
            let mut prealloc_size = 0;
            let mut preallocs = [::core::ptr::null_mut::<*mut crate::stdlib::uint8_t>(); 1024];
            let mut c2rust_current_block: u64;
            let mut i_stride = 0;
            let mut i_width = 0;
            let mut i_lines = 0;
            let mut luma_plane_count = 0;
            crate::stdlib::memset(
                frame as *mut ::core::ffi::c_void,
                0i32,
                ::core::mem::size_of::<crate::src::common::frame::x264_frame_t>(),
            );
            prealloc_idx = 0i32;
            prealloc_size = 0i64;
            preallocs = [::core::ptr::null_mut::<*mut crate::stdlib::uint8_t>(); 1024];
            i_width = (*h).mb.i_mb_width * 16i32;
            i_lines = (*h).mb.i_mb_height * 16i32;
            i_stride = align_stride(
                i_width
                    + ((if 32i32
                        > 64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    {
                        32i32
                    } else {
                        64i32
                            / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                    }) + crate::src::common::frame::PADH),
                align,
                disalign,
            );
            if i_csp == crate::x264_h::X264_CSP_NV12 || i_csp == crate::x264_h::X264_CSP_NV16 {
                let mut i = 0i32;
                luma_plane_count = 1i32;
                (*frame).i_plane = 2i32;
                while i < 2i32 {
                    (*frame).i_width[i as usize] = i_width >> i;
                    (*frame).i_lines[i as usize] = i_lines
                        >> (i != 0 && i_csp == crate::x264_h::X264_CSP_NV12) as ::core::ffi::c_int;
                    (*frame).i_stride[i as usize] = i_stride;
                    i += 1;
                }
                c2rust_current_block = 7245201122033322888;
            } else if i_csp == crate::x264_h::X264_CSP_I444 {
                let mut i_0 = 0i32;
                luma_plane_count = 3i32;
                (*frame).i_plane = 3i32;
                while i_0 < 3i32 {
                    (*frame).i_width[i_0 as usize] = i_width;
                    (*frame).i_lines[i_0 as usize] = i_lines;
                    (*frame).i_stride[i_0 as usize] = i_stride;
                    i_0 += 1;
                }
                c2rust_current_block = 7245201122033322888;
            } else if i_csp == crate::x264_h::X264_CSP_I400 {
                luma_plane_count = 1i32;
                (*frame).i_plane = 1i32;
                (*frame).i_width[0usize] = i_width;
                (*frame).i_lines[0usize] = i_lines;
                (*frame).i_stride[0usize] = i_stride;
                c2rust_current_block = 7245201122033322888;
            } else {
                c2rust_current_block = 1174525651277089986;
            }
            match c2rust_current_block {
                1174525651277089986 => {}
                _ => {
                    let mut i_1 = 0i32;
                    let mut p = 0i32;
                    (*frame).i_csp = i_csp;
                    (*frame).i_width_lowres = (*frame).i_width[0usize] / 2i32;
                    (*frame).i_lines_lowres = (*frame).i_lines[0usize] / 2i32;
                    (*frame).i_stride_lowres = align_stride(
                        (*frame).i_width_lowres
                            + ((if 32i32
                                > 64i32
                                    / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                        as ::core::ffi::c_int
                            {
                                32i32
                            } else {
                                64i32
                                    / ::core::mem::size_of::<crate::src::common::common::pixel>()
                                        as ::core::ffi::c_int
                            }) + crate::src::common::frame::PADH),
                        align,
                        disalign << 1i32,
                    );
                    while i_1 < (*h).param.i_bframe + 2i32 {
                        let mut j = 0i32;
                        while j < (*h).param.i_bframe + 2i32 {
                            (*frame).i_row_satds[i_1 as usize][j as usize] =
                                prealloc_size as *mut ::core::ffi::c_int;
                            let c2rust_fresh8 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh8 as usize] = (&raw mut *(&raw mut (*frame)
                                .i_row_satds
                                as *mut [*mut ::core::ffi::c_int; 18])
                                .offset(i_1 as isize)
                                as *mut *mut ::core::ffi::c_int)
                                .offset(j as isize)
                                as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += (((i_lines / 16i32) as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            j += 1;
                        }
                        i_1 += 1;
                    }
                    (*frame).i_poc = -(1i32);
                    (*frame).i_type = crate::x264_h::X264_TYPE_AUTO;
                    (*frame).i_qpplus1 = crate::x264_h::X264_QP_AUTO;
                    (*frame).i_pts = -1i64;
                    (*frame).i_frame = -(1i32);
                    (*frame).i_frame_num = -(1i32);
                    (*frame).i_lines_completed = -(1i32);
                    (*frame).b_fdec = b_fdec as crate::stdlib::uint8_t;
                    (*frame).i_pic_struct = crate::x264_h::PIC_STRUCT_AUTO as ::core::ffi::c_int;
                    (*frame).i_field_cnt = -1i64;
                    (*frame).i_cpb_delay = 0i64;
                    (*frame).i_dpb_output_delay = (*frame).i_cpb_delay;
                    (*frame).i_cpb_duration = (*frame).i_dpb_output_delay;
                    (*frame).i_duration = (*frame).i_cpb_duration;
                    (*frame).i_cpb_delay_lookahead = -1i64;
                    (*frame).i_coded_fields_lookahead = (*frame).i_cpb_delay_lookahead;
                    (*frame).orig = frame;
                    if i_csp == crate::x264_h::X264_CSP_NV12
                        || i_csp == crate::x264_h::X264_CSP_NV16
                    {
                        let mut chroma_padv =
                            i_padv >> (i_csp == crate::x264_h::X264_CSP_NV12) as ::core::ffi::c_int;
                        let mut chroma_plane_size = (*frame).i_stride[1usize]
                            * ((*frame).i_lines[1usize] + 2i32 * chroma_padv);
                        (*frame).buffer[1usize] =
                            prealloc_size as *mut crate::src::common::common::pixel;
                        let c2rust_fresh9 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh9 as usize] = (&raw mut (*frame).buffer
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1isize);
                        prealloc_size += ((chroma_plane_size
                            * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int)
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        if (*h).param.interlaced {
                            (*frame).buffer_fld[1usize] =
                                prealloc_size as *mut crate::src::common::common::pixel;
                            let c2rust_fresh10 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh10 as usize] = (&raw mut (*frame).buffer_fld
                                as *mut *mut crate::src::common::common::pixel)
                                .offset(1isize);
                            prealloc_size += ((chroma_plane_size
                                * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int)
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        }
                    }
                    while p < luma_plane_count {
                        let mut luma_plane_size = align_plane_size(
                            (*frame).i_stride[p as usize]
                                * ((*frame).i_lines[p as usize] + 2i32 * i_padv),
                            disalign,
                        )
                            as crate::stdlib::int64_t;
                        if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                            luma_plane_size *= 4i64;
                        }
                        (*frame).buffer[p as usize] =
                            prealloc_size as *mut crate::src::common::common::pixel;
                        let c2rust_fresh11 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh11 as usize] = (&raw mut (*frame).buffer
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(p as isize);
                        prealloc_size += (luma_plane_size
                            * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                as ::core::ffi::c_int
                                as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        if (*h).param.interlaced {
                            (*frame).buffer_fld[p as usize] =
                                prealloc_size as *mut crate::src::common::common::pixel;
                            let c2rust_fresh12 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh12 as usize] = (&raw mut (*frame).buffer_fld
                                as *mut *mut crate::src::common::common::pixel)
                                .offset(p as isize);
                            prealloc_size += (luma_plane_size
                                * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int
                                    as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        }
                        p += 1;
                    }
                    (*frame).duplicate = false;
                    if b_fdec != 0 {
                        (*frame).mb_type = prealloc_size as *mut crate::stdlib::int8_t;
                        let c2rust_fresh13 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh13 as usize] =
                            &raw mut (*frame).mb_type as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += ((i_mb_count as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).mb_partition = prealloc_size as *mut crate::stdlib::uint8_t;
                        let c2rust_fresh14 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh14 as usize] = &raw mut (*frame).mb_partition;
                        prealloc_size += ((i_mb_count as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).mv[0usize] = prealloc_size as *mut [crate::stdlib::int16_t; 2];
                        let c2rust_fresh15 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh15 as usize] = (&raw mut (*frame).mv
                            as *mut *mut [crate::stdlib::int16_t; 2])
                            .offset(0isize)
                            as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((2i32 * 16i32 * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).mv16x16 = prealloc_size as *mut [crate::stdlib::int16_t; 2];
                        let c2rust_fresh16 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh16 as usize] =
                            &raw mut (*frame).mv16x16 as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((2i32 * (i_mb_count + 1i32)) as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).ref_0[0usize] = prealloc_size as *mut crate::stdlib::int8_t;
                        let c2rust_fresh17 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh17 as usize] = (&raw mut (*frame).ref_0
                            as *mut *mut crate::stdlib::int8_t)
                            .offset(0isize)
                            as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((4i32 * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        if (*h).param.i_bframe != 0 {
                            (*frame).mv[1usize] = prealloc_size as *mut [crate::stdlib::int16_t; 2];
                            let c2rust_fresh18 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh18 as usize] = (&raw mut (*frame).mv
                                as *mut *mut [crate::stdlib::int16_t; 2])
                                .offset(1isize)
                                as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += (((2i32 * 16i32 * i_mb_count) as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            (*frame).ref_0[1usize] = prealloc_size as *mut crate::stdlib::int8_t;
                            let c2rust_fresh19 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh19 as usize] = (&raw mut (*frame).ref_0
                                as *mut *mut crate::stdlib::int8_t)
                                .offset(1isize)
                                as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += (((4i32 * i_mb_count) as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        } else {
                            (*frame).mv[1usize] =
                                ::core::ptr::null_mut::<[crate::stdlib::int16_t; 2]>();
                            (*frame).ref_0[1usize] =
                                ::core::ptr::null_mut::<crate::stdlib::int8_t>();
                        }
                        (*frame).i_row_bits = prealloc_size as *mut ::core::ffi::c_int;
                        let c2rust_fresh20 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh20 as usize] =
                            &raw mut (*frame).i_row_bits as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((i_lines / 16i32) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).f_row_qp = prealloc_size as *mut ::core::ffi::c_float;
                        let c2rust_fresh21 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh21 as usize] =
                            &raw mut (*frame).f_row_qp as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((i_lines / 16i32) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        (*frame).f_row_qscale = prealloc_size as *mut ::core::ffi::c_float;
                        let c2rust_fresh22 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[c2rust_fresh22 as usize] =
                            &raw mut (*frame).f_row_qscale as *mut *mut crate::stdlib::uint8_t;
                        prealloc_size += (((i_lines / 16i32) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>())
                            as crate::stdlib::int64_t
                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA {
                            (*frame).buffer[3usize] =
                                prealloc_size as *mut crate::src::common::common::pixel;
                            let c2rust_fresh23 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh23 as usize] = (&raw mut (*frame).buffer
                                as *mut *mut crate::src::common::common::pixel)
                                .offset(3isize);
                            prealloc_size += (((((*frame).i_stride[0usize]
                                * ((*frame).i_lines[0usize] + 2i32 * i_padv))
                                as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>())
                                << (*h).frames.have_sub8x8_esa as ::core::ffi::c_int)
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        }
                        if (*h).param.interlaced {
                            (*frame).field = prealloc_size as *mut crate::stdlib::uint8_t;
                            let c2rust_fresh24 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh24 as usize] = &raw mut (*frame).field;
                            prealloc_size += ((i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        }
                        if (*h).param.analyse.mb_info {
                            (*frame).effective_qp = prealloc_size as *mut crate::stdlib::uint8_t;
                            let c2rust_fresh25 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh25 as usize] = &raw mut (*frame).effective_qp;
                            prealloc_size += ((i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                        }
                    } else {
                        if (*h).frames.have_lowres {
                            let mut j_0 = 0i32;
                            let mut j_1 = 0i32;
                            let mut luma_plane_size_0 = align_plane_size(
                                (*frame).i_stride_lowres
                                    * ((*frame).i_lines[0usize] / 2i32
                                        + 2i32 * crate::src::common::frame::PADV),
                                disalign,
                            )
                                as crate::stdlib::int64_t;
                            (*frame).buffer_lowres =
                                prealloc_size as *mut crate::src::common::common::pixel;
                            let c2rust_fresh26 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh26 as usize] = &raw mut (*frame).buffer_lowres;
                            prealloc_size += (4i64
                                * luma_plane_size_0
                                * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int
                                    as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            while j_0 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
                                let mut i_2 = 0i32;
                                while i_2 <= (*h).param.i_bframe {
                                    (*frame).lowres_mvs[j_0 as usize][i_2 as usize] =
                                        prealloc_size as *mut [crate::stdlib::int16_t; 2];
                                    let c2rust_fresh27 = prealloc_idx;
                                    prealloc_idx = prealloc_idx + 1;
                                    preallocs[c2rust_fresh27 as usize] =
                                        (&raw mut *(&raw mut (*frame).lowres_mvs
                                            as *mut [*mut [crate::stdlib::int16_t; 2]; 17])
                                            .offset(j_0 as isize)
                                            as *mut *mut [crate::stdlib::int16_t; 2])
                                            .offset(i_2 as isize)
                                            as *mut *mut crate::stdlib::uint8_t;
                                    prealloc_size += (((2i32 * i_mb_count) as usize).wrapping_mul(
                                        ::core::mem::size_of::<crate::stdlib::int16_t>(),
                                    )
                                        as crate::stdlib::int64_t
                                        + (64i32 - 1i32) as crate::stdlib::int64_t)
                                        & !(64i32 - 1i32) as crate::stdlib::int64_t;
                                    (*frame).lowres_mv_costs[j_0 as usize][i_2 as usize] =
                                        prealloc_size as *mut ::core::ffi::c_int;
                                    let c2rust_fresh28 = prealloc_idx;
                                    prealloc_idx = prealloc_idx + 1;
                                    preallocs[c2rust_fresh28 as usize] =
                                        (&raw mut *(&raw mut (*frame).lowres_mv_costs
                                            as *mut [*mut ::core::ffi::c_int; 17])
                                            .offset(j_0 as isize)
                                            as *mut *mut ::core::ffi::c_int)
                                            .offset(i_2 as isize)
                                            as *mut *mut crate::stdlib::uint8_t;
                                    prealloc_size += ((i_mb_count as usize)
                                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                                        as crate::stdlib::int64_t
                                        + (64i32 - 1i32) as crate::stdlib::int64_t)
                                        & !(64i32 - 1i32) as crate::stdlib::int64_t;
                                    i_2 += 1;
                                }
                                j_0 += 1;
                            }
                            (*frame).i_propagate_cost =
                                prealloc_size as *mut crate::stdlib::uint16_t;
                            let c2rust_fresh29 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh29 as usize] = &raw mut (*frame).i_propagate_cost
                                as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += ((i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            while j_1 <= (*h).param.i_bframe + 1i32 {
                                let mut i_3 = 0i32;
                                while i_3 <= (*h).param.i_bframe + 1i32 {
                                    (*frame).lowres_costs[j_1 as usize][i_3 as usize] =
                                        prealloc_size as *mut crate::stdlib::uint16_t;
                                    let c2rust_fresh30 = prealloc_idx;
                                    prealloc_idx = prealloc_idx + 1;
                                    preallocs[c2rust_fresh30 as usize] =
                                        (&raw mut *(&raw mut (*frame).lowres_costs
                                            as *mut [*mut crate::stdlib::uint16_t; 18])
                                            .offset(j_1 as isize)
                                            as *mut *mut crate::stdlib::uint16_t)
                                            .offset(i_3 as isize)
                                            as *mut *mut crate::stdlib::uint8_t;
                                    prealloc_size +=
                                        ((i_mb_count as usize).wrapping_mul(::core::mem::size_of::<
                                            crate::stdlib::uint16_t,
                                        >(
                                        ))
                                            as crate::stdlib::int64_t
                                            + (64i32 - 1i32) as crate::stdlib::int64_t)
                                            & !(64i32 - 1i32) as crate::stdlib::int64_t;
                                    i_3 += 1;
                                }
                                j_1 += 1;
                            }
                        }
                        if (*h).param.rc.i_aq_mode != 0 {
                            (*frame).f_qp_offset = prealloc_size as *mut ::core::ffi::c_float;
                            let c2rust_fresh31 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh31 as usize] =
                                &raw mut (*frame).f_qp_offset as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += ((i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            (*frame).f_qp_offset_aq = prealloc_size as *mut ::core::ffi::c_float;
                            let c2rust_fresh32 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[c2rust_fresh32 as usize] = &raw mut (*frame).f_qp_offset_aq
                                as *mut *mut crate::stdlib::uint8_t;
                            prealloc_size += ((i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>())
                                as crate::stdlib::int64_t
                                + (64i32 - 1i32) as crate::stdlib::int64_t)
                                & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            if (*h).frames.have_lowres {
                                (*frame).i_inv_qscale_factor =
                                    prealloc_size as *mut crate::stdlib::uint16_t;
                                let c2rust_fresh33 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[c2rust_fresh33 as usize] = &raw mut (*frame)
                                    .i_inv_qscale_factor
                                    as *mut *mut crate::stdlib::uint8_t;
                                prealloc_size += ((i_mb_count as usize)
                                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint16_t>())
                                    as crate::stdlib::int64_t
                                    + (64i32 - 1i32) as crate::stdlib::int64_t)
                                    & !(64i32 - 1i32) as crate::stdlib::int64_t;
                            }
                        }
                        if (*h).frames.have_lowres {
                            prealloc_size += crate::osdep_h::NATIVE_ALIGN as crate::stdlib::int64_t;
                        }
                    }
                    (*frame).base = crate::src::common::base::x264_malloc(prealloc_size)
                        as *mut crate::stdlib::uint8_t;
                    if !(*frame).base.is_null() {
                        let mut p_0 = 0i32;
                        loop {
                            let c2rust_fresh34 = prealloc_idx;
                            prealloc_idx = prealloc_idx - 1;
                            if !(c2rust_fresh34 != 0) {
                                break;
                            }
                            *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize]
                                as crate::stdlib::intptr_t
                                + (*frame).base as crate::stdlib::intptr_t)
                                as *mut crate::stdlib::uint8_t;
                        }
                        if i_csp == crate::x264_h::X264_CSP_NV12
                            || i_csp == crate::x264_h::X264_CSP_NV16
                        {
                            let mut chroma_padv_0 = i_padv
                                >> (i_csp == crate::x264_h::X264_CSP_NV12) as ::core::ffi::c_int;
                            (*frame).plane[1usize] = (*frame).buffer[1usize]
                                .offset(((*frame).i_stride[1usize] * chroma_padv_0) as isize)
                                .offset(
                                    (if 32i32
                                        > 64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    {
                                        32i32
                                    } else {
                                        64i32
                                            / ::core::mem::size_of::<
                                                crate::src::common::common::pixel,
                                            >()
                                                as ::core::ffi::c_int
                                    }) as isize,
                                );
                            if (*h).param.interlaced {
                                (*frame).plane_fld[1usize] = (*frame).buffer_fld[1usize]
                                    .offset(((*frame).i_stride[1usize] * chroma_padv_0) as isize)
                                    .offset(
                                        (if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize,
                                    );
                            }
                        }
                        while p_0 < luma_plane_count {
                            let mut luma_plane_size_1 = align_plane_size(
                                (*frame).i_stride[p_0 as usize]
                                    * ((*frame).i_lines[p_0 as usize] + 2i32 * i_padv),
                                disalign,
                            )
                                as crate::stdlib::int64_t;
                            if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                                let mut i_4 = 0i32;
                                while i_4 < 4i32 {
                                    (*frame).filtered[p_0 as usize][i_4 as usize] = (*frame).buffer
                                        [p_0 as usize]
                                        .offset(
                                            (i_4 as crate::stdlib::int64_t * luma_plane_size_1)
                                                as isize,
                                        )
                                        .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                        .offset(
                                            (if 32i32
                                                > 64i32
                                                    / ::core::mem::size_of::<
                                                        crate::src::common::common::pixel,
                                                    >(
                                                    )
                                                        as ::core::ffi::c_int
                                            {
                                                32i32
                                            } else {
                                                64i32
                                                    / ::core::mem::size_of::<
                                                        crate::src::common::common::pixel,
                                                    >(
                                                    )
                                                        as ::core::ffi::c_int
                                            }) as isize,
                                        );
                                    if (*h).param.interlaced {
                                        (*frame).filtered_fld[p_0 as usize][i_4 as usize] =
                                            (*frame).buffer_fld[p_0 as usize]
                                                .offset(
                                                    (i_4 as crate::stdlib::int64_t
                                                        * luma_plane_size_1)
                                                        as isize,
                                                )
                                                .offset(
                                                    ((*frame).i_stride[p_0 as usize] * i_padv)
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 32i32
                                                        > 64i32
                                                            / ::core::mem::size_of::<
                                                                crate::src::common::common::pixel,
                                                            >(
                                                            )
                                                                as ::core::ffi::c_int
                                                    {
                                                        32i32
                                                    } else {
                                                        64i32
                                                            / ::core::mem::size_of::<
                                                                crate::src::common::common::pixel,
                                                            >(
                                                            )
                                                                as ::core::ffi::c_int
                                                    })
                                                        as isize,
                                                );
                                    }
                                    i_4 += 1;
                                }
                                (*frame).plane[p_0 as usize] =
                                    (*frame).filtered[p_0 as usize][0usize];
                                (*frame).plane_fld[p_0 as usize] =
                                    (*frame).filtered_fld[p_0 as usize][0usize];
                            } else {
                                (*frame).plane[p_0 as usize] = (*frame).buffer[p_0 as usize]
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize,
                                    );
                                (*frame).filtered[p_0 as usize][0usize] =
                                    (*frame).plane[p_0 as usize];
                                if (*h).param.interlaced {
                                    (*frame).plane_fld[p_0 as usize] = (*frame).buffer_fld
                                        [p_0 as usize]
                                        .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                        .offset(
                                            (if 32i32
                                                > 64i32
                                                    / ::core::mem::size_of::<
                                                        crate::src::common::common::pixel,
                                                    >(
                                                    )
                                                        as ::core::ffi::c_int
                                            {
                                                32i32
                                            } else {
                                                64i32
                                                    / ::core::mem::size_of::<
                                                        crate::src::common::common::pixel,
                                                    >(
                                                    )
                                                        as ::core::ffi::c_int
                                            }) as isize,
                                        );
                                    (*frame).filtered_fld[p_0 as usize][0usize] =
                                        (*frame).plane_fld[p_0 as usize];
                                }
                            }
                            p_0 += 1;
                        }
                        if b_fdec != 0 {
                            (*(&raw mut *(*frame).mv16x16.offset(0isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0u32;
                            (*frame).mv16x16 = (*frame).mv16x16.offset(1);
                            if (*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA {
                                (*frame).integral = ((*frame).buffer[3usize]
                                    as *mut crate::stdlib::uint16_t)
                                    .offset(((*frame).i_stride[0usize] * i_padv) as isize)
                                    .offset(
                                        (if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize,
                                    );
                            }
                        } else if (*h).frames.have_lowres {
                            let mut i_5 = 0i32;
                            let mut j_2 = 0i32;
                            let mut luma_plane_size_2 = align_plane_size(
                                (*frame).i_stride_lowres
                                    * ((*frame).i_lines[0usize] / 2i32
                                        + 2i32 * crate::src::common::frame::PADV),
                                disalign,
                            )
                                as crate::stdlib::int64_t;
                            while i_5 < 4i32 {
                                (*frame).lowres[i_5 as usize] = (*frame)
                                    .buffer_lowres
                                    .offset(
                                        ((*frame).i_stride_lowres * crate::src::common::frame::PADV)
                                            as isize,
                                    )
                                    .offset(
                                        (if 32i32
                                            > 64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        {
                                            32i32
                                        } else {
                                            64i32
                                                / ::core::mem::size_of::<
                                                    crate::src::common::common::pixel,
                                                >(
                                                )
                                                    as ::core::ffi::c_int
                                        }) as isize,
                                    )
                                    .offset(
                                        (i_5 as crate::stdlib::int64_t * luma_plane_size_2)
                                            as isize,
                                    );
                                i_5 += 1;
                            }
                            while j_2 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
                                let mut i_6 = 0i32;
                                while i_6 <= (*h).param.i_bframe {
                                    crate::stdlib::memset(
                                        (*frame).lowres_mvs[j_2 as usize][i_6 as usize]
                                            as *mut ::core::ffi::c_void,
                                        0i32,
                                        ((2i32 * i_mb_count) as crate::__stddef_size_t_h::size_t)
                                            .wrapping_mul(::core::mem::size_of::<
                                                crate::stdlib::int16_t,
                                            >(
                                            )),
                                    );
                                    i_6 += 1;
                                }
                                j_2 += 1;
                            }
                            (*frame).i_intra_cost = (*frame).lowres_costs[0usize][0usize];
                            crate::stdlib::memset(
                                (*frame).i_intra_cost as *mut ::core::ffi::c_void,
                                -(1i32),
                                (i_mb_count as crate::__stddef_size_t_h::size_t).wrapping_mul(
                                    ::core::mem::size_of::<crate::stdlib::uint16_t>(),
                                ),
                            );
                            if (*h).param.rc.i_aq_mode != 0 {
                                crate::stdlib::memset(
                                    (*frame).i_inv_qscale_factor as *mut ::core::ffi::c_void,
                                    0i32,
                                    (i_mb_count as crate::__stddef_size_t_h::size_t).wrapping_mul(
                                        ::core::mem::size_of::<crate::stdlib::uint16_t>(),
                                    ),
                                );
                            }
                        }
                        if !(crate::stdlib::pthread_mutex_init(
                            &raw mut (*frame).mutex,
                            ::core::ptr::null::<crate::stdlib::pthread_mutexattr_t>(),
                        ) != 0)
                            && !(crate::stdlib::pthread_cond_init(
                                &raw mut (*frame).cv,
                                ::core::ptr::null::<crate::stdlib::pthread_condattr_t>(),
                            ) != 0)
                        {
                            return frame;
                        }
                    }
                }
            }
        }
        crate::src::common::base::x264_free(frame as *mut ::core::ffi::c_void);
        ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>()
    }
}
pub unsafe extern "C" fn x264_8_frame_delete(
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        if !(*frame).duplicate {
            crate::src::common::base::x264_free((*frame).base as *mut ::core::ffi::c_void);
            if !(*frame).param.is_null() && (*(*frame).param).param_free.is_some() {
                crate::src::common::base::x264_param_cleanup((*frame).param);
                (*(*frame).param)
                    .param_free
                    .expect("non-null function pointer")(
                    (*frame).param as *mut ::core::ffi::c_void,
                );
            }
            if (*frame).mb_info_free.is_some() {
                (*frame).mb_info_free.expect("non-null function pointer")(
                    (*frame).mb_info as *mut ::core::ffi::c_void,
                );
            }
            if (*frame).extra_sei.sei_free.is_some() {
                let mut i = 0i32;
                while i < (*frame).extra_sei.num_payloads {
                    (*frame)
                        .extra_sei
                        .sei_free
                        .expect("non-null function pointer")(
                        (*(*frame).extra_sei.payloads.offset(i as isize)).payload
                            as *mut ::core::ffi::c_void,
                    );
                    i += 1;
                }
                (*frame)
                    .extra_sei
                    .sei_free
                    .expect("non-null function pointer")(
                    (*frame).extra_sei.payloads as *mut ::core::ffi::c_void,
                );
            }
            crate::stdlib::pthread_mutex_destroy(&raw mut (*frame).mutex);
            crate::stdlib::pthread_cond_destroy(&raw mut (*frame).cv);
        }
        crate::src::common::base::x264_free(frame as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn get_plane_ptr(
    mut h: *mut crate::src::common::common::x264_t,
    mut src: *mut crate::x264_h::x264_picture_t,
    mut pix: *mut *mut crate::stdlib::uint8_t,
    mut stride: *mut ::core::ffi::c_int,
    mut plane: ::core::ffi::c_int,
    mut xshift: ::core::ffi::c_int,
    mut yshift: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut width = (*h).param.i_width >> xshift;
        let mut height = (*h).param.i_height >> yshift;
        *pix = (*src).img.plane[plane as usize];
        *stride = (*src).img.i_stride[plane as usize];
        if (*src).img.i_csp & crate::x264_h::X264_CSP_VFLIP != 0 {
            *pix = (*pix).offset(((height - 1i32) * *stride) as isize);
            *stride = -*stride;
        }
        if width > crate::stdlib::abs(*stride) {
            log::error!(
                "Input picture width ({width}) is greater than stride ({})",
                *stride
            );
            return -(1i32);
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_8_frame_copy_picture(
    mut h: *mut crate::src::common::common::x264_t,
    mut dst: *mut crate::src::common::frame::x264_frame_t,
    mut src: *mut crate::x264_h::x264_picture_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pix = [::core::ptr::null_mut::<crate::stdlib::uint8_t>(); 3];
        let mut stride = [0; 3];
        let mut i_csp = (*src).img.i_csp & crate::x264_h::X264_CSP_MASK;
        if (*dst).i_csp != frame_internal_csp(i_csp) {
            log::error!("Invalid input colorspace");
            return -(1i32);
        }
        if (*src).img.i_csp & crate::x264_h::X264_CSP_HIGH_DEPTH != 0 {
            log::error!(
                "This build of x264 requires 8-bit input. Rebuild to support high depth input."
            );
            return -(1i32);
        }
        if crate::internal::BIT_DEPTH != 10i32 && i_csp == crate::x264_h::X264_CSP_V210 {
            log::error!("v210 input is only compatible with bit-depth of 10 bits");
            return -(1i32);
        }
        if (*src).i_type < crate::x264_h::X264_TYPE_AUTO
            || (*src).i_type > crate::x264_h::X264_TYPE_KEYFRAME
        {
            log::warn!(
                "forced frame type ({}) at {} is unknown",
                (*src).i_type,
                (*h).frames.i_input
            );
            (*dst).i_forced_type = crate::x264_h::X264_TYPE_AUTO;
        } else {
            (*dst).i_forced_type = (*src).i_type;
        }
        (*dst).i_type = (*dst).i_forced_type;
        (*dst).i_qpplus1 = (*src).i_qpplus1;
        (*dst).i_reordered_pts = (*src).i_pts;
        (*dst).i_pts = (*dst).i_reordered_pts;
        (*dst).param = (*src).param;
        (*dst).i_pic_struct = (*src).i_pic_struct;
        (*dst).extra_sei = (*src).extra_sei;
        (*dst).opaque = (*src).opaque;
        (*dst).mb_info = if (*h).param.analyse.mb_info {
            (*src).prop.mb_info
        } else {
            ::core::ptr::null_mut::<crate::stdlib::uint8_t>()
        };
        (*dst).mb_info_free = if (*h).param.analyse.mb_info {
            (*src).prop.mb_info_free
        } else {
            None
        };
        if i_csp == crate::x264_h::X264_CSP_YUYV || i_csp == crate::x264_h::X264_CSP_UYVY {
            let mut p = (i_csp == crate::x264_h::X264_CSP_UYVY) as ::core::ffi::c_int;
            (*h).mc
                .plane_copy_deinterleave_yuyv
                .expect("non-null function pointer")(
                (*dst).plane[p as usize],
                (*dst).i_stride[p as usize] as crate::stdlib::intptr_t,
                (*dst).plane[(p ^ 1i32) as usize],
                (*dst).i_stride[(p ^ 1i32) as usize] as crate::stdlib::intptr_t,
                (*src).img.plane[0usize],
                ((*src).img.i_stride[0usize] / crate::src::common::common::SIZEOF_PIXEL)
                    as crate::stdlib::intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
        } else if i_csp == crate::x264_h::X264_CSP_V210 {
            stride[0usize] = (*src).img.i_stride[0usize];
            pix[0usize] = (*src).img.plane[0usize];
            (*h).mc
                .plane_copy_deinterleave_v210
                .expect("non-null function pointer")(
                (*dst).plane[0usize],
                (*dst).i_stride[0usize] as crate::stdlib::intptr_t,
                (*dst).plane[1usize],
                (*dst).i_stride[1usize] as crate::stdlib::intptr_t,
                pix[0usize] as *mut crate::stdlib::uint32_t,
                (stride[0usize]
                    / ::core::mem::size_of::<crate::stdlib::uint32_t>() as ::core::ffi::c_int)
                    as crate::stdlib::intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
        } else if i_csp >= crate::x264_h::X264_CSP_BGR {
            stride[0usize] = (*src).img.i_stride[0usize];
            pix[0usize] = (*src).img.plane[0usize];
            if (*src).img.i_csp & crate::x264_h::X264_CSP_VFLIP != 0 {
                pix[0usize] =
                    pix[0usize].offset((((*h).param.i_height - 1i32) * stride[0usize]) as isize);
                stride[0usize] = -stride[0usize];
            }
            let mut b = (i_csp == crate::x264_h::X264_CSP_RGB) as ::core::ffi::c_int;
            (*h).mc
                .plane_copy_deinterleave_rgb
                .expect("non-null function pointer")(
                (*dst).plane[(1i32 + b) as usize],
                (*dst).i_stride[(1i32 + b) as usize] as crate::stdlib::intptr_t,
                (*dst).plane[0usize],
                (*dst).i_stride[0usize] as crate::stdlib::intptr_t,
                (*dst).plane[(2i32 - b) as usize],
                (*dst).i_stride[(2i32 - b) as usize] as crate::stdlib::intptr_t,
                pix[0usize],
                (stride[0usize] / crate::src::common::common::SIZEOF_PIXEL)
                    as crate::stdlib::intptr_t,
                if i_csp == crate::x264_h::X264_CSP_BGRA {
                    4i32
                } else {
                    3i32
                },
                (*h).param.i_width,
                (*h).param.i_height,
            );
        } else {
            let mut v_shift = (*h).sps.i_chroma_format_idc.is_420() as ::core::ffi::c_int;
            if get_plane_ptr(
                h,
                src,
                (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(0isize),
                (&raw mut stride as *mut ::core::ffi::c_int).offset(0isize),
                0i32,
                0i32,
                0i32,
            ) < 0i32
            {
                return -(1i32);
            }
            (*h).mc.plane_copy.expect("non-null function pointer")(
                (*dst).plane[0usize],
                (*dst).i_stride[0usize] as crate::stdlib::intptr_t,
                pix[0usize],
                (stride[0usize] / crate::src::common::common::SIZEOF_PIXEL)
                    as crate::stdlib::intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            if i_csp == crate::x264_h::X264_CSP_NV12 || i_csp == crate::x264_h::X264_CSP_NV16 {
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(1isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(1isize),
                    1i32,
                    0i32,
                    v_shift,
                ) < 0i32
                {
                    return -(1i32);
                }
                (*h).mc.plane_copy.expect("non-null function pointer")(
                    (*dst).plane[1usize],
                    (*dst).i_stride[1usize] as crate::stdlib::intptr_t,
                    pix[1usize],
                    (stride[1usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width,
                    (*h).param.i_height >> v_shift,
                );
            } else if i_csp == crate::x264_h::X264_CSP_NV21 {
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(1isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(1isize),
                    1i32,
                    0i32,
                    v_shift,
                ) < 0i32
                {
                    return -(1i32);
                }
                (*h).mc.plane_copy_swap.expect("non-null function pointer")(
                    (*dst).plane[1usize],
                    (*dst).i_stride[1usize] as crate::stdlib::intptr_t,
                    pix[1usize],
                    (stride[1usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width >> 1i32,
                    (*h).param.i_height >> v_shift,
                );
            } else if i_csp == crate::x264_h::X264_CSP_I420
                || i_csp == crate::x264_h::X264_CSP_I422
                || i_csp == crate::x264_h::X264_CSP_YV12
                || i_csp == crate::x264_h::X264_CSP_YV16
            {
                let mut uv_swap = (i_csp == crate::x264_h::X264_CSP_YV12
                    || i_csp == crate::x264_h::X264_CSP_YV16)
                    as ::core::ffi::c_int;
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(1isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(1isize),
                    if uv_swap != 0 { 2i32 } else { 1i32 },
                    1i32,
                    v_shift,
                ) < 0i32
                {
                    return -(1i32);
                }
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(2isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(2isize),
                    if uv_swap != 0 { 1i32 } else { 2i32 },
                    1i32,
                    v_shift,
                ) < 0i32
                {
                    return -(1i32);
                }
                (*h).mc
                    .plane_copy_interleave
                    .expect("non-null function pointer")(
                    (*dst).plane[1usize],
                    (*dst).i_stride[1usize] as crate::stdlib::intptr_t,
                    pix[1usize],
                    (stride[1usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    pix[2usize],
                    (stride[2usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width >> 1i32,
                    (*h).param.i_height >> v_shift,
                );
            } else if i_csp == crate::x264_h::X264_CSP_I444 || i_csp == crate::x264_h::X264_CSP_YV24
            {
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(1isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(1isize),
                    if i_csp == 0xci32 { 1i32 } else { 2i32 },
                    0i32,
                    0i32,
                ) < 0i32
                {
                    return -(1i32);
                }
                if get_plane_ptr(
                    h,
                    src,
                    (&raw mut pix as *mut *mut crate::stdlib::uint8_t).offset(2isize),
                    (&raw mut stride as *mut ::core::ffi::c_int).offset(2isize),
                    if i_csp == 0xci32 { 2i32 } else { 1i32 },
                    0i32,
                    0i32,
                ) < 0i32
                {
                    return -(1i32);
                }
                (*h).mc.plane_copy.expect("non-null function pointer")(
                    (*dst).plane[1usize],
                    (*dst).i_stride[1usize] as crate::stdlib::intptr_t,
                    pix[1usize],
                    (stride[1usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width,
                    (*h).param.i_height,
                );
                (*h).mc.plane_copy.expect("non-null function pointer")(
                    (*dst).plane[2usize],
                    (*dst).i_stride[2usize] as crate::stdlib::intptr_t,
                    pix[2usize],
                    (stride[2usize] / crate::src::common::common::SIZEOF_PIXEL)
                        as crate::stdlib::intptr_t,
                    (*h).param.i_width,
                    (*h).param.i_height,
                );
            }
        }
        0i32
    }
}
#[inline(always)]
unsafe extern "C" fn pixel_memset(
    mut dst: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
    mut len: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        let mut dstp = dst;
        let mut v1 = *src as crate::stdlib::uint32_t;
        let mut v2 = if size == 1i32 {
            v1.wrapping_add(v1 << 8i32)
        } else {
            (*(src as *mut crate::src::common::base::x264_union16_t)).i as crate::stdlib::uint32_t
        };
        let mut v4 = if size <= 2i32 {
            v2.wrapping_add(v2 << 16i32)
        } else {
            (*(src as *mut crate::src::common::base::x264_union32_t)).i
        };
        len *= size;
        if dstp as crate::stdlib::intptr_t
            & (crate::osdep_h::WORD_SIZE - 1i32) as crate::stdlib::intptr_t
            != 0
        {
            if size <= 2i32 && dstp as crate::stdlib::intptr_t & 3isize != 0 {
                if size == 1i32 && dstp as crate::stdlib::intptr_t & 1isize != 0 {
                    let c2rust_fresh0 = i;
                    i = i + 1;
                    *dstp.offset(c2rust_fresh0 as isize) = v1 as crate::stdlib::uint8_t;
                }
                if dstp as crate::stdlib::intptr_t & 2isize != 0 {
                    (*(dstp.offset(i as isize) as *mut crate::src::common::base::x264_union16_t))
                        .i = v2 as crate::stdlib::uint16_t;
                    i += 2i32;
                }
            }
            if crate::osdep_h::WORD_SIZE == 8i32 && dstp as crate::stdlib::intptr_t & 4isize != 0 {
                (*(dstp.offset(i as isize) as *mut crate::src::common::base::x264_union32_t)).i =
                    v4;
                i += 4i32;
            }
        }
        if crate::osdep_h::WORD_SIZE == 8i32 {
            let mut v8 = (v4 as crate::stdlib::uint64_t)
                .wrapping_add((v4 as crate::stdlib::uint64_t) << 32i32);
            while i < len - 7i32 {
                (*(dstp.offset(i as isize) as *mut crate::src::common::base::x264_union64_t)).i =
                    v8;
                i += 8i32;
            }
        }
        while i < len - 3i32 {
            (*(dstp.offset(i as isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
            i += 4i32;
        }
        if size <= 2i32 {
            if i < len - 1i32 {
                (*(dstp.offset(i as isize) as *mut crate::src::common::base::x264_union16_t)).i =
                    v2 as crate::stdlib::uint16_t;
                i += 2i32;
            }
            if size == 1i32 && i != len {
                *dstp.offset(i as isize) = v1 as crate::stdlib::uint8_t;
            }
        }
    }
}
#[inline(always)]
unsafe extern "C" fn plane_expand_border(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut i_padh: ::core::ffi::c_int,
    mut i_padv: ::core::ffi::c_int,
    mut b_pad_top: ::core::ffi::c_int,
    mut b_pad_bottom: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < i_height {
            pixel_memset(
                pix.offset(-i_padh as isize).offset((y * i_stride) as isize),
                pix.offset(0isize).offset((y * i_stride) as isize),
                i_padh >> b_chroma,
                crate::src::common::common::SIZEOF_PIXEL << b_chroma,
            );
            pixel_memset(
                pix.offset(i_width as isize).offset((y * i_stride) as isize),
                pix.offset((i_width - 1i32 - b_chroma) as isize)
                    .offset((y * i_stride) as isize),
                i_padh >> b_chroma,
                crate::src::common::common::SIZEOF_PIXEL << b_chroma,
            );
            y += 1;
        }
        if b_pad_top != 0 {
            let mut y_0 = 0i32;
            while y_0 < i_padv {
                crate::stdlib::memcpy(
                    pix.offset(-i_padh as isize)
                        .offset(((-y_0 - 1i32) * i_stride) as isize)
                        as *mut ::core::ffi::c_void,
                    pix.offset(-i_padh as isize)
                        .offset((0i32 * i_stride) as isize)
                        as *const ::core::ffi::c_void,
                    ((i_width + 2i32 * i_padh) * crate::src::common::common::SIZEOF_PIXEL)
                        as crate::__stddef_size_t_h::size_t,
                );
                y_0 += 1;
            }
        }
        if b_pad_bottom != 0 {
            let mut y_1 = 0i32;
            while y_1 < i_padv {
                crate::stdlib::memcpy(
                    pix.offset(-i_padh as isize)
                        .offset(((i_height + y_1) * i_stride) as isize)
                        as *mut ::core::ffi::c_void,
                    pix.offset(-i_padh as isize)
                        .offset(((i_height - 1i32) * i_stride) as isize)
                        as *const ::core::ffi::c_void,
                    ((i_width + 2i32 * i_padh) * crate::src::common::common::SIZEOF_PIXEL)
                        as crate::__stddef_size_t_h::size_t,
                );
                y_1 += 1;
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_expand_border(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        let mut pad_top = (mb_y == 0i32) as ::core::ffi::c_int;
        let mut pad_bot = (mb_y
            == (*h).mb.i_mb_height - ((1i32) << (*h).sh.mbaff as ::core::ffi::c_int))
            as ::core::ffi::c_int;
        let mut b_start = (mb_y == (*h).i_threadslice_start) as ::core::ffi::c_int;
        let mut b_end = (mb_y
            == (*h).i_threadslice_end - ((1i32) << (*h).sh.mbaff as ::core::ffi::c_int))
            as ::core::ffi::c_int;
        if mb_y & (*h).sh.mbaff as ::core::ffi::c_int != 0 {
            return;
        }
        while i < (*frame).i_plane {
            let mut pix = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
            let mut h_shift = (i != 0
                && ((*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422()))
                as ::core::ffi::c_int;
            let mut v_shift =
                (i != 0 && (*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
            let mut stride = (*frame).i_stride[i as usize];
            let mut width = 16i32 * (*h).mb.i_mb_width;
            let mut height = (if pad_bot != 0 {
                (16i32 * ((*h).mb.i_mb_height - mb_y)) >> (*h).sh.mbaff as ::core::ffi::c_int
            } else {
                16i32
            }) >> v_shift;
            let mut padh = crate::src::common::frame::PADH;
            let mut padv = crate::src::common::frame::PADV >> v_shift;
            if b_end != 0 && b_start == 0 {
                height += 4i32 >> (v_shift + (*h).sh.mbaff as ::core::ffi::c_int);
            }
            let mut starty = 16i32 * mb_y - 4i32 * (b_start == 0) as ::core::ffi::c_int;
            if (*h).sh.mbaff {
                pix =
                    (*frame).plane_fld[i as usize].offset(((starty * stride) >> v_shift) as isize);
                plane_expand_border(
                    pix,
                    stride * 2i32,
                    width,
                    height,
                    padh,
                    padv,
                    pad_top,
                    pad_bot,
                    h_shift,
                );
                plane_expand_border(
                    pix.offset(stride as isize),
                    stride * 2i32,
                    width,
                    height,
                    padh,
                    padv,
                    pad_top,
                    pad_bot,
                    h_shift,
                );
                height = (if pad_bot != 0 {
                    16i32 * ((*h).mb.i_mb_height - mb_y)
                } else {
                    32i32
                }) >> v_shift;
                if b_end != 0 && b_start == 0 {
                    height += 4i32 >> v_shift;
                }
                pix = (*frame).plane[i as usize].offset(((starty * stride) >> v_shift) as isize);
                plane_expand_border(
                    pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
                );
            } else {
                pix = (*frame).plane[i as usize].offset(((starty * stride) >> v_shift) as isize);
                plane_expand_border(
                    pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
                );
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_expand_border_filtered(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
    mut b_end: ::core::ffi::c_int,
) {
    unsafe {
        let mut p = 0i32;
        let mut b_start = (mb_y == 0) as ::core::ffi::c_int;
        let mut width = 16i32 * (*h).mb.i_mb_width + 8i32;
        let mut height = if b_end != 0 {
            ((16i32 * ((*h).mb.i_mb_height - mb_y)) >> (*h).sh.mbaff as ::core::ffi::c_int) + 16i32
        } else {
            16i32
        };
        let mut padh = crate::src::common::frame::PADH - 4i32;
        let mut padv = crate::src::common::frame::PADV - 8i32;
        while p
            < (if (*h).sps.i_chroma_format_idc.is_444() {
                3i32
            } else {
                1i32
            })
        {
            let mut i = 1i32;
            while i < 4i32 {
                let mut pix = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
                let mut stride = (*frame).i_stride[p as usize];
                if (*h).sh.mbaff {
                    pix = (*frame).filtered_fld[p as usize][i as usize]
                        .offset(((16i32 * mb_y - 16i32) * stride) as isize)
                        .offset(-(4isize));
                    plane_expand_border(
                        pix,
                        stride * 2i32,
                        width,
                        height,
                        padh,
                        padv,
                        b_start,
                        b_end,
                        0i32,
                    );
                    plane_expand_border(
                        pix.offset(stride as isize),
                        stride * 2i32,
                        width,
                        height,
                        padh,
                        padv,
                        b_start,
                        b_end,
                        0i32,
                    );
                }
                pix = (*frame).filtered[p as usize][i as usize]
                    .offset(((16i32 * mb_y - 8i32) * stride) as isize)
                    .offset(-(4isize));
                plane_expand_border(
                    pix,
                    stride,
                    width,
                    height << (*h).sh.mbaff as ::core::ffi::c_int,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0i32,
                );
                i += 1;
            }
            p += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_expand_border_lowres(
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut i = 0i32;
        while i < 4i32 {
            plane_expand_border(
                (*frame).lowres[i as usize],
                (*frame).i_stride_lowres,
                (*frame).i_width_lowres,
                (*frame).i_lines_lowres,
                crate::src::common::frame::PADH,
                crate::src::common::frame::PADV,
                1i32,
                1i32,
                0i32,
            );
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_expand_border_chroma(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut plane: ::core::ffi::c_int,
) {
    unsafe {
        let mut v_shift = (*h).sps.i_chroma_format_idc.is_420() as ::core::ffi::c_int;
        plane_expand_border(
            (*frame).plane[plane as usize],
            (*frame).i_stride[plane as usize],
            16i32 * (*h).mb.i_mb_width,
            (16i32 * (*h).mb.i_mb_height) >> v_shift,
            crate::src::common::frame::PADH,
            crate::src::common::frame::PADV >> v_shift,
            1i32,
            1i32,
            ((*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422())
                as ::core::ffi::c_int,
        );
    }
}
pub unsafe extern "C" fn x264_8_frame_expand_border_mod16(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut i = 0i32;
        while i < (*frame).i_plane {
            let mut i_width = (*h).param.i_width;
            let mut h_shift = (i != 0
                && ((*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422()))
                as ::core::ffi::c_int;
            let mut v_shift =
                (i != 0 && (*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
            let mut i_height = (*h).param.i_height >> v_shift;
            let mut i_padx = (*h).mb.i_mb_width * 16i32 - (*h).param.i_width;
            let mut i_pady = ((*h).mb.i_mb_height * 16i32 - (*h).param.i_height) >> v_shift;
            if i_padx != 0 {
                let mut y = 0i32;
                while y < i_height {
                    pixel_memset(
                        (*(&raw mut (*frame).plane as *mut *mut crate::src::common::common::pixel)
                            .offset(i as isize))
                        .offset(
                            (y * *(&raw mut (*frame).i_stride as *mut ::core::ffi::c_int)
                                .offset(i as isize)
                                + i_width) as isize,
                        ),
                        (*(&raw mut (*frame).plane as *mut *mut crate::src::common::common::pixel)
                            .offset(i as isize))
                        .offset(
                            (y * *(&raw mut (*frame).i_stride as *mut ::core::ffi::c_int)
                                .offset(i as isize)
                                + i_width
                                - 1i32
                                - h_shift) as isize,
                        ),
                        i_padx >> h_shift,
                        crate::src::common::common::SIZEOF_PIXEL << h_shift,
                    );
                    y += 1;
                }
            }
            if i_pady != 0 {
                let mut y_0 = i_height;
                while y_0 < i_height + i_pady {
                    crate::stdlib::memcpy(
                        (*(&raw mut (*frame).plane as *mut *mut crate::src::common::common::pixel)
                            .offset(i as isize))
                        .offset(
                            (y_0 * *(&raw mut (*frame).i_stride as *mut ::core::ffi::c_int)
                                .offset(i as isize)) as isize,
                        ) as *mut ::core::ffi::c_void,
                        (*(&raw mut (*frame).plane as *mut *mut crate::src::common::common::pixel)
                            .offset(i as isize))
                        .offset(
                            ((i_height
                                - (!y_0 & (*h).param.interlaced as ::core::ffi::c_int)
                                - 1i32)
                                * *(&raw mut (*frame).i_stride as *mut ::core::ffi::c_int)
                                    .offset(i as isize)) as isize,
                        ) as *const ::core::ffi::c_void,
                        ((i_width + i_padx) * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    y_0 += 1;
                }
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_expand_border_mbpair(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut _mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        while i < (*(*h).fenc).i_plane {
            let mut v_shift =
                (i != 0 && (*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
            let mut stride = (*(*h).fenc).i_stride[i as usize];
            let mut height = (*h).param.i_height >> v_shift;
            let mut pady = ((*h).mb.i_mb_height * 16i32 - (*h).param.i_height) >> v_shift;
            let mut fenc = (*(*h).fenc).plane[i as usize].offset((16i32 * mb_x) as isize);
            let mut y = height;
            while y < height + pady {
                crate::stdlib::memcpy(
                    fenc.offset((y * stride) as isize) as *mut ::core::ffi::c_void,
                    fenc.offset(((height - 1i32) * stride) as isize) as *const ::core::ffi::c_void,
                    (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                        as crate::__stddef_size_t_h::size_t,
                );
                y += 1;
            }
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_cond_broadcast(
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut i_lines_completed: ::core::ffi::c_int,
) {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*frame).mutex);
        (*frame).i_lines_completed = i_lines_completed;
        crate::stdlib::pthread_cond_broadcast(&raw mut (*frame).cv);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*frame).mutex);
    }
}
pub unsafe extern "C" fn x264_8_frame_cond_wait(
    mut frame: *mut crate::src::common::frame::x264_frame_t,
    mut i_lines_completed: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut completed = 0;
        crate::stdlib::pthread_mutex_lock(&raw mut (*frame).mutex);
        loop {
            completed = (*frame).i_lines_completed;
            if !(completed < i_lines_completed && i_lines_completed >= 0i32) {
                break;
            }
            crate::stdlib::pthread_cond_wait(&raw mut (*frame).cv, &raw mut (*frame).mutex);
        }
        crate::stdlib::pthread_mutex_unlock(&raw mut (*frame).mutex);
        completed
    }
}
pub unsafe extern "C" fn x264_8_threadslice_cond_broadcast(
    mut h: *mut crate::src::common::common::x264_t,
    mut pass: ::core::ffi::c_int,
) {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*h).mutex);
        (*h).i_threadslice_pass = pass;
        if pass > 0i32 {
            crate::stdlib::pthread_cond_broadcast(&raw mut (*h).cv);
        }
        crate::stdlib::pthread_mutex_unlock(&raw mut (*h).mutex);
    }
}
pub unsafe extern "C" fn x264_8_threadslice_cond_wait(
    mut h: *mut crate::src::common::common::x264_t,
    mut pass: ::core::ffi::c_int,
) {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*h).mutex);
        while (*h).i_threadslice_pass < pass {
            crate::stdlib::pthread_cond_wait(&raw mut (*h).cv, &raw mut (*h).mutex);
        }
        crate::stdlib::pthread_mutex_unlock(&raw mut (*h).mutex);
    }
}
pub unsafe extern "C" fn x264_8_frame_new_slice(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*h).param.i_slice_count_max != 0 {
            let mut slice_count = 0;
            if (*h).param.sliced_threads {
                slice_count = x264_pthread_fetch_and_add(
                    &raw mut (*frame).i_slice_count,
                    1i32,
                    &raw mut (*frame).mutex,
                );
            } else {
                let c2rust_fresh1 = (*frame).i_slice_count;
                (*frame).i_slice_count = (*frame).i_slice_count + 1;
                slice_count = c2rust_fresh1;
            }
            if slice_count >= (*h).param.i_slice_count_max {
                return -(1i32);
            }
        }
        0i32
    }
}
pub unsafe extern "C" fn x264_8_frame_push(
    mut list: *mut *mut crate::src::common::frame::x264_frame_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut i = 0i32;
        while !(*list.offset(i as isize)).is_null() {
            i += 1;
        }
        let ref mut c2rust_fresh2 = *list.offset(i as isize);
        *c2rust_fresh2 = frame;
    }
}
pub unsafe extern "C" fn x264_8_frame_pop(
    mut list: *mut *mut crate::src::common::frame::x264_frame_t,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        let mut i = 0i32;
        '_c2rust_label: {
            if !(*list.offset(0isize)).is_null() {
            } else {
                crate::stdlib::__assert_fail(
                    b"list[0]\0".as_ptr() as *const ::core::ffi::c_char,
                    b"common/frame.c\0".as_ptr() as *const ::core::ffi::c_char,
                    746u32,
                    b"x264_frame_t *x264_8_frame_pop(x264_frame_t **)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        while !(*list.offset((i + 1i32) as isize)).is_null() {
            i += 1;
        }
        let mut frame = *list.offset(i as isize);
        let ref mut c2rust_fresh3 = *list.offset(i as isize);
        *c2rust_fresh3 = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        frame
    }
}
pub unsafe extern "C" fn x264_8_frame_unshift(
    mut list: *mut *mut crate::src::common::frame::x264_frame_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut i = 0i32;
        while !(*list.offset(i as isize)).is_null() {
            i += 1;
        }
        loop {
            let c2rust_fresh4 = i;
            i = i - 1;
            if !(c2rust_fresh4 != 0) {
                break;
            }
            let ref mut c2rust_fresh5 = *list.offset((i + 1i32) as isize);
            *c2rust_fresh5 = *list.offset(i as isize);
        }
        let ref mut c2rust_fresh6 = *list.offset(0isize);
        *c2rust_fresh6 = frame;
    }
}
pub unsafe extern "C" fn x264_frame_shift(
    mut list: *mut *mut crate::src::common::frame::x264_frame_t,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        let mut i = 0i32;
        let mut frame = *list.offset(0isize);
        while !(*list.offset(i as isize)).is_null() {
            let ref mut c2rust_fresh7 = *list.offset(i as isize);
            *c2rust_fresh7 = *list.offset((i + 1i32) as isize);
            i += 1;
        }
        assert!(!frame.is_null());
        frame
    }
}
pub unsafe extern "C" fn x264_8_frame_push_unused(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        '_c2rust_label: {
            if (*frame).i_reference_count > 0i32 {
            } else {
                crate::stdlib::__assert_fail(
                    b"frame->i_reference_count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"common/frame.c\0".as_ptr() as *const ::core::ffi::c_char,
                    774u32,
                    b"void x264_8_frame_push_unused(x264_t *, x264_frame_t *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*frame).i_reference_count -= 1;
        if (*frame).i_reference_count == 0i32 {
            x264_8_frame_push((*h).frames.unused[(*frame).b_fdec as usize], frame);
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_pop_unused(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_fdec: ::core::ffi::c_int,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        let mut frame = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        if !(*(*h).frames.unused[b_fdec as usize].offset(0isize)).is_null() {
            frame = x264_8_frame_pop((*h).frames.unused[b_fdec as usize]);
        } else {
            frame = frame_new(h, b_fdec);
        }
        if frame.is_null() {
            return ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        }
        (*frame).b_last_minigop_bframe = 0u8;
        (*frame).i_reference_count = 1i32;
        (*frame).intra_calculated = false;
        (*frame).scenecut = true;
        (*frame).keyframe = false;
        (*frame).corrupt = false;
        (*frame).i_slice_count = if (*h).param.sliced_threads {
            (*h).param.i_threads
        } else {
            1i32
        };
        crate::stdlib::memset(
            &raw mut (*frame).weight as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<[[crate::src::common::mc::x264_weight_t; 3]; 16]>(),
        );
        crate::stdlib::memset(
            &raw mut (*frame).f_weighted_cost_delta as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<[::core::ffi::c_float; 18]>(),
        );
        frame
    }
}
pub unsafe extern "C" fn x264_8_frame_push_blank_unused(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        '_c2rust_label: {
            if (*frame).i_reference_count > 0i32 {
            } else {
                crate::stdlib::__assert_fail(
                    b"frame->i_reference_count > 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"common/frame.c\0".as_ptr() as *const ::core::ffi::c_char,
                    805u32,
                    b"void x264_8_frame_push_blank_unused(x264_t *, x264_frame_t *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*frame).i_reference_count -= 1;
        if (*frame).i_reference_count == 0i32 {
            x264_8_frame_push((*h).frames.blank_unused, frame);
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_pop_blank_unused(
    mut h: *mut crate::src::common::common::x264_t,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        let mut frame = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        if !(*(*h).frames.blank_unused.offset(0isize)).is_null() {
            frame = x264_8_frame_pop((*h).frames.blank_unused);
        } else {
            frame = crate::src::common::base::x264_malloc(::core::mem::size_of::<
                crate::src::common::frame::x264_frame_t,
            >() as crate::stdlib::int64_t)
                as *mut crate::src::common::frame::x264_frame_t;
        }
        if frame.is_null() {
            return ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        }
        (*frame).duplicate = true;
        (*frame).i_reference_count = 1i32;
        frame
    }
}
pub unsafe extern "C" fn x264_8_weight_scale_plane(
    mut _h: *mut crate::src::common::common::x264_t,
    mut dst: *mut crate::src::common::common::pixel,
    mut i_dst_stride: crate::stdlib::intptr_t,
    mut src: *mut crate::src::common::common::pixel,
    mut i_src_stride: crate::stdlib::intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut w: *mut crate::src::common::mc::x264_weight_t,
) {
    unsafe {
        while i_height > 0i32 {
            let mut x = 0i32;
            while x < i_width - 8i32 {
                (*(*w).weightfn.offset((16i32 >> 2i32) as isize))
                    .expect("non-null function pointer")(
                    dst.offset(x as isize),
                    i_dst_stride,
                    src.offset(x as isize),
                    i_src_stride,
                    w,
                    if i_height < 16i32 { i_height } else { 16i32 },
                );
                x += 16i32;
            }
            if x < i_width {
                (*(*w).weightfn.offset((8i32 >> 2i32) as isize))
                    .expect("non-null function pointer")(
                    dst.offset(x as isize),
                    i_dst_stride,
                    src.offset(x as isize),
                    i_src_stride,
                    w,
                    if i_height < 16i32 { i_height } else { 16i32 },
                );
            }
            i_height -= 16i32;
            dst = dst.offset(16isize * i_dst_stride);
            src = src.offset(16isize * i_src_stride);
        }
    }
}
pub unsafe extern "C" fn x264_8_frame_delete_list(
    mut list: *mut *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        let mut i = 0i32;
        if list.is_null() {
            return;
        }
        while !(*list.offset(i as isize)).is_null() {
            let c2rust_fresh35 = i;
            i = i + 1;
            x264_8_frame_delete(*list.offset(c2rust_fresh35 as isize));
        }
        crate::src::common::base::x264_free(list as *mut ::core::ffi::c_void);
    }
}
pub unsafe extern "C" fn x264_8_sync_frame_list_init(
    mut slist: *mut crate::src::common::frame::x264_sync_frame_list_t,
    mut max_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if max_size < 0i32 {
            return -(1i32);
        }
        (*slist).i_max_size = max_size;
        (*slist).i_size = 0i32;
        (*slist).list = crate::src::common::base::x264_malloc(
            ((max_size + 1i32) as usize).wrapping_mul(::core::mem::size_of::<
                *mut crate::src::common::frame::x264_frame_t,
            >()) as crate::stdlib::int64_t,
        ) as *mut *mut crate::src::common::frame::x264_frame_t;
        if (*slist).list.is_null() {
            -(1i32)
        } else {
            crate::stdlib::memset(
                (*slist).list as *mut ::core::ffi::c_void,
                0i32,
                ((max_size + 1i32) as crate::__stddef_size_t_h::size_t).wrapping_mul(
                    ::core::mem::size_of::<*mut crate::src::common::frame::x264_frame_t>(),
                ),
            );
            if crate::stdlib::pthread_mutex_init(
                &raw mut (*slist).mutex,
                ::core::ptr::null::<crate::stdlib::pthread_mutexattr_t>(),
            ) != 0
                || crate::stdlib::pthread_cond_init(
                    &raw mut (*slist).cv_fill,
                    ::core::ptr::null::<crate::stdlib::pthread_condattr_t>(),
                ) != 0
                || crate::stdlib::pthread_cond_init(
                    &raw mut (*slist).cv_empty,
                    ::core::ptr::null::<crate::stdlib::pthread_condattr_t>(),
                ) != 0
            {
                return -(1i32);
            }
            0i32
        }
    }
}
pub unsafe extern "C" fn x264_8_sync_frame_list_delete(
    mut slist: *mut crate::src::common::frame::x264_sync_frame_list_t,
) {
    unsafe {
        crate::stdlib::pthread_mutex_destroy(&raw mut (*slist).mutex);
        crate::stdlib::pthread_cond_destroy(&raw mut (*slist).cv_fill);
        crate::stdlib::pthread_cond_destroy(&raw mut (*slist).cv_empty);
        x264_8_frame_delete_list((*slist).list);
    }
}
pub unsafe extern "C" fn x264_8_sync_frame_list_push(
    mut slist: *mut crate::src::common::frame::x264_sync_frame_list_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*slist).mutex);
        while (*slist).i_size == (*slist).i_max_size {
            crate::stdlib::pthread_cond_wait(&raw mut (*slist).cv_empty, &raw mut (*slist).mutex);
        }
        let c2rust_fresh36 = (*slist).i_size;
        (*slist).i_size = (*slist).i_size + 1;
        let ref mut c2rust_fresh37 = *(*slist).list.offset(c2rust_fresh36 as isize);
        *c2rust_fresh37 = frame;
        crate::stdlib::pthread_mutex_unlock(&raw mut (*slist).mutex);
        crate::stdlib::pthread_cond_broadcast(&raw mut (*slist).cv_fill);
    }
}
pub unsafe extern "C" fn x264_8_sync_frame_list_pop(
    mut slist: *mut crate::src::common::frame::x264_sync_frame_list_t,
) -> *mut crate::src::common::frame::x264_frame_t {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*slist).mutex);
        while (*slist).i_size == 0 {
            crate::stdlib::pthread_cond_wait(&raw mut (*slist).cv_fill, &raw mut (*slist).mutex);
        }
        (*slist).i_size -= 1;
        let mut frame = *(*slist).list.offset((*slist).i_size as isize);
        let ref mut c2rust_fresh38 = *(*slist).list.offset((*slist).i_size as isize);
        *c2rust_fresh38 = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
        crate::stdlib::pthread_cond_broadcast(&raw mut (*slist).cv_empty);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*slist).mutex);
        frame
    }
}
