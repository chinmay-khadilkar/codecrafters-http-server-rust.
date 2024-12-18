#[allow(unused_imports)]
use std::net::{ TcpStream, TcpListener};
use std::io::{ Write, BufReader, BufRead, Read };
use std::{env, fs};
use std::path::Path;
use std::fs::File;

enum StatusCode {
    Success,
    NotFound,
    SuccessBody{content_len: u8, content: String},
    OctateSuccess{content_len: usize, content: String},
    Created
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                 println!("accepted new connection");
                 process_stream(stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}

fn handle_connection (stream: &mut TcpStream) -> StatusCode {
    // let mut buffer = BufReader::new(stream);
    let mut data: Vec<u8> = Vec::new();
    stream.read_to_end(&mut data);
    let mut entire_request = String::from_utf8(data).unwrap();
    // buffer.read_to_string(&mut entire_request);
    
    let req_vec:Vec<String> = entire_request.split("\r\n").map(|item| item.to_string()).collect();
    println!("{:?}", req_vec);
    // let http_request: Vec<String> = buffer.lines().map(|line| line.unwrap()).collect();
    let request_line: Vec<String> = req_vec[0].split(" ").map(|item| item.to_string()).collect();
    // let empty_pos = req_vec.iter().position(|item| item == String::from(""));
    let content_body = req_vec[req_vec.len() - 1].clone();

    if request_line[0].starts_with("POST") {
        let content:Vec<String> = request_line[1].split("/").map(|item| item.to_string()).collect();
        let file_name = content[content.len() - 1].clone();
        let env_args: Vec<String> = env::args().collect();
        let dir = env_args[2].clone();
        let file_path = Path::new(&dir).join(file_name);
        let prefix = file_path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut f = File::create(&file_path).unwrap();
        f.write_all(content_body.as_bytes()).unwrap();
        println!("{:?}", content_body);
        StatusCode::Created
        
    } else if request_line[1] == "/" {
        StatusCode::Success
    } else if request_line[1].starts_with("/echo") {
        
        let content:Vec<String> = request_line[1].split("/").map(|item| item.to_string()).collect();
        let response_body = content[content.len() - 1].clone();
        StatusCode::SuccessBody {
            content_len: response_body.len() as u8,
            content: response_body as String
        }
    } else if request_line[1].starts_with("/user-agent") {
        let content:Vec<String> = req_vec[req_vec.len() - 1].split(" ").map(|item| item.to_string()).collect();
        let response_body = content[content.len() - 1].clone();
        StatusCode::SuccessBody {
            content_len: response_body.len() as u8,
            content: response_body as String
        }
    } else if request_line[1].starts_with("/files") {
        let content:Vec<String> = request_line[1].split("/").map(|item| item.to_string()).collect();
        let files = content[content.len() - 1].clone();
        let env_args: Vec<String> = env::args().collect();
        let mut dir = env_args[2].clone();
        dir.push_str(&files);
        let file = fs::read(dir);
        match file {
            Ok(fc) => {
                StatusCode::OctateSuccess {
                    content_len: fc.len(),
                    content: String::from_utf8(fc).expect("file content")
                }
            },
            Err(..) => {
                StatusCode::NotFound
            }
        }
    } else {
        StatusCode::NotFound
    }
}

fn process_stream (mut stream: TcpStream) {
    let status_code = handle_connection(&mut stream);
    match status_code {
        StatusCode::Success => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        },
        StatusCode::NotFound => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        },
        StatusCode::SuccessBody{content_len, content} => {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",content_len, content);
            stream.write(response.as_bytes()).unwrap();
        },
        StatusCode::OctateSuccess{content_len, content} => {
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",content_len, content);
            stream.write(response.as_bytes()).unwrap();
        },
        StatusCode::Created => {
            println!("code comes here");
            stream.write("HTTP/1.1 201 Created\r\n\r\n".as_bytes()).unwrap();
        }
    }
    println!("Writing response to stream...");
    stream.flush().unwrap();
}
