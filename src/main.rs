use std::{io::Read, net::TcpStream};
mod lib;

fn main() {
    let addr = "127.0.0.1:8080";
    let handler = |mut x :TcpStream| {
        println!("Working handler!!!");
        let mut request = [0; 1024];
        x.read(&mut request).unwrap();
        println!("message: {}", String::from_utf8_lossy(&request[..]));
        Ok(())
    };

    let server = lib::Server::new(addr, handler);
    match server.run() {
        Ok(_) => println!("Receve successed"),
        Err(_) => println!("Receve failed"),
    }
}
