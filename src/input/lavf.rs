use core::ffi::{c_char, c_int, c_uint, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::avcodec_h::{
    avcodec_alloc_context3, avcodec_free_context, avcodec_open2, avcodec_parameters_to_context,
    avcodec_receive_frame, avcodec_send_packet, AVCodecContext,
};
use crate::avformat_h::{
    av_find_input_format, av_read_frame, avformat_close_input, avformat_find_stream_info,
    avformat_open_input, AVFormatContext, AVInputFormat, AVStream,
};
use crate::avutil_h::{AVMEDIA_TYPE_VIDEO, AV_NOPTS_VALUE};
use crate::codec_h::{avcodec_find_decoder, AVCodec};
use crate::dict_h::{av_dict_free, av_dict_set, AVDictionary};
use crate::error_h::AVERROR_EOF;
use crate::input_h::{
    cli_image_t, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t, x264_cli_pic_alloc,
    X264_CSP_OTHER,
};
use crate::libav_frame_h::{
    av_frame_alloc, av_frame_free, AVFrame, AV_FRAME_FLAG_INTERLACED, AV_FRAME_FLAG_TOP_FIELD_FIRST,
};
use crate::packet_h::{av_packet_alloc, av_packet_free, av_packet_unref, AVPacket};
use crate::pixdesc_h::av_get_pix_fmt_name;
use crate::pixfmt_h::{
    AVCOL_RANGE_JPEG, AV_PIX_FMT_BGR24, AV_PIX_FMT_BGRA, AV_PIX_FMT_YUV420P, AV_PIX_FMT_YUV422P,
    AV_PIX_FMT_YUV444P,
};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint8_t};
use crate::stdlib_h::{calloc, free, malloc};
use crate::string_h::{memcpy, memset, strcmp};
use crate::strings_h::strcasecmp;
use crate::x264_h::{X264_CSP_NONE, X264_CSP_VFLIP, X264_LOG_ERROR, X264_LOG_WARNING};
use crate::x264cli_h::{get_filename_extension, hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:9"]
struct lavf_hnd_t {
    lavf: *mut AVFormatContext,
    lavc: *mut AVCodecContext,
    frame: *mut AVFrame,
    pkt: *mut AVPacket,
    stream_id: c_int,
    next_frame: c_int,
    vfr_input: c_int,
    first_pic: *mut cli_pic_t,
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn handle_jpeg(mut csp: c_int, mut fullrange: *mut c_int) -> c_int {
    match csp {
        12 => {
            *fullrange = 1 as c_int;
            return AV_PIX_FMT_YUV420P as c_int;
        }
        13 => {
            *fullrange = 1 as c_int;
            return AV_PIX_FMT_YUV422P as c_int;
        }
        14 => {
            *fullrange = 1 as c_int;
            return AV_PIX_FMT_YUV444P as c_int;
        }
        _ => return csp,
    };
}
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn codec_from_stream(mut stream: *mut AVStream) -> *mut AVCodecContext {
    let mut codec: *mut AVCodec =
        avcodec_find_decoder((*(*stream).codecpar).codec_id) as *mut AVCodec;
    if codec.is_null() {
        return 0 as *mut AVCodecContext;
    }
    let mut c: *mut AVCodecContext = avcodec_alloc_context3(codec);
    if c.is_null() {
        return 0 as *mut AVCodecContext;
    }
    if avcodec_parameters_to_context(c, (*stream).codecpar) < 0 as c_int {
        avcodec_free_context(&mut c);
        return 0 as *mut AVCodecContext;
    }
    return c;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn read_frame_internal(
    mut p_pic: *mut cli_pic_t,
    mut h: *mut lavf_hnd_t,
    mut i_frame: c_int,
    mut info: *mut video_info_t,
) -> c_int {
    if !(*h).first_pic.is_null() && info.is_null() {
        if i_frame == 0 {
            let mut t: cli_image_t = (*p_pic).img;
            (*p_pic).img = (*(*h).first_pic).img;
            (*(*h).first_pic).img = t;
            (*p_pic).pts = (*(*h).first_pic).pts;
        }
        lavf_input.picture_clean.expect("non-null function pointer")((*h).first_pic, h as hnd_t);
        free((*h).first_pic as *mut c_void);
        (*h).first_pic = 0 as *mut cli_pic_t;
        if i_frame == 0 {
            return 0 as c_int;
        }
    }
    let mut pkt: *mut AVPacket = (*h).pkt;
    while i_frame >= (*h).next_frame {
        let mut ret: c_int = 0;
        loop {
            ret = avcodec_receive_frame((*h).lavc, (*h).frame);
            if !(ret != 0) {
                break;
            }
            if ret == -(11 as c_int) {
                loop {
                    ret = av_read_frame((*h).lavf, pkt);
                    if !(ret == 0 && (*pkt).stream_index != (*h).stream_id) {
                        break;
                    }
                    av_packet_unref(pkt);
                }
                if ret != 0 {
                    ret = avcodec_send_packet((*h).lavc, 0 as *const AVPacket);
                } else {
                    ret = avcodec_send_packet((*h).lavc, pkt);
                    av_packet_unref(pkt);
                }
            } else if ret == AVERROR_EOF {
                return -(1 as c_int);
            }
            if ret != 0 {
                x264_cli_log(
                    b"lavf\0" as *const u8 as *const c_char,
                    X264_LOG_WARNING,
                    b"video decoding failed on frame %d\n\0" as *const u8 as *const c_char,
                    (*h).next_frame,
                );
                return -(1 as c_int);
            }
        }
        (*h).next_frame += 1;
    }
    memcpy(
        (*p_pic).img.stride.as_mut_ptr() as *mut c_void,
        (*(*h).frame).linesize.as_mut_ptr() as *const c_void,
        ::core::mem::size_of::<[c_int; 4]>() as size_t,
    );
    memcpy(
        (*p_pic).img.plane.as_mut_ptr() as *mut c_void,
        (*(*h).frame).data.as_mut_ptr() as *const c_void,
        ::core::mem::size_of::<[*mut uint8_t; 4]>() as size_t,
    );
    let mut is_fullrange: c_int = 0 as c_int;
    (*p_pic).img.width = (*(*h).lavc).width;
    (*p_pic).img.height = (*(*h).lavc).height;
    (*p_pic).img.csp =
        handle_jpeg((*(*h).lavc).pix_fmt as c_int, &mut is_fullrange) | X264_CSP_OTHER;
    if !info.is_null() {
        (*info).fullrange = is_fullrange;
        (*info).interlaced = ((*(*h).frame).flags & AV_FRAME_FLAG_INTERLACED != 0) as c_int;
        (*info).tff = ((*(*h).frame).flags & AV_FRAME_FLAG_TOP_FIELD_FIRST != 0) as c_int;
    }
    if (*h).vfr_input != 0 {
        (*p_pic).duration = 0 as int64_t;
        (*p_pic).pts = (*p_pic).duration;
        if (*(*h).frame).pts != AV_NOPTS_VALUE {
            (*p_pic).pts = (*(*h).frame).pts;
        } else if (*(*h).frame).pkt_dts != AV_NOPTS_VALUE {
            (*p_pic).pts = (*(*h).frame).pkt_dts;
        } else if !info.is_null() {
            (*info).vfr = 0 as c_int;
            (*h).vfr_input = (*info).vfr;
            return 0 as c_int;
        }
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> c_int {
    let mut h: *mut lavf_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<lavf_hnd_t>() as size_t) as *mut lavf_hnd_t;
    if h.is_null() {
        return -(1 as c_int);
    }
    if strcmp(psz_filename, b"-\0" as *const u8 as *const c_char) == 0 {
        psz_filename = b"pipe:\0" as *const u8 as *const c_char as *mut c_char;
    }
    (*h).frame = av_frame_alloc();
    if (*h).frame.is_null() {
        return -(1 as c_int);
    }
    (*h).pkt = av_packet_alloc();
    if (*h).pkt.is_null() {
        return -(1 as c_int);
    }
    let mut options: *mut AVDictionary = 0 as *mut AVDictionary;
    if !(*opt).resolution.is_null() {
        av_dict_set(
            &mut options,
            b"video_size\0" as *const u8 as *const c_char,
            (*opt).resolution,
            0 as c_int,
        );
        let mut csp: *const c_char = if !(*opt).colorspace.is_null() {
            (*opt).colorspace as *const c_char
        } else {
            av_get_pix_fmt_name(AV_PIX_FMT_YUV420P)
        };
        av_dict_set(
            &mut options,
            b"pixel_format\0" as *const u8 as *const c_char,
            csp,
            0 as c_int,
        );
    }
    let mut format: *mut AVInputFormat = 0 as *mut AVInputFormat;
    if !(*opt).format.is_null() {
        format = av_find_input_format((*opt).format) as *mut AVInputFormat;
        if format.is_null() {
            x264_cli_log(
                b"lavf\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"unknown file format: %s\n\0" as *const u8 as *const c_char,
                (*opt).format,
            );
            return -(1 as c_int);
        }
    }
    if avformat_open_input(&mut (*h).lavf, psz_filename, format, &mut options) != 0 {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"could not open input file\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if !options.is_null() {
        av_dict_free(&mut options);
    }
    if avformat_find_stream_info((*h).lavf, 0 as *mut *mut AVDictionary) < 0 as c_int {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"could not find input stream info\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    let mut i: c_int = 0 as c_int;
    while (i as c_uint) < (*(*h).lavf).nb_streams
        && (*(**(*(*h).lavf).streams.offset(i as isize)).codecpar).codec_type as c_int
            != AVMEDIA_TYPE_VIDEO as c_int
    {
        i += 1;
    }
    if i as c_uint == (*(*h).lavf).nb_streams {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"could not find video stream\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    (*h).stream_id = i;
    (*h).next_frame = 0 as c_int;
    (*h).lavc = codec_from_stream(*(*(*h).lavf).streams.offset(i as isize));
    if (*h).lavc.is_null() {
        return -(1 as c_int);
    }
    (*info).fps_num = (**(*(*h).lavf).streams.offset(i as isize))
        .avg_frame_rate
        .num as uint32_t;
    (*info).fps_den = (**(*(*h).lavf).streams.offset(i as isize))
        .avg_frame_rate
        .den as uint32_t;
    (*info).timebase_num = (**(*(*h).lavf).streams.offset(i as isize)).time_base.num as uint32_t;
    (*info).timebase_den = (**(*(*h).lavf).streams.offset(i as isize)).time_base.den as uint32_t;
    (*info).thread_safe = 0 as c_int;
    (*h).vfr_input = (*info).vfr;
    if avcodec_open2(
        (*h).lavc,
        avcodec_find_decoder((*(*h).lavc).codec_id),
        0 as *mut *mut AVDictionary,
    ) != 0
    {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"could not find decoder for video stream\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    (*h).first_pic = malloc(::core::mem::size_of::<cli_pic_t>() as size_t) as *mut cli_pic_t;
    if (*h).first_pic.is_null()
        || lavf_input.picture_alloc.expect("non-null function pointer")(
            (*h).first_pic,
            h as hnd_t,
            0x4000 as c_int,
            (*info).width,
            (*info).height,
        ) != 0
    {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if read_frame_internal((*h).first_pic, h, 0 as c_int, info) != 0 {
        return -(1 as c_int);
    }
    (*info).width = (*(*h).lavc).width;
    (*info).height = (*(*h).lavc).height;
    (*info).csp = (*(*h).first_pic).img.csp;
    (*info).num_frames = (**(*(*h).lavf).streams.offset(i as isize)).nb_frames as c_int;
    (*info).sar_height = (*(*h).lavc).sample_aspect_ratio.den as uint32_t;
    (*info).sar_width = (*(*h).lavc).sample_aspect_ratio.num as uint32_t;
    (*info).fullrange |=
        ((*(*h).lavc).color_range as c_uint == AVCOL_RANGE_JPEG as c_int as c_uint) as c_int;
    if strcasecmp(
        get_filename_extension(psz_filename),
        b"avs\0" as *const u8 as *const c_char,
    ) == 0
        && ((*(*h).lavc).pix_fmt as c_int == AV_PIX_FMT_BGRA as c_int
            || (*(*h).lavc).pix_fmt as c_int == AV_PIX_FMT_BGR24 as c_int)
    {
        (*info).csp |= X264_CSP_VFLIP;
    }
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "250:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut _handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    if x264_cli_pic_alloc(pic, X264_CSP_NONE, width, height) != 0 {
        return -(1 as c_int);
    }
    (*pic).img.csp = csp;
    (*pic).img.planes = 4 as c_int;
    return 0 as c_int;
}
#[c2rust::src_loc = "259:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: c_int,
) -> c_int {
    return read_frame_internal(
        pic,
        handle as *mut lavf_hnd_t,
        i_frame,
        0 as *mut video_info_t,
    );
}
#[c2rust::src_loc = "264:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut _handle: hnd_t) {
    memset(
        pic as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut lavf_hnd_t = handle as *mut lavf_hnd_t;
    avcodec_free_context(&mut (*h).lavc);
    avformat_close_input(&mut (*h).lavf);
    av_packet_free(&mut (*h).pkt);
    av_frame_free(&mut (*h).frame);
    free(h as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "280:19"]
static mut lavf_input: cli_input_t = cli_input_t {
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
    release_frame: None,
    picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
    close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> c_int),
};
