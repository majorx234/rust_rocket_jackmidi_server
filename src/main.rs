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

#[post("/midi", format = "json", data = "<data>")]
fn midi(data: Json<MidiNote>) -> String {
    let input: String = format!(
        "MidiNote:{} {} {}",
        data.note_type, data.pitch, data.velocity
    );
    println!("{}", input);
    "Ok".to_string()
}

// async mainfunction
#[launch]
fn rocket() -> _ {
    // routes macro generates routes of webrequests
    println!("Start Rocket Midi Server!");
    rocket::build().mount("/api", routes![midi,])
}
