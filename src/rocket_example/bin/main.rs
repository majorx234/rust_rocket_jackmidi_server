#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use reqwest;
use rocket::{
    catch, catchers,
    fairing::{self, Fairing, Info, Kind},
    get,
    http::{Cookie, CookieJar, Method},
    launch, post,
    response::{content::Html, status::Created},
    routes,
    serde::json::Json,
    uri, Build, Data, Request, Rocket, State,
};
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
    let index2: Option<usize> = match user.binary_search(&name) {
        Ok(index) => Some(index),
        Err(_) => None,
    };

    let index: i32 = match index2 {
        Some(index) => index as i32,
        None => -1,
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
    rocket::build().mount("/api", routes![hello_world, hello_name, query_name,])
}
