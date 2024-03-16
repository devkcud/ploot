use std::{
    io::{Error, ErrorKind, Result},
    net::TcpListener,
};

pub struct Listener;

impl Listener {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: Refactor
    // TODO: Handle errors
    pub fn listen(&self, port: &str) -> Result<()> {
        if !port.contains(":") {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Must include ':'\nExample: `127.0.0.1:3000` or `:3000`",
            ));
        }

        let split = port.split(":").collect::<Vec<&str>>();

        let hostname = match *split.get(0).unwrap() {
            "" => "localhost",
            _ => split.get(0).unwrap(),
        };

        // FIXME: Passible string '3000:' -> Which is not valid
        let listener =
            TcpListener::bind(format!("{}:{}", hostname, split.get(1).unwrap())).unwrap();

        println!(
            "Listening at {}",
            listener.local_addr().unwrap().to_string()
        );

        for stream in listener.incoming() {
            println!("{:#?}", stream);
        }

        Ok(())
    }
}
