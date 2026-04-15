use crate::src::common::frame::{x264_frame_shift, x264_sync_frame_list_t};
unsafe extern "C" fn lookahead_shift(
    mut dst: &mut x264_sync_frame_list_t,
    mut src: &mut x264_sync_frame_list_t,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        for _ in 0..count {
            assert!(dst.i_size < dst.i_max_size);
            assert!(src.i_size != 0);
            *dst.list.offset(dst.i_size as isize) = x264_frame_shift(src.list);
            dst.i_size += 1;
            src.i_size -= 1;
        }
        if count != 0 {
            crate::stdlib::pthread_cond_broadcast(&raw mut dst.cv_fill);
            crate::stdlib::pthread_cond_broadcast(&raw mut src.cv_empty);
        }
    }
}
unsafe extern "C" fn lookahead_update_last_nonb(
    mut h: *mut crate::src::common::common::x264_t,
    mut new_nonb: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        if !(*(*h).lookahead).last_nonb.is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(h, (*(*h).lookahead).last_nonb);
        }
        (*(*h).lookahead).last_nonb = new_nonb;
        (*new_nonb).i_reference_count += 1;
    }
}
unsafe extern "C" fn lookahead_slicetype_decide(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_decide(h);
        lookahead_update_last_nonb(h, *(*(*h).lookahead).next.list.offset(0isize));
        let mut shift_frames =
            (**(*(*h).lookahead).next.list.offset(0isize)).i_bframes as ::core::ffi::c_int + 1i32;
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        while (*(*h).lookahead).ofbuf.i_size == (*(*h).lookahead).ofbuf.i_max_size {
            crate::stdlib::pthread_cond_wait(
                &raw mut (*(*h).lookahead).ofbuf.cv_empty,
                &raw mut (*(*h).lookahead).ofbuf.mutex,
            );
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        lookahead_shift(
            &mut (*(*h).lookahead).ofbuf,
            &mut (*(*h).lookahead).next,
            shift_frames,
        );
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        if (*(*h).lookahead).b_analyse_keyframe as ::core::ffi::c_int != 0
            && ((*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_I
                || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_IDR
                || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
        {
            crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_analyse(h, shift_frames);
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
                let mut shift = if (*(*h).lookahead).next.i_max_size - (*(*h).lookahead).next.i_size
                    < (*(*h).lookahead).ifbuf.i_size
                {
                    (*(*h).lookahead).next.i_max_size - (*(*h).lookahead).next.i_size
                } else {
                    (*(*h).lookahead).ifbuf.i_size
                };
                lookahead_shift(
                    &mut (*(*h).lookahead).next,
                    &mut (*(*h).lookahead).ifbuf,
                    shift,
                );
                crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
                if (*(*h).lookahead).next.i_size
                    <= (*(*h).lookahead).i_slicetype_length
                        + (*h).param.vfr_input as ::core::ffi::c_int
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
            &mut (*(*h).lookahead).next,
            &mut (*(*h).lookahead).ifbuf,
            (*(*h).lookahead).ifbuf.i_size,
        );
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
        while (*(*h).lookahead).next.i_size != 0 {
            lookahead_slicetype_decide(h);
        }
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        (*(*h).lookahead).b_thread_active = 0u8;
        crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ofbuf.cv_fill);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        crate::__stddef_null_h::NULL
    }
}
pub unsafe extern "C" fn x264_8_lookahead_init(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_slicetype_length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut look = crate::src::common::base::x264_malloc(::core::mem::size_of::<
            crate::src::common::common::x264_lookahead_t,
        >() as crate::stdlib::int64_t)
            as *mut crate::src::common::common::x264_lookahead_t;
        if !look.is_null() {
            let mut i = 0i32;
            crate::stdlib::memset(
                look as *mut ::core::ffi::c_void,
                0i32,
                ::core::mem::size_of::<crate::src::common::common::x264_lookahead_t>(),
            );
            while i < (*h).param.i_threads {
                (*(*h).thread[i as usize]).lookahead = look;
                i += 1;
            }
            (*look).i_last_keyframe = -(*h).param.i_keyint_max;
            (*look).b_analyse_keyframe = (((*h).param.rc.mb_tree
                || (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0)
                && !(*h).param.rc.stat_read)
                as crate::stdlib::uint8_t;
            (*look).i_slicetype_length = i_slicetype_length;
            if !(crate::src::common::frame::x264_8_sync_frame_list_init(
                &raw mut (*look).ifbuf,
                (*h).param.i_sync_lookahead + 3i32,
            ) != 0
                || crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*look).next,
                    (*h).frames.i_delay + 3i32,
                ) != 0
                || crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*look).ofbuf,
                    (*h).frames.i_delay + 3i32,
                ) != 0)
            {
                let mut look_h: *mut crate::src::common::common::x264_t<'_> =
                    ::core::ptr::null_mut();
                if (*h).param.i_sync_lookahead == 0 {
                    return 0i32;
                }
                look_h = (*h).thread[(*h).param.i_threads as usize];
                core::ptr::copy_nonoverlapping(h, look_h, 1);
                if (crate::src::common::macroblock::x264_8_macroblock_cache_allocate(look_h) == 0)
                    && (crate::src::common::macroblock::x264_8_macroblock_thread_allocate(
                        look_h, 1i32,
                    ) >= 0i32)
                    && (crate::stdlib::pthread_create(
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
                    ) == 0)
                {
                    (*look).b_thread_active = 1u8;
                    return 0i32;
                }
            }
        }
        crate::src::common::base::x264_free(look as *mut ::core::ffi::c_void);
        -(1i32)
    }
}
pub unsafe extern "C" fn x264_8_lookahead_delete(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*h).param.i_sync_lookahead != 0 {
            crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            ::core::ptr::write_volatile(
                &mut (*(*h).lookahead).b_exit_thread as *mut crate::stdlib::uint8_t,
                1u8,
            );
            crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ifbuf.cv_fill);
            crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ifbuf.mutex);
            crate::stdlib::pthread_join(
                (*(*h).lookahead).thread_handle,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            crate::src::common::macroblock::x264_8_macroblock_cache_free(
                (*h).thread[(*h).param.i_threads as usize],
            );
            crate::src::common::macroblock::x264_8_macroblock_thread_free(
                (*h).thread[(*h).param.i_threads as usize],
                1i32,
            );
            crate::src::common::base::x264_free(
                (*h).thread[(*h).param.i_threads as usize] as *mut ::core::ffi::c_void,
            );
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(&raw mut (*(*h).lookahead).ifbuf);
        crate::src::common::frame::x264_8_sync_frame_list_delete(&raw mut (*(*h).lookahead).next);
        if !(*(*h).lookahead).last_nonb.is_null() {
            crate::src::common::frame::x264_8_frame_push_unused(h, (*(*h).lookahead).last_nonb);
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(&raw mut (*(*h).lookahead).ofbuf);
        crate::src::common::base::x264_free((*h).lookahead as *mut ::core::ffi::c_void);
    }
}
pub unsafe extern "C" fn x264_8_lookahead_put_frame(
    mut h: *mut crate::src::common::common::x264_t,
    mut frame: *mut crate::src::common::frame::x264_frame_t,
) {
    unsafe {
        if (*h).param.i_sync_lookahead != 0 {
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*(*h).lookahead).ifbuf,
                frame,
            );
        } else {
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*(*h).lookahead).next,
                frame,
            );
        };
    }
}
pub unsafe extern "C" fn x264_8_lookahead_is_empty(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        crate::stdlib::pthread_mutex_lock(&raw mut (*(*h).lookahead).next.mutex);
        let mut b_empty = ((*(*h).lookahead).next.i_size == 0
            && (*(*h).lookahead).ofbuf.i_size == 0) as ::core::ffi::c_int;
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).next.mutex);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*(*h).lookahead).ofbuf.mutex);
        b_empty
    }
}
unsafe extern "C" fn lookahead_encoder_shift(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*(*h).lookahead).ofbuf.i_size == 0 {
            return;
        }
        let mut i_frames =
            (**(*(*h).lookahead).ofbuf.list.offset(0isize)).i_bframes as ::core::ffi::c_int + 1i32;
        loop {
            let c2rust_fresh3 = i_frames;
            i_frames -= 1;
            if c2rust_fresh3 == 0 {
                break;
            }
            crate::src::common::frame::x264_8_frame_push(
                (*h).frames.current,
                x264_frame_shift((*(*h).lookahead).ofbuf.list),
            );
            (*(*h).lookahead).ofbuf.i_size -= 1;
        }
        crate::stdlib::pthread_cond_broadcast(&raw mut (*(*h).lookahead).ofbuf.cv_empty);
    }
}
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
            if !(*(*h).frames.current.offset(0isize)).is_null()
                || (*(*h).lookahead).next.i_size == 0
            {
                return;
            }
            crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_decide(h);
            lookahead_update_last_nonb(h, *(*(*h).lookahead).next.list.offset(0isize));
            let mut shift_frames = (**(*(*h).lookahead).next.list.offset(0isize)).i_bframes
                as ::core::ffi::c_int
                + 1i32;
            lookahead_shift(
                &mut (*(*h).lookahead).ofbuf,
                &mut (*(*h).lookahead).next,
                shift_frames,
            );
            if (*(*h).lookahead).b_analyse_keyframe as ::core::ffi::c_int != 0
                && ((*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_I
                    || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_IDR
                    || (*(*(*h).lookahead).last_nonb).i_type == crate::x264_h::X264_TYPE_KEYFRAME)
            {
                crate::src::encoder::analyse::slicetype_c::x264_8_slicetype_analyse(
                    h,
                    shift_frames,
                );
            }
            lookahead_encoder_shift(h);
        };
    }
}
