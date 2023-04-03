

fn main() {

    let server = Server::new(String::from("127.0.0.1:8080"));

    server.run();
}

struct Server {
    addr: String,

}

impl Server {
    // Uppercase Self is an alias for the struct of this impl.
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    //methods always accept self as first parameter
    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method, 
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}
