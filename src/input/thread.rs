#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:27"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
        #[c2rust::src_loc = "33:10"]
        pub fn x264_10_threadpool_init(
            p_pool: *mut *mut x264_threadpool_t,
            threads: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "35:10"]
        pub fn x264_10_threadpool_run(
            pool: *mut x264_threadpool_t,
            func: Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            >,
            arg: *mut ::core::ffi::c_void,
        );
        #[c2rust::src_loc = "37:10"]
        pub fn x264_10_threadpool_wait(
            pool: *mut x264_threadpool_t,
            arg: *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "39:10"]
        pub fn x264_10_threadpool_delete(pool: *mut x264_threadpool_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:27"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_cli_log(
            name: *const ::core::ffi::c_char,
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:27"]
pub mod input_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:9"]
    pub struct cli_input_opt_t {
        pub index_file: *mut ::core::ffi::c_char,
        pub format: *mut ::core::ffi::c_char,
        pub resolution: *mut ::core::ffi::c_char,
        pub colorspace: *mut ::core::ffi::c_char,
        pub bit_depth: ::core::ffi::c_int,
        pub timebase: *mut ::core::ffi::c_char,
        pub seek: ::core::ffi::c_int,
        pub progress: ::core::ffi::c_int,
        pub output_csp: ::core::ffi::c_int,
        pub output_range: ::core::ffi::c_int,
        pub input_range: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct video_info_t {
        pub csp: ::core::ffi::c_int,
        pub fps_num: uint32_t,
        pub fps_den: uint32_t,
        pub fullrange: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub interlaced: ::core::ffi::c_int,
        pub num_frames: ::core::ffi::c_int,
        pub sar_width: uint32_t,
        pub sar_height: uint32_t,
        pub tff: ::core::ffi::c_int,
        pub thread_safe: ::core::ffi::c_int,
        pub timebase_num: uint32_t,
        pub timebase_den: uint32_t,
        pub vfr: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct cli_image_t {
        pub csp: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub planes: ::core::ffi::c_int,
        pub plane: [*mut uint8_t; 4],
        pub stride: [::core::ffi::c_int; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct cli_pic_t {
        pub img: cli_image_t,
        pub pts: int64_t,
        pub duration: int64_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:9"]
    pub struct cli_input_t {
        pub open_file: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                *mut hnd_t,
                *mut video_info_t,
                *mut cli_input_opt_t,
            ) -> ::core::ffi::c_int,
        >,
        pub picture_alloc: Option<
            unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub read_frame: Option<
            unsafe extern "C" fn(*mut cli_pic_t, hnd_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub release_frame:
            Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int>,
        pub picture_clean: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
        pub close_file: Option<unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int>,
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "111:20"]
        pub static mut cli_input: cli_input_t;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::input_h::{
    cli_image_t, cli_input, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t,
};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::stdlib_h::{free, malloc};
use self::threadpool_h::{
    x264_10_threadpool_delete, x264_10_threadpool_init, x264_10_threadpool_run,
    x264_10_threadpool_wait, x264_threadpool_t,
};
pub use self::types_h::{__int64_t, __uint32_t, __uint8_t};
pub use self::x264_h::X264_LOG_ERROR;
pub use self::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "32:9"]
pub struct thread_hnd_t {
    pub input: cli_input_t,
    pub p_handle: hnd_t,
    pub pic: cli_pic_t,
    pub pool: *mut x264_threadpool_t,
    pub next_frame: ::core::ffi::c_int,
    pub frame_total: ::core::ffi::c_int,
    pub next_args: *mut thread_input_arg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:16"]
pub struct thread_input_arg_t {
    pub h: *mut thread_hnd_t,
    pub pic: *mut cli_pic_t,
    pub i_frame: ::core::ffi::c_int,
    pub status: ::core::ffi::c_int,
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> ::core::ffi::c_int {
    let mut h: *mut thread_hnd_t =
        malloc(::core::mem::size_of::<thread_hnd_t>() as size_t) as *mut thread_hnd_t;
    if h.is_null()
        || cli_input.picture_alloc.expect("non-null function pointer")(
            &mut (*h).pic,
            *p_handle,
            (*info).csp,
            (*info).width,
            (*info).height,
        ) != 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).next_frame = -(1 as ::core::ffi::c_int);
    (*h).next_args =
        malloc(::core::mem::size_of::<thread_input_arg_t>() as size_t) as *mut thread_input_arg_t;
    if (*h).next_args.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*(*h).next_args).h = h;
    (*(*h).next_args).status = 0 as ::core::ffi::c_int;
    (*h).frame_total = (*info).num_frames;
    if x264_10_threadpool_init(&mut (*h).pool, 1 as ::core::ffi::c_int) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    *p_handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
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
    mut i_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).next_frame >= 0 as ::core::ffi::c_int {
        x264_10_threadpool_wait((*h).pool, (*h).next_args as *mut ::core::ffi::c_void);
        ret |= (*(*h).next_args).status;
    }
    if (*h).next_frame == i_frame {
        let mut t: cli_pic_t = *p_pic;
        *p_pic = (*h).pic;
        (*h).pic = t;
    } else {
        if (*h).next_frame >= 0 as ::core::ffi::c_int {
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
    if (*h).frame_total == 0 || (i_frame + 1 as ::core::ffi::c_int) < (*h).frame_total {
        (*(*h).next_args).i_frame = i_frame + 1 as ::core::ffi::c_int;
        (*h).next_frame = (*(*h).next_args).i_frame;
        (*(*h).next_args).pic = &mut (*h).pic;
        x264_10_threadpool_run(
            (*h).pool,
            ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
            >(::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut thread_input_arg_t) -> ()>,
                *mut ::core::ffi::c_void,
            >(Some(
                read_frame_thread_int as unsafe extern "C" fn(*mut thread_input_arg_t) -> (),
            ))),
            (*h).next_args as *mut ::core::ffi::c_void,
        );
    } else {
        (*h).next_frame = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> ::core::ffi::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    if (*h).input.release_frame.is_some() {
        return (*h).input.release_frame.expect("non-null function pointer")(pic, (*h).p_handle);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
unsafe extern "C" fn close_file(mut handle: hnd_t) -> ::core::ffi::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    x264_10_threadpool_delete((*h).pool);
    (*h).input.picture_clean.expect("non-null function pointer")(&mut (*h).pic, (*h).p_handle);
    (*h).input.close_file.expect("non-null function pointer")((*h).p_handle);
    free((*h).next_args as *mut ::core::ffi::c_void);
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "142:19"]
pub static mut thread_10_input: cli_input_t = unsafe {
    {
        let mut init = cli_input_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut hnd_t,
                        *mut video_info_t,
                        *mut cli_input_opt_t,
                    ) -> ::core::ffi::c_int,
            ),
            picture_alloc: Some(
                picture_alloc
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            read_frame: Some(
                read_frame
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            release_frame: Some(
                release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int,
            ),
            picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
            close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int),
        };
        init
    }
};
