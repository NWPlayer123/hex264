pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

use crate::src::common::base::x264_free;
use crate::src::common::base::x264_malloc;
pub use crate::src::common::bitstream::bs_s;
pub use crate::src::common::bitstream::bs_t;
pub use crate::src::common::bitstream::x264_bitstream_function_t;
pub use crate::src::common::bitstream::x264_run_level_t;
pub use crate::src::common::cabac::x264_cabac_t;
use crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_analyse;
use crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_decide;
pub use crate::stdlib::C2Rust_Unnamed_7;
use crate::stdlib::__assert_fail;
use crate::stdlib::__assert_single_arg;
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
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::frame::x264_8_frame_push;
pub use crate::src::common::frame::x264_8_frame_push_unused;
pub use crate::src::common::frame::x264_8_frame_shift;
pub use crate::src::common::frame::x264_8_sync_frame_list_delete;
pub use crate::src::common::frame::x264_8_sync_frame_list_init;
pub use crate::src::common::frame::x264_8_sync_frame_list_push;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;

use crate::src::common::macroblock::x264_8_macroblock_cache_allocate;
use crate::src::common::macroblock::x264_8_macroblock_cache_free;
use crate::src::common::macroblock::x264_8_macroblock_thread_allocate;
use crate::src::common::macroblock::x264_8_macroblock_thread_free;
pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_mc_functions_t_17;
pub use crate::src::common::mc::x264_weight_t;
pub use crate::src::common::pixel::x264_pixel_cmp_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x3_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x4_t;
pub use crate::src::common::pixel::x264_pixel_function_t;
pub use crate::src::common::predict::x264_predict8x8_t;
pub use crate::src::common::predict::x264_predict_8x8_filter_t;
pub use crate::src::common::predict::x264_predict_t;
pub use crate::src::common::quant::x264_quant_function_t;
pub use crate::stdlib::pthread_attr_t;
use crate::stdlib::pthread_cond_broadcast;
pub use crate::stdlib::pthread_cond_t;
use crate::stdlib::pthread_cond_wait;
use crate::stdlib::pthread_create;
use crate::stdlib::pthread_join;
use crate::stdlib::pthread_mutex_lock;
pub use crate::stdlib::pthread_mutex_t;
use crate::stdlib::pthread_mutex_unlock;
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
pub use crate::x264_h::X264_TYPE_I;
pub use crate::x264_h::X264_TYPE_IDR;
pub use crate::x264_h::X264_TYPE_KEYFRAME;

unsafe extern "C" fn lookahead_shift(
    mut dst: *mut crate::src::common::frame::x264_sync_frame_list_t,
    mut src: *mut crate::src::common::frame::x264_sync_frame_list_t,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = count;
        loop {
            let c2rust_fresh0 = i;
            i = i - 1;
            if !(c2rust_fresh0 != 0) {
                break;
            }
            '_c2rust_label: {
                if (*dst).i_size < (*dst).i_max_size {
                } else {
                    crate::stdlib::__assert_fail(
                        b"dst->i_size < dst->i_max_size\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"encoder/lookahead.c\0".as_ptr() as *const ::core::ffi::c_char,
                        47 as ::core::ffi::c_uint,
                        b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if (*src).i_size != 0 {
                } else {
                    crate::stdlib::__assert_fail(
                        b"src->i_size\0".as_ptr() as *const ::core::ffi::c_char,
                        b"encoder/lookahead.c\0".as_ptr() as *const ::core::ffi::c_char,
                        48 as ::core::ffi::c_uint,
                        b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            let c2rust_fresh1 = (*dst).i_size;
            (*dst).i_size = (*dst).i_size + 1;
            let ref mut c2rust_fresh2 = *(*dst).list.offset(c2rust_fresh1 as isize);
            *c2rust_fresh2 = crate::src::common::frame::x264_8_frame_shift(
                (*src).list as *mut *mut crate::src::common::frame::x264_frame,
            ) as *mut crate::src::common::frame::x264_frame;
            (*src).i_size -= 1;
        }
        if count != 0 {
            crate::stdlib::pthread_cond_broadcast(&raw mut (*dst).cv_fill);
            crate::stdlib::pthread_cond_broadcast(&raw mut (*src).cv_empty);
        }
    }
}

unsafe extern "C" fn lookahead_update_last_nonb(
    mut h: *mut crate::src::common::common::x264_t,
    mut new_nonb: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        if !(*(*h).lookahead).last_nonb.is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                h as *mut crate::src::common::common::x264_t,
                (*(*h).lookahead).last_nonb as *mut crate::src::common::frame::x264_frame,
            );
        }
        (*(*h).lookahead).last_nonb = new_nonb;
        (*new_nonb).i_reference_count += 1;
    }
}

unsafe extern "C" fn lookahead_slicetype_decide(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_decide(
            h as *mut crate::src::common::common::x264_t,
        );
        lookahead_update_last_nonb(
            h,
            *(*(*h).lookahead)
                .next
                .list
                .offset(0 as ::core::ffi::c_int as isize),
        );
        let mut shift_frames: ::core::ffi::c_int = (**(*(*h).lookahead)
            .next
            .list
            .offset(0 as ::core::ffi::c_int as isize))
        .i_bframes as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        while (*(*h).lookahead).ofbuf.i_size == (*(*h).lookahead).ofbuf.i_max_size {
            crate::stdlib::pthread_cond_wait(
                &raw mut (*(*h).lookahead).ofbuf.cv_empty,
                &raw mut (*(*h).lookahead).ofbuf.mutex,
            );
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        lookahead_shift(
            &raw mut (*(*h).lookahead).ofbuf,
            &raw mut (*(*h).lookahead).next,
            shift_frames,
        );
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        if (*(*h).lookahead).b_analyse_keyframe as ::core::ffi::c_int != 0
            && ((*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_I
                || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_IDR
                || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
        {
            crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_analyse(
                h as *mut crate::src::common::common::x264_t,
                shift_frames,
            );
        }
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
    }
}

unsafe extern "C" fn lookahead_thread(
    mut h: *mut crate::src::common::common::x264_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        loop {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            if (*(*h).lookahead).b_exit_thread != 0 {
                crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
                break;
            } else {
                crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
                let mut shift: ::core::ffi::c_int = if (*(*h).lookahead).next.i_max_size
                    - (*(*h).lookahead).next.i_size
                    < (*(*h).lookahead).ifbuf.i_size
                {
                    (*(*h).lookahead).next.i_max_size - (*(*h).lookahead).next.i_size
                } else {
                    (*(*h).lookahead).ifbuf.i_size
                };
                lookahead_shift(
                    &raw mut (*(*h).lookahead).next,
                    &raw mut (*(*h).lookahead).ifbuf,
                    shift,
                );
                crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
                if (*(*h).lookahead).next.i_size
                    <= (*(*h).lookahead).i_slicetype_length + (*h).param.b_vfr_input
                {
                    while (*(*h).lookahead).ifbuf.i_size == 0
                        && (*(*h).lookahead).b_exit_thread == 0
                    {
                        crate::stdlib::pthread_cond_wait(
                            &raw mut (*(*h).lookahead).ifbuf.cv_fill,
                            &raw mut (*(*h).lookahead).ifbuf.mutex,
                        );
                    }
                    crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
                } else {
                    crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
                    lookahead_slicetype_decide(h);
                }
            }
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        lookahead_shift(
            &raw mut (*(*h).lookahead).next,
            &raw mut (*(*h).lookahead).ifbuf,
            (*(*h).lookahead).ifbuf.i_size,
        );
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        while (*(*h).lookahead).next.i_size != 0 {
            lookahead_slicetype_decide(h);
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        (*(*h).lookahead).b_thread_active = 0 as crate::stdlib::uint8_t;
        crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ofbuf.cv_fill);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        return crate::__stddef_null_h::NULL;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_lookahead_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_slicetype_length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut look_h: *mut crate::src::common::common::x264_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_t>();
        let mut look: *mut crate::src::common::common::x264_lookahead_t =
            ::core::ptr::null_mut::<crate::src::common::common::x264_lookahead_t>();
        look = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::src::common::common::x264_lookahead_t,
        >() as crate::stdlib::int64_t)
            as *mut crate::src::common::common::x264_lookahead_t;
        if !look.is_null() {
            crate::stdlib::memset(
                look as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<crate::src::common::common::x264_lookahead_t>()
                    as crate::__stddef_size_t_h::size_t,
            );
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).param.i_threads {
                (*(*h).thread[i as usize]).lookahead = look;
                i += 1;
            }
            (*look).i_last_keyframe = -(*h).param.i_keyint_max;
            (*look).b_analyse_keyframe = (((*h).param.rc.b_mb_tree != 0
                || (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0)
                && (*h).param.rc.b_stat_read == 0)
                as ::core::ffi::c_int
                as crate::stdlib::uint8_t;
            (*look).i_slicetype_length = i_slicetype_length;
            if !(crate::src::common::frame::x264_8_sync_frame_list_init(
                &raw mut (*look).ifbuf as *mut _
                    as *mut crate::src::common::frame::x264_sync_frame_list_t,
                (*h).param.i_sync_lookahead + 3 as ::core::ffi::c_int,
            ) != 0
                || crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*look).next as *mut _
                        as *mut crate::src::common::frame::x264_sync_frame_list_t,
                    (*h).frames.i_delay + 3 as ::core::ffi::c_int,
                ) != 0
                || crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*look).ofbuf as *mut _
                        as *mut crate::src::common::frame::x264_sync_frame_list_t,
                    (*h).frames.i_delay + 3 as ::core::ffi::c_int,
                ) != 0)
            {
                if (*h).param.i_sync_lookahead == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                look_h = (*h).thread[(*h).param.i_threads as usize];
                *look_h = *h;
                if !(crate::src::common::macroblock::x264_8_macroblock_cache_allocate(
                    look_h as *mut crate::src::common::common::x264_t,
                ) != 0)
                {
                    if !(crate::src::common::macroblock::x264_8_macroblock_thread_allocate(
                        look_h as *mut crate::src::common::common::x264_t,
                        1 as ::core::ffi::c_int,
                    ) < 0 as ::core::ffi::c_int)
                    {
                        if !(crate::stdlib::pthread_create(
                            &raw mut (*look).thread_handle,
                            ::core::ptr::null::<crate::stdlib::pthread_attr_t>(),
                            ::core::mem::transmute::<
                                *mut ::core::ffi::c_void,
                                Option<
                                    unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                    )
                                        -> *mut ::core::ffi::c_void,
                                >,
                            >(::core::mem::transmute::<
                                Option<
                                    unsafe extern "C" fn(
                                        *mut crate::src::common::common::x264_t,
                                    )
                                        -> *mut ::core::ffi::c_void,
                                >,
                                *mut ::core::ffi::c_void,
                            >(Some(
                                lookahead_thread
                                    as unsafe extern "C" fn(
                                        *mut crate::src::common::common::x264_t,
                                    )
                                        -> *mut ::core::ffi::c_void,
                            ))),
                            look_h as *mut ::core::ffi::c_void,
                        ) != 0)
                        {
                            (*look).b_thread_active = 1 as crate::stdlib::uint8_t;
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        crate::src::common::base::x264_free(look as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_lookahead_delete(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        if (*h).param.i_sync_lookahead != 0 {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            ::core::ptr::write_volatile(
                &mut (*(*h).lookahead).b_exit_thread as *mut crate::stdlib::uint8_t,
                1 as crate::stdlib::uint8_t,
            );
            crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ifbuf.cv_fill);
            crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            crate::stdlib::pthread_join(
                (*(*h).lookahead).thread_handle,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            crate::src::common::macroblock::x264_8_macroblock_cache_free(
                (*h).thread[(*h).param.i_threads as usize]
                    as *mut crate::src::common::common::x264_t,
            );
            crate::src::common::macroblock::x264_8_macroblock_thread_free(
                (*h).thread[(*h).param.i_threads as usize]
                    as *mut crate::src::common::common::x264_t,
                1 as ::core::ffi::c_int,
            );
            crate::src::common::base::x264_free(
                (*h).thread[(*h).param.i_threads as usize] as *mut ::core::ffi::c_void,
            );
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(
            &raw mut (*(*h).lookahead).ifbuf as *mut _
                as *mut crate::src::common::frame::x264_sync_frame_list_t,
        );
        crate::src::common::frame::x264_8_sync_frame_list_delete(
            &raw mut (*(*h).lookahead).next as *mut _
                as *mut crate::src::common::frame::x264_sync_frame_list_t,
        );
        if !(*(*h).lookahead).last_nonb.is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(
                h as *mut crate::src::common::common::x264_t,
                (*(*h).lookahead).last_nonb as *mut crate::src::common::frame::x264_frame,
            );
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(
            &raw mut (*(*h).lookahead).ofbuf as *mut _
                as *mut crate::src::common::frame::x264_sync_frame_list_t,
        );
        crate::src::common::base::x264_free((*h).lookahead as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_lookahead_put_frame(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        if (*h).param.i_sync_lookahead != 0 {
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*(*h).lookahead).ifbuf as *mut _
                    as *mut crate::src::common::frame::x264_sync_frame_list_t,
                frame as *mut crate::src::common::frame::x264_frame,
            );
        } else {
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*(*h).lookahead).next as *mut _
                    as *mut crate::src::common::frame::x264_sync_frame_list_t,
                frame as *mut crate::src::common::frame::x264_frame,
            );
        };
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_lookahead_is_empty(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        let mut b_empty: ::core::ffi::c_int = ((*(*h).lookahead).next.i_size == 0
            && (*(*h).lookahead).ofbuf.i_size == 0)
            as ::core::ffi::c_int;
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        return b_empty;
    }
}

unsafe extern "C" fn lookahead_encoder_shift(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*(*h).lookahead).ofbuf.i_size == 0 {
            return;
        }
        let mut i_frames: ::core::ffi::c_int = (**(*(*h).lookahead)
            .ofbuf
            .list
            .offset(0 as ::core::ffi::c_int as isize))
        .i_bframes as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        loop {
            let c2rust_fresh3 = i_frames;
            i_frames = i_frames - 1;
            if !(c2rust_fresh3 != 0) {
                break;
            }
            crate::src::common::frame::x264_8_frame_push(
                (*h).frames.current as *mut *mut crate::src::common::frame::x264_frame,
                crate::src::common::frame::x264_8_frame_shift(
                    (*(*h).lookahead).ofbuf.list
                        as *mut *mut crate::src::common::frame::x264_frame,
                ) as *mut crate::src::common::frame::x264_frame
                    as *mut crate::src::common::frame::x264_frame,
            );
            (*(*h).lookahead).ofbuf.i_size -= 1;
        }
        crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ofbuf.cv_empty);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_lookahead_get_frames(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        if (*h).param.i_sync_lookahead != 0 {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
            while (*(*h).lookahead).ofbuf.i_size == 0
                && (*(*h).lookahead).b_thread_active as ::core::ffi::c_int != 0
            {
                crate::stdlib::pthread_cond_wait(
                    &raw mut (*(*h).lookahead).ofbuf.cv_fill,
                    &raw mut (*(*h).lookahead).ofbuf.mutex,
                );
            }
            lookahead_encoder_shift(h);
            crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        } else {
            if !(*(*h).frames.current.offset(0 as ::core::ffi::c_int as isize)).is_null()
                || (*(*h).lookahead).next.i_size == 0
            {
                return;
            }
            crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_decide(
                h as *mut crate::src::common::common::x264_t,
            );
            lookahead_update_last_nonb(
                h,
                *(*(*h).lookahead)
                    .next
                    .list
                    .offset(0 as ::core::ffi::c_int as isize),
            );
            let mut shift_frames: ::core::ffi::c_int = (**(*(*h).lookahead)
                .next
                .list
                .offset(0 as ::core::ffi::c_int as isize))
            .i_bframes as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            lookahead_shift(
                &raw mut (*(*h).lookahead).ofbuf,
                &raw mut (*(*h).lookahead).next,
                shift_frames,
            );
            if (*(*h).lookahead).b_analyse_keyframe as ::core::ffi::c_int != 0
                && ((*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_I
                    || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_IDR
                    || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
            {
                crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_analyse(
                    h as *mut crate::src::common::common::x264_t,
                    shift_frames,
                );
            }
            lookahead_encoder_shift(h);
        };
    }
}
