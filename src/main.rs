use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for streams in listener.incoming() {
        match streams {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let mut buffer = [0; 512];

                match _stream.read(&mut buffer) {
                    Ok(_) => {
                        let request = String::from_utf8_lossy(&buffer);
                        println!("Request: {}", request);

                        let request_line = request.lines().next().unwrap_or("");

                        // Split the first line to extract method, URL, and HTTP version
                        let parts: Vec<&str> = request_line.split_whitespace().collect();

                        // Check if we have at least 3 parts (method, URL, HTTP version)
                        if parts.len() >= 3 {
                            let method = parts[0];
                            let url = parts[1];
                            let http_version = parts[2];

                            // Print the extracted method, URL, and HTTP version
                            println!(
                                "Method: {}, URL: {}, HTTP Version: {}",
                                method, url, http_version
                            );

                            if url == "/" {
                                // _stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                                _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
                            } else {
                                _stream
                                    .write(b"HTTP/1.1 404 Not Found\r\n\r\n")
                                    .expect("404 \n");
                            }
                        } else {
                            println!("Malformed HTTP request");
                        }
                    }
                    Err(err) => {
                        println!("error: {}", err)
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
