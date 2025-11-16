pub use crate::__stddef_size_t_h::size_t;
pub use crate::avformat_h::{av_demuxer_iterate, AVCodecTag, AVInputFormat};
pub use crate::bits_stdio_h::putchar;
pub use crate::cpu_h::{x264_cpu_name_t, x264_cpu_names};
pub use crate::input_h::{x264_cli_csp_t, x264_cli_csps, X264_CSP_CLI_MAX};
pub use crate::log_h::{
    AVClass, AVClassCategory, AVOption, AVOptionRanges, AV_CLASS_CATEGORY_BITSTREAM_FILTER,
    AV_CLASS_CATEGORY_DECODER, AV_CLASS_CATEGORY_DEMUXER, AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT, AV_CLASS_CATEGORY_DEVICE_INPUT,
    AV_CLASS_CATEGORY_DEVICE_OUTPUT, AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT, AV_CLASS_CATEGORY_ENCODER, AV_CLASS_CATEGORY_FILTER,
    AV_CLASS_CATEGORY_HWDEVICE, AV_CLASS_CATEGORY_INPUT, AV_CLASS_CATEGORY_MUXER,
    AV_CLASS_CATEGORY_NA, AV_CLASS_CATEGORY_NB, AV_CLASS_CATEGORY_OUTPUT,
    AV_CLASS_CATEGORY_SWRESAMPLER, AV_CLASS_CATEGORY_SWSCALER,
};
pub use crate::pixdesc_h::{av_pix_fmt_desc_next, AVComponentDescriptor, AVPixFmtDescriptor};
pub use crate::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{printf, snprintf};
use crate::string_h::{strchr, strcmp, strlen, strncmp};
use crate::strings_h::strncasecmp;
pub use crate::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use crate::types_h::{__off64_t, __off_t, __uint32_t, __uint64_t, __uint8_t};
pub use crate::x264_h::{
    x264_avcintra_flavor_names, x264_b_pyramid_names, x264_colmatrix_names, x264_colorprim_names,
    x264_direct_pred_names, x264_motion_est_names, x264_nal_hrd_names, x264_overscan_names,
    x264_preset_names, x264_transfer_names, x264_tune_names, x264_vidformat_names, X264_CSP_MAX,
    X264_CSP_NONE,
};
use crate::x264cli_h::{
    x264_avcintra_class_names, x264_cqm_names, x264_demuxer_names, x264_log_level_names,
    x264_muxer_names, x264_output_csp_names, x264_partition_names, x264_pulldown_names,
    x264_range_names, x264_valid_profile_names,
};
pub use crate::FILE_h::FILE;
pub use crate::__stddef_null_h::NULL;
#[c2rust::src_loc = "35:27"]
static mut level_names: [*const ::core::ffi::c_char; 21] = [
    b"1\0" as *const u8 as *const ::core::ffi::c_char,
    b"1.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"1.2\0" as *const u8 as *const ::core::ffi::c_char,
    b"1.3\0" as *const u8 as *const ::core::ffi::c_char,
    b"1b\0" as *const u8 as *const ::core::ffi::c_char,
    b"2\0" as *const u8 as *const ::core::ffi::c_char,
    b"2.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"2.2\0" as *const u8 as *const ::core::ffi::c_char,
    b"3\0" as *const u8 as *const ::core::ffi::c_char,
    b"3.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"3.2\0" as *const u8 as *const ::core::ffi::c_char,
    b"4\0" as *const u8 as *const ::core::ffi::c_char,
    b"4.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"4.2\0" as *const u8 as *const ::core::ffi::c_char,
    b"5\0" as *const u8 as *const ::core::ffi::c_char,
    b"5.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"5.2\0" as *const u8 as *const ::core::ffi::c_char,
    b"6\0" as *const u8 as *const ::core::ffi::c_char,
    b"6.1\0" as *const u8 as *const ::core::ffi::c_char,
    b"6.2\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "47:27"]
static mut opts_suggest: [*const ::core::ffi::c_char; 38] = [
    b"--alternative-transfer\0" as *const u8 as *const ::core::ffi::c_char,
    b"--aq-mode\0" as *const u8 as *const ::core::ffi::c_char,
    b"--asm\0" as *const u8 as *const ::core::ffi::c_char,
    b"--avcintra-class\0" as *const u8 as *const ::core::ffi::c_char,
    b"--avcintra-flavor\0" as *const u8 as *const ::core::ffi::c_char,
    b"--b-adapt\0" as *const u8 as *const ::core::ffi::c_char,
    b"--b-pyramid\0" as *const u8 as *const ::core::ffi::c_char,
    b"--colormatrix\0" as *const u8 as *const ::core::ffi::c_char,
    b"--colorprim\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm\0" as *const u8 as *const ::core::ffi::c_char,
    b"--demuxer\0" as *const u8 as *const ::core::ffi::c_char,
    b"--direct\0" as *const u8 as *const ::core::ffi::c_char,
    b"--frame-packing\0" as *const u8 as *const ::core::ffi::c_char,
    b"--input-csp\0" as *const u8 as *const ::core::ffi::c_char,
    b"--input-fmt\0" as *const u8 as *const ::core::ffi::c_char,
    b"--input-range\0" as *const u8 as *const ::core::ffi::c_char,
    b"--level\0" as *const u8 as *const ::core::ffi::c_char,
    b"--log-level\0" as *const u8 as *const ::core::ffi::c_char,
    b"--me\0" as *const u8 as *const ::core::ffi::c_char,
    b"--muxer\0" as *const u8 as *const ::core::ffi::c_char,
    b"--nal-hrd\0" as *const u8 as *const ::core::ffi::c_char,
    b"--output-csp\0" as *const u8 as *const ::core::ffi::c_char,
    b"--overscan\0" as *const u8 as *const ::core::ffi::c_char,
    b"--pass\0" as *const u8 as *const ::core::ffi::c_char,
    b"-p\0" as *const u8 as *const ::core::ffi::c_char,
    b"--preset\0" as *const u8 as *const ::core::ffi::c_char,
    b"--profile\0" as *const u8 as *const ::core::ffi::c_char,
    b"--pulldown\0" as *const u8 as *const ::core::ffi::c_char,
    b"--range\0" as *const u8 as *const ::core::ffi::c_char,
    b"--subme\0" as *const u8 as *const ::core::ffi::c_char,
    b"-m\0" as *const u8 as *const ::core::ffi::c_char,
    b"--transfer\0" as *const u8 as *const ::core::ffi::c_char,
    b"--trellis\0" as *const u8 as *const ::core::ffi::c_char,
    b"-t\0" as *const u8 as *const ::core::ffi::c_char,
    b"--tune\0" as *const u8 as *const ::core::ffi::c_char,
    b"--videoformat\0" as *const u8 as *const ::core::ffi::c_char,
    b"--weightp\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "87:27"]
static mut opts_nosuggest: [*const ::core::ffi::c_char; 77] = [
    b"--b-bias\0" as *const u8 as *const ::core::ffi::c_char,
    b"--bframes\0" as *const u8 as *const ::core::ffi::c_char,
    b"-b\0" as *const u8 as *const ::core::ffi::c_char,
    b"--deblock\0" as *const u8 as *const ::core::ffi::c_char,
    b"-f\0" as *const u8 as *const ::core::ffi::c_char,
    b"--bitrate\0" as *const u8 as *const ::core::ffi::c_char,
    b"-B\0" as *const u8 as *const ::core::ffi::c_char,
    b"--chroma-qp-offset\0" as *const u8 as *const ::core::ffi::c_char,
    b"--chromaloc\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cplxblur\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4i\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4ic\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4iy\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4p\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4pc\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm4py\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm8\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm8i\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cqm8p\0" as *const u8 as *const ::core::ffi::c_char,
    b"--crf\0" as *const u8 as *const ::core::ffi::c_char,
    b"--crf-max\0" as *const u8 as *const ::core::ffi::c_char,
    b"--crop-rect\0" as *const u8 as *const ::core::ffi::c_char,
    b"--deadzone-inter\0" as *const u8 as *const ::core::ffi::c_char,
    b"--deadzone-intra\0" as *const u8 as *const ::core::ffi::c_char,
    b"--fps\0" as *const u8 as *const ::core::ffi::c_char,
    b"--frames\0" as *const u8 as *const ::core::ffi::c_char,
    b"--input-depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"--input-res\0" as *const u8 as *const ::core::ffi::c_char,
    b"--ipratio\0" as *const u8 as *const ::core::ffi::c_char,
    b"--keyint\0" as *const u8 as *const ::core::ffi::c_char,
    b"-I\0" as *const u8 as *const ::core::ffi::c_char,
    b"--lookahead-threads\0" as *const u8 as *const ::core::ffi::c_char,
    b"--mastering-display\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cll\0" as *const u8 as *const ::core::ffi::c_char,
    b"--merange\0" as *const u8 as *const ::core::ffi::c_char,
    b"--min-keyint\0" as *const u8 as *const ::core::ffi::c_char,
    b"-i\0" as *const u8 as *const ::core::ffi::c_char,
    b"--mvrange\0" as *const u8 as *const ::core::ffi::c_char,
    b"--mvrange-thread\0" as *const u8 as *const ::core::ffi::c_char,
    b"--nr\0" as *const u8 as *const ::core::ffi::c_char,
    b"--opencl-device\0" as *const u8 as *const ::core::ffi::c_char,
    b"--output-depth\0" as *const u8 as *const ::core::ffi::c_char,
    b"--partitions\0" as *const u8 as *const ::core::ffi::c_char,
    b"-A\0" as *const u8 as *const ::core::ffi::c_char,
    b"--pbratio\0" as *const u8 as *const ::core::ffi::c_char,
    b"--psy-rd\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qblur\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qcomp\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qp\0" as *const u8 as *const ::core::ffi::c_char,
    b"-q\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qpmax\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qpmin\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qpstep\0" as *const u8 as *const ::core::ffi::c_char,
    b"--ratetol\0" as *const u8 as *const ::core::ffi::c_char,
    b"--ref\0" as *const u8 as *const ::core::ffi::c_char,
    b"-r\0" as *const u8 as *const ::core::ffi::c_char,
    b"--rc-lookahead\0" as *const u8 as *const ::core::ffi::c_char,
    b"--sar\0" as *const u8 as *const ::core::ffi::c_char,
    b"--scenecut\0" as *const u8 as *const ::core::ffi::c_char,
    b"--seek\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slices\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slices-max\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slice-max-size\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slice-max-mbs\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slice-min-mbs\0" as *const u8 as *const ::core::ffi::c_char,
    b"--sps-id\0" as *const u8 as *const ::core::ffi::c_char,
    b"--sync-lookahead\0" as *const u8 as *const ::core::ffi::c_char,
    b"--threads\0" as *const u8 as *const ::core::ffi::c_char,
    b"--timebase\0" as *const u8 as *const ::core::ffi::c_char,
    b"--vbv-bufsize\0" as *const u8 as *const ::core::ffi::c_char,
    b"--vbv-init\0" as *const u8 as *const ::core::ffi::c_char,
    b"--vbv-maxrate\0" as *const u8 as *const ::core::ffi::c_char,
    b"--video-filter\0" as *const u8 as *const ::core::ffi::c_char,
    b"--vf\0" as *const u8 as *const ::core::ffi::c_char,
    b"--zones\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "160:27"]
static mut opts_filename: [*const ::core::ffi::c_char; 11] = [
    b"--cqmfile\0" as *const u8 as *const ::core::ffi::c_char,
    b"--dump-yuv\0" as *const u8 as *const ::core::ffi::c_char,
    b"--index\0" as *const u8 as *const ::core::ffi::c_char,
    b"--opencl-clbin\0" as *const u8 as *const ::core::ffi::c_char,
    b"--output\0" as *const u8 as *const ::core::ffi::c_char,
    b"-o\0" as *const u8 as *const ::core::ffi::c_char,
    b"--qpfile\0" as *const u8 as *const ::core::ffi::c_char,
    b"--stats\0" as *const u8 as *const ::core::ffi::c_char,
    b"--tcfile-in\0" as *const u8 as *const ::core::ffi::c_char,
    b"--tcfile-out\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "175:27"]
static mut opts_standalone: [*const ::core::ffi::c_char; 43] = [
    b"--8x8dct\0" as *const u8 as *const ::core::ffi::c_char,
    b"--aud\0" as *const u8 as *const ::core::ffi::c_char,
    b"--bff\0" as *const u8 as *const ::core::ffi::c_char,
    b"--bluray-compat\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cabac\0" as *const u8 as *const ::core::ffi::c_char,
    b"--constrained-intra\0" as *const u8 as *const ::core::ffi::c_char,
    b"--cpu-independent\0" as *const u8 as *const ::core::ffi::c_char,
    b"--dts-compress\0" as *const u8 as *const ::core::ffi::c_char,
    b"--fake-interlaced\0" as *const u8 as *const ::core::ffi::c_char,
    b"--fast-pskip\0" as *const u8 as *const ::core::ffi::c_char,
    b"--filler\0" as *const u8 as *const ::core::ffi::c_char,
    b"--force-cfr\0" as *const u8 as *const ::core::ffi::c_char,
    b"--mbtree\0" as *const u8 as *const ::core::ffi::c_char,
    b"--mixed-refs\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-8x8dct\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-asm\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-cabac\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-chroma-me\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-dct-decimate\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-deblock\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-fast-pskip\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-mbtree\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-mixed-refs\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-progress\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-psy\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-scenecut\0" as *const u8 as *const ::core::ffi::c_char,
    b"--no-weightb\0" as *const u8 as *const ::core::ffi::c_char,
    b"--non-deterministic\0" as *const u8 as *const ::core::ffi::c_char,
    b"--open-gop\0" as *const u8 as *const ::core::ffi::c_char,
    b"--opencl\0" as *const u8 as *const ::core::ffi::c_char,
    b"--pic-struct\0" as *const u8 as *const ::core::ffi::c_char,
    b"--psnr\0" as *const u8 as *const ::core::ffi::c_char,
    b"--quiet\0" as *const u8 as *const ::core::ffi::c_char,
    b"--sliced-threads\0" as *const u8 as *const ::core::ffi::c_char,
    b"--slow-firstpass\0" as *const u8 as *const ::core::ffi::c_char,
    b"--ssim\0" as *const u8 as *const ::core::ffi::c_char,
    b"--stitchable\0" as *const u8 as *const ::core::ffi::c_char,
    b"--tff\0" as *const u8 as *const ::core::ffi::c_char,
    b"--thread-input\0" as *const u8 as *const ::core::ffi::c_char,
    b"--verbose\0" as *const u8 as *const ::core::ffi::c_char,
    b"-v\0" as *const u8 as *const ::core::ffi::c_char,
    b"--weightb\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "222:27"]
static mut opts_special: [*const ::core::ffi::c_char; 6] = [
    b"--fullhelp\0" as *const u8 as *const ::core::ffi::c_char,
    b"--help\0" as *const u8 as *const ::core::ffi::c_char,
    b"-h\0" as *const u8 as *const ::core::ffi::c_char,
    b"--longhelp\0" as *const u8 as *const ::core::ffi::c_char,
    b"--version\0" as *const u8 as *const ::core::ffi::c_char,
    0 as *const ::core::ffi::c_char,
];
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn list_contains(
    mut list: *const *const ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *s != 0 {
        while !(*list).is_null() {
            if strcmp(*list, s) == 0 {
                return 1 as ::core::ffi::c_int;
            }
            list = list.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "240:1"]
unsafe extern "C" fn suggest(
    mut s: *const ::core::ffi::c_char,
    mut cur: *const ::core::ffi::c_char,
    mut cur_len: ::core::ffi::c_int,
) {
    if !s.is_null() && *s as ::core::ffi::c_int != 0 && strncmp(s, cur, cur_len as size_t) == 0 {
        printf(b"%s \0" as *const u8 as *const ::core::ffi::c_char, s);
    }
}
#[c2rust::src_loc = "246:1"]
unsafe extern "C" fn suggest_lower(
    mut s: *const ::core::ffi::c_char,
    mut cur: *const ::core::ffi::c_char,
    mut cur_len: ::core::ffi::c_int,
) {
    if !s.is_null() && *s as ::core::ffi::c_int != 0 && strncasecmp(s, cur, cur_len as size_t) == 0
    {
        while *s != 0 {
            putchar(
                if (*s as ::core::ffi::c_int) < 'A' as i32 || *s as ::core::ffi::c_int > 'Z' as i32
                {
                    *s as ::core::ffi::c_int
                } else {
                    *s as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int
                },
            );
            s = s.offset(1);
        }
        putchar(' ' as i32);
    }
}
#[c2rust::src_loc = "256:1"]
unsafe extern "C" fn suggest_num_range(
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
    mut cur: *const ::core::ffi::c_char,
    mut cur_len: ::core::ffi::c_int,
) {
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut i: ::core::ffi::c_int = start;
    while i <= end {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        suggest(buf.as_mut_ptr(), cur, cur_len);
        i += 1;
    }
}
#[c2rust::src_loc = "268:1"]
unsafe extern "C" fn suggest_token(
    mut s: *const ::core::ffi::c_char,
    mut delim: ::core::ffi::c_int,
    mut cur: *const ::core::ffi::c_char,
    mut cur_len: ::core::ffi::c_int,
) {
    if !s.is_null() && *s as ::core::ffi::c_int != 0 {
        let mut tok_end: *const ::core::ffi::c_char = 0 as *const ::core::ffi::c_char;
        loop {
            tok_end = strchr(s, delim);
            if tok_end.is_null() {
                break;
            }
            let mut tok_len: ::core::ffi::c_int =
                tok_end.offset_from(s) as ::core::ffi::c_long as ::core::ffi::c_int;
            if tok_len != 0 && tok_len >= cur_len && strncmp(s, cur, cur_len as size_t) == 0 {
                printf(
                    b"%.*s \0" as *const u8 as *const ::core::ffi::c_char,
                    tok_len,
                    s,
                );
            }
            s = tok_end.offset(1 as ::core::ffi::c_int as isize);
        }
        suggest(s, cur, cur_len);
    }
}
#[no_mangle]
#[c2rust::src_loc = "293:1"]
pub unsafe extern "C" fn x264_cli_autocomplete(
    mut prev: *const ::core::ffi::c_char,
    mut cur: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cur_len: ::core::ffi::c_int = strlen(cur) as ::core::ffi::c_int;
    if strcmp(
        prev,
        b"--alternative-transfer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s: *const *const ::core::ffi::c_char = x264_transfer_names.as_ptr();
        while !(*s).is_null() {
            suggest(*s, cur, cur_len);
            s = s.offset(1);
        }
    } else if strcmp(
        prev,
        b"--aq-mode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(prev, b"--asm\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut cpu: *const x264_cpu_name_t = x264_cpu_names.as_ptr();
        while (*cpu).flags != 0 {
            suggest_lower((*cpu).name, cur, cur_len);
            cpu = cpu.offset(1);
        }
    } else if strcmp(
        prev,
        b"--avcintra-class\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_0: *const *const ::core::ffi::c_char = x264_avcintra_class_names.as_ptr();
        while !(*s_0).is_null() {
            suggest(*s_0, cur, cur_len);
            s_0 = s_0.offset(1);
        }
    } else if strcmp(
        prev,
        b"--avcintra-flavor\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_1: *const *const ::core::ffi::c_char = x264_avcintra_flavor_names.as_ptr();
        while !(*s_1).is_null() {
            suggest(*s_1, cur, cur_len);
            s_1 = s_1.offset(1);
        }
    } else if strcmp(
        prev,
        b"--b-adapt\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(
        prev,
        b"--b-pyramid\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_2: *const *const ::core::ffi::c_char = x264_b_pyramid_names.as_ptr();
        while !(*s_2).is_null() {
            suggest(*s_2, cur, cur_len);
            s_2 = s_2.offset(1);
        }
    } else if strcmp(
        prev,
        b"--colormatrix\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_3: *const *const ::core::ffi::c_char = x264_colmatrix_names.as_ptr();
        while !(*s_3).is_null() {
            suggest(*s_3, cur, cur_len);
            s_3 = s_3.offset(1);
        }
    } else if strcmp(
        prev,
        b"--colorprim\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_4: *const *const ::core::ffi::c_char = x264_colorprim_names.as_ptr();
        while !(*s_4).is_null() {
            suggest(*s_4, cur, cur_len);
            s_4 = s_4.offset(1);
        }
    } else if strcmp(prev, b"--cqm\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut s_5: *const *const ::core::ffi::c_char = x264_cqm_names.as_ptr();
        while !(*s_5).is_null() {
            suggest(*s_5, cur, cur_len);
            s_5 = s_5.offset(1);
        }
    } else if strcmp(
        prev,
        b"--demuxer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_6: *const *const ::core::ffi::c_char = x264_demuxer_names.as_ptr();
        while !(*s_6).is_null() {
            suggest(*s_6, cur, cur_len);
            s_6 = s_6.offset(1);
        }
    } else if strcmp(
        prev,
        b"--direct\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_7: *const *const ::core::ffi::c_char = x264_direct_pred_names.as_ptr();
        while !(*s_7).is_null() {
            suggest(*s_7, cur, cur_len);
            s_7 = s_7.offset(1);
        }
    } else if strcmp(
        prev,
        b"--frame-packing\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            7 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(
        prev,
        b"--input-csp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut i: ::core::ffi::c_int = X264_CSP_NONE + 1 as ::core::ffi::c_int;
        while i < X264_CSP_CLI_MAX {
            suggest(
                (*x264_cli_csps.as_ptr().offset(i as isize)).name,
                cur,
                cur_len,
            );
            i += 1;
        }
        let mut d: *const AVPixFmtDescriptor = 0 as *const AVPixFmtDescriptor;
        loop {
            d = av_pix_fmt_desc_next(d);
            if d.is_null() {
                break;
            }
            suggest((*d).name, cur, cur_len);
        }
    } else if strcmp(
        prev,
        b"--input-fmt\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut i_0: *mut ::core::ffi::c_void = NULL;
        let mut f: *const AVInputFormat = 0 as *const AVInputFormat;
        loop {
            f = av_demuxer_iterate(&mut i_0);
            if f.is_null() {
                break;
            }
            suggest_token((*f).name, ',' as i32, cur, cur_len);
        }
    } else if strcmp(
        prev,
        b"--input-range\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_8: *const *const ::core::ffi::c_char = x264_range_names.as_ptr();
        while !(*s_8).is_null() {
            suggest(*s_8, cur, cur_len);
            s_8 = s_8.offset(1);
        }
    } else if strcmp(
        prev,
        b"--level\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_9: *const *const ::core::ffi::c_char = level_names.as_ptr();
        while !(*s_9).is_null() {
            suggest(*s_9, cur, cur_len);
            s_9 = s_9.offset(1);
        }
    } else if strcmp(
        prev,
        b"--log-level\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_10: *const *const ::core::ffi::c_char = x264_log_level_names.as_ptr();
        while !(*s_10).is_null() {
            suggest(*s_10, cur, cur_len);
            s_10 = s_10.offset(1);
        }
    } else if strcmp(prev, b"--me\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut s_11: *const *const ::core::ffi::c_char = x264_motion_est_names.as_ptr();
        while !(*s_11).is_null() {
            suggest(*s_11, cur, cur_len);
            s_11 = s_11.offset(1);
        }
    } else if strcmp(
        prev,
        b"--muxer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_12: *const *const ::core::ffi::c_char = x264_muxer_names.as_ptr();
        while !(*s_12).is_null() {
            suggest(*s_12, cur, cur_len);
            s_12 = s_12.offset(1);
        }
    } else if strcmp(
        prev,
        b"--nal-hrd\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_13: *const *const ::core::ffi::c_char = x264_nal_hrd_names.as_ptr();
        while !(*s_13).is_null() {
            suggest(*s_13, cur, cur_len);
            s_13 = s_13.offset(1);
        }
    } else if strcmp(
        prev,
        b"--output-csp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_14: *const *const ::core::ffi::c_char = x264_output_csp_names.as_ptr();
        while !(*s_14).is_null() {
            suggest(*s_14, cur, cur_len);
            s_14 = s_14.offset(1);
        }
    } else if strcmp(
        prev,
        b"--output-depth\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        suggest(
            b"8\0" as *const u8 as *const ::core::ffi::c_char,
            cur,
            cur_len,
        );
        suggest(
            b"10\0" as *const u8 as *const ::core::ffi::c_char,
            cur,
            cur_len,
        );
    } else if strcmp(
        prev,
        b"--overscan\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_15: *const *const ::core::ffi::c_char = x264_overscan_names.as_ptr();
        while !(*s_15).is_null() {
            suggest(*s_15, cur, cur_len);
            s_15 = s_15.offset(1);
        }
    } else if strcmp(
        prev,
        b"--partitions\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(prev, b"-A\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        let mut s_16: *const *const ::core::ffi::c_char = x264_partition_names.as_ptr();
        while !(*s_16).is_null() {
            suggest(*s_16, cur, cur_len);
            s_16 = s_16.offset(1);
        }
    } else if strcmp(prev, b"--pass\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(prev, b"-p\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        suggest_num_range(
            1 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(
        prev,
        b"--preset\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_17: *const *const ::core::ffi::c_char = x264_preset_names.as_ptr();
        while !(*s_17).is_null() {
            suggest(*s_17, cur, cur_len);
            s_17 = s_17.offset(1);
        }
    } else if strcmp(
        prev,
        b"--profile\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_18: *const *const ::core::ffi::c_char = x264_valid_profile_names.as_ptr();
        while !(*s_18).is_null() {
            suggest(*s_18, cur, cur_len);
            s_18 = s_18.offset(1);
        }
    } else if strcmp(
        prev,
        b"--pulldown\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_19: *const *const ::core::ffi::c_char = x264_pulldown_names.as_ptr();
        while !(*s_19).is_null() {
            suggest(*s_19, cur, cur_len);
            s_19 = s_19.offset(1);
        }
    } else if strcmp(
        prev,
        b"--range\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_20: *const *const ::core::ffi::c_char = x264_range_names.as_ptr();
        while !(*s_20).is_null() {
            suggest(*s_20, cur, cur_len);
            s_20 = s_20.offset(1);
        }
    } else if strcmp(
        prev,
        b"--subme\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(prev, b"-m\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            11 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(
        prev,
        b"--transfer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_21: *const *const ::core::ffi::c_char = x264_transfer_names.as_ptr();
        while !(*s_21).is_null() {
            suggest(*s_21, cur, cur_len);
            s_21 = s_21.offset(1);
        }
    } else if strcmp(
        prev,
        b"--trellis\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(prev, b"-t\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if strcmp(prev, b"--tune\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut s_22: *const *const ::core::ffi::c_char = x264_tune_names.as_ptr();
        while !(*s_22).is_null() {
            suggest(*s_22, cur, cur_len);
            s_22 = s_22.offset(1);
        }
    } else if strcmp(
        prev,
        b"--videoformat\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut s_23: *const *const ::core::ffi::c_char = x264_vidformat_names.as_ptr();
        while !(*s_23).is_null() {
            suggest(*s_23, cur, cur_len);
            s_23 = s_23.offset(1);
        }
    } else if strcmp(
        prev,
        b"--weightp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        suggest_num_range(
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            cur,
            cur_len,
        );
    } else if list_contains(opts_nosuggest.as_ptr(), prev) == 0
        && list_contains(opts_special.as_ptr(), prev) == 0
    {
        if list_contains(opts_filename.as_ptr(), prev) != 0
            || strncmp(
                cur,
                b"--\0" as *const u8 as *const ::core::ffi::c_char,
                2 as size_t,
            ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        let mut s_24: *const *const ::core::ffi::c_char = opts_suggest.as_ptr();
        while !(*s_24).is_null() {
            suggest(*s_24, cur, cur_len);
            s_24 = s_24.offset(1);
        }
        let mut s_25: *const *const ::core::ffi::c_char = opts_nosuggest.as_ptr();
        while !(*s_25).is_null() {
            suggest(*s_25, cur, cur_len);
            s_25 = s_25.offset(1);
        }
        let mut s_26: *const *const ::core::ffi::c_char = opts_filename.as_ptr();
        while !(*s_26).is_null() {
            suggest(*s_26, cur, cur_len);
            s_26 = s_26.offset(1);
        }
        let mut s_27: *const *const ::core::ffi::c_char = opts_standalone.as_ptr();
        while !(*s_27).is_null() {
            suggest(*s_27, cur, cur_len);
            s_27 = s_27.offset(1);
        }
        if *prev == 0 {
            let mut s_28: *const *const ::core::ffi::c_char = opts_special.as_ptr();
            while !(*s_28).is_null() {
                suggest(*s_28, cur, cur_len);
                s_28 = s_28.offset(1);
            }
        }
    }
    putchar('\n' as i32);
    return 0 as ::core::ffi::c_int;
}
