use std::{error::Error, path::Path};

use byteorder::{BigEndian, ByteOrder, WriteBytesExt as _};

const FLV_AUDIO_SAMPLESIZE_OFFSET: usize = 1;
const FLV_AUDIO_SAMPLERATE_OFFSET: usize = 2;
const FLV_AUDIO_CODECID_OFFSET: usize = 4;
const FLV_VIDEO_FRAMETYPE_OFFSET: usize = 4;

const FLV_AUDIO_CHANNEL_MASK: usize = 0x01;
const FLV_AUDIO_SAMPLESIZE_MASK: usize = 0x02;
const FLV_AUDIO_SAMPLERATE_MASK: usize = 0x0C;
const FLV_AUDIO_CODECID_MASK: usize = 0xF0;

const FLV_VIDEO_CODECID_MASK: usize = 0x0F;
const FLV_VIDEO_FRAMETYPE_MASK: usize = 0xF0;

const AMF_END_OF_OBJECT: usize = 0x09;

const FLV_HEADER_FLAG_HASVIDEO: usize = 1;
const FLV_HEADER_FLAG_HASAUDIO: usize = 4;

const FLV_TAG_TYPE_AUDIO: usize = 0x08;
const FLV_TAG_TYPE_VIDEO: usize = 0x09;
const FLV_TAG_TYPE_META: usize = 0x12;

const FLV_MONO: usize = 0;
const FLV_STEREO: usize = 1;

const FLV_SAMPLESSIZE_8BIT: usize = 0;
const FLV_SAMPLESSIZE_16BIT: usize = 1 << FLV_AUDIO_SAMPLESIZE_OFFSET;

/// Signifies 5512Hz and 8000Hz in the case of NELLYMOSER
const FLV_SAMPLERATE_SPECIAL: usize = 0;
const FLV_SAMPLERATE_11025HZ: usize = 1 << FLV_AUDIO_SAMPLERATE_OFFSET;
const FLV_SAMPLERATE_22050HZ: usize = 2 << FLV_AUDIO_SAMPLERATE_OFFSET;
const FLV_SAMPLERATE_44100HZ: usize = 3 << FLV_AUDIO_SAMPLERATE_OFFSET;

const FLV_CODECID_MP3: usize = 2 << FLV_AUDIO_CODECID_OFFSET;
const FLV_CODECID_AAC: usize = 10 << FLV_AUDIO_CODECID_OFFSET;

const FLV_CODECID_H264: usize = 7;

pub enum FLV_FRAME {
    KEY = 1 << FLV_VIDEO_FRAMETYPE_OFFSET,
    INTER = 2 << FLV_VIDEO_FRAMETYPE_OFFSET,
}

#[repr(u8)]
pub enum AmfDataType {
    Number = 0,
    Bool = 1,
    String = 2,
    Object = 3,
    Null = 5,
    Undefined = 6,
    Reference = 7,
    MixedArray = 8,
    ObjectEnd = 9,
    Array = 10,
    Date = 11,
    LongString = 12,
    Unsupported = 13,
}

pub struct FlvBuffer {
    buffer: Vec<u8>,
    writer: Box<dyn std::io::Write>,
    total_written: u64,
}

impl FlvBuffer {
    pub fn create_writer<P: AsRef<Path>>(filename: P) -> Result<Self, std::io::Error> {
        let writer: Box<dyn std::io::Write> = if filename.as_ref().as_os_str() == "-" {
            Box::new(std::io::stdout())
        } else {
            Box::new(std::fs::File::create(filename)?)
        };

        Ok(Self { buffer: Vec::new(), writer, total_written: 0 })
    }

    pub fn append_data(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn flush_data(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.buffer.is_empty() {
            self.writer.write_all(&self.buffer)?;
            self.total_written += self.buffer.len() as u64;
            self.buffer.clear();
        }
        Ok(())
    }

    pub fn rewrite_be24(&mut self, value: u32, position: usize) -> Result<(), Box<dyn Error>> {
        if position + 3 > self.buffer.len() {
            return Err("Position out of bounds for 24-bit write".into());
        }

        BigEndian::write_u24(&mut self.buffer[position..position + 3], value);
        Ok(())
    }

    pub fn dbl2int(value: f64) -> u64 {
        value.to_bits()
    }

    pub fn put_byte(&mut self, value: u8) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u8(value)?;
        Ok(())
    }

    pub fn put_be32(&mut self, value: u32) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u32::<BigEndian>(value)?;
        Ok(())
    }

    pub fn put_be64(&mut self, value: u64) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u64::<BigEndian>(value)?;
        Ok(())
    }

    pub fn put_be16(&mut self, value: u16) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u16::<BigEndian>(value)?;
        Ok(())
    }

    pub fn put_be24(&mut self, value: u32) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u24::<BigEndian>(value)?;
        Ok(())
    }

    pub fn put_tag(&mut self, value: String) -> Result<(), Box<dyn Error>> {
        self.buffer.extend_from_slice(value.as_bytes());
        Ok(())
    }

    pub fn put_amf_string(&mut self, value: String) -> Result<(), Box<dyn Error>> {
        if value.len() > u16::MAX.into() {
            return Err("String length too long to store".into());
        }

        self.buffer.write_u16::<BigEndian>(value.len() as u16)?;
        self.buffer.extend_from_slice(value.as_bytes());
        Ok(())
    }

    pub fn put_amf_double(&mut self, value: f64) -> Result<(), Box<dyn Error>> {
        self.buffer.write_u8(AmfDataType::Number as u8)?;
        self.buffer.write_f64::<BigEndian>(value)?;
        Ok(())
    }
}
