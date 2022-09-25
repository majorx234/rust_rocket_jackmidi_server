#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use reqwest;
use rocket::{
    catch, catchers,
    fairing::{self, Fairing, Info, Kind},
    get,
    http::{Cookie, CookieJar, Method},
    launch, post,
    response::{content, status::Created},
    routes,
    serde::json::Json,
    uri, Build, Data, Request, Rocket, State,
};
//use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fmt;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

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
struct MidiNote {
    note_type: NoteType,
    pitch: u8,
    velocity: u8,
}

#[post("/midi", format = "json", data = "<data>")]
fn midi_note(data: Json<MidiNote>) -> String {
    let input: String = format!(
        "MidiNote:{} {} {}",
        data.note_type, data.pitch, data.velocity
    );
    println!("{}", input);
    format!("got note {}!", input)
}

// async mainfunction
#[launch]
fn rocket() -> _ {
    // routes macro generates routes of webrequests
    println!("Start Rocket Midi Server!");
    rocket::build().mount("/api", routes![midi_note,])
}
