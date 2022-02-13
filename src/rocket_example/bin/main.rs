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

// async mainfunction
#[launch]
fn rocket() -> _ {
    // routes macro generates routes of webrequests
    rocket::build().mount("/api", routes![hello_world, hello_name])
}
