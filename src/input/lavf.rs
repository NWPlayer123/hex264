#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:27"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    #[inline]
    #[c2rust::src_loc = "67:1"]
    pub unsafe extern "C" fn get_filename_extension(
        mut filename: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        let mut ext: *mut ::core::ffi::c_char = filename.offset(strlen(filename) as isize);
        while *ext as ::core::ffi::c_int != '.' as i32 && ext > filename {
            ext = ext.offset(-1);
        }
        ext = ext.offset((*ext as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int as isize);
        return ext;
    }
    use super::string_h::strlen;
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
    #[c2rust::src_loc = "115:9"]
    pub const X264_CSP_OTHER: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "131:1"]
        pub fn x264_cli_pic_alloc(
            pic: *mut cli_pic_t,
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavcodec/packet.h:27"]
pub mod packet_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "529:16"]
    pub struct AVPacket {
        pub buf: *mut AVBufferRef,
        pub pts: int64_t,
        pub dts: int64_t,
        pub data: *mut uint8_t,
        pub size: ::core::ffi::c_int,
        pub stream_index: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
        pub side_data: *mut AVPacketSideData,
        pub side_data_elems: ::core::ffi::c_int,
        pub duration: int64_t,
        pub pos: int64_t,
        pub opaque: *mut ::core::ffi::c_void,
        pub opaque_ref: *mut AVBufferRef,
        pub time_base: AVRational,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "403:16"]
    pub struct AVPacketSideData {
        pub data: *mut uint8_t,
        pub size: size_t,
        pub type_0: AVPacketSideDataType,
    }
    #[c2rust::src_loc = "41:1"]
    pub type AVPacketSideDataType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "373:5"]
    pub const AV_PKT_DATA_NB: AVPacketSideDataType = 40;
    #[c2rust::src_loc = "363:5"]
    pub const AV_PKT_DATA_RTCP_SR: AVPacketSideDataType = 39;
    #[c2rust::src_loc = "357:5"]
    pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: AVPacketSideDataType = 38;
    #[c2rust::src_loc = "346:5"]
    pub const AV_PKT_DATA_LCEVC: AVPacketSideDataType = 37;
    #[c2rust::src_loc = "340:5"]
    pub const AV_PKT_DATA_FRAME_CROPPING: AVPacketSideDataType = 36;
    #[c2rust::src_loc = "327:5"]
    pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVPacketSideDataType = 35;
    #[c2rust::src_loc = "320:5"]
    pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVPacketSideDataType = 34;
    #[c2rust::src_loc = "312:5"]
    pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: AVPacketSideDataType = 33;
    #[c2rust::src_loc = "304:5"]
    pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: AVPacketSideDataType = 32;
    #[c2rust::src_loc = "296:5"]
    pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;
    #[c2rust::src_loc = "288:5"]
    pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
    #[c2rust::src_loc = "280:5"]
    pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
    #[c2rust::src_loc = "271:5"]
    pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
    #[c2rust::src_loc = "265:5"]
    pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
    #[c2rust::src_loc = "258:5"]
    pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
    #[c2rust::src_loc = "252:5"]
    pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
    #[c2rust::src_loc = "246:5"]
    pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
    #[c2rust::src_loc = "239:5"]
    pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
    #[c2rust::src_loc = "232:5"]
    pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
    #[c2rust::src_loc = "225:5"]
    pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
    #[c2rust::src_loc = "219:5"]
    pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
    #[c2rust::src_loc = "212:5"]
    pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
    #[c2rust::src_loc = "206:5"]
    pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
    #[c2rust::src_loc = "199:5"]
    pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
    #[c2rust::src_loc = "193:5"]
    pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
    #[c2rust::src_loc = "188:5"]
    pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
    #[c2rust::src_loc = "180:5"]
    pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
    #[c2rust::src_loc = "169:5"]
    pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
    #[c2rust::src_loc = "163:5"]
    pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
    #[c2rust::src_loc = "153:5"]
    pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
    #[c2rust::src_loc = "142:5"]
    pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
    #[c2rust::src_loc = "137:5"]
    pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
    #[c2rust::src_loc = "129:5"]
    pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
    #[c2rust::src_loc = "117:5"]
    pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
    #[c2rust::src_loc = "111:5"]
    pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
    #[c2rust::src_loc = "105:5"]
    pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
    #[c2rust::src_loc = "96:5"]
    pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
    #[c2rust::src_loc = "90:5"]
    pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
    #[c2rust::src_loc = "69:5"]
    pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
    #[c2rust::src_loc = "56:5"]
    pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
    #[c2rust::src_loc = "47:5"]
    pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
    use super::__stddef_size_t_h::size_t;
    use super::buffer_h::AVBufferRef;
    use super::rational_h::AVRational;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "644:1"]
        pub fn av_packet_alloc() -> *mut AVPacket;
        #[c2rust::src_loc = "665:1"]
        pub fn av_packet_free(pkt: *mut *mut AVPacket);
        #[c2rust::src_loc = "831:1"]
        pub fn av_packet_unref(pkt: *mut AVPacket);
    }
}
#[c2rust::header_src = "/usr/include/libavutil/rational.h:27"]
pub mod rational_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:16"]
    pub struct AVRational {
        pub num: ::core::ffi::c_int,
        pub den: ::core::ffi::c_int,
    }
}
#[c2rust::header_src = "/usr/include/libavutil/buffer.h:27"]
pub mod buffer_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct AVBufferRef {
        pub buffer: *mut AVBuffer,
        pub data: *mut uint8_t,
        pub size: size_t,
    }
    use super::__stddef_size_t_h::size_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "74:16"]
        pub type AVBuffer;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/frame.h:27"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "421:16"]
    pub struct AVFrame {
        pub data: [*mut uint8_t; 8],
        pub linesize: [::core::ffi::c_int; 8],
        pub extended_data: *mut *mut uint8_t,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub nb_samples: ::core::ffi::c_int,
        pub format: ::core::ffi::c_int,
        pub pict_type: AVPictureType,
        pub sample_aspect_ratio: AVRational,
        pub pts: int64_t,
        pub pkt_dts: int64_t,
        pub time_base: AVRational,
        pub quality: ::core::ffi::c_int,
        pub opaque: *mut ::core::ffi::c_void,
        pub repeat_pict: ::core::ffi::c_int,
        pub sample_rate: ::core::ffi::c_int,
        pub buf: [*mut AVBufferRef; 8],
        pub extended_buf: *mut *mut AVBufferRef,
        pub nb_extended_buf: ::core::ffi::c_int,
        pub side_data: *mut *mut AVFrameSideData,
        pub nb_side_data: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
        pub color_range: AVColorRange,
        pub color_primaries: AVColorPrimaries,
        pub color_trc: AVColorTransferCharacteristic,
        pub colorspace: AVColorSpace,
        pub chroma_location: AVChromaLocation,
        pub best_effort_timestamp: int64_t,
        pub metadata: *mut AVDictionary,
        pub decode_error_flags: ::core::ffi::c_int,
        pub hw_frames_ctx: *mut AVBufferRef,
        pub opaque_ref: *mut AVBufferRef,
        pub crop_top: size_t,
        pub crop_bottom: size_t,
        pub crop_left: size_t,
        pub crop_right: size_t,
        pub private_ref: *mut ::core::ffi::c_void,
        pub ch_layout: AVChannelLayout,
        pub duration: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "276:16"]
    pub struct AVFrameSideData {
        pub type_0: AVFrameSideDataType,
        pub data: *mut uint8_t,
        pub size: size_t,
        pub metadata: *mut AVDictionary,
        pub buf: *mut AVBufferRef,
    }
    #[c2rust::src_loc = "49:1"]
    pub type AVFrameSideDataType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "256:5"]
    pub const AV_FRAME_DATA_3D_REFERENCE_DISPLAYS: AVFrameSideDataType = 30;
    #[c2rust::src_loc = "245:5"]
    pub const AV_FRAME_DATA_VIEW_ID: AVFrameSideDataType = 29;
    #[c2rust::src_loc = "236:5"]
    pub const AV_FRAME_DATA_LCEVC: AVFrameSideDataType = 28;
    #[c2rust::src_loc = "230:5"]
    pub const AV_FRAME_DATA_VIDEO_HINT: AVFrameSideDataType = 27;
    #[c2rust::src_loc = "220:5"]
    pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVFrameSideDataType = 26;
    #[c2rust::src_loc = "215:5"]
    pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: AVFrameSideDataType = 25;
    #[c2rust::src_loc = "208:5"]
    pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;
    #[c2rust::src_loc = "201:5"]
    pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
    #[c2rust::src_loc = "194:5"]
    pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
    #[c2rust::src_loc = "188:5"]
    pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
    #[c2rust::src_loc = "178:5"]
    pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
    #[c2rust::src_loc = "170:5"]
    pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
    #[c2rust::src_loc = "165:5"]
    pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
    #[c2rust::src_loc = "159:5"]
    pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
    #[c2rust::src_loc = "152:5"]
    pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
    #[c2rust::src_loc = "144:5"]
    pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
    #[c2rust::src_loc = "137:5"]
    pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
    #[c2rust::src_loc = "131:5"]
    pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
    #[c2rust::src_loc = "125:5"]
    pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
    #[c2rust::src_loc = "120:5"]
    pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
    #[c2rust::src_loc = "114:5"]
    pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
    #[c2rust::src_loc = "109:5"]
    pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
    #[c2rust::src_loc = "97:5"]
    pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
    #[c2rust::src_loc = "90:5"]
    pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
    #[c2rust::src_loc = "85:5"]
    pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
    #[c2rust::src_loc = "77:5"]
    pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
    #[c2rust::src_loc = "73:5"]
    pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
    #[c2rust::src_loc = "68:5"]
    pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
    #[c2rust::src_loc = "64:5"]
    pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
    #[c2rust::src_loc = "59:5"]
    pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
    #[c2rust::src_loc = "53:5"]
    pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
    #[c2rust::src_loc = "644:9"]
    pub const AV_FRAME_FLAG_INTERLACED: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "649:9"]
    pub const AV_FRAME_FLAG_TOP_FIELD_FIRST: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    use super::__stddef_size_t_h::size_t;
    use super::avutil_h::AVPictureType;
    use super::buffer_h::AVBufferRef;
    use super::channel_layout_h::AVChannelLayout;
    use super::dict_h::AVDictionary;
    use super::pixfmt_h::{
        AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic,
    };
    use super::rational_h::AVRational;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "783:1"]
        pub fn av_frame_alloc() -> *mut AVFrame;
        #[c2rust::src_loc = "792:1"]
        pub fn av_frame_free(frame: *mut *mut AVFrame);
    }
}
#[c2rust::header_src = "/usr/include/libavutil/channel_layout.h:27"]
pub mod channel_layout_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "319:16"]
    pub struct AVChannelLayout {
        pub order: AVChannelOrder,
        pub nb_channels: ::core::ffi::c_int,
        pub u: C2RustUnnamed,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "336:5"]
    pub union C2RustUnnamed {
        pub mask: uint64_t,
        pub map: *mut AVChannelCustom,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "283:16"]
    pub struct AVChannelCustom {
        pub id: AVChannel,
        pub name: [::core::ffi::c_char; 16],
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "47:1"]
    pub type AVChannel = ::core::ffi::c_int;
    #[c2rust::src_loc = "111:5"]
    pub const AV_CHAN_AMBISONIC_END: AVChannel = 2047;
    #[c2rust::src_loc = "108:5"]
    pub const AV_CHAN_AMBISONIC_BASE: AVChannel = 1024;
    #[c2rust::src_loc = "94:5"]
    pub const AV_CHAN_UNKNOWN: AVChannel = 768;
    #[c2rust::src_loc = "91:5"]
    pub const AV_CHAN_UNUSED: AVChannel = 512;
    #[c2rust::src_loc = "88:5"]
    pub const AV_CHAN_BINAURAL_RIGHT: AVChannel = 62;
    #[c2rust::src_loc = "87:5"]
    pub const AV_CHAN_BINAURAL_LEFT: AVChannel = 61;
    #[c2rust::src_loc = "85:5"]
    pub const AV_CHAN_TOP_SURROUND_RIGHT: AVChannel = 44;
    #[c2rust::src_loc = "84:5"]
    pub const AV_CHAN_TOP_SURROUND_LEFT: AVChannel = 43;
    #[c2rust::src_loc = "83:5"]
    pub const AV_CHAN_SIDE_SURROUND_RIGHT: AVChannel = 42;
    #[c2rust::src_loc = "82:5"]
    pub const AV_CHAN_SIDE_SURROUND_LEFT: AVChannel = 41;
    #[c2rust::src_loc = "81:5"]
    pub const AV_CHAN_BOTTOM_FRONT_RIGHT: AVChannel = 40;
    #[c2rust::src_loc = "80:5"]
    pub const AV_CHAN_BOTTOM_FRONT_LEFT: AVChannel = 39;
    #[c2rust::src_loc = "79:5"]
    pub const AV_CHAN_BOTTOM_FRONT_CENTER: AVChannel = 38;
    #[c2rust::src_loc = "78:5"]
    pub const AV_CHAN_TOP_SIDE_RIGHT: AVChannel = 37;
    #[c2rust::src_loc = "77:5"]
    pub const AV_CHAN_TOP_SIDE_LEFT: AVChannel = 36;
    #[c2rust::src_loc = "76:5"]
    pub const AV_CHAN_LOW_FREQUENCY_2: AVChannel = 35;
    #[c2rust::src_loc = "75:5"]
    pub const AV_CHAN_SURROUND_DIRECT_RIGHT: AVChannel = 34;
    #[c2rust::src_loc = "74:5"]
    pub const AV_CHAN_SURROUND_DIRECT_LEFT: AVChannel = 33;
    #[c2rust::src_loc = "73:5"]
    pub const AV_CHAN_WIDE_RIGHT: AVChannel = 32;
    #[c2rust::src_loc = "72:5"]
    pub const AV_CHAN_WIDE_LEFT: AVChannel = 31;
    #[c2rust::src_loc = "71:5"]
    pub const AV_CHAN_STEREO_RIGHT: AVChannel = 30;
    #[c2rust::src_loc = "69:5"]
    pub const AV_CHAN_STEREO_LEFT: AVChannel = 29;
    #[c2rust::src_loc = "67:5"]
    pub const AV_CHAN_TOP_BACK_RIGHT: AVChannel = 17;
    #[c2rust::src_loc = "66:5"]
    pub const AV_CHAN_TOP_BACK_CENTER: AVChannel = 16;
    #[c2rust::src_loc = "65:5"]
    pub const AV_CHAN_TOP_BACK_LEFT: AVChannel = 15;
    #[c2rust::src_loc = "64:5"]
    pub const AV_CHAN_TOP_FRONT_RIGHT: AVChannel = 14;
    #[c2rust::src_loc = "63:5"]
    pub const AV_CHAN_TOP_FRONT_CENTER: AVChannel = 13;
    #[c2rust::src_loc = "62:5"]
    pub const AV_CHAN_TOP_FRONT_LEFT: AVChannel = 12;
    #[c2rust::src_loc = "61:5"]
    pub const AV_CHAN_TOP_CENTER: AVChannel = 11;
    #[c2rust::src_loc = "60:5"]
    pub const AV_CHAN_SIDE_RIGHT: AVChannel = 10;
    #[c2rust::src_loc = "59:5"]
    pub const AV_CHAN_SIDE_LEFT: AVChannel = 9;
    #[c2rust::src_loc = "58:5"]
    pub const AV_CHAN_BACK_CENTER: AVChannel = 8;
    #[c2rust::src_loc = "57:5"]
    pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: AVChannel = 7;
    #[c2rust::src_loc = "56:5"]
    pub const AV_CHAN_FRONT_LEFT_OF_CENTER: AVChannel = 6;
    #[c2rust::src_loc = "55:5"]
    pub const AV_CHAN_BACK_RIGHT: AVChannel = 5;
    #[c2rust::src_loc = "54:5"]
    pub const AV_CHAN_BACK_LEFT: AVChannel = 4;
    #[c2rust::src_loc = "53:5"]
    pub const AV_CHAN_LOW_FREQUENCY: AVChannel = 3;
    #[c2rust::src_loc = "52:5"]
    pub const AV_CHAN_FRONT_CENTER: AVChannel = 2;
    #[c2rust::src_loc = "51:5"]
    pub const AV_CHAN_FRONT_RIGHT: AVChannel = 1;
    #[c2rust::src_loc = "50:5"]
    pub const AV_CHAN_FRONT_LEFT: AVChannel = 0;
    #[c2rust::src_loc = "49:5"]
    pub const AV_CHAN_NONE: AVChannel = -1;
    #[c2rust::src_loc = "114:1"]
    pub type AVChannelOrder = ::core::ffi::c_uint;
    #[c2rust::src_loc = "159:5"]
    pub const FF_CHANNEL_ORDER_NB: AVChannelOrder = 4;
    #[c2rust::src_loc = "155:5"]
    pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;
    #[c2rust::src_loc = "132:5"]
    pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
    #[c2rust::src_loc = "125:5"]
    pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
    #[c2rust::src_loc = "119:5"]
    pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/usr/include/libavutil/dict.h:27"]
pub mod dict_h {
    extern "C" {
        #[c2rust::src_loc = "95:16"]
        pub type AVDictionary;
        #[c2rust::src_loc = "166:1"]
        pub fn av_dict_set(
            pm: *mut *mut AVDictionary,
            key: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
            flags: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "216:1"]
        pub fn av_dict_free(m: *mut *mut AVDictionary);
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixfmt.h:27"]
pub mod pixfmt_h {
    #[c2rust::src_loc = "786:1"]
    pub type AVChromaLocation = ::core::ffi::c_uint;
    #[c2rust::src_loc = "794:5"]
    pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
    #[c2rust::src_loc = "793:5"]
    pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
    #[c2rust::src_loc = "792:5"]
    pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
    #[c2rust::src_loc = "791:5"]
    pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
    #[c2rust::src_loc = "790:5"]
    pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
    #[c2rust::src_loc = "789:5"]
    pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
    #[c2rust::src_loc = "788:5"]
    pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
    #[c2rust::src_loc = "787:5"]
    pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
    #[c2rust::src_loc = "690:1"]
    pub type AVColorSpace = ::core::ffi::c_uint;
    #[c2rust::src_loc = "710:5"]
    pub const AVCOL_SPC_NB: AVColorSpace = 18;
    #[c2rust::src_loc = "709:5"]
    pub const AVCOL_SPC_YCGCO_RO: AVColorSpace = 17;
    #[c2rust::src_loc = "708:5"]
    pub const AVCOL_SPC_YCGCO_RE: AVColorSpace = 16;
    #[c2rust::src_loc = "707:5"]
    pub const AVCOL_SPC_IPT_C2: AVColorSpace = 15;
    #[c2rust::src_loc = "706:5"]
    pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
    #[c2rust::src_loc = "705:5"]
    pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
    #[c2rust::src_loc = "704:5"]
    pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
    #[c2rust::src_loc = "703:5"]
    pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
    #[c2rust::src_loc = "702:5"]
    pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
    #[c2rust::src_loc = "701:5"]
    pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
    #[c2rust::src_loc = "700:5"]
    pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
    #[c2rust::src_loc = "699:5"]
    pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
    #[c2rust::src_loc = "698:5"]
    pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
    #[c2rust::src_loc = "697:5"]
    pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
    #[c2rust::src_loc = "696:5"]
    pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
    #[c2rust::src_loc = "695:5"]
    pub const AVCOL_SPC_FCC: AVColorSpace = 4;
    #[c2rust::src_loc = "694:5"]
    pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
    #[c2rust::src_loc = "693:5"]
    pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
    #[c2rust::src_loc = "692:5"]
    pub const AVCOL_SPC_BT709: AVColorSpace = 1;
    #[c2rust::src_loc = "691:5"]
    pub const AVCOL_SPC_RGB: AVColorSpace = 0;
    #[c2rust::src_loc = "661:1"]
    pub type AVColorTransferCharacteristic = ::core::ffi::c_uint;
    #[c2rust::src_loc = "683:5"]
    pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
    #[c2rust::src_loc = "682:5"]
    pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
    #[c2rust::src_loc = "681:5"]
    pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
    #[c2rust::src_loc = "680:5"]
    pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
    #[c2rust::src_loc = "679:5"]
    pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
    #[c2rust::src_loc = "678:5"]
    pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
    #[c2rust::src_loc = "677:5"]
    pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
    #[c2rust::src_loc = "676:5"]
    pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
    #[c2rust::src_loc = "675:5"]
    pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
    #[c2rust::src_loc = "674:5"]
    pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
    #[c2rust::src_loc = "673:5"]
    pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
    #[c2rust::src_loc = "672:5"]
    pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
    #[c2rust::src_loc = "671:5"]
    pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
    #[c2rust::src_loc = "670:5"]
    pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
    #[c2rust::src_loc = "669:5"]
    pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
    #[c2rust::src_loc = "668:5"]
    pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
    #[c2rust::src_loc = "667:5"]
    pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
    #[c2rust::src_loc = "666:5"]
    pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
    #[c2rust::src_loc = "665:5"]
    pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
    #[c2rust::src_loc = "664:5"]
    pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
    #[c2rust::src_loc = "663:5"]
    pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
    #[c2rust::src_loc = "662:5"]
    pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
    #[c2rust::src_loc = "636:1"]
    pub type AVColorPrimaries = ::core::ffi::c_uint;
    #[c2rust::src_loc = "654:5"]
    pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
    #[c2rust::src_loc = "653:5"]
    pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
    #[c2rust::src_loc = "652:5"]
    pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
    #[c2rust::src_loc = "651:5"]
    pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
    #[c2rust::src_loc = "650:5"]
    pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
    #[c2rust::src_loc = "649:5"]
    pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
    #[c2rust::src_loc = "648:5"]
    pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
    #[c2rust::src_loc = "647:5"]
    pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
    #[c2rust::src_loc = "646:5"]
    pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
    #[c2rust::src_loc = "645:5"]
    pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
    #[c2rust::src_loc = "644:5"]
    pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
    #[c2rust::src_loc = "643:5"]
    pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
    #[c2rust::src_loc = "641:5"]
    pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
    #[c2rust::src_loc = "640:5"]
    pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
    #[c2rust::src_loc = "639:5"]
    pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
    #[c2rust::src_loc = "638:5"]
    pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
    #[c2rust::src_loc = "637:5"]
    pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
    #[c2rust::src_loc = "732:1"]
    pub type AVColorRange = ::core::ffi::c_uint;
    #[c2rust::src_loc = "768:5"]
    pub const AVCOL_RANGE_NB: AVColorRange = 3;
    #[c2rust::src_loc = "767:5"]
    pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
    #[c2rust::src_loc = "750:5"]
    pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
    #[c2rust::src_loc = "733:5"]
    pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
    #[c2rust::src_loc = "71:1"]
    pub type AVPixelFormat = ::core::ffi::c_int;
    #[c2rust::src_loc = "502:5"]
    pub const AV_PIX_FMT_NB: AVPixelFormat = 267;
    #[c2rust::src_loc = "500:5"]
    pub const AV_PIX_FMT_OHCODEC: AVPixelFormat = 266;
    #[c2rust::src_loc = "498:5"]
    pub const AV_PIX_FMT_GBRP12MSBLE: AVPixelFormat = 265;
    #[c2rust::src_loc = "497:5"]
    pub const AV_PIX_FMT_GBRP12MSBBE: AVPixelFormat = 264;
    #[c2rust::src_loc = "496:5"]
    pub const AV_PIX_FMT_GBRP10MSBLE: AVPixelFormat = 263;
    #[c2rust::src_loc = "495:5"]
    pub const AV_PIX_FMT_GBRP10MSBBE: AVPixelFormat = 262;
    #[c2rust::src_loc = "494:5"]
    pub const AV_PIX_FMT_YUV444P12MSBLE: AVPixelFormat = 261;
    #[c2rust::src_loc = "493:5"]
    pub const AV_PIX_FMT_YUV444P12MSBBE: AVPixelFormat = 260;
    #[c2rust::src_loc = "492:5"]
    pub const AV_PIX_FMT_YUV444P10MSBLE: AVPixelFormat = 259;
    #[c2rust::src_loc = "491:5"]
    pub const AV_PIX_FMT_YUV444P10MSBBE: AVPixelFormat = 258;
    #[c2rust::src_loc = "489:5"]
    pub const AV_PIX_FMT_GBRAP32LE: AVPixelFormat = 257;
    #[c2rust::src_loc = "488:5"]
    pub const AV_PIX_FMT_GBRAP32BE: AVPixelFormat = 256;
    #[c2rust::src_loc = "486:5"]
    pub const AV_PIX_FMT_YAF16LE: AVPixelFormat = 255;
    #[c2rust::src_loc = "485:5"]
    pub const AV_PIX_FMT_YAF16BE: AVPixelFormat = 254;
    #[c2rust::src_loc = "483:5"]
    pub const AV_PIX_FMT_YAF32LE: AVPixelFormat = 253;
    #[c2rust::src_loc = "482:5"]
    pub const AV_PIX_FMT_YAF32BE: AVPixelFormat = 252;
    #[c2rust::src_loc = "480:5"]
    pub const AV_PIX_FMT_GRAY32LE: AVPixelFormat = 251;
    #[c2rust::src_loc = "479:5"]
    pub const AV_PIX_FMT_GRAY32BE: AVPixelFormat = 250;
    #[c2rust::src_loc = "477:5"]
    pub const AV_PIX_FMT_AMF_SURFACE: AVPixelFormat = 249;
    #[c2rust::src_loc = "472:5"]
    pub const AV_PIX_FMT_GRAYF16LE: AVPixelFormat = 248;
    #[c2rust::src_loc = "471:5"]
    pub const AV_PIX_FMT_GRAYF16BE: AVPixelFormat = 247;
    #[c2rust::src_loc = "469:5"]
    pub const AV_PIX_FMT_GBRAPF16LE: AVPixelFormat = 246;
    #[c2rust::src_loc = "468:5"]
    pub const AV_PIX_FMT_GBRAPF16BE: AVPixelFormat = 245;
    #[c2rust::src_loc = "467:5"]
    pub const AV_PIX_FMT_GBRPF16LE: AVPixelFormat = 244;
    #[c2rust::src_loc = "466:5"]
    pub const AV_PIX_FMT_GBRPF16BE: AVPixelFormat = 243;
    #[c2rust::src_loc = "464:5"]
    pub const AV_PIX_FMT_XV48LE: AVPixelFormat = 242;
    #[c2rust::src_loc = "463:5"]
    pub const AV_PIX_FMT_XV48BE: AVPixelFormat = 241;
    #[c2rust::src_loc = "461:5"]
    pub const AV_PIX_FMT_Y216LE: AVPixelFormat = 240;
    #[c2rust::src_loc = "460:5"]
    pub const AV_PIX_FMT_Y216BE: AVPixelFormat = 239;
    #[c2rust::src_loc = "458:5"]
    pub const AV_PIX_FMT_RGB96LE: AVPixelFormat = 238;
    #[c2rust::src_loc = "457:5"]
    pub const AV_PIX_FMT_RGB96BE: AVPixelFormat = 237;
    #[c2rust::src_loc = "455:5"]
    pub const AV_PIX_FMT_RGBA128LE: AVPixelFormat = 236;
    #[c2rust::src_loc = "454:5"]
    pub const AV_PIX_FMT_RGBA128BE: AVPixelFormat = 235;
    #[c2rust::src_loc = "452:5"]
    pub const AV_PIX_FMT_RGBF16LE: AVPixelFormat = 234;
    #[c2rust::src_loc = "451:5"]
    pub const AV_PIX_FMT_RGBF16BE: AVPixelFormat = 233;
    #[c2rust::src_loc = "449:5"]
    pub const AV_PIX_FMT_V30XLE: AVPixelFormat = 232;
    #[c2rust::src_loc = "448:5"]
    pub const AV_PIX_FMT_V30XBE: AVPixelFormat = 231;
    #[c2rust::src_loc = "446:5"]
    pub const AV_PIX_FMT_VYU444: AVPixelFormat = 230;
    #[c2rust::src_loc = "444:5"]
    pub const AV_PIX_FMT_UYVA: AVPixelFormat = 229;
    #[c2rust::src_loc = "442:5"]
    pub const AV_PIX_FMT_AYUV: AVPixelFormat = 228;
    #[c2rust::src_loc = "440:5"]
    pub const AV_PIX_FMT_D3D12: AVPixelFormat = 227;
    #[c2rust::src_loc = "433:5"]
    pub const AV_PIX_FMT_GBRAP14LE: AVPixelFormat = 226;
    #[c2rust::src_loc = "432:5"]
    pub const AV_PIX_FMT_GBRAP14BE: AVPixelFormat = 225;
    #[c2rust::src_loc = "430:5"]
    pub const AV_PIX_FMT_P412LE: AVPixelFormat = 224;
    #[c2rust::src_loc = "429:5"]
    pub const AV_PIX_FMT_P412BE: AVPixelFormat = 223;
    #[c2rust::src_loc = "427:5"]
    pub const AV_PIX_FMT_P212LE: AVPixelFormat = 222;
    #[c2rust::src_loc = "426:5"]
    pub const AV_PIX_FMT_P212BE: AVPixelFormat = 221;
    #[c2rust::src_loc = "424:5"]
    pub const AV_PIX_FMT_RGBAF32LE: AVPixelFormat = 220;
    #[c2rust::src_loc = "423:5"]
    pub const AV_PIX_FMT_RGBAF32BE: AVPixelFormat = 219;
    #[c2rust::src_loc = "421:5"]
    pub const AV_PIX_FMT_RGBF32LE: AVPixelFormat = 218;
    #[c2rust::src_loc = "420:5"]
    pub const AV_PIX_FMT_RGBF32BE: AVPixelFormat = 217;
    #[c2rust::src_loc = "418:5"]
    pub const AV_PIX_FMT_XV36LE: AVPixelFormat = 216;
    #[c2rust::src_loc = "417:5"]
    pub const AV_PIX_FMT_XV36BE: AVPixelFormat = 215;
    #[c2rust::src_loc = "415:5"]
    pub const AV_PIX_FMT_XV30LE: AVPixelFormat = 214;
    #[c2rust::src_loc = "414:5"]
    pub const AV_PIX_FMT_XV30BE: AVPixelFormat = 213;
    #[c2rust::src_loc = "412:5"]
    pub const AV_PIX_FMT_Y212LE: AVPixelFormat = 212;
    #[c2rust::src_loc = "411:5"]
    pub const AV_PIX_FMT_Y212BE: AVPixelFormat = 211;
    #[c2rust::src_loc = "409:5"]
    pub const AV_PIX_FMT_P012BE: AVPixelFormat = 210;
    #[c2rust::src_loc = "408:5"]
    pub const AV_PIX_FMT_P012LE: AVPixelFormat = 209;
    #[c2rust::src_loc = "406:5"]
    pub const AV_PIX_FMT_VUYX: AVPixelFormat = 208;
    #[c2rust::src_loc = "404:5"]
    pub const AV_PIX_FMT_RGBAF16LE: AVPixelFormat = 207;
    #[c2rust::src_loc = "403:5"]
    pub const AV_PIX_FMT_RGBAF16BE: AVPixelFormat = 206;
    #[c2rust::src_loc = "401:5"]
    pub const AV_PIX_FMT_VUYA: AVPixelFormat = 205;
    #[c2rust::src_loc = "399:5"]
    pub const AV_PIX_FMT_P416LE: AVPixelFormat = 204;
    #[c2rust::src_loc = "398:5"]
    pub const AV_PIX_FMT_P416BE: AVPixelFormat = 203;
    #[c2rust::src_loc = "396:5"]
    pub const AV_PIX_FMT_P216LE: AVPixelFormat = 202;
    #[c2rust::src_loc = "395:5"]
    pub const AV_PIX_FMT_P216BE: AVPixelFormat = 201;
    #[c2rust::src_loc = "393:5"]
    pub const AV_PIX_FMT_P410LE: AVPixelFormat = 200;
    #[c2rust::src_loc = "392:5"]
    pub const AV_PIX_FMT_P410BE: AVPixelFormat = 199;
    #[c2rust::src_loc = "390:5"]
    pub const AV_PIX_FMT_P210LE: AVPixelFormat = 198;
    #[c2rust::src_loc = "389:5"]
    pub const AV_PIX_FMT_P210BE: AVPixelFormat = 197;
    #[c2rust::src_loc = "387:5"]
    pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 196;
    #[c2rust::src_loc = "386:5"]
    pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 195;
    #[c2rust::src_loc = "385:5"]
    pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 194;
    #[c2rust::src_loc = "384:5"]
    pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 193;
    #[c2rust::src_loc = "382:5"]
    pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 192;
    #[c2rust::src_loc = "381:5"]
    pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 191;
    #[c2rust::src_loc = "379:5"]
    pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 190;
    #[c2rust::src_loc = "372:5"]
    pub const AV_PIX_FMT_NV42: AVPixelFormat = 189;
    #[c2rust::src_loc = "371:5"]
    pub const AV_PIX_FMT_NV24: AVPixelFormat = 188;
    #[c2rust::src_loc = "369:5"]
    pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 187;
    #[c2rust::src_loc = "368:5"]
    pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 186;
    #[c2rust::src_loc = "367:5"]
    pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 185;
    #[c2rust::src_loc = "366:5"]
    pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 184;
    #[c2rust::src_loc = "364:5"]
    pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 183;
    #[c2rust::src_loc = "363:5"]
    pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 182;
    #[c2rust::src_loc = "361:5"]
    pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 181;
    #[c2rust::src_loc = "360:5"]
    pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 180;
    #[c2rust::src_loc = "358:5"]
    pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 179;
    #[c2rust::src_loc = "351:5"]
    pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 178;
    #[c2rust::src_loc = "344:5"]
    pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 177;
    #[c2rust::src_loc = "343:5"]
    pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 176;
    #[c2rust::src_loc = "342:5"]
    pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 175;
    #[c2rust::src_loc = "341:5"]
    pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 174;
    #[c2rust::src_loc = "339:5"]
    pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 173;
    #[c2rust::src_loc = "338:5"]
    pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 172;
    #[c2rust::src_loc = "336:5"]
    pub const AV_PIX_FMT_D3D11: AVPixelFormat = 171;
    #[c2rust::src_loc = "324:5"]
    pub const AV_PIX_FMT_P016BE: AVPixelFormat = 170;
    #[c2rust::src_loc = "323:5"]
    pub const AV_PIX_FMT_P016LE: AVPixelFormat = 169;
    #[c2rust::src_loc = "321:5"]
    pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 168;
    #[c2rust::src_loc = "320:5"]
    pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 167;
    #[c2rust::src_loc = "319:5"]
    pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 166;
    #[c2rust::src_loc = "318:5"]
    pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 165;
    #[c2rust::src_loc = "316:5"]
    pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 164;
    #[c2rust::src_loc = "314:5"]
    pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 163;
    #[c2rust::src_loc = "313:5"]
    pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 162;
    #[c2rust::src_loc = "311:5"]
    pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 161;
    #[c2rust::src_loc = "310:5"]
    pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 160;
    #[c2rust::src_loc = "308:5"]
    pub const AV_PIX_FMT_P010BE: AVPixelFormat = 159;
    #[c2rust::src_loc = "307:5"]
    pub const AV_PIX_FMT_P010LE: AVPixelFormat = 158;
    #[c2rust::src_loc = "305:5"]
    pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 157;
    #[c2rust::src_loc = "303:5"]
    pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 156;
    #[c2rust::src_loc = "302:5"]
    pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 155;
    #[c2rust::src_loc = "301:5"]
    pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 154;
    #[c2rust::src_loc = "300:5"]
    pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 153;
    #[c2rust::src_loc = "299:5"]
    pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 152;
    #[c2rust::src_loc = "298:5"]
    pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 151;
    #[c2rust::src_loc = "296:5"]
    pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150;
    #[c2rust::src_loc = "295:5"]
    pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149;
    #[c2rust::src_loc = "294:5"]
    pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148;
    #[c2rust::src_loc = "293:5"]
    pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147;
    #[c2rust::src_loc = "292:5"]
    pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146;
    #[c2rust::src_loc = "291:5"]
    pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145;
    #[c2rust::src_loc = "290:5"]
    pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144;
    #[c2rust::src_loc = "289:5"]
    pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143;
    #[c2rust::src_loc = "288:5"]
    pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142;
    #[c2rust::src_loc = "287:5"]
    pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141;
    #[c2rust::src_loc = "286:5"]
    pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140;
    #[c2rust::src_loc = "285:5"]
    pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139;
    #[c2rust::src_loc = "283:5"]
    pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138;
    #[c2rust::src_loc = "282:5"]
    pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137;
    #[c2rust::src_loc = "281:5"]
    pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136;
    #[c2rust::src_loc = "280:5"]
    pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135;
    #[c2rust::src_loc = "279:5"]
    pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134;
    #[c2rust::src_loc = "278:5"]
    pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133;
    #[c2rust::src_loc = "277:5"]
    pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132;
    #[c2rust::src_loc = "276:5"]
    pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131;
    #[c2rust::src_loc = "275:5"]
    pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130;
    #[c2rust::src_loc = "274:5"]
    pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129;
    #[c2rust::src_loc = "273:5"]
    pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128;
    #[c2rust::src_loc = "272:5"]
    pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127;
    #[c2rust::src_loc = "271:5"]
    pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126;
    #[c2rust::src_loc = "270:5"]
    pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125;
    #[c2rust::src_loc = "269:5"]
    pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124;
    #[c2rust::src_loc = "268:5"]
    pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123;
    #[c2rust::src_loc = "267:5"]
    pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122;
    #[c2rust::src_loc = "265:5"]
    pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121;
    #[c2rust::src_loc = "264:5"]
    pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120;
    #[c2rust::src_loc = "263:5"]
    pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119;
    #[c2rust::src_loc = "262:5"]
    pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118;
    #[c2rust::src_loc = "260:5"]
    pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;
    #[c2rust::src_loc = "254:5"]
    pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116;
    #[c2rust::src_loc = "252:5"]
    pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;
    #[c2rust::src_loc = "247:5"]
    pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
    #[c2rust::src_loc = "214:5"]
    pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113;
    #[c2rust::src_loc = "213:5"]
    pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112;
    #[c2rust::src_loc = "212:5"]
    pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111;
    #[c2rust::src_loc = "210:5"]
    pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110;
    #[c2rust::src_loc = "209:5"]
    pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109;
    #[c2rust::src_loc = "207:5"]
    pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108;
    #[c2rust::src_loc = "205:5"]
    pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
    #[c2rust::src_loc = "204:5"]
    pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106;
    #[c2rust::src_loc = "203:5"]
    pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105;
    #[c2rust::src_loc = "202:5"]
    pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104;
    #[c2rust::src_loc = "200:5"]
    pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103;
    #[c2rust::src_loc = "199:5"]
    pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102;
    #[c2rust::src_loc = "198:5"]
    pub const AV_PIX_FMT_NV16: AVPixelFormat = 101;
    #[c2rust::src_loc = "197:5"]
    pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100;
    #[c2rust::src_loc = "196:5"]
    pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99;
    #[c2rust::src_loc = "194:5"]
    pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98;
    #[c2rust::src_loc = "192:5"]
    pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97;
    #[c2rust::src_loc = "191:5"]
    pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96;
    #[c2rust::src_loc = "190:5"]
    pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95;
    #[c2rust::src_loc = "189:5"]
    pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94;
    #[c2rust::src_loc = "188:5"]
    pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93;
    #[c2rust::src_loc = "187:5"]
    pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92;
    #[c2rust::src_loc = "186:5"]
    pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91;
    #[c2rust::src_loc = "185:5"]
    pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90;
    #[c2rust::src_loc = "184:5"]
    pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89;
    #[c2rust::src_loc = "183:5"]
    pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88;
    #[c2rust::src_loc = "182:5"]
    pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87;
    #[c2rust::src_loc = "181:5"]
    pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86;
    #[c2rust::src_loc = "180:5"]
    pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85;
    #[c2rust::src_loc = "179:5"]
    pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84;
    #[c2rust::src_loc = "178:5"]
    pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83;
    #[c2rust::src_loc = "177:5"]
    pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82;
    #[c2rust::src_loc = "176:5"]
    pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81;
    #[c2rust::src_loc = "175:5"]
    pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80;
    #[c2rust::src_loc = "174:5"]
    pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79;
    #[c2rust::src_loc = "173:5"]
    pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78;
    #[c2rust::src_loc = "172:5"]
    pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77;
    #[c2rust::src_loc = "171:5"]
    pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76;
    #[c2rust::src_loc = "170:5"]
    pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75;
    #[c2rust::src_loc = "169:5"]
    pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74;
    #[c2rust::src_loc = "168:5"]
    pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73;
    #[c2rust::src_loc = "167:5"]
    pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72;
    #[c2rust::src_loc = "166:5"]
    pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 71;
    #[c2rust::src_loc = "165:5"]
    pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71;
    #[c2rust::src_loc = "164:5"]
    pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70;
    #[c2rust::src_loc = "163:5"]
    pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69;
    #[c2rust::src_loc = "162:5"]
    pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68;
    #[c2rust::src_loc = "161:5"]
    pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67;
    #[c2rust::src_loc = "160:5"]
    pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66;
    #[c2rust::src_loc = "159:5"]
    pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65;
    #[c2rust::src_loc = "158:5"]
    pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64;
    #[c2rust::src_loc = "157:5"]
    pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63;
    #[c2rust::src_loc = "156:5"]
    pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62;
    #[c2rust::src_loc = "155:5"]
    pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61;
    #[c2rust::src_loc = "154:5"]
    pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60;
    #[c2rust::src_loc = "153:5"]
    pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59;
    #[c2rust::src_loc = "146:5"]
    pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
    #[c2rust::src_loc = "145:5"]
    pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57;
    #[c2rust::src_loc = "143:5"]
    pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 56;
    #[c2rust::src_loc = "142:5"]
    pub const AV_PIX_FMT_Y400A: AVPixelFormat = 56;
    #[c2rust::src_loc = "140:5"]
    pub const AV_PIX_FMT_YA8: AVPixelFormat = 56;
    #[c2rust::src_loc = "139:5"]
    pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55;
    #[c2rust::src_loc = "138:5"]
    pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54;
    #[c2rust::src_loc = "137:5"]
    pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53;
    #[c2rust::src_loc = "136:5"]
    pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52;
    #[c2rust::src_loc = "134:5"]
    pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51;
    #[c2rust::src_loc = "133:5"]
    pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50;
    #[c2rust::src_loc = "132:5"]
    pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
    #[c2rust::src_loc = "131:5"]
    pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
    #[c2rust::src_loc = "130:5"]
    pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
    #[c2rust::src_loc = "129:5"]
    pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
    #[c2rust::src_loc = "128:5"]
    pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
    #[c2rust::src_loc = "126:5"]
    pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;
    #[c2rust::src_loc = "120:5"]
    pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
    #[c2rust::src_loc = "119:5"]
    pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
    #[c2rust::src_loc = "118:5"]
    pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
    #[c2rust::src_loc = "117:5"]
    pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
    #[c2rust::src_loc = "115:5"]
    pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
    #[c2rust::src_loc = "114:5"]
    pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
    #[c2rust::src_loc = "113:5"]
    pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
    #[c2rust::src_loc = "112:5"]
    pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
    #[c2rust::src_loc = "110:5"]
    pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
    #[c2rust::src_loc = "109:5"]
    pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
    #[c2rust::src_loc = "108:5"]
    pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
    #[c2rust::src_loc = "107:5"]
    pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
    #[c2rust::src_loc = "106:5"]
    pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
    #[c2rust::src_loc = "105:5"]
    pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
    #[c2rust::src_loc = "104:5"]
    pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
    #[c2rust::src_loc = "102:5"]
    pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
    #[c2rust::src_loc = "101:5"]
    pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
    #[c2rust::src_loc = "100:5"]
    pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
    #[c2rust::src_loc = "99:5"]
    pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
    #[c2rust::src_loc = "97:5"]
    pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
    #[c2rust::src_loc = "96:5"]
    pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
    #[c2rust::src_loc = "95:5"]
    pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
    #[c2rust::src_loc = "94:5"]
    pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
    #[c2rust::src_loc = "93:5"]
    pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
    #[c2rust::src_loc = "92:5"]
    pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
    #[c2rust::src_loc = "91:5"]
    pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
    #[c2rust::src_loc = "90:5"]
    pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
    #[c2rust::src_loc = "89:5"]
    pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
    #[c2rust::src_loc = "88:5"]
    pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
    #[c2rust::src_loc = "87:5"]
    pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
    #[c2rust::src_loc = "86:5"]
    pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
    #[c2rust::src_loc = "85:5"]
    pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
    #[c2rust::src_loc = "84:5"]
    pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
    #[c2rust::src_loc = "83:5"]
    pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
    #[c2rust::src_loc = "82:5"]
    pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
    #[c2rust::src_loc = "81:5"]
    pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
    #[c2rust::src_loc = "80:5"]
    pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
    #[c2rust::src_loc = "79:5"]
    pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
    #[c2rust::src_loc = "78:5"]
    pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
    #[c2rust::src_loc = "77:5"]
    pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
    #[c2rust::src_loc = "76:5"]
    pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
    #[c2rust::src_loc = "75:5"]
    pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
    #[c2rust::src_loc = "74:5"]
    pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
    #[c2rust::src_loc = "73:5"]
    pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
    #[c2rust::src_loc = "72:5"]
    pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
}
#[c2rust::header_src = "/usr/include/libavutil/avutil.h:27"]
pub mod avutil_h {
    #[c2rust::src_loc = "276:1"]
    pub type AVPictureType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "284:5"]
    pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
    #[c2rust::src_loc = "283:5"]
    pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
    #[c2rust::src_loc = "282:5"]
    pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
    #[c2rust::src_loc = "281:5"]
    pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
    #[c2rust::src_loc = "280:5"]
    pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
    #[c2rust::src_loc = "279:5"]
    pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
    #[c2rust::src_loc = "278:5"]
    pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
    #[c2rust::src_loc = "277:5"]
    pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
    #[c2rust::src_loc = "198:1"]
    pub type AVMediaType = ::core::ffi::c_int;
    #[c2rust::src_loc = "205:5"]
    pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
    #[c2rust::src_loc = "204:5"]
    pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
    #[c2rust::src_loc = "203:5"]
    pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
    #[c2rust::src_loc = "202:5"]
    pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
    #[c2rust::src_loc = "201:5"]
    pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
    #[c2rust::src_loc = "200:5"]
    pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
    #[c2rust::src_loc = "199:5"]
    pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
    #[c2rust::src_loc = "247:9"]
    pub const AV_NOPTS_VALUE: int64_t = 0x8000000000000000 as ::core::ffi::c_ulong as int64_t;
    use super::stdint_intn_h::int64_t;
}
#[c2rust::header_src = "/usr/include/libavcodec/avcodec.h:30"]
pub mod avcodec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "431:16"]
    pub struct AVCodecContext {
        pub av_class: *const AVClass,
        pub log_level_offset: ::core::ffi::c_int,
        pub codec_type: AVMediaType,
        pub codec: *const AVCodec,
        pub codec_id: AVCodecID,
        pub codec_tag: ::core::ffi::c_uint,
        pub priv_data: *mut ::core::ffi::c_void,
        pub internal: *mut AVCodecInternal,
        pub opaque: *mut ::core::ffi::c_void,
        pub bit_rate: int64_t,
        pub flags: ::core::ffi::c_int,
        pub flags2: ::core::ffi::c_int,
        pub extradata: *mut uint8_t,
        pub extradata_size: ::core::ffi::c_int,
        pub time_base: AVRational,
        pub pkt_timebase: AVRational,
        pub framerate: AVRational,
        pub delay: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub coded_width: ::core::ffi::c_int,
        pub coded_height: ::core::ffi::c_int,
        pub sample_aspect_ratio: AVRational,
        pub pix_fmt: AVPixelFormat,
        pub sw_pix_fmt: AVPixelFormat,
        pub color_primaries: AVColorPrimaries,
        pub color_trc: AVColorTransferCharacteristic,
        pub colorspace: AVColorSpace,
        pub color_range: AVColorRange,
        pub chroma_sample_location: AVChromaLocation,
        pub field_order: AVFieldOrder,
        pub refs: ::core::ffi::c_int,
        pub has_b_frames: ::core::ffi::c_int,
        pub slice_flags: ::core::ffi::c_int,
        pub draw_horiz_band: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *const AVFrame,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub get_format: Option<
            unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
        >,
        pub max_b_frames: ::core::ffi::c_int,
        pub b_quant_factor: ::core::ffi::c_float,
        pub b_quant_offset: ::core::ffi::c_float,
        pub i_quant_factor: ::core::ffi::c_float,
        pub i_quant_offset: ::core::ffi::c_float,
        pub lumi_masking: ::core::ffi::c_float,
        pub temporal_cplx_masking: ::core::ffi::c_float,
        pub spatial_cplx_masking: ::core::ffi::c_float,
        pub p_masking: ::core::ffi::c_float,
        pub dark_masking: ::core::ffi::c_float,
        pub nsse_weight: ::core::ffi::c_int,
        pub me_cmp: ::core::ffi::c_int,
        pub me_sub_cmp: ::core::ffi::c_int,
        pub mb_cmp: ::core::ffi::c_int,
        pub ildct_cmp: ::core::ffi::c_int,
        pub dia_size: ::core::ffi::c_int,
        pub last_predictor_count: ::core::ffi::c_int,
        pub me_pre_cmp: ::core::ffi::c_int,
        pub pre_dia_size: ::core::ffi::c_int,
        pub me_subpel_quality: ::core::ffi::c_int,
        pub me_range: ::core::ffi::c_int,
        pub mb_decision: ::core::ffi::c_int,
        pub intra_matrix: *mut uint16_t,
        pub inter_matrix: *mut uint16_t,
        pub chroma_intra_matrix: *mut uint16_t,
        pub intra_dc_precision: ::core::ffi::c_int,
        pub mb_lmin: ::core::ffi::c_int,
        pub mb_lmax: ::core::ffi::c_int,
        pub bidir_refine: ::core::ffi::c_int,
        pub keyint_min: ::core::ffi::c_int,
        pub gop_size: ::core::ffi::c_int,
        pub mv0_threshold: ::core::ffi::c_int,
        pub slices: ::core::ffi::c_int,
        pub sample_rate: ::core::ffi::c_int,
        pub sample_fmt: AVSampleFormat,
        pub ch_layout: AVChannelLayout,
        pub frame_size: ::core::ffi::c_int,
        pub block_align: ::core::ffi::c_int,
        pub cutoff: ::core::ffi::c_int,
        pub audio_service_type: AVAudioServiceType,
        pub request_sample_fmt: AVSampleFormat,
        pub initial_padding: ::core::ffi::c_int,
        pub trailing_padding: ::core::ffi::c_int,
        pub seek_preroll: ::core::ffi::c_int,
        pub get_buffer2: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVFrame,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub bit_rate_tolerance: ::core::ffi::c_int,
        pub global_quality: ::core::ffi::c_int,
        pub compression_level: ::core::ffi::c_int,
        pub qcompress: ::core::ffi::c_float,
        pub qblur: ::core::ffi::c_float,
        pub qmin: ::core::ffi::c_int,
        pub qmax: ::core::ffi::c_int,
        pub max_qdiff: ::core::ffi::c_int,
        pub rc_buffer_size: ::core::ffi::c_int,
        pub rc_override_count: ::core::ffi::c_int,
        pub rc_override: *mut RcOverride,
        pub rc_max_rate: int64_t,
        pub rc_min_rate: int64_t,
        pub rc_max_available_vbv_use: ::core::ffi::c_float,
        pub rc_min_vbv_overflow_use: ::core::ffi::c_float,
        pub rc_initial_buffer_occupancy: ::core::ffi::c_int,
        pub trellis: ::core::ffi::c_int,
        pub stats_out: *mut ::core::ffi::c_char,
        pub stats_in: *mut ::core::ffi::c_char,
        pub workaround_bugs: ::core::ffi::c_int,
        pub strict_std_compliance: ::core::ffi::c_int,
        pub error_concealment: ::core::ffi::c_int,
        pub debug: ::core::ffi::c_int,
        pub err_recognition: ::core::ffi::c_int,
        pub hwaccel: *const AVHWAccel,
        pub hwaccel_context: *mut ::core::ffi::c_void,
        pub hw_frames_ctx: *mut AVBufferRef,
        pub hw_device_ctx: *mut AVBufferRef,
        pub hwaccel_flags: ::core::ffi::c_int,
        pub extra_hw_frames: ::core::ffi::c_int,
        pub error: [uint64_t; 8],
        pub dct_algo: ::core::ffi::c_int,
        pub idct_algo: ::core::ffi::c_int,
        pub bits_per_coded_sample: ::core::ffi::c_int,
        pub bits_per_raw_sample: ::core::ffi::c_int,
        pub thread_count: ::core::ffi::c_int,
        pub thread_type: ::core::ffi::c_int,
        pub active_thread_type: ::core::ffi::c_int,
        pub execute: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                Option<
                    unsafe extern "C" fn(
                        *mut AVCodecContext,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub execute2: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                Option<
                    unsafe extern "C" fn(
                        *mut AVCodecContext,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub profile: ::core::ffi::c_int,
        pub level: ::core::ffi::c_int,
        pub properties: ::core::ffi::c_uint,
        pub skip_loop_filter: AVDiscard,
        pub skip_idct: AVDiscard,
        pub skip_frame: AVDiscard,
        pub skip_alpha: ::core::ffi::c_int,
        pub skip_top: ::core::ffi::c_int,
        pub skip_bottom: ::core::ffi::c_int,
        pub lowres: ::core::ffi::c_int,
        pub codec_descriptor: *const AVCodecDescriptor,
        pub sub_charenc: *mut ::core::ffi::c_char,
        pub sub_charenc_mode: ::core::ffi::c_int,
        pub subtitle_header_size: ::core::ffi::c_int,
        pub subtitle_header: *mut uint8_t,
        pub dump_separator: *mut uint8_t,
        pub codec_whitelist: *mut ::core::ffi::c_char,
        pub coded_side_data: *mut AVPacketSideData,
        pub nb_coded_side_data: ::core::ffi::c_int,
        pub export_side_data: ::core::ffi::c_int,
        pub max_pixels: int64_t,
        pub apply_cropping: ::core::ffi::c_int,
        pub discard_damaged_percentage: ::core::ffi::c_int,
        pub max_samples: int64_t,
        pub get_encode_buffer: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVPacket,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub frame_num: int64_t,
        pub side_data_prefer_packet: *mut ::core::ffi::c_int,
        pub nb_side_data_prefer_packet: ::core::ffi::c_uint,
        pub decoded_side_data: *mut *mut AVFrameSideData,
        pub nb_decoded_side_data: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct AVHWAccel {
        pub name: *const ::core::ffi::c_char,
        pub type_0: AVMediaType,
        pub id: AVCodecID,
        pub pix_fmt: AVPixelFormat,
        pub capabilities: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "193:16"]
    pub struct RcOverride {
        pub start_frame: ::core::ffi::c_int,
        pub end_frame: ::core::ffi::c_int,
        pub qscale: ::core::ffi::c_int,
        pub quality_factor: ::core::ffi::c_float,
    }
    use super::avutil_h::AVMediaType;
    use super::buffer_h::AVBufferRef;
    use super::channel_layout_h::AVChannelLayout;
    use super::codec_desc_h::AVCodecDescriptor;
    use super::codec_h::AVCodec;
    use super::codec_id_h::AVCodecID;
    use super::codec_par_h::AVCodecParameters;
    use super::defs_h::{AVAudioServiceType, AVDiscard, AVFieldOrder};
    use super::dict_h::AVDictionary;
    use super::frame_h::{AVFrame, AVFrameSideData};
    use super::log_h::AVClass;
    use super::packet_h::{AVPacket, AVPacketSideData};
    use super::pixfmt_h::{
        AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic, AVPixelFormat,
    };
    use super::rational_h::AVRational;
    use super::samplefmt_h::AVSampleFormat;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint16_t, uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "466:12"]
        pub type AVCodecInternal;
        #[c2rust::src_loc = "2112:1"]
        pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
        #[c2rust::src_loc = "2118:1"]
        pub fn avcodec_free_context(avctx: *mut *mut AVCodecContext);
        #[c2rust::src_loc = "2154:1"]
        pub fn avcodec_parameters_to_context(
            codec: *mut AVCodecContext,
            par: *const AVCodecParameters,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2218:1"]
        pub fn avcodec_open2(
            avctx: *mut AVCodecContext,
            codec: *const AVCodec,
            options: *mut *mut AVDictionary,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2345:1"]
        pub fn avcodec_send_packet(
            avctx: *mut AVCodecContext,
            avpkt: *const AVPacket,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2366:1"]
        pub fn avcodec_receive_frame(
            avctx: *mut AVCodecContext,
            frame: *mut AVFrame,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavcodec/codec_desc.h:30"]
pub mod codec_desc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct AVCodecDescriptor {
        pub id: AVCodecID,
        pub type_0: AVMediaType,
        pub name: *const ::core::ffi::c_char,
        pub long_name: *const ::core::ffi::c_char,
        pub props: ::core::ffi::c_int,
        pub mime_types: *const *const ::core::ffi::c_char,
        pub profiles: *const AVProfile,
    }
    use super::avutil_h::AVMediaType;
    use super::codec_h::AVProfile;
    use super::codec_id_h::AVCodecID;
}
#[c2rust::header_src = "/usr/include/libavcodec/codec.h:30"]
pub mod codec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:16"]
    pub struct AVProfile {
        pub profile: ::core::ffi::c_int,
        pub name: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "172:16"]
    pub struct AVCodec {
        pub name: *const ::core::ffi::c_char,
        pub long_name: *const ::core::ffi::c_char,
        pub type_0: AVMediaType,
        pub id: AVCodecID,
        pub capabilities: ::core::ffi::c_int,
        pub max_lowres: uint8_t,
        pub supported_framerates: *const AVRational,
        pub pix_fmts: *const AVPixelFormat,
        pub supported_samplerates: *const ::core::ffi::c_int,
        pub sample_fmts: *const AVSampleFormat,
        pub priv_class: *const AVClass,
        pub profiles: *const AVProfile,
        pub wrapper_name: *const ::core::ffi::c_char,
        pub ch_layouts: *const AVChannelLayout,
    }
    use super::avutil_h::AVMediaType;
    use super::channel_layout_h::AVChannelLayout;
    use super::codec_id_h::{AVCodecID, AV_CODEC_ID_NONE};
    use super::log_h::AVClass;
    use super::pixfmt_h::AVPixelFormat;
    use super::rational_h::AVRational;
    use super::samplefmt_h::AVSampleFormat;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "246:1"]
        pub fn avcodec_find_decoder(id: AVCodecID) -> *const AVCodec;
    }
}
#[c2rust::header_src = "/usr/include/libavcodec/codec_id.h:27"]
pub mod codec_id_h {
    #[c2rust::src_loc = "49:1"]
    pub type AVCodecID = ::core::ffi::c_uint;
    #[c2rust::src_loc = "626:5"]
    pub const AV_CODEC_ID_ANULL: AVCodecID = 135171;
    #[c2rust::src_loc = "621:5"]
    pub const AV_CODEC_ID_VNULL: AVCodecID = 135170;
    #[c2rust::src_loc = "616:5"]
    pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
    #[c2rust::src_loc = "615:5"]
    pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
    #[c2rust::src_loc = "613:5"]
    pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
    #[c2rust::src_loc = "611:5"]
    pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
    #[c2rust::src_loc = "609:5"]
    pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
    #[c2rust::src_loc = "606:5"]
    pub const AV_CODEC_ID_SMPTE_436M_ANC: AVCodecID = 98317;
    #[c2rust::src_loc = "605:5"]
    pub const AV_CODEC_ID_LCEVC: AVCodecID = 98316;
    #[c2rust::src_loc = "604:5"]
    pub const AV_CODEC_ID_SMPTE_2038: AVCodecID = 98315;
    #[c2rust::src_loc = "603:5"]
    pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 98314;
    #[c2rust::src_loc = "602:5"]
    pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 98313;
    #[c2rust::src_loc = "601:5"]
    pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 98312;
    #[c2rust::src_loc = "600:5"]
    pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 98311;
    #[c2rust::src_loc = "599:5"]
    pub const AV_CODEC_ID_OTF: AVCodecID = 98310;
    #[c2rust::src_loc = "598:5"]
    pub const AV_CODEC_ID_IDF: AVCodecID = 98309;
    #[c2rust::src_loc = "597:5"]
    pub const AV_CODEC_ID_XBIN: AVCodecID = 98308;
    #[c2rust::src_loc = "596:5"]
    pub const AV_CODEC_ID_BINTEXT: AVCodecID = 98307;
    #[c2rust::src_loc = "595:5"]
    pub const AV_CODEC_ID_EPG: AVCodecID = 98306;
    #[c2rust::src_loc = "594:5"]
    pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
    #[c2rust::src_loc = "592:5"]
    pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
    #[c2rust::src_loc = "591:5"]
    pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
    #[c2rust::src_loc = "588:5"]
    pub const AV_CODEC_ID_IVTV_VBI: AVCodecID = 94234;
    #[c2rust::src_loc = "587:5"]
    pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 94233;
    #[c2rust::src_loc = "586:5"]
    pub const AV_CODEC_ID_TTML: AVCodecID = 94232;
    #[c2rust::src_loc = "585:5"]
    pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 94231;
    #[c2rust::src_loc = "584:5"]
    pub const AV_CODEC_ID_ASS: AVCodecID = 94230;
    #[c2rust::src_loc = "583:5"]
    pub const AV_CODEC_ID_PJS: AVCodecID = 94229;
    #[c2rust::src_loc = "582:5"]
    pub const AV_CODEC_ID_VPLAYER: AVCodecID = 94228;
    #[c2rust::src_loc = "581:5"]
    pub const AV_CODEC_ID_MPL2: AVCodecID = 94227;
    #[c2rust::src_loc = "580:5"]
    pub const AV_CODEC_ID_WEBVTT: AVCodecID = 94226;
    #[c2rust::src_loc = "579:5"]
    pub const AV_CODEC_ID_SUBRIP: AVCodecID = 94225;
    #[c2rust::src_loc = "578:5"]
    pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 94224;
    #[c2rust::src_loc = "577:5"]
    pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 94223;
    #[c2rust::src_loc = "576:5"]
    pub const AV_CODEC_ID_STL: AVCodecID = 94222;
    #[c2rust::src_loc = "575:5"]
    pub const AV_CODEC_ID_REALTEXT: AVCodecID = 94221;
    #[c2rust::src_loc = "574:5"]
    pub const AV_CODEC_ID_SAMI: AVCodecID = 94220;
    #[c2rust::src_loc = "573:5"]
    pub const AV_CODEC_ID_JACOSUB: AVCodecID = 94219;
    #[c2rust::src_loc = "572:5"]
    pub const AV_CODEC_ID_EIA_608: AVCodecID = 94218;
    #[c2rust::src_loc = "571:5"]
    pub const AV_CODEC_ID_MICRODVD: AVCodecID = 94217;
    #[c2rust::src_loc = "570:5"]
    pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
    #[c2rust::src_loc = "569:5"]
    pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
    #[c2rust::src_loc = "568:5"]
    pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
    #[c2rust::src_loc = "567:5"]
    pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
    #[c2rust::src_loc = "566:5"]
    pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
    #[c2rust::src_loc = "565:5"]
    pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
    #[c2rust::src_loc = "564:5"]
    pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
    #[c2rust::src_loc = "563:5"]
    pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
    #[c2rust::src_loc = "562:5"]
    pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
    #[c2rust::src_loc = "561:5"]
    pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
    #[c2rust::src_loc = "558:5"]
    pub const AV_CODEC_ID_G728: AVCodecID = 86123;
    #[c2rust::src_loc = "557:5"]
    pub const AV_CODEC_ID_LC3: AVCodecID = 86122;
    #[c2rust::src_loc = "556:5"]
    pub const AV_CODEC_ID_QOA: AVCodecID = 86121;
    #[c2rust::src_loc = "555:5"]
    pub const AV_CODEC_ID_OSQ: AVCodecID = 86120;
    #[c2rust::src_loc = "554:5"]
    pub const AV_CODEC_ID_AC4: AVCodecID = 86119;
    #[c2rust::src_loc = "553:5"]
    pub const AV_CODEC_ID_RKA: AVCodecID = 86118;
    #[c2rust::src_loc = "552:5"]
    pub const AV_CODEC_ID_WAVARC: AVCodecID = 86117;
    #[c2rust::src_loc = "551:5"]
    pub const AV_CODEC_ID_FTR: AVCodecID = 86116;
    #[c2rust::src_loc = "550:5"]
    pub const AV_CODEC_ID_APAC: AVCodecID = 86115;
    #[c2rust::src_loc = "549:5"]
    pub const AV_CODEC_ID_MISC4: AVCodecID = 86114;
    #[c2rust::src_loc = "548:5"]
    pub const AV_CODEC_ID_BONK: AVCodecID = 86113;
    #[c2rust::src_loc = "547:5"]
    pub const AV_CODEC_ID_DFPWM: AVCodecID = 86112;
    #[c2rust::src_loc = "546:5"]
    pub const AV_CODEC_ID_MSNSIREN: AVCodecID = 86111;
    #[c2rust::src_loc = "545:5"]
    pub const AV_CODEC_ID_FASTAUDIO: AVCodecID = 86110;
    #[c2rust::src_loc = "544:5"]
    pub const AV_CODEC_ID_HCA: AVCodecID = 86109;
    #[c2rust::src_loc = "543:5"]
    pub const AV_CODEC_ID_SIREN: AVCodecID = 86108;
    #[c2rust::src_loc = "542:5"]
    pub const AV_CODEC_ID_MPEGH_3D_AUDIO: AVCodecID = 86107;
    #[c2rust::src_loc = "541:5"]
    pub const AV_CODEC_ID_ACELP_KELVIN: AVCodecID = 86106;
    #[c2rust::src_loc = "540:5"]
    pub const AV_CODEC_ID_HCOM: AVCodecID = 86105;
    #[c2rust::src_loc = "539:5"]
    pub const AV_CODEC_ID_ATRAC9: AVCodecID = 86104;
    #[c2rust::src_loc = "538:5"]
    pub const AV_CODEC_ID_SBC: AVCodecID = 86103;
    #[c2rust::src_loc = "537:5"]
    pub const AV_CODEC_ID_APTX_HD: AVCodecID = 86102;
    #[c2rust::src_loc = "536:5"]
    pub const AV_CODEC_ID_APTX: AVCodecID = 86101;
    #[c2rust::src_loc = "535:5"]
    pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 86100;
    #[c2rust::src_loc = "534:5"]
    pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 86099;
    #[c2rust::src_loc = "533:5"]
    pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 86098;
    #[c2rust::src_loc = "532:5"]
    pub const AV_CODEC_ID_DST: AVCodecID = 86097;
    #[c2rust::src_loc = "531:5"]
    pub const AV_CODEC_ID_XMA2: AVCodecID = 86096;
    #[c2rust::src_loc = "530:5"]
    pub const AV_CODEC_ID_XMA1: AVCodecID = 86095;
    #[c2rust::src_loc = "529:5"]
    pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 86094;
    #[c2rust::src_loc = "528:5"]
    pub const AV_CODEC_ID_4GV: AVCodecID = 86093;
    #[c2rust::src_loc = "527:5"]
    pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 86092;
    #[c2rust::src_loc = "526:5"]
    pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 86091;
    #[c2rust::src_loc = "525:5"]
    pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 86090;
    #[c2rust::src_loc = "524:5"]
    pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 86089;
    #[c2rust::src_loc = "523:5"]
    pub const AV_CODEC_ID_SMV: AVCodecID = 86088;
    #[c2rust::src_loc = "522:5"]
    pub const AV_CODEC_ID_EVRC: AVCodecID = 86087;
    #[c2rust::src_loc = "521:5"]
    pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 86086;
    #[c2rust::src_loc = "520:5"]
    pub const AV_CODEC_ID_SONIC: AVCodecID = 86085;
    #[c2rust::src_loc = "519:5"]
    pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 86084;
    #[c2rust::src_loc = "518:5"]
    pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
    #[c2rust::src_loc = "517:5"]
    pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
    #[c2rust::src_loc = "516:5"]
    pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
    #[c2rust::src_loc = "515:5"]
    pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
    #[c2rust::src_loc = "514:5"]
    pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
    #[c2rust::src_loc = "513:5"]
    pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
    #[c2rust::src_loc = "512:5"]
    pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
    #[c2rust::src_loc = "511:5"]
    pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
    #[c2rust::src_loc = "510:5"]
    pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
    #[c2rust::src_loc = "509:5"]
    pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
    #[c2rust::src_loc = "508:5"]
    pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
    #[c2rust::src_loc = "507:5"]
    pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
    #[c2rust::src_loc = "506:5"]
    pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
    #[c2rust::src_loc = "505:5"]
    pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
    #[c2rust::src_loc = "504:5"]
    pub const AV_CODEC_ID_G729: AVCodecID = 86069;
    #[c2rust::src_loc = "503:5"]
    pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
    #[c2rust::src_loc = "502:5"]
    pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
    #[c2rust::src_loc = "501:5"]
    pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
    #[c2rust::src_loc = "500:5"]
    pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
    #[c2rust::src_loc = "499:5"]
    pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
    #[c2rust::src_loc = "498:5"]
    pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
    #[c2rust::src_loc = "497:5"]
    pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
    #[c2rust::src_loc = "496:5"]
    pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
    #[c2rust::src_loc = "495:5"]
    pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
    #[c2rust::src_loc = "494:5"]
    pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
    #[c2rust::src_loc = "493:5"]
    pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
    #[c2rust::src_loc = "492:5"]
    pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
    #[c2rust::src_loc = "491:5"]
    pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
    #[c2rust::src_loc = "490:5"]
    pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
    #[c2rust::src_loc = "489:5"]
    pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
    #[c2rust::src_loc = "488:5"]
    pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
    #[c2rust::src_loc = "487:5"]
    pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
    #[c2rust::src_loc = "486:5"]
    pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
    #[c2rust::src_loc = "485:5"]
    pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
    #[c2rust::src_loc = "484:5"]
    pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
    #[c2rust::src_loc = "483:5"]
    pub const AV_CODEC_ID_APE: AVCodecID = 86048;
    #[c2rust::src_loc = "482:5"]
    pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
    #[c2rust::src_loc = "481:5"]
    pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
    #[c2rust::src_loc = "480:5"]
    pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
    #[c2rust::src_loc = "479:5"]
    pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
    #[c2rust::src_loc = "478:5"]
    pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
    #[c2rust::src_loc = "477:5"]
    pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
    #[c2rust::src_loc = "476:5"]
    pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
    #[c2rust::src_loc = "475:5"]
    pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
    #[c2rust::src_loc = "474:5"]
    pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
    #[c2rust::src_loc = "473:5"]
    pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
    #[c2rust::src_loc = "472:5"]
    pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
    #[c2rust::src_loc = "471:5"]
    pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
    #[c2rust::src_loc = "470:5"]
    pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
    #[c2rust::src_loc = "469:5"]
    pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
    #[c2rust::src_loc = "468:5"]
    pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
    #[c2rust::src_loc = "467:5"]
    pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
    #[c2rust::src_loc = "466:5"]
    pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
    #[c2rust::src_loc = "465:5"]
    pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
    #[c2rust::src_loc = "464:5"]
    pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
    #[c2rust::src_loc = "463:5"]
    pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
    #[c2rust::src_loc = "462:5"]
    pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
    #[c2rust::src_loc = "461:5"]
    pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
    #[c2rust::src_loc = "460:5"]
    pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
    #[c2rust::src_loc = "459:5"]
    pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
    #[c2rust::src_loc = "458:5"]
    pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
    #[c2rust::src_loc = "457:5"]
    pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
    #[c2rust::src_loc = "456:5"]
    pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
    #[c2rust::src_loc = "455:5"]
    pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
    #[c2rust::src_loc = "454:5"]
    pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
    #[c2rust::src_loc = "453:5"]
    pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
    #[c2rust::src_loc = "452:5"]
    pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
    #[c2rust::src_loc = "451:5"]
    pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
    #[c2rust::src_loc = "448:5"]
    pub const AV_CODEC_ID_CBD2_DPCM: AVCodecID = 81928;
    #[c2rust::src_loc = "447:5"]
    pub const AV_CODEC_ID_WADY_DPCM: AVCodecID = 81927;
    #[c2rust::src_loc = "446:5"]
    pub const AV_CODEC_ID_DERF_DPCM: AVCodecID = 81926;
    #[c2rust::src_loc = "445:5"]
    pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 81925;
    #[c2rust::src_loc = "444:5"]
    pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 81924;
    #[c2rust::src_loc = "443:5"]
    pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
    #[c2rust::src_loc = "442:5"]
    pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
    #[c2rust::src_loc = "441:5"]
    pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
    #[c2rust::src_loc = "440:5"]
    pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
    #[c2rust::src_loc = "437:5"]
    pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
    #[c2rust::src_loc = "436:5"]
    pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
    #[c2rust::src_loc = "433:5"]
    pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
    #[c2rust::src_loc = "432:5"]
    pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
    #[c2rust::src_loc = "429:5"]
    pub const AV_CODEC_ID_ADPCM_SANYO: AVCodecID = 69685;
    #[c2rust::src_loc = "428:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_XBOX: AVCodecID = 69684;
    #[c2rust::src_loc = "427:5"]
    pub const AV_CODEC_ID_ADPCM_XMD: AVCodecID = 69683;
    #[c2rust::src_loc = "426:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_ACORN: AVCodecID = 69682;
    #[c2rust::src_loc = "425:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: AVCodecID = 69681;
    #[c2rust::src_loc = "424:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: AVCodecID = 69680;
    #[c2rust::src_loc = "423:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_MTF: AVCodecID = 69679;
    #[c2rust::src_loc = "422:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_ALP: AVCodecID = 69678;
    #[c2rust::src_loc = "421:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_APM: AVCodecID = 69677;
    #[c2rust::src_loc = "420:5"]
    pub const AV_CODEC_ID_ADPCM_ZORK: AVCodecID = 69676;
    #[c2rust::src_loc = "419:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_SSI: AVCodecID = 69675;
    #[c2rust::src_loc = "418:5"]
    pub const AV_CODEC_ID_ADPCM_ARGO: AVCodecID = 69674;
    #[c2rust::src_loc = "417:5"]
    pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 69673;
    #[c2rust::src_loc = "416:5"]
    pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 69672;
    #[c2rust::src_loc = "415:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 69671;
    #[c2rust::src_loc = "414:5"]
    pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 69670;
    #[c2rust::src_loc = "413:5"]
    pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 69669;
    #[c2rust::src_loc = "412:5"]
    pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 69668;
    #[c2rust::src_loc = "411:5"]
    pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 69667;
    #[c2rust::src_loc = "410:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 69666;
    #[c2rust::src_loc = "409:5"]
    pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 69665;
    #[c2rust::src_loc = "408:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 69664;
    #[c2rust::src_loc = "407:5"]
    pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 69663;
    #[c2rust::src_loc = "406:5"]
    pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
    #[c2rust::src_loc = "405:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
    #[c2rust::src_loc = "404:5"]
    pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
    #[c2rust::src_loc = "403:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
    #[c2rust::src_loc = "402:5"]
    pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
    #[c2rust::src_loc = "401:5"]
    pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
    #[c2rust::src_loc = "400:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
    #[c2rust::src_loc = "399:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
    #[c2rust::src_loc = "398:5"]
    pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
    #[c2rust::src_loc = "397:5"]
    pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
    #[c2rust::src_loc = "396:5"]
    pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
    #[c2rust::src_loc = "395:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
    #[c2rust::src_loc = "394:5"]
    pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
    #[c2rust::src_loc = "393:5"]
    pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
    #[c2rust::src_loc = "392:5"]
    pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
    #[c2rust::src_loc = "391:5"]
    pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
    #[c2rust::src_loc = "390:5"]
    pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
    #[c2rust::src_loc = "389:5"]
    pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
    #[c2rust::src_loc = "388:5"]
    pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
    #[c2rust::src_loc = "387:5"]
    pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
    #[c2rust::src_loc = "386:5"]
    pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
    #[c2rust::src_loc = "385:5"]
    pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
    #[c2rust::src_loc = "384:5"]
    pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
    #[c2rust::src_loc = "383:5"]
    pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
    #[c2rust::src_loc = "382:5"]
    pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
    #[c2rust::src_loc = "381:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
    #[c2rust::src_loc = "380:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
    #[c2rust::src_loc = "379:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
    #[c2rust::src_loc = "378:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
    #[c2rust::src_loc = "377:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
    #[c2rust::src_loc = "376:5"]
    pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
    #[c2rust::src_loc = "373:5"]
    pub const AV_CODEC_ID_PCM_SGA: AVCodecID = 65572;
    #[c2rust::src_loc = "372:5"]
    pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 65571;
    #[c2rust::src_loc = "371:5"]
    pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 65570;
    #[c2rust::src_loc = "370:5"]
    pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 65569;
    #[c2rust::src_loc = "369:5"]
    pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 65568;
    #[c2rust::src_loc = "368:5"]
    pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 65567;
    #[c2rust::src_loc = "367:5"]
    pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
    #[c2rust::src_loc = "366:5"]
    pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
    #[c2rust::src_loc = "365:5"]
    pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
    #[c2rust::src_loc = "364:5"]
    pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
    #[c2rust::src_loc = "363:5"]
    pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
    #[c2rust::src_loc = "362:5"]
    pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
    #[c2rust::src_loc = "361:5"]
    pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
    #[c2rust::src_loc = "360:5"]
    pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
    #[c2rust::src_loc = "359:5"]
    pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
    #[c2rust::src_loc = "358:5"]
    pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
    #[c2rust::src_loc = "357:5"]
    pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
    #[c2rust::src_loc = "356:5"]
    pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
    #[c2rust::src_loc = "355:5"]
    pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
    #[c2rust::src_loc = "354:5"]
    pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
    #[c2rust::src_loc = "353:5"]
    pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
    #[c2rust::src_loc = "352:5"]
    pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
    #[c2rust::src_loc = "351:5"]
    pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
    #[c2rust::src_loc = "350:5"]
    pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
    #[c2rust::src_loc = "349:5"]
    pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
    #[c2rust::src_loc = "348:5"]
    pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
    #[c2rust::src_loc = "347:5"]
    pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
    #[c2rust::src_loc = "346:5"]
    pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
    #[c2rust::src_loc = "345:5"]
    pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
    #[c2rust::src_loc = "344:5"]
    pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
    #[c2rust::src_loc = "343:5"]
    pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
    #[c2rust::src_loc = "342:5"]
    pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
    #[c2rust::src_loc = "341:5"]
    pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
    #[c2rust::src_loc = "340:5"]
    pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
    #[c2rust::src_loc = "339:5"]
    pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
    #[c2rust::src_loc = "338:5"]
    pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
    #[c2rust::src_loc = "337:5"]
    pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
    #[c2rust::src_loc = "336:5"]
    pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
    #[c2rust::src_loc = "333:5"]
    pub const AV_CODEC_ID_PRORES_RAW: AVCodecID = 274;
    #[c2rust::src_loc = "332:5"]
    pub const AV_CODEC_ID_APV: AVCodecID = 273;
    #[c2rust::src_loc = "331:5"]
    pub const AV_CODEC_ID_JPEGXL_ANIM: AVCodecID = 272;
    #[c2rust::src_loc = "330:5"]
    pub const AV_CODEC_ID_RV60: AVCodecID = 271;
    #[c2rust::src_loc = "329:5"]
    pub const AV_CODEC_ID_DNXUC: AVCodecID = 270;
    #[c2rust::src_loc = "328:5"]
    pub const AV_CODEC_ID_LEAD: AVCodecID = 269;
    #[c2rust::src_loc = "327:5"]
    pub const AV_CODEC_ID_VMIX: AVCodecID = 268;
    #[c2rust::src_loc = "326:5"]
    pub const AV_CODEC_ID_RTV1: AVCodecID = 267;
    #[c2rust::src_loc = "325:5"]
    pub const AV_CODEC_ID_EVC: AVCodecID = 266;
    #[c2rust::src_loc = "324:5"]
    pub const AV_CODEC_ID_PDV: AVCodecID = 265;
    #[c2rust::src_loc = "323:5"]
    pub const AV_CODEC_ID_VQC: AVCodecID = 264;
    #[c2rust::src_loc = "322:5"]
    pub const AV_CODEC_ID_MEDIA100: AVCodecID = 263;
    #[c2rust::src_loc = "321:5"]
    pub const AV_CODEC_ID_WBMP: AVCodecID = 262;
    #[c2rust::src_loc = "320:5"]
    pub const AV_CODEC_ID_RADIANCE_HDR: AVCodecID = 261;
    #[c2rust::src_loc = "319:5"]
    pub const AV_CODEC_ID_PHM: AVCodecID = 260;
    #[c2rust::src_loc = "318:5"]
    pub const AV_CODEC_ID_QOI: AVCodecID = 259;
    #[c2rust::src_loc = "317:5"]
    pub const AV_CODEC_ID_JPEGXL: AVCodecID = 258;
    #[c2rust::src_loc = "316:5"]
    pub const AV_CODEC_ID_VBN: AVCodecID = 257;
    #[c2rust::src_loc = "315:5"]
    pub const AV_CODEC_ID_GEM: AVCodecID = 256;
    #[c2rust::src_loc = "314:5"]
    pub const AV_CODEC_ID_SGA_VIDEO: AVCodecID = 255;
    #[c2rust::src_loc = "313:5"]
    pub const AV_CODEC_ID_SIMBIOSIS_IMX: AVCodecID = 254;
    #[c2rust::src_loc = "312:5"]
    pub const AV_CODEC_ID_CRI: AVCodecID = 253;
    #[c2rust::src_loc = "311:5"]
    pub const AV_CODEC_ID_ARGO: AVCodecID = 252;
    #[c2rust::src_loc = "310:5"]
    pub const AV_CODEC_ID_IPU: AVCodecID = 251;
    #[c2rust::src_loc = "309:5"]
    pub const AV_CODEC_ID_PHOTOCD: AVCodecID = 250;
    #[c2rust::src_loc = "308:5"]
    pub const AV_CODEC_ID_MOBICLIP: AVCodecID = 249;
    #[c2rust::src_loc = "307:5"]
    pub const AV_CODEC_ID_PFM: AVCodecID = 248;
    #[c2rust::src_loc = "306:5"]
    pub const AV_CODEC_ID_NOTCHLC: AVCodecID = 247;
    #[c2rust::src_loc = "305:5"]
    pub const AV_CODEC_ID_MV30: AVCodecID = 246;
    #[c2rust::src_loc = "304:5"]
    pub const AV_CODEC_ID_CDTOONS: AVCodecID = 245;
    #[c2rust::src_loc = "303:5"]
    pub const AV_CODEC_ID_MVHA: AVCodecID = 244;
    #[c2rust::src_loc = "302:5"]
    pub const AV_CODEC_ID_MVDV: AVCodecID = 243;
    #[c2rust::src_loc = "301:5"]
    pub const AV_CODEC_ID_IMM5: AVCodecID = 242;
    #[c2rust::src_loc = "300:5"]
    pub const AV_CODEC_ID_VP4: AVCodecID = 241;
    #[c2rust::src_loc = "299:5"]
    pub const AV_CODEC_ID_LSCR: AVCodecID = 240;
    #[c2rust::src_loc = "298:5"]
    pub const AV_CODEC_ID_AGM: AVCodecID = 239;
    #[c2rust::src_loc = "297:5"]
    pub const AV_CODEC_ID_ARBC: AVCodecID = 238;
    #[c2rust::src_loc = "296:5"]
    pub const AV_CODEC_ID_HYMT: AVCodecID = 237;
    #[c2rust::src_loc = "295:5"]
    pub const AV_CODEC_ID_RASC: AVCodecID = 236;
    #[c2rust::src_loc = "294:5"]
    pub const AV_CODEC_ID_WCMV: AVCodecID = 235;
    #[c2rust::src_loc = "293:5"]
    pub const AV_CODEC_ID_MWSC: AVCodecID = 234;
    #[c2rust::src_loc = "292:5"]
    pub const AV_CODEC_ID_PROSUMER: AVCodecID = 233;
    #[c2rust::src_loc = "291:5"]
    pub const AV_CODEC_ID_IMM4: AVCodecID = 232;
    #[c2rust::src_loc = "290:5"]
    pub const AV_CODEC_ID_FITS: AVCodecID = 231;
    #[c2rust::src_loc = "289:5"]
    pub const AV_CODEC_ID_GDV: AVCodecID = 230;
    #[c2rust::src_loc = "288:5"]
    pub const AV_CODEC_ID_SVG: AVCodecID = 229;
    #[c2rust::src_loc = "287:5"]
    pub const AV_CODEC_ID_SRGC: AVCodecID = 228;
    #[c2rust::src_loc = "286:5"]
    pub const AV_CODEC_ID_MSCC: AVCodecID = 227;
    #[c2rust::src_loc = "285:5"]
    pub const AV_CODEC_ID_BITPACKED: AVCodecID = 226;
    #[c2rust::src_loc = "284:5"]
    pub const AV_CODEC_ID_AV1: AVCodecID = 225;
    #[c2rust::src_loc = "283:5"]
    pub const AV_CODEC_ID_XPM: AVCodecID = 224;
    #[c2rust::src_loc = "282:5"]
    pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 223;
    #[c2rust::src_loc = "281:5"]
    pub const AV_CODEC_ID_SCPR: AVCodecID = 222;
    #[c2rust::src_loc = "280:5"]
    pub const AV_CODEC_ID_FMVC: AVCodecID = 221;
    #[c2rust::src_loc = "279:5"]
    pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 220;
    #[c2rust::src_loc = "278:5"]
    pub const AV_CODEC_ID_PIXLET: AVCodecID = 219;
    #[c2rust::src_loc = "277:5"]
    pub const AV_CODEC_ID_PSD: AVCodecID = 218;
    #[c2rust::src_loc = "276:5"]
    pub const AV_CODEC_ID_YLC: AVCodecID = 217;
    #[c2rust::src_loc = "275:5"]
    pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 216;
    #[c2rust::src_loc = "274:5"]
    pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 215;
    #[c2rust::src_loc = "273:5"]
    pub const AV_CODEC_ID_M101: AVCodecID = 214;
    #[c2rust::src_loc = "272:5"]
    pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 213;
    #[c2rust::src_loc = "271:5"]
    pub const AV_CODEC_ID_CFHD: AVCodecID = 212;
    #[c2rust::src_loc = "270:5"]
    pub const AV_CODEC_ID_DAALA: AVCodecID = 211;
    #[c2rust::src_loc = "269:5"]
    pub const AV_CODEC_ID_APNG: AVCodecID = 210;
    #[c2rust::src_loc = "268:5"]
    pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 209;
    #[c2rust::src_loc = "267:5"]
    pub const AV_CODEC_ID_SNOW: AVCodecID = 208;
    #[c2rust::src_loc = "266:5"]
    pub const AV_CODEC_ID_XFACE: AVCodecID = 207;
    #[c2rust::src_loc = "265:5"]
    pub const AV_CODEC_ID_CPIA: AVCodecID = 206;
    #[c2rust::src_loc = "264:5"]
    pub const AV_CODEC_ID_AVRN: AVCodecID = 205;
    #[c2rust::src_loc = "263:5"]
    pub const AV_CODEC_ID_YUV4: AVCodecID = 204;
    #[c2rust::src_loc = "261:5"]
    pub const AV_CODEC_ID_V408: AVCodecID = 203;
    #[c2rust::src_loc = "260:5"]
    pub const AV_CODEC_ID_V308: AVCodecID = 202;
    #[c2rust::src_loc = "258:5"]
    pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 201;
    #[c2rust::src_loc = "257:5"]
    pub const AV_CODEC_ID_AVUI: AVCodecID = 200;
    #[c2rust::src_loc = "256:5"]
    pub const AV_CODEC_ID_012V: AVCodecID = 199;
    #[c2rust::src_loc = "255:5"]
    pub const AV_CODEC_ID_AVRP: AVCodecID = 198;
    #[c2rust::src_loc = "254:5"]
    pub const AV_CODEC_ID_Y41P: AVCodecID = 197;
    #[c2rust::src_loc = "252:5"]
    pub const AV_CODEC_ID_VVC: AVCodecID = 196;
    #[c2rust::src_loc = "251:5"]
    pub const AV_CODEC_ID_MSP2: AVCodecID = 195;
    #[c2rust::src_loc = "250:5"]
    pub const AV_CODEC_ID_AVS3: AVCodecID = 194;
    #[c2rust::src_loc = "249:5"]
    pub const AV_CODEC_ID_PGX: AVCodecID = 193;
    #[c2rust::src_loc = "248:5"]
    pub const AV_CODEC_ID_AVS2: AVCodecID = 192;
    #[c2rust::src_loc = "247:5"]
    pub const AV_CODEC_ID_RSCC: AVCodecID = 191;
    #[c2rust::src_loc = "246:5"]
    pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
    #[c2rust::src_loc = "245:5"]
    pub const AV_CODEC_ID_DXV: AVCodecID = 189;
    #[c2rust::src_loc = "244:5"]
    pub const AV_CODEC_ID_DDS: AVCodecID = 188;
    #[c2rust::src_loc = "243:5"]
    pub const AV_CODEC_ID_HAP: AVCodecID = 187;
    #[c2rust::src_loc = "242:5"]
    pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
    #[c2rust::src_loc = "241:5"]
    pub const AV_CODEC_ID_TDSC: AVCodecID = 185;
    #[c2rust::src_loc = "240:5"]
    pub const AV_CODEC_ID_HQX: AVCodecID = 184;
    #[c2rust::src_loc = "239:5"]
    pub const AV_CODEC_ID_MVC2: AVCodecID = 183;
    #[c2rust::src_loc = "238:5"]
    pub const AV_CODEC_ID_MVC1: AVCodecID = 182;
    #[c2rust::src_loc = "237:5"]
    pub const AV_CODEC_ID_SGIRLE: AVCodecID = 181;
    #[c2rust::src_loc = "236:5"]
    pub const AV_CODEC_ID_SANM: AVCodecID = 180;
    #[c2rust::src_loc = "235:5"]
    pub const AV_CODEC_ID_VP7: AVCodecID = 179;
    #[c2rust::src_loc = "234:5"]
    pub const AV_CODEC_ID_EXR: AVCodecID = 178;
    #[c2rust::src_loc = "233:5"]
    pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
    #[c2rust::src_loc = "232:5"]
    pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
    #[c2rust::src_loc = "231:5"]
    pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
    #[c2rust::src_loc = "230:5"]
    pub const AV_CODEC_ID_FIC: AVCodecID = 174;
    #[c2rust::src_loc = "228:5"]
    pub const AV_CODEC_ID_HEVC: AVCodecID = 173;
    #[c2rust::src_loc = "227:5"]
    pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
    #[c2rust::src_loc = "226:5"]
    pub const AV_CODEC_ID_WEBP: AVCodecID = 171;
    #[c2rust::src_loc = "225:5"]
    pub const AV_CODEC_ID_G2M: AVCodecID = 170;
    #[c2rust::src_loc = "224:5"]
    pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
    #[c2rust::src_loc = "223:5"]
    pub const AV_CODEC_ID_AIC: AVCodecID = 168;
    #[c2rust::src_loc = "222:5"]
    pub const AV_CODEC_ID_VP9: AVCodecID = 167;
    #[c2rust::src_loc = "221:5"]
    pub const AV_CODEC_ID_MSS2: AVCodecID = 166;
    #[c2rust::src_loc = "220:5"]
    pub const AV_CODEC_ID_CLLC: AVCodecID = 165;
    #[c2rust::src_loc = "219:5"]
    pub const AV_CODEC_ID_MTS2: AVCodecID = 164;
    #[c2rust::src_loc = "218:5"]
    pub const AV_CODEC_ID_TSCC2: AVCodecID = 163;
    #[c2rust::src_loc = "217:5"]
    pub const AV_CODEC_ID_MSA1: AVCodecID = 162;
    #[c2rust::src_loc = "216:5"]
    pub const AV_CODEC_ID_MSS1: AVCodecID = 161;
    #[c2rust::src_loc = "215:5"]
    pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
    #[c2rust::src_loc = "214:5"]
    pub const AV_CODEC_ID_XBM: AVCodecID = 159;
    #[c2rust::src_loc = "213:5"]
    pub const AV_CODEC_ID_CDXL: AVCodecID = 158;
    #[c2rust::src_loc = "212:5"]
    pub const AV_CODEC_ID_XWD: AVCodecID = 157;
    #[c2rust::src_loc = "210:5"]
    pub const AV_CODEC_ID_V410: AVCodecID = 156;
    #[c2rust::src_loc = "208:5"]
    pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
    #[c2rust::src_loc = "207:5"]
    pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
    #[c2rust::src_loc = "206:5"]
    pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
    #[c2rust::src_loc = "205:5"]
    pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
    #[c2rust::src_loc = "204:5"]
    pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
    #[c2rust::src_loc = "203:5"]
    pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
    #[c2rust::src_loc = "202:5"]
    pub const AV_CODEC_ID_DFA: AVCodecID = 149;
    #[c2rust::src_loc = "201:5"]
    pub const AV_CODEC_ID_JV: AVCodecID = 148;
    #[c2rust::src_loc = "200:5"]
    pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
    #[c2rust::src_loc = "199:5"]
    pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
    #[c2rust::src_loc = "198:5"]
    pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
    #[c2rust::src_loc = "197:5"]
    pub const AV_CODEC_ID_R10K: AVCodecID = 144;
    #[c2rust::src_loc = "196:5"]
    pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
    #[c2rust::src_loc = "195:5"]
    pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
    #[c2rust::src_loc = "194:5"]
    pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
    #[c2rust::src_loc = "193:5"]
    pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
    #[c2rust::src_loc = "192:5"]
    pub const AV_CODEC_ID_VP8: AVCodecID = 139;
    #[c2rust::src_loc = "191:5"]
    pub const AV_CODEC_ID_YOP: AVCodecID = 138;
    #[c2rust::src_loc = "190:5"]
    pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
    #[c2rust::src_loc = "188:5"]
    pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
    #[c2rust::src_loc = "187:5"]
    pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
    #[c2rust::src_loc = "186:5"]
    pub const AV_CODEC_ID_ANM: AVCodecID = 134;
    #[c2rust::src_loc = "185:5"]
    pub const AV_CODEC_ID_R210: AVCodecID = 133;
    #[c2rust::src_loc = "184:5"]
    pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
    #[c2rust::src_loc = "183:5"]
    pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
    #[c2rust::src_loc = "182:5"]
    pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
    #[c2rust::src_loc = "181:5"]
    pub const AV_CODEC_ID_MAD: AVCodecID = 129;
    #[c2rust::src_loc = "180:5"]
    pub const AV_CODEC_ID_DPX: AVCodecID = 128;
    #[c2rust::src_loc = "179:5"]
    pub const AV_CODEC_ID_V210: AVCodecID = 127;
    #[c2rust::src_loc = "178:5"]
    pub const AV_CODEC_ID_TMV: AVCodecID = 126;
    #[c2rust::src_loc = "177:5"]
    pub const AV_CODEC_ID_V210X: AVCodecID = 125;
    #[c2rust::src_loc = "176:5"]
    pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
    #[c2rust::src_loc = "175:5"]
    pub const AV_CODEC_ID_AURA: AVCodecID = 123;
    #[c2rust::src_loc = "174:5"]
    pub const AV_CODEC_ID_TQI: AVCodecID = 122;
    #[c2rust::src_loc = "173:5"]
    pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
    #[c2rust::src_loc = "172:5"]
    pub const AV_CODEC_ID_TGV: AVCodecID = 120;
    #[c2rust::src_loc = "171:5"]
    pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
    #[c2rust::src_loc = "170:5"]
    pub const AV_CODEC_ID_CMV: AVCodecID = 118;
    #[c2rust::src_loc = "169:5"]
    pub const AV_CODEC_ID_BFI: AVCodecID = 117;
    #[c2rust::src_loc = "168:5"]
    pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
    #[c2rust::src_loc = "167:5"]
    pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
    #[c2rust::src_loc = "166:5"]
    pub const AV_CODEC_ID_RL2: AVCodecID = 114;
    #[c2rust::src_loc = "165:5"]
    pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
    #[c2rust::src_loc = "164:5"]
    pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
    #[c2rust::src_loc = "163:5"]
    pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
    #[c2rust::src_loc = "162:5"]
    pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
    #[c2rust::src_loc = "161:5"]
    pub const AV_CODEC_ID_PCX: AVCodecID = 109;
    #[c2rust::src_loc = "160:5"]
    pub const AV_CODEC_ID_VB: AVCodecID = 108;
    #[c2rust::src_loc = "159:5"]
    pub const AV_CODEC_ID_AMV: AVCodecID = 107;
    #[c2rust::src_loc = "158:5"]
    pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
    #[c2rust::src_loc = "157:5"]
    pub const AV_CODEC_ID_TXD: AVCodecID = 105;
    #[c2rust::src_loc = "156:5"]
    pub const AV_CODEC_ID_PTX: AVCodecID = 104;
    #[c2rust::src_loc = "155:5"]
    pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
    #[c2rust::src_loc = "154:5"]
    pub const AV_CODEC_ID_C93: AVCodecID = 102;
    #[c2rust::src_loc = "153:5"]
    pub const AV_CODEC_ID_SGI: AVCodecID = 101;
    #[c2rust::src_loc = "152:5"]
    pub const AV_CODEC_ID_THP: AVCodecID = 100;
    #[c2rust::src_loc = "151:5"]
    pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
    #[c2rust::src_loc = "150:5"]
    pub const AV_CODEC_ID_DXA: AVCodecID = 98;
    #[c2rust::src_loc = "149:5"]
    pub const AV_CODEC_ID_GIF: AVCodecID = 97;
    #[c2rust::src_loc = "148:5"]
    pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
    #[c2rust::src_loc = "147:5"]
    pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
    #[c2rust::src_loc = "146:5"]
    pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
    #[c2rust::src_loc = "145:5"]
    pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
    #[c2rust::src_loc = "144:5"]
    pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
    #[c2rust::src_loc = "143:5"]
    pub const AV_CODEC_ID_VP6: AVCodecID = 91;
    #[c2rust::src_loc = "142:5"]
    pub const AV_CODEC_ID_VP5: AVCodecID = 90;
    #[c2rust::src_loc = "141:5"]
    pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
    #[c2rust::src_loc = "140:5"]
    pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
    #[c2rust::src_loc = "139:5"]
    pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
    #[c2rust::src_loc = "138:5"]
    pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
    #[c2rust::src_loc = "137:5"]
    pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
    #[c2rust::src_loc = "136:5"]
    pub const AV_CODEC_ID_NUV: AVCodecID = 84;
    #[c2rust::src_loc = "135:5"]
    pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
    #[c2rust::src_loc = "134:5"]
    pub const AV_CODEC_ID_AVS: AVCodecID = 82;
    #[c2rust::src_loc = "133:5"]
    pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
    #[c2rust::src_loc = "132:5"]
    pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
    #[c2rust::src_loc = "131:5"]
    pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
    #[c2rust::src_loc = "130:5"]
    pub const AV_CODEC_ID_BMP: AVCodecID = 78;
    #[c2rust::src_loc = "129:5"]
    pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
    #[c2rust::src_loc = "128:5"]
    pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
    #[c2rust::src_loc = "127:5"]
    pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
    #[c2rust::src_loc = "126:5"]
    pub const AV_CODEC_ID_AASC: AVCodecID = 74;
    #[c2rust::src_loc = "125:5"]
    pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
    #[c2rust::src_loc = "124:5"]
    pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
    #[c2rust::src_loc = "123:5"]
    pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
    #[c2rust::src_loc = "122:5"]
    pub const AV_CODEC_ID_VC1: AVCodecID = 70;
    #[c2rust::src_loc = "121:5"]
    pub const AV_CODEC_ID_RV40: AVCodecID = 69;
    #[c2rust::src_loc = "120:5"]
    pub const AV_CODEC_ID_RV30: AVCodecID = 68;
    #[c2rust::src_loc = "119:5"]
    pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
    #[c2rust::src_loc = "118:5"]
    pub const AV_CODEC_ID_PAM: AVCodecID = 66;
    #[c2rust::src_loc = "117:5"]
    pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
    #[c2rust::src_loc = "116:5"]
    pub const AV_CODEC_ID_PGM: AVCodecID = 64;
    #[c2rust::src_loc = "115:5"]
    pub const AV_CODEC_ID_PBM: AVCodecID = 63;
    #[c2rust::src_loc = "114:5"]
    pub const AV_CODEC_ID_PPM: AVCodecID = 62;
    #[c2rust::src_loc = "113:5"]
    pub const AV_CODEC_ID_PNG: AVCodecID = 61;
    #[c2rust::src_loc = "112:5"]
    pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
    #[c2rust::src_loc = "111:5"]
    pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
    #[c2rust::src_loc = "110:5"]
    pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
    #[c2rust::src_loc = "109:5"]
    pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
    #[c2rust::src_loc = "108:5"]
    pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
    #[c2rust::src_loc = "107:5"]
    pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
    #[c2rust::src_loc = "106:5"]
    pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
    #[c2rust::src_loc = "105:5"]
    pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
    #[c2rust::src_loc = "104:5"]
    pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
    #[c2rust::src_loc = "103:5"]
    pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
    #[c2rust::src_loc = "102:5"]
    pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
    #[c2rust::src_loc = "101:5"]
    pub const AV_CODEC_ID_SMC: AVCodecID = 49;
    #[c2rust::src_loc = "100:5"]
    pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
    #[c2rust::src_loc = "99:5"]
    pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
    #[c2rust::src_loc = "98:5"]
    pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
    #[c2rust::src_loc = "97:5"]
    pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
    #[c2rust::src_loc = "96:5"]
    pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
    #[c2rust::src_loc = "95:5"]
    pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
    #[c2rust::src_loc = "94:5"]
    pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
    #[c2rust::src_loc = "93:5"]
    pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
    #[c2rust::src_loc = "92:5"]
    pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
    #[c2rust::src_loc = "91:5"]
    pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
    #[c2rust::src_loc = "90:5"]
    pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
    #[c2rust::src_loc = "89:5"]
    pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
    #[c2rust::src_loc = "88:5"]
    pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
    #[c2rust::src_loc = "87:5"]
    pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
    #[c2rust::src_loc = "86:5"]
    pub const AV_CODEC_ID_4XM: AVCodecID = 34;
    #[c2rust::src_loc = "85:5"]
    pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
    #[c2rust::src_loc = "84:5"]
    pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
    #[c2rust::src_loc = "83:5"]
    pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
    #[c2rust::src_loc = "82:5"]
    pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
    #[c2rust::src_loc = "81:5"]
    pub const AV_CODEC_ID_VP3: AVCodecID = 29;
    #[c2rust::src_loc = "80:5"]
    pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
    #[c2rust::src_loc = "79:5"]
    pub const AV_CODEC_ID_H264: AVCodecID = 27;
    #[c2rust::src_loc = "78:5"]
    pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
    #[c2rust::src_loc = "77:5"]
    pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
    #[c2rust::src_loc = "76:5"]
    pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
    #[c2rust::src_loc = "75:5"]
    pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
    #[c2rust::src_loc = "74:5"]
    pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
    #[c2rust::src_loc = "73:5"]
    pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
    #[c2rust::src_loc = "72:5"]
    pub const AV_CODEC_ID_H263I: AVCodecID = 20;
    #[c2rust::src_loc = "71:5"]
    pub const AV_CODEC_ID_H263P: AVCodecID = 19;
    #[c2rust::src_loc = "70:5"]
    pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
    #[c2rust::src_loc = "69:5"]
    pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
    #[c2rust::src_loc = "68:5"]
    pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
    #[c2rust::src_loc = "67:5"]
    pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
    #[c2rust::src_loc = "66:5"]
    pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
    #[c2rust::src_loc = "65:5"]
    pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
    #[c2rust::src_loc = "64:5"]
    pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
    #[c2rust::src_loc = "63:5"]
    pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
    #[c2rust::src_loc = "62:5"]
    pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
    #[c2rust::src_loc = "61:5"]
    pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
    #[c2rust::src_loc = "60:5"]
    pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
    #[c2rust::src_loc = "59:5"]
    pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
    #[c2rust::src_loc = "58:5"]
    pub const AV_CODEC_ID_RV20: AVCodecID = 6;
    #[c2rust::src_loc = "57:5"]
    pub const AV_CODEC_ID_RV10: AVCodecID = 5;
    #[c2rust::src_loc = "56:5"]
    pub const AV_CODEC_ID_H263: AVCodecID = 4;
    #[c2rust::src_loc = "55:5"]
    pub const AV_CODEC_ID_H261: AVCodecID = 3;
    #[c2rust::src_loc = "54:5"]
    pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
    #[c2rust::src_loc = "53:5"]
    pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
    #[c2rust::src_loc = "50:5"]
    pub const AV_CODEC_ID_NONE: AVCodecID = 0;
}
#[c2rust::header_src = "/usr/include/libavcodec/defs.h:27"]
pub mod defs_h {
    #[c2rust::src_loc = "223:1"]
    pub type AVDiscard = ::core::ffi::c_int;
    #[c2rust::src_loc = "232:5"]
    pub const AVDISCARD_ALL: AVDiscard = 48;
    #[c2rust::src_loc = "231:5"]
    pub const AVDISCARD_NONKEY: AVDiscard = 32;
    #[c2rust::src_loc = "230:5"]
    pub const AVDISCARD_NONINTRA: AVDiscard = 24;
    #[c2rust::src_loc = "229:5"]
    pub const AVDISCARD_BIDIR: AVDiscard = 16;
    #[c2rust::src_loc = "228:5"]
    pub const AVDISCARD_NONREF: AVDiscard = 8;
    #[c2rust::src_loc = "227:5"]
    pub const AVDISCARD_DEFAULT: AVDiscard = 0;
    #[c2rust::src_loc = "226:5"]
    pub const AVDISCARD_NONE: AVDiscard = -16;
    #[c2rust::src_loc = "235:1"]
    pub type AVAudioServiceType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "245:5"]
    pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
    #[c2rust::src_loc = "244:5"]
    pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
    #[c2rust::src_loc = "243:5"]
    pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
    #[c2rust::src_loc = "242:5"]
    pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
    #[c2rust::src_loc = "241:5"]
    pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
    #[c2rust::src_loc = "240:5"]
    pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
    #[c2rust::src_loc = "239:5"]
    pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
    #[c2rust::src_loc = "238:5"]
    pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
    #[c2rust::src_loc = "237:5"]
    pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
    #[c2rust::src_loc = "236:5"]
    pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
    #[c2rust::src_loc = "211:1"]
    pub type AVFieldOrder = ::core::ffi::c_uint;
    #[c2rust::src_loc = "217:5"]
    pub const AV_FIELD_BT: AVFieldOrder = 5;
    #[c2rust::src_loc = "216:5"]
    pub const AV_FIELD_TB: AVFieldOrder = 4;
    #[c2rust::src_loc = "215:5"]
    pub const AV_FIELD_BB: AVFieldOrder = 3;
    #[c2rust::src_loc = "214:5"]
    pub const AV_FIELD_TT: AVFieldOrder = 2;
    #[c2rust::src_loc = "213:5"]
    pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
    #[c2rust::src_loc = "212:5"]
    pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
}
#[c2rust::header_src = "/usr/include/libavutil/samplefmt.h:27"]
pub mod samplefmt_h {
    #[c2rust::src_loc = "55:1"]
    pub type AVSampleFormat = ::core::ffi::c_int;
    #[c2rust::src_loc = "71:5"]
    pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
    #[c2rust::src_loc = "69:5"]
    pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
    #[c2rust::src_loc = "68:5"]
    pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
    #[c2rust::src_loc = "67:5"]
    pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
    #[c2rust::src_loc = "66:5"]
    pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
    #[c2rust::src_loc = "65:5"]
    pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
    #[c2rust::src_loc = "64:5"]
    pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
    #[c2rust::src_loc = "63:5"]
    pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
    #[c2rust::src_loc = "61:5"]
    pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
    #[c2rust::src_loc = "60:5"]
    pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
    #[c2rust::src_loc = "59:5"]
    pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
    #[c2rust::src_loc = "58:5"]
    pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
    #[c2rust::src_loc = "57:5"]
    pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
    #[c2rust::src_loc = "56:5"]
    pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
}
#[c2rust::header_src = "/usr/include/libavutil/log.h:27"]
pub mod log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct AVClass {
        pub class_name: *const ::core::ffi::c_char,
        pub item_name:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char>,
        pub option: *const AVOption,
        pub version: ::core::ffi::c_int,
        pub log_level_offset_offset: ::core::ffi::c_int,
        pub parent_log_context_offset: ::core::ffi::c_int,
        pub category: AVClassCategory,
        pub get_category: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory>,
        pub query_ranges: Option<
            unsafe extern "C" fn(
                *mut *mut AVOptionRanges,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub child_next: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub child_class_iterate:
            Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass>,
        pub state_flags_offset: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "28:9"]
    pub type AVClassCategory = ::core::ffi::c_uint;
    #[c2rust::src_loc = "47:5"]
    pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
    #[c2rust::src_loc = "46:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
    #[c2rust::src_loc = "45:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
    #[c2rust::src_loc = "44:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
    #[c2rust::src_loc = "43:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
    #[c2rust::src_loc = "42:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
    #[c2rust::src_loc = "41:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
    #[c2rust::src_loc = "40:5"]
    pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
    #[c2rust::src_loc = "39:5"]
    pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
    #[c2rust::src_loc = "38:5"]
    pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
    #[c2rust::src_loc = "37:5"]
    pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
    #[c2rust::src_loc = "36:5"]
    pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
    #[c2rust::src_loc = "35:5"]
    pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
    #[c2rust::src_loc = "34:5"]
    pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
    #[c2rust::src_loc = "33:5"]
    pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
    #[c2rust::src_loc = "32:5"]
    pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
    #[c2rust::src_loc = "31:5"]
    pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
    #[c2rust::src_loc = "30:5"]
    pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
    #[c2rust::src_loc = "29:5"]
    pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
    extern "C" {
        #[c2rust::src_loc = "69:8"]
        pub type AVOptionRanges;
        #[c2rust::src_loc = "96:18"]
        pub type AVOption;
    }
}
#[c2rust::header_src = "/usr/include/libavformat/avformat.h:30"]
pub mod avformat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1270:16"]
    pub struct AVFormatContext {
        pub av_class: *const AVClass,
        pub iformat: *const AVInputFormat,
        pub oformat: *const AVOutputFormat,
        pub priv_data: *mut ::core::ffi::c_void,
        pub pb: *mut AVIOContext,
        pub ctx_flags: ::core::ffi::c_int,
        pub nb_streams: ::core::ffi::c_uint,
        pub streams: *mut *mut AVStream,
        pub nb_stream_groups: ::core::ffi::c_uint,
        pub stream_groups: *mut *mut AVStreamGroup,
        pub nb_chapters: ::core::ffi::c_uint,
        pub chapters: *mut *mut AVChapter,
        pub url: *mut ::core::ffi::c_char,
        pub start_time: int64_t,
        pub duration: int64_t,
        pub bit_rate: int64_t,
        pub packet_size: ::core::ffi::c_uint,
        pub max_delay: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
        pub probesize: int64_t,
        pub max_analyze_duration: int64_t,
        pub key: *const uint8_t,
        pub keylen: ::core::ffi::c_int,
        pub nb_programs: ::core::ffi::c_uint,
        pub programs: *mut *mut AVProgram,
        pub video_codec_id: AVCodecID,
        pub audio_codec_id: AVCodecID,
        pub subtitle_codec_id: AVCodecID,
        pub data_codec_id: AVCodecID,
        pub metadata: *mut AVDictionary,
        pub start_time_realtime: int64_t,
        pub fps_probe_size: ::core::ffi::c_int,
        pub error_recognition: ::core::ffi::c_int,
        pub interrupt_callback: AVIOInterruptCB,
        pub debug: ::core::ffi::c_int,
        pub max_streams: ::core::ffi::c_int,
        pub max_index_size: ::core::ffi::c_uint,
        pub max_picture_buffer: ::core::ffi::c_uint,
        pub max_interleave_delta: int64_t,
        pub max_ts_probe: ::core::ffi::c_int,
        pub max_chunk_duration: ::core::ffi::c_int,
        pub max_chunk_size: ::core::ffi::c_int,
        pub max_probe_packets: ::core::ffi::c_int,
        pub strict_std_compliance: ::core::ffi::c_int,
        pub event_flags: ::core::ffi::c_int,
        pub avoid_negative_ts: ::core::ffi::c_int,
        pub audio_preload: ::core::ffi::c_int,
        pub use_wallclock_as_timestamps: ::core::ffi::c_int,
        pub skip_estimate_duration_from_pts: ::core::ffi::c_int,
        pub avio_flags: ::core::ffi::c_int,
        pub duration_estimation_method: AVDurationEstimationMethod,
        pub skip_initial_bytes: int64_t,
        pub correct_ts_overflow: ::core::ffi::c_uint,
        pub seek2any: ::core::ffi::c_int,
        pub flush_packets: ::core::ffi::c_int,
        pub probe_score: ::core::ffi::c_int,
        pub format_probesize: ::core::ffi::c_int,
        pub codec_whitelist: *mut ::core::ffi::c_char,
        pub format_whitelist: *mut ::core::ffi::c_char,
        pub protocol_whitelist: *mut ::core::ffi::c_char,
        pub protocol_blacklist: *mut ::core::ffi::c_char,
        pub io_repositioned: ::core::ffi::c_int,
        pub video_codec: *const AVCodec,
        pub audio_codec: *const AVCodec,
        pub subtitle_codec: *const AVCodec,
        pub data_codec: *const AVCodec,
        pub metadata_header_padding: ::core::ffi::c_int,
        pub opaque: *mut ::core::ffi::c_void,
        pub control_message_cb: av_format_control_message,
        pub output_ts_offset: int64_t,
        pub dump_separator: *mut uint8_t,
        pub io_open: Option<
            unsafe extern "C" fn(
                *mut AVFormatContext,
                *mut *mut AVIOContext,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut *mut AVDictionary,
            ) -> ::core::ffi::c_int,
        >,
        pub io_close2: Option<
            unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> ::core::ffi::c_int,
        >,
        pub duration_probesize: int64_t,
    }
    #[c2rust::src_loc = "1240:1"]
    pub type av_format_control_message = Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            size_t,
        ) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "1250:1"]
    pub type AVDurationEstimationMethod = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1253:5"]
    pub const AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
    #[c2rust::src_loc = "1252:5"]
    pub const AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
    #[c2rust::src_loc = "1251:5"]
    pub const AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1194:16"]
    pub struct AVProgram {
        pub id: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
        pub discard: AVDiscard,
        pub stream_index: *mut ::core::ffi::c_uint,
        pub nb_stream_indexes: ::core::ffi::c_uint,
        pub metadata: *mut AVDictionary,
        pub program_num: ::core::ffi::c_int,
        pub pmt_pid: ::core::ffi::c_int,
        pub pcr_pid: ::core::ffi::c_int,
        pub pmt_version: ::core::ffi::c_int,
        pub start_time: int64_t,
        pub end_time: int64_t,
        pub pts_wrap_reference: int64_t,
        pub pts_wrap_behavior: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1229:16"]
    pub struct AVChapter {
        pub id: int64_t,
        pub time_base: AVRational,
        pub start: int64_t,
        pub end: int64_t,
        pub metadata: *mut AVDictionary,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1100:16"]
    pub struct AVStreamGroup {
        pub av_class: *const AVClass,
        pub priv_data: *mut ::core::ffi::c_void,
        pub index: ::core::ffi::c_uint,
        pub id: int64_t,
        pub type_0: AVStreamGroupParamsType,
        pub params: C2RustUnnamed_0,
        pub metadata: *mut AVDictionary,
        pub nb_streams: ::core::ffi::c_uint,
        pub streams: *mut *mut AVStream,
        pub disposition: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct AVStream {
        pub av_class: *const AVClass,
        pub index: ::core::ffi::c_int,
        pub id: ::core::ffi::c_int,
        pub codecpar: *mut AVCodecParameters,
        pub priv_data: *mut ::core::ffi::c_void,
        pub time_base: AVRational,
        pub start_time: int64_t,
        pub duration: int64_t,
        pub nb_frames: int64_t,
        pub disposition: ::core::ffi::c_int,
        pub discard: AVDiscard,
        pub sample_aspect_ratio: AVRational,
        pub metadata: *mut AVDictionary,
        pub avg_frame_rate: AVRational,
        pub attached_pic: AVPacket,
        pub event_flags: ::core::ffi::c_int,
        pub r_frame_rate: AVRational,
        pub pts_wrap_bits: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1132:5"]
    pub union C2RustUnnamed_0 {
        pub iamf_audio_element: *mut AVIAMFAudioElement,
        pub iamf_mix_presentation: *mut AVIAMFMixPresentation,
        pub tile_grid: *mut AVStreamGroupTileGrid,
        pub lcevc: *mut AVStreamGroupLCEVC,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1072:16"]
    pub struct AVStreamGroupLCEVC {
        pub av_class: *const AVClass,
        pub lcevc_index: ::core::ffi::c_uint,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "953:16"]
    pub struct AVStreamGroupTileGrid {
        pub av_class: *const AVClass,
        pub nb_tiles: ::core::ffi::c_uint,
        pub coded_width: ::core::ffi::c_int,
        pub coded_height: ::core::ffi::c_int,
        pub offsets: *mut C2RustUnnamed_1,
        pub background: [uint8_t; 4],
        pub horizontal_offset: ::core::ffi::c_int,
        pub vertical_offset: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub coded_side_data: *mut AVPacketSideData,
        pub nb_coded_side_data: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "986:5"]
    pub struct C2RustUnnamed_1 {
        pub idx: ::core::ffi::c_uint,
        pub horizontal: ::core::ffi::c_int,
        pub vertical: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "1089:1"]
    pub type AVStreamGroupParamsType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1094:5"]
    pub const AV_STREAM_GROUP_PARAMS_LCEVC: AVStreamGroupParamsType = 4;
    #[c2rust::src_loc = "1093:5"]
    pub const AV_STREAM_GROUP_PARAMS_TILE_GRID: AVStreamGroupParamsType = 3;
    #[c2rust::src_loc = "1092:5"]
    pub const AV_STREAM_GROUP_PARAMS_IAMF_MIX_PRESENTATION: AVStreamGroupParamsType = 2;
    #[c2rust::src_loc = "1091:5"]
    pub const AV_STREAM_GROUP_PARAMS_IAMF_AUDIO_ELEMENT: AVStreamGroupParamsType = 1;
    #[c2rust::src_loc = "1090:5"]
    pub const AV_STREAM_GROUP_PARAMS_NONE: AVStreamGroupParamsType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "505:16"]
    pub struct AVOutputFormat {
        pub name: *const ::core::ffi::c_char,
        pub long_name: *const ::core::ffi::c_char,
        pub mime_type: *const ::core::ffi::c_char,
        pub extensions: *const ::core::ffi::c_char,
        pub audio_codec: AVCodecID,
        pub video_codec: AVCodecID,
        pub subtitle_codec: AVCodecID,
        pub flags: ::core::ffi::c_int,
        pub codec_tag: *const *const AVCodecTag,
        pub priv_class: *const AVClass,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "544:16"]
    pub struct AVInputFormat {
        pub name: *const ::core::ffi::c_char,
        pub long_name: *const ::core::ffi::c_char,
        pub flags: ::core::ffi::c_int,
        pub extensions: *const ::core::ffi::c_char,
        pub codec_tag: *const *const AVCodecTag,
        pub priv_class: *const AVClass,
        pub mime_type: *const ::core::ffi::c_char,
    }
    use super::__stddef_size_t_h::size_t;
    use super::avio_h::{AVIOContext, AVIOInterruptCB};
    use super::codec_h::AVCodec;
    use super::codec_id_h::AVCodecID;
    use super::codec_par_h::AVCodecParameters;
    use super::defs_h::AVDiscard;
    use super::dict_h::AVDictionary;
    use super::log_h::AVClass;
    use super::packet_h::{AVPacket, AVPacketSideData};
    use super::rational_h::AVRational;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "1098:8"]
        pub type AVIAMFMixPresentation;
        #[c2rust::src_loc = "1097:8"]
        pub type AVIAMFAudioElement;
        #[c2rust::src_loc = "446:8"]
        pub type AVCodecTag;
        #[c2rust::src_loc = "2105:1"]
        pub fn av_find_input_format(short_name: *const ::core::ffi::c_char)
            -> *const AVInputFormat;
        #[c2rust::src_loc = "2192:1"]
        pub fn avformat_open_input(
            ps: *mut *mut AVFormatContext,
            url: *const ::core::ffi::c_char,
            fmt: *const AVInputFormat,
            options: *mut *mut AVDictionary,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2216:1"]
        pub fn avformat_find_stream_info(
            ic: *mut AVFormatContext,
            options: *mut *mut AVDictionary,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2293:1"]
        pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2375:1"]
        pub fn avformat_close_input(s: *mut *mut AVFormatContext);
    }
}
#[c2rust::header_src = "/usr/include/libavformat/avio.h:27"]
pub mod avio_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:16"]
    pub struct AVIOContext {
        pub av_class: *const AVClass,
        pub buffer: *mut ::core::ffi::c_uchar,
        pub buffer_size: ::core::ffi::c_int,
        pub buf_ptr: *mut ::core::ffi::c_uchar,
        pub buf_end: *mut ::core::ffi::c_uchar,
        pub opaque: *mut ::core::ffi::c_void,
        pub read_packet: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub write_packet: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub seek: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, int64_t, ::core::ffi::c_int) -> int64_t,
        >,
        pub pos: int64_t,
        pub eof_reached: ::core::ffi::c_int,
        pub error: ::core::ffi::c_int,
        pub write_flag: ::core::ffi::c_int,
        pub max_packet_size: ::core::ffi::c_int,
        pub min_packet_size: ::core::ffi::c_int,
        pub checksum: ::core::ffi::c_ulong,
        pub checksum_ptr: *mut ::core::ffi::c_uchar,
        pub update_checksum: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_ulong,
                *const uint8_t,
                ::core::ffi::c_uint,
            ) -> ::core::ffi::c_ulong,
        >,
        pub read_pause: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub read_seek: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                int64_t,
                ::core::ffi::c_int,
            ) -> int64_t,
        >,
        pub seekable: ::core::ffi::c_int,
        pub direct: ::core::ffi::c_int,
        pub protocol_whitelist: *const ::core::ffi::c_char,
        pub protocol_blacklist: *const ::core::ffi::c_char,
        pub write_data_type: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const uint8_t,
                ::core::ffi::c_int,
                AVIODataMarkerType,
                int64_t,
            ) -> ::core::ffi::c_int,
        >,
        pub ignore_boundary_point: ::core::ffi::c_int,
        pub buf_ptr_max: *mut ::core::ffi::c_uchar,
        pub bytes_read: int64_t,
        pub bytes_written: int64_t,
    }
    #[c2rust::src_loc = "110:1"]
    pub type AVIODataMarkerType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "145:5"]
    pub const AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
    #[c2rust::src_loc = "139:5"]
    pub const AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
    #[c2rust::src_loc = "134:5"]
    pub const AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
    #[c2rust::src_loc = "127:5"]
    pub const AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
    #[c2rust::src_loc = "121:5"]
    pub const AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
    #[c2rust::src_loc = "114:5"]
    pub const AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:16"]
    pub struct AVIOInterruptCB {
        pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub opaque: *mut ::core::ffi::c_void,
    }
    use super::log_h::AVClass;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/usr/include/libavcodec/codec_par.h:27"]
pub mod codec_par_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct AVCodecParameters {
        pub codec_type: AVMediaType,
        pub codec_id: AVCodecID,
        pub codec_tag: uint32_t,
        pub extradata: *mut uint8_t,
        pub extradata_size: ::core::ffi::c_int,
        pub coded_side_data: *mut AVPacketSideData,
        pub nb_coded_side_data: ::core::ffi::c_int,
        pub format: ::core::ffi::c_int,
        pub bit_rate: int64_t,
        pub bits_per_coded_sample: ::core::ffi::c_int,
        pub bits_per_raw_sample: ::core::ffi::c_int,
        pub profile: ::core::ffi::c_int,
        pub level: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub sample_aspect_ratio: AVRational,
        pub framerate: AVRational,
        pub field_order: AVFieldOrder,
        pub color_range: AVColorRange,
        pub color_primaries: AVColorPrimaries,
        pub color_trc: AVColorTransferCharacteristic,
        pub color_space: AVColorSpace,
        pub chroma_location: AVChromaLocation,
        pub video_delay: ::core::ffi::c_int,
        pub ch_layout: AVChannelLayout,
        pub sample_rate: ::core::ffi::c_int,
        pub block_align: ::core::ffi::c_int,
        pub frame_size: ::core::ffi::c_int,
        pub initial_padding: ::core::ffi::c_int,
        pub trailing_padding: ::core::ffi::c_int,
        pub seek_preroll: ::core::ffi::c_int,
    }
    use super::avutil_h::AVMediaType;
    use super::channel_layout_h::AVChannelLayout;
    use super::codec_id_h::AVCodecID;
    use super::defs_h::AVFieldOrder;
    use super::packet_h::AVPacketSideData;
    use super::pixfmt_h::{
        AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace,
        AVColorTransferCharacteristic,
    };
    use super::rational_h::AVRational;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixdesc.h:30"]
pub mod pixdesc_h {
    use super::pixfmt_h::{AVPixelFormat, AV_PIX_FMT_YUV420P};
    extern "C" {
        #[c2rust::src_loc = "313:1"]
        pub fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "273:9"]
    pub const X264_CSP_VFLIP: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/strings.h:27"]
pub mod strings_h {
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/error.h:27"]
pub mod error_h {
    #[c2rust::src_loc = "57:9"]
    pub const AVERROR_EOF: ::core::ffi::c_int = -((('E' as i32
        | ('O' as i32) << 8 as ::core::ffi::c_int
        | ('F' as i32) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:30"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::avcodec_h::{
    avcodec_alloc_context3, avcodec_free_context, avcodec_open2, avcodec_parameters_to_context,
    avcodec_receive_frame, avcodec_send_packet, AVCodecContext, AVCodecInternal, AVHWAccel,
    RcOverride,
};
pub use self::avformat_h::{
    av_find_input_format, av_format_control_message, av_read_frame, avformat_close_input,
    avformat_find_stream_info, avformat_open_input, AVChapter, AVCodecTag,
    AVDurationEstimationMethod, AVFormatContext, AVIAMFAudioElement, AVIAMFMixPresentation,
    AVInputFormat, AVOutputFormat, AVProgram, AVStream, AVStreamGroup, AVStreamGroupLCEVC,
    AVStreamGroupParamsType, AVStreamGroupTileGrid, C2RustUnnamed_0, C2RustUnnamed_1,
    AVFMT_DURATION_FROM_BITRATE, AVFMT_DURATION_FROM_PTS, AVFMT_DURATION_FROM_STREAM,
    AV_STREAM_GROUP_PARAMS_IAMF_AUDIO_ELEMENT, AV_STREAM_GROUP_PARAMS_IAMF_MIX_PRESENTATION,
    AV_STREAM_GROUP_PARAMS_LCEVC, AV_STREAM_GROUP_PARAMS_NONE, AV_STREAM_GROUP_PARAMS_TILE_GRID,
};
pub use self::avio_h::{
    AVIOContext, AVIODataMarkerType, AVIOInterruptCB, AVIO_DATA_MARKER_BOUNDARY_POINT,
    AVIO_DATA_MARKER_FLUSH_POINT, AVIO_DATA_MARKER_HEADER, AVIO_DATA_MARKER_SYNC_POINT,
    AVIO_DATA_MARKER_TRAILER, AVIO_DATA_MARKER_UNKNOWN,
};
pub use self::avutil_h::{
    AVMediaType, AVPictureType, AVMEDIA_TYPE_ATTACHMENT, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_DATA,
    AVMEDIA_TYPE_NB, AVMEDIA_TYPE_SUBTITLE, AVMEDIA_TYPE_UNKNOWN, AVMEDIA_TYPE_VIDEO,
    AV_NOPTS_VALUE, AV_PICTURE_TYPE_B, AV_PICTURE_TYPE_BI, AV_PICTURE_TYPE_I, AV_PICTURE_TYPE_NONE,
    AV_PICTURE_TYPE_P, AV_PICTURE_TYPE_S, AV_PICTURE_TYPE_SI, AV_PICTURE_TYPE_SP,
};
pub use self::buffer_h::{AVBuffer, AVBufferRef};
pub use self::channel_layout_h::{
    AVChannel, AVChannelCustom, AVChannelLayout, AVChannelOrder, C2RustUnnamed,
    AV_CHANNEL_ORDER_AMBISONIC, AV_CHANNEL_ORDER_CUSTOM, AV_CHANNEL_ORDER_NATIVE,
    AV_CHANNEL_ORDER_UNSPEC, AV_CHAN_AMBISONIC_BASE, AV_CHAN_AMBISONIC_END, AV_CHAN_BACK_CENTER,
    AV_CHAN_BACK_LEFT, AV_CHAN_BACK_RIGHT, AV_CHAN_BINAURAL_LEFT, AV_CHAN_BINAURAL_RIGHT,
    AV_CHAN_BOTTOM_FRONT_CENTER, AV_CHAN_BOTTOM_FRONT_LEFT, AV_CHAN_BOTTOM_FRONT_RIGHT,
    AV_CHAN_FRONT_CENTER, AV_CHAN_FRONT_LEFT, AV_CHAN_FRONT_LEFT_OF_CENTER, AV_CHAN_FRONT_RIGHT,
    AV_CHAN_FRONT_RIGHT_OF_CENTER, AV_CHAN_LOW_FREQUENCY, AV_CHAN_LOW_FREQUENCY_2, AV_CHAN_NONE,
    AV_CHAN_SIDE_LEFT, AV_CHAN_SIDE_RIGHT, AV_CHAN_SIDE_SURROUND_LEFT, AV_CHAN_SIDE_SURROUND_RIGHT,
    AV_CHAN_STEREO_LEFT, AV_CHAN_STEREO_RIGHT, AV_CHAN_SURROUND_DIRECT_LEFT,
    AV_CHAN_SURROUND_DIRECT_RIGHT, AV_CHAN_TOP_BACK_CENTER, AV_CHAN_TOP_BACK_LEFT,
    AV_CHAN_TOP_BACK_RIGHT, AV_CHAN_TOP_CENTER, AV_CHAN_TOP_FRONT_CENTER, AV_CHAN_TOP_FRONT_LEFT,
    AV_CHAN_TOP_FRONT_RIGHT, AV_CHAN_TOP_SIDE_LEFT, AV_CHAN_TOP_SIDE_RIGHT,
    AV_CHAN_TOP_SURROUND_LEFT, AV_CHAN_TOP_SURROUND_RIGHT, AV_CHAN_UNKNOWN, AV_CHAN_UNUSED,
    AV_CHAN_WIDE_LEFT, AV_CHAN_WIDE_RIGHT, FF_CHANNEL_ORDER_NB,
};
pub use self::codec_desc_h::AVCodecDescriptor;
pub use self::codec_h::{avcodec_find_decoder, AVCodec, AVProfile};
pub use self::codec_id_h::{
    AVCodecID, AV_CODEC_ID_012V, AV_CODEC_ID_4GV, AV_CODEC_ID_4XM, AV_CODEC_ID_8BPS,
    AV_CODEC_ID_8SVX_EXP, AV_CODEC_ID_8SVX_FIB, AV_CODEC_ID_A64_MULTI, AV_CODEC_ID_A64_MULTI5,
    AV_CODEC_ID_AAC, AV_CODEC_ID_AAC_LATM, AV_CODEC_ID_AASC, AV_CODEC_ID_AC3, AV_CODEC_ID_AC4,
    AV_CODEC_ID_ACELP_KELVIN, AV_CODEC_ID_ADPCM_4XM, AV_CODEC_ID_ADPCM_ADX, AV_CODEC_ID_ADPCM_AFC,
    AV_CODEC_ID_ADPCM_AGM, AV_CODEC_ID_ADPCM_AICA, AV_CODEC_ID_ADPCM_ARGO, AV_CODEC_ID_ADPCM_CT,
    AV_CODEC_ID_ADPCM_DTK, AV_CODEC_ID_ADPCM_EA, AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
    AV_CODEC_ID_ADPCM_EA_R1, AV_CODEC_ID_ADPCM_EA_R2, AV_CODEC_ID_ADPCM_EA_R3,
    AV_CODEC_ID_ADPCM_EA_XAS, AV_CODEC_ID_ADPCM_G722, AV_CODEC_ID_ADPCM_G726,
    AV_CODEC_ID_ADPCM_G726LE, AV_CODEC_ID_ADPCM_IMA_ACORN, AV_CODEC_ID_ADPCM_IMA_ALP,
    AV_CODEC_ID_ADPCM_IMA_AMV, AV_CODEC_ID_ADPCM_IMA_APC, AV_CODEC_ID_ADPCM_IMA_APM,
    AV_CODEC_ID_ADPCM_IMA_CUNNING, AV_CODEC_ID_ADPCM_IMA_DAT4, AV_CODEC_ID_ADPCM_IMA_DK3,
    AV_CODEC_ID_ADPCM_IMA_DK4, AV_CODEC_ID_ADPCM_IMA_EA_EACS, AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
    AV_CODEC_ID_ADPCM_IMA_ISS, AV_CODEC_ID_ADPCM_IMA_MOFLEX, AV_CODEC_ID_ADPCM_IMA_MTF,
    AV_CODEC_ID_ADPCM_IMA_OKI, AV_CODEC_ID_ADPCM_IMA_QT, AV_CODEC_ID_ADPCM_IMA_RAD,
    AV_CODEC_ID_ADPCM_IMA_SMJPEG, AV_CODEC_ID_ADPCM_IMA_SSI, AV_CODEC_ID_ADPCM_IMA_WAV,
    AV_CODEC_ID_ADPCM_IMA_WS, AV_CODEC_ID_ADPCM_IMA_XBOX, AV_CODEC_ID_ADPCM_MS,
    AV_CODEC_ID_ADPCM_MTAF, AV_CODEC_ID_ADPCM_PSX, AV_CODEC_ID_ADPCM_SANYO,
    AV_CODEC_ID_ADPCM_SBPRO_2, AV_CODEC_ID_ADPCM_SBPRO_3, AV_CODEC_ID_ADPCM_SBPRO_4,
    AV_CODEC_ID_ADPCM_SWF, AV_CODEC_ID_ADPCM_THP, AV_CODEC_ID_ADPCM_THP_LE, AV_CODEC_ID_ADPCM_VIMA,
    AV_CODEC_ID_ADPCM_XA, AV_CODEC_ID_ADPCM_XMD, AV_CODEC_ID_ADPCM_YAMAHA, AV_CODEC_ID_ADPCM_ZORK,
    AV_CODEC_ID_AGM, AV_CODEC_ID_AIC, AV_CODEC_ID_ALAC, AV_CODEC_ID_ALIAS_PIX, AV_CODEC_ID_AMR_NB,
    AV_CODEC_ID_AMR_WB, AV_CODEC_ID_AMV, AV_CODEC_ID_ANM, AV_CODEC_ID_ANSI, AV_CODEC_ID_ANULL,
    AV_CODEC_ID_APAC, AV_CODEC_ID_APE, AV_CODEC_ID_APNG, AV_CODEC_ID_APTX, AV_CODEC_ID_APTX_HD,
    AV_CODEC_ID_APV, AV_CODEC_ID_ARBC, AV_CODEC_ID_ARGO, AV_CODEC_ID_ARIB_CAPTION, AV_CODEC_ID_ASS,
    AV_CODEC_ID_ASV1, AV_CODEC_ID_ASV2, AV_CODEC_ID_ATRAC1, AV_CODEC_ID_ATRAC3,
    AV_CODEC_ID_ATRAC3AL, AV_CODEC_ID_ATRAC3P, AV_CODEC_ID_ATRAC3PAL, AV_CODEC_ID_ATRAC9,
    AV_CODEC_ID_AURA, AV_CODEC_ID_AURA2, AV_CODEC_ID_AV1, AV_CODEC_ID_AVRN, AV_CODEC_ID_AVRP,
    AV_CODEC_ID_AVS, AV_CODEC_ID_AVS2, AV_CODEC_ID_AVS3, AV_CODEC_ID_AVUI, AV_CODEC_ID_BETHSOFTVID,
    AV_CODEC_ID_BFI, AV_CODEC_ID_BINKAUDIO_DCT, AV_CODEC_ID_BINKAUDIO_RDFT, AV_CODEC_ID_BINKVIDEO,
    AV_CODEC_ID_BINTEXT, AV_CODEC_ID_BIN_DATA, AV_CODEC_ID_BITPACKED, AV_CODEC_ID_BMP,
    AV_CODEC_ID_BMV_AUDIO, AV_CODEC_ID_BMV_VIDEO, AV_CODEC_ID_BONK, AV_CODEC_ID_BRENDER_PIX,
    AV_CODEC_ID_C93, AV_CODEC_ID_CAVS, AV_CODEC_ID_CBD2_DPCM, AV_CODEC_ID_CDGRAPHICS,
    AV_CODEC_ID_CDTOONS, AV_CODEC_ID_CDXL, AV_CODEC_ID_CELT, AV_CODEC_ID_CFHD, AV_CODEC_ID_CINEPAK,
    AV_CODEC_ID_CLEARVIDEO, AV_CODEC_ID_CLJR, AV_CODEC_ID_CLLC, AV_CODEC_ID_CMV,
    AV_CODEC_ID_CODEC2, AV_CODEC_ID_COMFORT_NOISE, AV_CODEC_ID_COOK, AV_CODEC_ID_CPIA,
    AV_CODEC_ID_CRI, AV_CODEC_ID_CSCD, AV_CODEC_ID_CYUV, AV_CODEC_ID_DAALA, AV_CODEC_ID_DDS,
    AV_CODEC_ID_DERF_DPCM, AV_CODEC_ID_DFA, AV_CODEC_ID_DFPWM, AV_CODEC_ID_DIRAC,
    AV_CODEC_ID_DNXHD, AV_CODEC_ID_DNXUC, AV_CODEC_ID_DOLBY_E, AV_CODEC_ID_DPX,
    AV_CODEC_ID_DSD_LSBF, AV_CODEC_ID_DSD_LSBF_PLANAR, AV_CODEC_ID_DSD_MSBF,
    AV_CODEC_ID_DSD_MSBF_PLANAR, AV_CODEC_ID_DSICINAUDIO, AV_CODEC_ID_DSICINVIDEO,
    AV_CODEC_ID_DSS_SP, AV_CODEC_ID_DST, AV_CODEC_ID_DTS, AV_CODEC_ID_DVAUDIO,
    AV_CODEC_ID_DVB_SUBTITLE, AV_CODEC_ID_DVB_TELETEXT, AV_CODEC_ID_DVD_NAV,
    AV_CODEC_ID_DVD_SUBTITLE, AV_CODEC_ID_DVVIDEO, AV_CODEC_ID_DXA, AV_CODEC_ID_DXTORY,
    AV_CODEC_ID_DXV, AV_CODEC_ID_EAC3, AV_CODEC_ID_EIA_608, AV_CODEC_ID_EPG, AV_CODEC_ID_ESCAPE124,
    AV_CODEC_ID_ESCAPE130, AV_CODEC_ID_EVC, AV_CODEC_ID_EVRC, AV_CODEC_ID_EXR,
    AV_CODEC_ID_FASTAUDIO, AV_CODEC_ID_FFMETADATA, AV_CODEC_ID_FFV1, AV_CODEC_ID_FFVHUFF,
    AV_CODEC_ID_FFWAVESYNTH, AV_CODEC_ID_FIC, AV_CODEC_ID_FIRST_AUDIO, AV_CODEC_ID_FIRST_SUBTITLE,
    AV_CODEC_ID_FIRST_UNKNOWN, AV_CODEC_ID_FITS, AV_CODEC_ID_FLAC, AV_CODEC_ID_FLASHSV,
    AV_CODEC_ID_FLASHSV2, AV_CODEC_ID_FLIC, AV_CODEC_ID_FLV1, AV_CODEC_ID_FMVC, AV_CODEC_ID_FRAPS,
    AV_CODEC_ID_FRWU, AV_CODEC_ID_FTR, AV_CODEC_ID_G2M, AV_CODEC_ID_G723_1, AV_CODEC_ID_G728,
    AV_CODEC_ID_G729, AV_CODEC_ID_GDV, AV_CODEC_ID_GEM, AV_CODEC_ID_GIF, AV_CODEC_ID_GREMLIN_DPCM,
    AV_CODEC_ID_GSM, AV_CODEC_ID_GSM_MS, AV_CODEC_ID_H261, AV_CODEC_ID_H263, AV_CODEC_ID_H263I,
    AV_CODEC_ID_H263P, AV_CODEC_ID_H264, AV_CODEC_ID_HAP, AV_CODEC_ID_HCA, AV_CODEC_ID_HCOM,
    AV_CODEC_ID_HDMV_PGS_SUBTITLE, AV_CODEC_ID_HDMV_TEXT_SUBTITLE, AV_CODEC_ID_HEVC,
    AV_CODEC_ID_HNM4_VIDEO, AV_CODEC_ID_HQX, AV_CODEC_ID_HQ_HQA, AV_CODEC_ID_HUFFYUV,
    AV_CODEC_ID_HYMT, AV_CODEC_ID_IAC, AV_CODEC_ID_IDCIN, AV_CODEC_ID_IDF, AV_CODEC_ID_IFF_ILBM,
    AV_CODEC_ID_ILBC, AV_CODEC_ID_IMC, AV_CODEC_ID_IMM4, AV_CODEC_ID_IMM5, AV_CODEC_ID_INDEO2,
    AV_CODEC_ID_INDEO3, AV_CODEC_ID_INDEO4, AV_CODEC_ID_INDEO5, AV_CODEC_ID_INTERPLAY_ACM,
    AV_CODEC_ID_INTERPLAY_DPCM, AV_CODEC_ID_INTERPLAY_VIDEO, AV_CODEC_ID_IPU, AV_CODEC_ID_IVTV_VBI,
    AV_CODEC_ID_JACOSUB, AV_CODEC_ID_JPEG2000, AV_CODEC_ID_JPEGLS, AV_CODEC_ID_JPEGXL,
    AV_CODEC_ID_JPEGXL_ANIM, AV_CODEC_ID_JV, AV_CODEC_ID_KGV1, AV_CODEC_ID_KMVC,
    AV_CODEC_ID_LAGARITH, AV_CODEC_ID_LC3, AV_CODEC_ID_LCEVC, AV_CODEC_ID_LEAD, AV_CODEC_ID_LJPEG,
    AV_CODEC_ID_LOCO, AV_CODEC_ID_LSCR, AV_CODEC_ID_M101, AV_CODEC_ID_MACE3, AV_CODEC_ID_MACE6,
    AV_CODEC_ID_MAD, AV_CODEC_ID_MAGICYUV, AV_CODEC_ID_MDEC, AV_CODEC_ID_MEDIA100,
    AV_CODEC_ID_METASOUND, AV_CODEC_ID_MICRODVD, AV_CODEC_ID_MIMIC, AV_CODEC_ID_MISC4,
    AV_CODEC_ID_MJPEG, AV_CODEC_ID_MJPEGB, AV_CODEC_ID_MLP, AV_CODEC_ID_MMVIDEO,
    AV_CODEC_ID_MOBICLIP, AV_CODEC_ID_MOTIONPIXELS, AV_CODEC_ID_MOV_TEXT, AV_CODEC_ID_MP1,
    AV_CODEC_ID_MP2, AV_CODEC_ID_MP3, AV_CODEC_ID_MP3ADU, AV_CODEC_ID_MP3ON4, AV_CODEC_ID_MP4ALS,
    AV_CODEC_ID_MPEG1VIDEO, AV_CODEC_ID_MPEG2TS, AV_CODEC_ID_MPEG2VIDEO, AV_CODEC_ID_MPEG4,
    AV_CODEC_ID_MPEG4SYSTEMS, AV_CODEC_ID_MPEGH_3D_AUDIO, AV_CODEC_ID_MPL2, AV_CODEC_ID_MSA1,
    AV_CODEC_ID_MSCC, AV_CODEC_ID_MSMPEG4V1, AV_CODEC_ID_MSMPEG4V2, AV_CODEC_ID_MSMPEG4V3,
    AV_CODEC_ID_MSNSIREN, AV_CODEC_ID_MSP2, AV_CODEC_ID_MSRLE, AV_CODEC_ID_MSS1, AV_CODEC_ID_MSS2,
    AV_CODEC_ID_MSVIDEO1, AV_CODEC_ID_MSZH, AV_CODEC_ID_MTS2, AV_CODEC_ID_MUSEPACK7,
    AV_CODEC_ID_MUSEPACK8, AV_CODEC_ID_MV30, AV_CODEC_ID_MVC1, AV_CODEC_ID_MVC2, AV_CODEC_ID_MVDV,
    AV_CODEC_ID_MVHA, AV_CODEC_ID_MWSC, AV_CODEC_ID_MXPEG, AV_CODEC_ID_NELLYMOSER,
    AV_CODEC_ID_NONE, AV_CODEC_ID_NOTCHLC, AV_CODEC_ID_NUV, AV_CODEC_ID_ON2AVC, AV_CODEC_ID_OPUS,
    AV_CODEC_ID_OSQ, AV_CODEC_ID_OTF, AV_CODEC_ID_PAF_AUDIO, AV_CODEC_ID_PAF_VIDEO,
    AV_CODEC_ID_PAM, AV_CODEC_ID_PBM, AV_CODEC_ID_PCM_ALAW, AV_CODEC_ID_PCM_BLURAY,
    AV_CODEC_ID_PCM_DVD, AV_CODEC_ID_PCM_F16LE, AV_CODEC_ID_PCM_F24LE, AV_CODEC_ID_PCM_F32BE,
    AV_CODEC_ID_PCM_F32LE, AV_CODEC_ID_PCM_F64BE, AV_CODEC_ID_PCM_F64LE, AV_CODEC_ID_PCM_LXF,
    AV_CODEC_ID_PCM_MULAW, AV_CODEC_ID_PCM_S16BE, AV_CODEC_ID_PCM_S16BE_PLANAR,
    AV_CODEC_ID_PCM_S16LE, AV_CODEC_ID_PCM_S16LE_PLANAR, AV_CODEC_ID_PCM_S24BE,
    AV_CODEC_ID_PCM_S24DAUD, AV_CODEC_ID_PCM_S24LE, AV_CODEC_ID_PCM_S24LE_PLANAR,
    AV_CODEC_ID_PCM_S32BE, AV_CODEC_ID_PCM_S32LE, AV_CODEC_ID_PCM_S32LE_PLANAR,
    AV_CODEC_ID_PCM_S64BE, AV_CODEC_ID_PCM_S64LE, AV_CODEC_ID_PCM_S8, AV_CODEC_ID_PCM_S8_PLANAR,
    AV_CODEC_ID_PCM_SGA, AV_CODEC_ID_PCM_U16BE, AV_CODEC_ID_PCM_U16LE, AV_CODEC_ID_PCM_U24BE,
    AV_CODEC_ID_PCM_U24LE, AV_CODEC_ID_PCM_U32BE, AV_CODEC_ID_PCM_U32LE, AV_CODEC_ID_PCM_U8,
    AV_CODEC_ID_PCM_VIDC, AV_CODEC_ID_PCM_ZORK, AV_CODEC_ID_PCX, AV_CODEC_ID_PDV, AV_CODEC_ID_PFM,
    AV_CODEC_ID_PGM, AV_CODEC_ID_PGMYUV, AV_CODEC_ID_PGX, AV_CODEC_ID_PHM, AV_CODEC_ID_PHOTOCD,
    AV_CODEC_ID_PICTOR, AV_CODEC_ID_PIXLET, AV_CODEC_ID_PJS, AV_CODEC_ID_PNG, AV_CODEC_ID_PPM,
    AV_CODEC_ID_PROBE, AV_CODEC_ID_PRORES, AV_CODEC_ID_PRORES_RAW, AV_CODEC_ID_PROSUMER,
    AV_CODEC_ID_PSD, AV_CODEC_ID_PTX, AV_CODEC_ID_QCELP, AV_CODEC_ID_QDM2, AV_CODEC_ID_QDMC,
    AV_CODEC_ID_QDRAW, AV_CODEC_ID_QOA, AV_CODEC_ID_QOI, AV_CODEC_ID_QPEG, AV_CODEC_ID_QTRLE,
    AV_CODEC_ID_R10K, AV_CODEC_ID_R210, AV_CODEC_ID_RADIANCE_HDR, AV_CODEC_ID_RALF,
    AV_CODEC_ID_RASC, AV_CODEC_ID_RAWVIDEO, AV_CODEC_ID_RA_144, AV_CODEC_ID_RA_288,
    AV_CODEC_ID_REALTEXT, AV_CODEC_ID_RKA, AV_CODEC_ID_RL2, AV_CODEC_ID_ROQ, AV_CODEC_ID_ROQ_DPCM,
    AV_CODEC_ID_RPZA, AV_CODEC_ID_RSCC, AV_CODEC_ID_RTV1, AV_CODEC_ID_RV10, AV_CODEC_ID_RV20,
    AV_CODEC_ID_RV30, AV_CODEC_ID_RV40, AV_CODEC_ID_RV60, AV_CODEC_ID_S302M, AV_CODEC_ID_SAMI,
    AV_CODEC_ID_SANM, AV_CODEC_ID_SBC, AV_CODEC_ID_SCPR, AV_CODEC_ID_SCREENPRESSO,
    AV_CODEC_ID_SCTE_35, AV_CODEC_ID_SDX2_DPCM, AV_CODEC_ID_SGA_VIDEO, AV_CODEC_ID_SGI,
    AV_CODEC_ID_SGIRLE, AV_CODEC_ID_SHEERVIDEO, AV_CODEC_ID_SHORTEN, AV_CODEC_ID_SIMBIOSIS_IMX,
    AV_CODEC_ID_SIPR, AV_CODEC_ID_SIREN, AV_CODEC_ID_SMACKAUDIO, AV_CODEC_ID_SMACKVIDEO,
    AV_CODEC_ID_SMC, AV_CODEC_ID_SMPTE_2038, AV_CODEC_ID_SMPTE_436M_ANC, AV_CODEC_ID_SMPTE_KLV,
    AV_CODEC_ID_SMV, AV_CODEC_ID_SMVJPEG, AV_CODEC_ID_SNOW, AV_CODEC_ID_SOL_DPCM,
    AV_CODEC_ID_SONIC, AV_CODEC_ID_SONIC_LS, AV_CODEC_ID_SP5X, AV_CODEC_ID_SPEEDHQ,
    AV_CODEC_ID_SPEEX, AV_CODEC_ID_SRGC, AV_CODEC_ID_SRT, AV_CODEC_ID_SSA, AV_CODEC_ID_STL,
    AV_CODEC_ID_SUBRIP, AV_CODEC_ID_SUBVIEWER, AV_CODEC_ID_SUBVIEWER1, AV_CODEC_ID_SUNRAST,
    AV_CODEC_ID_SVG, AV_CODEC_ID_SVQ1, AV_CODEC_ID_SVQ3, AV_CODEC_ID_TAK, AV_CODEC_ID_TARGA,
    AV_CODEC_ID_TARGA_Y216, AV_CODEC_ID_TDSC, AV_CODEC_ID_TEXT, AV_CODEC_ID_TGQ, AV_CODEC_ID_TGV,
    AV_CODEC_ID_THEORA, AV_CODEC_ID_THP, AV_CODEC_ID_TIERTEXSEQVIDEO, AV_CODEC_ID_TIFF,
    AV_CODEC_ID_TIMED_ID3, AV_CODEC_ID_TMV, AV_CODEC_ID_TQI, AV_CODEC_ID_TRUEHD,
    AV_CODEC_ID_TRUEMOTION1, AV_CODEC_ID_TRUEMOTION2, AV_CODEC_ID_TRUEMOTION2RT,
    AV_CODEC_ID_TRUESPEECH, AV_CODEC_ID_TSCC, AV_CODEC_ID_TSCC2, AV_CODEC_ID_TTA, AV_CODEC_ID_TTF,
    AV_CODEC_ID_TTML, AV_CODEC_ID_TWINVQ, AV_CODEC_ID_TXD, AV_CODEC_ID_ULTI, AV_CODEC_ID_UTVIDEO,
    AV_CODEC_ID_V210, AV_CODEC_ID_V210X, AV_CODEC_ID_V308, AV_CODEC_ID_V408, AV_CODEC_ID_V410,
    AV_CODEC_ID_VB, AV_CODEC_ID_VBLE, AV_CODEC_ID_VBN, AV_CODEC_ID_VC1, AV_CODEC_ID_VC1IMAGE,
    AV_CODEC_ID_VCR1, AV_CODEC_ID_VIXL, AV_CODEC_ID_VMDAUDIO, AV_CODEC_ID_VMDVIDEO,
    AV_CODEC_ID_VMIX, AV_CODEC_ID_VMNC, AV_CODEC_ID_VNULL, AV_CODEC_ID_VORBIS, AV_CODEC_ID_VP3,
    AV_CODEC_ID_VP4, AV_CODEC_ID_VP5, AV_CODEC_ID_VP6, AV_CODEC_ID_VP6A, AV_CODEC_ID_VP6F,
    AV_CODEC_ID_VP7, AV_CODEC_ID_VP8, AV_CODEC_ID_VP9, AV_CODEC_ID_VPLAYER, AV_CODEC_ID_VQC,
    AV_CODEC_ID_VVC, AV_CODEC_ID_WADY_DPCM, AV_CODEC_ID_WAVARC, AV_CODEC_ID_WAVPACK,
    AV_CODEC_ID_WBMP, AV_CODEC_ID_WCMV, AV_CODEC_ID_WEBP, AV_CODEC_ID_WEBVTT,
    AV_CODEC_ID_WESTWOOD_SND1, AV_CODEC_ID_WMALOSSLESS, AV_CODEC_ID_WMAPRO, AV_CODEC_ID_WMAV1,
    AV_CODEC_ID_WMAV2, AV_CODEC_ID_WMAVOICE, AV_CODEC_ID_WMV1, AV_CODEC_ID_WMV2, AV_CODEC_ID_WMV3,
    AV_CODEC_ID_WMV3IMAGE, AV_CODEC_ID_WNV1, AV_CODEC_ID_WRAPPED_AVFRAME, AV_CODEC_ID_WS_VQA,
    AV_CODEC_ID_XAN_DPCM, AV_CODEC_ID_XAN_WC3, AV_CODEC_ID_XAN_WC4, AV_CODEC_ID_XBIN,
    AV_CODEC_ID_XBM, AV_CODEC_ID_XFACE, AV_CODEC_ID_XMA1, AV_CODEC_ID_XMA2, AV_CODEC_ID_XPM,
    AV_CODEC_ID_XSUB, AV_CODEC_ID_XWD, AV_CODEC_ID_Y41P, AV_CODEC_ID_YLC, AV_CODEC_ID_YOP,
    AV_CODEC_ID_YUV4, AV_CODEC_ID_ZEROCODEC, AV_CODEC_ID_ZLIB, AV_CODEC_ID_ZMBV,
};
pub use self::codec_par_h::AVCodecParameters;
pub use self::defs_h::{
    AVAudioServiceType, AVDiscard, AVFieldOrder, AVDISCARD_ALL, AVDISCARD_BIDIR, AVDISCARD_DEFAULT,
    AVDISCARD_NONE, AVDISCARD_NONINTRA, AVDISCARD_NONKEY, AVDISCARD_NONREF,
    AV_AUDIO_SERVICE_TYPE_COMMENTARY, AV_AUDIO_SERVICE_TYPE_DIALOGUE,
    AV_AUDIO_SERVICE_TYPE_EFFECTS, AV_AUDIO_SERVICE_TYPE_EMERGENCY,
    AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED, AV_AUDIO_SERVICE_TYPE_KARAOKE,
    AV_AUDIO_SERVICE_TYPE_MAIN, AV_AUDIO_SERVICE_TYPE_NB, AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED,
    AV_AUDIO_SERVICE_TYPE_VOICE_OVER, AV_FIELD_BB, AV_FIELD_BT, AV_FIELD_PROGRESSIVE, AV_FIELD_TB,
    AV_FIELD_TT, AV_FIELD_UNKNOWN,
};
use self::dict_h::{av_dict_free, av_dict_set, AVDictionary};
pub use self::error_h::AVERROR_EOF;
pub use self::frame_h::{
    av_frame_alloc, av_frame_free, AVFrame, AVFrameSideData, AVFrameSideDataType,
    AV_FRAME_DATA_3D_REFERENCE_DISPLAYS, AV_FRAME_DATA_A53_CC, AV_FRAME_DATA_AFD,
    AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT, AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
    AV_FRAME_DATA_CONTENT_LIGHT_LEVEL, AV_FRAME_DATA_DETECTION_BBOXES, AV_FRAME_DATA_DISPLAYMATRIX,
    AV_FRAME_DATA_DOVI_METADATA, AV_FRAME_DATA_DOVI_RPU_BUFFER, AV_FRAME_DATA_DOWNMIX_INFO,
    AV_FRAME_DATA_DYNAMIC_HDR_PLUS, AV_FRAME_DATA_DYNAMIC_HDR_VIVID,
    AV_FRAME_DATA_FILM_GRAIN_PARAMS, AV_FRAME_DATA_GOP_TIMECODE, AV_FRAME_DATA_ICC_PROFILE,
    AV_FRAME_DATA_LCEVC, AV_FRAME_DATA_MASTERING_DISPLAY_METADATA, AV_FRAME_DATA_MATRIXENCODING,
    AV_FRAME_DATA_MOTION_VECTORS, AV_FRAME_DATA_PANSCAN, AV_FRAME_DATA_REGIONS_OF_INTEREST,
    AV_FRAME_DATA_REPLAYGAIN, AV_FRAME_DATA_S12M_TIMECODE, AV_FRAME_DATA_SEI_UNREGISTERED,
    AV_FRAME_DATA_SKIP_SAMPLES, AV_FRAME_DATA_SPHERICAL, AV_FRAME_DATA_STEREO3D,
    AV_FRAME_DATA_VIDEO_ENC_PARAMS, AV_FRAME_DATA_VIDEO_HINT, AV_FRAME_DATA_VIEW_ID,
    AV_FRAME_FLAG_INTERLACED, AV_FRAME_FLAG_TOP_FIELD_FIRST,
};
pub use self::input_h::{
    cli_image_t, cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t, x264_cli_pic_alloc,
    X264_CSP_OTHER,
};
pub use self::log_h::{
    AVClass, AVClassCategory, AVOption, AVOptionRanges, AV_CLASS_CATEGORY_BITSTREAM_FILTER,
    AV_CLASS_CATEGORY_DECODER, AV_CLASS_CATEGORY_DEMUXER, AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT, AV_CLASS_CATEGORY_DEVICE_INPUT,
    AV_CLASS_CATEGORY_DEVICE_OUTPUT, AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT, AV_CLASS_CATEGORY_ENCODER, AV_CLASS_CATEGORY_FILTER,
    AV_CLASS_CATEGORY_HWDEVICE, AV_CLASS_CATEGORY_INPUT, AV_CLASS_CATEGORY_MUXER,
    AV_CLASS_CATEGORY_NA, AV_CLASS_CATEGORY_NB, AV_CLASS_CATEGORY_OUTPUT,
    AV_CLASS_CATEGORY_SWRESAMPLER, AV_CLASS_CATEGORY_SWSCALER,
};
pub use self::packet_h::{
    av_packet_alloc, av_packet_free, av_packet_unref, AVPacket, AVPacketSideData,
    AVPacketSideDataType, AV_PKT_DATA_3D_REFERENCE_DISPLAYS, AV_PKT_DATA_A53_CC, AV_PKT_DATA_AFD,
    AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT, AV_PKT_DATA_AUDIO_SERVICE_TYPE,
    AV_PKT_DATA_CONTENT_LIGHT_LEVEL, AV_PKT_DATA_CPB_PROPERTIES, AV_PKT_DATA_DISPLAYMATRIX,
    AV_PKT_DATA_DOVI_CONF, AV_PKT_DATA_DYNAMIC_HDR10_PLUS, AV_PKT_DATA_ENCRYPTION_INFO,
    AV_PKT_DATA_ENCRYPTION_INIT_INFO, AV_PKT_DATA_FALLBACK_TRACK, AV_PKT_DATA_FRAME_CROPPING,
    AV_PKT_DATA_H263_MB_INFO, AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM,
    AV_PKT_DATA_IAMF_MIX_GAIN_PARAM, AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM,
    AV_PKT_DATA_ICC_PROFILE, AV_PKT_DATA_JP_DUALMONO, AV_PKT_DATA_LCEVC,
    AV_PKT_DATA_MASTERING_DISPLAY_METADATA, AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
    AV_PKT_DATA_METADATA_UPDATE, AV_PKT_DATA_MPEGTS_STREAM_ID, AV_PKT_DATA_NB,
    AV_PKT_DATA_NEW_EXTRADATA, AV_PKT_DATA_PALETTE, AV_PKT_DATA_PARAM_CHANGE, AV_PKT_DATA_PRFT,
    AV_PKT_DATA_QUALITY_STATS, AV_PKT_DATA_REPLAYGAIN, AV_PKT_DATA_RTCP_SR,
    AV_PKT_DATA_S12M_TIMECODE, AV_PKT_DATA_SKIP_SAMPLES, AV_PKT_DATA_SPHERICAL,
    AV_PKT_DATA_STEREO3D, AV_PKT_DATA_STRINGS_METADATA, AV_PKT_DATA_SUBTITLE_POSITION,
    AV_PKT_DATA_WEBVTT_IDENTIFIER, AV_PKT_DATA_WEBVTT_SETTINGS,
};
use self::pixdesc_h::av_get_pix_fmt_name;
pub use self::pixfmt_h::{
    AVChromaLocation, AVColorPrimaries, AVColorRange, AVColorSpace, AVColorTransferCharacteristic,
    AVPixelFormat, AVCHROMA_LOC_BOTTOM, AVCHROMA_LOC_BOTTOMLEFT, AVCHROMA_LOC_CENTER,
    AVCHROMA_LOC_LEFT, AVCHROMA_LOC_NB, AVCHROMA_LOC_TOP, AVCHROMA_LOC_TOPLEFT,
    AVCHROMA_LOC_UNSPECIFIED, AVCOL_PRI_BT2020, AVCOL_PRI_BT470BG, AVCOL_PRI_BT470M,
    AVCOL_PRI_BT709, AVCOL_PRI_EBU3213, AVCOL_PRI_FILM, AVCOL_PRI_JEDEC_P22, AVCOL_PRI_NB,
    AVCOL_PRI_RESERVED, AVCOL_PRI_RESERVED0, AVCOL_PRI_SMPTE170M, AVCOL_PRI_SMPTE240M,
    AVCOL_PRI_SMPTE428, AVCOL_PRI_SMPTE431, AVCOL_PRI_SMPTE432, AVCOL_PRI_SMPTEST428_1,
    AVCOL_PRI_UNSPECIFIED, AVCOL_RANGE_JPEG, AVCOL_RANGE_MPEG, AVCOL_RANGE_NB,
    AVCOL_RANGE_UNSPECIFIED, AVCOL_SPC_BT2020_CL, AVCOL_SPC_BT2020_NCL, AVCOL_SPC_BT470BG,
    AVCOL_SPC_BT709, AVCOL_SPC_CHROMA_DERIVED_CL, AVCOL_SPC_CHROMA_DERIVED_NCL, AVCOL_SPC_FCC,
    AVCOL_SPC_ICTCP, AVCOL_SPC_IPT_C2, AVCOL_SPC_NB, AVCOL_SPC_RESERVED, AVCOL_SPC_RGB,
    AVCOL_SPC_SMPTE170M, AVCOL_SPC_SMPTE2085, AVCOL_SPC_SMPTE240M, AVCOL_SPC_UNSPECIFIED,
    AVCOL_SPC_YCGCO, AVCOL_SPC_YCGCO_RE, AVCOL_SPC_YCGCO_RO, AVCOL_SPC_YCOCG,
    AVCOL_TRC_ARIB_STD_B67, AVCOL_TRC_BT1361_ECG, AVCOL_TRC_BT2020_10, AVCOL_TRC_BT2020_12,
    AVCOL_TRC_BT709, AVCOL_TRC_GAMMA22, AVCOL_TRC_GAMMA28, AVCOL_TRC_IEC61966_2_1,
    AVCOL_TRC_IEC61966_2_4, AVCOL_TRC_LINEAR, AVCOL_TRC_LOG, AVCOL_TRC_LOG_SQRT, AVCOL_TRC_NB,
    AVCOL_TRC_RESERVED, AVCOL_TRC_RESERVED0, AVCOL_TRC_SMPTE170M, AVCOL_TRC_SMPTE2084,
    AVCOL_TRC_SMPTE240M, AVCOL_TRC_SMPTE428, AVCOL_TRC_SMPTEST2084, AVCOL_TRC_SMPTEST428_1,
    AVCOL_TRC_UNSPECIFIED, AV_PIX_FMT_0BGR, AV_PIX_FMT_0RGB, AV_PIX_FMT_ABGR,
    AV_PIX_FMT_AMF_SURFACE, AV_PIX_FMT_ARGB, AV_PIX_FMT_AYUV, AV_PIX_FMT_AYUV64BE,
    AV_PIX_FMT_AYUV64LE, AV_PIX_FMT_BAYER_BGGR16BE, AV_PIX_FMT_BAYER_BGGR16LE,
    AV_PIX_FMT_BAYER_BGGR8, AV_PIX_FMT_BAYER_GBRG16BE, AV_PIX_FMT_BAYER_GBRG16LE,
    AV_PIX_FMT_BAYER_GBRG8, AV_PIX_FMT_BAYER_GRBG16BE, AV_PIX_FMT_BAYER_GRBG16LE,
    AV_PIX_FMT_BAYER_GRBG8, AV_PIX_FMT_BAYER_RGGB16BE, AV_PIX_FMT_BAYER_RGGB16LE,
    AV_PIX_FMT_BAYER_RGGB8, AV_PIX_FMT_BGR0, AV_PIX_FMT_BGR24, AV_PIX_FMT_BGR4,
    AV_PIX_FMT_BGR444BE, AV_PIX_FMT_BGR444LE, AV_PIX_FMT_BGR48BE, AV_PIX_FMT_BGR48LE,
    AV_PIX_FMT_BGR4_BYTE, AV_PIX_FMT_BGR555BE, AV_PIX_FMT_BGR555LE, AV_PIX_FMT_BGR565BE,
    AV_PIX_FMT_BGR565LE, AV_PIX_FMT_BGR8, AV_PIX_FMT_BGRA, AV_PIX_FMT_BGRA64BE,
    AV_PIX_FMT_BGRA64LE, AV_PIX_FMT_CUDA, AV_PIX_FMT_D3D11, AV_PIX_FMT_D3D11VA_VLD,
    AV_PIX_FMT_D3D12, AV_PIX_FMT_DRM_PRIME, AV_PIX_FMT_DXVA2_VLD, AV_PIX_FMT_GBR24P,
    AV_PIX_FMT_GBRAP, AV_PIX_FMT_GBRAP10BE, AV_PIX_FMT_GBRAP10LE, AV_PIX_FMT_GBRAP12BE,
    AV_PIX_FMT_GBRAP12LE, AV_PIX_FMT_GBRAP14BE, AV_PIX_FMT_GBRAP14LE, AV_PIX_FMT_GBRAP16BE,
    AV_PIX_FMT_GBRAP16LE, AV_PIX_FMT_GBRAP32BE, AV_PIX_FMT_GBRAP32LE, AV_PIX_FMT_GBRAPF16BE,
    AV_PIX_FMT_GBRAPF16LE, AV_PIX_FMT_GBRAPF32BE, AV_PIX_FMT_GBRAPF32LE, AV_PIX_FMT_GBRP,
    AV_PIX_FMT_GBRP10BE, AV_PIX_FMT_GBRP10LE, AV_PIX_FMT_GBRP10MSBBE, AV_PIX_FMT_GBRP10MSBLE,
    AV_PIX_FMT_GBRP12BE, AV_PIX_FMT_GBRP12LE, AV_PIX_FMT_GBRP12MSBBE, AV_PIX_FMT_GBRP12MSBLE,
    AV_PIX_FMT_GBRP14BE, AV_PIX_FMT_GBRP14LE, AV_PIX_FMT_GBRP16BE, AV_PIX_FMT_GBRP16LE,
    AV_PIX_FMT_GBRP9BE, AV_PIX_FMT_GBRP9LE, AV_PIX_FMT_GBRPF16BE, AV_PIX_FMT_GBRPF16LE,
    AV_PIX_FMT_GBRPF32BE, AV_PIX_FMT_GBRPF32LE, AV_PIX_FMT_GRAY10BE, AV_PIX_FMT_GRAY10LE,
    AV_PIX_FMT_GRAY12BE, AV_PIX_FMT_GRAY12LE, AV_PIX_FMT_GRAY14BE, AV_PIX_FMT_GRAY14LE,
    AV_PIX_FMT_GRAY16BE, AV_PIX_FMT_GRAY16LE, AV_PIX_FMT_GRAY32BE, AV_PIX_FMT_GRAY32LE,
    AV_PIX_FMT_GRAY8, AV_PIX_FMT_GRAY8A, AV_PIX_FMT_GRAY9BE, AV_PIX_FMT_GRAY9LE,
    AV_PIX_FMT_GRAYF16BE, AV_PIX_FMT_GRAYF16LE, AV_PIX_FMT_GRAYF32BE, AV_PIX_FMT_GRAYF32LE,
    AV_PIX_FMT_MEDIACODEC, AV_PIX_FMT_MMAL, AV_PIX_FMT_MONOBLACK, AV_PIX_FMT_MONOWHITE,
    AV_PIX_FMT_NB, AV_PIX_FMT_NONE, AV_PIX_FMT_NV12, AV_PIX_FMT_NV16, AV_PIX_FMT_NV20BE,
    AV_PIX_FMT_NV20LE, AV_PIX_FMT_NV21, AV_PIX_FMT_NV24, AV_PIX_FMT_NV42, AV_PIX_FMT_OHCODEC,
    AV_PIX_FMT_OPENCL, AV_PIX_FMT_P010BE, AV_PIX_FMT_P010LE, AV_PIX_FMT_P012BE, AV_PIX_FMT_P012LE,
    AV_PIX_FMT_P016BE, AV_PIX_FMT_P016LE, AV_PIX_FMT_P210BE, AV_PIX_FMT_P210LE, AV_PIX_FMT_P212BE,
    AV_PIX_FMT_P212LE, AV_PIX_FMT_P216BE, AV_PIX_FMT_P216LE, AV_PIX_FMT_P410BE, AV_PIX_FMT_P410LE,
    AV_PIX_FMT_P412BE, AV_PIX_FMT_P412LE, AV_PIX_FMT_P416BE, AV_PIX_FMT_P416LE, AV_PIX_FMT_PAL8,
    AV_PIX_FMT_QSV, AV_PIX_FMT_RGB0, AV_PIX_FMT_RGB24, AV_PIX_FMT_RGB4, AV_PIX_FMT_RGB444BE,
    AV_PIX_FMT_RGB444LE, AV_PIX_FMT_RGB48BE, AV_PIX_FMT_RGB48LE, AV_PIX_FMT_RGB4_BYTE,
    AV_PIX_FMT_RGB555BE, AV_PIX_FMT_RGB555LE, AV_PIX_FMT_RGB565BE, AV_PIX_FMT_RGB565LE,
    AV_PIX_FMT_RGB8, AV_PIX_FMT_RGB96BE, AV_PIX_FMT_RGB96LE, AV_PIX_FMT_RGBA, AV_PIX_FMT_RGBA128BE,
    AV_PIX_FMT_RGBA128LE, AV_PIX_FMT_RGBA64BE, AV_PIX_FMT_RGBA64LE, AV_PIX_FMT_RGBAF16BE,
    AV_PIX_FMT_RGBAF16LE, AV_PIX_FMT_RGBAF32BE, AV_PIX_FMT_RGBAF32LE, AV_PIX_FMT_RGBF16BE,
    AV_PIX_FMT_RGBF16LE, AV_PIX_FMT_RGBF32BE, AV_PIX_FMT_RGBF32LE, AV_PIX_FMT_UYVA,
    AV_PIX_FMT_UYVY422, AV_PIX_FMT_UYYVYY411, AV_PIX_FMT_V30XBE, AV_PIX_FMT_V30XLE,
    AV_PIX_FMT_VAAPI, AV_PIX_FMT_VDPAU, AV_PIX_FMT_VIDEOTOOLBOX, AV_PIX_FMT_VULKAN,
    AV_PIX_FMT_VUYA, AV_PIX_FMT_VUYX, AV_PIX_FMT_VYU444, AV_PIX_FMT_X2BGR10BE,
    AV_PIX_FMT_X2BGR10LE, AV_PIX_FMT_X2RGB10BE, AV_PIX_FMT_X2RGB10LE, AV_PIX_FMT_XV30BE,
    AV_PIX_FMT_XV30LE, AV_PIX_FMT_XV36BE, AV_PIX_FMT_XV36LE, AV_PIX_FMT_XV48BE, AV_PIX_FMT_XV48LE,
    AV_PIX_FMT_XYZ12BE, AV_PIX_FMT_XYZ12LE, AV_PIX_FMT_Y210BE, AV_PIX_FMT_Y210LE,
    AV_PIX_FMT_Y212BE, AV_PIX_FMT_Y212LE, AV_PIX_FMT_Y216BE, AV_PIX_FMT_Y216LE, AV_PIX_FMT_Y400A,
    AV_PIX_FMT_YA16BE, AV_PIX_FMT_YA16LE, AV_PIX_FMT_YA8, AV_PIX_FMT_YAF16BE, AV_PIX_FMT_YAF16LE,
    AV_PIX_FMT_YAF32BE, AV_PIX_FMT_YAF32LE, AV_PIX_FMT_YUV410P, AV_PIX_FMT_YUV411P,
    AV_PIX_FMT_YUV420P, AV_PIX_FMT_YUV420P10BE, AV_PIX_FMT_YUV420P10LE, AV_PIX_FMT_YUV420P12BE,
    AV_PIX_FMT_YUV420P12LE, AV_PIX_FMT_YUV420P14BE, AV_PIX_FMT_YUV420P14LE, AV_PIX_FMT_YUV420P16BE,
    AV_PIX_FMT_YUV420P16LE, AV_PIX_FMT_YUV420P9BE, AV_PIX_FMT_YUV420P9LE, AV_PIX_FMT_YUV422P,
    AV_PIX_FMT_YUV422P10BE, AV_PIX_FMT_YUV422P10LE, AV_PIX_FMT_YUV422P12BE, AV_PIX_FMT_YUV422P12LE,
    AV_PIX_FMT_YUV422P14BE, AV_PIX_FMT_YUV422P14LE, AV_PIX_FMT_YUV422P16BE, AV_PIX_FMT_YUV422P16LE,
    AV_PIX_FMT_YUV422P9BE, AV_PIX_FMT_YUV422P9LE, AV_PIX_FMT_YUV440P, AV_PIX_FMT_YUV440P10BE,
    AV_PIX_FMT_YUV440P10LE, AV_PIX_FMT_YUV440P12BE, AV_PIX_FMT_YUV440P12LE, AV_PIX_FMT_YUV444P,
    AV_PIX_FMT_YUV444P10BE, AV_PIX_FMT_YUV444P10LE, AV_PIX_FMT_YUV444P10MSBBE,
    AV_PIX_FMT_YUV444P10MSBLE, AV_PIX_FMT_YUV444P12BE, AV_PIX_FMT_YUV444P12LE,
    AV_PIX_FMT_YUV444P12MSBBE, AV_PIX_FMT_YUV444P12MSBLE, AV_PIX_FMT_YUV444P14BE,
    AV_PIX_FMT_YUV444P14LE, AV_PIX_FMT_YUV444P16BE, AV_PIX_FMT_YUV444P16LE, AV_PIX_FMT_YUV444P9BE,
    AV_PIX_FMT_YUV444P9LE, AV_PIX_FMT_YUVA420P, AV_PIX_FMT_YUVA420P10BE, AV_PIX_FMT_YUVA420P10LE,
    AV_PIX_FMT_YUVA420P16BE, AV_PIX_FMT_YUVA420P16LE, AV_PIX_FMT_YUVA420P9BE,
    AV_PIX_FMT_YUVA420P9LE, AV_PIX_FMT_YUVA422P, AV_PIX_FMT_YUVA422P10BE, AV_PIX_FMT_YUVA422P10LE,
    AV_PIX_FMT_YUVA422P12BE, AV_PIX_FMT_YUVA422P12LE, AV_PIX_FMT_YUVA422P16BE,
    AV_PIX_FMT_YUVA422P16LE, AV_PIX_FMT_YUVA422P9BE, AV_PIX_FMT_YUVA422P9LE, AV_PIX_FMT_YUVA444P,
    AV_PIX_FMT_YUVA444P10BE, AV_PIX_FMT_YUVA444P10LE, AV_PIX_FMT_YUVA444P12BE,
    AV_PIX_FMT_YUVA444P12LE, AV_PIX_FMT_YUVA444P16BE, AV_PIX_FMT_YUVA444P16LE,
    AV_PIX_FMT_YUVA444P9BE, AV_PIX_FMT_YUVA444P9LE, AV_PIX_FMT_YUVJ411P, AV_PIX_FMT_YUVJ420P,
    AV_PIX_FMT_YUVJ422P, AV_PIX_FMT_YUVJ440P, AV_PIX_FMT_YUVJ444P, AV_PIX_FMT_YUYV422,
    AV_PIX_FMT_YVYU422,
};
pub use self::rational_h::AVRational;
pub use self::samplefmt_h::{
    AVSampleFormat, AV_SAMPLE_FMT_DBL, AV_SAMPLE_FMT_DBLP, AV_SAMPLE_FMT_FLT, AV_SAMPLE_FMT_FLTP,
    AV_SAMPLE_FMT_NB, AV_SAMPLE_FMT_NONE, AV_SAMPLE_FMT_S16, AV_SAMPLE_FMT_S16P, AV_SAMPLE_FMT_S32,
    AV_SAMPLE_FMT_S32P, AV_SAMPLE_FMT_S64, AV_SAMPLE_FMT_S64P, AV_SAMPLE_FMT_U8, AV_SAMPLE_FMT_U8P,
};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdlib_h::{calloc, free, malloc};
use self::string_h::{memcpy, memset, strcmp, strlen};
use self::strings_h::strcasecmp;
pub use self::types_h::{__int64_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t};
pub use self::x264_h::{X264_CSP_NONE, X264_CSP_VFLIP, X264_LOG_ERROR, X264_LOG_WARNING};
pub use self::x264cli_h::{get_filename_extension, hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:9"]
pub struct lavf_hnd_t {
    pub lavf: *mut AVFormatContext,
    pub lavc: *mut AVCodecContext,
    pub frame: *mut AVFrame,
    pub pkt: *mut AVPacket,
    pub stream_id: ::core::ffi::c_int,
    pub next_frame: ::core::ffi::c_int,
    pub vfr_input: ::core::ffi::c_int,
    pub first_pic: *mut cli_pic_t,
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn handle_jpeg(
    mut csp: ::core::ffi::c_int,
    mut fullrange: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match csp {
        12 => {
            *fullrange = 1 as ::core::ffi::c_int;
            return AV_PIX_FMT_YUV420P as ::core::ffi::c_int;
        }
        13 => {
            *fullrange = 1 as ::core::ffi::c_int;
            return AV_PIX_FMT_YUV422P as ::core::ffi::c_int;
        }
        14 => {
            *fullrange = 1 as ::core::ffi::c_int;
            return AV_PIX_FMT_YUV444P as ::core::ffi::c_int;
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
    if avcodec_parameters_to_context(c, (*stream).codecpar) < 0 as ::core::ffi::c_int {
        avcodec_free_context(&mut c);
        return 0 as *mut AVCodecContext;
    }
    return c;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn read_frame_internal(
    mut p_pic: *mut cli_pic_t,
    mut h: *mut lavf_hnd_t,
    mut i_frame: ::core::ffi::c_int,
    mut info: *mut video_info_t,
) -> ::core::ffi::c_int {
    if !(*h).first_pic.is_null() && info.is_null() {
        if i_frame == 0 {
            let mut t: cli_image_t = (*p_pic).img;
            (*p_pic).img = (*(*h).first_pic).img;
            (*(*h).first_pic).img = t;
            (*p_pic).pts = (*(*h).first_pic).pts;
        }
        lavf_input.picture_clean.expect("non-null function pointer")((*h).first_pic, h as hnd_t);
        free((*h).first_pic as *mut ::core::ffi::c_void);
        (*h).first_pic = 0 as *mut cli_pic_t;
        if i_frame == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut pkt: *mut AVPacket = (*h).pkt;
    while i_frame >= (*h).next_frame {
        let mut ret: ::core::ffi::c_int = 0;
        loop {
            ret = avcodec_receive_frame((*h).lavc, (*h).frame);
            if !(ret != 0) {
                break;
            }
            if ret == -(11 as ::core::ffi::c_int) {
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
                return -(1 as ::core::ffi::c_int);
            }
            if ret != 0 {
                x264_cli_log(
                    b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_WARNING,
                    b"video decoding failed on frame %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*h).next_frame,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        (*h).next_frame += 1;
    }
    memcpy(
        (*p_pic).img.stride.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*(*h).frame).linesize.as_mut_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_int; 4]>() as size_t,
    );
    memcpy(
        (*p_pic).img.plane.as_mut_ptr() as *mut ::core::ffi::c_void,
        (*(*h).frame).data.as_mut_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[*mut uint8_t; 4]>() as size_t,
    );
    let mut is_fullrange: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*p_pic).img.width = (*(*h).lavc).width;
    (*p_pic).img.height = (*(*h).lavc).height;
    (*p_pic).img.csp = handle_jpeg(
        (*(*h).lavc).pix_fmt as ::core::ffi::c_int,
        &mut is_fullrange,
    ) | X264_CSP_OTHER;
    if !info.is_null() {
        (*info).fullrange = is_fullrange;
        (*info).interlaced =
            ((*(*h).frame).flags & AV_FRAME_FLAG_INTERLACED != 0) as ::core::ffi::c_int;
        (*info).tff =
            ((*(*h).frame).flags & AV_FRAME_FLAG_TOP_FIELD_FIRST != 0) as ::core::ffi::c_int;
    }
    if (*h).vfr_input != 0 {
        (*p_pic).duration = 0 as int64_t;
        (*p_pic).pts = (*p_pic).duration;
        if (*(*h).frame).pts != AV_NOPTS_VALUE {
            (*p_pic).pts = (*(*h).frame).pts;
        } else if (*(*h).frame).pkt_dts != AV_NOPTS_VALUE {
            (*p_pic).pts = (*(*h).frame).pkt_dts;
        } else if !info.is_null() {
            (*info).vfr = 0 as ::core::ffi::c_int;
            (*h).vfr_input = (*info).vfr;
            return 0 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> ::core::ffi::c_int {
    let mut h: *mut lavf_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<lavf_hnd_t>() as size_t) as *mut lavf_hnd_t;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if strcmp(
        psz_filename,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        psz_filename =
            b"pipe:\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    (*h).frame = av_frame_alloc();
    if (*h).frame.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).pkt = av_packet_alloc();
    if (*h).pkt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut options: *mut AVDictionary = 0 as *mut AVDictionary;
    if !(*opt).resolution.is_null() {
        av_dict_set(
            &mut options,
            b"video_size\0" as *const u8 as *const ::core::ffi::c_char,
            (*opt).resolution,
            0 as ::core::ffi::c_int,
        );
        let mut csp: *const ::core::ffi::c_char = if !(*opt).colorspace.is_null() {
            (*opt).colorspace as *const ::core::ffi::c_char
        } else {
            av_get_pix_fmt_name(AV_PIX_FMT_YUV420P)
        };
        av_dict_set(
            &mut options,
            b"pixel_format\0" as *const u8 as *const ::core::ffi::c_char,
            csp,
            0 as ::core::ffi::c_int,
        );
    }
    let mut format: *mut AVInputFormat = 0 as *mut AVInputFormat;
    if !(*opt).format.is_null() {
        format = av_find_input_format((*opt).format) as *mut AVInputFormat;
        if format.is_null() {
            x264_cli_log(
                b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"unknown file format: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*opt).format,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if avformat_open_input(&mut (*h).lavf, psz_filename, format, &mut options) != 0 {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"could not open input file\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !options.is_null() {
        av_dict_free(&mut options);
    }
    if avformat_find_stream_info((*h).lavf, 0 as *mut *mut AVDictionary) < 0 as ::core::ffi::c_int {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"could not find input stream info\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*(*h).lavf).nb_streams
        && (*(**(*(*h).lavf).streams.offset(i as isize)).codecpar).codec_type as ::core::ffi::c_int
            != AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        i += 1;
    }
    if i as ::core::ffi::c_uint == (*(*h).lavf).nb_streams {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"could not find video stream\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).stream_id = i;
    (*h).next_frame = 0 as ::core::ffi::c_int;
    (*h).lavc = codec_from_stream(*(*(*h).lavf).streams.offset(i as isize));
    if (*h).lavc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*info).fps_num = (**(*(*h).lavf).streams.offset(i as isize))
        .avg_frame_rate
        .num as uint32_t;
    (*info).fps_den = (**(*(*h).lavf).streams.offset(i as isize))
        .avg_frame_rate
        .den as uint32_t;
    (*info).timebase_num = (**(*(*h).lavf).streams.offset(i as isize)).time_base.num as uint32_t;
    (*info).timebase_den = (**(*(*h).lavf).streams.offset(i as isize)).time_base.den as uint32_t;
    (*info).thread_safe = 0 as ::core::ffi::c_int;
    (*h).vfr_input = (*info).vfr;
    if avcodec_open2(
        (*h).lavc,
        avcodec_find_decoder((*(*h).lavc).codec_id),
        0 as *mut *mut AVDictionary,
    ) != 0
    {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"could not find decoder for video stream\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).first_pic = malloc(::core::mem::size_of::<cli_pic_t>() as size_t) as *mut cli_pic_t;
    if (*h).first_pic.is_null()
        || lavf_input.picture_alloc.expect("non-null function pointer")(
            (*h).first_pic,
            h as hnd_t,
            0x4000 as ::core::ffi::c_int,
            (*info).width,
            (*info).height,
        ) != 0
    {
        x264_cli_log(
            b"lavf\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if read_frame_internal((*h).first_pic, h, 0 as ::core::ffi::c_int, info) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*info).width = (*(*h).lavc).width;
    (*info).height = (*(*h).lavc).height;
    (*info).csp = (*(*h).first_pic).img.csp;
    (*info).num_frames =
        (**(*(*h).lavf).streams.offset(i as isize)).nb_frames as ::core::ffi::c_int;
    (*info).sar_height = (*(*h).lavc).sample_aspect_ratio.den as uint32_t;
    (*info).sar_width = (*(*h).lavc).sample_aspect_ratio.num as uint32_t;
    (*info).fullrange |= ((*(*h).lavc).color_range as ::core::ffi::c_uint
        == AVCOL_RANGE_JPEG as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    if strcasecmp(
        get_filename_extension(psz_filename),
        b"avs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        && ((*(*h).lavc).pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_BGRA as ::core::ffi::c_int
            || (*(*h).lavc).pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_BGR24 as ::core::ffi::c_int)
    {
        (*info).csp |= X264_CSP_VFLIP;
    }
    *p_handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "250:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if x264_cli_pic_alloc(pic, X264_CSP_NONE, width, height) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic).img.csp = csp;
    (*pic).img.planes = 4 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "259:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return read_frame_internal(
        pic,
        handle as *mut lavf_hnd_t,
        i_frame,
        0 as *mut video_info_t,
    );
}
#[c2rust::src_loc = "264:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> ::core::ffi::c_int {
    let mut h: *mut lavf_hnd_t = handle as *mut lavf_hnd_t;
    avcodec_free_context(&mut (*h).lavc);
    avformat_close_input(&mut (*h).lavf);
    av_packet_free(&mut (*h).pkt);
    av_frame_free(&mut (*h).frame);
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "280:19"]
pub static mut lavf_input: cli_input_t = unsafe {
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
            release_frame: None,
            picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
            close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int),
        };
        init
    }
};
