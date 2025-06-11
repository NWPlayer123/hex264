/// Contains all frame/image properties.
use snafu::prelude::*;

/// Error conditions when creating or manipulating video frames.
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum FrameError {
    /// Thrown if the pixel format is not supported for allocation.
    #[snafu(display("Pixel format {format:?} is not supported for frame allocation"))]
    UnsupportedFormat { format: PixelFormat },

    /// Thrown if the pixel format is invalid or unrecognized.
    #[snafu(display("Invalid pixel format"))]
    InvalidFormat,

    /// Thrown if frame dimensions are invalid.
    #[snafu(display("Invalid frame dimensions: {width}x{height}"))]
    InvalidDimensions { width: u32, height: u32 },

    /// Thrown if frame dimensions would cause integer overflow in calculations.
    #[snafu(display("Frame size calculation overflow: {width}x{height} with format {format:?}"))]
    SizeOverflow { width: u32, height: u32, format: PixelFormat },

    /// Thrown if memory allocation fails.
    #[snafu(display("Failed to allocate {size} bytes for frame data"))]
    AllocationFailed { size: usize },

    /// Thrown if an error occurs when trying to read or write frame data.
    #[snafu(transparent)]
    IoError { source: std::io::Error },
}

/// Flat representation of H.264 colorspaces, for converting to/from files.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<PixelFormat> for ColorspaceRaw {
    fn from(format: PixelFormat) -> Self {
        match format {
            PixelFormat::None => ColorspaceRaw::None,
            PixelFormat::Monochrome => ColorspaceRaw::I400,
            PixelFormat::Yuv420(format) => {
                match format {
                    Yuv420Layout::Planar => Self::I420,
                    Yuv420Layout::PlanarYvu => Self::Yv12,
                    Yuv420Layout::SemiPlanar => Self::Nv12,
                    Yuv420Layout::SemiPlanarVu => Self::Nv21,
                }
            }
            PixelFormat::Yuv422(format) => {
                match format {
                    Yuv422Layout::Planar => Self::I422,
                    Yuv422Layout::PlanarYvu => Self::Yv16,
                    Yuv422Layout::SemiPlanar => Self::Nv16,
                    Yuv422Layout::PackedYuyv => Self::Yuyv,
                    Yuv422Layout::PackedUyvy => Self::Uyvy,
                    Yuv422Layout::PackedV210 => Self::V210,
                }
            }
            PixelFormat::Yuv444(format) => {
                match format {
                    Yuv444Layout::Planar => Self::I444,
                    Yuv444Layout::PlanarYvu => Self::Yv24,
                }
            }
            PixelFormat::Rgb(format) => {
                match format {
                    RgbLayout::Bgr24 => Self::Bgr,
                    RgbLayout::Bgra32 => Self::Bgra,
                    RgbLayout::Rgb24 => Self::Rgb,
                }
            }
        }
    }
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

/// Bit depth per pixel component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    /// 8 bits per component (standard)
    #[default]
    Eight = 1,
    /// 16 bits per component (high depth)
    Sixteen = 2,
}

/// Complete pixel format with bit depth and orientation modifiers.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PixelFormatInfo {
    /// The base pixel format and memory layout
    pub format: PixelFormat,
    /// Bit depth per component (8 or 16 bits)
    pub bit_depth: BitDepth,
    /// Whether the image is vertically flipped
    pub vertical_flip: bool,
}

/// Semantic frame type with temporal prediction information.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Let the encoder choose the optimal frame type
    #[default]
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

/// Per-frame quantization control for quality vs file size tradeoff.
// TODO: make this Option<Quality> on Frame?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Quantizer {
    /// Let encoder decide optimal QP based on rate control
    #[default]
    Auto,
    /// Force specific quality level
    Quality(Quality),
}

/// Semantic quality levels for intuitive quality selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    /// Perfect quality, massive file size (QP 0)
    Lossless,
    /// Excellent quality for archival (QP ~18)
    VeryHigh,
    /// High quality for streaming/distribution (QP ~23)
    High,
    /// Balanced quality/size for most use cases (QP ~28)
    Medium,
    /// Lower quality, smaller files (QP ~35)
    Low,
    /// Poor quality, very small files (QP ~42)
    VeryLow,
    /// Worst quality, minimal file size (QP 51)
    Minimal,
    /// Raw QP value for advanced users
    Custom(QpValue),
}

/// Validated raw quantization parameter (0-51).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QpValue(u8);

impl QpValue {
    /// Create QP value, returns None if outside 0-51 range.
    pub fn new(qp: u8) -> Option<Self> {
        (qp <= 51).then(|| Self(qp))
    }

    /// Get the raw QP value.
    pub fn get(self) -> u8 {
        self.0
    }
}

/// Frame structure information for display timing and field ordering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FrameStructure {
    /// Let encoder automatically decide structure
    #[default]
    Auto,
    /// Standard progressive frame (most common)
    Progressive,
    /// Interlaced content with specific field ordering
    Interlaced(FieldOrder),
    /// Frame repetition for pulldown/frame rate conversion
    Repeated(RepeatPattern),
}

/// Field ordering for interlaced content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldOrder {
    /// Top field first, then bottom field
    TopBottom,
    /// Bottom field first, then top field  
    BottomTop,
    /// Top, bottom, top (3:2 pulldown pattern)
    TopBottomTop,
    /// Bottom, top, bottom (3:2 pulldown pattern)
    BottomTopBottom,
}

/// Frame repetition patterns for frame rate conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatPattern {
    /// Display frame twice (double frame rate)
    Double,
    /// Display frame three times (triple frame rate)
    Triple,
}

// NOTE: We use usize here because a Vec can only ever be as long as usize on the target platform.
#[derive(Debug, Default)]
pub struct Image {
    format: PixelFormatInfo,
    plane_count: usize,
    strides: [usize; 3],
    planes: [usize; 3],
    plane_data: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct Frame {
    structure: FrameStructure,
    frame_type: FrameType,
    quantizer: Quantizer,
    image: Image,
}

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_image(format: PixelFormatInfo, width: u32, height: u32) -> Result<Self, FrameError> {
        const LAYOUT_TABLE: [(u8, [usize; 3], [usize; 3]); 17] = [
            // X264_CSP_NONE = 0
            (0, [0, 0, 0], [0, 0, 0]),
            // X264_CSP_I400 = 1
            (1, [256, 0, 0], [256, 0, 0]),
            // X264_CSP_I420 = 2
            (3, [256, 128, 128], [256, 128, 128]),
            // X264_CSP_YV12 = 3
            (3, [256, 128, 128], [256, 128, 128]),
            // X264_CSP_NV12 = 4
            (2, [256, 256, 0], [256, 128, 0]),
            // X264_CSP_NV21 = 5
            (2, [256, 256, 0], [256, 128, 0]),
            // X264_CSP_I422 = 6
            (3, [256, 128, 128], [256, 256, 256]),
            // X264_CSP_YV16 = 7
            (3, [256, 128, 128], [256, 256, 256]),
            // X264_CSP_NV16 = 8
            (2, [256, 256, 0], [256, 256, 0]),
            // X264_CSP_YUYV = 9
            (1, [512, 0, 0], [256, 0, 0]),
            // X264_CSP_UYVY = 10
            (1, [512, 0, 0], [256, 0, 0]),
            // X264_CSP_V210 = 11 (unsupported)
            (0, [0, 0, 0], [0, 0, 0]),
            // X264_CSP_I444 = 12
            (3, [256, 256, 256], [256, 256, 256]),
            // X264_CSP_YV24 = 13
            (3, [256, 256, 256], [256, 256, 256]),
            // X264_CSP_BGR = 14
            (1, [768, 0, 0], [256, 0, 0]),
            // X264_CSP_BGRA = 15
            (1, [1024, 0, 0], [256, 0, 0]),
            // X264_CSP_RGB = 16
            (1, [768, 0, 0], [256, 0, 0]),
        ];
        // Make sure that the LAYOUT_TABLE and ColorspaceRaw are the same length. TODO: use nightly
        // core::mem::variant_count.
        const _: () = assert!(LAYOUT_TABLE.len() == (ColorspaceRaw::Rgb as usize + 1));

        let csp_index = ColorspaceRaw::from(format.format) as usize;
        let (plane_count, width_scales, height_scales) = LAYOUT_TABLE[csp_index];
        ensure!(plane_count != 0, UnsupportedFormatSnafu { format: format.format });

        let mut frame = Self::new();

        frame.image.format = format;
        frame.image.plane_count = plane_count.into();

        let depth_factor = format.bit_depth as usize;
        let mut frame_size = 0;

        for n in 0..plane_count as usize {
            let stride = ((width as usize * width_scales[n]) >> 8) * depth_factor;
            let plane_size = ((height as usize * height_scales[n]) >> 8) * stride;
            frame.image.strides[n] = stride;
            frame.image.planes[n] = frame_size;
            frame_size += plane_size;
        }
        // TODO: explicit allocation error handling whenever allocator_api ever lands.
        frame.image.plane_data = vec![0u8; frame_size];

        Ok(frame)
    }
}
