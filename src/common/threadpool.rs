
use ::core::ffi::{c_int, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_free, x264_malloc};
use crate::frame_h::{
    x264_10_frame_shift, x264_10_sync_frame_list_delete, x264_10_sync_frame_list_init,
    x264_10_sync_frame_list_pop, x264_10_sync_frame_list_push,  x264_frame_t, x264_sync_frame_list_t,
};
use crate::pthread_h::{
    pthread_cond_broadcast, pthread_cond_wait, pthread_create, pthread_join, pthread_mutex_lock,
    pthread_mutex_unlock,
};
use crate::pthreadtypes_h::{pthread_attr_t, pthread_t};
use crate::stdint_intn_h::{int64_t};
use crate::string_h::memset;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "35:8"]
struct x264_threadpool_t {
    exit: c_int,
    threads: c_int,
    thread_handle: *mut pthread_t,
    uninit: x264_sync_frame_list_t,
    run: x264_sync_frame_list_t,
    done: x264_sync_frame_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "28:9"]
struct x264_threadpool_job_t {
    func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
    ret: *mut c_void,
}
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn threadpool_thread(
    mut pool: *mut x264_threadpool_t,
) -> *mut c_void {
    while (*pool).exit == 0 {
        let mut job: *mut x264_threadpool_job_t = 0 as *mut x264_threadpool_job_t;
        pthread_mutex_lock(&mut (*pool).run.mutex);
        while (*pool).exit == 0 && (*pool).run.i_size == 0 {
            pthread_cond_wait(&mut (*pool).run.cv_fill, &mut (*pool).run.mutex);
        }
        if (*pool).run.i_size != 0 {
            job = x264_10_frame_shift((*pool).run.list) as *mut c_void
                as *mut x264_threadpool_job_t;
            (*pool).run.i_size -= 1;
        }
        pthread_mutex_unlock(&mut (*pool).run.mutex);
        if job.is_null() {
            continue;
        }
        (*job).ret = (*job).func.expect("non-null function pointer")((*job).arg);
        x264_10_sync_frame_list_push(
            &mut (*pool).done,
            job as *mut c_void as *mut x264_frame_t,
        );
    }
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn x264_10_threadpool_init(
    mut p_pool: *mut *mut x264_threadpool_t,
    mut threads: c_int,
) -> c_int {
    let mut current_block: u64;
    if threads <= 0 as c_int {
        return -(1 as c_int);
    }
    if (0 as c_int) < 0 as c_int {
        return -(1 as c_int);
    }
    let mut pool: *mut x264_threadpool_t = 0 as *mut x264_threadpool_t;
    pool = x264_malloc(::core::mem::size_of::<x264_threadpool_t>() as int64_t)
        as *mut x264_threadpool_t;
    if !pool.is_null() {
        memset(
            pool as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<x264_threadpool_t>() as size_t,
        );
        *p_pool = pool;
        (*pool).threads = threads;
        (*pool).thread_handle = x264_malloc(
            ((*pool).threads as usize).wrapping_mul(::core::mem::size_of::<pthread_t>() as usize)
                as int64_t,
        ) as *mut pthread_t;
        if !(*pool).thread_handle.is_null() {
            if !(x264_10_sync_frame_list_init(&mut (*pool).uninit, (*pool).threads) != 0
                || x264_10_sync_frame_list_init(&mut (*pool).run, (*pool).threads) != 0
                || x264_10_sync_frame_list_init(&mut (*pool).done, (*pool).threads) != 0)
            {
                let mut i: c_int = 0 as c_int;
                loop {
                    if !(i < (*pool).threads) {
                        current_block = 11584701595673473500;
                        break;
                    }
                    let mut job: *mut x264_threadpool_job_t = 0 as *mut x264_threadpool_job_t;
                    job = x264_malloc(::core::mem::size_of::<x264_threadpool_job_t>() as int64_t)
                        as *mut x264_threadpool_job_t;
                    if job.is_null() {
                        current_block = 598935323617260105;
                        break;
                    }
                    x264_10_sync_frame_list_push(
                        &mut (*pool).uninit,
                        job as *mut c_void as *mut x264_frame_t,
                    );
                    i += 1;
                }
                match current_block {
                    598935323617260105 => {}
                    _ => {
                        let mut i_0: c_int = 0 as c_int;
                        loop {
                            if !(i_0 < (*pool).threads) {
                                current_block = 5634871135123216486;
                                break;
                            }
                            if pthread_create(
                                (*pool).thread_handle.offset(i_0 as isize),
                                0 as *const pthread_attr_t,
                                ::core::mem::transmute::<
                                    *mut c_void,
                                    Option<
                                        unsafe extern "C" fn(
                                            *mut c_void,
                                        )
                                            -> *mut c_void,
                                    >,
                                >(::core::mem::transmute::<
                                    Option<
                                        unsafe extern "C" fn(
                                            *mut x264_threadpool_t,
                                        )
                                            -> *mut c_void,
                                    >,
                                    *mut c_void,
                                >(
                                    Some(
                                        threadpool_thread
                                            as unsafe extern "C" fn(
                                                *mut x264_threadpool_t,
                                            )
                                                -> *mut c_void,
                                    ),
                                )),
                                pool as *mut c_void,
                            ) != 0
                            {
                                current_block = 598935323617260105;
                                break;
                            }
                            i_0 += 1;
                        }
                        match current_block {
                            598935323617260105 => {}
                            _ => return 0 as c_int,
                        }
                    }
                }
            }
        }
    }
    return -(1 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn x264_10_threadpool_run(
    mut pool: *mut x264_threadpool_t,
    mut func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    mut arg: *mut c_void,
) {
    let mut job: *mut x264_threadpool_job_t = x264_10_sync_frame_list_pop(&mut (*pool).uninit)
        as *mut c_void
        as *mut x264_threadpool_job_t;
    (*job).func = func;
    (*job).arg = arg;
    x264_10_sync_frame_list_push(
        &mut (*pool).run,
        job as *mut c_void as *mut x264_frame_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn x264_10_threadpool_wait(
    mut pool: *mut x264_threadpool_t,
    mut arg: *mut c_void,
) -> *mut c_void {
    pthread_mutex_lock(&mut (*pool).done.mutex);
    loop {
        let mut i: c_int = 0 as c_int;
        while i < (*pool).done.i_size {
            if (*(*(*pool).done.list.offset(i as isize) as *mut x264_threadpool_job_t)).arg == arg {
                let mut job: *mut x264_threadpool_job_t =
                    x264_10_frame_shift((*pool).done.list.offset(i as isize))
                        as *mut c_void
                        as *mut x264_threadpool_job_t;
                (*pool).done.i_size -= 1;
                pthread_mutex_unlock(&mut (*pool).done.mutex);
                let mut ret: *mut c_void = (*job).ret;
                x264_10_sync_frame_list_push(
                    &mut (*pool).uninit,
                    job as *mut c_void as *mut x264_frame_t,
                );
                return ret;
            }
            i += 1;
        }
        pthread_cond_wait(&mut (*pool).done.cv_fill, &mut (*pool).done.mutex);
    }
}
#[c2rust::src_loc = "135:1"]
unsafe extern "C" fn threadpool_list_delete(mut slist: *mut x264_sync_frame_list_t) {
    let mut i: c_int = 0 as c_int;
    while !(*(*slist).list.offset(i as isize)).is_null() {
        x264_free(*(*slist).list.offset(i as isize) as *mut c_void);
        let ref mut fresh0 = *(*slist).list.offset(i as isize);
        *fresh0 = 0 as *mut x264_frame_t;
        i += 1;
    }
    x264_10_sync_frame_list_delete(slist);
}
#[no_mangle]
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn x264_10_threadpool_delete(mut pool: *mut x264_threadpool_t) {
    pthread_mutex_lock(&mut (*pool).run.mutex);
    ::core::ptr::write_volatile(
        &mut (*pool).exit as *mut c_int,
        1 as c_int,
    );
    pthread_cond_broadcast(&mut (*pool).run.cv_fill);
    pthread_mutex_unlock(&mut (*pool).run.mutex);
    let mut i: c_int = 0 as c_int;
    while i < (*pool).threads {
        pthread_join(
            *(*pool).thread_handle.offset(i as isize),
            0 as *mut *mut c_void,
        );
        i += 1;
    }
    threadpool_list_delete(&mut (*pool).uninit);
    threadpool_list_delete(&mut (*pool).run);
    threadpool_list_delete(&mut (*pool).done);
    x264_free((*pool).thread_handle as *mut c_void);
    x264_free(pool as *mut c_void);
}
