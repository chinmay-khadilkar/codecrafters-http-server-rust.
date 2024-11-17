#[allow(unused_imports)]
use std::net::{ TcpStream, TcpListener};
use std::io::{ Write, BufReader, BufRead };

enum StatusCode {
    Success,
    NotFound
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
             Ok(mut stream) => {
                 println!("accepted new connection");
                 let status_code = handle_connection(&mut stream);
                 match status_code {
                    StatusCode::Success => {
                        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                    },
                    StatusCode::NotFound => {
                        stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                    }
                 }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}

fn handle_connection (stream: &mut TcpStream) -> StatusCode {
    let buffer = BufReader::new(stream);
    let http_request: Vec<String> = buffer.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
    let request_line: Vec<String> = http_request[0].split(" ").map(|item| item.to_string()).collect();

    if request_line[1] == "/" {
        StatusCode::Success
    } else {
        StatusCode::NotFound
    }
}
