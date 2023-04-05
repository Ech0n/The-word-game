use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use std::sync::Mutex;
use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct BoardState{
    cells:Mutex<[[u8;15];15]>,
}

#[derive(Deserialize,Debug)]
struct Move{
    letters: [u8;15],
    column: bool,
    num:u8,
}

#[post("/word")]
async fn word(req_body: String , data: web::Data<BoardState> ) -> Result<HttpResponse, Error> {

    let v: Move = serde_json::from_str(&req_body)?;
    let mut cells = data.cells.lock().unwrap();
    for i in 0..15{
        if v.letters[i]!=0
        {
            if v.column {
                cells[i][v.num as usize] = v.letters[i];
            }
        }
    }
    println!("{:?}",cells);
    Ok(HttpResponse::Ok().body("Halo"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let board = web::Data::new( BoardState {
        cells: Mutex::new([[0;15];15]),
    });
    HttpServer::new(move || {
        //ONLY USE THIS SETTING IN PRODUCTION
        let cors = Cors::permissive();
        App::new()
            .app_data(board.clone())
            .wrap(cors)
            .service(word)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}