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
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum NoteType {
    NoteOn,
    NoteOff,
}

#[derive(Deserialize)]
struct MidiNote {
    note_type: NoteType,
    pitch: u8,
    velocity: u8,
}

#[post("/midi", data = "<data>")]
fn midi_note(data: String) -> String {
    format!("got note {}!", data)
}

// async mainfunction
#[launch]
fn rocket() -> _ {
    // routes macro generates routes of webrequests
    println!("Start Rocket Midi Server!");
    rocket::build().mount("/api", routes![midi_note,])
}
