use std::io::Read;
use std::io::Write;
use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 200];

    let answer = "HTTP/1.1 200 OK
Date: Sun, 10 Oct 2010 23:26:07 GMT
Server: Lea/0.1.0 2021
Last-Modified: Sun, 26 Sep 2010 22:04:35 GMT
Accept-Ranges: bytes
Content-Length: 13
Connection: close
Content-Type: text/html
        

Hello world!
";

    while match stream.read(&mut data) {
        Ok(_) => {
            println!("{}", std::str::from_utf8(&data).unwrap());
            stream.write(answer.as_bytes()).unwrap();
            stream.shutdown(Shutdown::Write).unwrap();
            false
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    drop(listener);
    Ok(())
}
