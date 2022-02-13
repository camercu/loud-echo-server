use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    // bind to port 3246, or "echo" typed on phone keypad
    let listener = TcpListener::bind("127.0.0.1:3246").expect("bind failed");

    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                thread::spawn(move || handle(conn));
            }
            Err(e) => {
                eprintln!("connection failed: {}", e);
            }
        }
    }
}

fn handle(mut conn: TcpStream) {
    let mut buf = vec![0u8; 4096];
    let minute = Some(Duration::from_secs(60));
    conn.set_read_timeout(minute)
        .expect("set read timeout failed");
    conn.set_write_timeout(minute)
        .expect("set write timeout failed");

    loop {
        // read data from client
        let len = match conn.read(buf.as_mut_slice()) {
            Err(_) => break,
            Ok(n) if n == 0 => break, // EOF
            Ok(n) => n,
        };

        // transform bytes to uppercase
        for b in buf.iter_mut().take(len) {
            b.make_ascii_uppercase();
        }

        // write to client
        match conn.write_all(&buf.as_mut_slice()[..len]) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}
