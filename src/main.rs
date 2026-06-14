use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 7177;

fn get_file_path() -> String {
   // need to get a file path here 
   return "~/Desktop/test.txt".to_string();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let data = &buffer[..bytes_read];
    println!("Received: {}", String::from_utf8_lossy(data));

    let mut file = File::open(get_file_path()).unwrap();
    let mut file_buff = [0u8; 1024];
    
    let header = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: application/pdf\r\n\
        Content-Disposition: attachment; filename='test.txt'\r\n\r\n",
        file.metadata().unwrap().len()
    );
    let mut stream = stream;
    stream.write_all(header.as_bytes()).unwrap();

    loop {
        let bytes_read = file.read(&mut file_buff).unwrap();
        if bytes_read == 0 {
            break;
        }
        stream.write_all(&file_buff[..bytes_read]).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", ADDRESS, PORT)).unwrap();
    println!("SERVER LISTING ON.... {}:{}", ADDRESS, PORT);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                print!("New connection..!");
                handle_connection(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}