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
mod midi_note;
use crate::midi_note::MidiNote;
mod jackprocess;
use crate::jackprocess::start_jack_thread;
use std::thread;

//struct ServerState(mpsc::SyncSender<midi_note::MidiNote>);
struct ServerState(flume::Sender<midi_note::MidiNote>);

#[post("/midi", format = "json", data = "<data>")]
fn midi(data: Json<MidiNote>, tx_midi_note: &State<ServerState>) -> String {
    let input: String = format!(
        "MidiNote:{} {} {}",
        data.note_type, data.pitch, data.velocity
    );
    let recv_midi_node = MidiNote {
        note_type: data.note_type,
        pitch: data.pitch,
        velocity: data.velocity,
    };
    tx_midi_note.0.try_send(recv_midi_node).map_err(|_| "Error");
    println!("{}", input);
    "Ok".to_string()
}

#[launch]
fn rocket() -> _ {
    let (tx_midi_note, rx_midi_note) = flume::bounded(32);
    //mpsc::sync_channel::<midi_note::MidiNote>(256);
    let _jack_thread = start_jack_thread(rx_midi_note);

    // routes macro generates routes of webrequests
    println!("Start Rocket Midi Server!");

    rocket::build()
        .mount("/api", routes![midi,])
        .manage(ServerState(tx_midi_note))
}
