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
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

// Basic static Get Request
#[get("/test")]
fn hello_world() -> &'static str {
    "Hello world!"
}

// dynamic get equest with variable
#[get("/name/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello {}!", name)
}

#[derive(Deserialize)]
struct MidiNote {
    noteOnOff: String,
    pitch: u8,
    velocity: u8,
}

#[post("/midi", data = "<data>")]
fn midi_note(data: String) -> String {
    format!("got note {}!", data)
}

#[post("/logout", data = "<data>")]
fn logout(data: String) -> String {
    format!("got note {}!", data)
}

#[get("/search?<name>&<salutation>")]
fn query_name(name: String, salutation: Option<String>) -> String {
    let mut user: Vec<String> = vec![
        "Joe".to_string(),
        "Donald".to_string(),
        "Hillary".to_string(),
        "Bill".to_string(),
        "Barack".to_string(),
    ];

    // binsearch needs sorted array
    user.sort_unstable();
    let index: i32 = match user.binary_search(&name) {
        Ok(index) => index as i32,
        Err(_) => -1,
    };

    match salutation {
        Some(s) => format!("{} {} is user index: {}", s, name, index),
        None => format!("Hello {} with index {}", name, index),
    }
}

// async mainfunction
#[launch]
fn rocket() -> _ {
    // routes macro generates routes of webrequests
    rocket::build().mount(
        "/api",
        routes![hello_world, hello_name, query_name, midi_note, logout,],
    )
}
