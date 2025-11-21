use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{cli_input, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t};
use crate::stdlib_h::{free, malloc};
use crate::threadpool_h::{
    x264_10_threadpool_delete, x264_10_threadpool_init, x264_10_threadpool_run,
    x264_10_threadpool_wait, x264_threadpool_t,
};
use crate::x264_h::X264_LOG_ERROR;
use crate::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "32:9"]
struct thread_hnd_t {
    input: cli_input_t,
    p_handle: hnd_t,
    pic: cli_pic_t,
    pool: *mut x264_threadpool_t,
    next_frame: c_int,
    frame_total: c_int,
    next_args: *mut thread_input_arg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:16"]
struct thread_input_arg_t {
    h: *mut thread_hnd_t,
    pic: *mut cli_pic_t,
    i_frame: c_int,
    status: c_int,
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn open_file(
    mut _psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut _opt: *mut cli_input_opt_t,
) -> c_int {
    let mut h: *mut thread_hnd_t =
        malloc(::core::mem::size_of::<thread_hnd_t>() as size_t) as *mut thread_hnd_t;
    if h.is_null()
        || cli_input.picture_alloc.expect("non-null function pointer")(
            &mut (*h).pic,
            *p_handle,
            (*info).csp,
            (*info).width as c_int,
            (*info).height as c_int,
        ) != 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).next_frame = -1;
    (*h).next_args =
        malloc(::core::mem::size_of::<thread_input_arg_t>() as size_t) as *mut thread_input_arg_t;
    if (*h).next_args.is_null() {
        return -1;
    }
    (*(*h).next_args).h = h;
    (*(*h).next_args).status = 0 as c_int;
    (*h).frame_total = (*info).num_frames;
    if x264_10_threadpool_init(&mut (*h).pool, 1 as c_int) != 0 {
        return -1;
    }
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn read_frame_thread_int(mut i: *mut thread_input_arg_t) {
    (*i).status =
        (*(*i).h)
            .input
            .read_frame
            .expect("non-null function pointer")((*i).pic, (*(*i).h).p_handle, (*i).i_frame);
}
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn read_frame(
    mut p_pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: c_int,
) -> c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    let mut ret: c_int = 0 as c_int;
    if (*h).next_frame >= 0 as c_int {
        x264_10_threadpool_wait((*h).pool, (*h).next_args as *mut c_void);
        ret |= (*(*h).next_args).status;
    }
    if (*h).next_frame == i_frame {
        let mut t: cli_pic_t = *p_pic;
        *p_pic = (*h).pic;
        (*h).pic = t;
    } else {
        if (*h).next_frame >= 0 as c_int {
            thread_10_input
                .release_frame
                .expect("non-null function pointer")(&mut (*h).pic, handle);
        }
        ret |= (*h).input.read_frame.expect("non-null function pointer")(
            p_pic,
            (*h).p_handle,
            i_frame,
        );
    }
    if (*h).frame_total == 0 || (i_frame + 1 as c_int) < (*h).frame_total {
        (*(*h).next_args).i_frame = i_frame + 1 as c_int;
        (*h).next_frame = (*(*h).next_args).i_frame;
        (*(*h).next_args).pic = &mut (*h).pic;
        x264_10_threadpool_run(
            (*h).pool,
            ::core::mem::transmute::<
                *mut c_void,
                Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
            >(::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut thread_input_arg_t) -> ()>,
                *mut c_void,
            >(Some(
                read_frame_thread_int as unsafe extern "C" fn(*mut thread_input_arg_t) -> (),
            ))),
            (*h).next_args as *mut c_void,
        );
    } else {
        (*h).next_frame = -1;
    }
    return ret;
}
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    if (*h).input.release_frame.is_some() {
        return (*h).input.release_frame.expect("non-null function pointer")(pic, (*h).p_handle);
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    return (*h).input.picture_alloc.expect("non-null function pointer")(
        pic,
        (*h).p_handle,
        csp,
        width,
        height,
    );
}
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    (*h).input.picture_clean.expect("non-null function pointer")(pic, (*h).p_handle);
}
#[c2rust::src_loc = "131:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    x264_10_threadpool_delete((*h).pool);
    (*h).input.picture_clean.expect("non-null function pointer")(&mut (*h).pic, (*h).p_handle);
    (*h).input.close_file.expect("non-null function pointer")((*h).p_handle);
    free((*h).next_args as *mut c_void);
    free(h as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "142:19"]
static mut thread_10_input: cli_input_t = cli_input_t {
    open_file: Some(
        open_file
            as unsafe extern "C" fn(
                *mut c_char,
                *mut hnd_t,
                *mut video_info_t,
                *mut cli_input_opt_t,
            ) -> c_int,
    ),
    picture_alloc: Some(
        picture_alloc as unsafe extern "C" fn(*mut cli_pic_t, hnd_t, c_int, c_int, c_int) -> c_int,
    ),
    read_frame: Some(read_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t, c_int) -> c_int),
    release_frame: Some(release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> c_int),
    picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
    close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> c_int),
};
