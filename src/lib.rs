use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::Result;

struct Server<A, T>
    where A: ToSocketAddrs, T: Fn() -> Result<()>
{
    addr: A,
    handler: T,
}

impl <A, T> Server<A, T>
    where A: ToSocketAddrs,T: Fn() -> Result<()>
{
    /// Set server parametors.
    /// `addr` is listening address for server.
    /// `handler` is working when connected to server.
    fn new(addr: A, handler:T) -> Self
    {
        Server {
            addr,
            handler,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Server;

    #[test]
    fn create_new_server() {
        let handler = ||{
            Ok(())
        };
        Server::new("127.0.0.1", handler);
    }
}
