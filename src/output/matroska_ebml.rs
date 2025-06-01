use std::{io::Write as _, path::Path};

use byteorder::{BigEndian, WriteBytesExt as _};
use slab::Slab;
use snafu::{Snafu, ensure};

#[derive(Debug, Snafu)]
pub enum MuxerError {
    /// Thrown if an error occurs when trying to read or write files.
    #[snafu(transparent)]
    FileError { source: std::io::Error },

    #[snafu(display("Encountered invalid state!"))]
    InvalidState,

    #[snafu(display("Element ID {element_id:#X} exceeds 4-octet limit"))]
    ElementIdTooLarge { element_id: u64 },

    #[snafu(display("Element ID {size} exceeds 4-octet limit"))]
    ElementSizeTooLarge { size: u64 },
}

/// Display unit for MKV/Matroska files.
///
/// Corresponds to the DisplayUnit element (ID: 0x54B2) in the MKV specification.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayUnit {
    /// Display dimensions in pixels
    Pixels = 0,

    /// Display dimensions in centimeters
    Centimeters = 1,

    /// Display dimensions in inches
    Inches = 2,

    /// Display aspect ratio (width:height ratio)
    DisplayAspectRatio = 3,

    /// Unknown or unspecified unit
    #[default]
    Unknown = 4,
}

/// Stereo-3D video mode for MKV/Matroska files.
///
/// Corresponds to the StereoMode element (ID: 0x53B8) in the MKV specification.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StereoMode {
    /// Standard 2D video (no stereo)
    #[default]
    Mono = 0,

    /// Side by side stereo with left eye first
    SideBySideLeftFirst = 1,

    /// Top-bottom stereo with right eye first  
    TopBottomRightFirst = 2,

    /// Top-bottom stereo with left eye first
    TopBottomLeftFirst = 3,

    /// Checkerboard pattern with right eye first
    CheckboardRightFirst = 4,

    /// Checkerboard pattern with left eye first
    CheckboardLeftFirst = 5,

    /// Row interleaved with right eye first
    RowInterleavedRightFirst = 6,

    /// Row interleaved with left eye first
    RowInterleavedLeftFirst = 7,

    /// Column interleaved with right eye first
    ColumnInterleavedRightFirst = 8,

    /// Column interleaved with left eye first
    ColumnInterleavedLeftFirst = 9,

    /// Anaglyph stereo (cyan/red)
    AnaglyphCyanRed = 10,

    /// Side by side stereo with right eye first
    SideBySideRightFirst = 11,

    /// Anaglyph stereo (green/magenta)
    AnaglyphGreenMagenta = 12,

    /// Both eyes laced in one block with left eye first
    BothEyesLacedLeftFirst = 13,

    /// Both eyes laced in one block with right eye first
    BothEyesLacedRightFirst = 14,
}

/// Track type for MKV/Matroska files.
///
/// Corresponds to the TrackType element (ID: 0x83) in the MKV specification.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TrackType {
    /// Video track containing visual content
    #[default]
    Video = 1,

    /// Audio track containing audio content
    Audio = 2,

    /// Complex track (combination of different media types)
    Complex = 3,

    /// Logo track (overlay graphics)
    Logo = 16,

    /// Subtitle track containing text subtitles
    Subtitle = 17,

    /// Buttons track for menu navigation
    Buttons = 18,

    /// Control track for application control
    Control = 32,

    /// Metadata track containing additional information
    Metadata = 33,
}

struct MkContext {
    /// Matroska Element ID
    element_id: usize,
    /// Index for the parent object in the slab
    parent_id: Option<usize>,
    /// The data associated with the current context
    data: Vec<u8>,
}

impl MkContext {
    fn new(element_id: usize, parent_id: Option<usize>) -> Self {
        Self {
            element_id,
            parent_id,
            data: Vec::new(),
        }
    }

    fn write_elem_id(&mut self, element_id: u64) -> Result<(), MuxerError> {
        ensure!(
            element_id <= u32::MAX.into(),
            ElementIdTooLargeSnafu { element_id }
        );

        let index = element_id.leading_zeros();
        match index {
            0..=7 => self.data.write_u32::<BigEndian>(element_id as u32)?,
            8..=15 => self.data.write_u24::<BigEndian>(element_id as u32)?,
            16..=23 => self.data.write_u16::<BigEndian>(element_id as u16)?,
            24..=31 => self.data.write_u8(element_id as u8)?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn write_size(&mut self, size: u64) -> Result<(), MuxerError> {
        ensure!(size <= u32::MAX.into(), ElementSizeTooLargeSnafu { size });

        // Note that all-one-bit VINT_DATA values are reserved for "unknown size"
        match size {
            0..0x7F => {
                // 1 byte: 1xxxxxxx
                self.data.write_u8(size as u8 | (1 << 7))?
            }
            0x7F..0x3FFF => {
                // 2 bytes: 01xxxxxx xxxxxxxx
                self.data.write_u16::<BigEndian>(size as u16 | (1 << 14))?;
            }
            0x3FFF..0x1F_FFFF => {
                // 3 bytes: 001xxxxx xxxxxxxx xxxxxxxx
                self.data.write_u24::<BigEndian>(size as u32 | (1 << 31))?;
            }
            0x1F_FFFF..0x0FFF_FFFF => {
                // 4 bytes: 0001xxxx xxxxxxxx xxxxxxxx xxxxxxxx
                self.data.write_u32::<BigEndian>(size as u32 | (1 << 28))?;
            }
            0x0FFF_FFFF..=0xFFFF_FFFF => {
                // 5 bytes: 00001000 xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
                self.data.write_uint::<BigEndian>(size | (1 << 35), 5)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn write_string(&mut self, element_id: u64, string: &str) -> Result<(), MuxerError> {
        self.write_elem_id(element_id)?;
        self.write_size(string.len() as u64)?;
        self.data.write_all(string.as_bytes())?;
        Ok(())
    }

    fn write_binary(&mut self, element_id: u64, data: &[u8]) -> Result<(), MuxerError> {
        self.write_elem_id(element_id)?;
        self.write_size(data.len() as u64)?;
        self.data.write_all(data)?;
        Ok(())
    }

    fn write_uint(&mut self, elem_id: u64, value: u64) -> Result<(), MuxerError> {
        self.write_elem_id(elem_id)?;

        // Find minimum bytes needed (EBML integers are big-endian, leading zeros trimmed)
        let index = value.leading_zeros();
        self.write_size(8 - (index / 8) as u64)?;

        match index {
            0..=7 => self.data.write_u8(value as u8)?,
            8..=15 => self.data.write_u16::<BigEndian>(value as u16)?,
            16..=23 => self.data.write_u24::<BigEndian>(value as u32)?,
            24..=31 => self.data.write_u32::<BigEndian>(value as u32)?,
            32..=39 => self.data.write_uint::<BigEndian>(value, 5)?,
            40..=47 => self.data.write_uint::<BigEndian>(value, 6)?,
            48..=55 => self.data.write_uint::<BigEndian>(value, 7)?,
            56..=63 => self.data.write_u64::<BigEndian>(value)?,
            _ => unreachable!(),
        }
        Ok(())
    }

    fn write_float_raw(&mut self, value: f32) -> Result<(), MuxerError> {
        Ok(self.data.write_f32::<BigEndian>(value)?)
    }

    fn write_float(&mut self, element_id: u64, value: f32) -> Result<(), MuxerError> {
        self.write_elem_id(element_id)?;
        self.write_size(4)?;
        self.write_float_raw(value)
    }
}

struct MkWriter {
    /// The object we're writing all data to (stdout/file)
    writer: Box<dyn std::io::Write>,

    /// All active MKV context and its associated data and hierarchy.
    contexts: Slab<MkContext>,

    /// Slab ID for the root node, for easy lookup.
    root_id: usize,

    /// The lookup ID for the current cluster context when writing frames.
    cluster_id: Option<usize>,

    /// The lookup ID for the current frame context, if it exists.
    frame_id: Option<usize>,

    timescale: u64,
    default_duration: u64,
    wrote_header: bool,
    /// Whether or not we're in the middle of constructing a frame.
    in_frame: bool,
    keyframe: bool,
    skippable: bool,
    duration_offset: usize,

    /// The timestamp of the current frame being written out.
    frame_time: u64,

    /// The largest frame time we've seen thus far.
    max_frame_time: u64,

    /// The base timecode for the current cluster of frames, used for deltas on individual frames.
    cluster_base_time: u64,

    /// Flag to prevent the case that the user calls close() and `impl Drop` calls close() again.
    closed: bool,
}

impl MkWriter {
    fn new<P: AsRef<Path>>(filename: P) -> Result<Self, MuxerError> {
        let mut mkv = Self {
            writer: if filename.as_ref().as_os_str() == "-" {
                Box::new(std::io::stdout())
            } else {
                Box::new(std::fs::File::create(filename)?)
            },
            contexts: Slab::new(),

            root_id: 0,
            cluster_id: None,
            frame_id: None,

            timescale: 0,
            default_duration: 0,
            wrote_header: false,
            duration_offset: 0,
            in_frame: false,
            keyframe: false,
            skippable: false,
            frame_time: 0,
            max_frame_time: 0,
            cluster_base_time: 0,

            closed: false,
        };

        mkv.root_id = mkv.create_context(None, 0)?;
        // The default timestamp scale is in milliseconds
        mkv.timescale = 1000000;

        Ok(mkv)
    }

    fn write_header(
        &mut self,
        writing_app: &str,
        codec_id: &str,
        codec_private: &[u8],
        default_duration: u64,
        timescale: u64,
        width: u64,
        height: u64,
        d_width: u64,
        d_height: u64,
        display_unit: DisplayUnit,
        stereo_mode: Option<StereoMode>,
    ) -> Result<(), MuxerError> {
        ensure!(self.wrote_header == false, InvalidStateSnafu);
        ensure!(default_duration != 0, InvalidStateSnafu);
        ensure!(timescale != 0, InvalidStateSnafu);
        ensure!(width != 0, InvalidStateSnafu);
        ensure!(height != 0, InvalidStateSnafu);
        ensure!(d_width != 0, InvalidStateSnafu);
        ensure!(d_height != 0, InvalidStateSnafu);

        self.timescale = timescale;
        self.default_duration = default_duration;

        let context_id = self.create_context(Some(self.root_id), 0x1A45DFA3)?; // EBML
        {
            let context = self.contexts.get_mut(context_id).unwrap();
            context.write_uint(0x4286, 1)?; // EBMLVersion
            context.write_uint(0x42F7, 1)?; // EMBLReadVersion
            context.write_uint(0x42F2, 4)?; // EMBLMaxIDLength
            context.write_uint(0x42F3, 8)?; // EMBLMaxSizeLength
            context.write_string(0x4282, "matroska")?; // DocType
            context.write_uint(0x4287, if stereo_mode.is_some() { 3 } else { 2 })?; // DocTypeVersion
            context.write_uint(0x4285, 2)?; // DocTypeReadVersion
        }
        self.close_context(context_id)?;

        let context_id = self.create_context(Some(self.root_id), 0x18538067)?; // Segment
        self.flush_context_id(context_id)?;
        self.close_context(context_id)?;

        let context_id = self.create_context(Some(self.root_id), 0x1549A966)?; // SegmentInfo
        let context_length = {
            let context = self.contexts.get_mut(context_id).unwrap();
            // TODO: actually write out git rev
            context.write_string(0x4D80, "hex264-0.1.0+00000000")?; // MuxingApp
            context.write_string(0x5741, writing_app)?; // WritingApp
            context.write_uint(0x2AD7B1, self.timescale)?; // TimestampScale
            context.write_float(0x4489, 0.0)?; // Duration
            context.data.len()
        };
        let parent_offset = self.close_context(context_id)?;
        // The parent_offset is right before we flush the actual data of our context, and then we
        // can "index" into the raw blob. Since we know the float is the last 4 bytes, we just grab
        // context_length - 4.
        self.duration_offset = parent_offset + context_length - size_of::<f32>();

        let context_id = self.create_context(Some(self.root_id), 0x1654AE6B)?; // Tracks
        let track_id = self.create_context(Some(context_id), 0xAE)?; // TrackEntry
        {
            let track = self.contexts.get_mut(track_id).unwrap();
            track.write_uint(0xD7, 1)?; // TrackNumber
            track.write_uint(0x73C5, 1)?; // TrackUID
            track.write_uint(0x83, TrackType::Video as u64)?; // TrackType
            track.write_uint(0x9C, 0)?; // FlagLacing
            track.write_string(0x86, codec_id)?; // CodecID
            if !codec_private.is_empty() {
                track.write_binary(0x63A2, codec_private)?; // CodecPrivate
            }
            if default_duration != 0 {
                track.write_uint(0x23E383, default_duration)?; // DefaultDuration
            }

            let video_id = self.create_context(Some(track_id), 0xE0)?; // Video
            {
                let video = self.contexts.get_mut(video_id).unwrap();
                video.write_uint(0xB0, width)?; // PixelWidth
                video.write_uint(0xBA, height)?; // PixelHeight
                video.write_uint(0x54B2, display_unit as u64)?; // DisplayUnit
                video.write_uint(0x54B0, d_width)?; // DisplayWidth
                video.write_uint(0x54BA, d_height)?; // DisplayHeight
                if let Some(stereo_mode) = stereo_mode {
                    video.write_uint(0x53B8, stereo_mode as u64)?; // StereoMode
                }
            }
            self.close_context(video_id)?;
        }
        self.close_context(track_id)?;
        self.close_context(context_id)?;

        self.flush_context_data(self.root_id)?;

        self.wrote_header = true;

        Ok(())
    }

    fn create_context(
        &mut self,
        parent_id: Option<usize>,
        element_id: usize,
    ) -> Result<usize, MuxerError> {
        let context = MkContext::new(element_id, parent_id);
        let context_id = self.contexts.insert(context);
        Ok(context_id)
    }

    fn flush_context_id(&mut self, context_id: usize) -> Result<(), MuxerError> {
        let (id, parent_id) = {
            let context = &self.contexts[context_id];
            if context.element_id == 0 {
                return Ok(());
            }
            (context.element_id, context.parent_id)
        };

        if let Some(parent_id) = parent_id {
            self.contexts[parent_id].write_elem_id(id as u64)?;
            self.contexts[parent_id].data.write_u8(0xFF)?;
        }

        self.contexts[context_id].element_id = 0;
        Ok(())
    }

    fn flush_context_data(&mut self, context_id: usize) -> Result<(), MuxerError> {
        let parent_id = {
            let context = &self.contexts[context_id];
            if context.data.is_empty() {
                return Ok(());
            }
            context.parent_id
        };

        let data = core::mem::take(&mut self.contexts[context_id].data);

        if let Some(parent_id) = parent_id {
            self.contexts[parent_id].data.write_all(&data)?;
        } else {
            self.writer.write_all(&data)?;
        }

        Ok(())
    }

    fn close_context(&mut self, context_id: usize) -> Result<usize, MuxerError> {
        let (element_id, parent_id, length) = {
            let context = &self.contexts[context_id];
            (context.element_id, context.parent_id, context.data.len())
        };

        let length = if let Some(parent_id) = parent_id {
            if element_id != 0 {
                self.contexts[parent_id].write_elem_id(element_id as u64)?;
                self.contexts[parent_id].write_size(length as u64)?;
            }

            self.contexts[parent_id].data.len()
        } else {
            0
        };

        self.flush_context_data(context_id)?;
        self.contexts.remove(context_id);

        Ok(length)
    }

    fn start_frame(&mut self) -> Result<(), MuxerError> {
        self.flush_frame()?;
        self.in_frame = true;
        self.keyframe = false;
        self.skippable = false;
        Ok(())
    }

    fn set_frame_flags(
        &mut self,
        timestamp: u64,
        keyframe: bool,
        skippable: bool,
    ) -> Result<(), MuxerError> {
        if self.in_frame {
            self.frame_time = timestamp;
            self.keyframe = keyframe;
            self.skippable = skippable;

            if self.max_frame_time < self.frame_time {
                self.max_frame_time = self.frame_time;
            }
        }

        Ok(())
    }

    fn add_frame_data(&mut self, data: &[u8]) -> Result<(), MuxerError> {
        if self.in_frame {
            let frame_id = match self.frame_id {
                Some(frame_id) => frame_id,
                None => {
                    let frame_id = self.create_context(None, 0)?;
                    self.frame_id = Some(frame_id);
                    frame_id
                }
            };
            self.contexts[frame_id].data.write_all(data)?;
        }
        Ok(())
    }

    fn flush_frame(&mut self) -> Result<(), MuxerError> {
        if self.in_frame {
            let frame_scaled = self.frame_time / self.timescale;
            let cluster_scaled = self.cluster_base_time;

            let mut delta = frame_scaled.wrapping_sub(cluster_scaled) as i64;
            if delta > i16::MAX.into() || delta < i16::MIN.into() {
                self.close_cluster()?;
            }

            // If we don't currently have a cluster (i.e. we just closed it), let's start a new one.
            if self.cluster_id.is_none() {
                self.cluster_base_time = self.frame_time / self.timescale;
                let cluster_id = self.create_context(Some(self.root_id), 0x1F43B675)?; // Cluster
                self.contexts[cluster_id].write_uint(0xE7, self.cluster_base_time)?;
                self.cluster_id = Some(cluster_id);
                delta = 0;
            }

            let frame_size = match self.frame_id {
                Some(frame_id) => self.contexts[frame_id].data.len(),
                None => 0,
            };

            // We have to do this weird destructuring due to Rust's borrow checker limitations.
            let (cluster, frame) = match self.frame_id {
                Some(frame_id) => {
                    let (cluster, frame) = self
                        .contexts
                        .get2_mut(self.cluster_id.unwrap(), frame_id)
                        .unwrap();
                    (cluster, Some(frame))
                }
                None => (
                    self.contexts.get_mut(self.cluster_id.unwrap()).unwrap(),
                    None,
                ),
            };

            cluster.write_elem_id(0xA3)?; // SimpleBlock
            cluster.write_size((frame_size + 4) as u64)?; // Size
            cluster.write_size(1)?; // TrackNumber

            cluster.data.write_i16::<BigEndian>(delta as i16)?;
            let keyframe = self.keyframe as u8;
            let skippable = self.skippable as u8;
            cluster.data.write_u8((keyframe << 7) | skippable)?;
            if let Some(frame) = frame {
                let frame_data = core::mem::take(&mut frame.data);
                cluster.data.write_all(&frame_data)?;
            }

            self.in_frame = false;

            if cluster.data.len() > 0x100000 {
                self.close_cluster()?;
            }
        }

        Ok(())
    }

    fn close_cluster(&mut self) -> Result<(), MuxerError> {
        if let Some(cluster_id) = self.cluster_id {
            self.close_context(cluster_id)?;
        }

        self.cluster_id = None;
        self.flush_context_data(self.root_id)?;
        Ok(())
    }

    pub fn close(mut self, last_delta: u64) -> Result<(), MuxerError> {
        // We take `mut self` so that it implicitly drops
        self.close_internal(Some(last_delta))
    }

    fn close_internal(&mut self, last_delta: Option<u64>) -> Result<(), MuxerError> {
        if !self.closed {
            self.flush_frame()?;
            self.close_cluster()?;

            if self.wrote_header {
                let last_frametime = if self.default_duration != 0 {
                    self.default_duration
                } else if let Some(last_delta) = last_delta {
                    last_delta
                } else {
                    // Fallback: assume 30fps, in the case we got implicitly dropped.
                    self.timescale / 30
                };
                let total_duration = self.max_frame_time + last_frametime;
                let root = &mut self.contexts[self.root_id];
                let duration = (total_duration / self.timescale) as f32;
                root.data[self.duration_offset..self.duration_offset + size_of::<f32>()]
                    .copy_from_slice(&duration.to_be_bytes());
                self.flush_context_data(self.root_id)?;
            }

            self.closed = true;
        }
        Ok(())
    }
}

impl Drop for MkWriter {
    fn drop(&mut self) {
        let _ = self.close_internal(None);
    }
}
