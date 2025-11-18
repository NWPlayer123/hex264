use ::core::ffi::{c_char, c_float, c_int, c_long, c_ulong, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_free, x264_malloc};
use crate::confname_h::_SC_PAGESIZE;
use crate::input_h::{cli_mmap_t, cli_pic_t, x264_cli_csp_t, X264_CSP_CLI_MAX, X264_CSP_OTHER};
use crate::mman_h::{madvise, mmap, munmap, MAP_FAILED};
use crate::mman_linux_h::{MADV_WILLNEED, MAP_FIXED, MAP_PRIVATE, PROT_READ};
use crate::osdep_h::NATIVE_ALIGN;
use crate::stat_h::fstat;
use crate::stdint_h::{intptr_t, SIZE_MAX};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint64_t, uint8_t};
use crate::stdio_h::fileno;
use crate::string_h::memset;
use crate::struct_stat_h::stat;
use crate::struct_timespec_h::timespec;
use crate::types_h::__off64_t;
use crate::unistd_h::sysconf;
use crate::x264_h::{X264_CSP_HIGH_DEPTH, X264_CSP_MASK, X264_CSP_NONE, X264_CSP_V210};
use crate::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "36:22"]
static mut x264_cli_csps: [x264_cli_csp_t; 17] = [
    x264_cli_csp_t {
        name: 0 as *const c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i400\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [1 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i420\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            mod_width: 2 as c_int,
            mod_height: 2 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv12\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            mod_width: 2 as c_int,
            mod_height: 2 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv12\0" as *const u8 as *const c_char,
            planes: 2 as c_int,
            width: [1 as c_int as c_float, 1 as c_int as c_float, 0., 0.],
            height: [1 as c_int as c_float, 0.5f64 as c_float, 0., 0.],
            mod_width: 2 as c_int,
            mod_height: 2 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv21\0" as *const u8 as *const c_char,
            planes: 2 as c_int,
            width: [1 as c_int as c_float, 1 as c_int as c_float, 0., 0.],
            height: [1 as c_int as c_float, 0.5f64 as c_float, 0., 0.],
            mod_width: 2 as c_int,
            mod_height: 2 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i422\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            mod_width: 2 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv16\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                0.5f64 as c_float,
                0.5f64 as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            mod_width: 2 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"nv16\0" as *const u8 as *const c_char,
            planes: 2 as c_int,
            width: [1 as c_int as c_float, 1 as c_int as c_float, 0., 0.],
            height: [1 as c_int as c_float, 1 as c_int as c_float, 0., 0.],
            mod_width: 2 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yuyv\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [2 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 2 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"uyvy\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [2 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 2 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    x264_cli_csp_t {
        name: 0 as *const c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"i444\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"yv24\0" as *const u8 as *const c_char,
            planes: 3 as c_int,
            width: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            height: [
                1 as c_int as c_float,
                1 as c_int as c_float,
                1 as c_int as c_float,
                0.,
            ],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"bgr\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [3 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"bgra\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [4 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
    {
        let mut init = x264_cli_csp_t {
            name: b"rgb\0" as *const u8 as *const c_char,
            planes: 1 as c_int,
            width: [3 as c_int as c_float, 0., 0., 0.],
            height: [1 as c_int as c_float, 0., 0., 0.],
            mod_width: 1 as c_int,
            mod_height: 1 as c_int,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn x264_cli_csp_is_invalid(mut csp: c_int) -> c_int {
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    return (csp_mask <= X264_CSP_NONE
        || csp_mask >= X264_CSP_CLI_MAX
        || csp_mask == X264_CSP_V210
        || csp & X264_CSP_OTHER != 0) as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn x264_cli_csp_depth_factor(mut csp: c_int) -> c_int {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as c_int;
    }
    return if csp & X264_CSP_HIGH_DEPTH != 0 {
        2 as c_int
    } else {
        1 as c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn x264_cli_pic_plane_size(
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
    mut plane: c_int,
) -> int64_t {
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid(csp) != 0
        || plane < 0 as c_int
        || plane >= x264_cli_csps[csp_mask as usize].planes
    {
        return 0 as int64_t;
    }
    let mut size: int64_t = width as int64_t * height as int64_t;
    size = (size as c_float
        * (x264_cli_csps[csp_mask as usize].width[plane as usize]
            * x264_cli_csps[csp_mask as usize].height[plane as usize])) as int64_t;
    size *= x264_cli_csp_depth_factor(csp) as int64_t;
    return size;
}
#[no_mangle]
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn x264_cli_pic_size(
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> int64_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as int64_t;
    }
    let mut size: int64_t = 0 as int64_t;
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    let mut i: c_int = 0 as c_int;
    while i < x264_cli_csps[csp_mask as usize].planes {
        size += x264_cli_pic_plane_size(csp, width, height, i);
        i += 1;
    }
    return size;
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn cli_pic_init_internal(
    mut pic: *mut cli_pic_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
    mut align: c_int,
    mut alloc: c_int,
) -> c_int {
    memset(
        pic as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid(csp) != 0 {
        (*pic).img.planes = 0 as c_int;
    } else {
        (*pic).img.planes = x264_cli_csps[csp_mask as usize].planes;
    }
    (*pic).img.csp = csp;
    (*pic).img.width = width;
    (*pic).img.height = height;
    let mut i: c_int = 0 as c_int;
    while i < (*pic).img.planes {
        let mut stride: c_int =
            (width as c_float * x264_cli_csps[csp_mask as usize].width[i as usize]) as c_int;
        stride *= x264_cli_csp_depth_factor(csp);
        stride = stride + (align - 1 as c_int) & !(align - 1 as c_int);
        (*pic).img.stride[i as usize] = stride;
        if alloc != 0 {
            let mut size: int64_t = (height as c_float
                * x264_cli_csps[csp_mask as usize].height[i as usize])
                as int64_t
                * stride as int64_t;
            (*pic).img.plane[i as usize] = x264_malloc(size) as *mut uint8_t;
            if (*pic).img.plane[i as usize].is_null() {
                return -(1 as c_int);
            }
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn x264_cli_pic_alloc(
    mut pic: *mut cli_pic_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    return cli_pic_init_internal(pic, csp, width, height, 1 as c_int, 1 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn x264_cli_pic_alloc_aligned(
    mut pic: *mut cli_pic_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    return cli_pic_init_internal(pic, csp, width, height, NATIVE_ALIGN, 1 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn x264_cli_pic_init_noalloc(
    mut pic: *mut cli_pic_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    return cli_pic_init_internal(pic, csp, width, height, 1 as c_int, 0 as c_int);
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
unsafe extern "C" fn x264_cli_pic_clean(mut pic: *mut cli_pic_t) {
    let mut i: c_int = 0 as c_int;
    while i < (*pic).img.planes {
        x264_free((*pic).img.plane[i as usize] as *mut c_void);
        i += 1;
    }
    memset(
        pic as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn x264_cli_get_csp(mut csp: c_int) -> *const x264_cli_csp_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as *const x264_cli_csp_t;
    }
    return x264_cli_csps
        .as_ptr()
        .offset((csp & X264_CSP_MASK) as isize);
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn x264_cli_mmap_init(mut h: *mut cli_mmap_t, mut fh: *mut FILE) -> c_int {
    let mut fd: c_int = fileno(fh);
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &mut file_stat) == 0 {
        (*h).file_size = file_stat.st_size as int64_t;
        (*h).align_mask = (sysconf(_SC_PAGESIZE as c_int) - 1 as c_long) as c_int;
        (*h).fd = fd;
        return ((*h).align_mask < 0 as c_int || fd < 0 as c_int) as c_int;
    }
    return -(1 as c_int);
}
#[c2rust::src_loc = "183:9"]
const MMAP_PADDING: c_int = 64 as c_int;
#[no_mangle]
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn x264_cli_mmap(
    mut h: *mut cli_mmap_t,
    mut offset: int64_t,
    mut size: int64_t,
) -> *mut c_void {
    let mut base: *mut uint8_t = 0 as *mut uint8_t;
    let mut align: c_int = (offset & (*h).align_mask as int64_t) as c_int;
    if offset < 0 as int64_t
        || size < 0 as int64_t
        || size as uint64_t
            > (SIZE_MAX as uint64_t)
                .wrapping_sub(MMAP_PADDING as uint64_t)
                .wrapping_sub(align as uint64_t)
    {
        return NULL;
    }
    offset -= align as int64_t;
    size += align as int64_t;
    let mut padded_size: size_t = (size + MMAP_PADDING as int64_t) as size_t;
    base = mmap(
        NULL,
        padded_size,
        PROT_READ,
        MAP_PRIVATE,
        (*h).fd,
        offset as __off64_t,
    ) as *mut uint8_t;
    if base != MAP_FAILED as *mut uint8_t {
        madvise(base as *mut c_void, size as size_t, MADV_WILLNEED);
        let mut aligned_size: size_t =
            padded_size.wrapping_sub(1 as size_t) & !(*h).align_mask as size_t;
        if (offset as size_t).wrapping_add(aligned_size) >= (*h).file_size as size_t {
            mmap(
                base.offset(aligned_size as isize) as *mut c_void,
                padded_size.wrapping_sub(aligned_size),
                PROT_READ,
                MAP_PRIVATE | MAP_FIXED,
                (*h).fd,
                offset as __off64_t + size as __off64_t - 1 as __off64_t
                    & !(*h).align_mask as __off64_t,
            );
        }
        return base.offset(align as isize) as *mut c_void;
    }
    return NULL;
}
#[no_mangle]
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn x264_cli_munmap(
    mut h: *mut cli_mmap_t,
    mut addr: *mut c_void,
    mut size: int64_t,
) -> c_int {
    let mut base: *mut c_void = (addr as intptr_t & !(*h).align_mask as intptr_t) as *mut c_void;
    if size < 0 as int64_t
        || size as c_ulong
            > SIZE_MAX
                .wrapping_sub(MMAP_PADDING as c_ulong)
                .wrapping_sub((addr as intptr_t - base as intptr_t) as c_ulong)
    {
        return -(1 as c_int);
    }
    return munmap(
        base,
        (size + MMAP_PADDING as int64_t + addr as int64_t - base as int64_t) as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn x264_cli_mmap_close(mut _h: *mut cli_mmap_t) {}
