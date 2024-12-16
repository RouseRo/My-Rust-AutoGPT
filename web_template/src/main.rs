
#[allow(unused_imports)] 
use actix_cors::Cors;
#[allow(unused_imports)] 
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
#[allow(unused_imports)] 
use serde::{Deserialize, Serialize};

#[allow(unused_imports)] 
use std::sync::Mutex;
#[allow(unused_imports)] 
use std::collections::HashMap;
#[allow(unused_imports)] 
use std::fs;
#[allow(unused_imports)] 
use std::io::Write;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>
}
fn main() {
    println!("Hello, world!");
}
