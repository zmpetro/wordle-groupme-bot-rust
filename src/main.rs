#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer, HttpResponse};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Msg {
    name: String,
    text: String,
    user_id: String,
}

lazy_static! {
    static ref WORDLE_SCORE: Regex = {
        Regex::new(r"^Wordle\s\d+\s[1-6X]/6").unwrap()
    };
    static ref WORDLE_CMD: Regex = {
        Regex::new(r"^/wordle").unwrap()
    };
}

async fn wordle(msg: web::Json<Msg>) -> HttpResponse {
    if WORDLE_SCORE.is_match(&msg.text) {
        println!("WORDLE SCORE DETECTED")
    } else if WORDLE_CMD.is_match(&msg.text) {
        println!("WORDLE COMMAND DETECTED")
    }
    // Change this to logging
    print!("name: {}\ntext: {}\nuser_id: {}\n\n",
                msg.name,
                msg.text,
                msg.user_id);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(wordle)))
        .bind(("0.0.0.0", 9300))?
        .run()
        .await
}