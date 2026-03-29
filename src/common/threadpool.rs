pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

use crate::src::common::base::x264_free;
use crate::src::common::base::x264_malloc;
pub use crate::src::common::bitstream::bs_s;
pub use crate::src::common::bitstream::bs_t;
pub use crate::src::common::bitstream::x264_bitstream_function_t;
pub use crate::src::common::bitstream::x264_run_level_t;
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
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::frame::x264_8_frame_shift;
pub use crate::src::common::frame::x264_8_sync_frame_list_delete;
pub use crate::src::common::frame::x264_8_sync_frame_list_init;
pub use crate::src::common::frame::x264_8_sync_frame_list_pop;
pub use crate::src::common::frame::x264_8_sync_frame_list_push;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;

pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_mc_functions_t_11;
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
pub use crate::stdlib::__pthread_cond_s;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
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
            let mut job: *mut x264_threadpool_job_t =
                ::core::ptr::null_mut::<x264_threadpool_job_t>();
            crate::stdlib::pthread_mutex_lock(&raw mut (*pool).run.mutex);
            while (*pool).exit == 0 && (*pool).run.i_size == 0 {
                crate::stdlib::pthread_cond_wait(
                    &raw mut (*pool).run.cv_fill,
                    &raw mut (*pool).run.mutex,
                );
            }
            if (*pool).run.i_size != 0 {
                job = crate::src::common::frame::x264_8_frame_shift(
                    (*pool).run.list as *mut *mut crate::src::common::frame::x264_frame,
                ) as *mut crate::src::common::frame::x264_frame
                    as *mut ::core::ffi::c_void as *mut x264_threadpool_job_t;
                (*pool).run.i_size -= 1;
            }
            crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).run.mutex);
            if job.is_null() {
                continue;
            }
            (*job).ret = (*job).func.expect("non-null function pointer")((*job).arg);
            crate::src::common::frame::x264_8_sync_frame_list_push(
                &raw mut (*pool).done as *mut _
                    as *mut crate::src::common::frame::x264_sync_frame_list_t,
                job as *mut ::core::ffi::c_void as *mut crate::src::common::frame::x264_frame_t
                    as *mut crate::src::common::frame::x264_frame,
            );
        }
        return crate::__stddef_null_h::NULL;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_threadpool_init(
    mut p_pool: *mut *mut x264_threadpool_t,
    mut threads: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        if threads <= 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        if (0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        let mut pool: *mut x264_threadpool_t = ::core::ptr::null_mut::<x264_threadpool_t>();
        pool = crate::src::common::base::x264_malloc(
            ::core::mem::size_of::<x264_threadpool_t>() as crate::stdlib::int64_t
        ) as *mut x264_threadpool_t;
        if !pool.is_null() {
            crate::stdlib::memset(
                pool as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<x264_threadpool_t>() as crate::__stddef_size_t_h::size_t,
            );
            *p_pool = pool;
            (*pool).threads = threads;
            (*pool).thread_handle = crate::src::common::base::x264_malloc(
                ((*pool).threads as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::pthread_t>() as usize)
                    as crate::stdlib::int64_t,
            ) as *mut crate::stdlib::pthread_t;
            if !(*pool).thread_handle.is_null() {
                if !(crate::src::common::frame::x264_8_sync_frame_list_init(
                    &raw mut (*pool).uninit as *mut _
                        as *mut crate::src::common::frame::x264_sync_frame_list_t,
                    (*pool).threads,
                ) != 0
                    || crate::src::common::frame::x264_8_sync_frame_list_init(
                        &raw mut (*pool).run as *mut _
                            as *mut crate::src::common::frame::x264_sync_frame_list_t,
                        (*pool).threads,
                    ) != 0
                    || crate::src::common::frame::x264_8_sync_frame_list_init(
                        &raw mut (*pool).done as *mut _
                            as *mut crate::src::common::frame::x264_sync_frame_list_t,
                        (*pool).threads,
                    ) != 0)
                {
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < (*pool).threads) {
                            c2rust_current_block = 11584701595673473500;
                            break;
                        }
                        let mut job: *mut x264_threadpool_job_t =
                            ::core::ptr::null_mut::<x264_threadpool_job_t>();
                        job = crate::src::common::base::x264_malloc(::core::mem::size_of::<
                            x264_threadpool_job_t,
                        >()
                            as crate::stdlib::int64_t)
                            as *mut x264_threadpool_job_t;
                        if job.is_null() {
                            c2rust_current_block = 11983467789922532698;
                            break;
                        }
                        crate::src::common::frame::x264_8_sync_frame_list_push(
                            &raw mut (*pool).uninit as *mut _
                                as *mut crate::src::common::frame::x264_sync_frame_list_t,
                            job as *mut ::core::ffi::c_void
                                as *mut crate::src::common::frame::x264_frame_t
                                as *mut crate::src::common::frame::x264_frame,
                        );
                        i += 1;
                    }
                    match c2rust_current_block {
                        11983467789922532698 => {}
                        _ => {
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i_0 < (*pool).threads) {
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
                                _ => return 0 as ::core::ffi::c_int,
                            }
                        }
                    }
                }
            }
        }
        return -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_threadpool_run(
    mut pool: *mut x264_threadpool_t,
    mut func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut job: *mut x264_threadpool_job_t =
            crate::src::common::frame::x264_8_sync_frame_list_pop(
                &raw mut (*pool).uninit as *mut _
                    as *mut crate::src::common::frame::x264_sync_frame_list_t,
            ) as *mut crate::src::common::frame::x264_frame
                as *mut ::core::ffi::c_void as *mut x264_threadpool_job_t;
        (*job).func = func;
        (*job).arg = arg;
        crate::src::common::frame::x264_8_sync_frame_list_push(
            &raw mut (*pool).run as *mut _
                as *mut crate::src::common::frame::x264_sync_frame_list_t,
            job as *mut ::core::ffi::c_void as *mut crate::src::common::frame::x264_frame_t
                as *mut crate::src::common::frame::x264_frame,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_threadpool_wait(
    mut pool: *mut x264_threadpool_t,
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*pool).done.mutex);
        loop {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*pool).done.i_size {
                if (*(*(*pool).done.list.offset(i as isize) as *mut x264_threadpool_job_t)).arg
                    == arg
                {
                    let mut job: *mut x264_threadpool_job_t =
                        crate::src::common::frame::x264_8_frame_shift(
                            (*pool).done.list.offset(i as isize)
                                as *mut *mut crate::src::common::frame::x264_frame,
                        ) as *mut crate::src::common::frame::x264_frame
                            as *mut ::core::ffi::c_void
                            as *mut x264_threadpool_job_t;
                    (*pool).done.i_size -= 1;
                    crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).done.mutex);
                    let mut ret: *mut ::core::ffi::c_void = (*job).ret;
                    crate::src::common::frame::x264_8_sync_frame_list_push(
                        &raw mut (*pool).uninit as *mut _
                            as *mut crate::src::common::frame::x264_sync_frame_list_t,
                        job as *mut ::core::ffi::c_void
                            as *mut crate::src::common::frame::x264_frame_t
                            as *mut crate::src::common::frame::x264_frame,
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
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !(*(*slist).list.offset(i as isize)).is_null() {
            crate::src::common::base::x264_free(
                *(*slist).list.offset(i as isize) as *mut ::core::ffi::c_void
            );
            let ref mut c2rust_fresh0 = *(*slist).list.offset(i as isize);
            *c2rust_fresh0 = ::core::ptr::null_mut::<crate::src::common::frame::x264_frame_t>();
            i += 1;
        }
        crate::src::common::frame::x264_8_sync_frame_list_delete(
            slist as *mut crate::src::common::frame::x264_sync_frame_list_t,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_threadpool_delete(mut pool: *mut x264_threadpool_t) {
    unsafe {
        crate::stdlib::pthread_mutex_lock(&raw mut (*pool).run.mutex);
        ::core::ptr::write_volatile(
            &mut (*pool).exit as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        crate::stdlib::pthread_cond_broadcast(&raw mut (*pool).run.cv_fill);
        crate::stdlib::pthread_mutex_unlock(&raw mut (*pool).run.mutex);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
