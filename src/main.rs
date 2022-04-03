use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Msg {
    name: String,
    text: String,
    user_id: String,
}

async fn index(msg: web::Json<Msg>) -> Result<String> {
    print!("\nname: {}\ntext: {}\nuser_id: {}\n",
                msg.name,
                msg.text,
                msg.user_id);
    Ok(format!("Welcome {}!", msg.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("0.0.0.0", 9300))?
        .run()
        .await
}