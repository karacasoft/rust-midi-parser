pub mod track;
pub mod track_event;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::vec;

use track::Track;
use track::TrackError;

#[derive(Debug)]
pub struct MidiFileHeader {
    pub length: u32,
    pub format: u16,
    pub n_tracks: u16,
    pub division: i16,
}

#[derive(Debug)]
pub struct MidiFile {
    pub filename: String,
    pub header: MidiFileHeader,
    pub tracks: vec::Vec<track::Track>,
}

#[derive(Debug)]
pub enum MidiFileError {
    IOError(io::Error),
    InvalidSignature,

    InvalidHeaderLength(u32),
    InvalidFormat(u16),

    TrackParseError(TrackError),

}

impl MidiFile {

    fn check_signature(file: &mut fs::File) -> Result<(), MidiFileError> {
        let mut buf = [0; 4];
        let result = file.read_exact(&mut buf);
        if result.is_ok() {
            if buf[0] == 'M' as u8 &&
                buf[1] == 'T' as u8 &&
                buf[2] == 'h' as u8 &&
                buf[3] == 'd' as u8
             {
                return Ok(());
            } else {
                return Err(MidiFileError::InvalidSignature);
            }
        } else {
            return Err(MidiFileError::IOError(result.unwrap_err()));
        }
    }

    fn parse_header(file: &mut fs::File) -> Result<MidiFileHeader, MidiFileError> {
        let mut buf = [0; 4];
        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(MidiFileError::IOError(result.unwrap_err()));
        }

        let length = u32::from_be_bytes(buf);
        if length != 6 {
            return Err(MidiFileError::InvalidHeaderLength(length));
        }

        let mut buf = [0; 2];
        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(MidiFileError::IOError(result.unwrap_err()));
        }

        let format = u16::from_be_bytes(buf);
        if format != 0 && format != 1 && format != 2 {
            return Err(MidiFileError::InvalidFormat(format));
        }

        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(MidiFileError::IOError(result.unwrap_err()));
        }

        let n_tracks = u16::from_be_bytes(buf);
        
        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(MidiFileError::IOError(result.unwrap_err()));
        }

        let division = i16::from_be_bytes(buf);


        Ok(MidiFileHeader {
            length: length,
            division: division,
            format: format,
            n_tracks: n_tracks,
        })
    }

    pub fn from_file(filename: String) -> Result<MidiFile, MidiFileError> {
        let file_result = fs::File::open(filename.as_str());
        if file_result.is_err() {
            return Err(MidiFileError::IOError(file_result.unwrap_err()));
        }
        let mut file = file_result.unwrap();

        let result = MidiFile::check_signature(&mut file);
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let result = MidiFile::parse_header(&mut file);
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let midi_file_header = result.unwrap();
        println!("n_tracks = {}", midi_file_header.n_tracks);
        let mut tracks: Vec<Track> = vec!();
        for _i in 0..midi_file_header.n_tracks {
            println!("i = {}", _i);
            let result = Track::from_file(&mut file);
            if result.is_err() {
                return Err(MidiFileError::TrackParseError(result.unwrap_err()));
            }
            let track = result.unwrap();
            tracks.push(track);
        }
        
        return Ok(MidiFile {
            filename,
            header: midi_file_header,
            tracks,
        });
    }
}