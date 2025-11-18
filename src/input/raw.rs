use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{
    cli_input_opt_t, cli_input_t, cli_mmap_t, cli_pic_t, video_info_t, x264_cli_csp_depth_factor,
    x264_cli_csp_t, x264_cli_csps, x264_cli_get_csp, x264_cli_mmap, x264_cli_mmap_close,
    x264_cli_mmap_init, x264_cli_munmap, x264_cli_pic_alloc, x264_cli_pic_clean,
    x264_cli_pic_init_noalloc, x264_cli_pic_plane_size, X264_CSP_CLI_MAX,
};
use crate::osdep_h::x264_is_regular_file;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint16_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fopen, fread, fseeko, ftello, sscanf, stdin, SEEK_END, SEEK_SET};
use crate::stdlib_h::{calloc, free};
use crate::string_h::{memset, strcmp};
use crate::strings_h::strcasecmp;
use crate::types_h::__off64_t;
use crate::x264_h::{X264_CSP_HIGH_DEPTH, X264_CSP_I420, X264_CSP_NONE, X264_LOG_ERROR};
use crate::x264cli_h::{hnd_t, x264_cli_log};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "32:9"]
struct raw_hnd_t {
    fh: *mut FILE,
    next_frame: c_int,
    plane_size: [int64_t; 4],
    frame_size: int64_t,
    bit_depth: c_int,
    mmap: cli_mmap_t,
    use_mmap: c_int,
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> c_int {
    let mut h: *mut raw_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<raw_hnd_t>() as size_t) as *mut raw_hnd_t;
    if h.is_null() {
        return -(1 as c_int);
    }
    if (*opt).resolution.is_null() {
        let mut p: *mut c_char = psz_filename;
        while *p != 0 {
            if *p as c_int >= '0' as i32
                && *p as c_int <= '9' as i32
                && sscanf(
                    p,
                    b"%dx%d\0" as *const u8 as *const c_char,
                    &mut (*info).width as *mut c_int,
                    &mut (*info).height as *mut c_int,
                ) == 2 as c_int
            {
                break;
            }
            p = p.offset(1);
        }
    } else {
        sscanf(
            (*opt).resolution,
            b"%dx%d\0" as *const u8 as *const c_char,
            &mut (*info).width as *mut c_int,
            &mut (*info).height as *mut c_int,
        );
    }
    if (*info).width == 0 || (*info).height == 0 {
        x264_cli_log(
            b"raw\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"raw input requires a resolution.\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if !(*opt).colorspace.is_null() {
        (*info).csp = X264_CSP_CLI_MAX - 1 as c_int;
        while (*info).csp > X264_CSP_NONE {
            if !(*x264_cli_csps.as_ptr().offset((*info).csp as isize))
                .name
                .is_null()
                && strcasecmp(
                    (*x264_cli_csps.as_ptr().offset((*info).csp as isize)).name,
                    (*opt).colorspace,
                ) == 0
            {
                break;
            }
            (*info).csp -= 1;
        }
        if (*info).csp == 0 as c_int {
            x264_cli_log(
                b"raw\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"unsupported colorspace `%s'\n\0" as *const u8 as *const c_char,
                (*opt).colorspace,
            );
            return -(1 as c_int);
        }
    } else {
        (*info).csp = X264_CSP_I420;
    }
    (*h).bit_depth = (*opt).bit_depth;
    if (*h).bit_depth < 8 as c_int || (*h).bit_depth > 16 as c_int {
        x264_cli_log(
            b"raw\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"unsupported bit depth `%d'\n\0" as *const u8 as *const c_char,
            (*h).bit_depth,
        );
        return -(1 as c_int);
    }
    if (*h).bit_depth > 8 as c_int {
        (*info).csp |= X264_CSP_HIGH_DEPTH;
    }
    if strcmp(
        psz_filename,
        b"-\0" as *const u8 as *const c_char,
    ) == 0
    {
        (*h).fh = stdin;
    } else {
        (*h).fh = fopen(
            psz_filename,
            b"rb\0" as *const u8 as *const c_char,
        ) as *mut FILE;
    }
    if (*h).fh.is_null() {
        return -(1 as c_int);
    }
    (*info).thread_safe = 1 as c_int;
    (*info).num_frames = 0 as c_int;
    (*info).vfr = 0 as c_int;
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*info).csp);
    let mut i: c_int = 0 as c_int;
    while i < (*csp).planes {
        (*h).plane_size[i as usize] =
            x264_cli_pic_plane_size((*info).csp, (*info).width, (*info).height, i);
        (*h).frame_size += (*h).plane_size[i as usize];
        (*h).plane_size[i as usize] /= x264_cli_csp_depth_factor((*info).csp) as int64_t;
        i += 1;
    }
    if x264_is_regular_file((*h).fh) != 0 {
        fseeko((*h).fh, 0 as __off64_t, SEEK_END);
        let mut size: int64_t = ftello((*h).fh) as int64_t;
        fseeko((*h).fh, 0 as __off64_t, SEEK_SET);
        (*info).num_frames = (size / (*h).frame_size) as c_int;
        if (*info).num_frames == 0 {
            x264_cli_log(
                b"raw\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"empty input file\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        if (*h).bit_depth & 7 as c_int == 0 {
            (*h).use_mmap =
                (x264_cli_mmap_init(&mut (*h).mmap, (*h).fh) == 0) as c_int;
        }
    }
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn read_frame_internal(
    mut pic: *mut cli_pic_t,
    mut h: *mut raw_hnd_t,
    mut bit_depth_uc: c_int,
) -> c_int {
    let mut pixel_depth: c_int = x264_cli_csp_depth_factor((*pic).img.csp);
    let mut i: c_int = 0 as c_int;
    while i < (*pic).img.planes {
        if (*h).use_mmap != 0 {
            if i != 0 {
                (*pic).img.plane[i as usize] =
                    (*pic).img.plane[(i - 1 as c_int) as usize].offset(
                        (pixel_depth as int64_t
                            * (*h).plane_size[(i - 1 as c_int) as usize])
                            as isize,
                    );
            }
        } else if fread(
            (*pic).img.plane[i as usize] as *mut c_void,
            pixel_depth as size_t,
            (*h).plane_size[i as usize] as size_t,
            (*h).fh,
        ) as uint64_t
            != (*h).plane_size[i as usize] as uint64_t
        {
            return -(1 as c_int);
        }
        if bit_depth_uc != 0 {
            let mut plane: *mut uint16_t = (*pic).img.plane[i as usize] as *mut uint16_t;
            let mut pixel_count: int64_t = (*h).plane_size[i as usize];
            let mut lshift: c_int = 16 as c_int - (*h).bit_depth;
            let mut j: int64_t = 0 as int64_t;
            while j < pixel_count {
                *plane.offset(j as isize) =
                    ((*plane.offset(j as isize) as c_int) << lshift) as uint16_t;
                j += 1;
            }
        }
        i += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: c_int,
) -> c_int {
    let mut h: *mut raw_hnd_t = handle as *mut raw_hnd_t;
    if (*h).use_mmap != 0 {
        (*pic).img.plane[0 as c_int as usize] = x264_cli_mmap(
            &mut (*h).mmap,
            i_frame as int64_t * (*h).frame_size,
            (*h).frame_size,
        ) as *mut uint8_t;
        if (*pic).img.plane[0 as c_int as usize].is_null() {
            return -(1 as c_int);
        }
    } else if i_frame > (*h).next_frame {
        if x264_is_regular_file((*h).fh) != 0 {
            fseeko(
                (*h).fh,
                i_frame as __off64_t * (*h).frame_size as __off64_t,
                SEEK_SET,
            );
        } else {
            while i_frame > (*h).next_frame {
                if read_frame_internal(pic, h, 0 as c_int) != 0 {
                    return -(1 as c_int);
                }
                (*h).next_frame += 1;
            }
        }
    }
    if read_frame_internal(pic, h, (*h).bit_depth & 7 as c_int) != 0 {
        return -(1 as c_int);
    }
    (*h).next_frame = i_frame + 1 as c_int;
    return 0 as c_int;
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> c_int {
    let mut h: *mut raw_hnd_t = handle as *mut raw_hnd_t;
    if (*h).use_mmap != 0 {
        return x264_cli_munmap(
            &mut (*h).mmap,
            (*pic).img.plane[0 as c_int as usize] as *mut c_void,
            (*h).frame_size,
        );
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    let mut h: *mut raw_hnd_t = handle as *mut raw_hnd_t;
    return if (*h).use_mmap != 0 {
        Some(
            x264_cli_pic_init_noalloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    c_int,
                    c_int,
                    c_int,
                ) -> c_int,
        )
    } else {
        Some(
            x264_cli_pic_alloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    c_int,
                    c_int,
                    c_int,
                ) -> c_int,
        )
    }
    .expect("non-null function pointer")(pic, csp, width, height);
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut raw_hnd_t = handle as *mut raw_hnd_t;
    if (*h).use_mmap != 0 {
        memset(
            pic as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<cli_pic_t>() as size_t,
        );
    } else {
        x264_cli_pic_clean(pic);
    };
}
#[c2rust::src_loc = "194:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut raw_hnd_t = handle as *mut raw_hnd_t;
    if h.is_null() || (*h).fh.is_null() {
        return 0 as c_int;
    }
    if (*h).use_mmap != 0 {
        x264_cli_mmap_close(&mut (*h).mmap);
    }
    fclose((*h).fh);
    free(h as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "206:19"]
static mut raw_input: cli_input_t = cli_input_t {
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
        picture_alloc
            as unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                c_int,
                c_int,
                c_int,
            ) -> c_int,
    ),
    read_frame: Some(
        read_frame
            as unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                c_int,
            ) -> c_int,
    ),
    release_frame: Some(
        release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> c_int,
    ),
    picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
    close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> c_int),
};
