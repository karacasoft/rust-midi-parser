use std::fs;
use std::io;
use std::io::prelude::*;
use super::track_event::TrackEvent;
use super::track_event::TrackEventType;
use super::track_event::TrackEventError;

#[derive(Debug)]
pub struct Track {
    pub length: u32,
    pub events: Vec<TrackEvent>,
}

#[derive(Debug)]
pub enum TrackError {
    IOError(io::Error),

    InvalidSignature,

    TrackEvent(TrackEventError),
}

impl Track {

    fn parse_track_events(file: &mut fs::File) -> Result<Vec<TrackEvent>, TrackError> {
        let mut events = Vec::new();

        loop {
            let event = TrackEvent::parse_event(file);
            if event.is_err() {
                return Err(TrackError::TrackEvent(event.unwrap_err()));
            }

            let event = event.unwrap();
            if event.event_type == TrackEventType::MetaEndOfTrack {
                events.push(event);
                break;
            } else {
                events.push(event);
            }
        }
        
        return Ok(events)
    }

    pub fn from_file(file: &mut fs::File) -> Result<Track, TrackError> {
        let mut buf = [0; 4];
        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(TrackError::IOError(result.unwrap_err()));
        }

        if buf != ['M' as u8, 'T' as u8, 'r' as u8, 'k' as u8] {
            return Err(TrackError::InvalidSignature);
        }

        let result = file.read_exact(&mut buf);
        if result.is_err() {
            return Err(TrackError::IOError(result.unwrap_err()));
        }

        let length = u32::from_be_bytes(buf);
        println!("Track length = {}", length);

        let events = Self::parse_track_events(file);
        if events.is_err() {
            return Err(events.unwrap_err());
        }

        return Ok(Track {
            length,
            events: events.unwrap(),
        });
    }
}