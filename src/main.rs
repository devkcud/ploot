use ploot::{response::ResponseConstructor, server};
use std::{
    io::{Result, Write},
    net::TcpStream,
};

fn home(mut stream: TcpStream) -> Result<()> {
    stream.write_all(&ResponseConstructor::new().set_content("Hi").build())
}

fn main() {
    let mut server = server::Listener::new();

    server.GET("/", home);

    match server.listen(":1234") {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
