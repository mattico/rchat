use std::net::{UdpSocket, SocketAddr};
use std::str::from_utf8;
use std::collections::HashMap;
use std::io;
use chrono::*;

mod packet;

extern crate chrono;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:6666").unwrap();

    let mut clients = HashMap::<SocketAddr, DateTime<UTC>>::new();

    let ttl = Duration::seconds(15);

    let mut console = io::stdin();

    loop {
        let mut buf = [0u8; 32];
        let (_, src) = socket.recv_from(&mut buf).unwrap();

        if clients.contains_key(&src) {
            *clients.get_mut(&src).unwrap() = UTC::now();
        } else {
            clients.insert(src, UTC::now());
        }
        println!("{}: {}", src, from_utf8(&buf).unwrap());

        // remove clients that haven't ACKed in a while
        for (addr, time) in clients.clone().iter() {
            if *time - UTC::now() > ttl {
                clients.remove(&addr);
            }
        }

        let mut line = String::new();
        while let Ok(len) = console.read_line(&mut line) {
            for (addr, _) in clients.clone() {
                let _ = socket.send_to(line.trim_right().as_bytes(), &addr);
            }

            if len == 0 {
                break;
            }

            line = String::new();
        }
    }
}
