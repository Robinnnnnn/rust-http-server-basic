
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
        println!("Listening on {}", self.addr)
    }
}
