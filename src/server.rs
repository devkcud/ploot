use std::{
    io::{Error, ErrorKind, Result},
    net::TcpListener,
};

const DEFAULT_HOSTNAME: &str = "127.0.0.1";

pub struct Listener;

impl Listener {
    pub fn new() -> Self {
        Self {}
    }

    pub fn listen(&self, host: &str) -> Result<()> {
        let (ip, port) = match Self::parse_host(host) {
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        };

        let listener = TcpListener::bind(format!("{ip}:{port}"))?;

        println!(
            "Listening at http://{}",
            listener.local_addr().unwrap().to_string()
        );

        for stream in listener.incoming() {
            println!("{:#?}", stream);
        }

        Ok(())
    }

    fn parse_host(host: &str) -> Result<(String, u16)> {
        let parts: Vec<&str> = host.split(':').collect();

        if parts.len() > 2 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid host format"));
        }

        let ip = if parts[0].is_empty() {
            DEFAULT_HOSTNAME.to_string()
        } else {
            parts[0].to_string()
        };

        let port: u16 = match parts[parts.len() - 1].parse() {
            Ok(o) => o,
            Err(_) => {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid port number"));
            }
        };

        Ok((ip, port))
    }
}
