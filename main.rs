extern crate rusqlite;
extern crate serde;
extern crate serde_json;


use std::io::prelude::*; //get access to certain traits allow read from and write to the stream
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;  // standard filesystem module

// use rusqlite 
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use rusqlite::params;

// for JSON serialization
//use serde::{Serialize, Deserialize};
//use serde_json::{Value};
//use serde_json::Error as Serde_JSON_Error;

use std::collections::HashMap;


fn main() -> Result<()> {
  // Database related stuff
  let conn = Connection::open("storage.db")?;
  init_db(&conn);

  // Establish port  
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream, &conn);
  }

  Ok(())
}

fn init_db(connect: &rusqlite::Connection) -> Result<()>{

  connect.execute(
      "create table if not exists storage (
           stackname  text not null,
           data       text not null
       )",
      NO_PARAMS,
  )?;

  Ok(())

}

//insert into storage (stackname, data) values ('abc', '123');
fn stack_push(connect: &rusqlite::Connection, dataMap: HashMap<String, String>) -> Result<String> {

  println!("stack_push: dataMap = {:?}", dataMap);

  connect.execute(
      "insert into storage (stackname, data) 
          values (?1, ?2)",
      params![dataMap["stackname"], dataMap["data"]]
  )?;

  Ok(format!("\nPushed '{}' onto stack '{}'\n", dataMap["data"], dataMap["stackname"]))

}

//select stackname, data, min(rowid) from storage where stackname = 'abc';
//delete from storage where rowid = (select min(rowid) and stackname = 'abc' from storage);
fn stack_pop(connect: &rusqlite::Connection, dataMap: HashMap<String, String>) -> Result<(String)>{

  println!("stack_pop: dataMap = {:?}", dataMap);

  /**
  let pop_data:String = connect.query_row_and_then(
      "SELECT stackname, data, MAX(rowid) FROM storage WHERE stackname='?';",
      params![dataMap["stackname"]],
      |row| row.get(0),
  ).unwrap();
  **/

  let pop_data:Result< String > = connect.query_row_and_then(
      "SELECT data, MAX(rowid) FROM storage WHERE stackname= ?1;",
      params![dataMap["stackname"]],
      |row| row.get(0),
  );
  println!("pop result\n");
  //println!("pop result: {:?}\n", &pop_data.unwrap());

  connect.execute(
      "DELETE FROM storage WHERE
         rowid = (SELECT max(rowid) FROM storage) AND stackname = ?1;",
      params![dataMap["stackname"]],
  )?;

  //Ok( format!("{}", &pop_data.unwrap()) )
  Ok(  match pop_data {
    Ok(pop_data) => pop_data,
    Err(error) => format!("Stack Underflow!"),
  })

}


//select stackname, data where stackname = 'abc';
fn stack_peek(connect: &rusqlite::Connection, dataMap: HashMap<String, String>) -> Result<String>{

  //let mut stmt = connect.prepare(
//    "SELECT data FROM storage WHERE stackname= ?1 ORDER BY rowid DESC';"   
  //)?;

  //let out_string:String = "".to_string();
  //let mut data_iter = stmt.query(params![dataMap["stackname"]])?;

  //let datum = Vec::new();
  //for result_data in data_iter { 
  //  datum.push(result_data.get(0)?);
  //  let out_string = format!("{}\n{}", result_data.get(0)?, out_string);
  //}

  //println!("result of PEEK: {}", out_string);

  //Ok(out_string)
  Ok("PEEK test".to_string())

}

fn attempt_json_parse (input_str: String) -> HashMap<String, String> {
        //let val:String = serde_json::from_str(&input_str).unwrap();
    println!("attempt_json_parse:input_str -> ##{}##\n", input_str);

    //let val: HashMap<String, String> =  HashMap::new();
    //let val:Stackdata = Stackdata {stackname:"abc".to_string(), data:"test".to_string()};
    //let val = serde_json::from_str(&input_str)?;
    
    //const MSG: &str = r#"{ 
    //"stackname" : "abc" ,
    //"data" : "xyz" 
    //}"#;

    let val:HashMap<String, String> = serde_json::from_str(&input_str).unwrap();

    //if input_str.len() != 0 {
      // MUST SPECIFY THE RETURN TYPE!!!!  ARGH!
      //let val:HashMap<String, String> = match serde_json::from_str(&input_str) {
        //Ok(val) => val,
        //Err(error) => { panic!("Problem with reading JSON from string: < {} >", error) }
      //};

      //println!("\n Parsed Value: {:#?}", val);
    //}

    println!("\n Parsed Value: {:#?}", val);
    val
}

fn get_json_bracket(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = std::usize::MAX;

    for (i, &item) in bytes.iter().enumerate() {
      match &item {
        b'{' => { start = i; },
        b'}' => { if start != std::usize::MAX {println!("start: {} - end: {}\n", start, i+1); return &s[start..i+1]; } },
        _ => {}
      }
    }

    &s[0..0]
}

fn handle_connection(mut stream:TcpStream, connect: &rusqlite::Connection) {
  let mut buffer = [0; 512];

  stream.read(&mut buffer).unwrap();

  // Show the RAW request  
  println!("Request:\n{}\n", String::from_utf8_lossy(&buffer[..]));

  // attempt_json_parse
  let json_data:HashMap<String, String> = attempt_json_parse(get_json_bracket(&format!("{}", String::from_utf8_lossy(&buffer[..]).to_string())).to_string());

  println!("\n in Main - Parsed Value: {:#?}", json_data);
  /**
  let raw_string = first_json_bracket(&format!("{}", String::from_utf8_lossy(&buffer[..]).to_string()));
  attempt_json_parse(raw_string.to_string());
  **/

  // A Blank OK response
  //let response = "HTTP/1.1 200 OK\r\n\r\n";

  // SIMPLE response page
  //let contents = fs::read_to_string("hello.html").unwrap();
  //let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
  //stream.write(response.as_bytes()).unwrap();
  //stream.flush().unwrap();

  // b"" is turning the string into a byte string"
  //let get = b"GET / HTTP/1.1\r\n";
  let get = b"GET";
  let push = b"UPDATE /api/stack";
  let pop = b"GET /api/stack";
  let peek = b"GET /api/stack/peek";
   
  let (status_line, filename, result) = { 
    if buffer.starts_with(push) { ("HTTP/1.1 200 OK\r\n\r\n", "push.html", stack_push(&connect, json_data).unwrap()) }
    else if buffer.starts_with(peek) { ("HTTP/1.1 200 OK\r\n\r\n", "peek.html", "peek".to_string()) }
    else if buffer.starts_with(pop) { ("HTTP/1.1 200 OK\r\n\r\n", "pop.html", stack_pop(&connect, json_data).unwrap()) }
    else if buffer.starts_with(get) { ("HTTP/1.1 200 OK\r\n\r\n", "hello.html", "Unknown GET".to_string()) }
    else { ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html", "Incorrect input".to_string()) }
  };

  

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!("{}\n{}\n** {} **\n", status_line, contents, result);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();

}