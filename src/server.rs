use std::{net::TcpListener, io::Read};
use crate::http::{Request, request};
use std::convert::TryFrom;
pub struct Server {
    addr: String,
}

impl Server {
    // Uppercase Self is an alias for the struct of this impl.
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    //methods always accept self as first parameter
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        //creating the socket
        let listener = TcpListener::bind(&self.addr).unwrap();

        //listening to accept connections
        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a Request: {}", String::from_utf8_lossy(&buffer));

                            //need to explicity
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to Parse Request: {}", e),
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                Err(e) => println!("{}", e),
                
            }
        }
    }
}
