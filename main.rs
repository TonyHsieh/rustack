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

// for JSON serialization
//use serde::{Serialize, Deserialize};
//use serde_json::{Value};
use serde_json::Error as Serde_JSON_Error;

use std::collections::HashMap;

fn main() -> Result<()> {
  // Database related stuff
  let conn = Connection::open("storage.db")?;
  init_db(conn);

  // Establish port  
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }

  Ok(())
}

fn init_db(connect: rusqlite::Connection) -> Result<()>{

  connect.execute(
      "create table if not exists storage (
           id         integer primary key,
           stackname  text not null,
           data       text not null
       )",
      NO_PARAMS,
  )?;

  Ok(())

}

fn attempt_json_parse (input_str: String) -> Result<()> {
    // Try to parse this string???
    //let val:String = serde_json::from_str(&input_str).unwrap();
    println!("attempt_json_parse:input_str -> ##{}##", input_str);

    // let val:String = serde_json::from_str(&input_str)?;
    
    if input_str.len() != 0 {
      let val:HashMap<String, String> = match serde_json::from_str(&input_str) {
        Ok(val) => val,
        Err(error) => { panic!("Problem with reading JSON from string: {:?}", error) }
      };

      //println!("\n Username: {}", val["username"]);
      println!("\n Username: {:#?}", val);
    }

    Ok(())
}

fn get_json_bracket(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        match &item {
          b'{' => { start = i; println!("start = {}", start); },
          b'}' => { if start != 0 { return &s[start..i+1]; } },
          _ => {}
        }
        
        //if item == b'{' {
        //  start = i;
        //  println!("start = {}", start);
        //}
        //if item == b'}' {
        //  return &s[start..i+1];
        //}
    }

    &s[0..0]
}

fn handle_connection(mut stream:TcpStream) {
   let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // Show the RAW request  
    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));
    

    // attempt_json_parse
    attempt_json_parse(get_json_bracket(&format!("{}", String::from_utf8_lossy(&buffer[..]).to_string())).to_string());

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

    /**
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
      let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
      let contents = fs::read_to_string("404.html").unwrap();

      let response = format!("{}{}", status_line, contents);

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
    }
    // **/

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}