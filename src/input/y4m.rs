use ::core::mem::size_of;
use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::x264_reduce_fraction;
use crate::input_h::{
    cli_input_opt_t, cli_input_t, cli_mmap_t, cli_pic_t, video_info_t, x264_cli_csp_depth_factor,
    x264_cli_csp_t, x264_cli_get_csp, x264_cli_mmap, x264_cli_mmap_close, x264_cli_mmap_init,
    x264_cli_munmap, x264_cli_pic_alloc, x264_cli_pic_clean, x264_cli_pic_init_noalloc,
    x264_cli_pic_plane_size,
};
use crate::osdep_h::x264_is_regular_file;
use crate::src::x264::x264_cli_log;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{
    fclose, fgetc, fopen, fread, fseeko, ftello, sscanf, stdin, SEEK_END, SEEK_SET,
};
use crate::stdlib_h::{calloc, free, strtol};
use crate::string_h::{memcmp, memset, strchr, strcmp, strncmp};
use crate::types_h::__off64_t;
use crate::x264_h::{
    X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MAX,
    X264_CSP_NONE, X264_LOG_ERROR,
};
use crate::x264cli_h::hnd_t;
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:9"]
struct y4m_hnd_t {
    fh: *mut FILE,
    next_frame: c_int,
    seq_header_len: c_int,
    frame_header_len: c_int,
    frame_size: int64_t,
    plane_size: [int64_t; 3],
    bit_depth: c_int,
    mmap: cli_mmap_t,
    use_mmap: c_int,
}
static mut slen: size_t = 0;
#[c2rust::src_loc = "44:9"]
const Y4M_MAGIC: [c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [c_char; 10]>(*b"YUV4MPEG2\0") };
#[c2rust::src_loc = "45:9"]
const Y4M_FRAME_MAGIC: [c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [c_char; 6]>(*b"FRAME\0") };
#[c2rust::src_loc = "46:9"]
const Y4M_MAX_HEADER: c_int = 256 as c_int;
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn parse_csp_and_depth(
    mut csp_name: *mut c_char,
    mut bit_depth: *mut c_int,
) -> c_int {
    let mut csp: c_int = X264_CSP_MAX;
    if strncmp(
        b"mono\0" as *const u8 as *const c_char,
        csp_name,
        4 as size_t,
    ) == 0
    {
        csp = X264_CSP_I400;
    } else if strncmp(
        b"420\0" as *const u8 as *const c_char,
        csp_name,
        3 as size_t,
    ) == 0
    {
        csp = X264_CSP_I420;
    } else if strncmp(
        b"422\0" as *const u8 as *const c_char,
        csp_name,
        3 as size_t,
    ) == 0
    {
        csp = X264_CSP_I422;
    } else if strncmp(
        b"444\0" as *const u8 as *const c_char,
        csp_name,
        3 as size_t,
    ) == 0
        && strncmp(
            b"444alpha\0" as *const u8 as *const c_char,
            csp_name,
            8 as size_t,
        ) != 0
    {
        csp = X264_CSP_I444;
    }
    if sscanf(
        csp_name,
        b"mono%d\0" as *const u8 as *const c_char,
        bit_depth,
    ) != 1 as c_int
        && sscanf(
            csp_name,
            b"%*d%*[pP]%d\0" as *const u8 as *const c_char,
            bit_depth,
        ) != 1 as c_int
    {
        *bit_depth = 8 as c_int;
    }
    return csp;
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut _opt: *mut cli_input_opt_t,
) -> c_int {
    let mut h: *mut y4m_hnd_t =
        calloc(1 as size_t, size_of::<y4m_hnd_t>() as size_t) as *mut y4m_hnd_t;
    let mut i: c_int = 0;
    let mut n: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut header: [c_char; 266] = [0; 266];
    let mut tokend: *mut c_char = 0 as *mut c_char;
    let mut header_end: *mut c_char = 0 as *mut c_char;
    let mut colorspace: c_int = X264_CSP_NONE;
    let mut alt_colorspace: c_int = X264_CSP_NONE;
    let mut alt_bit_depth: c_int = 8 as c_int;
    if h.is_null() {
        return -1;
    }
    (*info).vfr = false;
    if strcmp(psz_filename, b"-\0" as *const u8 as *const c_char) == 0 {
        (*h).fh = stdin;
    } else {
        (*h).fh = fopen(psz_filename, b"rb\0" as *const u8 as *const c_char) as *mut FILE;
    }
    if (*h).fh.is_null() {
        return -1;
    }
    i = 0 as c_int;
    while i < Y4M_MAX_HEADER {
        header[i as usize] = fgetc((*h).fh) as c_char;
        if header[i as usize] as c_int == '\n' as i32 {
            header[(i + 1 as c_int) as usize] = 0x20 as c_char;
            header[(i + 2 as c_int) as usize] = 0 as c_char;
            break;
        } else {
            i += 1;
        }
    }
    if strncmp(
        header.as_mut_ptr(),
        b"YUV4MPEG2\0" as *const u8 as *const c_char,
        (size_of::<[c_char; 10]>() as size_t).wrapping_sub(1 as size_t),
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"bad sequence header magic\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if i == 256 as c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"bad sequence header length\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    header_end = &mut *header.as_mut_ptr().offset((i + 1 as c_int) as isize) as *mut c_char;
    (*h).seq_header_len = i + 1 as c_int;
    let mut tokstart: *mut c_char = header
        .as_mut_ptr()
        .offset(size_of::<[c_char; 10]>() as usize as isize);
    while tokstart < header_end {
        if !(*tokstart as c_int == 0x20 as c_int) {
            let fresh0 = tokstart;
            tokstart = tokstart.offset(1);
            match *fresh0 as c_int {
                87 => {
                    (*info).width = strtol(tokstart, &mut tokend, 10 as c_int) as u32;
                    tokstart = tokend;
                }
                72 => {
                    (*info).height = strtol(tokstart, &mut tokend, 10 as c_int) as u32;
                    tokstart = tokend;
                }
                67 => {
                    colorspace = parse_csp_and_depth(tokstart, &mut (*h).bit_depth);
                    tokstart = strchr(tokstart, 0x20 as c_int);
                }
                73 => {
                    let fresh1 = tokstart;
                    tokstart = tokstart.offset(1);
                    match *fresh1 as c_int {
                        116 => {
                            (*info).interlaced = true;
                            (*info).tff = true;
                        }
                        98 => {
                            (*info).interlaced = true;
                            (*info).tff = false;
                        }
                        109 => {
                            (*info).interlaced = true;
                        }
                        _ => {}
                    }
                }
                70 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as c_int
                        && n != 0
                        && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).fps_num = n;
                        (*info).fps_den = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as c_int);
                }
                65 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as c_int
                        && n != 0
                        && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).sar_width = n;
                        (*info).sar_height = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as c_int);
                }
                88 => {
                    if strncmp(
                        b"YSCSS=\0" as *const u8 as *const c_char,
                        tokstart,
                        6 as size_t,
                    ) == 0
                    {
                        tokstart = tokstart.offset(6);
                        alt_colorspace = parse_csp_and_depth(tokstart, &mut alt_bit_depth);
                    } else if strncmp(
                        b"COLORRANGE=\0" as *const u8 as *const c_char,
                        tokstart,
                        11 as size_t,
                    ) == 0
                    {
                        tokstart = tokstart.offset(11 as c_int as isize);
                        if strncmp(
                            b"FULL\0" as *const u8 as *const c_char,
                            tokstart,
                            4 as size_t,
                        ) == 0
                        {
                            (*info).fullrange = 1 as c_int;
                        } else if strncmp(
                            b"LIMITED\0" as *const u8 as *const c_char,
                            tokstart,
                            7 as size_t,
                        ) == 0
                        {
                            (*info).fullrange = 0 as c_int;
                        }
                    }
                    tokstart = strchr(tokstart, 0x20 as c_int);
                }
                _ => {}
            }
        }
        tokstart = tokstart.offset(1);
    }
    if colorspace == X264_CSP_NONE {
        colorspace = alt_colorspace;
        (*h).bit_depth = alt_bit_depth;
    }
    if colorspace == X264_CSP_NONE {
        colorspace = X264_CSP_I420;
        (*h).bit_depth = 8 as c_int;
    }
    if colorspace <= 0 as c_int || colorspace >= 0x11 as c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"colorspace unhandled\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if (*h).bit_depth < 8 as c_int || (*h).bit_depth > 16 as c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"unsupported bit depth `%d'\n\0" as *const u8 as *const c_char,
            (*h).bit_depth,
        );
        return -1;
    }
    (*info).thread_safe = 1 as c_int;
    (*info).num_frames = 0 as c_int;
    (*info).csp = colorspace;
    if (*h).bit_depth > 8 as c_int {
        (*info).csp |= X264_CSP_HIGH_DEPTH;
    }
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*info).csp);
    i = 0 as c_int;
    while i < (*csp).planes {
        (*h).plane_size[i as usize] = x264_cli_pic_plane_size(
            (*info).csp,
            (*info).width as c_int,
            (*info).height as c_int,
            i,
        );
        (*h).frame_size += (*h).plane_size[i as usize];
        (*h).plane_size[i as usize] /= x264_cli_csp_depth_factor((*info).csp) as int64_t;
        i += 1;
    }
    if x264_is_regular_file((*h).fh) != 0 {
        let mut init_pos: int64_t = ftello((*h).fh) as int64_t;
        let mut len: size_t = 1 as size_t;
        while len <= Y4M_MAX_HEADER as size_t && fgetc((*h).fh) != '\n' as i32 {
            len = len.wrapping_add(1);
        }
        if len > 256 as size_t || len < size_of::<[c_char; 6]>() as usize {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        (*h).frame_header_len = len as c_int;
        (*h).frame_size = ((*h).frame_size as size_t).wrapping_add(len) as int64_t as int64_t;
        fseeko((*h).fh, 0 as __off64_t, SEEK_END);
        let mut i_size: int64_t = ftello((*h).fh) as int64_t;
        fseeko((*h).fh, init_pos as __off64_t, SEEK_SET);
        (*info).num_frames = ((i_size - (*h).seq_header_len as int64_t) / (*h).frame_size) as c_int;
        if (*info).num_frames == 0 {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"empty input file\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        if (*h).bit_depth & 7 as c_int == 0 {
            (*h).use_mmap = (x264_cli_mmap_init(&mut (*h).mmap, (*h).fh) == 0) as c_int;
        }
    }
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "249:1"]
unsafe extern "C" fn read_frame_internal(
    mut pic: *mut cli_pic_t,
    mut h: *mut y4m_hnd_t,
    mut bit_depth_uc: c_int,
) -> c_int {
    let mut pixel_depth: c_int = x264_cli_csp_depth_factor((*pic).img.csp);
    let mut i: c_int = size_of::<[c_char; 6]>() as c_int;
    let mut header_buf: [c_char; 16] = [0; 16];
    let mut header: *mut c_char = 0 as *mut c_char;
    if (*h).use_mmap != 0 {
        header = (*pic).img.plane[0] as *mut c_char;
        (*pic).img.plane[0] = (*pic).img.plane[0].offset((*h).frame_header_len as isize);
        while i <= (*h).frame_header_len
            && *header.offset((i - 1 as c_int) as isize) as c_int != '\n' as i32
        {
            i += 1;
        }
        if i != (*h).frame_header_len {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
    } else {
        header = header_buf.as_mut_ptr();
        if fread(header as *mut c_void, 1 as size_t, slen, (*h).fh) as size_t != slen {
            return -1;
        }
        while i <= Y4M_MAX_HEADER && fgetc((*h).fh) != '\n' as i32 {
            i += 1;
        }
        if i > 256 as c_int {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"bad frame header length\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
    }
    if memcmp(
        header as *const c_void,
        b"FRAME\0" as *const u8 as *const c_char as *const c_void,
        slen,
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"bad frame header magic\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    i = 0 as c_int;
    while i < (*pic).img.planes {
        if (*h).use_mmap != 0 {
            if i != 0 {
                (*pic).img.plane[i as usize] = (*pic).img.plane[(i - 1 as c_int) as usize].offset(
                    (pixel_depth as int64_t * (*h).plane_size[(i - 1 as c_int) as usize]) as isize,
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
            return -1;
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
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: c_int,
) -> c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        (*pic).img.plane[0] = x264_cli_mmap(
            &mut (*h).mmap,
            (*h).frame_size * i_frame as int64_t + (*h).seq_header_len as int64_t,
            (*h).frame_size,
        ) as *mut uint8_t;
        if (*pic).img.plane[0].is_null() {
            return -1;
        }
    } else if i_frame > (*h).next_frame {
        if x264_is_regular_file((*h).fh) != 0 {
            fseeko(
                (*h).fh,
                (*h).frame_size as __off64_t * i_frame as __off64_t
                    + (*h).seq_header_len as __off64_t,
                SEEK_SET,
            );
        } else {
            while i_frame > (*h).next_frame {
                if read_frame_internal(pic, h, 0 as c_int) != 0 {
                    return -1;
                }
                (*h).next_frame += 1;
            }
        }
    }
    if read_frame_internal(pic, h, (*h).bit_depth & 7 as c_int) != 0 {
        return -1;
    }
    (*h).next_frame = i_frame + 1 as c_int;
    return 0 as c_int;
}
#[c2rust::src_loc = "335:1"]
unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        return x264_cli_munmap(
            &mut (*h).mmap,
            (*pic).img.plane[0].offset(-((*h).frame_header_len as isize)) as *mut c_void,
            (*h).frame_size,
        );
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "343:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    return if (*h).use_mmap != 0 {
        Some(
            x264_cli_pic_init_noalloc
                as unsafe extern "C" fn(*mut cli_pic_t, c_int, c_int, c_int) -> c_int,
        )
    } else {
        Some(
            x264_cli_pic_alloc
                as unsafe extern "C" fn(*mut cli_pic_t, c_int, c_int, c_int) -> c_int,
        )
    }
    .expect("non-null function pointer")(pic, csp, width, height);
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        memset(
            pic as *mut c_void,
            0 as c_int,
            size_of::<cli_pic_t>() as size_t,
        );
    } else {
        x264_cli_pic_clean(pic);
    };
}
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
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
#[c2rust::src_loc = "370:19"]
static mut y4m_input: cli_input_t = cli_input_t {
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
unsafe extern "C" fn run_static_initializers() {
    slen = (size_of::<[c_char; 6]>() as size_t).wrapping_sub(1 as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
