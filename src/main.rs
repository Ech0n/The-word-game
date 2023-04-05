use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use std::sync::Mutex;
use serde_json::{Value};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct BoardState{
    cells:Mutex<[[u8;15];15]>,
}

// #[derive(Deserialize,Debug)]
// struct Move{
//     letters: [String;15],
//     column: bool,
//     num:u8,
// }

#[post("/word")]
async fn word(req_body: String ) -> Result<HttpResponse, Error> {

    let v: Value = serde_json::from_str(&req_body)?;

    println!("{} ",v["letters"]);
    Ok(HttpResponse::Ok().body("Halo"))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        //ONLY USE THIS SETTING IN PRODUCTION
        let cors = Cors::permissive();
        let board = web::Data::new( BoardState {
            cells: Mutex::new([[0;15];15]),
        });
        App::new()
            .app_data(board.clone())
            .wrap(cors)
            .service(word)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}