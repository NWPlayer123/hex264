use core::ffi::{c_char, c_int, c_uint, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::analyse_h::{x264_10_slicetype_analyse, x264_10_slicetype_decide};
use crate::assert_h::__assert_fail;
use crate::base_h::{x264_free, x264_malloc};
use crate::common_h::{x264_lookahead_t, x264_t};
use crate::frame_h::{
    x264_10_frame_push, x264_10_frame_push_unused, x264_10_frame_shift,
    x264_10_sync_frame_list_delete, x264_10_sync_frame_list_init, x264_10_sync_frame_list_push,
    x264_frame_t, x264_sync_frame_list_t,
};
use crate::macroblock_h::{
    x264_10_macroblock_cache_allocate, x264_10_macroblock_cache_free,
    x264_10_macroblock_thread_allocate, x264_10_macroblock_thread_free,
};
use crate::pthread_h::{
    pthread_cond_broadcast, pthread_cond_wait, pthread_create, pthread_join, pthread_mutex_lock,
    pthread_mutex_unlock,
};
use crate::pthreadtypes_h::pthread_attr_t;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::uint8_t;
use crate::string_h::memset;
use crate::x264_h::{X264_TYPE_I, X264_TYPE_IDR, X264_TYPE_KEYFRAME};
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn lookahead_shift(
    mut dst: *mut x264_sync_frame_list_t,
    mut src: *mut x264_sync_frame_list_t,
    mut count: c_int,
) {
    let mut i: c_int = count;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        if (*dst).i_size < (*dst).i_max_size {
        } else {
            __assert_fail(
                b"dst->i_size < dst->i_max_size\0" as *const u8
                    as *const c_char,
                b"encoder/lookahead.c\0" as *const u8 as *const c_char,
                47 as c_uint,
                ::core::mem::transmute::<
                    [u8; 78],
                    [c_char; 78],
                >(
                        *b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                    )
                    .as_ptr(),
            );
        }
        if (*src).i_size != 0 {
        } else {
            __assert_fail(
                b"src->i_size\0" as *const u8 as *const c_char,
                b"encoder/lookahead.c\0" as *const u8 as *const c_char,
                48 as c_uint,
                ::core::mem::transmute::<
                    [u8; 78],
                    [c_char; 78],
                >(
                        *b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                    )
                    .as_ptr(),
            );
        }
        let fresh1 = (*dst).i_size;
        (*dst).i_size = (*dst).i_size + 1;
        let ref mut fresh2 = *(*dst).list.offset(fresh1 as isize);
        *fresh2 = x264_10_frame_shift((*src).list);
        (*src).i_size -= 1;
    }
    if count != 0 {
        pthread_cond_broadcast(&mut (*dst).cv_fill);
        pthread_cond_broadcast(&mut (*src).cv_empty);
    }
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn lookahead_update_last_nonb(
    mut h: *mut x264_t,
    mut new_nonb: *mut x264_frame_t,
) {
    if !(*(*h).lookahead).last_nonb.is_null() {
        x264_10_frame_push_unused(h, (*(*h).lookahead).last_nonb);
    }
    (*(*h).lookahead).last_nonb = new_nonb;
    (*new_nonb).i_reference_count += 1;
}
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn lookahead_slicetype_decide(mut h: *mut x264_t) {
    x264_10_slicetype_decide(h);
    lookahead_update_last_nonb(h, *(*(*h).lookahead).next.list.offset(0 as c_int as isize));
    let mut shift_frames: c_int =
        (**(*(*h).lookahead).next.list.offset(0 as c_int as isize)).i_bframes as c_int + 1 as c_int;
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    while (*(*h).lookahead).ofbuf.i_size == (*(*h).lookahead).ofbuf.i_max_size {
        pthread_cond_wait(
            &mut (*(*h).lookahead).ofbuf.cv_empty,
            &mut (*(*h).lookahead).ofbuf.mutex,
        );
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    lookahead_shift(
        &mut (*(*h).lookahead).ofbuf,
        &mut (*(*h).lookahead).next,
        shift_frames,
    );
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    if (*(*h).lookahead).b_analyse_keyframe as c_int != 0
        && ((*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_I
            || (*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_IDR
            || (*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_KEYFRAME)
    {
        x264_10_slicetype_analyse(h, shift_frames);
    }
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn lookahead_thread(mut h: *mut x264_t) -> *mut c_void {
    loop {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        if (*(*h).lookahead).b_exit_thread != 0 {
            pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
            break;
        } else {
            pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
            let mut shift: c_int = if (*(*h).lookahead).next.i_max_size
                - (*(*h).lookahead).next.i_size
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
            pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
            if (*(*h).lookahead).next.i_size
                <= (*(*h).lookahead).i_slicetype_length + (*h).param.b_vfr_input
            {
                while (*(*h).lookahead).ifbuf.i_size == 0 && (*(*h).lookahead).b_exit_thread == 0 {
                    pthread_cond_wait(
                        &mut (*(*h).lookahead).ifbuf.cv_fill,
                        &mut (*(*h).lookahead).ifbuf.mutex,
                    );
                }
                pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
            } else {
                pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
                lookahead_slicetype_decide(h);
            }
        }
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    lookahead_shift(
        &mut (*(*h).lookahead).next,
        &mut (*(*h).lookahead).ifbuf,
        (*(*h).lookahead).ifbuf.i_size,
    );
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    while (*(*h).lookahead).next.i_size != 0 {
        lookahead_slicetype_decide(h);
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    (*(*h).lookahead).b_thread_active = 0 as uint8_t;
    pthread_cond_broadcast(&mut (*(*h).lookahead).ofbuf.cv_fill);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn x264_10_lookahead_init(
    mut h: *mut x264_t,
    mut i_slicetype_length: c_int,
) -> c_int {
    let mut look_h: *mut x264_t = 0 as *mut x264_t;
    let mut look: *mut x264_lookahead_t = 0 as *mut x264_lookahead_t;
    look =
        x264_malloc(::core::mem::size_of::<x264_lookahead_t>() as int64_t) as *mut x264_lookahead_t;
    if !look.is_null() {
        memset(
            look as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<x264_lookahead_t>() as size_t,
        );
        let mut i: c_int = 0 as c_int;
        while i < (*h).param.i_threads {
            (*(*h).thread[i as usize]).lookahead = look;
            i += 1;
        }
        (*look).i_last_keyframe = -(*h).param.i_keyint_max;
        (*look).b_analyse_keyframe = (((*h).param.rc.b_mb_tree != 0
            || (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0)
            && (*h).param.rc.b_stat_read == 0) as c_int
            as uint8_t;
        (*look).i_slicetype_length = i_slicetype_length;
        if !(x264_10_sync_frame_list_init(
            &mut (*look).ifbuf,
            (*h).param.i_sync_lookahead + 3 as c_int,
        ) != 0
            || x264_10_sync_frame_list_init(&mut (*look).next, (*h).frames.i_delay + 3 as c_int)
                != 0
            || x264_10_sync_frame_list_init(&mut (*look).ofbuf, (*h).frames.i_delay + 3 as c_int)
                != 0)
        {
            if (*h).param.i_sync_lookahead == 0 {
                return 0 as c_int;
            }
            look_h = (*h).thread[(*h).param.i_threads as usize];
            *look_h = *h;
            if !(x264_10_macroblock_cache_allocate(look_h) != 0) {
                if !(x264_10_macroblock_thread_allocate(look_h, 1 as c_int) < 0 as c_int) {
                    if !(pthread_create(
                        &mut (*look).thread_handle,
                        0 as *const pthread_attr_t,
                        ::core::mem::transmute::<
                            *mut c_void,
                            Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
                        >(::core::mem::transmute::<
                            Option<unsafe extern "C" fn(*mut x264_t) -> *mut c_void>,
                            *mut c_void,
                        >(Some(
                            lookahead_thread as unsafe extern "C" fn(*mut x264_t) -> *mut c_void,
                        ))),
                        look_h as *mut c_void,
                    ) != 0)
                    {
                        (*look).b_thread_active = 1 as uint8_t;
                        return 0 as c_int;
                    }
                }
            }
        }
    }
    x264_free(look as *mut c_void);
    return -(1 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn x264_10_lookahead_delete(mut h: *mut x264_t) {
    if (*h).param.i_sync_lookahead != 0 {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        ::core::ptr::write_volatile(
            &mut (*(*h).lookahead).b_exit_thread as *mut uint8_t,
            1 as uint8_t,
        );
        pthread_cond_broadcast(&mut (*(*h).lookahead).ifbuf.cv_fill);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
        pthread_join((*(*h).lookahead).thread_handle, 0 as *mut *mut c_void);
        x264_10_macroblock_cache_free((*h).thread[(*h).param.i_threads as usize]);
        x264_10_macroblock_thread_free((*h).thread[(*h).param.i_threads as usize], 1 as c_int);
        x264_free((*h).thread[(*h).param.i_threads as usize] as *mut c_void);
    }
    x264_10_sync_frame_list_delete(&mut (*(*h).lookahead).ifbuf);
    x264_10_sync_frame_list_delete(&mut (*(*h).lookahead).next);
    if !(*(*h).lookahead).last_nonb.is_null() {
        x264_10_frame_push_unused(h, (*(*h).lookahead).last_nonb);
    }
    x264_10_sync_frame_list_delete(&mut (*(*h).lookahead).ofbuf);
    x264_free((*h).lookahead as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "192:1"]
unsafe extern "C" fn x264_10_lookahead_put_frame(mut h: *mut x264_t, mut frame: *mut x264_frame_t) {
    if (*h).param.i_sync_lookahead != 0 {
        x264_10_sync_frame_list_push(&mut (*(*h).lookahead).ifbuf, frame);
    } else {
        x264_10_sync_frame_list_push(&mut (*(*h).lookahead).next, frame);
    };
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn x264_10_lookahead_is_empty(mut h: *mut x264_t) -> c_int {
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    let mut b_empty: c_int =
        ((*(*h).lookahead).next.i_size == 0 && (*(*h).lookahead).ofbuf.i_size == 0) as c_int;
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    return b_empty;
}
#[c2rust::src_loc = "210:1"]
unsafe extern "C" fn lookahead_encoder_shift(mut h: *mut x264_t) {
    if (*(*h).lookahead).ofbuf.i_size == 0 {
        return;
    }
    let mut i_frames: c_int = (**(*(*h).lookahead).ofbuf.list.offset(0 as c_int as isize)).i_bframes
        as c_int
        + 1 as c_int;
    loop {
        let fresh3 = i_frames;
        i_frames = i_frames - 1;
        if !(fresh3 != 0) {
            break;
        }
        x264_10_frame_push(
            (*h).frames.current,
            x264_10_frame_shift((*(*h).lookahead).ofbuf.list),
        );
        (*(*h).lookahead).ofbuf.i_size -= 1;
    }
    pthread_cond_broadcast(&mut (*(*h).lookahead).ofbuf.cv_empty);
}
#[no_mangle]
#[c2rust::src_loc = "223:1"]
unsafe extern "C" fn x264_10_lookahead_get_frames(mut h: *mut x264_t) {
    if (*h).param.i_sync_lookahead != 0 {
        pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
        while (*(*h).lookahead).ofbuf.i_size == 0 && (*(*h).lookahead).b_thread_active as c_int != 0
        {
            pthread_cond_wait(
                &mut (*(*h).lookahead).ofbuf.cv_fill,
                &mut (*(*h).lookahead).ofbuf.mutex,
            );
        }
        lookahead_encoder_shift(h);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    } else {
        if !(*(*h).frames.current.offset(0 as c_int as isize)).is_null()
            || (*(*h).lookahead).next.i_size == 0
        {
            return;
        }
        x264_10_slicetype_decide(h);
        lookahead_update_last_nonb(h, *(*(*h).lookahead).next.list.offset(0 as c_int as isize));
        let mut shift_frames: c_int = (**(*(*h).lookahead).next.list.offset(0 as c_int as isize))
            .i_bframes as c_int
            + 1 as c_int;
        lookahead_shift(
            &mut (*(*h).lookahead).ofbuf,
            &mut (*(*h).lookahead).next,
            shift_frames,
        );
        if (*(*h).lookahead).b_analyse_keyframe as c_int != 0
            && ((*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_I
                || (*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_IDR
                || (*(*(*h).lookahead).last_nonb).i_type == X264_TYPE_KEYFRAME)
        {
            x264_10_slicetype_analyse(h, shift_frames);
        }
        lookahead_encoder_shift(h);
    };
}
