use std::fs;
use std::io::prelude::*;
use crate::u32_from_vlq;
use std::io;

#[derive(Debug)]
pub struct TrackEvent {
    pub delta_time: u32,
    pub event_type: TrackEventType,
    pub data_byte_count: usize,
    pub data_bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TrackEventType {
    //Channel Voice Messages
    NoteOff(u8),
    NoteOn(u8),
    PolyphonicKeyPressure(u8),
    ControlChange(u8),
    ProgramChange(u8),
    ChannelPressure(u8),
    PitchWheelChange(u8),

    //Channel Mode Messages
    ChannelModeMessages(u8),

    //System Common Messages
    SystemExclusive,
    Undefined,
    SongPositionPointer,
    SongSelect,
    TuneRequest,
    EndOfExclusive,

    //System Real-time Messages
    RTTimingClock,
    RTStart,
    RTContinue,
    RTStop,
    RTActiveSensing,

    // Meta Events
    MetaSequenceNumber,
    MetaText,
    MetaCopyrightNotice,
    MetaSequenceOrTrackName,
    MetaInstrumentName,
    MetaLyricText,
    MetaMarkerText,
    MetaCuePoint,
    MetaMIDIChannelPrefixAssignment,
    MetaEndOfTrack,
    MetaTempoSetting,
    MetaSMPTEOffset,
    MetaTimeSignature,
    MetaKeySignature,
    MetaSequencerSpecificEvent,
}

enum TrackEventSize {
    Fixed(usize),
    Variable,
    SysExDelimited,
}

fn get_track_event_size_for_event(event_type: &TrackEventType) -> TrackEventSize {
    match event_type {
        TrackEventType::NoteOff(_) => TrackEventSize::Fixed(2),
        TrackEventType::NoteOn(_) => TrackEventSize::Fixed(2),
        TrackEventType::PolyphonicKeyPressure(_) => TrackEventSize::Fixed(2),
        TrackEventType::ControlChange(_) => TrackEventSize::Fixed(2),
        TrackEventType::ProgramChange(_) => TrackEventSize::Fixed(1),
        TrackEventType::ChannelPressure(_) => TrackEventSize::Fixed(1),
        TrackEventType::PitchWheelChange(_) => TrackEventSize::Fixed(2),

        //Channel Mode Messages
        TrackEventType::ChannelModeMessages(_) => TrackEventSize::Fixed(2),

        //System Common Messages
        TrackEventType::SystemExclusive => TrackEventSize::SysExDelimited,
        TrackEventType::Undefined => TrackEventSize::Fixed(0),
        TrackEventType::SongPositionPointer => TrackEventSize::Fixed(2),
        TrackEventType::SongSelect => TrackEventSize::Fixed(1),
        TrackEventType::TuneRequest => TrackEventSize::Fixed(0),
        TrackEventType::EndOfExclusive => TrackEventSize::Fixed(0),

        //System Real-time Messages
        TrackEventType::RTTimingClock => TrackEventSize::Fixed(0),
        TrackEventType::RTStart => TrackEventSize::Fixed(0),
        TrackEventType::RTContinue => TrackEventSize::Fixed(0),
        TrackEventType::RTStop => TrackEventSize::Fixed(0),
        TrackEventType::RTActiveSensing => TrackEventSize::Fixed(0),

        // Meta Events
        TrackEventType::MetaSequenceNumber => TrackEventSize::Variable,
        TrackEventType::MetaText => TrackEventSize::Variable,
        TrackEventType::MetaCopyrightNotice => TrackEventSize::Variable,
        TrackEventType::MetaSequenceOrTrackName => TrackEventSize::Variable,
        TrackEventType::MetaInstrumentName => TrackEventSize::Variable,
        TrackEventType::MetaLyricText => TrackEventSize::Variable,
        TrackEventType::MetaMarkerText => TrackEventSize::Variable,
        TrackEventType::MetaCuePoint => TrackEventSize::Variable,
        TrackEventType::MetaMIDIChannelPrefixAssignment => TrackEventSize::Variable,
        TrackEventType::MetaEndOfTrack => TrackEventSize::Variable,
        TrackEventType::MetaTempoSetting => TrackEventSize::Variable,
        TrackEventType::MetaSMPTEOffset => TrackEventSize::Variable,
        TrackEventType::MetaTimeSignature => TrackEventSize::Variable,
        TrackEventType::MetaKeySignature => TrackEventSize::Variable,
        TrackEventType::MetaSequencerSpecificEvent => TrackEventSize::Variable,
    }
}

#[derive(Debug)]
pub enum TrackEventError {
    IOError(io::Error)
}

impl TrackEvent {

    fn read_vlq(file: &mut fs::File, buf: &mut [u8]) -> Result<u8, TrackEventError> {
        let mut i = 0;
        loop {
            let result = file.read_exact(&mut buf[i..i+1]);
            if result.is_err() {
                return Err(TrackEventError::IOError(result.unwrap_err()));
            }
            if buf[i] <= 127 {
                break;
            }
            i += 1;
        }
        return Ok(i as u8);
    }

    fn parse_event_type(file: &mut fs::File) -> Result<TrackEventType, TrackEventError> {
        let mut buf = [0; 1];
        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(TrackEventError::IOError(result.unwrap_err()));
        }

        let mut meta_buf = [0; 1];
        if buf[0] == 0xFF {
            let result = file.read_exact(&mut meta_buf);
            if result.is_err() {
                return Err(TrackEventError::IOError(result.unwrap_err()));
            }
        }

        Ok(match buf[0] {
            x if (x & 0xF0 == 0x80) => TrackEventType::NoteOff(x & 0xF),
            x if (x & 0xF0 == 0x90) => TrackEventType::NoteOn(x & 0xF),
            x if (x & 0xF0 == 0xA0) => TrackEventType::PolyphonicKeyPressure(x & 0xF),
            x if (x & 0xF0 == 0xB0) => TrackEventType::ControlChange(x & 0xF),
            x if (x & 0xF0 == 0xC0) => TrackEventType::ProgramChange(x & 0xF),
            x if (x & 0xF0 == 0xD0) => TrackEventType::ChannelPressure(x & 0xF),
            x if (x & 0xF0 == 0xE0) => TrackEventType::PitchWheelChange(x & 0xF),

            0xF0 => TrackEventType::SystemExclusive,
            0xF2 => TrackEventType::SongPositionPointer,
            0xF3 => TrackEventType::SongSelect,
            0xF6 => TrackEventType::TuneRequest,
            0xF7 => TrackEventType::EndOfExclusive,

            0xF8 => TrackEventType::RTTimingClock,
            0xFA => TrackEventType::RTStart,
            0xFB => TrackEventType::RTContinue,
            0xFC => TrackEventType::RTStop,
            0xFE => TrackEventType::RTActiveSensing,
            0xFF => match meta_buf[0] {
                0x00 => TrackEventType::MetaSequenceNumber,
                0x01 => TrackEventType::MetaText,
                0x02 => TrackEventType::MetaCopyrightNotice,
                0x03 => TrackEventType::MetaSequenceOrTrackName,
                0x04 => TrackEventType::MetaInstrumentName,
                0x05 => TrackEventType::MetaLyricText,
                0x06 => TrackEventType::MetaMarkerText,
                0x07 => TrackEventType::MetaCuePoint,

                0x20 => TrackEventType::MetaMIDIChannelPrefixAssignment,
                0x2F => TrackEventType::MetaEndOfTrack,
                0x51 => TrackEventType::MetaTempoSetting,
                0x54 => TrackEventType::MetaSMPTEOffset,
                0x58 => TrackEventType::MetaTimeSignature,
                0x59 => TrackEventType::MetaKeySignature,
                0x7F => TrackEventType::MetaSequencerSpecificEvent,
                _ => TrackEventType::Undefined,
            },
            _ => TrackEventType::Undefined,
        })
    }

    pub fn parse_event(file: &mut fs::File) -> Result<TrackEvent, TrackEventError> {
        let mut buf = [0; 4];
        let result = Self::read_vlq(file, &mut buf);
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let delta_time = u32_from_vlq!(buf);
        
        let event_type = Self::parse_event_type(file);
        if event_type.is_err() {
            return Err(result.unwrap_err());
        }
        let event_type = event_type.unwrap();
        let event_size = get_track_event_size_for_event(&event_type);
        let data_byte_count = match event_size {
            TrackEventSize::Fixed(x) => x,
            TrackEventSize::Variable => {
                let result = Self::read_vlq(file, &mut buf);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                u32_from_vlq!(buf) as usize
            },
            _ => 0,
        };

        let mut data_bytes = vec![0; data_byte_count];

        let result = file.read_exact(&mut data_bytes);
        if result.is_err() {
            return Err(TrackEventError::IOError(result.unwrap_err()));
        }

        return Ok(TrackEvent {
            delta_time,
            event_type: event_type,
            data_byte_count,
            data_bytes,
        })
    }
}