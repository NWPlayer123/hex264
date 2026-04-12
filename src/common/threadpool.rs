#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_threadpool_t {
    pub exit: ::core::ffi::c_int,
    pub threads: ::core::ffi::c_int,
    pub thread_handle: *mut crate::stdlib::pthread_t,
    pub uninit: crate::src::common::frame::x264_sync_frame_list_t,
    pub run: crate::src::common::frame::x264_sync_frame_list_t,
    pub done: crate::src::common::frame::x264_sync_frame_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_threadpool_job_t {
    pub func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    pub arg: *mut ::core::ffi::c_void,
    pub ret: *mut ::core::ffi::c_void,
}
unsafe extern "C" fn threadpool_thread(
    mut pool: *mut x264_threadpool_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        while (*pool).exit == 0 {
            let mut job = ::core::ptr::null_mut::<x264_threadpool_job_t>();
            crate::stdlib::pthread_mutex_lock(&raw mut (*pool).run.mutex);
            while (*pool).exit == 0 && (*pool).run.i_size == 0 {
                crate::stdlib::pthread_cond_wait(
                    &raw mut (*pool).run.cv_fill,
                    &raw mut (*pool).run.mutex,
                );
            }
            if (*pool).run.i_size != 0 {
                job = crate::src::common::frame::x264_frame_shift((*pool).run.list)
                    as *mut x264_threadpool_job_t;
                (*pool).run.i_size -= 1;
            }
            crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).run.mutex);
            if job.is_null() {
                continue;
            }
            (*job).ret = (*job).func.expect("non-null function pointer")((*job).arg);
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*pool).done,
                job as *mut crate::src::common::frame::x264_frame,
            );
        }
        crate::__stddef_null_h::NULL
    }
}
pub unsafe extern "C" fn x264_8_threadpool_init(
    mut p_pool: *mut *mut x264_threadpool_t,
    mut threads: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if threads <= 0i32 {
            return -(1i32);
        }
        if (0i32) < 0i32 {
            return -(1i32);
        }
        let mut pool = crate::src::common::base::x264_malloc(
            ::core::mem::size_of::<x264_threadpool_t>() as crate::stdlib::int64_t,
        ) as *mut x264_threadpool_t;
        if !pool.is_null() {
            crate::stdlib::memset(
                pool as *mut ::core::ffi::c_void,
                0i32,
                ::core::mem::size_of::<x264_threadpool_t>(),
            );
            *p_pool = pool;
            (*pool).threads = threads;
            (*pool).thread_handle = crate::src::common::base::x264_malloc(
                ((*pool).threads as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::pthread_t>())
                    as crate::stdlib::int64_t,
            ) as *mut crate::stdlib::pthread_t;
            if !(*pool).thread_handle.is_null()
                && !(crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*pool).uninit,
                    (*pool).threads,
                ) != 0
                    || crate::src::common::frame::x264_8_sync_frame_list_init(
                        &raw mut (*pool).run,
                        (*pool).threads,
                    ) != 0
                    || crate::src::common::frame::x264_8_sync_frame_list_init(
                        &raw mut (*pool).done,
                        (*pool).threads,
                    ) != 0)
            {
                let mut c2rust_current_block: u64;
                loop {
                    let mut i = 0i32;
                    if i >= (*pool).threads {
                        c2rust_current_block = 11584701595673473500;
                        break;
                    }
                    let mut job = crate::src::common::base::x264_malloc(::core::mem::size_of::<
                        x264_threadpool_job_t,
                    >()
                        as crate::stdlib::int64_t)
                        as *mut x264_threadpool_job_t;
                    if job.is_null() {
                        c2rust_current_block = 11983467789922532698;
                        break;
                    }
                    crate::src::common::frame::x264_8_sync_frame_list_push(
                        &raw mut (*pool).uninit,
                        job as *mut crate::src::common::frame::x264_frame,
                    );
                    i += 1;
                }
                match c2rust_current_block {
                    11983467789922532698 => {}
                    _ => {
                        loop {
                            let mut i_0 = 0i32;
                            if i_0 >= (*pool).threads {
                                c2rust_current_block = 5634871135123216486;
                                break;
                            }
                            if crate::stdlib::pthread_create(
                                (*pool).thread_handle.offset(i_0 as isize),
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
                                            *mut x264_threadpool_t,
                                        )
                                            -> *mut ::core::ffi::c_void,
                                    >,
                                    *mut ::core::ffi::c_void,
                                >(
                                    Some(
                                        threadpool_thread
                                            as unsafe extern "C" fn(
                                                *mut x264_threadpool_t,
                                            )
                                                -> *mut ::core::ffi::c_void,
                                    ),
                                )),
                                pool as *mut ::core::ffi::c_void,
                            ) != 0
                            {
                                c2rust_current_block = 11983467789922532698;
                                break;
                            }
                            i_0 += 1;
                        }
                        match c2rust_current_block {
                            11983467789922532698 => {}
                            _ => return 0i32,
                        }
                    }
                }
            }
        }
        -(1i32)
    }
}
pub unsafe extern "C" fn x264_8_threadpool_run(
    mut pool: *mut x264_threadpool_t,
    mut func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut job = crate::src::common::frame::x264_8_sync_frame_list_pop(&raw mut (*pool).uninit)
            as *mut x264_threadpool_job_t;
        (*job).func = func;
        (*job).arg = arg;
        crate::src::common::frame::x264_8_sync_frame_list_push(
            &raw mut (*pool).run,
            job as *mut crate::src::common::frame::x264_frame,
        );
    }
}
pub unsafe extern "C" fn x264_8_threadpool_wait(
    mut pool: *mut x264_threadpool_t,
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*pool).done.mutex);
        loop {
            let mut i = 0i32;
            while i < (*pool).done.i_size {
                if (*(*(*pool).done.list.offset(i as isize) as *mut x264_threadpool_job_t)).arg
                    == arg
                {
                    let mut job = crate::src::common::frame::x264_frame_shift(
                        (*pool).done.list.offset(i as isize),
                    ) as *mut x264_threadpool_job_t;
                    (*pool).done.i_size -= 1;
                    crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).done.mutex);
                    let mut ret = (*job).ret;
                    crate::src::common::frame::x264_8_sync_frame_list_push(
                        &raw mut (*pool).uninit,
                        job as *mut crate::src::common::frame::x264_frame,
                    );
                    return ret;
                }
                i += 1;
            }
            crate::stdlib::pthread_cond_wait(
                &raw mut (*pool).done.cv_fill,
                &raw mut (*pool).done.mutex,
            );
        }
    }
}
unsafe extern "C" fn threadpool_list_delete(
    mut slist: *mut crate::src::common::frame::x264_sync_frame_list_t,
) {
    unsafe {
        let mut i = 0i32;
        while !(*(*slist).list.offset(i as isize)).is_null() {
            crate::src::common::base::x264_free(
                *(*slist).list.offset(i as isize) as *mut ::core::ffi::c_void
            );
            let ref mut c2rust_fresh0 = *(*slist).list.offset(i as isize);
            *c2rust_fresh0 = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
            i += 1;
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(slist);
    }
}
pub unsafe extern "C" fn x264_8_threadpool_delete(mut pool: *mut x264_threadpool_t) {
    unsafe {
        let mut i = 0i32;
        crate::stdlib::pthread_mutex_lock(&raw mut (*pool).run.mutex);
        ::core::ptr::write_volatile(&mut (*pool).exit as *mut ::core::ffi::c_int, 1i32);
        crate::stdlib::pthread_cond_broadcast(&raw mut (*pool).run.cv_fill);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).run.mutex);
        while i < (*pool).threads {
            crate::stdlib::pthread_join(
                *(*pool).thread_handle.offset(i as isize),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            i += 1;
        }
        threadpool_list_delete(&raw mut (*pool).uninit);
        threadpool_list_delete(&raw mut (*pool).run);
        threadpool_list_delete(&raw mut (*pool).done);
        crate::src::common::base::x264_free((*pool).thread_handle as *mut ::core::ffi::c_void);
        crate::src::common::base::x264_free(pool as *mut ::core::ffi::c_void);
    }
}
