use ::core::mem::size_of;
use core::ffi::{c_char, c_double, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{cli_input, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t};
use crate::mathcalls_h::{fabs, floor, log10, pow, round};
use crate::osdep_h::x264_is_regular_file;
use crate::src::x264::x264_cli_log;
use crate::stdint_h::UINT32_MAX;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint64_t};
use crate::stdio_h::{fclose, fgets, fopen, fseeko, ftello, sscanf, SEEK_SET};
use crate::stdlib_h::{free, malloc, strtoul};
use crate::types_h::__off64_t;
use crate::x264_h::{X264_LOG_ERROR, X264_LOG_INFO};
use crate::x264cli_h::{gcd, hnd_t, lcm};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "30:9"]
struct timecode_hnd_t {
    input: cli_input_t,
    p_handle: hnd_t,
    auto_timebase_num: c_int,
    auto_timebase_den: c_int,
    timebase_num: uint64_t,
    timebase_den: uint64_t,
    stored_pts_num: c_int,
    pts: *mut int64_t,
    assume_fps: c_double,
    last_timecode: c_double,
}
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn sigexp10(mut value: c_double, mut exponent: *mut c_double) -> c_double {
    *exponent = pow(10 as c_int as c_double, floor(log10(value)));
    return value / *exponent;
}
#[c2rust::src_loc = "51:9"]
const DOUBLE_EPSILON: c_double = 5e-6f64;
#[c2rust::src_loc = "52:9"]
const MKV_TIMEBASE_DEN: c_int = 1000000000 as c_int;
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn correct_fps(mut fps: c_double, mut h: *mut timecode_hnd_t) -> c_double {
    let mut i: c_int = 1 as c_int;
    let mut fps_num: uint64_t = 0;
    let mut fps_den: uint64_t = 0;
    let mut exponent: c_double = 0.;
    let mut fps_sig: c_double = sigexp10(fps, &mut exponent);
    loop {
        fps_den = (i as uint64_t).wrapping_mul((*h).timebase_num);
        fps_num = (round(fps_den as c_double * fps_sig) * exponent) as uint64_t;
        if fps_num > 4294967295 as uint64_t {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"tcfile fps correction failed.\n                  Specify an appropriate timebase manually or remake tcfile.\n\0"
                    as *const u8 as *const c_char,
            );
            return -1 as c_double;
        }
        if fabs(fps_num as c_double / fps_den as c_double / exponent - fps_sig) < DOUBLE_EPSILON {
            break;
        }
        i += 1;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = if (*h).timebase_den != 0 {
            lcm((*h).timebase_den, fps_num)
        } else {
            fps_num
        };
        if (*h).timebase_den > UINT32_MAX as uint64_t {
            (*h).auto_timebase_den = 0 as c_int;
        }
    }
    return fps_num as c_double / fps_den as c_double;
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn try_mkv_timebase_den(
    mut fpss: *mut c_double,
    mut h: *mut timecode_hnd_t,
    mut loop_num: c_int,
) -> c_int {
    (*h).timebase_num = 0 as uint64_t;
    (*h).timebase_den = MKV_TIMEBASE_DEN as uint64_t;
    let mut num: c_int = 0 as c_int;
    while num < loop_num {
        let mut fps_den: uint64_t = 0;
        let mut exponent: c_double = 0.;
        let mut fps_sig: c_double = sigexp10(*fpss.offset(num as isize), &mut exponent);
        fps_den = (round(MKV_TIMEBASE_DEN as c_double / fps_sig) / exponent) as uint64_t;
        (*h).timebase_num = if fps_den != 0 && (*h).timebase_num != 0 {
            gcd((*h).timebase_num, fps_den)
        } else {
            fps_den
        };
        if (*h).timebase_num > 4294967295 as uint64_t || (*h).timebase_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"automatic timebase generation failed.\n                  Specify timebase manually.\n\0"
                    as *const u8 as *const c_char,
            );
            return -1;
        }
        num += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn parse_tcfile(
    mut tcfile_in: *mut FILE,
    mut h: *mut timecode_hnd_t,
    mut info: *mut video_info_t,
) -> c_int {
    let mut current_block: u64;
    let mut buff: [c_char; 256] = [0; 256];
    let mut ret: c_int = 0;
    let mut tcfv: c_int = 0;
    let mut num: c_int = 0;
    let mut seq_num: c_int = 0;
    let mut timecodes_num: c_int = 0;
    let mut timecodes: *mut c_double = 0 as *mut c_double;
    let mut fpss: *mut c_double = 0 as *mut c_double;
    ret = (!fgets(
        buff.as_mut_ptr(),
        size_of::<[c_char; 256]>() as c_int,
        tcfile_in,
    )
    .is_null()
        && (sscanf(
            buff.as_mut_ptr(),
            b"# timecode format v%d\0" as *const u8 as *const c_char,
            &mut tcfv as *mut c_int,
        ) == 1 as c_int
            || sscanf(
                buff.as_mut_ptr(),
                b"# timestamp format v%d\0" as *const u8 as *const c_char,
                &mut tcfv as *mut c_int,
            ) == 1 as c_int)) as c_int;
    if ret == 0 || tcfv != 1 as c_int && tcfv != 2 as c_int {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"unsupported timecode format\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if tcfv == 1 as c_int {
        let mut file_pos: int64_t = 0;
        let mut assume_fps: c_double = 0.;
        let mut seq_fps: c_double = 0.;
        let mut start: c_int = 0;
        let mut end: c_int = -1;
        let mut prev_start: c_int = -1;
        let mut prev_end: c_int = -1;
        (*h).assume_fps = 0 as c_int as c_double;
        num = 2 as c_int;
        while !fgets(
            buff.as_mut_ptr(),
            size_of::<[c_char; 256]>() as c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0] as c_int == '#' as i32
                || buff[0] as c_int == '\n' as i32
                || buff[0] as c_int == '\r' as i32
            {
                num += 1;
            } else {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"assume %lf\0" as *const u8 as *const c_char,
                    &mut (*h).assume_fps as *mut c_double,
                ) != 1 as c_int
                    && sscanf(
                        buff.as_mut_ptr(),
                        b"Assume %lf\0" as *const u8 as *const c_char,
                        &mut (*h).assume_fps as *mut c_double,
                    ) != 1 as c_int
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const c_char,
                        X264_LOG_ERROR,
                        b"tcfile parsing error: assumed fps not found\n\0" as *const u8
                            as *const c_char,
                    );
                    return -1;
                }
                break;
            }
        }
        if (*h).assume_fps <= 0 as c_int as c_double {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"invalid assumed fps %.6f\n\0" as *const u8 as *const c_char,
                (*h).assume_fps,
            );
            return -1;
        }
        file_pos = ftello(tcfile_in) as int64_t;
        (*h).stored_pts_num = 0 as c_int;
        seq_num = 0 as c_int;
        while !fgets(
            buff.as_mut_ptr(),
            size_of::<[c_char; 256]>() as c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0] as c_int == '#' as i32
                || buff[0] as c_int == '\n' as i32
                || buff[0] as c_int == '\r' as i32
            {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"# TDecimate Mode 3:  Last Frame = %d\0" as *const u8 as *const c_char,
                    &mut end as *mut c_int,
                ) == 1 as c_int
                {
                    (*h).stored_pts_num = end + 1 as c_int;
                }
            } else {
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%d,%d,%lf\0" as *const u8 as *const c_char,
                    &mut start as *mut c_int,
                    &mut end as *mut c_int,
                    &mut seq_fps as *mut c_double,
                );
                if ret != 3 as c_int && ret != -1 {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile\n\0" as *const u8 as *const c_char,
                    );
                    return -1;
                }
                if start > end
                    || start <= prev_start
                    || end <= prev_end
                    || seq_fps <= 0 as c_int as c_double
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile at line %d: %s\n\0" as *const u8 as *const c_char,
                        num,
                        buff.as_mut_ptr(),
                    );
                    return -1;
                }
                prev_start = start;
                prev_end = end;
                if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                    seq_num += 1;
                }
            }
            num += 1;
        }
        if (*h).stored_pts_num == 0 {
            (*h).stored_pts_num = end + 2 as c_int;
        }
        timecodes_num = (*h).stored_pts_num;
        fseeko(tcfile_in, file_pos as __off64_t, SEEK_SET);
        timecodes = malloc((timecodes_num as size_t).wrapping_mul(size_of::<c_double>() as size_t))
            as *mut c_double;
        if timecodes.is_null() {
            return -1;
        }
        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
            fpss = malloc(
                ((seq_num + 1 as c_int) as size_t).wrapping_mul(size_of::<c_double>() as size_t),
            ) as *mut c_double;
            if fpss.is_null() {
                current_block = 15792084957793291482;
            } else {
                current_block = 13678349939556791712;
            }
        } else {
            current_block = 13678349939556791712;
        }
        match current_block {
            15792084957793291482 => {}
            _ => {
                assume_fps = correct_fps((*h).assume_fps, h);
                if assume_fps < 0 as c_int as c_double {
                    current_block = 15792084957793291482;
                } else {
                    *timecodes.offset(0) = 0 as c_int as c_double;
                    seq_num = 0 as c_int;
                    num = seq_num;
                    loop {
                        if !(num < timecodes_num - 1 as c_int
                            && !fgets(
                                buff.as_mut_ptr(),
                                size_of::<[c_char; 256]>() as c_int,
                                tcfile_in,
                            )
                            .is_null())
                        {
                            current_block = 13660591889533726445;
                            break;
                        }
                        if buff[0] as c_int == '#' as i32
                            || buff[0] as c_int == '\n' as i32
                            || buff[0] as c_int == '\r' as i32
                        {
                            continue;
                        }
                        ret = sscanf(
                            buff.as_mut_ptr(),
                            b"%d,%d,%lf\0" as *const u8 as *const c_char,
                            &mut start as *mut c_int,
                            &mut end as *mut c_int,
                            &mut seq_fps as *mut c_double,
                        );
                        if ret != 3 as c_int {
                            end = timecodes_num - 1 as c_int;
                            start = end;
                        }
                        while num < start && num < timecodes_num - 1 as c_int {
                            *timecodes.offset((num + 1 as c_int) as isize) = *timecodes
                                .offset(num as isize)
                                + 1 as c_int as c_double / assume_fps;
                            num += 1;
                        }
                        if !(num < timecodes_num - 1 as c_int) {
                            continue;
                        }
                        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                            let fresh0 = seq_num;
                            seq_num = seq_num + 1;
                            *fpss.offset(fresh0 as isize) = seq_fps;
                        }
                        seq_fps = correct_fps(seq_fps, h);
                        if seq_fps < 0 as c_int as c_double {
                            current_block = 15792084957793291482;
                            break;
                        }
                        num = start;
                        while num <= end && num < timecodes_num - 1 as c_int {
                            *timecodes.offset((num + 1 as c_int) as isize) =
                                *timecodes.offset(num as isize) + 1 as c_int as c_double / seq_fps;
                            num += 1;
                        }
                    }
                    match current_block {
                        15792084957793291482 => {}
                        _ => {
                            while num < timecodes_num - 1 as c_int {
                                *timecodes.offset((num + 1 as c_int) as isize) = *timecodes
                                    .offset(num as isize)
                                    + 1 as c_int as c_double / assume_fps;
                                num += 1;
                            }
                            if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                                *fpss.offset(seq_num as isize) = (*h).assume_fps;
                            }
                            if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                                let mut exponent: c_double = 0.;
                                let mut assume_fps_sig: c_double = 0.;
                                let mut seq_fps_sig: c_double = 0.;
                                if try_mkv_timebase_den(fpss, h, seq_num + 1 as c_int) < 0 as c_int
                                {
                                    current_block = 15792084957793291482;
                                } else {
                                    fseeko(tcfile_in, file_pos as __off64_t, SEEK_SET);
                                    assume_fps_sig = sigexp10((*h).assume_fps, &mut exponent);
                                    assume_fps = MKV_TIMEBASE_DEN as c_double
                                        / (round(MKV_TIMEBASE_DEN as c_double / assume_fps_sig)
                                            / exponent);
                                    num = 0 as c_int;
                                    while num < timecodes_num - 1 as c_int
                                        && !fgets(
                                            buff.as_mut_ptr(),
                                            size_of::<[c_char; 256]>() as c_int,
                                            tcfile_in,
                                        )
                                        .is_null()
                                    {
                                        if buff[0] as c_int == '#' as i32
                                            || buff[0] as c_int == '\n' as i32
                                            || buff[0] as c_int == '\r' as i32
                                        {
                                            continue;
                                        }
                                        ret = sscanf(
                                            buff.as_mut_ptr(),
                                            b"%d,%d,%lf\0" as *const u8 as *const c_char,
                                            &mut start as *mut c_int,
                                            &mut end as *mut c_int,
                                            &mut seq_fps as *mut c_double,
                                        );
                                        if ret != 3 as c_int {
                                            end = timecodes_num - 1 as c_int;
                                            start = end;
                                        }
                                        seq_fps_sig = sigexp10(seq_fps, &mut exponent);
                                        seq_fps = MKV_TIMEBASE_DEN as c_double
                                            / (round(MKV_TIMEBASE_DEN as c_double / seq_fps_sig)
                                                / exponent);
                                        while num < start && num < timecodes_num - 1 as c_int {
                                            *timecodes.offset((num + 1 as c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as c_int as c_double / assume_fps;
                                            num += 1;
                                        }
                                        num = start;
                                        while num <= end && num < timecodes_num - 1 as c_int {
                                            *timecodes.offset((num + 1 as c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as c_int as c_double / seq_fps;
                                            num += 1;
                                        }
                                    }
                                    while num < timecodes_num - 1 as c_int {
                                        *timecodes.offset((num + 1 as c_int) as isize) = *timecodes
                                            .offset(num as isize)
                                            + 1 as c_int as c_double / assume_fps;
                                        num += 1;
                                    }
                                    current_block = 496303045384785551;
                                }
                            } else {
                                current_block = 496303045384785551;
                            }
                            match current_block {
                                15792084957793291482 => {}
                                _ => {
                                    if !fpss.is_null() {
                                        free(fpss as *mut c_void);
                                        fpss = 0 as *mut c_double;
                                    }
                                    (*h).assume_fps = assume_fps;
                                    (*h).last_timecode =
                                        *timecodes.offset((timecodes_num - 1 as c_int) as isize);
                                    current_block = 12129449210080749085;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut file_pos_0: int64_t = ftello(tcfile_in) as int64_t;
        (*h).stored_pts_num = 0 as c_int;
        while !fgets(
            buff.as_mut_ptr(),
            size_of::<[c_char; 256]>() as c_int,
            tcfile_in,
        )
        .is_null()
        {
            if buff[0] as c_int == '#' as i32
                || buff[0] as c_int == '\n' as i32
                || buff[0] as c_int == '\r' as i32
            {
                if (*h).stored_pts_num == 0 {
                    file_pos_0 = ftello(tcfile_in) as int64_t;
                }
            } else {
                (*h).stored_pts_num += 1;
            }
        }
        timecodes_num = (*h).stored_pts_num;
        if timecodes_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"input tcfile doesn't have any timecodes!\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        fseeko(tcfile_in, file_pos_0 as __off64_t, SEEK_SET);
        timecodes = malloc((timecodes_num as size_t).wrapping_mul(size_of::<c_double>() as size_t))
            as *mut c_double;
        if timecodes.is_null() {
            return -1;
        }
        num = 0 as c_int;
        if !fgets(
            buff.as_mut_ptr(),
            size_of::<[c_char; 256]>() as c_int,
            tcfile_in,
        )
        .is_null()
        {
            ret = sscanf(
                buff.as_mut_ptr(),
                b"%lf\0" as *const u8 as *const c_char,
                &mut *timecodes.offset(0) as *mut c_double,
            );
            *timecodes.offset(0) *= 1e-3f64;
            if ret != 1 as c_int {
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid input tcfile for frame 0\n\0" as *const u8 as *const c_char,
                );
                return -1;
            }
            num = 1 as c_int;
            while num < timecodes_num
                && !fgets(
                    buff.as_mut_ptr(),
                    size_of::<[c_char; 256]>() as c_int,
                    tcfile_in,
                )
                .is_null()
            {
                if buff[0] as c_int == '#' as i32
                    || buff[0] as c_int == '\n' as i32
                    || buff[0] as c_int == '\r' as i32
                {
                    continue;
                }
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%lf\0" as *const u8 as *const c_char,
                    &mut *timecodes.offset(num as isize) as *mut c_double,
                );
                *timecodes.offset(num as isize) *= 1e-3f64;
                if ret != 1 as c_int
                    || *timecodes.offset(num as isize)
                        <= *timecodes.offset((num - 1 as c_int) as isize)
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const c_char,
                        X264_LOG_ERROR,
                        b"invalid input tcfile for frame %d\n\0" as *const u8 as *const c_char,
                        num,
                    );
                    return -1;
                }
                num += 1;
            }
        }
        if num < timecodes_num {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"failed to read input tcfile for frame %d\0" as *const u8 as *const c_char,
                num,
            );
            return -1;
        }
        if timecodes_num == 1 as c_int {
            (*h).timebase_den = (*info).fps_num as uint64_t;
            current_block = 6938158527927677584;
        } else if (*h).auto_timebase_den != 0 {
            fpss = malloc(
                ((timecodes_num - 1 as c_int) as size_t)
                    .wrapping_mul(size_of::<c_double>() as size_t),
            ) as *mut c_double;
            if fpss.is_null() {
                current_block = 15792084957793291482;
            } else {
                num = 0 as c_int;
                while num < timecodes_num - 1 as c_int {
                    *fpss.offset(num as isize) = 1 as c_int as c_double
                        / (*timecodes.offset((num + 1 as c_int) as isize)
                            - *timecodes.offset(num as isize));
                    if (*h).auto_timebase_den != 0 {
                        let mut i: c_int = 1 as c_int;
                        let mut fps_num: uint64_t = 0;
                        let mut fps_den: uint64_t = 0;
                        let mut exponent_0: c_double = 0.;
                        let mut fps_sig: c_double =
                            sigexp10(*fpss.offset(num as isize), &mut exponent_0);
                        loop {
                            fps_den = (i as uint64_t).wrapping_mul((*h).timebase_num);
                            fps_num =
                                (round(fps_den as c_double * fps_sig) * exponent_0) as uint64_t;
                            if fps_num > UINT32_MAX as uint64_t
                                || fabs(
                                    fps_num as c_double / fps_den as c_double / exponent_0
                                        - fps_sig,
                                ) < DOUBLE_EPSILON
                            {
                                break;
                            }
                            i += 1;
                        }
                        (*h).timebase_den = if fps_num != 0 && (*h).timebase_den != 0 {
                            lcm((*h).timebase_den, fps_num)
                        } else {
                            fps_num
                        };
                        if (*h).timebase_den > UINT32_MAX as uint64_t {
                            (*h).auto_timebase_den = 0 as c_int;
                        }
                    }
                    num += 1;
                }
                if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                    if try_mkv_timebase_den(fpss, h, timecodes_num - 1 as c_int) < 0 as c_int {
                        current_block = 15792084957793291482;
                    } else {
                        current_block = 10783567741412653655;
                    }
                } else {
                    current_block = 10783567741412653655;
                }
                match current_block {
                    15792084957793291482 => {}
                    _ => {
                        free(fpss as *mut c_void);
                        fpss = 0 as *mut c_double;
                        current_block = 6938158527927677584;
                    }
                }
            }
        } else {
            current_block = 6938158527927677584;
        }
        match current_block {
            15792084957793291482 => {}
            _ => {
                if timecodes_num > 1 as c_int {
                    (*h).assume_fps = 1 as c_int as c_double
                        / (*timecodes.offset((timecodes_num - 1 as c_int) as isize)
                            - *timecodes.offset((timecodes_num - 2 as c_int) as isize));
                } else {
                    (*h).assume_fps = (*info).fps_num as c_double / (*info).fps_den as c_double;
                }
                (*h).last_timecode = *timecodes.offset((timecodes_num - 1 as c_int) as isize);
                current_block = 12129449210080749085;
            }
        }
    }
    match current_block {
        12129449210080749085 => {
            if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                let mut i_0: uint64_t = gcd((*h).timebase_num, (*h).timebase_den);
                (*h).timebase_num = (*h).timebase_num.wrapping_div(i_0);
                (*h).timebase_den = (*h).timebase_den.wrapping_div(i_0);
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const c_char,
                    X264_LOG_INFO,
                    b"automatic timebase generation %lu/%lu\n\0" as *const u8 as *const c_char,
                    (*h).timebase_num,
                    (*h).timebase_den,
                );
            } else if (*h).timebase_den > 4294967295 as uint64_t || (*h).timebase_den == 0 {
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"automatic timebase generation failed.\n                  Specify an appropriate timebase manually.\n\0"
                        as *const u8 as *const c_char,
                );
                return -1;
            }
            (*h).pts = malloc(
                ((*h).stored_pts_num as size_t).wrapping_mul(size_of::<int64_t>() as size_t),
            ) as *mut int64_t;
            if !(*h).pts.is_null() {
                num = 0 as c_int;
                while num < (*h).stored_pts_num {
                    *(*h).pts.offset(num as isize) = (*timecodes.offset(num as isize)
                        * ((*h).timebase_den as c_double / (*h).timebase_num as c_double)
                        + 0.5f64) as int64_t;
                    if num > 0 as c_int
                        && *(*h).pts.offset(num as isize)
                            <= *(*h).pts.offset((num - 1 as c_int) as isize)
                    {
                        x264_cli_log(
                            b"timecode\0" as *const u8 as *const c_char,
                            X264_LOG_ERROR,
                            b"invalid timebase or timecode for frame %d\n\0" as *const u8
                                as *const c_char,
                            num,
                        );
                        return -1;
                    }
                    num += 1;
                }
                free(timecodes as *mut c_void);
                return 0 as c_int;
            }
        }
        _ => {}
    }
    if !timecodes.is_null() {
        free(timecodes as *mut c_void);
    }
    if !fpss.is_null() {
        free(fpss as *mut c_void);
    }
    return -1;
}
#[c2rust::src_loc = "344:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut tcfile_in: *mut FILE = 0 as *mut FILE;
    let mut h: *mut timecode_hnd_t =
        malloc(size_of::<timecode_hnd_t>() as size_t) as *mut timecode_hnd_t;
    if h.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).pts = 0 as *mut int64_t;
    if !(*opt).timebase.is_null() {
        ret = sscanf(
            (*opt).timebase,
            b"%lu/%lu\0" as *const u8 as *const c_char,
            &mut (*h).timebase_num as *mut uint64_t,
            &mut (*h).timebase_den as *mut uint64_t,
        );
        if ret == 1 as c_int {
            (*h).timebase_num =
                strtoul((*opt).timebase, 0 as *mut *mut c_char, 10 as c_int) as uint64_t;
            (*h).timebase_den = 0 as uint64_t;
        }
        if (*h).timebase_num > 4294967295 as uint64_t || (*h).timebase_den > 4294967295 as uint64_t
        {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"timebase you specified exceeds H.264 maximum\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
    }
    (*h).auto_timebase_num = (ret == 0) as c_int;
    (*h).auto_timebase_den = (ret < 2 as c_int) as c_int;
    if (*h).auto_timebase_num != 0 {
        (*h).timebase_num = (*info).fps_den as uint64_t;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = 0 as uint64_t;
    }
    tcfile_in = fopen(psz_filename, b"rb\0" as *const u8 as *const c_char) as *mut FILE;
    if tcfile_in.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"can't open `%s'\n\0" as *const u8 as *const c_char,
            psz_filename,
        );
        return -1;
    }
    if x264_is_regular_file(tcfile_in) == 0 {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"tcfile input incompatible with non-regular file `%s'\n\0" as *const u8
                as *const c_char,
            psz_filename,
        );
        fclose(tcfile_in);
        return -1;
    }
    if parse_tcfile(tcfile_in, h, info) < 0 as c_int {
        if !(*h).pts.is_null() {
            free((*h).pts as *mut c_void);
        }
        fclose(tcfile_in);
        return -1;
    }
    fclose(tcfile_in);
    (*info).timebase_num = (*h).timebase_num as uint32_t;
    (*info).timebase_den = (*h).timebase_den as uint32_t;
    (*info).vfr = 1 as c_int;
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn get_frame_pts(
    mut h: *mut timecode_hnd_t,
    mut frame: c_int,
    mut real_frame: c_int,
) -> int64_t {
    if frame < (*h).stored_pts_num {
        return *(*h).pts.offset(frame as isize);
    } else {
        if !(*h).pts.is_null() && real_frame != 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const c_char,
                X264_LOG_INFO,
                b"input timecode file missing data for frame %d and later\n                 assuming constant fps %.6f\n\0"
                    as *const u8 as *const c_char,
                frame,
                (*h).assume_fps,
            );
            free((*h).pts as *mut c_void);
            (*h).pts = 0 as *mut int64_t;
        }
        let mut timecode: c_double = (*h).last_timecode + 1 as c_int as c_double / (*h).assume_fps;
        if real_frame != 0 {
            (*h).last_timecode = timecode;
        }
        return (timecode * ((*h).timebase_den as c_double / (*h).timebase_num as c_double) + 0.5f64)
            as int64_t;
    };
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if (*h).input.read_frame.expect("non-null function pointer")(pic, (*h).p_handle, frame) != 0 {
        return -1;
    }
    (*pic).pts = get_frame_pts(h, frame, 1 as c_int);
    (*pic).duration = get_frame_pts(h, frame + 1 as c_int, 0 as c_int) - (*pic).pts;
    return 0 as c_int;
}
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if (*h).input.release_frame.is_some() {
        return (*h).input.release_frame.expect("non-null function pointer")(pic, (*h).p_handle);
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "437:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    return (*h).input.picture_alloc.expect("non-null function pointer")(
        pic,
        (*h).p_handle,
        csp,
        width,
        height,
    );
}
#[c2rust::src_loc = "443:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    (*h).input.picture_clean.expect("non-null function pointer")(pic, (*h).p_handle);
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if !(*h).pts.is_null() {
        free((*h).pts as *mut c_void);
    }
    (*h).input.close_file.expect("non-null function pointer")((*h).p_handle);
    free(h as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "459:19"]
static mut timecode_input: cli_input_t = cli_input_t {
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
