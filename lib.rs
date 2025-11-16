#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(register_tool)]
//#![feature(stdsimd)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
pub mod src {
    pub mod autocomplete;
    pub mod common {
        pub mod base;
        pub mod bitstream;
        pub mod cabac;
        pub mod common;
        pub mod cpu;
        pub mod dct;
        pub mod deblock;
        pub mod frame;
        pub mod macroblock;
        pub mod mc;
        pub mod mvpred;
        pub mod osdep;
        pub mod pixel;
        pub mod predict;
        pub mod quant;
        pub mod rectangle;
        pub mod set;
        pub mod tables;
        pub mod threadpool;
        pub mod vlc;
    } // mod common
    pub mod encoder {
        pub mod analyse;
        pub mod api;
        pub mod cabac;
        pub mod cavlc;
        pub mod encoder;
        pub mod lookahead;
        pub mod macroblock;
        pub mod me;
        pub mod ratecontrol;
        pub mod set;
    } // mod encoder
    pub mod filters {
        pub mod filters;
        pub mod video {
            pub mod cache;
            pub mod crop;
            pub mod depth;
            pub mod fix_vfr_pts;
            pub mod internal;
            pub mod resize;
            pub mod select_every;
            pub mod source;
            pub mod video;
        } // mod video
    } // mod filters
    pub mod input {
        pub mod avs;
        pub mod input;
        pub mod lavf;
        pub mod raw;
        pub mod thread;
        pub mod timecode;
        pub mod y4m;
    } // mod input
    pub mod output {
        pub mod flv;
        pub mod flv_bytestream;
        pub mod matroska;
        pub mod matroska_ebml;
        pub mod mp4_lsmash;
        pub mod raw;
    } // mod output
    pub mod x264;
} // mod src
