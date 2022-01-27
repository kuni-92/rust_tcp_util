use std::net::TcpStream;
mod lib;

fn main() {
    let addr = "127.0.0.1:8080";
    let handler = |x :TcpStream| {
        println!("Receve request!");
        Ok(())
    };

    let server = lib::Server::new(addr, handler);
    match server.run() {
        Ok(_) => println!("Receve successed"),
        Err(_) => println!("Receve failed"),
    }
}
