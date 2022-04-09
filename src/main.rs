#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

struct AppData {
    api_url: String,
    bot_id: String,
    client: reqwest::Client,
    pool: DbPool,
}

#[derive(Deserialize)]
struct RcvMsg {
    name: String,
    text: String,
    user_id: String,
}

#[derive(Serialize)]
struct SendMsg {
    bot_id: String,
    text: String,
}

lazy_static! {
    static ref WORDLE_SCORE: Regex = Regex::new(r"^Wordle\s\d+\s[1-6X]/6").unwrap();
    static ref WORDLE_CMD: Regex = Regex::new(r"^/wordle").unwrap();
}

async fn get_all_time_stats(data: web::Data<AppData>) -> String {
    let all_time_stats = web::block(move || {
        let conn = data.pool.get()?;
        actions::get_all_time_stats(&conn)
    })
    .await;

    

    String::from("ALL TIME STATS")
}

#[allow(unused_must_use)]
async fn send_msg(data: &web::Data<AppData>, text: String) -> () {
    println!("RESPONSE:\n{}\n", text);
    let msg = SendMsg {
        bot_id: data.bot_id.clone(),
        text: text,
    };
    data.client
        .post(data.api_url.clone())
        .json(&msg)
        .send()
        .await;
}

async fn process_score(data: &web::Data<AppData>, user_id: &str, name: &str, score: char) -> () {
    println!(
        "RECEIVED SCORE:\nuser_id: {}\nname: {}\nscore: {}/6\n",
        user_id, name, score
    );
}

async fn process_cmd(data: &web::Data<AppData>, user_id: &str, name: &str, cmd: &str) -> () {
    println!(
        "RECEIVED COMMAND:\nuser_id: {}\nname: {}\ncmd: {}\n",
        user_id, name, cmd
    );
    let msg: String = match cmd {
        "daily" => String::from("daily"),
        "weekly" => String::from("weekly"),
        "all" => get_all_time_stats(web::Data::clone(&data)).await,
        "my" => String::from("my"),
        "leaderboard" => String::from("leaderboard"),
        _ => String::from(
            r#"Available commands:

help - show help menu
daily - show daily stats
weekly - show weekly stats
all - show all time stats
my - show personal stats
leaderboard - show ranked leaderboard"#,
        ),
    };
    send_msg(&data, msg).await;
}

async fn wordle(data: web::Data<AppData>, msg: web::Json<RcvMsg>) -> impl Responder {
    if WORDLE_SCORE.is_match(&msg.text) {
        let vec: Vec<&str> = msg.text.split_whitespace().collect();
        let score: char = vec[2].chars().nth(0).unwrap();
        process_score(&data, &msg.user_id, &msg.name, score).await
    } else if WORDLE_CMD.is_match(&msg.text) {
        let vec: Vec<&str> = msg.text.split_whitespace().collect();
        let cmd: &str = if vec.len() >= 2 { vec[1] } else { "" };
        process_cmd(&data, &msg.user_id, &msg.name, cmd).await
    }
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_url = env::var("API_URL").expect("API_URL must be set");
    let bot_id = env::var("BOT_ID").expect("BOT_ID must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let client = reqwest::Client::new();

    let app_data = web::Data::new(AppData {
        api_url: api_url,
        bot_id: bot_id,
        client: client,
        pool: pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&app_data))
            .route("/", web::post().to(wordle))
    })
    .bind(("0.0.0.0", 9300))?
    .run()
    .await
}
