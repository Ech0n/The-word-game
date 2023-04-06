use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use std::sync::Mutex;
use serde::{Deserialize};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

struct BoardState{
    cells:Mutex<[[u8;15];15]>,
}

//Todo : implement DAWG structure as its far more efective than simple word list
struct WordsDict{
    words:Vec<String>,
}

#[derive(Deserialize,Debug)]
struct Move{
    letters: [u8;15],
    column: bool,
    num: usize,
}
fn calc_col(col : bool,i : usize,num:usize) -> usize
{
    num*((col )as usize)+(i * !col as usize)
}
fn calc_row(col : bool,i : usize,num:usize) -> usize
{
    i*col as usize+(num * (!col) as usize)
}

#[post("/word")]
async fn word(req_body: String , data: web::Data<BoardState>, wordsState: web::Data<WordsDict> ) -> Result<HttpResponse, Error> {

    let v: Move = serde_json::from_str(&req_body)?;
    let mut cells = data.cells.lock().unwrap();
    let mut words_to_check : Vec<String> = Vec::new();
    let mut temp : String = String::new();
    let mut i = 0;
    let mut col:usize = 0;
    let mut row:usize = 0;

    // Sprawdzanie wyrazu cells[num*column as u8+(i * !column as u8)][i*column as u8+(num * !column as u8)]
    while v.letters[i] == 0 && i<15
    {
        i +=1;
    }
    //Doszlismy do poczatku wyrazu 
    // Teraz sie cofamy aby sprawdzic czy przed wyrazem znajduja sie juz jakies litery
    let mut j = i-1;
    col = calc_col(v.column,j,v.num);
    row = calc_row(v.column,j,v.num);
    while j>0 &&  cells[col][row] != 0
    {
        println!("Cofamy sie!");
        temp.insert_str(0, &(cells[col][row] as char).to_string() );
        j-=1;
        col = calc_col(v.column,j,v.num);
        row = calc_row(v.column,j,v.num);
    }
    col = calc_col(v.column,i,v.num);
    row = calc_row(v.column,i,v.num);
    while i<15 && (v.letters[i] | cells[col][row])!= 0
    {
        temp.push((v.letters[i] | cells[col][row])  as char);
        cells[col][row] |= v.letters[i]; 

        //Check if any word was created in other direction
        if v.letters[i] != 0
        {    
            let mut new_word = String::new();
            let mut k = 1;
            let mut other_direction_col = calc_col(v.column,i,v.num-k);
            let mut other_direction_row = calc_row(v.column,i,v.num-k);
            while v.num >=k && cells[other_direction_col][other_direction_row] !=0
            {
                new_word.insert_str(0, &(cells[other_direction_col][other_direction_row] as char).to_string() );
                k+=1;
                other_direction_col = calc_col(v.column,i,v.num-k);
                other_direction_row = calc_row(v.column,i,v.num-k);
            }
            new_word.push( v.letters[i] as char );
            k = 1;
            other_direction_col = calc_col(v.column,i,v.num-k);
            other_direction_row = calc_row(v.column,i,v.num-k);
            while v.num+k<15 && ( cells[other_direction_col][other_direction_row])!=0
            {
                new_word.push(cells[other_direction_col][other_direction_row] as char);
                k+=1;
                other_direction_col = calc_col(v.column,i,v.num+k);
                other_direction_row = calc_row(v.column,i,v.num+k);
            }
            println!("New word in other direction: {}",new_word);
            if new_word.len() > 1 && !wordsState.words.contains(&new_word)
            {
                return Ok(HttpResponse::Ok().body("{valid:false , which: ".to_string()+ &new_word+"}"));
            }
        }

        //increment
        i+=1;
        col = calc_col(v.column,i,v.num);
        row = calc_row(v.column,i,v.num);
    }
    
    println!("Nowe slowo: {}",temp);
    if !wordsState.words.contains(&temp)
    {
        println!("Word does not exist {}, {}",temp, temp.len());
        Ok(HttpResponse::Ok().body("{valid:false}"))
    }else
    {
        Ok(HttpResponse::Ok().body("{valid:true}"))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //open file containing words
    // In production its best to load a test file because a larga file might take a while to load
    let f = File::open("./src/test_words.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();
    let mut words : Vec<String> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let mut tempstr : String = String::new();
    println!("Loading words!");
    for value in buffer {
        if value as char == '\n'{
            words.push(tempstr.trim().to_string());
            tempstr = String::new();
        }else
        {
            tempstr.push( value as char);
        }
    }
    println!("Finished loading words!");
    let words = web::Data::new(WordsDict{
        words: words,
    });
    let board = web::Data::new( BoardState {
        cells: Mutex::new([[0;15];15]),
    });
    HttpServer::new(move || {
        //ONLY USE THIS SETTING IN PRODUCTION
        let cors = Cors::permissive();
        App::new()
            .app_data(board.clone())
            .app_data(words.clone())
            .wrap(cors)
            .service(word)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}