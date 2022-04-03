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

fn process_cmd(cmd: &str) -> () {
    match cmd {
        "daily" => println!("0"),
        "weekly" => println!("1"),
        "all" => println!("2"),
        "my" => println!("3"),
        "leaderboard" => println!("4"),
        _ => println!(
                "Available commands:\n\nhelp - show help menu\ndaily - show daily stats\nweekly - show weekly stats\nall - show all time stats\nmy - show personal stats\nleaderboard - show ranked leaderboard\n"
            ),
    }
}

async fn wordle(msg: web::Json<Msg>) -> HttpResponse {
    if WORDLE_SCORE.is_match(&msg.text) {
        println!("WORDLE SCORE DETECTED")
    } else if WORDLE_CMD.is_match(&msg.text) {
        let vec: Vec<&str> = msg.text.split_whitespace().collect();
        let cmd: &str = if vec.len() >= 2 { vec[1] } else { "" };
        process_cmd(cmd)
    }
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(wordle)))
        .bind(("0.0.0.0", 9300))?
        .run()
        .await
}