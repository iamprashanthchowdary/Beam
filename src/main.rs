use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use qrcode::{render::unicode, QrCode};
use std::process;



const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 7177;

//user this func to generate a QR code from a URL
fn generate_qr_code(data: &str) -> String {
    let qr_code = QrCode::new(data).unwrap();
    let qr_code = qr_code.render::<unicode::Dense1x2>().build();
    qr_code
}

fn get_file_path() -> String {
   // need to get a file path here 
   let args: Vec<String> = std::env::args().collect();
      let path = match args.get(1) {
          Some(p) => p,
          None => {
              eprintln!("No file path provided");
              process::exit(1);
          }
      };
      println!("{:?}", &args);
   return path.to_string();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let data = &buffer[..bytes_read];
    println!("Received: {}", String::from_utf8_lossy(data));

    let file_path = get_file_path();
    let mut file = File::open(&file_path).unwrap();
    let mut file_buff = [0u8; 1024];
     
    let file_name = std::path::Path::new(&file_path).file_name().and_then(|n| n.to_str()).unwrap_or("file_via_beam");
    let file_size = file.metadata().unwrap().len();
    println!("file_name: {}", file_name);
    let header = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: application/pdf\r\n\
        Content-Disposition: attachment; filename={}\r\n\r\n",
        file_size, file_name
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