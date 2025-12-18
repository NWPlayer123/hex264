use core::ffi::{c_double, c_float, c_int};

use crate::stdint_intn_h::{int32_t, int8_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
use crate::tables_h::vlc_t;
use crate::x264_h::x264_level_t;
#[no_mangle]
#[c2rust::src_loc = "29:20"]
static mut x264_levels: [x264_level_t; 21] = [
    {
        let mut init = x264_level_t {
            level_idc: 10,
            mbps: 1485 as int32_t,
            frame_size: 99 as int32_t,
            dpb: 396 as int32_t,
            bitrate: 64 as int32_t,
            cpb: 175 as int32_t,
            mv_range: 64 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 9,
            mbps: 1485 as int32_t,
            frame_size: 99 as int32_t,
            dpb: 396 as int32_t,
            bitrate: 128 as int32_t,
            cpb: 350 as int32_t,
            mv_range: 64 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 11,
            mbps: 3000 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 900 as int32_t,
            bitrate: 192 as int32_t,
            cpb: 500 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 12,
            mbps: 6000 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 384 as int32_t,
            cpb: 1000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 13,
            mbps: 11880 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 768 as int32_t,
            cpb: 2000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 20,
            mbps: 11880 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 2000 as int32_t,
            cpb: 2000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 21,
            mbps: 19800 as int32_t,
            frame_size: 792 as int32_t,
            dpb: 4752 as int32_t,
            bitrate: 4000 as int32_t,
            cpb: 4000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 22,
            mbps: 20250 as int32_t,
            frame_size: 1620 as int32_t,
            dpb: 8100 as int32_t,
            bitrate: 4000 as int32_t,
            cpb: 4000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 64,
            slice_rate: 0,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 30,
            mbps: 40500 as int32_t,
            frame_size: 1620 as int32_t,
            dpb: 8100 as int32_t,
            bitrate: 10000 as int32_t,
            cpb: 10000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 32,
            slice_rate: 22,
            mincr: 2,
            bipred8x8: 0,
            direct8x8: 1,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 31,
            mbps: 108000 as int32_t,
            frame_size: 3600 as int32_t,
            dpb: 18000 as int32_t,
            bitrate: 14000 as int32_t,
            cpb: 14000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 60,
            mincr: 4,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 32,
            mbps: 216000 as int32_t,
            frame_size: 5120 as int32_t,
            dpb: 20480 as int32_t,
            bitrate: 20000 as int32_t,
            cpb: 20000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 60,
            mincr: 4,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 40,
            mbps: 245760 as int32_t,
            frame_size: 8192 as int32_t,
            dpb: 32768 as int32_t,
            bitrate: 20000 as int32_t,
            cpb: 25000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 60,
            mincr: 4,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 41,
            mbps: 245760 as int32_t,
            frame_size: 8192 as int32_t,
            dpb: 32768 as int32_t,
            bitrate: 50000 as int32_t,
            cpb: 62500 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 0,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 42,
            mbps: 522240 as int32_t,
            frame_size: 8704 as int32_t,
            dpb: 34816 as int32_t,
            bitrate: 50000 as int32_t,
            cpb: 62500 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 50,
            mbps: 589824 as int32_t,
            frame_size: 22080 as int32_t,
            dpb: 110400 as int32_t,
            bitrate: 135000 as int32_t,
            cpb: 135000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 51,
            mbps: 983040 as int32_t,
            frame_size: 36864 as int32_t,
            dpb: 184320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 52,
            mbps: 2073600 as int32_t,
            frame_size: 36864 as int32_t,
            dpb: 184320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 60,
            mbps: 4177920 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 61,
            mbps: 8355840 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 480000 as int32_t,
            cpb: 480000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 62,
            mbps: 16711680 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 800000 as int32_t,
            cpb: 800000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16,
            slice_rate: 24,
            mincr: 2,
            bipred8x8: 1,
            direct8x8: 1,
            frame_only: 1,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 0,
            mbps: 0,
            frame_size: 0,
            dpb: 0,
            bitrate: 0,
            cpb: 0,
            mv_range: 0,
            mvs_per_2mb: 0,
            slice_rate: 0,
            mincr: 0,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 0,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "58:15"]
static mut x264_exp2_lut: [uint8_t; 64] = [
    0, 3, 6, 8, 11, 14, 17, 20, 23, 26, 29, 32, 36, 39, 42, 45, 48, 52, 55, 58, 62, 65, 69, 72, 76,
    80, 83, 87, 91, 94, 98, 102, 106, 110, 114, 118, 122, 126, 130, 135, 139, 143, 147, 152, 156,
    161, 165, 170, 175, 179, 184, 189, 194, 198, 203, 208, 214, 219, 224, 229, 234, 240, 245, 250,
];
#[no_mangle]
#[c2rust::src_loc = "66:13"]
static mut x264_log2_lut: [c_float; 128] = [
    0.00000f64 as c_float,
    0.01123f64 as c_float,
    0.02237f64 as c_float,
    0.03342f64 as c_float,
    0.04439f64 as c_float,
    0.05528f64 as c_float,
    0.06609f64 as c_float,
    0.07682f64 as c_float,
    0.08746f64 as c_float,
    0.09803f64 as c_float,
    0.10852f64 as c_float,
    0.11894f64 as c_float,
    0.12928f64 as c_float,
    0.13955f64 as c_float,
    0.14975f64 as c_float,
    0.15987f64 as c_float,
    0.16993f64 as c_float,
    0.17991f64 as c_float,
    0.18982f64 as c_float,
    0.19967f64 as c_float,
    0.20945f64 as c_float,
    0.21917f64 as c_float,
    0.22882f64 as c_float,
    0.23840f64 as c_float,
    0.24793f64 as c_float,
    0.25739f64 as c_float,
    0.26679f64 as c_float,
    0.27612f64 as c_float,
    0.28540f64 as c_float,
    0.29462f64 as c_float,
    0.30378f64 as c_float,
    0.31288f64 as c_float,
    0.32193f64 as c_float,
    0.33092f64 as c_float,
    0.33985f64 as c_float,
    0.34873f64 as c_float,
    0.35755f64 as c_float,
    0.36632f64 as c_float,
    0.37504f64 as c_float,
    0.38370f64 as c_float,
    0.39232f64 as c_float,
    0.40088f64 as c_float,
    0.40939f64 as c_float,
    0.41785f64 as c_float,
    0.42626f64 as c_float,
    0.43463f64 as c_float,
    0.44294f64 as c_float,
    0.45121f64 as c_float,
    0.45943f64 as c_float,
    0.46761f64 as c_float,
    0.47573f64 as c_float,
    0.48382f64 as c_float,
    0.49185f64 as c_float,
    0.49985f64 as c_float,
    0.50779f64 as c_float,
    0.51570f64 as c_float,
    0.52356f64 as c_float,
    0.53138f64 as c_float,
    0.53916f64 as c_float,
    0.54689f64 as c_float,
    0.55459f64 as c_float,
    0.56224f64 as c_float,
    0.56986f64 as c_float,
    0.57743f64 as c_float,
    0.58496f64 as c_float,
    0.59246f64 as c_float,
    0.59991f64 as c_float,
    0.60733f64 as c_float,
    0.61471f64 as c_float,
    0.62205f64 as c_float,
    0.62936f64 as c_float,
    0.63662f64 as c_float,
    0.64386f64 as c_float,
    0.65105f64 as c_float,
    0.65821f64 as c_float,
    0.66534f64 as c_float,
    0.67243f64 as c_float,
    0.67948f64 as c_float,
    0.68650f64 as c_float,
    0.69349f64 as c_float,
    0.70044f64 as c_float,
    0.70736f64 as c_float,
    0.71425f64 as c_float,
    0.72110f64 as c_float,
    0.72792f64 as c_float,
    0.73471f64 as c_float,
    0.74147f64 as c_float,
    0.74819f64 as c_float,
    0.75489f64 as c_float,
    0.76155f64 as c_float,
    0.76818f64 as c_float,
    0.77479f64 as c_float,
    0.78136f64 as c_float,
    0.78790f64 as c_float,
    0.79442f64 as c_float,
    0.80090f64 as c_float,
    0.80735f64 as c_float,
    0.81378f64 as c_float,
    0.82018f64 as c_float,
    0.82655f64 as c_float,
    0.83289f64 as c_float,
    0.83920f64 as c_float,
    0.84549f64 as c_float,
    0.85175f64 as c_float,
    0.85798f64 as c_float,
    0.86419f64 as c_float,
    0.87036f64 as c_float,
    0.87652f64 as c_float,
    0.88264f64 as c_float,
    0.88874f64 as c_float,
    0.89482f64 as c_float,
    0.90087f64 as c_float,
    0.90689f64 as c_float,
    0.91289f64 as c_float,
    0.91886f64 as c_float,
    0.92481f64 as c_float,
    0.93074f64 as c_float,
    0.93664f64 as c_float,
    0.94251f64 as c_float,
    0.94837f64 as c_float,
    0.95420f64 as c_float,
    0.96000f64 as c_float,
    0.96578f64 as c_float,
    0.97154f64 as c_float,
    0.97728f64 as c_float,
    0.98299f64 as c_float,
    0.98868f64 as c_float,
    0.99435f64 as c_float,
];
#[no_mangle]
#[c2rust::src_loc = "87:13"]
static mut x264_log2_lz_lut: [c_float; 32] = [
    31 as c_float,
    30 as c_float,
    29 as c_float,
    28 as c_float,
    27 as c_float,
    26 as c_float,
    25 as c_float,
    24 as c_float,
    23 as c_float,
    22 as c_float,
    21 as c_float,
    20 as c_float,
    19 as c_float,
    18 as c_float,
    17 as c_float,
    16 as c_float,
    15 as c_float,
    14 as c_float,
    13 as c_float,
    12 as c_float,
    11 as c_float,
    10 as c_float,
    9 as c_float,
    8 as c_float,
    7 as c_float,
    6 as c_float,
    5 as c_float,
    4 as c_float,
    3 as c_float,
    2 as c_float,
    1 as c_float,
    0 as c_float,
];
#[no_mangle]
#[c2rust::src_loc = "97:16"]
static mut x264_lambda_tab: [uint16_t; 82] = [
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    1 as uint16_t,
    2 as uint16_t,
    2 as uint16_t,
    2 as uint16_t,
    2 as uint16_t,
    3 as uint16_t,
    3 as uint16_t,
    3 as uint16_t,
    4 as uint16_t,
    4 as uint16_t,
    4 as uint16_t,
    5 as uint16_t,
    6 as uint16_t,
    6 as uint16_t,
    7 as uint16_t,
    8 as uint16_t,
    9 as uint16_t,
    10 as uint16_t,
    11 as uint16_t,
    13 as uint16_t,
    14 as uint16_t,
    16 as uint16_t,
    18 as uint16_t,
    20 as uint16_t,
    23 as uint16_t,
    25 as uint16_t,
    29 as uint16_t,
    32 as uint16_t,
    36 as uint16_t,
    40 as uint16_t,
    45 as uint16_t,
    51 as uint16_t,
    57 as uint16_t,
    64 as uint16_t,
    72 as uint16_t,
    81 as uint16_t,
    91 as uint16_t,
    102 as uint16_t,
    114 as uint16_t,
    128 as uint16_t,
    144 as uint16_t,
    161 as uint16_t,
    181 as uint16_t,
    203 as uint16_t,
    228 as uint16_t,
    256 as uint16_t,
    287 as uint16_t,
    323 as uint16_t,
    362 as uint16_t,
    406 as uint16_t,
    456 as uint16_t,
    512 as uint16_t,
    575 as uint16_t,
    645 as uint16_t,
    724 as uint16_t,
    813 as uint16_t,
    912 as uint16_t,
    1024 as uint16_t,
    1149 as uint16_t,
    1290 as uint16_t,
    1448 as uint16_t,
    1625 as uint16_t,
    1825 as uint16_t,
    2048 as uint16_t,
    2299 as uint16_t,
    2580 as uint16_t,
    2896 as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "114:11"]
static mut x264_lambda2_tab: [c_int; 82] = [
    14, 18, 22, 28, 36, 45, 57, 72, 91, 115, 145, 182, 230, 290, 365, 460, 580, 731, 921, 1161,
    1462, 1843, 2322, 2925, 3686, 4644, 5851, 7372, 9289, 11703, 14745, 18578, 23407, 29491, 37156,
    46814, 58982, 74313, 93628, 117964, 148626, 187257, 235929, 297252, 374514, 471859, 594505,
    749029, 943718, 1189010, 1498059, 1887436, 2378021, 2996119, 3774873, 4756042, 5992238,
    7549747, 9512085, 11984476, 15099494, 19024170, 23968953, 30198988, 38048341, 47937906,
    60397977, 76096683, 95875813, 120795955, 134217727, 134217727, 134217727, 134217727, 134217727,
    134217727, 134217727, 134217727, 134217727, 134217727, 134217727, 134217727,
];
#[no_mangle]
#[c2rust::src_loc = "131:11"]
static mut x264_trellis_lambda2_tab: [[c_int; 82]; 2] = [
    [
        46, 58, 73, 92, 117, 147, 185, 233, 294, 370, 466, 587, 740, 932, 1174, 1480, 1864, 2349,
        2959, 3728, 4697, 5918, 7457, 9395, 11837, 14914, 18790, 23674, 29828, 37581, 47349, 59656,
        75163, 94699, 119313, 150326, 189399, 238627, 300652, 378798, 477255, 601304, 757596,
        954511, 1202608, 1515192, 1909022, 2405217, 3030384, 3818045, 4810435, 6060769, 7636091,
        9620872, 12121539, 15272182, 19241743, 24243077, 30544363, 38483486, 48486154, 61088726,
        76966972, 96972308, 122177453, 134217727, 134217727, 134217727, 134217727, 134217727,
        134217727, 134217727, 134217727, 134217727, 134217727, 134217727, 0, 0, 0, 0, 0, 0,
    ],
    [
        27, 34, 43, 54, 68, 86, 108, 136, 172, 216, 273, 343, 433, 545, 687, 865, 1090, 1374, 1731,
        2180, 2747, 3461, 4361, 5494, 6922, 8721, 10988, 13844, 17442, 21976, 27688, 34885, 43953,
        55377, 69771, 87906, 110755, 139543, 175813, 221511, 279087, 351627, 443023, 558174,
        703255, 886046, 1116348, 1406511, 1772093, 2232697, 2813022, 3544186, 4465396, 5626046,
        7088374, 8930791, 11252092, 14176748, 17861583, 22504184, 28353495, 35723165, 45008368,
        56706990, 71446330, 90016736, 113413980, 134217727, 134217727, 134217727, 134217727,
        134217727, 134217727, 134217727, 134217727, 134217727, 134217727, 134217727, 134217727,
        134217727, 134217727, 134217727,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "168:16"]
static mut x264_chroma_lambda2_offset_tab: [uint16_t; 37] = [
    16 as uint16_t,
    20 as uint16_t,
    25 as uint16_t,
    32 as uint16_t,
    40 as uint16_t,
    50 as uint16_t,
    64 as uint16_t,
    80 as uint16_t,
    101 as uint16_t,
    128 as uint16_t,
    161 as uint16_t,
    203 as uint16_t,
    256 as uint16_t,
    322 as uint16_t,
    406 as uint16_t,
    512 as uint16_t,
    645 as uint16_t,
    812 as uint16_t,
    1024 as uint16_t,
    1290 as uint16_t,
    1625 as uint16_t,
    2048 as uint16_t,
    2580 as uint16_t,
    3250 as uint16_t,
    4096 as uint16_t,
    5160 as uint16_t,
    6501 as uint16_t,
    8192 as uint16_t,
    10321 as uint16_t,
    13003 as uint16_t,
    16384 as uint16_t,
    20642 as uint16_t,
    26007 as uint16_t,
    32768 as uint16_t,
    41285 as uint16_t,
    52015 as uint16_t,
    65535 as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "183:15"]
static mut x264_hpel_ref0: [uint8_t; 16] = [0, 1, 1, 1, 0, 1, 1, 1, 2, 3, 3, 3, 0, 1, 1, 1];
#[no_mangle]
#[c2rust::src_loc = "184:15"]
static mut x264_hpel_ref1: [uint8_t; 16] = [0, 0, 1, 0, 2, 2, 3, 2, 2, 2, 3, 2, 2, 2, 3, 2];
#[no_mangle]
#[c2rust::src_loc = "191:15"]
static mut x264_cqm_jvt4i: [uint8_t; 16] = [
    6, 13, 20, 28, 13, 20, 28, 32, 20, 28, 32, 37, 28, 32, 37, 42,
];
#[no_mangle]
#[c2rust::src_loc = "198:15"]
static mut x264_cqm_jvt4p: [uint8_t; 16] = [
    10, 14, 20, 24, 14, 20, 24, 27, 20, 24, 27, 30, 24, 27, 30, 34,
];
#[no_mangle]
#[c2rust::src_loc = "205:15"]
static mut x264_cqm_jvt8i: [uint8_t; 64] = [
    6, 10, 13, 16, 18, 23, 25, 27, 10, 11, 16, 18, 23, 25, 27, 29, 13, 16, 18, 23, 25, 27, 29, 31,
    16, 18, 23, 25, 27, 29, 31, 33, 18, 23, 25, 27, 29, 31, 33, 36, 23, 25, 27, 29, 31, 33, 36, 38,
    25, 27, 29, 31, 33, 36, 38, 40, 27, 29, 31, 33, 36, 38, 40, 42,
];
#[no_mangle]
#[c2rust::src_loc = "216:15"]
static mut x264_cqm_jvt8p: [uint8_t; 64] = [
    9, 13, 15, 17, 19, 21, 22, 24, 13, 13, 17, 19, 21, 22, 24, 25, 15, 17, 19, 21, 22, 24, 25, 27,
    17, 19, 21, 22, 24, 25, 27, 28, 19, 21, 22, 24, 25, 27, 28, 30, 21, 22, 24, 25, 27, 28, 30, 32,
    22, 24, 25, 27, 28, 30, 32, 33, 24, 25, 27, 28, 30, 32, 33, 35,
];
#[no_mangle]
#[c2rust::src_loc = "227:15"]
static mut x264_cqm_flat16: [uint8_t; 64] = [
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
];
#[no_mangle]
#[c2rust::src_loc = "238:23"]
static mut x264_cqm_jvt: [*const uint8_t; 8] = unsafe {
    [
        x264_cqm_jvt4i.as_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        x264_cqm_jvt8p.as_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        x264_cqm_jvt8p.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "247:15"]
static mut x264_cqm_avci50_4ic: [uint8_t; 16] = [
    16, 22, 28, 40, 22, 28, 40, 44, 28, 40, 44, 48, 40, 44, 48, 60,
];
#[no_mangle]
#[c2rust::src_loc = "256:15"]
static mut x264_cqm_avci50_p_8iy: [uint8_t; 64] = [
    16, 18, 19, 21, 24, 27, 30, 33, 18, 19, 21, 24, 27, 30, 33, 78, 19, 21, 24, 27, 30, 33, 78, 81,
    21, 24, 27, 30, 33, 78, 81, 84, 24, 27, 30, 33, 78, 81, 84, 87, 27, 30, 33, 78, 81, 84, 87, 90,
    30, 33, 78, 81, 84, 87, 90, 93, 33, 78, 81, 84, 87, 90, 93, 96,
];
#[no_mangle]
#[c2rust::src_loc = "269:15"]
static mut x264_cqm_avci50_1080i_8iy: [uint8_t; 64] = [
    16, 18, 19, 21, 27, 33, 81, 87, 18, 19, 21, 24, 30, 33, 81, 87, 19, 21, 24, 27, 30, 78, 84, 90,
    21, 24, 27, 30, 33, 78, 84, 90, 24, 27, 30, 33, 78, 81, 84, 90, 24, 27, 30, 33, 78, 81, 84, 93,
    27, 30, 33, 78, 78, 81, 87, 93, 30, 33, 33, 78, 81, 84, 87, 96,
];
#[no_mangle]
#[c2rust::src_loc = "282:15"]
static mut x264_cqm_avci100_720p_4ic: [uint8_t; 16] = [
    16, 21, 27, 34, 21, 27, 34, 41, 27, 34, 41, 46, 34, 41, 46, 54,
];
#[no_mangle]
#[c2rust::src_loc = "291:15"]
static mut x264_cqm_avci100_720p_8iy: [uint8_t; 64] = [
    16, 18, 19, 21, 22, 24, 26, 32, 18, 19, 19, 21, 22, 24, 26, 32, 19, 19, 21, 22, 22, 24, 26, 32,
    21, 21, 22, 22, 23, 24, 26, 34, 22, 22, 22, 23, 24, 25, 26, 34, 24, 24, 24, 24, 25, 26, 34, 36,
    26, 26, 26, 26, 26, 34, 36, 38, 32, 32, 32, 34, 34, 36, 38, 42,
];
#[no_mangle]
#[c2rust::src_loc = "304:15"]
static mut x264_cqm_avci100_1080_4ic: [uint8_t; 16] = [
    16, 20, 26, 32, 20, 26, 32, 38, 26, 32, 38, 44, 32, 38, 44, 50,
];
#[no_mangle]
#[c2rust::src_loc = "313:15"]
static mut x264_cqm_avci100_1080i_8iy: [uint8_t; 64] = [
    16, 19, 20, 23, 24, 26, 32, 42, 18, 19, 22, 24, 26, 32, 36, 42, 18, 20, 23, 24, 26, 32, 36, 63,
    19, 20, 23, 26, 32, 36, 42, 63, 20, 22, 24, 26, 32, 36, 59, 63, 22, 23, 24, 26, 32, 36, 59, 68,
    22, 23, 24, 26, 32, 42, 59, 68, 22, 23, 24, 26, 36, 42, 59, 72,
];
#[no_mangle]
#[c2rust::src_loc = "326:15"]
static mut x264_cqm_avci100_1080p_8iy: [uint8_t; 64] = [
    16, 18, 19, 20, 22, 23, 24, 26, 18, 19, 20, 22, 23, 24, 26, 32, 19, 20, 22, 23, 24, 26, 32, 36,
    20, 22, 23, 24, 26, 32, 36, 42, 22, 23, 24, 26, 32, 36, 42, 59, 23, 24, 26, 32, 36, 42, 59, 63,
    24, 26, 32, 36, 42, 59, 63, 68, 26, 32, 36, 42, 59, 63, 68, 72,
];
#[no_mangle]
#[c2rust::src_loc = "339:15"]
static mut x264_cqm_avci300_2160p_4iy: [uint8_t; 16] = [
    12, 16, 19, 20, 16, 19, 20, 24, 19, 20, 24, 33, 20, 24, 33, 39,
];
#[no_mangle]
#[c2rust::src_loc = "348:15"]
static mut x264_cqm_avci300_2160p_4ic: [uint8_t; 16] = [
    28, 39, 56, 67, 39, 56, 67, 77, 56, 67, 77, 104, 67, 77, 104, 133,
];
#[no_mangle]
#[c2rust::src_loc = "357:15"]
static mut x264_cqm_avci300_2160p_8iy: [uint8_t; 64] = [
    12, 14, 16, 17, 19, 20, 20, 24, 14, 16, 17, 19, 20, 20, 24, 30, 16, 17, 19, 20, 20, 24, 30, 42,
    17, 19, 20, 20, 24, 30, 42, 56, 19, 20, 20, 24, 30, 42, 56, 72, 20, 20, 24, 30, 42, 56, 72, 76,
    20, 24, 30, 42, 56, 72, 76, 80, 24, 30, 42, 56, 72, 76, 80, 84,
];
#[no_mangle]
#[c2rust::src_loc = "373:15"]
static mut x264_decimate_table4: [uint8_t; 16] = [3, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
#[c2rust::src_loc = "377:15"]
static mut x264_decimate_table8: [uint8_t; 64] = [
    3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
#[c2rust::src_loc = "397:16"]
static mut x264_dct8_weight_tab: [uint32_t; 64] = [0; 64];
#[no_mangle]
#[c2rust::src_loc = "413:16"]
static mut x264_dct4_weight_tab: [uint32_t; 16] = [0; 16];
#[no_mangle]
#[c2rust::src_loc = "425:16"]
static mut x264_dct4_weight2_tab: [uint32_t; 16] = [0; 16];
#[no_mangle]
#[c2rust::src_loc = "439:16"]
static mut x264_dct8_weight2_tab: [uint32_t; 64] = [0; 64];
#[no_mangle]
#[c2rust::src_loc = "456:14"]
static mut x264_cabac_context_init_I: [[int8_t; 2]; 1024] = [
    [20 as int8_t, -(15) as int8_t],
    [2 as int8_t, 54 as int8_t],
    [3 as int8_t, 74 as int8_t],
    [20 as int8_t, -(15) as int8_t],
    [2 as int8_t, 54 as int8_t],
    [3 as int8_t, 74 as int8_t],
    [-(28) as int8_t, 127 as int8_t],
    [-(23) as int8_t, 104 as int8_t],
    [-(6) as int8_t, 53 as int8_t],
    [-1 as int8_t, 54 as int8_t],
    [7 as int8_t, 51 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [0 as int8_t, 41 as int8_t],
    [0 as int8_t, 63 as int8_t],
    [0 as int8_t, 63 as int8_t],
    [0 as int8_t, 63 as int8_t],
    [-(9) as int8_t, 83 as int8_t],
    [4 as int8_t, 86 as int8_t],
    [0 as int8_t, 97 as int8_t],
    [-(7) as int8_t, 72 as int8_t],
    [13 as int8_t, 41 as int8_t],
    [3 as int8_t, 62 as int8_t],
    [0 as int8_t, 11 as int8_t],
    [1 as int8_t, 55 as int8_t],
    [0 as int8_t, 69 as int8_t],
    [-(17) as int8_t, 127 as int8_t],
    [-(13) as int8_t, 102 as int8_t],
    [0 as int8_t, 82 as int8_t],
    [-(7) as int8_t, 74 as int8_t],
    [-(21) as int8_t, 107 as int8_t],
    [-(27) as int8_t, 127 as int8_t],
    [-(31) as int8_t, 127 as int8_t],
    [-(24) as int8_t, 127 as int8_t],
    [-(18) as int8_t, 95 as int8_t],
    [-(27) as int8_t, 127 as int8_t],
    [-(21) as int8_t, 114 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-(17) as int8_t, 123 as int8_t],
    [-(12) as int8_t, 115 as int8_t],
    [-(16) as int8_t, 122 as int8_t],
    [-(11) as int8_t, 115 as int8_t],
    [-(12) as int8_t, 63 as int8_t],
    [-(2) as int8_t, 68 as int8_t],
    [-(15) as int8_t, 84 as int8_t],
    [-(13) as int8_t, 104 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-1 as int8_t, 74 as int8_t],
    [-(6) as int8_t, 97 as int8_t],
    [-(7) as int8_t, 91 as int8_t],
    [-(20) as int8_t, 127 as int8_t],
    [-(4) as int8_t, 56 as int8_t],
    [-(5) as int8_t, 82 as int8_t],
    [-(7) as int8_t, 76 as int8_t],
    [-(22) as int8_t, 125 as int8_t],
    [-(7) as int8_t, 93 as int8_t],
    [-(11) as int8_t, 87 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(5) as int8_t, 71 as int8_t],
    [-(4) as int8_t, 63 as int8_t],
    [-(4) as int8_t, 68 as int8_t],
    [-(12) as int8_t, 84 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [-(7) as int8_t, 65 as int8_t],
    [8 as int8_t, 61 as int8_t],
    [5 as int8_t, 56 as int8_t],
    [-(2) as int8_t, 66 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [-(2) as int8_t, 78 as int8_t],
    [1 as int8_t, 50 as int8_t],
    [7 as int8_t, 52 as int8_t],
    [10 as int8_t, 35 as int8_t],
    [0 as int8_t, 44 as int8_t],
    [11 as int8_t, 38 as int8_t],
    [1 as int8_t, 45 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [5 as int8_t, 44 as int8_t],
    [31 as int8_t, 17 as int8_t],
    [1 as int8_t, 51 as int8_t],
    [7 as int8_t, 50 as int8_t],
    [28 as int8_t, 19 as int8_t],
    [16 as int8_t, 33 as int8_t],
    [14 as int8_t, 62 as int8_t],
    [-(13) as int8_t, 108 as int8_t],
    [-(15) as int8_t, 100 as int8_t],
    [-(13) as int8_t, 101 as int8_t],
    [-(13) as int8_t, 91 as int8_t],
    [-(12) as int8_t, 94 as int8_t],
    [-(10) as int8_t, 88 as int8_t],
    [-(16) as int8_t, 84 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(7) as int8_t, 83 as int8_t],
    [-(13) as int8_t, 87 as int8_t],
    [-(19) as int8_t, 94 as int8_t],
    [1 as int8_t, 70 as int8_t],
    [0 as int8_t, 72 as int8_t],
    [-(5) as int8_t, 74 as int8_t],
    [18 as int8_t, 59 as int8_t],
    [-(8) as int8_t, 102 as int8_t],
    [-(15) as int8_t, 100 as int8_t],
    [0 as int8_t, 95 as int8_t],
    [-(4) as int8_t, 75 as int8_t],
    [2 as int8_t, 72 as int8_t],
    [-(11) as int8_t, 75 as int8_t],
    [-(3) as int8_t, 71 as int8_t],
    [15 as int8_t, 46 as int8_t],
    [-(13) as int8_t, 69 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [0 as int8_t, 65 as int8_t],
    [21 as int8_t, 37 as int8_t],
    [-(15) as int8_t, 72 as int8_t],
    [9 as int8_t, 57 as int8_t],
    [16 as int8_t, 54 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [12 as int8_t, 72 as int8_t],
    [24 as int8_t, 0 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [8 as int8_t, 25 as int8_t],
    [13 as int8_t, 18 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [13 as int8_t, 19 as int8_t],
    [10 as int8_t, 37 as int8_t],
    [12 as int8_t, 18 as int8_t],
    [6 as int8_t, 29 as int8_t],
    [20 as int8_t, 33 as int8_t],
    [15 as int8_t, 30 as int8_t],
    [4 as int8_t, 45 as int8_t],
    [1 as int8_t, 58 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [7 as int8_t, 61 as int8_t],
    [12 as int8_t, 38 as int8_t],
    [11 as int8_t, 45 as int8_t],
    [15 as int8_t, 39 as int8_t],
    [11 as int8_t, 42 as int8_t],
    [13 as int8_t, 44 as int8_t],
    [16 as int8_t, 45 as int8_t],
    [12 as int8_t, 41 as int8_t],
    [10 as int8_t, 49 as int8_t],
    [30 as int8_t, 34 as int8_t],
    [18 as int8_t, 42 as int8_t],
    [10 as int8_t, 55 as int8_t],
    [17 as int8_t, 51 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [0 as int8_t, 89 as int8_t],
    [26 as int8_t, -(19) as int8_t],
    [22 as int8_t, -(17) as int8_t],
    [26 as int8_t, -(17) as int8_t],
    [30 as int8_t, -(25) as int8_t],
    [28 as int8_t, -(20) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [37 as int8_t, -(27) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [40 as int8_t, -(28) as int8_t],
    [38 as int8_t, -(17) as int8_t],
    [33 as int8_t, -(11) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [41 as int8_t, -(6) as int8_t],
    [38 as int8_t, 1 as int8_t],
    [41 as int8_t, 17 as int8_t],
    [30 as int8_t, -(6) as int8_t],
    [27 as int8_t, 3 as int8_t],
    [26 as int8_t, 22 as int8_t],
    [37 as int8_t, -(16) as int8_t],
    [35 as int8_t, -(4) as int8_t],
    [38 as int8_t, -(8) as int8_t],
    [38 as int8_t, -(3) as int8_t],
    [37 as int8_t, 3 as int8_t],
    [38 as int8_t, 5 as int8_t],
    [42 as int8_t, 0 as int8_t],
    [35 as int8_t, 16 as int8_t],
    [39 as int8_t, 22 as int8_t],
    [14 as int8_t, 48 as int8_t],
    [27 as int8_t, 37 as int8_t],
    [21 as int8_t, 60 as int8_t],
    [12 as int8_t, 68 as int8_t],
    [2 as int8_t, 97 as int8_t],
    [-(3) as int8_t, 71 as int8_t],
    [-(6) as int8_t, 42 as int8_t],
    [-(5) as int8_t, 50 as int8_t],
    [-(3) as int8_t, 54 as int8_t],
    [-(2) as int8_t, 62 as int8_t],
    [0 as int8_t, 58 as int8_t],
    [1 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 72 as int8_t],
    [-1 as int8_t, 74 as int8_t],
    [-(9) as int8_t, 91 as int8_t],
    [-(5) as int8_t, 67 as int8_t],
    [-(5) as int8_t, 27 as int8_t],
    [-(3) as int8_t, 39 as int8_t],
    [-(2) as int8_t, 44 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [-(16) as int8_t, 64 as int8_t],
    [-(8) as int8_t, 68 as int8_t],
    [-(10) as int8_t, 78 as int8_t],
    [-(6) as int8_t, 77 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(12) as int8_t, 92 as int8_t],
    [-(15) as int8_t, 55 as int8_t],
    [-(10) as int8_t, 60 as int8_t],
    [-(6) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 65 as int8_t],
    [-(12) as int8_t, 73 as int8_t],
    [-(8) as int8_t, 76 as int8_t],
    [-(7) as int8_t, 80 as int8_t],
    [-(9) as int8_t, 88 as int8_t],
    [-(17) as int8_t, 110 as int8_t],
    [-(11) as int8_t, 97 as int8_t],
    [-(20) as int8_t, 84 as int8_t],
    [-(11) as int8_t, 79 as int8_t],
    [-(6) as int8_t, 73 as int8_t],
    [-(4) as int8_t, 74 as int8_t],
    [-(13) as int8_t, 86 as int8_t],
    [-(13) as int8_t, 96 as int8_t],
    [-(11) as int8_t, 97 as int8_t],
    [-(19) as int8_t, 117 as int8_t],
    [-(8) as int8_t, 78 as int8_t],
    [-(5) as int8_t, 33 as int8_t],
    [-(4) as int8_t, 48 as int8_t],
    [-(2) as int8_t, 53 as int8_t],
    [-(3) as int8_t, 62 as int8_t],
    [-(13) as int8_t, 71 as int8_t],
    [-(10) as int8_t, 79 as int8_t],
    [-(12) as int8_t, 86 as int8_t],
    [-(13) as int8_t, 90 as int8_t],
    [-(14) as int8_t, 97 as int8_t],
    [0 as int8_t, 0 as int8_t],
    [-(6) as int8_t, 93 as int8_t],
    [-(6) as int8_t, 84 as int8_t],
    [-(8) as int8_t, 79 as int8_t],
    [0 as int8_t, 66 as int8_t],
    [-1 as int8_t, 71 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [-(2) as int8_t, 60 as int8_t],
    [-(2) as int8_t, 59 as int8_t],
    [-(5) as int8_t, 75 as int8_t],
    [-(3) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 58 as int8_t],
    [-(9) as int8_t, 66 as int8_t],
    [-1 as int8_t, 79 as int8_t],
    [0 as int8_t, 71 as int8_t],
    [3 as int8_t, 68 as int8_t],
    [10 as int8_t, 44 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [15 as int8_t, 36 as int8_t],
    [14 as int8_t, 40 as int8_t],
    [16 as int8_t, 27 as int8_t],
    [12 as int8_t, 29 as int8_t],
    [1 as int8_t, 44 as int8_t],
    [20 as int8_t, 36 as int8_t],
    [18 as int8_t, 32 as int8_t],
    [5 as int8_t, 42 as int8_t],
    [1 as int8_t, 48 as int8_t],
    [10 as int8_t, 62 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [9 as int8_t, 64 as int8_t],
    [-(12) as int8_t, 104 as int8_t],
    [-(11) as int8_t, 97 as int8_t],
    [-(16) as int8_t, 96 as int8_t],
    [-(7) as int8_t, 88 as int8_t],
    [-(8) as int8_t, 85 as int8_t],
    [-(7) as int8_t, 85 as int8_t],
    [-(9) as int8_t, 85 as int8_t],
    [-(13) as int8_t, 88 as int8_t],
    [4 as int8_t, 66 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(3) as int8_t, 76 as int8_t],
    [-(6) as int8_t, 76 as int8_t],
    [10 as int8_t, 58 as int8_t],
    [-1 as int8_t, 76 as int8_t],
    [-1 as int8_t, 83 as int8_t],
    [-(7) as int8_t, 99 as int8_t],
    [-(14) as int8_t, 95 as int8_t],
    [2 as int8_t, 95 as int8_t],
    [0 as int8_t, 76 as int8_t],
    [-(5) as int8_t, 74 as int8_t],
    [0 as int8_t, 70 as int8_t],
    [-(11) as int8_t, 75 as int8_t],
    [1 as int8_t, 68 as int8_t],
    [0 as int8_t, 65 as int8_t],
    [-(14) as int8_t, 73 as int8_t],
    [3 as int8_t, 62 as int8_t],
    [4 as int8_t, 62 as int8_t],
    [-1 as int8_t, 68 as int8_t],
    [-(13) as int8_t, 75 as int8_t],
    [11 as int8_t, 55 as int8_t],
    [5 as int8_t, 64 as int8_t],
    [12 as int8_t, 70 as int8_t],
    [15 as int8_t, 6 as int8_t],
    [6 as int8_t, 19 as int8_t],
    [7 as int8_t, 16 as int8_t],
    [12 as int8_t, 14 as int8_t],
    [18 as int8_t, 13 as int8_t],
    [13 as int8_t, 11 as int8_t],
    [13 as int8_t, 15 as int8_t],
    [15 as int8_t, 16 as int8_t],
    [12 as int8_t, 23 as int8_t],
    [13 as int8_t, 23 as int8_t],
    [15 as int8_t, 20 as int8_t],
    [14 as int8_t, 26 as int8_t],
    [14 as int8_t, 44 as int8_t],
    [17 as int8_t, 40 as int8_t],
    [17 as int8_t, 47 as int8_t],
    [24 as int8_t, 17 as int8_t],
    [21 as int8_t, 21 as int8_t],
    [25 as int8_t, 22 as int8_t],
    [31 as int8_t, 27 as int8_t],
    [22 as int8_t, 29 as int8_t],
    [19 as int8_t, 35 as int8_t],
    [14 as int8_t, 50 as int8_t],
    [10 as int8_t, 57 as int8_t],
    [7 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 77 as int8_t],
    [-(4) as int8_t, 82 as int8_t],
    [-(3) as int8_t, 94 as int8_t],
    [9 as int8_t, 69 as int8_t],
    [-(12) as int8_t, 109 as int8_t],
    [36 as int8_t, -(35) as int8_t],
    [36 as int8_t, -(34) as int8_t],
    [32 as int8_t, -(26) as int8_t],
    [37 as int8_t, -(30) as int8_t],
    [44 as int8_t, -(32) as int8_t],
    [34 as int8_t, -(18) as int8_t],
    [34 as int8_t, -(15) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [33 as int8_t, -(7) as int8_t],
    [35 as int8_t, -(5) as int8_t],
    [33 as int8_t, 0 as int8_t],
    [38 as int8_t, 2 as int8_t],
    [33 as int8_t, 13 as int8_t],
    [23 as int8_t, 35 as int8_t],
    [13 as int8_t, 58 as int8_t],
    [29 as int8_t, -(3) as int8_t],
    [26 as int8_t, 0 as int8_t],
    [22 as int8_t, 30 as int8_t],
    [31 as int8_t, -(7) as int8_t],
    [35 as int8_t, -(15) as int8_t],
    [34 as int8_t, -(3) as int8_t],
    [34 as int8_t, 3 as int8_t],
    [36 as int8_t, -1 as int8_t],
    [34 as int8_t, 5 as int8_t],
    [32 as int8_t, 11 as int8_t],
    [35 as int8_t, 5 as int8_t],
    [34 as int8_t, 12 as int8_t],
    [39 as int8_t, 11 as int8_t],
    [30 as int8_t, 29 as int8_t],
    [34 as int8_t, 26 as int8_t],
    [29 as int8_t, 39 as int8_t],
    [19 as int8_t, 66 as int8_t],
    [31 as int8_t, 21 as int8_t],
    [31 as int8_t, 31 as int8_t],
    [25 as int8_t, 50 as int8_t],
    [-(17) as int8_t, 120 as int8_t],
    [-(20) as int8_t, 112 as int8_t],
    [-(18) as int8_t, 114 as int8_t],
    [-(11) as int8_t, 85 as int8_t],
    [-(15) as int8_t, 92 as int8_t],
    [-(14) as int8_t, 89 as int8_t],
    [-(26) as int8_t, 71 as int8_t],
    [-(15) as int8_t, 81 as int8_t],
    [-(14) as int8_t, 80 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(14) as int8_t, 70 as int8_t],
    [-(24) as int8_t, 56 as int8_t],
    [-(23) as int8_t, 68 as int8_t],
    [-(24) as int8_t, 50 as int8_t],
    [-(11) as int8_t, 74 as int8_t],
    [23 as int8_t, -(13) as int8_t],
    [26 as int8_t, -(13) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [49 as int8_t, -(14) as int8_t],
    [44 as int8_t, 3 as int8_t],
    [45 as int8_t, 6 as int8_t],
    [44 as int8_t, 34 as int8_t],
    [33 as int8_t, 54 as int8_t],
    [19 as int8_t, 82 as int8_t],
    [-(3) as int8_t, 75 as int8_t],
    [-1 as int8_t, 23 as int8_t],
    [1 as int8_t, 34 as int8_t],
    [1 as int8_t, 43 as int8_t],
    [0 as int8_t, 54 as int8_t],
    [-(2) as int8_t, 55 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(9) as int8_t, 92 as int8_t],
    [-(14) as int8_t, 106 as int8_t],
    [-(13) as int8_t, 97 as int8_t],
    [-(15) as int8_t, 90 as int8_t],
    [-(12) as int8_t, 90 as int8_t],
    [-(18) as int8_t, 88 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(9) as int8_t, 79 as int8_t],
    [-(14) as int8_t, 86 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(10) as int8_t, 70 as int8_t],
    [-(10) as int8_t, 69 as int8_t],
    [-(5) as int8_t, 66 as int8_t],
    [-(9) as int8_t, 64 as int8_t],
    [-(5) as int8_t, 58 as int8_t],
    [2 as int8_t, 59 as int8_t],
    [21 as int8_t, -(10) as int8_t],
    [24 as int8_t, -(11) as int8_t],
    [28 as int8_t, -(8) as int8_t],
    [28 as int8_t, -1 as int8_t],
    [29 as int8_t, 3 as int8_t],
    [29 as int8_t, 9 as int8_t],
    [35 as int8_t, 20 as int8_t],
    [29 as int8_t, 36 as int8_t],
    [14 as int8_t, 67 as int8_t],
    [-(17) as int8_t, 123 as int8_t],
    [-(12) as int8_t, 115 as int8_t],
    [-(16) as int8_t, 122 as int8_t],
    [-(11) as int8_t, 115 as int8_t],
    [-(12) as int8_t, 63 as int8_t],
    [-(2) as int8_t, 68 as int8_t],
    [-(15) as int8_t, 84 as int8_t],
    [-(13) as int8_t, 104 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-(17) as int8_t, 123 as int8_t],
    [-(12) as int8_t, 115 as int8_t],
    [-(16) as int8_t, 122 as int8_t],
    [-(11) as int8_t, 115 as int8_t],
    [-(12) as int8_t, 63 as int8_t],
    [-(2) as int8_t, 68 as int8_t],
    [-(15) as int8_t, 84 as int8_t],
    [-(13) as int8_t, 104 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-(7) as int8_t, 93 as int8_t],
    [-(11) as int8_t, 87 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(5) as int8_t, 71 as int8_t],
    [-(4) as int8_t, 63 as int8_t],
    [-(4) as int8_t, 68 as int8_t],
    [-(12) as int8_t, 84 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [-(7) as int8_t, 65 as int8_t],
    [8 as int8_t, 61 as int8_t],
    [5 as int8_t, 56 as int8_t],
    [-(2) as int8_t, 66 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [-(2) as int8_t, 78 as int8_t],
    [1 as int8_t, 50 as int8_t],
    [7 as int8_t, 52 as int8_t],
    [10 as int8_t, 35 as int8_t],
    [0 as int8_t, 44 as int8_t],
    [11 as int8_t, 38 as int8_t],
    [1 as int8_t, 45 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [5 as int8_t, 44 as int8_t],
    [31 as int8_t, 17 as int8_t],
    [1 as int8_t, 51 as int8_t],
    [7 as int8_t, 50 as int8_t],
    [28 as int8_t, 19 as int8_t],
    [16 as int8_t, 33 as int8_t],
    [14 as int8_t, 62 as int8_t],
    [-(13) as int8_t, 108 as int8_t],
    [-(15) as int8_t, 100 as int8_t],
    [-(13) as int8_t, 101 as int8_t],
    [-(13) as int8_t, 91 as int8_t],
    [-(12) as int8_t, 94 as int8_t],
    [-(10) as int8_t, 88 as int8_t],
    [-(16) as int8_t, 84 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(7) as int8_t, 83 as int8_t],
    [-(13) as int8_t, 87 as int8_t],
    [-(19) as int8_t, 94 as int8_t],
    [1 as int8_t, 70 as int8_t],
    [0 as int8_t, 72 as int8_t],
    [-(5) as int8_t, 74 as int8_t],
    [18 as int8_t, 59 as int8_t],
    [-(7) as int8_t, 93 as int8_t],
    [-(11) as int8_t, 87 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(5) as int8_t, 71 as int8_t],
    [-(4) as int8_t, 63 as int8_t],
    [-(4) as int8_t, 68 as int8_t],
    [-(12) as int8_t, 84 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [-(7) as int8_t, 65 as int8_t],
    [8 as int8_t, 61 as int8_t],
    [5 as int8_t, 56 as int8_t],
    [-(2) as int8_t, 66 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [-(2) as int8_t, 78 as int8_t],
    [1 as int8_t, 50 as int8_t],
    [7 as int8_t, 52 as int8_t],
    [10 as int8_t, 35 as int8_t],
    [0 as int8_t, 44 as int8_t],
    [11 as int8_t, 38 as int8_t],
    [1 as int8_t, 45 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [5 as int8_t, 44 as int8_t],
    [31 as int8_t, 17 as int8_t],
    [1 as int8_t, 51 as int8_t],
    [7 as int8_t, 50 as int8_t],
    [28 as int8_t, 19 as int8_t],
    [16 as int8_t, 33 as int8_t],
    [14 as int8_t, 62 as int8_t],
    [-(13) as int8_t, 108 as int8_t],
    [-(15) as int8_t, 100 as int8_t],
    [-(13) as int8_t, 101 as int8_t],
    [-(13) as int8_t, 91 as int8_t],
    [-(12) as int8_t, 94 as int8_t],
    [-(10) as int8_t, 88 as int8_t],
    [-(16) as int8_t, 84 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(7) as int8_t, 83 as int8_t],
    [-(13) as int8_t, 87 as int8_t],
    [-(19) as int8_t, 94 as int8_t],
    [1 as int8_t, 70 as int8_t],
    [0 as int8_t, 72 as int8_t],
    [-(5) as int8_t, 74 as int8_t],
    [18 as int8_t, 59 as int8_t],
    [24 as int8_t, 0 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [8 as int8_t, 25 as int8_t],
    [13 as int8_t, 18 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [13 as int8_t, 19 as int8_t],
    [10 as int8_t, 37 as int8_t],
    [12 as int8_t, 18 as int8_t],
    [6 as int8_t, 29 as int8_t],
    [20 as int8_t, 33 as int8_t],
    [15 as int8_t, 30 as int8_t],
    [4 as int8_t, 45 as int8_t],
    [1 as int8_t, 58 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [7 as int8_t, 61 as int8_t],
    [12 as int8_t, 38 as int8_t],
    [11 as int8_t, 45 as int8_t],
    [15 as int8_t, 39 as int8_t],
    [11 as int8_t, 42 as int8_t],
    [13 as int8_t, 44 as int8_t],
    [16 as int8_t, 45 as int8_t],
    [12 as int8_t, 41 as int8_t],
    [10 as int8_t, 49 as int8_t],
    [30 as int8_t, 34 as int8_t],
    [18 as int8_t, 42 as int8_t],
    [10 as int8_t, 55 as int8_t],
    [17 as int8_t, 51 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [0 as int8_t, 89 as int8_t],
    [26 as int8_t, -(19) as int8_t],
    [22 as int8_t, -(17) as int8_t],
    [26 as int8_t, -(17) as int8_t],
    [30 as int8_t, -(25) as int8_t],
    [28 as int8_t, -(20) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [37 as int8_t, -(27) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [40 as int8_t, -(28) as int8_t],
    [38 as int8_t, -(17) as int8_t],
    [33 as int8_t, -(11) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [41 as int8_t, -(6) as int8_t],
    [38 as int8_t, 1 as int8_t],
    [41 as int8_t, 17 as int8_t],
    [24 as int8_t, 0 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [8 as int8_t, 25 as int8_t],
    [13 as int8_t, 18 as int8_t],
    [15 as int8_t, 9 as int8_t],
    [13 as int8_t, 19 as int8_t],
    [10 as int8_t, 37 as int8_t],
    [12 as int8_t, 18 as int8_t],
    [6 as int8_t, 29 as int8_t],
    [20 as int8_t, 33 as int8_t],
    [15 as int8_t, 30 as int8_t],
    [4 as int8_t, 45 as int8_t],
    [1 as int8_t, 58 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [7 as int8_t, 61 as int8_t],
    [12 as int8_t, 38 as int8_t],
    [11 as int8_t, 45 as int8_t],
    [15 as int8_t, 39 as int8_t],
    [11 as int8_t, 42 as int8_t],
    [13 as int8_t, 44 as int8_t],
    [16 as int8_t, 45 as int8_t],
    [12 as int8_t, 41 as int8_t],
    [10 as int8_t, 49 as int8_t],
    [30 as int8_t, 34 as int8_t],
    [18 as int8_t, 42 as int8_t],
    [10 as int8_t, 55 as int8_t],
    [17 as int8_t, 51 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [0 as int8_t, 89 as int8_t],
    [26 as int8_t, -(19) as int8_t],
    [22 as int8_t, -(17) as int8_t],
    [26 as int8_t, -(17) as int8_t],
    [30 as int8_t, -(25) as int8_t],
    [28 as int8_t, -(20) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [37 as int8_t, -(27) as int8_t],
    [33 as int8_t, -(23) as int8_t],
    [40 as int8_t, -(28) as int8_t],
    [38 as int8_t, -(17) as int8_t],
    [33 as int8_t, -(11) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [41 as int8_t, -(6) as int8_t],
    [38 as int8_t, 1 as int8_t],
    [41 as int8_t, 17 as int8_t],
    [-(17) as int8_t, 120 as int8_t],
    [-(20) as int8_t, 112 as int8_t],
    [-(18) as int8_t, 114 as int8_t],
    [-(11) as int8_t, 85 as int8_t],
    [-(15) as int8_t, 92 as int8_t],
    [-(14) as int8_t, 89 as int8_t],
    [-(26) as int8_t, 71 as int8_t],
    [-(15) as int8_t, 81 as int8_t],
    [-(14) as int8_t, 80 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(14) as int8_t, 70 as int8_t],
    [-(24) as int8_t, 56 as int8_t],
    [-(23) as int8_t, 68 as int8_t],
    [-(24) as int8_t, 50 as int8_t],
    [-(11) as int8_t, 74 as int8_t],
    [-(14) as int8_t, 106 as int8_t],
    [-(13) as int8_t, 97 as int8_t],
    [-(15) as int8_t, 90 as int8_t],
    [-(12) as int8_t, 90 as int8_t],
    [-(18) as int8_t, 88 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(9) as int8_t, 79 as int8_t],
    [-(14) as int8_t, 86 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(10) as int8_t, 70 as int8_t],
    [-(10) as int8_t, 69 as int8_t],
    [-(5) as int8_t, 66 as int8_t],
    [-(9) as int8_t, 64 as int8_t],
    [-(5) as int8_t, 58 as int8_t],
    [2 as int8_t, 59 as int8_t],
    [23 as int8_t, -(13) as int8_t],
    [26 as int8_t, -(13) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [49 as int8_t, -(14) as int8_t],
    [44 as int8_t, 3 as int8_t],
    [45 as int8_t, 6 as int8_t],
    [44 as int8_t, 34 as int8_t],
    [33 as int8_t, 54 as int8_t],
    [19 as int8_t, 82 as int8_t],
    [21 as int8_t, -(10) as int8_t],
    [24 as int8_t, -(11) as int8_t],
    [28 as int8_t, -(8) as int8_t],
    [28 as int8_t, -1 as int8_t],
    [29 as int8_t, 3 as int8_t],
    [29 as int8_t, 9 as int8_t],
    [35 as int8_t, 20 as int8_t],
    [29 as int8_t, 36 as int8_t],
    [14 as int8_t, 67 as int8_t],
    [-(3) as int8_t, 75 as int8_t],
    [-1 as int8_t, 23 as int8_t],
    [1 as int8_t, 34 as int8_t],
    [1 as int8_t, 43 as int8_t],
    [0 as int8_t, 54 as int8_t],
    [-(2) as int8_t, 55 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(9) as int8_t, 92 as int8_t],
    [-(17) as int8_t, 120 as int8_t],
    [-(20) as int8_t, 112 as int8_t],
    [-(18) as int8_t, 114 as int8_t],
    [-(11) as int8_t, 85 as int8_t],
    [-(15) as int8_t, 92 as int8_t],
    [-(14) as int8_t, 89 as int8_t],
    [-(26) as int8_t, 71 as int8_t],
    [-(15) as int8_t, 81 as int8_t],
    [-(14) as int8_t, 80 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(14) as int8_t, 70 as int8_t],
    [-(24) as int8_t, 56 as int8_t],
    [-(23) as int8_t, 68 as int8_t],
    [-(24) as int8_t, 50 as int8_t],
    [-(11) as int8_t, 74 as int8_t],
    [-(14) as int8_t, 106 as int8_t],
    [-(13) as int8_t, 97 as int8_t],
    [-(15) as int8_t, 90 as int8_t],
    [-(12) as int8_t, 90 as int8_t],
    [-(18) as int8_t, 88 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(9) as int8_t, 79 as int8_t],
    [-(14) as int8_t, 86 as int8_t],
    [-(10) as int8_t, 73 as int8_t],
    [-(10) as int8_t, 70 as int8_t],
    [-(10) as int8_t, 69 as int8_t],
    [-(5) as int8_t, 66 as int8_t],
    [-(9) as int8_t, 64 as int8_t],
    [-(5) as int8_t, 58 as int8_t],
    [2 as int8_t, 59 as int8_t],
    [23 as int8_t, -(13) as int8_t],
    [26 as int8_t, -(13) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [49 as int8_t, -(14) as int8_t],
    [44 as int8_t, 3 as int8_t],
    [45 as int8_t, 6 as int8_t],
    [44 as int8_t, 34 as int8_t],
    [33 as int8_t, 54 as int8_t],
    [19 as int8_t, 82 as int8_t],
    [21 as int8_t, -(10) as int8_t],
    [24 as int8_t, -(11) as int8_t],
    [28 as int8_t, -(8) as int8_t],
    [28 as int8_t, -1 as int8_t],
    [29 as int8_t, 3 as int8_t],
    [29 as int8_t, 9 as int8_t],
    [35 as int8_t, 20 as int8_t],
    [29 as int8_t, 36 as int8_t],
    [14 as int8_t, 67 as int8_t],
    [-(3) as int8_t, 75 as int8_t],
    [-1 as int8_t, 23 as int8_t],
    [1 as int8_t, 34 as int8_t],
    [1 as int8_t, 43 as int8_t],
    [0 as int8_t, 54 as int8_t],
    [-(2) as int8_t, 55 as int8_t],
    [0 as int8_t, 61 as int8_t],
    [1 as int8_t, 64 as int8_t],
    [0 as int8_t, 68 as int8_t],
    [-(9) as int8_t, 92 as int8_t],
    [-(6) as int8_t, 93 as int8_t],
    [-(6) as int8_t, 84 as int8_t],
    [-(8) as int8_t, 79 as int8_t],
    [0 as int8_t, 66 as int8_t],
    [-1 as int8_t, 71 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [-(2) as int8_t, 60 as int8_t],
    [-(2) as int8_t, 59 as int8_t],
    [-(5) as int8_t, 75 as int8_t],
    [-(3) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 58 as int8_t],
    [-(9) as int8_t, 66 as int8_t],
    [-1 as int8_t, 79 as int8_t],
    [0 as int8_t, 71 as int8_t],
    [3 as int8_t, 68 as int8_t],
    [10 as int8_t, 44 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [15 as int8_t, 36 as int8_t],
    [14 as int8_t, 40 as int8_t],
    [16 as int8_t, 27 as int8_t],
    [12 as int8_t, 29 as int8_t],
    [1 as int8_t, 44 as int8_t],
    [20 as int8_t, 36 as int8_t],
    [18 as int8_t, 32 as int8_t],
    [5 as int8_t, 42 as int8_t],
    [1 as int8_t, 48 as int8_t],
    [10 as int8_t, 62 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [9 as int8_t, 64 as int8_t],
    [-(12) as int8_t, 104 as int8_t],
    [-(11) as int8_t, 97 as int8_t],
    [-(16) as int8_t, 96 as int8_t],
    [-(7) as int8_t, 88 as int8_t],
    [-(8) as int8_t, 85 as int8_t],
    [-(7) as int8_t, 85 as int8_t],
    [-(9) as int8_t, 85 as int8_t],
    [-(13) as int8_t, 88 as int8_t],
    [4 as int8_t, 66 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(3) as int8_t, 76 as int8_t],
    [-(6) as int8_t, 76 as int8_t],
    [10 as int8_t, 58 as int8_t],
    [-1 as int8_t, 76 as int8_t],
    [-1 as int8_t, 83 as int8_t],
    [-(6) as int8_t, 93 as int8_t],
    [-(6) as int8_t, 84 as int8_t],
    [-(8) as int8_t, 79 as int8_t],
    [0 as int8_t, 66 as int8_t],
    [-1 as int8_t, 71 as int8_t],
    [0 as int8_t, 62 as int8_t],
    [-(2) as int8_t, 60 as int8_t],
    [-(2) as int8_t, 59 as int8_t],
    [-(5) as int8_t, 75 as int8_t],
    [-(3) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 58 as int8_t],
    [-(9) as int8_t, 66 as int8_t],
    [-1 as int8_t, 79 as int8_t],
    [0 as int8_t, 71 as int8_t],
    [3 as int8_t, 68 as int8_t],
    [10 as int8_t, 44 as int8_t],
    [-(7) as int8_t, 62 as int8_t],
    [15 as int8_t, 36 as int8_t],
    [14 as int8_t, 40 as int8_t],
    [16 as int8_t, 27 as int8_t],
    [12 as int8_t, 29 as int8_t],
    [1 as int8_t, 44 as int8_t],
    [20 as int8_t, 36 as int8_t],
    [18 as int8_t, 32 as int8_t],
    [5 as int8_t, 42 as int8_t],
    [1 as int8_t, 48 as int8_t],
    [10 as int8_t, 62 as int8_t],
    [17 as int8_t, 46 as int8_t],
    [9 as int8_t, 64 as int8_t],
    [-(12) as int8_t, 104 as int8_t],
    [-(11) as int8_t, 97 as int8_t],
    [-(16) as int8_t, 96 as int8_t],
    [-(7) as int8_t, 88 as int8_t],
    [-(8) as int8_t, 85 as int8_t],
    [-(7) as int8_t, 85 as int8_t],
    [-(9) as int8_t, 85 as int8_t],
    [-(13) as int8_t, 88 as int8_t],
    [4 as int8_t, 66 as int8_t],
    [-(3) as int8_t, 77 as int8_t],
    [-(3) as int8_t, 76 as int8_t],
    [-(6) as int8_t, 76 as int8_t],
    [10 as int8_t, 58 as int8_t],
    [-1 as int8_t, 76 as int8_t],
    [-1 as int8_t, 83 as int8_t],
    [15 as int8_t, 6 as int8_t],
    [6 as int8_t, 19 as int8_t],
    [7 as int8_t, 16 as int8_t],
    [12 as int8_t, 14 as int8_t],
    [18 as int8_t, 13 as int8_t],
    [13 as int8_t, 11 as int8_t],
    [13 as int8_t, 15 as int8_t],
    [15 as int8_t, 16 as int8_t],
    [12 as int8_t, 23 as int8_t],
    [13 as int8_t, 23 as int8_t],
    [15 as int8_t, 20 as int8_t],
    [14 as int8_t, 26 as int8_t],
    [14 as int8_t, 44 as int8_t],
    [17 as int8_t, 40 as int8_t],
    [17 as int8_t, 47 as int8_t],
    [24 as int8_t, 17 as int8_t],
    [21 as int8_t, 21 as int8_t],
    [25 as int8_t, 22 as int8_t],
    [31 as int8_t, 27 as int8_t],
    [22 as int8_t, 29 as int8_t],
    [19 as int8_t, 35 as int8_t],
    [14 as int8_t, 50 as int8_t],
    [10 as int8_t, 57 as int8_t],
    [7 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 77 as int8_t],
    [-(4) as int8_t, 82 as int8_t],
    [-(3) as int8_t, 94 as int8_t],
    [9 as int8_t, 69 as int8_t],
    [-(12) as int8_t, 109 as int8_t],
    [36 as int8_t, -(35) as int8_t],
    [36 as int8_t, -(34) as int8_t],
    [32 as int8_t, -(26) as int8_t],
    [37 as int8_t, -(30) as int8_t],
    [44 as int8_t, -(32) as int8_t],
    [34 as int8_t, -(18) as int8_t],
    [34 as int8_t, -(15) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [33 as int8_t, -(7) as int8_t],
    [35 as int8_t, -(5) as int8_t],
    [33 as int8_t, 0 as int8_t],
    [38 as int8_t, 2 as int8_t],
    [33 as int8_t, 13 as int8_t],
    [23 as int8_t, 35 as int8_t],
    [13 as int8_t, 58 as int8_t],
    [15 as int8_t, 6 as int8_t],
    [6 as int8_t, 19 as int8_t],
    [7 as int8_t, 16 as int8_t],
    [12 as int8_t, 14 as int8_t],
    [18 as int8_t, 13 as int8_t],
    [13 as int8_t, 11 as int8_t],
    [13 as int8_t, 15 as int8_t],
    [15 as int8_t, 16 as int8_t],
    [12 as int8_t, 23 as int8_t],
    [13 as int8_t, 23 as int8_t],
    [15 as int8_t, 20 as int8_t],
    [14 as int8_t, 26 as int8_t],
    [14 as int8_t, 44 as int8_t],
    [17 as int8_t, 40 as int8_t],
    [17 as int8_t, 47 as int8_t],
    [24 as int8_t, 17 as int8_t],
    [21 as int8_t, 21 as int8_t],
    [25 as int8_t, 22 as int8_t],
    [31 as int8_t, 27 as int8_t],
    [22 as int8_t, 29 as int8_t],
    [19 as int8_t, 35 as int8_t],
    [14 as int8_t, 50 as int8_t],
    [10 as int8_t, 57 as int8_t],
    [7 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 77 as int8_t],
    [-(4) as int8_t, 82 as int8_t],
    [-(3) as int8_t, 94 as int8_t],
    [9 as int8_t, 69 as int8_t],
    [-(12) as int8_t, 109 as int8_t],
    [36 as int8_t, -(35) as int8_t],
    [36 as int8_t, -(34) as int8_t],
    [32 as int8_t, -(26) as int8_t],
    [37 as int8_t, -(30) as int8_t],
    [44 as int8_t, -(32) as int8_t],
    [34 as int8_t, -(18) as int8_t],
    [34 as int8_t, -(15) as int8_t],
    [40 as int8_t, -(15) as int8_t],
    [33 as int8_t, -(7) as int8_t],
    [35 as int8_t, -(5) as int8_t],
    [33 as int8_t, 0 as int8_t],
    [38 as int8_t, 2 as int8_t],
    [33 as int8_t, 13 as int8_t],
    [23 as int8_t, 35 as int8_t],
    [13 as int8_t, 58 as int8_t],
    [-(3) as int8_t, 71 as int8_t],
    [-(6) as int8_t, 42 as int8_t],
    [-(5) as int8_t, 50 as int8_t],
    [-(3) as int8_t, 54 as int8_t],
    [-(2) as int8_t, 62 as int8_t],
    [0 as int8_t, 58 as int8_t],
    [1 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 72 as int8_t],
    [-1 as int8_t, 74 as int8_t],
    [-(9) as int8_t, 91 as int8_t],
    [-(5) as int8_t, 67 as int8_t],
    [-(5) as int8_t, 27 as int8_t],
    [-(3) as int8_t, 39 as int8_t],
    [-(2) as int8_t, 44 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [-(16) as int8_t, 64 as int8_t],
    [-(8) as int8_t, 68 as int8_t],
    [-(10) as int8_t, 78 as int8_t],
    [-(6) as int8_t, 77 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(12) as int8_t, 92 as int8_t],
    [-(15) as int8_t, 55 as int8_t],
    [-(10) as int8_t, 60 as int8_t],
    [-(6) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 65 as int8_t],
    [-(12) as int8_t, 73 as int8_t],
    [-(8) as int8_t, 76 as int8_t],
    [-(7) as int8_t, 80 as int8_t],
    [-(9) as int8_t, 88 as int8_t],
    [-(17) as int8_t, 110 as int8_t],
    [-(3) as int8_t, 71 as int8_t],
    [-(6) as int8_t, 42 as int8_t],
    [-(5) as int8_t, 50 as int8_t],
    [-(3) as int8_t, 54 as int8_t],
    [-(2) as int8_t, 62 as int8_t],
    [0 as int8_t, 58 as int8_t],
    [1 as int8_t, 63 as int8_t],
    [-(2) as int8_t, 72 as int8_t],
    [-1 as int8_t, 74 as int8_t],
    [-(9) as int8_t, 91 as int8_t],
    [-(5) as int8_t, 67 as int8_t],
    [-(5) as int8_t, 27 as int8_t],
    [-(3) as int8_t, 39 as int8_t],
    [-(2) as int8_t, 44 as int8_t],
    [0 as int8_t, 46 as int8_t],
    [-(16) as int8_t, 64 as int8_t],
    [-(8) as int8_t, 68 as int8_t],
    [-(10) as int8_t, 78 as int8_t],
    [-(6) as int8_t, 77 as int8_t],
    [-(10) as int8_t, 86 as int8_t],
    [-(12) as int8_t, 92 as int8_t],
    [-(15) as int8_t, 55 as int8_t],
    [-(10) as int8_t, 60 as int8_t],
    [-(6) as int8_t, 62 as int8_t],
    [-(4) as int8_t, 65 as int8_t],
    [-(12) as int8_t, 73 as int8_t],
    [-(8) as int8_t, 76 as int8_t],
    [-(7) as int8_t, 80 as int8_t],
    [-(9) as int8_t, 88 as int8_t],
    [-(17) as int8_t, 110 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
    [-(3) as int8_t, 70 as int8_t],
    [-(8) as int8_t, 93 as int8_t],
    [-(10) as int8_t, 90 as int8_t],
    [-(30) as int8_t, 127 as int8_t],
];
#[no_mangle]
#[c2rust::src_loc = "768:14"]
static mut x264_cabac_context_init_PB: [[[int8_t; 2]; 1024]; 3] = [
    [
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [-(28) as int8_t, 127 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [-1 as int8_t, 54 as int8_t],
        [7 as int8_t, 51 as int8_t],
        [23 as int8_t, 33 as int8_t],
        [23 as int8_t, 2 as int8_t],
        [21 as int8_t, 0 as int8_t],
        [1 as int8_t, 9 as int8_t],
        [0 as int8_t, 49 as int8_t],
        [-(37) as int8_t, 118 as int8_t],
        [5 as int8_t, 57 as int8_t],
        [-(13) as int8_t, 78 as int8_t],
        [-(11) as int8_t, 65 as int8_t],
        [1 as int8_t, 62 as int8_t],
        [12 as int8_t, 49 as int8_t],
        [-(4) as int8_t, 73 as int8_t],
        [17 as int8_t, 50 as int8_t],
        [18 as int8_t, 64 as int8_t],
        [9 as int8_t, 43 as int8_t],
        [29 as int8_t, 0 as int8_t],
        [26 as int8_t, 67 as int8_t],
        [16 as int8_t, 90 as int8_t],
        [9 as int8_t, 104 as int8_t],
        [-(46) as int8_t, 127 as int8_t],
        [-(20) as int8_t, 104 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [-(13) as int8_t, 78 as int8_t],
        [-(11) as int8_t, 65 as int8_t],
        [1 as int8_t, 62 as int8_t],
        [-(6) as int8_t, 86 as int8_t],
        [-(17) as int8_t, 95 as int8_t],
        [-(6) as int8_t, 61 as int8_t],
        [9 as int8_t, 45 as int8_t],
        [-(3) as int8_t, 69 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(11) as int8_t, 96 as int8_t],
        [6 as int8_t, 55 as int8_t],
        [7 as int8_t, 67 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [2 as int8_t, 88 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [-(3) as int8_t, 76 as int8_t],
        [-(10) as int8_t, 94 as int8_t],
        [5 as int8_t, 54 as int8_t],
        [4 as int8_t, 69 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [0 as int8_t, 88 as int8_t],
        [-(7) as int8_t, 67 as int8_t],
        [-(5) as int8_t, 74 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 80 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [1 as int8_t, 58 as int8_t],
        [0 as int8_t, 41 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [4 as int8_t, 86 as int8_t],
        [0 as int8_t, 97 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [3 as int8_t, 62 as int8_t],
        [0 as int8_t, 45 as int8_t],
        [-(4) as int8_t, 78 as int8_t],
        [-(3) as int8_t, 96 as int8_t],
        [-(27) as int8_t, 126 as int8_t],
        [-(28) as int8_t, 98 as int8_t],
        [-(25) as int8_t, 101 as int8_t],
        [-(23) as int8_t, 67 as int8_t],
        [-(28) as int8_t, 82 as int8_t],
        [-(20) as int8_t, 94 as int8_t],
        [-(16) as int8_t, 83 as int8_t],
        [-(22) as int8_t, 110 as int8_t],
        [-(21) as int8_t, 91 as int8_t],
        [-(18) as int8_t, 102 as int8_t],
        [-(13) as int8_t, 93 as int8_t],
        [-(29) as int8_t, 127 as int8_t],
        [-(7) as int8_t, 92 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 96 as int8_t],
        [-(13) as int8_t, 108 as int8_t],
        [-(3) as int8_t, 46 as int8_t],
        [-1 as int8_t, 65 as int8_t],
        [-1 as int8_t, 57 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
        [5 as int8_t, 54 as int8_t],
        [6 as int8_t, 60 as int8_t],
        [6 as int8_t, 59 as int8_t],
        [6 as int8_t, 69 as int8_t],
        [-1 as int8_t, 48 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(4) as int8_t, 69 as int8_t],
        [-(8) as int8_t, 88 as int8_t],
        [-(2) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-1 as int8_t, 75 as int8_t],
        [-(7) as int8_t, 77 as int8_t],
        [2 as int8_t, 54 as int8_t],
        [5 as int8_t, 50 as int8_t],
        [-(3) as int8_t, 68 as int8_t],
        [1 as int8_t, 50 as int8_t],
        [6 as int8_t, 42 as int8_t],
        [-(4) as int8_t, 81 as int8_t],
        [1 as int8_t, 63 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(2) as int8_t, 76 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [1 as int8_t, 61 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [18 as int8_t, 25 as int8_t],
        [12 as int8_t, 24 as int8_t],
        [13 as int8_t, 29 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [-(10) as int8_t, 93 as int8_t],
        [-(7) as int8_t, 73 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [13 as int8_t, 46 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [-(7) as int8_t, 100 as int8_t],
        [9 as int8_t, 53 as int8_t],
        [2 as int8_t, 53 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [-(2) as int8_t, 61 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [-(13) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 60 as int8_t],
        [-1 as int8_t, 62 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(6) as int8_t, 69 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [14 as int8_t, 39 as int8_t],
        [4 as int8_t, 51 as int8_t],
        [13 as int8_t, 68 as int8_t],
        [3 as int8_t, 64 as int8_t],
        [1 as int8_t, 61 as int8_t],
        [9 as int8_t, 63 as int8_t],
        [7 as int8_t, 50 as int8_t],
        [16 as int8_t, 39 as int8_t],
        [5 as int8_t, 44 as int8_t],
        [4 as int8_t, 52 as int8_t],
        [11 as int8_t, 48 as int8_t],
        [-(5) as int8_t, 60 as int8_t],
        [-1 as int8_t, 59 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [22 as int8_t, 33 as int8_t],
        [5 as int8_t, 44 as int8_t],
        [14 as int8_t, 43 as int8_t],
        [-1 as int8_t, 78 as int8_t],
        [0 as int8_t, 60 as int8_t],
        [9 as int8_t, 69 as int8_t],
        [11 as int8_t, 28 as int8_t],
        [2 as int8_t, 40 as int8_t],
        [3 as int8_t, 44 as int8_t],
        [0 as int8_t, 49 as int8_t],
        [0 as int8_t, 46 as int8_t],
        [2 as int8_t, 44 as int8_t],
        [2 as int8_t, 51 as int8_t],
        [0 as int8_t, 47 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [2 as int8_t, 62 as int8_t],
        [6 as int8_t, 46 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [3 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 51 as int8_t],
        [6 as int8_t, 57 as int8_t],
        [7 as int8_t, 53 as int8_t],
        [6 as int8_t, 52 as int8_t],
        [6 as int8_t, 55 as int8_t],
        [11 as int8_t, 45 as int8_t],
        [14 as int8_t, 36 as int8_t],
        [8 as int8_t, 53 as int8_t],
        [-1 as int8_t, 82 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [15 as int8_t, 46 as int8_t],
        [22 as int8_t, 31 as int8_t],
        [-1 as int8_t, 84 as int8_t],
        [25 as int8_t, 7 as int8_t],
        [30 as int8_t, -(7) as int8_t],
        [28 as int8_t, 3 as int8_t],
        [28 as int8_t, 4 as int8_t],
        [32 as int8_t, 0 as int8_t],
        [34 as int8_t, -1 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [32 as int8_t, 9 as int8_t],
        [31 as int8_t, 19 as int8_t],
        [26 as int8_t, 27 as int8_t],
        [26 as int8_t, 30 as int8_t],
        [37 as int8_t, 20 as int8_t],
        [28 as int8_t, 34 as int8_t],
        [17 as int8_t, 70 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [5 as int8_t, 59 as int8_t],
        [9 as int8_t, 67 as int8_t],
        [16 as int8_t, 30 as int8_t],
        [18 as int8_t, 32 as int8_t],
        [18 as int8_t, 35 as int8_t],
        [22 as int8_t, 29 as int8_t],
        [24 as int8_t, 31 as int8_t],
        [23 as int8_t, 38 as int8_t],
        [18 as int8_t, 43 as int8_t],
        [20 as int8_t, 41 as int8_t],
        [11 as int8_t, 63 as int8_t],
        [9 as int8_t, 59 as int8_t],
        [9 as int8_t, 64 as int8_t],
        [-1 as int8_t, 94 as int8_t],
        [-(2) as int8_t, 89 as int8_t],
        [-(9) as int8_t, 108 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(2) as int8_t, 44 as int8_t],
        [0 as int8_t, 45 as int8_t],
        [0 as int8_t, 52 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [-(2) as int8_t, 59 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [-(4) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 82 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(9) as int8_t, 77 as int8_t],
        [3 as int8_t, 24 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [0 as int8_t, 48 as int8_t],
        [0 as int8_t, 55 as int8_t],
        [-(6) as int8_t, 59 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(12) as int8_t, 83 as int8_t],
        [-(11) as int8_t, 87 as int8_t],
        [-(30) as int8_t, 119 as int8_t],
        [1 as int8_t, 58 as int8_t],
        [-(3) as int8_t, 29 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [1 as int8_t, 38 as int8_t],
        [2 as int8_t, 43 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [0 as int8_t, 64 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 90 as int8_t],
        [0 as int8_t, 70 as int8_t],
        [-(4) as int8_t, 29 as int8_t],
        [5 as int8_t, 31 as int8_t],
        [7 as int8_t, 42 as int8_t],
        [1 as int8_t, 59 as int8_t],
        [-(2) as int8_t, 58 as int8_t],
        [-(3) as int8_t, 72 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [-(11) as int8_t, 97 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [8 as int8_t, 5 as int8_t],
        [10 as int8_t, 14 as int8_t],
        [14 as int8_t, 18 as int8_t],
        [13 as int8_t, 27 as int8_t],
        [2 as int8_t, 40 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [-(3) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 79 as int8_t],
        [-(8) as int8_t, 85 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(16) as int8_t, 106 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(21) as int8_t, 114 as int8_t],
        [-(18) as int8_t, 110 as int8_t],
        [-(14) as int8_t, 98 as int8_t],
        [-(22) as int8_t, 110 as int8_t],
        [-(21) as int8_t, 106 as int8_t],
        [-(18) as int8_t, 103 as int8_t],
        [-(21) as int8_t, 107 as int8_t],
        [-(23) as int8_t, 108 as int8_t],
        [-(26) as int8_t, 112 as int8_t],
        [-(10) as int8_t, 96 as int8_t],
        [-(12) as int8_t, 95 as int8_t],
        [-(5) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(22) as int8_t, 94 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [9 as int8_t, 67 as int8_t],
        [-(4) as int8_t, 80 as int8_t],
        [-(10) as int8_t, 85 as int8_t],
        [-1 as int8_t, 70 as int8_t],
        [7 as int8_t, 60 as int8_t],
        [9 as int8_t, 58 as int8_t],
        [5 as int8_t, 61 as int8_t],
        [12 as int8_t, 50 as int8_t],
        [15 as int8_t, 50 as int8_t],
        [18 as int8_t, 49 as int8_t],
        [17 as int8_t, 54 as int8_t],
        [10 as int8_t, 41 as int8_t],
        [7 as int8_t, 46 as int8_t],
        [-1 as int8_t, 51 as int8_t],
        [7 as int8_t, 49 as int8_t],
        [8 as int8_t, 52 as int8_t],
        [9 as int8_t, 41 as int8_t],
        [6 as int8_t, 47 as int8_t],
        [2 as int8_t, 55 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [10 as int8_t, 44 as int8_t],
        [6 as int8_t, 50 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [13 as int8_t, 49 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 64 as int8_t],
        [-(2) as int8_t, 69 as int8_t],
        [-(2) as int8_t, 59 as int8_t],
        [6 as int8_t, 70 as int8_t],
        [10 as int8_t, 44 as int8_t],
        [9 as int8_t, 31 as int8_t],
        [12 as int8_t, 43 as int8_t],
        [3 as int8_t, 53 as int8_t],
        [14 as int8_t, 34 as int8_t],
        [10 as int8_t, 38 as int8_t],
        [-(3) as int8_t, 52 as int8_t],
        [13 as int8_t, 40 as int8_t],
        [17 as int8_t, 32 as int8_t],
        [7 as int8_t, 44 as int8_t],
        [7 as int8_t, 38 as int8_t],
        [13 as int8_t, 50 as int8_t],
        [10 as int8_t, 57 as int8_t],
        [26 as int8_t, 43 as int8_t],
        [14 as int8_t, 11 as int8_t],
        [11 as int8_t, 14 as int8_t],
        [9 as int8_t, 11 as int8_t],
        [18 as int8_t, 11 as int8_t],
        [21 as int8_t, 9 as int8_t],
        [23 as int8_t, -(2) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [34 as int8_t, -(21) as int8_t],
        [39 as int8_t, -(23) as int8_t],
        [42 as int8_t, -(33) as int8_t],
        [41 as int8_t, -(31) as int8_t],
        [46 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [21 as int8_t, 29 as int8_t],
        [45 as int8_t, -(24) as int8_t],
        [53 as int8_t, -(45) as int8_t],
        [48 as int8_t, -(26) as int8_t],
        [65 as int8_t, -(43) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [39 as int8_t, -(10) as int8_t],
        [30 as int8_t, 9 as int8_t],
        [18 as int8_t, 26 as int8_t],
        [20 as int8_t, 27 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(14) as int8_t, 82 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(19) as int8_t, 97 as int8_t],
        [-(35) as int8_t, 125 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [27 as int8_t, 6 as int8_t],
        [34 as int8_t, 8 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [24 as int8_t, 22 as int8_t],
        [33 as int8_t, 19 as int8_t],
        [22 as int8_t, 32 as int8_t],
        [26 as int8_t, 31 as int8_t],
        [21 as int8_t, 41 as int8_t],
        [26 as int8_t, 44 as int8_t],
        [23 as int8_t, 47 as int8_t],
        [16 as int8_t, 65 as int8_t],
        [14 as int8_t, 71 as int8_t],
        [8 as int8_t, 60 as int8_t],
        [6 as int8_t, 63 as int8_t],
        [17 as int8_t, 65 as int8_t],
        [21 as int8_t, 24 as int8_t],
        [23 as int8_t, 20 as int8_t],
        [26 as int8_t, 23 as int8_t],
        [27 as int8_t, 32 as int8_t],
        [28 as int8_t, 23 as int8_t],
        [28 as int8_t, 24 as int8_t],
        [23 as int8_t, 40 as int8_t],
        [24 as int8_t, 32 as int8_t],
        [28 as int8_t, 29 as int8_t],
        [23 as int8_t, 42 as int8_t],
        [19 as int8_t, 57 as int8_t],
        [22 as int8_t, 53 as int8_t],
        [22 as int8_t, 61 as int8_t],
        [11 as int8_t, 86 as int8_t],
        [12 as int8_t, 40 as int8_t],
        [11 as int8_t, 51 as int8_t],
        [14 as int8_t, 59 as int8_t],
        [-(4) as int8_t, 79 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(5) as int8_t, 69 as int8_t],
        [-(9) as int8_t, 70 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(10) as int8_t, 68 as int8_t],
        [-(19) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 69 as int8_t],
        [-(16) as int8_t, 70 as int8_t],
        [-(15) as int8_t, 67 as int8_t],
        [-(20) as int8_t, 62 as int8_t],
        [-(19) as int8_t, 70 as int8_t],
        [-(16) as int8_t, 66 as int8_t],
        [-(22) as int8_t, 65 as int8_t],
        [-(20) as int8_t, 63 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [26 as int8_t, -(9) as int8_t],
        [33 as int8_t, -(9) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [41 as int8_t, -(2) as int8_t],
        [45 as int8_t, 3 as int8_t],
        [49 as int8_t, 9 as int8_t],
        [45 as int8_t, 27 as int8_t],
        [36 as int8_t, 59 as int8_t],
        [-(6) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 35 as int8_t],
        [-(7) as int8_t, 42 as int8_t],
        [-(8) as int8_t, 45 as int8_t],
        [-(5) as int8_t, 48 as int8_t],
        [-(12) as int8_t, 56 as int8_t],
        [-(6) as int8_t, 60 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [21 as int8_t, -(13) as int8_t],
        [33 as int8_t, -(14) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [46 as int8_t, -(2) as int8_t],
        [51 as int8_t, 2 as int8_t],
        [60 as int8_t, 6 as int8_t],
        [61 as int8_t, 17 as int8_t],
        [55 as int8_t, 34 as int8_t],
        [42 as int8_t, 62 as int8_t],
        [-(7) as int8_t, 92 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 96 as int8_t],
        [-(13) as int8_t, 108 as int8_t],
        [-(3) as int8_t, 46 as int8_t],
        [-1 as int8_t, 65 as int8_t],
        [-1 as int8_t, 57 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
        [-(7) as int8_t, 92 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 96 as int8_t],
        [-(13) as int8_t, 108 as int8_t],
        [-(3) as int8_t, 46 as int8_t],
        [-1 as int8_t, 65 as int8_t],
        [-1 as int8_t, 57 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
        [-(2) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-1 as int8_t, 75 as int8_t],
        [-(7) as int8_t, 77 as int8_t],
        [2 as int8_t, 54 as int8_t],
        [5 as int8_t, 50 as int8_t],
        [-(3) as int8_t, 68 as int8_t],
        [1 as int8_t, 50 as int8_t],
        [6 as int8_t, 42 as int8_t],
        [-(4) as int8_t, 81 as int8_t],
        [1 as int8_t, 63 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(2) as int8_t, 76 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [1 as int8_t, 61 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [18 as int8_t, 25 as int8_t],
        [12 as int8_t, 24 as int8_t],
        [13 as int8_t, 29 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [-(10) as int8_t, 93 as int8_t],
        [-(7) as int8_t, 73 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [13 as int8_t, 46 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [-(7) as int8_t, 100 as int8_t],
        [9 as int8_t, 53 as int8_t],
        [2 as int8_t, 53 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [-(2) as int8_t, 61 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [-(13) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 60 as int8_t],
        [-1 as int8_t, 62 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(6) as int8_t, 69 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [14 as int8_t, 39 as int8_t],
        [4 as int8_t, 51 as int8_t],
        [13 as int8_t, 68 as int8_t],
        [-(2) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-1 as int8_t, 75 as int8_t],
        [-(7) as int8_t, 77 as int8_t],
        [2 as int8_t, 54 as int8_t],
        [5 as int8_t, 50 as int8_t],
        [-(3) as int8_t, 68 as int8_t],
        [1 as int8_t, 50 as int8_t],
        [6 as int8_t, 42 as int8_t],
        [-(4) as int8_t, 81 as int8_t],
        [1 as int8_t, 63 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(2) as int8_t, 76 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [1 as int8_t, 61 as int8_t],
        [11 as int8_t, 35 as int8_t],
        [18 as int8_t, 25 as int8_t],
        [12 as int8_t, 24 as int8_t],
        [13 as int8_t, 29 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [-(10) as int8_t, 93 as int8_t],
        [-(7) as int8_t, 73 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [13 as int8_t, 46 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [-(7) as int8_t, 100 as int8_t],
        [9 as int8_t, 53 as int8_t],
        [2 as int8_t, 53 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [-(2) as int8_t, 61 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [0 as int8_t, 56 as int8_t],
        [-(13) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 60 as int8_t],
        [-1 as int8_t, 62 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(6) as int8_t, 69 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [14 as int8_t, 39 as int8_t],
        [4 as int8_t, 51 as int8_t],
        [13 as int8_t, 68 as int8_t],
        [11 as int8_t, 28 as int8_t],
        [2 as int8_t, 40 as int8_t],
        [3 as int8_t, 44 as int8_t],
        [0 as int8_t, 49 as int8_t],
        [0 as int8_t, 46 as int8_t],
        [2 as int8_t, 44 as int8_t],
        [2 as int8_t, 51 as int8_t],
        [0 as int8_t, 47 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [2 as int8_t, 62 as int8_t],
        [6 as int8_t, 46 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [3 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 51 as int8_t],
        [6 as int8_t, 57 as int8_t],
        [7 as int8_t, 53 as int8_t],
        [6 as int8_t, 52 as int8_t],
        [6 as int8_t, 55 as int8_t],
        [11 as int8_t, 45 as int8_t],
        [14 as int8_t, 36 as int8_t],
        [8 as int8_t, 53 as int8_t],
        [-1 as int8_t, 82 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [15 as int8_t, 46 as int8_t],
        [22 as int8_t, 31 as int8_t],
        [-1 as int8_t, 84 as int8_t],
        [25 as int8_t, 7 as int8_t],
        [30 as int8_t, -(7) as int8_t],
        [28 as int8_t, 3 as int8_t],
        [28 as int8_t, 4 as int8_t],
        [32 as int8_t, 0 as int8_t],
        [34 as int8_t, -1 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [32 as int8_t, 9 as int8_t],
        [31 as int8_t, 19 as int8_t],
        [26 as int8_t, 27 as int8_t],
        [26 as int8_t, 30 as int8_t],
        [37 as int8_t, 20 as int8_t],
        [28 as int8_t, 34 as int8_t],
        [17 as int8_t, 70 as int8_t],
        [11 as int8_t, 28 as int8_t],
        [2 as int8_t, 40 as int8_t],
        [3 as int8_t, 44 as int8_t],
        [0 as int8_t, 49 as int8_t],
        [0 as int8_t, 46 as int8_t],
        [2 as int8_t, 44 as int8_t],
        [2 as int8_t, 51 as int8_t],
        [0 as int8_t, 47 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [2 as int8_t, 62 as int8_t],
        [6 as int8_t, 46 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [3 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 51 as int8_t],
        [6 as int8_t, 57 as int8_t],
        [7 as int8_t, 53 as int8_t],
        [6 as int8_t, 52 as int8_t],
        [6 as int8_t, 55 as int8_t],
        [11 as int8_t, 45 as int8_t],
        [14 as int8_t, 36 as int8_t],
        [8 as int8_t, 53 as int8_t],
        [-1 as int8_t, 82 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [15 as int8_t, 46 as int8_t],
        [22 as int8_t, 31 as int8_t],
        [-1 as int8_t, 84 as int8_t],
        [25 as int8_t, 7 as int8_t],
        [30 as int8_t, -(7) as int8_t],
        [28 as int8_t, 3 as int8_t],
        [28 as int8_t, 4 as int8_t],
        [32 as int8_t, 0 as int8_t],
        [34 as int8_t, -1 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [30 as int8_t, 6 as int8_t],
        [32 as int8_t, 9 as int8_t],
        [31 as int8_t, 19 as int8_t],
        [26 as int8_t, 27 as int8_t],
        [26 as int8_t, 30 as int8_t],
        [37 as int8_t, 20 as int8_t],
        [28 as int8_t, 34 as int8_t],
        [17 as int8_t, 70 as int8_t],
        [-(4) as int8_t, 79 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(5) as int8_t, 69 as int8_t],
        [-(9) as int8_t, 70 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(10) as int8_t, 68 as int8_t],
        [-(19) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 69 as int8_t],
        [-(16) as int8_t, 70 as int8_t],
        [-(15) as int8_t, 67 as int8_t],
        [-(20) as int8_t, 62 as int8_t],
        [-(19) as int8_t, 70 as int8_t],
        [-(16) as int8_t, 66 as int8_t],
        [-(22) as int8_t, 65 as int8_t],
        [-(20) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [26 as int8_t, -(9) as int8_t],
        [33 as int8_t, -(9) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [41 as int8_t, -(2) as int8_t],
        [45 as int8_t, 3 as int8_t],
        [49 as int8_t, 9 as int8_t],
        [45 as int8_t, 27 as int8_t],
        [36 as int8_t, 59 as int8_t],
        [21 as int8_t, -(13) as int8_t],
        [33 as int8_t, -(14) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [46 as int8_t, -(2) as int8_t],
        [51 as int8_t, 2 as int8_t],
        [60 as int8_t, 6 as int8_t],
        [61 as int8_t, 17 as int8_t],
        [55 as int8_t, 34 as int8_t],
        [42 as int8_t, 62 as int8_t],
        [-(6) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 35 as int8_t],
        [-(7) as int8_t, 42 as int8_t],
        [-(8) as int8_t, 45 as int8_t],
        [-(5) as int8_t, 48 as int8_t],
        [-(12) as int8_t, 56 as int8_t],
        [-(6) as int8_t, 60 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(4) as int8_t, 79 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(5) as int8_t, 69 as int8_t],
        [-(9) as int8_t, 70 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(10) as int8_t, 68 as int8_t],
        [-(19) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 69 as int8_t],
        [-(16) as int8_t, 70 as int8_t],
        [-(15) as int8_t, 67 as int8_t],
        [-(20) as int8_t, 62 as int8_t],
        [-(19) as int8_t, 70 as int8_t],
        [-(16) as int8_t, 66 as int8_t],
        [-(22) as int8_t, 65 as int8_t],
        [-(20) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [26 as int8_t, -(9) as int8_t],
        [33 as int8_t, -(9) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [41 as int8_t, -(2) as int8_t],
        [45 as int8_t, 3 as int8_t],
        [49 as int8_t, 9 as int8_t],
        [45 as int8_t, 27 as int8_t],
        [36 as int8_t, 59 as int8_t],
        [21 as int8_t, -(13) as int8_t],
        [33 as int8_t, -(14) as int8_t],
        [39 as int8_t, -(7) as int8_t],
        [46 as int8_t, -(2) as int8_t],
        [51 as int8_t, 2 as int8_t],
        [60 as int8_t, 6 as int8_t],
        [61 as int8_t, 17 as int8_t],
        [55 as int8_t, 34 as int8_t],
        [42 as int8_t, 62 as int8_t],
        [-(6) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 35 as int8_t],
        [-(7) as int8_t, 42 as int8_t],
        [-(8) as int8_t, 45 as int8_t],
        [-(5) as int8_t, 48 as int8_t],
        [-(12) as int8_t, 56 as int8_t],
        [-(6) as int8_t, 60 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(16) as int8_t, 106 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(21) as int8_t, 114 as int8_t],
        [-(18) as int8_t, 110 as int8_t],
        [-(14) as int8_t, 98 as int8_t],
        [-(22) as int8_t, 110 as int8_t],
        [-(21) as int8_t, 106 as int8_t],
        [-(18) as int8_t, 103 as int8_t],
        [-(21) as int8_t, 107 as int8_t],
        [-(23) as int8_t, 108 as int8_t],
        [-(26) as int8_t, 112 as int8_t],
        [-(10) as int8_t, 96 as int8_t],
        [-(12) as int8_t, 95 as int8_t],
        [-(5) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(22) as int8_t, 94 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [9 as int8_t, 67 as int8_t],
        [-(4) as int8_t, 80 as int8_t],
        [-(10) as int8_t, 85 as int8_t],
        [-1 as int8_t, 70 as int8_t],
        [7 as int8_t, 60 as int8_t],
        [9 as int8_t, 58 as int8_t],
        [5 as int8_t, 61 as int8_t],
        [12 as int8_t, 50 as int8_t],
        [15 as int8_t, 50 as int8_t],
        [18 as int8_t, 49 as int8_t],
        [17 as int8_t, 54 as int8_t],
        [10 as int8_t, 41 as int8_t],
        [7 as int8_t, 46 as int8_t],
        [-1 as int8_t, 51 as int8_t],
        [7 as int8_t, 49 as int8_t],
        [8 as int8_t, 52 as int8_t],
        [9 as int8_t, 41 as int8_t],
        [6 as int8_t, 47 as int8_t],
        [2 as int8_t, 55 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [10 as int8_t, 44 as int8_t],
        [6 as int8_t, 50 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [13 as int8_t, 49 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 64 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(16) as int8_t, 106 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(21) as int8_t, 114 as int8_t],
        [-(18) as int8_t, 110 as int8_t],
        [-(14) as int8_t, 98 as int8_t],
        [-(22) as int8_t, 110 as int8_t],
        [-(21) as int8_t, 106 as int8_t],
        [-(18) as int8_t, 103 as int8_t],
        [-(21) as int8_t, 107 as int8_t],
        [-(23) as int8_t, 108 as int8_t],
        [-(26) as int8_t, 112 as int8_t],
        [-(10) as int8_t, 96 as int8_t],
        [-(12) as int8_t, 95 as int8_t],
        [-(5) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 93 as int8_t],
        [-(22) as int8_t, 94 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [9 as int8_t, 67 as int8_t],
        [-(4) as int8_t, 80 as int8_t],
        [-(10) as int8_t, 85 as int8_t],
        [-1 as int8_t, 70 as int8_t],
        [7 as int8_t, 60 as int8_t],
        [9 as int8_t, 58 as int8_t],
        [5 as int8_t, 61 as int8_t],
        [12 as int8_t, 50 as int8_t],
        [15 as int8_t, 50 as int8_t],
        [18 as int8_t, 49 as int8_t],
        [17 as int8_t, 54 as int8_t],
        [10 as int8_t, 41 as int8_t],
        [7 as int8_t, 46 as int8_t],
        [-1 as int8_t, 51 as int8_t],
        [7 as int8_t, 49 as int8_t],
        [8 as int8_t, 52 as int8_t],
        [9 as int8_t, 41 as int8_t],
        [6 as int8_t, 47 as int8_t],
        [2 as int8_t, 55 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [10 as int8_t, 44 as int8_t],
        [6 as int8_t, 50 as int8_t],
        [5 as int8_t, 53 as int8_t],
        [13 as int8_t, 49 as int8_t],
        [4 as int8_t, 63 as int8_t],
        [6 as int8_t, 64 as int8_t],
        [14 as int8_t, 11 as int8_t],
        [11 as int8_t, 14 as int8_t],
        [9 as int8_t, 11 as int8_t],
        [18 as int8_t, 11 as int8_t],
        [21 as int8_t, 9 as int8_t],
        [23 as int8_t, -(2) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [34 as int8_t, -(21) as int8_t],
        [39 as int8_t, -(23) as int8_t],
        [42 as int8_t, -(33) as int8_t],
        [41 as int8_t, -(31) as int8_t],
        [46 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [21 as int8_t, 29 as int8_t],
        [45 as int8_t, -(24) as int8_t],
        [53 as int8_t, -(45) as int8_t],
        [48 as int8_t, -(26) as int8_t],
        [65 as int8_t, -(43) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [39 as int8_t, -(10) as int8_t],
        [30 as int8_t, 9 as int8_t],
        [18 as int8_t, 26 as int8_t],
        [20 as int8_t, 27 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(14) as int8_t, 82 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(19) as int8_t, 97 as int8_t],
        [-(35) as int8_t, 125 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [27 as int8_t, 6 as int8_t],
        [34 as int8_t, 8 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [24 as int8_t, 22 as int8_t],
        [33 as int8_t, 19 as int8_t],
        [22 as int8_t, 32 as int8_t],
        [26 as int8_t, 31 as int8_t],
        [21 as int8_t, 41 as int8_t],
        [26 as int8_t, 44 as int8_t],
        [23 as int8_t, 47 as int8_t],
        [16 as int8_t, 65 as int8_t],
        [14 as int8_t, 71 as int8_t],
        [14 as int8_t, 11 as int8_t],
        [11 as int8_t, 14 as int8_t],
        [9 as int8_t, 11 as int8_t],
        [18 as int8_t, 11 as int8_t],
        [21 as int8_t, 9 as int8_t],
        [23 as int8_t, -(2) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [32 as int8_t, -(15) as int8_t],
        [34 as int8_t, -(21) as int8_t],
        [39 as int8_t, -(23) as int8_t],
        [42 as int8_t, -(33) as int8_t],
        [41 as int8_t, -(31) as int8_t],
        [46 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [21 as int8_t, 29 as int8_t],
        [45 as int8_t, -(24) as int8_t],
        [53 as int8_t, -(45) as int8_t],
        [48 as int8_t, -(26) as int8_t],
        [65 as int8_t, -(43) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [39 as int8_t, -(10) as int8_t],
        [30 as int8_t, 9 as int8_t],
        [18 as int8_t, 26 as int8_t],
        [20 as int8_t, 27 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(14) as int8_t, 82 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(19) as int8_t, 97 as int8_t],
        [-(35) as int8_t, 125 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [27 as int8_t, 6 as int8_t],
        [34 as int8_t, 8 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [24 as int8_t, 22 as int8_t],
        [33 as int8_t, 19 as int8_t],
        [22 as int8_t, 32 as int8_t],
        [26 as int8_t, 31 as int8_t],
        [21 as int8_t, 41 as int8_t],
        [26 as int8_t, 44 as int8_t],
        [23 as int8_t, 47 as int8_t],
        [16 as int8_t, 65 as int8_t],
        [14 as int8_t, 71 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(2) as int8_t, 44 as int8_t],
        [0 as int8_t, 45 as int8_t],
        [0 as int8_t, 52 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [-(2) as int8_t, 59 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [-(4) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 82 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(9) as int8_t, 77 as int8_t],
        [3 as int8_t, 24 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [0 as int8_t, 48 as int8_t],
        [0 as int8_t, 55 as int8_t],
        [-(6) as int8_t, 59 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(12) as int8_t, 83 as int8_t],
        [-(11) as int8_t, 87 as int8_t],
        [-(30) as int8_t, 119 as int8_t],
        [1 as int8_t, 58 as int8_t],
        [-(3) as int8_t, 29 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [1 as int8_t, 38 as int8_t],
        [2 as int8_t, 43 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [0 as int8_t, 64 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 90 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(2) as int8_t, 44 as int8_t],
        [0 as int8_t, 45 as int8_t],
        [0 as int8_t, 52 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [-(2) as int8_t, 59 as int8_t],
        [-(4) as int8_t, 70 as int8_t],
        [-(4) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 82 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(9) as int8_t, 77 as int8_t],
        [3 as int8_t, 24 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [0 as int8_t, 48 as int8_t],
        [0 as int8_t, 55 as int8_t],
        [-(6) as int8_t, 59 as int8_t],
        [-(7) as int8_t, 71 as int8_t],
        [-(12) as int8_t, 83 as int8_t],
        [-(11) as int8_t, 87 as int8_t],
        [-(30) as int8_t, 119 as int8_t],
        [1 as int8_t, 58 as int8_t],
        [-(3) as int8_t, 29 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [1 as int8_t, 38 as int8_t],
        [2 as int8_t, 43 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [0 as int8_t, 64 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 90 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 126 as int8_t],
    ],
    [
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [-(28) as int8_t, 127 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [-1 as int8_t, 54 as int8_t],
        [7 as int8_t, 51 as int8_t],
        [22 as int8_t, 25 as int8_t],
        [34 as int8_t, 0 as int8_t],
        [16 as int8_t, 0 as int8_t],
        [-(2) as int8_t, 9 as int8_t],
        [4 as int8_t, 41 as int8_t],
        [-(29) as int8_t, 118 as int8_t],
        [2 as int8_t, 65 as int8_t],
        [-(6) as int8_t, 71 as int8_t],
        [-(13) as int8_t, 79 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [9 as int8_t, 50 as int8_t],
        [-(3) as int8_t, 70 as int8_t],
        [10 as int8_t, 54 as int8_t],
        [26 as int8_t, 34 as int8_t],
        [19 as int8_t, 22 as int8_t],
        [40 as int8_t, 0 as int8_t],
        [57 as int8_t, 2 as int8_t],
        [41 as int8_t, 36 as int8_t],
        [26 as int8_t, 69 as int8_t],
        [-(45) as int8_t, 127 as int8_t],
        [-(15) as int8_t, 101 as int8_t],
        [-(4) as int8_t, 76 as int8_t],
        [-(6) as int8_t, 71 as int8_t],
        [-(13) as int8_t, 79 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [6 as int8_t, 69 as int8_t],
        [-(13) as int8_t, 90 as int8_t],
        [0 as int8_t, 52 as int8_t],
        [8 as int8_t, 43 as int8_t],
        [-(2) as int8_t, 69 as int8_t],
        [-(5) as int8_t, 82 as int8_t],
        [-(10) as int8_t, 96 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [2 as int8_t, 75 as int8_t],
        [-(3) as int8_t, 87 as int8_t],
        [-(3) as int8_t, 100 as int8_t],
        [1 as int8_t, 56 as int8_t],
        [-(3) as int8_t, 74 as int8_t],
        [-(6) as int8_t, 85 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [-(7) as int8_t, 86 as int8_t],
        [-(5) as int8_t, 95 as int8_t],
        [-1 as int8_t, 66 as int8_t],
        [-1 as int8_t, 77 as int8_t],
        [1 as int8_t, 70 as int8_t],
        [-(2) as int8_t, 86 as int8_t],
        [-(5) as int8_t, 72 as int8_t],
        [0 as int8_t, 61 as int8_t],
        [0 as int8_t, 41 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [4 as int8_t, 86 as int8_t],
        [0 as int8_t, 97 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [3 as int8_t, 62 as int8_t],
        [13 as int8_t, 15 as int8_t],
        [7 as int8_t, 51 as int8_t],
        [2 as int8_t, 80 as int8_t],
        [-(39) as int8_t, 127 as int8_t],
        [-(18) as int8_t, 91 as int8_t],
        [-(17) as int8_t, 96 as int8_t],
        [-(26) as int8_t, 81 as int8_t],
        [-(35) as int8_t, 98 as int8_t],
        [-(24) as int8_t, 102 as int8_t],
        [-(23) as int8_t, 97 as int8_t],
        [-(27) as int8_t, 119 as int8_t],
        [-(24) as int8_t, 99 as int8_t],
        [-(21) as int8_t, 110 as int8_t],
        [-(18) as int8_t, 102 as int8_t],
        [-(36) as int8_t, 127 as int8_t],
        [0 as int8_t, 80 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 94 as int8_t],
        [-(4) as int8_t, 92 as int8_t],
        [0 as int8_t, 39 as int8_t],
        [0 as int8_t, 65 as int8_t],
        [-(15) as int8_t, 84 as int8_t],
        [-(35) as int8_t, 127 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [3 as int8_t, 55 as int8_t],
        [7 as int8_t, 56 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [8 as int8_t, 61 as int8_t],
        [-(3) as int8_t, 53 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(7) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 88 as int8_t],
        [-(13) as int8_t, 103 as int8_t],
        [-(13) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 89 as int8_t],
        [-(14) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(12) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 110 as int8_t],
        [-(24) as int8_t, 105 as int8_t],
        [-(10) as int8_t, 78 as int8_t],
        [-(20) as int8_t, 112 as int8_t],
        [-(17) as int8_t, 99 as int8_t],
        [-(78) as int8_t, 127 as int8_t],
        [-(70) as int8_t, 127 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(46) as int8_t, 127 as int8_t],
        [-(4) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [-1 as int8_t, 55 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 89 as int8_t],
        [-(34) as int8_t, 119 as int8_t],
        [-(3) as int8_t, 75 as int8_t],
        [32 as int8_t, 20 as int8_t],
        [30 as int8_t, 22 as int8_t],
        [-(44) as int8_t, 127 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [-(5) as int8_t, 61 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [-1 as int8_t, 60 as int8_t],
        [-(3) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 67 as int8_t],
        [-(25) as int8_t, 84 as int8_t],
        [-(14) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 65 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [0 as int8_t, 61 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-(11) as int8_t, 70 as int8_t],
        [18 as int8_t, 55 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [7 as int8_t, 61 as int8_t],
        [9 as int8_t, 41 as int8_t],
        [18 as int8_t, 25 as int8_t],
        [9 as int8_t, 32 as int8_t],
        [5 as int8_t, 43 as int8_t],
        [9 as int8_t, 47 as int8_t],
        [0 as int8_t, 44 as int8_t],
        [0 as int8_t, 51 as int8_t],
        [2 as int8_t, 46 as int8_t],
        [19 as int8_t, 38 as int8_t],
        [-(4) as int8_t, 66 as int8_t],
        [15 as int8_t, 38 as int8_t],
        [12 as int8_t, 42 as int8_t],
        [9 as int8_t, 34 as int8_t],
        [0 as int8_t, 89 as int8_t],
        [4 as int8_t, 45 as int8_t],
        [10 as int8_t, 28 as int8_t],
        [10 as int8_t, 31 as int8_t],
        [33 as int8_t, -(11) as int8_t],
        [52 as int8_t, -(43) as int8_t],
        [18 as int8_t, 15 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [35 as int8_t, -(22) as int8_t],
        [38 as int8_t, -(25) as int8_t],
        [34 as int8_t, 0 as int8_t],
        [39 as int8_t, -(18) as int8_t],
        [32 as int8_t, -(12) as int8_t],
        [102 as int8_t, -(94) as int8_t],
        [0 as int8_t, 0 as int8_t],
        [56 as int8_t, -(15) as int8_t],
        [33 as int8_t, -(4) as int8_t],
        [29 as int8_t, 10 as int8_t],
        [37 as int8_t, -(5) as int8_t],
        [51 as int8_t, -(29) as int8_t],
        [39 as int8_t, -(9) as int8_t],
        [52 as int8_t, -(34) as int8_t],
        [69 as int8_t, -(58) as int8_t],
        [67 as int8_t, -(63) as int8_t],
        [44 as int8_t, -(5) as int8_t],
        [32 as int8_t, 7 as int8_t],
        [55 as int8_t, -(29) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [27 as int8_t, 36 as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [34 as int8_t, -(30) as int8_t],
        [36 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(27) as int8_t],
        [34 as int8_t, -(18) as int8_t],
        [35 as int8_t, -(16) as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [32 as int8_t, -(8) as int8_t],
        [37 as int8_t, -(6) as int8_t],
        [35 as int8_t, 0 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [28 as int8_t, 18 as int8_t],
        [26 as int8_t, 25 as int8_t],
        [29 as int8_t, 41 as int8_t],
        [0 as int8_t, 75 as int8_t],
        [2 as int8_t, 72 as int8_t],
        [8 as int8_t, 77 as int8_t],
        [14 as int8_t, 35 as int8_t],
        [18 as int8_t, 31 as int8_t],
        [17 as int8_t, 35 as int8_t],
        [21 as int8_t, 30 as int8_t],
        [17 as int8_t, 45 as int8_t],
        [20 as int8_t, 42 as int8_t],
        [18 as int8_t, 45 as int8_t],
        [27 as int8_t, 26 as int8_t],
        [16 as int8_t, 54 as int8_t],
        [7 as int8_t, 66 as int8_t],
        [16 as int8_t, 56 as int8_t],
        [11 as int8_t, 73 as int8_t],
        [10 as int8_t, 67 as int8_t],
        [-(10) as int8_t, 116 as int8_t],
        [-(23) as int8_t, 112 as int8_t],
        [-(15) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(11) as int8_t, 77 as int8_t],
        [-(9) as int8_t, 80 as int8_t],
        [-(9) as int8_t, 84 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(34) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 101 as int8_t],
        [-(3) as int8_t, 39 as int8_t],
        [-(5) as int8_t, 53 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [-(11) as int8_t, 75 as int8_t],
        [-(15) as int8_t, 77 as int8_t],
        [-(17) as int8_t, 91 as int8_t],
        [-(25) as int8_t, 107 as int8_t],
        [-(25) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 122 as int8_t],
        [-(11) as int8_t, 76 as int8_t],
        [-(10) as int8_t, 44 as int8_t],
        [-(10) as int8_t, 52 as int8_t],
        [-(10) as int8_t, 57 as int8_t],
        [-(9) as int8_t, 58 as int8_t],
        [-(16) as int8_t, 72 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(4) as int8_t, 69 as int8_t],
        [-(5) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 86 as int8_t],
        [2 as int8_t, 66 as int8_t],
        [-(9) as int8_t, 34 as int8_t],
        [1 as int8_t, 32 as int8_t],
        [11 as int8_t, 31 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [-(2) as int8_t, 55 as int8_t],
        [-(2) as int8_t, 67 as int8_t],
        [0 as int8_t, 73 as int8_t],
        [-(8) as int8_t, 89 as int8_t],
        [3 as int8_t, 52 as int8_t],
        [7 as int8_t, 4 as int8_t],
        [10 as int8_t, 8 as int8_t],
        [17 as int8_t, 8 as int8_t],
        [16 as int8_t, 19 as int8_t],
        [3 as int8_t, 37 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(5) as int8_t, 73 as int8_t],
        [-1 as int8_t, 70 as int8_t],
        [-(4) as int8_t, 78 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [-(21) as int8_t, 126 as int8_t],
        [-(23) as int8_t, 124 as int8_t],
        [-(20) as int8_t, 110 as int8_t],
        [-(26) as int8_t, 126 as int8_t],
        [-(25) as int8_t, 124 as int8_t],
        [-(17) as int8_t, 105 as int8_t],
        [-(27) as int8_t, 121 as int8_t],
        [-(27) as int8_t, 117 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(27) as int8_t, 116 as int8_t],
        [-(33) as int8_t, 122 as int8_t],
        [-(10) as int8_t, 95 as int8_t],
        [-(14) as int8_t, 100 as int8_t],
        [-(8) as int8_t, 95 as int8_t],
        [-(17) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 114 as int8_t],
        [-(6) as int8_t, 89 as int8_t],
        [-(2) as int8_t, 80 as int8_t],
        [-(4) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 85 as int8_t],
        [-(8) as int8_t, 81 as int8_t],
        [-1 as int8_t, 72 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [9 as int8_t, 56 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [1 as int8_t, 69 as int8_t],
        [7 as int8_t, 69 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(16) as int8_t, 77 as int8_t],
        [-(2) as int8_t, 64 as int8_t],
        [2 as int8_t, 61 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(3) as int8_t, 65 as int8_t],
        [-(3) as int8_t, 66 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [9 as int8_t, 51 as int8_t],
        [-1 as int8_t, 66 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 75 as int8_t],
        [-1 as int8_t, 70 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [14 as int8_t, 60 as int8_t],
        [16 as int8_t, 37 as int8_t],
        [0 as int8_t, 47 as int8_t],
        [18 as int8_t, 35 as int8_t],
        [11 as int8_t, 37 as int8_t],
        [12 as int8_t, 41 as int8_t],
        [10 as int8_t, 41 as int8_t],
        [2 as int8_t, 48 as int8_t],
        [12 as int8_t, 41 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [3 as int8_t, 50 as int8_t],
        [19 as int8_t, 40 as int8_t],
        [3 as int8_t, 66 as int8_t],
        [18 as int8_t, 50 as int8_t],
        [19 as int8_t, -(6) as int8_t],
        [18 as int8_t, -(6) as int8_t],
        [14 as int8_t, 0 as int8_t],
        [26 as int8_t, -(12) as int8_t],
        [31 as int8_t, -(16) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [33 as int8_t, -(22) as int8_t],
        [37 as int8_t, -(28) as int8_t],
        [39 as int8_t, -(30) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [47 as int8_t, -(42) as int8_t],
        [45 as int8_t, -(36) as int8_t],
        [49 as int8_t, -(34) as int8_t],
        [41 as int8_t, -(17) as int8_t],
        [32 as int8_t, 9 as int8_t],
        [69 as int8_t, -(71) as int8_t],
        [63 as int8_t, -(63) as int8_t],
        [66 as int8_t, -(64) as int8_t],
        [77 as int8_t, -(74) as int8_t],
        [54 as int8_t, -(39) as int8_t],
        [52 as int8_t, -(35) as int8_t],
        [41 as int8_t, -(10) as int8_t],
        [36 as int8_t, 0 as int8_t],
        [40 as int8_t, -1 as int8_t],
        [30 as int8_t, 14 as int8_t],
        [28 as int8_t, 26 as int8_t],
        [23 as int8_t, 37 as int8_t],
        [12 as int8_t, 55 as int8_t],
        [11 as int8_t, 65 as int8_t],
        [37 as int8_t, -(33) as int8_t],
        [39 as int8_t, -(36) as int8_t],
        [40 as int8_t, -(37) as int8_t],
        [38 as int8_t, -(30) as int8_t],
        [46 as int8_t, -(33) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [40 as int8_t, -(24) as int8_t],
        [49 as int8_t, -(29) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [40 as int8_t, -(10) as int8_t],
        [38 as int8_t, -(3) as int8_t],
        [46 as int8_t, -(5) as int8_t],
        [31 as int8_t, 20 as int8_t],
        [29 as int8_t, 30 as int8_t],
        [25 as int8_t, 44 as int8_t],
        [12 as int8_t, 48 as int8_t],
        [11 as int8_t, 49 as int8_t],
        [26 as int8_t, 45 as int8_t],
        [22 as int8_t, 22 as int8_t],
        [23 as int8_t, 22 as int8_t],
        [27 as int8_t, 21 as int8_t],
        [33 as int8_t, 20 as int8_t],
        [26 as int8_t, 28 as int8_t],
        [30 as int8_t, 24 as int8_t],
        [27 as int8_t, 34 as int8_t],
        [18 as int8_t, 42 as int8_t],
        [25 as int8_t, 39 as int8_t],
        [18 as int8_t, 50 as int8_t],
        [12 as int8_t, 70 as int8_t],
        [21 as int8_t, 54 as int8_t],
        [14 as int8_t, 71 as int8_t],
        [11 as int8_t, 83 as int8_t],
        [25 as int8_t, 32 as int8_t],
        [21 as int8_t, 49 as int8_t],
        [21 as int8_t, 54 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 71 as int8_t],
        [0 as int8_t, 24 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [-(2) as int8_t, 42 as int8_t],
        [-(2) as int8_t, 52 as int8_t],
        [-(9) as int8_t, 57 as int8_t],
        [-(6) as int8_t, 63 as int8_t],
        [-(4) as int8_t, 65 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(7) as int8_t, 82 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [-(3) as int8_t, 76 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-(12) as int8_t, 72 as int8_t],
        [-(14) as int8_t, 68 as int8_t],
        [-(3) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 61 as int8_t],
        [-(9) as int8_t, 60 as int8_t],
        [1 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [0 as int8_t, 80 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 94 as int8_t],
        [-(4) as int8_t, 92 as int8_t],
        [0 as int8_t, 39 as int8_t],
        [0 as int8_t, 65 as int8_t],
        [-(15) as int8_t, 84 as int8_t],
        [-(35) as int8_t, 127 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [0 as int8_t, 80 as int8_t],
        [-(5) as int8_t, 89 as int8_t],
        [-(7) as int8_t, 94 as int8_t],
        [-(4) as int8_t, 92 as int8_t],
        [0 as int8_t, 39 as int8_t],
        [0 as int8_t, 65 as int8_t],
        [-(15) as int8_t, 84 as int8_t],
        [-(35) as int8_t, 127 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [-(13) as int8_t, 103 as int8_t],
        [-(13) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 89 as int8_t],
        [-(14) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(12) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 110 as int8_t],
        [-(24) as int8_t, 105 as int8_t],
        [-(10) as int8_t, 78 as int8_t],
        [-(20) as int8_t, 112 as int8_t],
        [-(17) as int8_t, 99 as int8_t],
        [-(78) as int8_t, 127 as int8_t],
        [-(70) as int8_t, 127 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(46) as int8_t, 127 as int8_t],
        [-(4) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [-1 as int8_t, 55 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 89 as int8_t],
        [-(34) as int8_t, 119 as int8_t],
        [-(3) as int8_t, 75 as int8_t],
        [32 as int8_t, 20 as int8_t],
        [30 as int8_t, 22 as int8_t],
        [-(44) as int8_t, 127 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [-(5) as int8_t, 61 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [-1 as int8_t, 60 as int8_t],
        [-(3) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 67 as int8_t],
        [-(25) as int8_t, 84 as int8_t],
        [-(14) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 65 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [0 as int8_t, 61 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-(11) as int8_t, 70 as int8_t],
        [18 as int8_t, 55 as int8_t],
        [-(13) as int8_t, 103 as int8_t],
        [-(13) as int8_t, 91 as int8_t],
        [-(9) as int8_t, 89 as int8_t],
        [-(14) as int8_t, 92 as int8_t],
        [-(8) as int8_t, 76 as int8_t],
        [-(12) as int8_t, 87 as int8_t],
        [-(23) as int8_t, 110 as int8_t],
        [-(24) as int8_t, 105 as int8_t],
        [-(10) as int8_t, 78 as int8_t],
        [-(20) as int8_t, 112 as int8_t],
        [-(17) as int8_t, 99 as int8_t],
        [-(78) as int8_t, 127 as int8_t],
        [-(70) as int8_t, 127 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(46) as int8_t, 127 as int8_t],
        [-(4) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [-1 as int8_t, 55 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 89 as int8_t],
        [-(34) as int8_t, 119 as int8_t],
        [-(3) as int8_t, 75 as int8_t],
        [32 as int8_t, 20 as int8_t],
        [30 as int8_t, 22 as int8_t],
        [-(44) as int8_t, 127 as int8_t],
        [0 as int8_t, 54 as int8_t],
        [-(5) as int8_t, 61 as int8_t],
        [0 as int8_t, 58 as int8_t],
        [-1 as int8_t, 60 as int8_t],
        [-(3) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 67 as int8_t],
        [-(25) as int8_t, 84 as int8_t],
        [-(14) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 65 as int8_t],
        [5 as int8_t, 52 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [0 as int8_t, 61 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-(11) as int8_t, 70 as int8_t],
        [18 as int8_t, 55 as int8_t],
        [4 as int8_t, 45 as int8_t],
        [10 as int8_t, 28 as int8_t],
        [10 as int8_t, 31 as int8_t],
        [33 as int8_t, -(11) as int8_t],
        [52 as int8_t, -(43) as int8_t],
        [18 as int8_t, 15 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [35 as int8_t, -(22) as int8_t],
        [38 as int8_t, -(25) as int8_t],
        [34 as int8_t, 0 as int8_t],
        [39 as int8_t, -(18) as int8_t],
        [32 as int8_t, -(12) as int8_t],
        [102 as int8_t, -(94) as int8_t],
        [0 as int8_t, 0 as int8_t],
        [56 as int8_t, -(15) as int8_t],
        [33 as int8_t, -(4) as int8_t],
        [29 as int8_t, 10 as int8_t],
        [37 as int8_t, -(5) as int8_t],
        [51 as int8_t, -(29) as int8_t],
        [39 as int8_t, -(9) as int8_t],
        [52 as int8_t, -(34) as int8_t],
        [69 as int8_t, -(58) as int8_t],
        [67 as int8_t, -(63) as int8_t],
        [44 as int8_t, -(5) as int8_t],
        [32 as int8_t, 7 as int8_t],
        [55 as int8_t, -(29) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [27 as int8_t, 36 as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [34 as int8_t, -(30) as int8_t],
        [36 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(27) as int8_t],
        [34 as int8_t, -(18) as int8_t],
        [35 as int8_t, -(16) as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [32 as int8_t, -(8) as int8_t],
        [37 as int8_t, -(6) as int8_t],
        [35 as int8_t, 0 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [28 as int8_t, 18 as int8_t],
        [26 as int8_t, 25 as int8_t],
        [29 as int8_t, 41 as int8_t],
        [4 as int8_t, 45 as int8_t],
        [10 as int8_t, 28 as int8_t],
        [10 as int8_t, 31 as int8_t],
        [33 as int8_t, -(11) as int8_t],
        [52 as int8_t, -(43) as int8_t],
        [18 as int8_t, 15 as int8_t],
        [28 as int8_t, 0 as int8_t],
        [35 as int8_t, -(22) as int8_t],
        [38 as int8_t, -(25) as int8_t],
        [34 as int8_t, 0 as int8_t],
        [39 as int8_t, -(18) as int8_t],
        [32 as int8_t, -(12) as int8_t],
        [102 as int8_t, -(94) as int8_t],
        [0 as int8_t, 0 as int8_t],
        [56 as int8_t, -(15) as int8_t],
        [33 as int8_t, -(4) as int8_t],
        [29 as int8_t, 10 as int8_t],
        [37 as int8_t, -(5) as int8_t],
        [51 as int8_t, -(29) as int8_t],
        [39 as int8_t, -(9) as int8_t],
        [52 as int8_t, -(34) as int8_t],
        [69 as int8_t, -(58) as int8_t],
        [67 as int8_t, -(63) as int8_t],
        [44 as int8_t, -(5) as int8_t],
        [32 as int8_t, 7 as int8_t],
        [55 as int8_t, -(29) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [27 as int8_t, 36 as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [34 as int8_t, -(30) as int8_t],
        [36 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(28) as int8_t],
        [38 as int8_t, -(27) as int8_t],
        [34 as int8_t, -(18) as int8_t],
        [35 as int8_t, -(16) as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [32 as int8_t, -(8) as int8_t],
        [37 as int8_t, -(6) as int8_t],
        [35 as int8_t, 0 as int8_t],
        [30 as int8_t, 10 as int8_t],
        [28 as int8_t, 18 as int8_t],
        [26 as int8_t, 25 as int8_t],
        [29 as int8_t, 41 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [-(3) as int8_t, 76 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-(12) as int8_t, 72 as int8_t],
        [-(14) as int8_t, 68 as int8_t],
        [-(3) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 61 as int8_t],
        [-(9) as int8_t, 60 as int8_t],
        [1 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 71 as int8_t],
        [0 as int8_t, 24 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [-(2) as int8_t, 42 as int8_t],
        [-(2) as int8_t, 52 as int8_t],
        [-(9) as int8_t, 57 as int8_t],
        [-(6) as int8_t, 63 as int8_t],
        [-(4) as int8_t, 65 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(7) as int8_t, 82 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(6) as int8_t, 81 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [-(17) as int8_t, 80 as int8_t],
        [-(18) as int8_t, 73 as int8_t],
        [-(4) as int8_t, 74 as int8_t],
        [-(10) as int8_t, 83 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-1 as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(14) as int8_t, 66 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [2 as int8_t, 59 as int8_t],
        [-(3) as int8_t, 81 as int8_t],
        [-(3) as int8_t, 76 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [-(6) as int8_t, 78 as int8_t],
        [-(12) as int8_t, 72 as int8_t],
        [-(14) as int8_t, 68 as int8_t],
        [-(3) as int8_t, 70 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(5) as int8_t, 62 as int8_t],
        [0 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 61 as int8_t],
        [-(9) as int8_t, 60 as int8_t],
        [1 as int8_t, 54 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [17 as int8_t, -(10) as int8_t],
        [32 as int8_t, -(13) as int8_t],
        [42 as int8_t, -(9) as int8_t],
        [49 as int8_t, -(5) as int8_t],
        [53 as int8_t, 0 as int8_t],
        [64 as int8_t, 3 as int8_t],
        [68 as int8_t, 10 as int8_t],
        [66 as int8_t, 27 as int8_t],
        [47 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 71 as int8_t],
        [0 as int8_t, 24 as int8_t],
        [-1 as int8_t, 36 as int8_t],
        [-(2) as int8_t, 42 as int8_t],
        [-(2) as int8_t, 52 as int8_t],
        [-(9) as int8_t, 57 as int8_t],
        [-(6) as int8_t, 63 as int8_t],
        [-(4) as int8_t, 65 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(7) as int8_t, 82 as int8_t],
        [-(21) as int8_t, 126 as int8_t],
        [-(23) as int8_t, 124 as int8_t],
        [-(20) as int8_t, 110 as int8_t],
        [-(26) as int8_t, 126 as int8_t],
        [-(25) as int8_t, 124 as int8_t],
        [-(17) as int8_t, 105 as int8_t],
        [-(27) as int8_t, 121 as int8_t],
        [-(27) as int8_t, 117 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(27) as int8_t, 116 as int8_t],
        [-(33) as int8_t, 122 as int8_t],
        [-(10) as int8_t, 95 as int8_t],
        [-(14) as int8_t, 100 as int8_t],
        [-(8) as int8_t, 95 as int8_t],
        [-(17) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 114 as int8_t],
        [-(6) as int8_t, 89 as int8_t],
        [-(2) as int8_t, 80 as int8_t],
        [-(4) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 85 as int8_t],
        [-(8) as int8_t, 81 as int8_t],
        [-1 as int8_t, 72 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [9 as int8_t, 56 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [1 as int8_t, 69 as int8_t],
        [7 as int8_t, 69 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(16) as int8_t, 77 as int8_t],
        [-(2) as int8_t, 64 as int8_t],
        [2 as int8_t, 61 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(3) as int8_t, 65 as int8_t],
        [-(3) as int8_t, 66 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [9 as int8_t, 51 as int8_t],
        [-1 as int8_t, 66 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 75 as int8_t],
        [-(21) as int8_t, 126 as int8_t],
        [-(23) as int8_t, 124 as int8_t],
        [-(20) as int8_t, 110 as int8_t],
        [-(26) as int8_t, 126 as int8_t],
        [-(25) as int8_t, 124 as int8_t],
        [-(17) as int8_t, 105 as int8_t],
        [-(27) as int8_t, 121 as int8_t],
        [-(27) as int8_t, 117 as int8_t],
        [-(17) as int8_t, 102 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(27) as int8_t, 116 as int8_t],
        [-(33) as int8_t, 122 as int8_t],
        [-(10) as int8_t, 95 as int8_t],
        [-(14) as int8_t, 100 as int8_t],
        [-(8) as int8_t, 95 as int8_t],
        [-(17) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 114 as int8_t],
        [-(6) as int8_t, 89 as int8_t],
        [-(2) as int8_t, 80 as int8_t],
        [-(4) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 85 as int8_t],
        [-(8) as int8_t, 81 as int8_t],
        [-1 as int8_t, 72 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [9 as int8_t, 56 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [1 as int8_t, 69 as int8_t],
        [7 as int8_t, 69 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(16) as int8_t, 77 as int8_t],
        [-(2) as int8_t, 64 as int8_t],
        [2 as int8_t, 61 as int8_t],
        [-(6) as int8_t, 67 as int8_t],
        [-(3) as int8_t, 64 as int8_t],
        [2 as int8_t, 57 as int8_t],
        [-(3) as int8_t, 65 as int8_t],
        [-(3) as int8_t, 66 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [9 as int8_t, 51 as int8_t],
        [-1 as int8_t, 66 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 75 as int8_t],
        [19 as int8_t, -(6) as int8_t],
        [18 as int8_t, -(6) as int8_t],
        [14 as int8_t, 0 as int8_t],
        [26 as int8_t, -(12) as int8_t],
        [31 as int8_t, -(16) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [33 as int8_t, -(22) as int8_t],
        [37 as int8_t, -(28) as int8_t],
        [39 as int8_t, -(30) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [47 as int8_t, -(42) as int8_t],
        [45 as int8_t, -(36) as int8_t],
        [49 as int8_t, -(34) as int8_t],
        [41 as int8_t, -(17) as int8_t],
        [32 as int8_t, 9 as int8_t],
        [69 as int8_t, -(71) as int8_t],
        [63 as int8_t, -(63) as int8_t],
        [66 as int8_t, -(64) as int8_t],
        [77 as int8_t, -(74) as int8_t],
        [54 as int8_t, -(39) as int8_t],
        [52 as int8_t, -(35) as int8_t],
        [41 as int8_t, -(10) as int8_t],
        [36 as int8_t, 0 as int8_t],
        [40 as int8_t, -1 as int8_t],
        [30 as int8_t, 14 as int8_t],
        [28 as int8_t, 26 as int8_t],
        [23 as int8_t, 37 as int8_t],
        [12 as int8_t, 55 as int8_t],
        [11 as int8_t, 65 as int8_t],
        [37 as int8_t, -(33) as int8_t],
        [39 as int8_t, -(36) as int8_t],
        [40 as int8_t, -(37) as int8_t],
        [38 as int8_t, -(30) as int8_t],
        [46 as int8_t, -(33) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [40 as int8_t, -(24) as int8_t],
        [49 as int8_t, -(29) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [40 as int8_t, -(10) as int8_t],
        [38 as int8_t, -(3) as int8_t],
        [46 as int8_t, -(5) as int8_t],
        [31 as int8_t, 20 as int8_t],
        [29 as int8_t, 30 as int8_t],
        [25 as int8_t, 44 as int8_t],
        [19 as int8_t, -(6) as int8_t],
        [18 as int8_t, -(6) as int8_t],
        [14 as int8_t, 0 as int8_t],
        [26 as int8_t, -(12) as int8_t],
        [31 as int8_t, -(16) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [33 as int8_t, -(22) as int8_t],
        [37 as int8_t, -(28) as int8_t],
        [39 as int8_t, -(30) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [47 as int8_t, -(42) as int8_t],
        [45 as int8_t, -(36) as int8_t],
        [49 as int8_t, -(34) as int8_t],
        [41 as int8_t, -(17) as int8_t],
        [32 as int8_t, 9 as int8_t],
        [69 as int8_t, -(71) as int8_t],
        [63 as int8_t, -(63) as int8_t],
        [66 as int8_t, -(64) as int8_t],
        [77 as int8_t, -(74) as int8_t],
        [54 as int8_t, -(39) as int8_t],
        [52 as int8_t, -(35) as int8_t],
        [41 as int8_t, -(10) as int8_t],
        [36 as int8_t, 0 as int8_t],
        [40 as int8_t, -1 as int8_t],
        [30 as int8_t, 14 as int8_t],
        [28 as int8_t, 26 as int8_t],
        [23 as int8_t, 37 as int8_t],
        [12 as int8_t, 55 as int8_t],
        [11 as int8_t, 65 as int8_t],
        [37 as int8_t, -(33) as int8_t],
        [39 as int8_t, -(36) as int8_t],
        [40 as int8_t, -(37) as int8_t],
        [38 as int8_t, -(30) as int8_t],
        [46 as int8_t, -(33) as int8_t],
        [42 as int8_t, -(30) as int8_t],
        [40 as int8_t, -(24) as int8_t],
        [49 as int8_t, -(29) as int8_t],
        [38 as int8_t, -(12) as int8_t],
        [40 as int8_t, -(10) as int8_t],
        [38 as int8_t, -(3) as int8_t],
        [46 as int8_t, -(5) as int8_t],
        [31 as int8_t, 20 as int8_t],
        [29 as int8_t, 30 as int8_t],
        [25 as int8_t, 44 as int8_t],
        [-(23) as int8_t, 112 as int8_t],
        [-(15) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(11) as int8_t, 77 as int8_t],
        [-(9) as int8_t, 80 as int8_t],
        [-(9) as int8_t, 84 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(34) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 101 as int8_t],
        [-(3) as int8_t, 39 as int8_t],
        [-(5) as int8_t, 53 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [-(11) as int8_t, 75 as int8_t],
        [-(15) as int8_t, 77 as int8_t],
        [-(17) as int8_t, 91 as int8_t],
        [-(25) as int8_t, 107 as int8_t],
        [-(25) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 122 as int8_t],
        [-(11) as int8_t, 76 as int8_t],
        [-(10) as int8_t, 44 as int8_t],
        [-(10) as int8_t, 52 as int8_t],
        [-(10) as int8_t, 57 as int8_t],
        [-(9) as int8_t, 58 as int8_t],
        [-(16) as int8_t, 72 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(4) as int8_t, 69 as int8_t],
        [-(5) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 86 as int8_t],
        [-(23) as int8_t, 112 as int8_t],
        [-(15) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [-(5) as int8_t, 66 as int8_t],
        [-(11) as int8_t, 77 as int8_t],
        [-(9) as int8_t, 80 as int8_t],
        [-(9) as int8_t, 84 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [-(34) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 101 as int8_t],
        [-(3) as int8_t, 39 as int8_t],
        [-(5) as int8_t, 53 as int8_t],
        [-(7) as int8_t, 61 as int8_t],
        [-(11) as int8_t, 75 as int8_t],
        [-(15) as int8_t, 77 as int8_t],
        [-(17) as int8_t, 91 as int8_t],
        [-(25) as int8_t, 107 as int8_t],
        [-(25) as int8_t, 111 as int8_t],
        [-(28) as int8_t, 122 as int8_t],
        [-(11) as int8_t, 76 as int8_t],
        [-(10) as int8_t, 44 as int8_t],
        [-(10) as int8_t, 52 as int8_t],
        [-(10) as int8_t, 57 as int8_t],
        [-(9) as int8_t, 58 as int8_t],
        [-(16) as int8_t, 72 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [-(4) as int8_t, 69 as int8_t],
        [-(5) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 86 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [-(2) as int8_t, 73 as int8_t],
        [-(12) as int8_t, 104 as int8_t],
        [-(9) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
    ],
    [
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [20 as int8_t, -(15) as int8_t],
        [2 as int8_t, 54 as int8_t],
        [3 as int8_t, 74 as int8_t],
        [-(28) as int8_t, 127 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [-1 as int8_t, 54 as int8_t],
        [7 as int8_t, 51 as int8_t],
        [29 as int8_t, 16 as int8_t],
        [25 as int8_t, 0 as int8_t],
        [14 as int8_t, 0 as int8_t],
        [-(10) as int8_t, 51 as int8_t],
        [-(3) as int8_t, 62 as int8_t],
        [-(27) as int8_t, 99 as int8_t],
        [26 as int8_t, 16 as int8_t],
        [-(4) as int8_t, 85 as int8_t],
        [-(24) as int8_t, 102 as int8_t],
        [5 as int8_t, 57 as int8_t],
        [6 as int8_t, 57 as int8_t],
        [-(17) as int8_t, 73 as int8_t],
        [14 as int8_t, 57 as int8_t],
        [20 as int8_t, 40 as int8_t],
        [20 as int8_t, 10 as int8_t],
        [29 as int8_t, 0 as int8_t],
        [54 as int8_t, 0 as int8_t],
        [37 as int8_t, 42 as int8_t],
        [12 as int8_t, 97 as int8_t],
        [-(32) as int8_t, 127 as int8_t],
        [-(22) as int8_t, 117 as int8_t],
        [-(2) as int8_t, 74 as int8_t],
        [-(4) as int8_t, 85 as int8_t],
        [-(24) as int8_t, 102 as int8_t],
        [5 as int8_t, 57 as int8_t],
        [-(6) as int8_t, 93 as int8_t],
        [-(14) as int8_t, 88 as int8_t],
        [-(6) as int8_t, 44 as int8_t],
        [4 as int8_t, 55 as int8_t],
        [-(11) as int8_t, 89 as int8_t],
        [-(15) as int8_t, 103 as int8_t],
        [-(21) as int8_t, 116 as int8_t],
        [19 as int8_t, 57 as int8_t],
        [20 as int8_t, 58 as int8_t],
        [4 as int8_t, 84 as int8_t],
        [6 as int8_t, 96 as int8_t],
        [1 as int8_t, 63 as int8_t],
        [-(5) as int8_t, 85 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [5 as int8_t, 63 as int8_t],
        [6 as int8_t, 75 as int8_t],
        [-(3) as int8_t, 90 as int8_t],
        [-1 as int8_t, 101 as int8_t],
        [3 as int8_t, 55 as int8_t],
        [-(4) as int8_t, 79 as int8_t],
        [-(2) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 97 as int8_t],
        [-(7) as int8_t, 50 as int8_t],
        [1 as int8_t, 60 as int8_t],
        [0 as int8_t, 41 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [0 as int8_t, 63 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [4 as int8_t, 86 as int8_t],
        [0 as int8_t, 97 as int8_t],
        [-(7) as int8_t, 72 as int8_t],
        [13 as int8_t, 41 as int8_t],
        [3 as int8_t, 62 as int8_t],
        [7 as int8_t, 34 as int8_t],
        [-(9) as int8_t, 88 as int8_t],
        [-(20) as int8_t, 127 as int8_t],
        [-(36) as int8_t, 127 as int8_t],
        [-(17) as int8_t, 91 as int8_t],
        [-(14) as int8_t, 95 as int8_t],
        [-(25) as int8_t, 84 as int8_t],
        [-(25) as int8_t, 86 as int8_t],
        [-(12) as int8_t, 89 as int8_t],
        [-(17) as int8_t, 91 as int8_t],
        [-(31) as int8_t, 127 as int8_t],
        [-(14) as int8_t, 76 as int8_t],
        [-(18) as int8_t, 103 as int8_t],
        [-(13) as int8_t, 90 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [11 as int8_t, 80 as int8_t],
        [5 as int8_t, 76 as int8_t],
        [2 as int8_t, 84 as int8_t],
        [5 as int8_t, 78 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [4 as int8_t, 61 as int8_t],
        [-(14) as int8_t, 83 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
        [0 as int8_t, 65 as int8_t],
        [-(2) as int8_t, 79 as int8_t],
        [0 as int8_t, 72 as int8_t],
        [-(4) as int8_t, 92 as int8_t],
        [-(6) as int8_t, 56 as int8_t],
        [3 as int8_t, 68 as int8_t],
        [-(8) as int8_t, 71 as int8_t],
        [-(13) as int8_t, 98 as int8_t],
        [-(4) as int8_t, 86 as int8_t],
        [-(12) as int8_t, 88 as int8_t],
        [-(5) as int8_t, 82 as int8_t],
        [-(3) as int8_t, 72 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 89 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-1 as int8_t, 59 as int8_t],
        [5 as int8_t, 66 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [-1 as int8_t, 74 as int8_t],
        [-(4) as int8_t, 44 as int8_t],
        [-1 as int8_t, 69 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [-(7) as int8_t, 51 as int8_t],
        [-(4) as int8_t, 47 as int8_t],
        [-(6) as int8_t, 42 as int8_t],
        [-(3) as int8_t, 41 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [8 as int8_t, 76 as int8_t],
        [-(9) as int8_t, 78 as int8_t],
        [-(11) as int8_t, 83 as int8_t],
        [9 as int8_t, 52 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [-(5) as int8_t, 90 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [-(15) as int8_t, 72 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 80 as int8_t],
        [-(21) as int8_t, 83 as int8_t],
        [-(21) as int8_t, 64 as int8_t],
        [-(13) as int8_t, 31 as int8_t],
        [-(25) as int8_t, 64 as int8_t],
        [-(29) as int8_t, 94 as int8_t],
        [9 as int8_t, 75 as int8_t],
        [17 as int8_t, 63 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 35 as int8_t],
        [-(2) as int8_t, 27 as int8_t],
        [13 as int8_t, 91 as int8_t],
        [3 as int8_t, 65 as int8_t],
        [-(7) as int8_t, 69 as int8_t],
        [8 as int8_t, 77 as int8_t],
        [-(10) as int8_t, 66 as int8_t],
        [3 as int8_t, 62 as int8_t],
        [-(3) as int8_t, 68 as int8_t],
        [-(20) as int8_t, 81 as int8_t],
        [0 as int8_t, 30 as int8_t],
        [1 as int8_t, 7 as int8_t],
        [-(3) as int8_t, 23 as int8_t],
        [-(21) as int8_t, 74 as int8_t],
        [16 as int8_t, 66 as int8_t],
        [-(23) as int8_t, 124 as int8_t],
        [17 as int8_t, 37 as int8_t],
        [44 as int8_t, -(18) as int8_t],
        [50 as int8_t, -(34) as int8_t],
        [-(22) as int8_t, 127 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [7 as int8_t, 34 as int8_t],
        [11 as int8_t, 29 as int8_t],
        [8 as int8_t, 31 as int8_t],
        [6 as int8_t, 37 as int8_t],
        [7 as int8_t, 42 as int8_t],
        [3 as int8_t, 40 as int8_t],
        [8 as int8_t, 33 as int8_t],
        [13 as int8_t, 43 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [4 as int8_t, 47 as int8_t],
        [3 as int8_t, 55 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [6 as int8_t, 60 as int8_t],
        [8 as int8_t, 44 as int8_t],
        [11 as int8_t, 44 as int8_t],
        [14 as int8_t, 42 as int8_t],
        [7 as int8_t, 48 as int8_t],
        [4 as int8_t, 56 as int8_t],
        [4 as int8_t, 52 as int8_t],
        [13 as int8_t, 37 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [19 as int8_t, 58 as int8_t],
        [10 as int8_t, 48 as int8_t],
        [12 as int8_t, 45 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [20 as int8_t, 33 as int8_t],
        [8 as int8_t, 63 as int8_t],
        [35 as int8_t, -(18) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [28 as int8_t, -(3) as int8_t],
        [24 as int8_t, 10 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [52 as int8_t, -(44) as int8_t],
        [39 as int8_t, -(24) as int8_t],
        [19 as int8_t, 17 as int8_t],
        [31 as int8_t, 25 as int8_t],
        [36 as int8_t, 29 as int8_t],
        [24 as int8_t, 33 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [30 as int8_t, 20 as int8_t],
        [22 as int8_t, 73 as int8_t],
        [20 as int8_t, 34 as int8_t],
        [19 as int8_t, 31 as int8_t],
        [27 as int8_t, 44 as int8_t],
        [19 as int8_t, 16 as int8_t],
        [15 as int8_t, 36 as int8_t],
        [15 as int8_t, 36 as int8_t],
        [21 as int8_t, 28 as int8_t],
        [25 as int8_t, 21 as int8_t],
        [30 as int8_t, 20 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [27 as int8_t, 16 as int8_t],
        [24 as int8_t, 42 as int8_t],
        [0 as int8_t, 93 as int8_t],
        [14 as int8_t, 56 as int8_t],
        [15 as int8_t, 57 as int8_t],
        [26 as int8_t, 38 as int8_t],
        [-(24) as int8_t, 127 as int8_t],
        [-(24) as int8_t, 115 as int8_t],
        [-(22) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 62 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [-(14) as int8_t, 85 as int8_t],
        [-(13) as int8_t, 89 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(11) as int8_t, 92 as int8_t],
        [-(29) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 100 as int8_t],
        [-(14) as int8_t, 57 as int8_t],
        [-(12) as int8_t, 67 as int8_t],
        [-(11) as int8_t, 71 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(21) as int8_t, 85 as int8_t],
        [-(16) as int8_t, 88 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(15) as int8_t, 98 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(10) as int8_t, 82 as int8_t],
        [-(8) as int8_t, 48 as int8_t],
        [-(8) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(14) as int8_t, 75 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [-(12) as int8_t, 92 as int8_t],
        [-(18) as int8_t, 108 as int8_t],
        [-(4) as int8_t, 79 as int8_t],
        [-(22) as int8_t, 69 as int8_t],
        [-(16) as int8_t, 75 as int8_t],
        [-(2) as int8_t, 58 as int8_t],
        [1 as int8_t, 58 as int8_t],
        [-(13) as int8_t, 78 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [-(4) as int8_t, 81 as int8_t],
        [-(13) as int8_t, 99 as int8_t],
        [-(13) as int8_t, 81 as int8_t],
        [-(6) as int8_t, 38 as int8_t],
        [-(13) as int8_t, 62 as int8_t],
        [-(6) as int8_t, 58 as int8_t],
        [-(2) as int8_t, 59 as int8_t],
        [-(16) as int8_t, 73 as int8_t],
        [-(10) as int8_t, 76 as int8_t],
        [-(13) as int8_t, 86 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [-(10) as int8_t, 87 as int8_t],
        [0 as int8_t, 0 as int8_t],
        [-(22) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 120 as int8_t],
        [-(27) as int8_t, 127 as int8_t],
        [-(19) as int8_t, 114 as int8_t],
        [-(23) as int8_t, 117 as int8_t],
        [-(25) as int8_t, 118 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(24) as int8_t, 113 as int8_t],
        [-(28) as int8_t, 118 as int8_t],
        [-(31) as int8_t, 120 as int8_t],
        [-(37) as int8_t, 124 as int8_t],
        [-(10) as int8_t, 94 as int8_t],
        [-(15) as int8_t, 102 as int8_t],
        [-(10) as int8_t, 99 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 92 as int8_t],
        [17 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(12) as int8_t, 91 as int8_t],
        [-(2) as int8_t, 77 as int8_t],
        [0 as int8_t, 71 as int8_t],
        [-1 as int8_t, 73 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [15 as int8_t, 57 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(10) as int8_t, 67 as int8_t],
        [1 as int8_t, 68 as int8_t],
        [0 as int8_t, 77 as int8_t],
        [2 as int8_t, 64 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [5 as int8_t, 59 as int8_t],
        [2 as int8_t, 65 as int8_t],
        [14 as int8_t, 54 as int8_t],
        [15 as int8_t, 44 as int8_t],
        [5 as int8_t, 60 as int8_t],
        [2 as int8_t, 70 as int8_t],
        [-(2) as int8_t, 76 as int8_t],
        [-(18) as int8_t, 86 as int8_t],
        [12 as int8_t, 70 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [-(12) as int8_t, 70 as int8_t],
        [11 as int8_t, 55 as int8_t],
        [5 as int8_t, 56 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [2 as int8_t, 65 as int8_t],
        [-(6) as int8_t, 74 as int8_t],
        [5 as int8_t, 54 as int8_t],
        [7 as int8_t, 54 as int8_t],
        [-(6) as int8_t, 76 as int8_t],
        [-(11) as int8_t, 82 as int8_t],
        [-(2) as int8_t, 77 as int8_t],
        [-(2) as int8_t, 77 as int8_t],
        [25 as int8_t, 42 as int8_t],
        [17 as int8_t, -(13) as int8_t],
        [16 as int8_t, -(9) as int8_t],
        [17 as int8_t, -(12) as int8_t],
        [27 as int8_t, -(21) as int8_t],
        [37 as int8_t, -(30) as int8_t],
        [41 as int8_t, -(40) as int8_t],
        [42 as int8_t, -(41) as int8_t],
        [48 as int8_t, -(47) as int8_t],
        [39 as int8_t, -(32) as int8_t],
        [46 as int8_t, -(40) as int8_t],
        [52 as int8_t, -(51) as int8_t],
        [46 as int8_t, -(41) as int8_t],
        [52 as int8_t, -(39) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [32 as int8_t, 11 as int8_t],
        [61 as int8_t, -(55) as int8_t],
        [56 as int8_t, -(46) as int8_t],
        [62 as int8_t, -(50) as int8_t],
        [81 as int8_t, -(67) as int8_t],
        [45 as int8_t, -(20) as int8_t],
        [35 as int8_t, -(2) as int8_t],
        [28 as int8_t, 15 as int8_t],
        [34 as int8_t, 1 as int8_t],
        [39 as int8_t, 1 as int8_t],
        [30 as int8_t, 17 as int8_t],
        [20 as int8_t, 38 as int8_t],
        [18 as int8_t, 45 as int8_t],
        [15 as int8_t, 54 as int8_t],
        [0 as int8_t, 79 as int8_t],
        [36 as int8_t, -(16) as int8_t],
        [37 as int8_t, -(14) as int8_t],
        [37 as int8_t, -(17) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [29 as int8_t, 15 as int8_t],
        [24 as int8_t, 25 as int8_t],
        [34 as int8_t, 22 as int8_t],
        [31 as int8_t, 16 as int8_t],
        [35 as int8_t, 18 as int8_t],
        [31 as int8_t, 28 as int8_t],
        [33 as int8_t, 41 as int8_t],
        [36 as int8_t, 28 as int8_t],
        [27 as int8_t, 47 as int8_t],
        [21 as int8_t, 62 as int8_t],
        [18 as int8_t, 31 as int8_t],
        [19 as int8_t, 26 as int8_t],
        [36 as int8_t, 24 as int8_t],
        [24 as int8_t, 23 as int8_t],
        [27 as int8_t, 16 as int8_t],
        [24 as int8_t, 30 as int8_t],
        [31 as int8_t, 29 as int8_t],
        [22 as int8_t, 41 as int8_t],
        [22 as int8_t, 42 as int8_t],
        [16 as int8_t, 60 as int8_t],
        [15 as int8_t, 52 as int8_t],
        [14 as int8_t, 60 as int8_t],
        [3 as int8_t, 78 as int8_t],
        [-(16) as int8_t, 123 as int8_t],
        [21 as int8_t, 53 as int8_t],
        [22 as int8_t, 56 as int8_t],
        [25 as int8_t, 61 as int8_t],
        [21 as int8_t, 33 as int8_t],
        [19 as int8_t, 50 as int8_t],
        [17 as int8_t, 61 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 37 as int8_t],
        [-(8) as int8_t, 44 as int8_t],
        [-(11) as int8_t, 49 as int8_t],
        [-(10) as int8_t, 56 as int8_t],
        [-(12) as int8_t, 59 as int8_t],
        [-(8) as int8_t, 63 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-(6) as int8_t, 68 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [11 as int8_t, 80 as int8_t],
        [5 as int8_t, 76 as int8_t],
        [2 as int8_t, 84 as int8_t],
        [5 as int8_t, 78 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [4 as int8_t, 61 as int8_t],
        [-(14) as int8_t, 83 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
        [11 as int8_t, 80 as int8_t],
        [5 as int8_t, 76 as int8_t],
        [2 as int8_t, 84 as int8_t],
        [5 as int8_t, 78 as int8_t],
        [-(6) as int8_t, 55 as int8_t],
        [4 as int8_t, 61 as int8_t],
        [-(14) as int8_t, 83 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
        [-(4) as int8_t, 86 as int8_t],
        [-(12) as int8_t, 88 as int8_t],
        [-(5) as int8_t, 82 as int8_t],
        [-(3) as int8_t, 72 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 89 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-1 as int8_t, 59 as int8_t],
        [5 as int8_t, 66 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [-1 as int8_t, 74 as int8_t],
        [-(4) as int8_t, 44 as int8_t],
        [-1 as int8_t, 69 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [-(7) as int8_t, 51 as int8_t],
        [-(4) as int8_t, 47 as int8_t],
        [-(6) as int8_t, 42 as int8_t],
        [-(3) as int8_t, 41 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [8 as int8_t, 76 as int8_t],
        [-(9) as int8_t, 78 as int8_t],
        [-(11) as int8_t, 83 as int8_t],
        [9 as int8_t, 52 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [-(5) as int8_t, 90 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [-(15) as int8_t, 72 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 80 as int8_t],
        [-(21) as int8_t, 83 as int8_t],
        [-(21) as int8_t, 64 as int8_t],
        [-(13) as int8_t, 31 as int8_t],
        [-(25) as int8_t, 64 as int8_t],
        [-(29) as int8_t, 94 as int8_t],
        [9 as int8_t, 75 as int8_t],
        [17 as int8_t, 63 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 35 as int8_t],
        [-(2) as int8_t, 27 as int8_t],
        [13 as int8_t, 91 as int8_t],
        [-(4) as int8_t, 86 as int8_t],
        [-(12) as int8_t, 88 as int8_t],
        [-(5) as int8_t, 82 as int8_t],
        [-(3) as int8_t, 72 as int8_t],
        [-(4) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 89 as int8_t],
        [-(9) as int8_t, 69 as int8_t],
        [-1 as int8_t, 59 as int8_t],
        [5 as int8_t, 66 as int8_t],
        [4 as int8_t, 57 as int8_t],
        [-(4) as int8_t, 71 as int8_t],
        [-(2) as int8_t, 71 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [-1 as int8_t, 74 as int8_t],
        [-(4) as int8_t, 44 as int8_t],
        [-1 as int8_t, 69 as int8_t],
        [0 as int8_t, 62 as int8_t],
        [-(7) as int8_t, 51 as int8_t],
        [-(4) as int8_t, 47 as int8_t],
        [-(6) as int8_t, 42 as int8_t],
        [-(3) as int8_t, 41 as int8_t],
        [-(6) as int8_t, 53 as int8_t],
        [8 as int8_t, 76 as int8_t],
        [-(9) as int8_t, 78 as int8_t],
        [-(11) as int8_t, 83 as int8_t],
        [9 as int8_t, 52 as int8_t],
        [0 as int8_t, 67 as int8_t],
        [-(5) as int8_t, 90 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [-(15) as int8_t, 72 as int8_t],
        [-(5) as int8_t, 75 as int8_t],
        [-(8) as int8_t, 80 as int8_t],
        [-(21) as int8_t, 83 as int8_t],
        [-(21) as int8_t, 64 as int8_t],
        [-(13) as int8_t, 31 as int8_t],
        [-(25) as int8_t, 64 as int8_t],
        [-(29) as int8_t, 94 as int8_t],
        [9 as int8_t, 75 as int8_t],
        [17 as int8_t, 63 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(5) as int8_t, 35 as int8_t],
        [-(2) as int8_t, 27 as int8_t],
        [13 as int8_t, 91 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [7 as int8_t, 34 as int8_t],
        [11 as int8_t, 29 as int8_t],
        [8 as int8_t, 31 as int8_t],
        [6 as int8_t, 37 as int8_t],
        [7 as int8_t, 42 as int8_t],
        [3 as int8_t, 40 as int8_t],
        [8 as int8_t, 33 as int8_t],
        [13 as int8_t, 43 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [4 as int8_t, 47 as int8_t],
        [3 as int8_t, 55 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [6 as int8_t, 60 as int8_t],
        [8 as int8_t, 44 as int8_t],
        [11 as int8_t, 44 as int8_t],
        [14 as int8_t, 42 as int8_t],
        [7 as int8_t, 48 as int8_t],
        [4 as int8_t, 56 as int8_t],
        [4 as int8_t, 52 as int8_t],
        [13 as int8_t, 37 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [19 as int8_t, 58 as int8_t],
        [10 as int8_t, 48 as int8_t],
        [12 as int8_t, 45 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [20 as int8_t, 33 as int8_t],
        [8 as int8_t, 63 as int8_t],
        [35 as int8_t, -(18) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [28 as int8_t, -(3) as int8_t],
        [24 as int8_t, 10 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [52 as int8_t, -(44) as int8_t],
        [39 as int8_t, -(24) as int8_t],
        [19 as int8_t, 17 as int8_t],
        [31 as int8_t, 25 as int8_t],
        [36 as int8_t, 29 as int8_t],
        [24 as int8_t, 33 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [30 as int8_t, 20 as int8_t],
        [22 as int8_t, 73 as int8_t],
        [4 as int8_t, 39 as int8_t],
        [0 as int8_t, 42 as int8_t],
        [7 as int8_t, 34 as int8_t],
        [11 as int8_t, 29 as int8_t],
        [8 as int8_t, 31 as int8_t],
        [6 as int8_t, 37 as int8_t],
        [7 as int8_t, 42 as int8_t],
        [3 as int8_t, 40 as int8_t],
        [8 as int8_t, 33 as int8_t],
        [13 as int8_t, 43 as int8_t],
        [13 as int8_t, 36 as int8_t],
        [4 as int8_t, 47 as int8_t],
        [3 as int8_t, 55 as int8_t],
        [2 as int8_t, 58 as int8_t],
        [6 as int8_t, 60 as int8_t],
        [8 as int8_t, 44 as int8_t],
        [11 as int8_t, 44 as int8_t],
        [14 as int8_t, 42 as int8_t],
        [7 as int8_t, 48 as int8_t],
        [4 as int8_t, 56 as int8_t],
        [4 as int8_t, 52 as int8_t],
        [13 as int8_t, 37 as int8_t],
        [9 as int8_t, 49 as int8_t],
        [19 as int8_t, 58 as int8_t],
        [10 as int8_t, 48 as int8_t],
        [12 as int8_t, 45 as int8_t],
        [0 as int8_t, 69 as int8_t],
        [20 as int8_t, 33 as int8_t],
        [8 as int8_t, 63 as int8_t],
        [35 as int8_t, -(18) as int8_t],
        [33 as int8_t, -(25) as int8_t],
        [28 as int8_t, -(3) as int8_t],
        [24 as int8_t, 10 as int8_t],
        [27 as int8_t, 0 as int8_t],
        [34 as int8_t, -(14) as int8_t],
        [52 as int8_t, -(44) as int8_t],
        [39 as int8_t, -(24) as int8_t],
        [19 as int8_t, 17 as int8_t],
        [31 as int8_t, 25 as int8_t],
        [36 as int8_t, 29 as int8_t],
        [24 as int8_t, 33 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [30 as int8_t, 20 as int8_t],
        [22 as int8_t, 73 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 37 as int8_t],
        [-(8) as int8_t, 44 as int8_t],
        [-(11) as int8_t, 49 as int8_t],
        [-(10) as int8_t, 56 as int8_t],
        [-(12) as int8_t, 59 as int8_t],
        [-(8) as int8_t, 63 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-(6) as int8_t, 68 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [-(3) as int8_t, 78 as int8_t],
        [-(8) as int8_t, 74 as int8_t],
        [-(9) as int8_t, 72 as int8_t],
        [-(10) as int8_t, 72 as int8_t],
        [-(18) as int8_t, 75 as int8_t],
        [-(12) as int8_t, 71 as int8_t],
        [-(11) as int8_t, 63 as int8_t],
        [-(5) as int8_t, 70 as int8_t],
        [-(17) as int8_t, 75 as int8_t],
        [-(14) as int8_t, 72 as int8_t],
        [-(16) as int8_t, 67 as int8_t],
        [-(8) as int8_t, 53 as int8_t],
        [-(14) as int8_t, 59 as int8_t],
        [-(9) as int8_t, 52 as int8_t],
        [-(11) as int8_t, 68 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [9 as int8_t, -(2) as int8_t],
        [30 as int8_t, -(10) as int8_t],
        [31 as int8_t, -(4) as int8_t],
        [33 as int8_t, -1 as int8_t],
        [33 as int8_t, 7 as int8_t],
        [31 as int8_t, 12 as int8_t],
        [37 as int8_t, 23 as int8_t],
        [31 as int8_t, 38 as int8_t],
        [20 as int8_t, 64 as int8_t],
        [-(9) as int8_t, 71 as int8_t],
        [-(7) as int8_t, 37 as int8_t],
        [-(8) as int8_t, 44 as int8_t],
        [-(11) as int8_t, 49 as int8_t],
        [-(10) as int8_t, 56 as int8_t],
        [-(12) as int8_t, 59 as int8_t],
        [-(8) as int8_t, 63 as int8_t],
        [-(9) as int8_t, 67 as int8_t],
        [-(6) as int8_t, 68 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(22) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 120 as int8_t],
        [-(27) as int8_t, 127 as int8_t],
        [-(19) as int8_t, 114 as int8_t],
        [-(23) as int8_t, 117 as int8_t],
        [-(25) as int8_t, 118 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(24) as int8_t, 113 as int8_t],
        [-(28) as int8_t, 118 as int8_t],
        [-(31) as int8_t, 120 as int8_t],
        [-(37) as int8_t, 124 as int8_t],
        [-(10) as int8_t, 94 as int8_t],
        [-(15) as int8_t, 102 as int8_t],
        [-(10) as int8_t, 99 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 92 as int8_t],
        [17 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(12) as int8_t, 91 as int8_t],
        [-(2) as int8_t, 77 as int8_t],
        [0 as int8_t, 71 as int8_t],
        [-1 as int8_t, 73 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [15 as int8_t, 57 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(10) as int8_t, 67 as int8_t],
        [1 as int8_t, 68 as int8_t],
        [0 as int8_t, 77 as int8_t],
        [2 as int8_t, 64 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [5 as int8_t, 59 as int8_t],
        [2 as int8_t, 65 as int8_t],
        [14 as int8_t, 54 as int8_t],
        [15 as int8_t, 44 as int8_t],
        [5 as int8_t, 60 as int8_t],
        [2 as int8_t, 70 as int8_t],
        [-(22) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 127 as int8_t],
        [-(25) as int8_t, 120 as int8_t],
        [-(27) as int8_t, 127 as int8_t],
        [-(19) as int8_t, 114 as int8_t],
        [-(23) as int8_t, 117 as int8_t],
        [-(25) as int8_t, 118 as int8_t],
        [-(26) as int8_t, 117 as int8_t],
        [-(24) as int8_t, 113 as int8_t],
        [-(28) as int8_t, 118 as int8_t],
        [-(31) as int8_t, 120 as int8_t],
        [-(37) as int8_t, 124 as int8_t],
        [-(10) as int8_t, 94 as int8_t],
        [-(15) as int8_t, 102 as int8_t],
        [-(10) as int8_t, 99 as int8_t],
        [-(13) as int8_t, 106 as int8_t],
        [-(50) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 92 as int8_t],
        [17 as int8_t, 57 as int8_t],
        [-(5) as int8_t, 86 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(12) as int8_t, 91 as int8_t],
        [-(2) as int8_t, 77 as int8_t],
        [0 as int8_t, 71 as int8_t],
        [-1 as int8_t, 73 as int8_t],
        [4 as int8_t, 64 as int8_t],
        [-(7) as int8_t, 81 as int8_t],
        [5 as int8_t, 64 as int8_t],
        [15 as int8_t, 57 as int8_t],
        [1 as int8_t, 67 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(10) as int8_t, 67 as int8_t],
        [1 as int8_t, 68 as int8_t],
        [0 as int8_t, 77 as int8_t],
        [2 as int8_t, 64 as int8_t],
        [0 as int8_t, 68 as int8_t],
        [-(5) as int8_t, 78 as int8_t],
        [7 as int8_t, 55 as int8_t],
        [5 as int8_t, 59 as int8_t],
        [2 as int8_t, 65 as int8_t],
        [14 as int8_t, 54 as int8_t],
        [15 as int8_t, 44 as int8_t],
        [5 as int8_t, 60 as int8_t],
        [2 as int8_t, 70 as int8_t],
        [17 as int8_t, -(13) as int8_t],
        [16 as int8_t, -(9) as int8_t],
        [17 as int8_t, -(12) as int8_t],
        [27 as int8_t, -(21) as int8_t],
        [37 as int8_t, -(30) as int8_t],
        [41 as int8_t, -(40) as int8_t],
        [42 as int8_t, -(41) as int8_t],
        [48 as int8_t, -(47) as int8_t],
        [39 as int8_t, -(32) as int8_t],
        [46 as int8_t, -(40) as int8_t],
        [52 as int8_t, -(51) as int8_t],
        [46 as int8_t, -(41) as int8_t],
        [52 as int8_t, -(39) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [32 as int8_t, 11 as int8_t],
        [61 as int8_t, -(55) as int8_t],
        [56 as int8_t, -(46) as int8_t],
        [62 as int8_t, -(50) as int8_t],
        [81 as int8_t, -(67) as int8_t],
        [45 as int8_t, -(20) as int8_t],
        [35 as int8_t, -(2) as int8_t],
        [28 as int8_t, 15 as int8_t],
        [34 as int8_t, 1 as int8_t],
        [39 as int8_t, 1 as int8_t],
        [30 as int8_t, 17 as int8_t],
        [20 as int8_t, 38 as int8_t],
        [18 as int8_t, 45 as int8_t],
        [15 as int8_t, 54 as int8_t],
        [0 as int8_t, 79 as int8_t],
        [36 as int8_t, -(16) as int8_t],
        [37 as int8_t, -(14) as int8_t],
        [37 as int8_t, -(17) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [29 as int8_t, 15 as int8_t],
        [24 as int8_t, 25 as int8_t],
        [34 as int8_t, 22 as int8_t],
        [31 as int8_t, 16 as int8_t],
        [35 as int8_t, 18 as int8_t],
        [31 as int8_t, 28 as int8_t],
        [33 as int8_t, 41 as int8_t],
        [36 as int8_t, 28 as int8_t],
        [27 as int8_t, 47 as int8_t],
        [21 as int8_t, 62 as int8_t],
        [17 as int8_t, -(13) as int8_t],
        [16 as int8_t, -(9) as int8_t],
        [17 as int8_t, -(12) as int8_t],
        [27 as int8_t, -(21) as int8_t],
        [37 as int8_t, -(30) as int8_t],
        [41 as int8_t, -(40) as int8_t],
        [42 as int8_t, -(41) as int8_t],
        [48 as int8_t, -(47) as int8_t],
        [39 as int8_t, -(32) as int8_t],
        [46 as int8_t, -(40) as int8_t],
        [52 as int8_t, -(51) as int8_t],
        [46 as int8_t, -(41) as int8_t],
        [52 as int8_t, -(39) as int8_t],
        [43 as int8_t, -(19) as int8_t],
        [32 as int8_t, 11 as int8_t],
        [61 as int8_t, -(55) as int8_t],
        [56 as int8_t, -(46) as int8_t],
        [62 as int8_t, -(50) as int8_t],
        [81 as int8_t, -(67) as int8_t],
        [45 as int8_t, -(20) as int8_t],
        [35 as int8_t, -(2) as int8_t],
        [28 as int8_t, 15 as int8_t],
        [34 as int8_t, 1 as int8_t],
        [39 as int8_t, 1 as int8_t],
        [30 as int8_t, 17 as int8_t],
        [20 as int8_t, 38 as int8_t],
        [18 as int8_t, 45 as int8_t],
        [15 as int8_t, 54 as int8_t],
        [0 as int8_t, 79 as int8_t],
        [36 as int8_t, -(16) as int8_t],
        [37 as int8_t, -(14) as int8_t],
        [37 as int8_t, -(17) as int8_t],
        [32 as int8_t, 1 as int8_t],
        [34 as int8_t, 15 as int8_t],
        [29 as int8_t, 15 as int8_t],
        [24 as int8_t, 25 as int8_t],
        [34 as int8_t, 22 as int8_t],
        [31 as int8_t, 16 as int8_t],
        [35 as int8_t, 18 as int8_t],
        [31 as int8_t, 28 as int8_t],
        [33 as int8_t, 41 as int8_t],
        [36 as int8_t, 28 as int8_t],
        [27 as int8_t, 47 as int8_t],
        [21 as int8_t, 62 as int8_t],
        [-(24) as int8_t, 115 as int8_t],
        [-(22) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 62 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [-(14) as int8_t, 85 as int8_t],
        [-(13) as int8_t, 89 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(11) as int8_t, 92 as int8_t],
        [-(29) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 100 as int8_t],
        [-(14) as int8_t, 57 as int8_t],
        [-(12) as int8_t, 67 as int8_t],
        [-(11) as int8_t, 71 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(21) as int8_t, 85 as int8_t],
        [-(16) as int8_t, 88 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(15) as int8_t, 98 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(10) as int8_t, 82 as int8_t],
        [-(8) as int8_t, 48 as int8_t],
        [-(8) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(14) as int8_t, 75 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [-(12) as int8_t, 92 as int8_t],
        [-(18) as int8_t, 108 as int8_t],
        [-(24) as int8_t, 115 as int8_t],
        [-(22) as int8_t, 82 as int8_t],
        [-(9) as int8_t, 62 as int8_t],
        [0 as int8_t, 53 as int8_t],
        [0 as int8_t, 59 as int8_t],
        [-(14) as int8_t, 85 as int8_t],
        [-(13) as int8_t, 89 as int8_t],
        [-(13) as int8_t, 94 as int8_t],
        [-(11) as int8_t, 92 as int8_t],
        [-(29) as int8_t, 127 as int8_t],
        [-(21) as int8_t, 100 as int8_t],
        [-(14) as int8_t, 57 as int8_t],
        [-(12) as int8_t, 67 as int8_t],
        [-(11) as int8_t, 71 as int8_t],
        [-(10) as int8_t, 77 as int8_t],
        [-(21) as int8_t, 85 as int8_t],
        [-(16) as int8_t, 88 as int8_t],
        [-(23) as int8_t, 104 as int8_t],
        [-(15) as int8_t, 98 as int8_t],
        [-(37) as int8_t, 127 as int8_t],
        [-(10) as int8_t, 82 as int8_t],
        [-(8) as int8_t, 48 as int8_t],
        [-(8) as int8_t, 61 as int8_t],
        [-(8) as int8_t, 66 as int8_t],
        [-(7) as int8_t, 70 as int8_t],
        [-(14) as int8_t, 75 as int8_t],
        [-(10) as int8_t, 79 as int8_t],
        [-(9) as int8_t, 83 as int8_t],
        [-(12) as int8_t, 92 as int8_t],
        [-(18) as int8_t, 108 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
        [-(5) as int8_t, 79 as int8_t],
        [-(11) as int8_t, 104 as int8_t],
        [-(11) as int8_t, 91 as int8_t],
        [-(30) as int8_t, 127 as int8_t],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1668:15"]
static mut x264_cabac_range_lps: [[uint8_t; 4]; 64] = [
    [2, 2, 2, 2],
    [6, 7, 8, 9],
    [6, 7, 9, 10],
    [6, 8, 9, 11],
    [7, 8, 10, 11],
    [7, 9, 10, 12],
    [7, 9, 11, 12],
    [8, 9, 11, 13],
    [8, 10, 12, 14],
    [9, 11, 12, 14],
    [9, 11, 13, 15],
    [10, 12, 14, 16],
    [10, 12, 15, 17],
    [11, 13, 15, 18],
    [11, 14, 16, 19],
    [12, 14, 17, 20],
    [12, 15, 18, 21],
    [13, 16, 19, 22],
    [14, 17, 20, 23],
    [14, 18, 21, 24],
    [15, 19, 22, 25],
    [16, 20, 23, 27],
    [17, 21, 25, 28],
    [18, 22, 26, 30],
    [19, 23, 27, 31],
    [20, 24, 29, 33],
    [21, 26, 30, 35],
    [22, 27, 32, 37],
    [23, 28, 33, 39],
    [24, 30, 35, 41],
    [26, 31, 37, 43],
    [27, 33, 39, 45],
    [29, 35, 41, 48],
    [30, 37, 43, 50],
    [32, 39, 46, 53],
    [33, 41, 48, 56],
    [35, 43, 51, 59],
    [37, 45, 54, 62],
    [39, 48, 56, 65],
    [41, 50, 59, 69],
    [43, 53, 63, 72],
    [46, 56, 66, 76],
    [48, 59, 69, 80],
    [51, 62, 73, 85],
    [53, 65, 77, 89],
    [56, 69, 81, 94],
    [59, 72, 86, 99],
    [62, 76, 90, 104],
    [66, 80, 95, 110],
    [69, 85, 100, 116],
    [73, 89, 105, 122],
    [77, 94, 111, 128],
    [81, 99, 117, 135],
    [85, 104, 123, 142],
    [90, 110, 130, 150],
    [95, 116, 137, 158],
    [100, 122, 144, 166],
    [105, 128, 152, 175],
    [111, 135, 160, 185],
    [116, 142, 169, 195],
    [123, 150, 178, 205],
    [128, 158, 187, 216],
    [128, 167, 197, 227],
    [128, 176, 208, 240],
];
#[no_mangle]
#[c2rust::src_loc = "1688:15"]
static mut x264_cabac_transition: [[uint8_t; 2]; 128] = [
    [0, 0],
    [1, 1],
    [2, 50],
    [51, 3],
    [2, 50],
    [51, 3],
    [4, 52],
    [53, 5],
    [6, 52],
    [53, 7],
    [8, 52],
    [53, 9],
    [10, 54],
    [55, 11],
    [12, 54],
    [55, 13],
    [14, 54],
    [55, 15],
    [16, 56],
    [57, 17],
    [18, 56],
    [57, 19],
    [20, 56],
    [57, 21],
    [22, 58],
    [59, 23],
    [24, 58],
    [59, 25],
    [26, 60],
    [61, 27],
    [28, 60],
    [61, 29],
    [30, 60],
    [61, 31],
    [32, 62],
    [63, 33],
    [34, 62],
    [63, 35],
    [36, 64],
    [65, 37],
    [38, 66],
    [67, 39],
    [40, 66],
    [67, 41],
    [42, 66],
    [67, 43],
    [44, 68],
    [69, 45],
    [46, 68],
    [69, 47],
    [48, 70],
    [71, 49],
    [50, 72],
    [73, 51],
    [52, 72],
    [73, 53],
    [54, 74],
    [75, 55],
    [56, 74],
    [75, 57],
    [58, 76],
    [77, 59],
    [60, 78],
    [79, 61],
    [62, 78],
    [79, 63],
    [64, 80],
    [81, 65],
    [66, 82],
    [83, 67],
    [68, 82],
    [83, 69],
    [70, 84],
    [85, 71],
    [72, 84],
    [85, 73],
    [74, 88],
    [89, 75],
    [76, 88],
    [89, 77],
    [78, 90],
    [91, 79],
    [80, 90],
    [91, 81],
    [82, 94],
    [95, 83],
    [84, 94],
    [95, 85],
    [86, 96],
    [97, 87],
    [88, 96],
    [97, 89],
    [90, 100],
    [101, 91],
    [92, 100],
    [101, 93],
    [94, 102],
    [103, 95],
    [96, 104],
    [105, 97],
    [98, 104],
    [105, 99],
    [100, 108],
    [109, 101],
    [102, 108],
    [109, 103],
    [104, 110],
    [111, 105],
    [106, 112],
    [113, 107],
    [108, 114],
    [115, 109],
    [110, 116],
    [117, 111],
    [112, 118],
    [119, 113],
    [114, 118],
    [119, 115],
    [116, 122],
    [123, 117],
    [118, 122],
    [123, 119],
    [120, 124],
    [125, 121],
    [122, 126],
    [127, 123],
    [124, 127],
    [126, 125],
];
#[no_mangle]
#[c2rust::src_loc = "1708:15"]
static mut x264_cabac_renorm_shift: [uint8_t; 64] = [
    6, 5, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[no_mangle]
#[c2rust::src_loc = "1717:16"]
static mut x264_cabac_entropy: [uint16_t; 128] = [
    (0.0273f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.7370f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0288f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.6618f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0303f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.5866f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0320f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.5114f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0337f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.4362f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0355f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.3610f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0375f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.2859f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0395f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.2106f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0416f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.1354f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0439f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (5.0602f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0463f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.9851f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0488f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.9099f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0515f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.8347f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0543f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.7595f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0572f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.6843f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0604f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.6091f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0637f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.5339f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0671f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.4588f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0708f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.3836f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0747f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.3083f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0788f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.2332f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0832f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.1580f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0878f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.0828f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0926f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (4.0076f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.0977f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.9324f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1032f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.8572f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1089f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.7820f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1149f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.7068f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1214f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.6316f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1282f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.5565f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1353f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.4813f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1429f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.4061f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1510f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.3309f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1596f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.2557f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1686f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.1805f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1782f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.1053f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1884f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (3.0301f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.1992f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.9549f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2107f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.8797f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2229f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.8046f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2358f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.7294f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2496f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.6542f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2642f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.5790f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2798f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.5038f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.2964f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.4286f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.3142f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.3534f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.3331f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.2782f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.3532f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.2030f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.3748f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.1278f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.3979f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (2.0527f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.4226f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.9775f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.4491f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.9023f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.4776f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.8271f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.5082f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.7519f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.5412f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.6767f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.5768f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.6015f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.6152f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.5263f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.6568f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.4511f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.7020f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.3759f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.7513f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.3008f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.8050f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.2256f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.8638f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.1504f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (0.9285f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.0752f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
    (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "1758:15"]
static mut x264_significant_coeff_flag_offset_8x8: [[uint8_t; 64]; 2] = [
    [
        0, 1, 2, 3, 4, 5, 5, 4, 4, 3, 3, 4, 4, 4, 5, 5, 4, 4, 4, 4, 3, 3, 6, 7, 7, 7, 8, 9, 10, 9,
        8, 7, 7, 6, 11, 12, 13, 11, 6, 7, 8, 9, 14, 10, 9, 8, 6, 11, 12, 13, 11, 6, 9, 14, 10, 9,
        11, 12, 13, 11, 14, 10, 12, 0,
    ],
    [
        0, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 7, 7, 8, 4, 5, 6, 9, 10, 10, 8, 11, 12, 11, 9, 9, 10, 10,
        8, 11, 12, 11, 9, 9, 10, 10, 8, 11, 12, 11, 9, 9, 10, 10, 8, 13, 13, 9, 9, 10, 10, 8, 13,
        13, 9, 9, 10, 10, 14, 14, 14, 14, 14, 0,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1770:15"]
static mut x264_last_coeff_flag_offset_8x8: [uint8_t; 63] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
];
#[no_mangle]
#[c2rust::src_loc = "1777:15"]
static mut x264_coeff_flag_offset_chroma_422_dc: [uint8_t; 7] = [0, 0, 1, 1, 2, 2, 2];
#[no_mangle]
#[c2rust::src_loc = "1778:16"]
static mut x264_significant_coeff_flag_offset: [[uint16_t; 16]; 2] = [
    [
        (105 + 0) as uint16_t,
        (105 + 15) as uint16_t,
        (105 + 29) as uint16_t,
        (105 + 44) as uint16_t,
        (105 + 47) as uint16_t,
        402 as uint16_t,
        (484 + 0) as uint16_t,
        (484 + 15) as uint16_t,
        (484 + 29) as uint16_t,
        660 as uint16_t,
        (528 + 0) as uint16_t,
        (528 + 15) as uint16_t,
        (528 + 29) as uint16_t,
        718 as uint16_t,
        0 as uint16_t,
        0 as uint16_t,
    ],
    [
        (277 + 0) as uint16_t,
        (277 + 15) as uint16_t,
        (277 + 29) as uint16_t,
        (277 + 44) as uint16_t,
        (277 + 47) as uint16_t,
        436 as uint16_t,
        (776 + 0) as uint16_t,
        (776 + 15) as uint16_t,
        (776 + 29) as uint16_t,
        675 as uint16_t,
        (820 + 0) as uint16_t,
        (820 + 15) as uint16_t,
        (820 + 29) as uint16_t,
        733 as uint16_t,
        0 as uint16_t,
        0 as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1783:16"]
static mut x264_last_coeff_flag_offset: [[uint16_t; 16]; 2] = [
    [
        (166 + 0) as uint16_t,
        (166 + 15) as uint16_t,
        (166 + 29) as uint16_t,
        (166 + 44) as uint16_t,
        (166 + 47) as uint16_t,
        417 as uint16_t,
        (572 + 0) as uint16_t,
        (572 + 15) as uint16_t,
        (572 + 29) as uint16_t,
        690 as uint16_t,
        (616 + 0) as uint16_t,
        (616 + 15) as uint16_t,
        (616 + 29) as uint16_t,
        748 as uint16_t,
        0 as uint16_t,
        0 as uint16_t,
    ],
    [
        (338 + 0) as uint16_t,
        (338 + 15) as uint16_t,
        (338 + 29) as uint16_t,
        (338 + 44) as uint16_t,
        (338 + 47) as uint16_t,
        451 as uint16_t,
        (864 + 0) as uint16_t,
        (864 + 15) as uint16_t,
        (864 + 29) as uint16_t,
        699 as uint16_t,
        (908 + 0) as uint16_t,
        (908 + 15) as uint16_t,
        (908 + 29) as uint16_t,
        757 as uint16_t,
        0 as uint16_t,
        0 as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1788:16"]
static mut x264_coeff_abs_level_m1_offset: [uint16_t; 16] = [
    (227 + 0) as uint16_t,
    (227 + 10) as uint16_t,
    (227 + 20) as uint16_t,
    (227 + 30) as uint16_t,
    (227 + 39) as uint16_t,
    426 as uint16_t,
    (952 + 0) as uint16_t,
    (952 + 10) as uint16_t,
    (952 + 20) as uint16_t,
    708 as uint16_t,
    (982 + 0) as uint16_t,
    (982 + 10) as uint16_t,
    (982 + 20) as uint16_t,
    766 as uint16_t,
    0,
    0,
];
#[no_mangle]
#[c2rust::src_loc = "1792:15"]
static mut x264_count_cat_m1: [uint8_t; 14] =
    [15, 14, 15, 3, 14, 63, 15, 14, 15, 63, 15, 14, 15, 63];
#[no_mangle]
#[c2rust::src_loc = "1799:13"]
static mut x264_coeff0_token: [vlc_t; 6] = [
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 1,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x3 as uint8_t,
            i_size: 2,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0xf as uint8_t,
            i_size: 4,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x3 as uint8_t,
            i_size: 6,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 2,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 1,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "1810:13"]
static mut x264_coeff_token: [[[vlc_t; 4]; 16]; 6] = [
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 2,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 5,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 7,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 8,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 11,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 13,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 14,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 14,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 15,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 15,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 15,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 16,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 16,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 16,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 2,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 3,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 5,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 7,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 11,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 11,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 12,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 13,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 13,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 13,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 14,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 14,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 4,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 4,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 5,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 4,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 5,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 7,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 8,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 8,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 9,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 10,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 10,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 6,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x10 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x11 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x12 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x13 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x14 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x15 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x16 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x17 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x18 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x19 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1a as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1b as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x1c as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1d as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1e as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1f as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x20 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x21 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x22 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x23 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x24 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x25 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x26 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x27 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x28 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x29 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2a as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2b as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x2c as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2d as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2e as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2f as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x30 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x31 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x32 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x33 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x34 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x35 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x36 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x37 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x38 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x39 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3a as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3b as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x3c as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3d as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3e as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3f as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 1,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 6,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 8,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0,
                    i_size: 7,
                };
                init
            },
        ],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 2,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 5,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 7,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 6,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 7,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 7,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 11,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 13,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 12,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 11,
                };
                init
            },
        ],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2265:13"]
static mut x264_total_zeros: [[vlc_t; 16]; 15] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 7,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 7,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 8,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 8,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 9,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 9,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 9,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 5,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 5,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2435:13"]
static mut x264_total_zeros_2x2_dc: [[vlc_t; 4]; 3] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2455:13"]
static mut x264_total_zeros_2x4_dc: [[vlc_t; 8]; 7] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 5,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2509:13"]
static mut x264_run_before_init: [[vlc_t; 16]; 7] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 1,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 2,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 7,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 8,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 9,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 10,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 11,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2570:21"]
static mut x264_zero: [uint8_t; 1024] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
unsafe extern "C" fn run_static_initializers() {
    x264_dct4_weight_tab = [
        (if 0 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.76777f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.11803f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.70711f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
    ];
    x264_dct8_weight_tab = [
        (if 0 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.0000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.8859f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (1.6000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.9415f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.2651f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.1910f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
    ];
    x264_dct4_weight2_tab = [
        (if 0 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (3.125f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (1.25f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (0.5f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
    ];
    x264_dct8_weight2_tab = [
        (if 0 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 0 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 0 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 4 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 4 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 2 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 2 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 3 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 3 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 5 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 5 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
        (if 1 == 0 {
            (1.00000f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 1 {
            (0.78487f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 2 {
            (2.56132f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 3 {
            (0.88637f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 4 {
            (1.60040f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else if 1 == 5 {
            (1.41850f64 * ((1) << 8) as c_double + 0.5f64) as c_int
        } else {
            0
        }) as uint32_t,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
