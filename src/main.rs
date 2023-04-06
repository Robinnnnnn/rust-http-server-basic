#![allow(dead_code)]


use http::Method;
use http::Request;
use server::Server; //user Server struct from the mod server

mod http; //finds files/folders with modules 
mod server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));

    server.run();
}

//modules are private by default
//other files are considered as modules in rust
// when inserting modules as serperate files, you dont need to keep mod syntax,
// you only need the contents of the module
