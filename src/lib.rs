use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::io::Result;

/// `addr` is local endpoint. e.g. "127.0.0.1:8080".
/// `handler` is working when connect to local endpoint.
pub struct Server<A, T>
    where A: ToSocketAddrs, T: Fn(TcpStream) -> Result<()>
{
    addr: A,
    handler: T,
}

impl <A, T> Server<A, T>
    where A: ToSocketAddrs,T: Fn(TcpStream) -> Result<()>
{
    /// Set server parametors.
    /// `addr` is listening address for server.
    /// `handler` is working when connected to server.
    pub fn new(addr: A, handler:T) -> Self {
        Server {
            addr,
            handler,
        }
    }

    /// The server running.
    /// Bind to local endpoint that be specified to `addr`.
    /// When the server is connected, The server will work
    /// the action specified to `handler`.
    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        
        for stream in listener.incoming() {
            match (self.handler)(stream?) {
                Ok(_) => println!("message receved!"),
                Err(_) => println!("message receved error."),
            }
        }
        Ok(())
    }
}
