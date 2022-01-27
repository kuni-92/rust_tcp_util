use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::io::Result;

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

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        
        for stream in listener.incoming() {
            (self.handler)(stream?);
        }
        Ok(())
    }
}
