use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum NoteType {
    NoteOn,
    NoteOff,
}

impl fmt::Display for NoteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NoteType::NoteOn => write!(f, "NoteOn"),
            NoteType::NoteOff => write!(f, "NoteOff"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MidiNote {
    pub note_type: NoteType,
    pub pitch: u8,
    pub velocity: u8,
}
