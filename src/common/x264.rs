use core::fmt;

// TODO: merge this into other modules as needed
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Flat representation of H.264 colorspaces, for converting to/from files.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum ColorspaceRaw {
    /// Invalid/unspecified format
    #[default]
    None = 0,
    /// Monochrome 4:0:0 (Y plane only)
    I400 = 1,
    /// YUV 4:2:0 planar (Y, U, V planes)
    I420 = 2,
    /// YVU 4:2:0 planar (Y, V, U planes)
    Yv12 = 3,
    /// YUV 4:2:0 with Y plane and packed U+V
    Nv12 = 4,
    /// YUV 4:2:0 with Y plane and packed V+U
    Nv21 = 5,
    /// YUV 4:2:2 planar (Y, U, V planes)
    I422 = 6,
    /// YVU 4:2:2 planar (Y, V, U planes)
    Yv16 = 7,
    /// YUV 4:2:2 with Y plane and packed U+V
    Nv16 = 8,
    /// YUYV 4:2:2 packed
    Yuyv = 9,
    /// UYVY 4:2:2 packed
    Uyvy = 10,
    /// 10-bit YUV 4:2:2 packed in 32-bit words
    V210 = 11,
    /// YUV 4:4:4 planar (Y, U, V planes)
    I444 = 12,
    /// YVU 4:4:4 planar (Y, V, U planes)
    Yv24 = 13,
    /// 24-bit packed BGR
    Bgr = 14,
    /// 32-bit packed BGRA
    Bgra = 15,
    /// 24-bit packed RGB
    Rgb = 16,
}

/// H.264 slice/frame types for temporal prediction structure.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SliceType {
    /// Let encoder choose optimal type
    #[default]
    Auto = 0,
    /// Instantaneous Decoder Refresh (resets reference buffer)
    Idr = 1,
    /// Intra frame (spatial prediction only)
    I = 2,
    /// Predicted frame (uses previous frames)
    P = 3,
    /// Reference B-frame (can be used by other frames)
    BRef = 4,
    /// Bi-predicted frame (uses past/future, disposable)
    B = 5,
    /// Keyframe (IDR or I based on open GOP setting)
    Keyframe = 6,
}

/// Semantic pixel format representation with chroma subsampling grouping.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// YUV 4:2:0 formats (chroma subsampled 2:1 in both directions)
    Yuv420(Yuv420Layout),
    /// YUV 4:2:2 formats (chroma subsampled 2:1 horizontally)
    Yuv422(Yuv422Layout),
    /// YUV 4:4:4 formats (no chroma subsampling)
    Yuv444(Yuv444Layout),
    /// RGB color formats
    Rgb(RgbLayout),
    /// Monochrome/grayscale (Y-component only)
    Monochrome,
    /// Invalid/unspecified format
    #[default]
    None,
}

/// Memory layout variants for YUV 4:2:0 formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yuv420Layout {
    /// Y, U, V planes (I420)
    Planar,
    /// Y, V, U planes (YV12)
    PlanarYvu,
    /// Y plane + packed U+V plane (NV12)
    SemiPlanar,
    /// Y plane + packed V+U plane (NV21)
    SemiPlanarVu,
}

/// Memory layout variants for YUV 4:2:2 formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yuv422Layout {
    /// Y, U, V planes (I422)
    Planar,
    /// Y, V, U planes (YV16)
    PlanarYvu,
    /// Y plane + packed U+V plane (NV16)
    SemiPlanar,
    /// Y0U0Y1V0 packed format (YUYV)
    PackedYuyv,
    /// U0Y0V0Y1 packed format (UYVY)
    PackedUyvy,
    /// 10-bit packed in 32-bit words (V210)
    PackedV210,
}

/// Memory layout variants for YUV 4:4:4 formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yuv444Layout {
    /// Y, U, V planes (I444)
    Planar,
    /// Y, V, U planes (YV24)
    PlanarYvu,
}

/// RGB color format variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgbLayout {
    /// 24-bit packed BGR
    Bgr24,
    /// 32-bit packed BGRA with alpha
    Bgra32,
    /// 24-bit packed RGB
    Rgb24,
}

/// Complete pixel format with bit depth and orientation modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelFormatInfo {
    /// The base pixel format and memory layout
    pub format: PixelFormat,
    /// Bit depth per component (8 or 16 bits)
    pub bit_depth: BitDepth,
    /// Whether the image is vertically flipped
    pub vertical_flip: bool,
}

/// Bit depth per pixel component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    /// 8 bits per component (standard)
    Eight,
    /// 16 bits per component (high depth)
    Sixteen,
}

/// Semantic frame type with temporal prediction information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Let the encoder choose the optimal frame type
    Auto,
    /// Intra-coded frame (no temporal prediction)
    Intra(IntraType),
    /// Predicted frame (uses previous frames for prediction)
    Predicted,
    /// Bi-predicted frame (can use both past and future frames)
    BiPredicted {
        /// Whether this frame can be disposed of (not used as reference)
        disposable: bool,
    },
}

/// Variants of intra-coded frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntraType {
    /// Regular I-frame
    I,
    /// Instantaneous Decoder Refresh (resets reference frame buffer)
    Idr,
    /// Keyframe (IDR or I depending on encoder settings)
    Keyframe,
}

/// Chroma subsampling patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaSubsampling {
    /// No chroma information or no subsampling
    None,
    /// 4:2:0 chroma sampled at 1/4 resolution
    Yuv420,
    /// 4:2:2 chroma sampled at 1/2 horizontal resolution
    Yuv422,
    /// 4:4:4 full chroma resolution
    Yuv444,
}

/// Memory layout patterns for pixel data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryLayout {
    /// No valid layout
    None,
    /// Single plane (grayscale or packed formats)
    Single,
    /// Three separate planes: Y, U, V
    Planar,
    /// Three separate planes: Y, V, U (U and V swapped)
    PlanarSwapped,
    /// Two planes: Y + interleaved chroma
    SemiPlanar,
    /// All components packed into single buffer
    Packed,
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Monochrome => write!(f, "Monochrome (4:0:0)"),
            Self::Yuv420(layout) => write!(f, "YUV 4:2:0 {}", layout),
            Self::Yuv422(layout) => write!(f, "YUV 4:2:2 {}", layout),
            Self::Yuv444(layout) => write!(f, "YUV 4:4:4 {}", layout),
            Self::Rgb(layout) => write!(f, "RGB {}", layout),
        }
    }
}

impl fmt::Display for Yuv420Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planar => write!(f, "Planar (I420)"),
            Self::PlanarYvu => write!(f, "Planar YVU (YV12)"),
            Self::SemiPlanar => write!(f, "Semi-Planar (NV12)"),
            Self::SemiPlanarVu => write!(f, "Semi-Planar VU (NV21)"),
        }
    }
}

impl fmt::Display for Yuv422Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planar => write!(f, "Planar (I422)"),
            Self::PlanarYvu => write!(f, "Planar YVU (YV16)"),
            Self::SemiPlanar => write!(f, "Semi-Planar (NV16)"),
            Self::PackedYuyv => write!(f, "Packed YUYV"),
            Self::PackedUyvy => write!(f, "Packed UYVY"),
            Self::PackedV210 => write!(f, "Packed V210 (10-bit)"),
        }
    }
}

impl fmt::Display for Yuv444Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planar => write!(f, "Planar (I444)"),
            Self::PlanarYvu => write!(f, "Planar YVU (YV24)"),
        }
    }
}

impl fmt::Display for RgbLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bgr24 => write!(f, "24-bit BGR"),
            Self::Bgra32 => write!(f, "32-bit BGRA"),
            Self::Rgb24 => write!(f, "24-bit RGB"),
        }
    }
}

impl fmt::Display for PixelFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format)?;

        if matches!(self.bit_depth, BitDepth::Sixteen) {
            write!(f, " (16-bit)")?;
        }

        if self.vertical_flip {
            write!(f, " (vertically flipped)")?;
        }

        Ok(())
    }
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto (encoder chooses)"),
            Self::Intra(intra_type) => write!(f, "Intra ({})", intra_type),
            Self::Predicted => write!(f, "Predicted (P-frame)"),
            Self::BiPredicted { disposable: true } => write!(f, "Bi-Predicted Disposable (B-frame)"),
            Self::BiPredicted { disposable: false } => write!(f, "Bi-Predicted Reference (B-ref frame)"),
        }
    }
}

impl fmt::Display for IntraType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I => write!(f, "I-frame"),
            Self::Idr => write!(f, "IDR frame"),
            Self::Keyframe => write!(f, "Keyframe"),
        }
    }
}

impl fmt::Display for BitDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eight => write!(f, "8-bit"),
            Self::Sixteen => write!(f, "16-bit"),
        }
    }
}

impl fmt::Display for ChromaSubsampling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "No chroma subsampling"),
            Self::Yuv420 => write!(f, "4:2:0 chroma subsampling"),
            Self::Yuv422 => write!(f, "4:2:2 chroma subsampling"),
            Self::Yuv444 => write!(f, "4:4:4 (no chroma subsampling)"),
        }
    }
}

impl fmt::Display for MemoryLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "No layout"),
            Self::Single => write!(f, "Single plane"),
            Self::Planar => write!(f, "Planar (Y, U, V)"),
            Self::PlanarSwapped => write!(f, "Planar swapped (Y, V, U)"),
            Self::SemiPlanar => write!(f, "Semi-planar (Y + UV)"),
            Self::Packed => write!(f, "Packed"),
        }
    }
}
