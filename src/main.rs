#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
// #[macro_use] extern crate lazy_static;
extern crate rocket_cors;

use rocket::http::Method;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};
use rocket::response::content;

type ID = usize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Message {
    id: ID,
    content: String,
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080/html/",
        "http://localhost:8080",
        "http://best-ball.surge.sh/",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error building cors")
}

#[get("/")]
fn hello() -> JsonValue {
    json!([
        {
            "id":  "001",
            "name": "osama"
        },
        {
            "id": "002",
            "name": "khizar"
        },
        {
            "id": "003",
            "name": "asgher"
        }

    ])
}


#[get("/jsonContant")]
fn json()->content::Json<&'static str>{
    content::Json("{ name: osama qamar}")
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type MessageMap = Mutex<HashMap<ID, String>>;

#[post("/add", data = "<user_input>")]
fn add(user_input: Json<Message>, map: State<'_, MessageMap>)->Option<JsonValue> {
    println!("{:?}", user_input.0.content);
    Some(json!({"status" : "Ok"}))
}
fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![hello, add, json])
        .attach(make_cors())
        //manage to sore data comming from real time
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}
