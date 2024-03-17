use crate::response::ResponseConstructor;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Error, ErrorKind, Result, Write},
    net::{TcpListener, TcpStream},
};

const DEFAULT_HOSTNAME: &str = "127.0.0.1";

type MethodHandle = fn() -> ResponseConstructor;

pub struct Listener {
    routes: HashMap<&'static str, Route>,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn listen(&self, host: &str) -> Result<()> {
        let (ip, port) = match Self::parse_host(host) {
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        };

        let listener = TcpListener::bind(format!("{ip}:{port}"))?;

        let mut count = 0;

        for route in &self.routes {
            count += route.1.methods.len();
        }

        println!("Loaded {} methods in {} routes", count, self.routes.len());
        println!(
            "Listening at http://{}",
            listener.local_addr().unwrap().to_string()
        );

        for stream in listener.incoming() {
            self.handle_request(stream.unwrap());
        }

        Ok(())
    }

    #[allow(non_snake_case)]
    pub fn GET(&mut self, path: &'static str, handle: MethodHandle) -> () {
        self.route("GET", path, handle);
    }

    #[allow(non_snake_case)]
    pub fn POST(&mut self, path: &'static str, handle: MethodHandle) -> () {
        self.route("POST", path, handle);
    }

    #[allow(non_snake_case)]
    pub fn PUT(&mut self, path: &'static str, handle: MethodHandle) -> () {
        self.route("PUT", path, handle);
    }

    #[allow(non_snake_case)]
    pub fn PATCH(&mut self, path: &'static str, handle: MethodHandle) -> () {
        self.route("PATCH", path, handle);
    }

    #[allow(non_snake_case)]
    pub fn DELETE(&mut self, path: &'static str, handle: MethodHandle) -> () {
        self.route("DELETE", path, handle);
    }

    fn route(&mut self, method: &'static str, path: &'static str, handle: MethodHandle) -> () {
        self.routes
            .entry(path)
            .or_insert(Route {
                methods: HashMap::new(),
            })
            .methods
            .insert(method, handle);
    }

    fn handle_request(&self, mut stream: TcpStream) -> () {
        let buf_reader = BufReader::new(&mut stream);
        let http_request = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();

        let status_line = http_request
            .get(0)
            .unwrap()
            .split_whitespace()
            .take(2)
            .collect::<Vec<&str>>();

        let path = *status_line.get(1).unwrap();
        let method = *status_line.get(0).unwrap();

        if self.routes.get(path).is_none() {
            stream
                .write_all(
                    &ResponseConstructor::new()
                        .set_status("404 Not Found")
                        .set_content_type("application/json")
                        .set_content(r#"{ "error": "Not Found" }"#)
                        .build(),
                )
                .unwrap();
            return;
        }

        if self.routes[path].methods.get(method).is_none() {
            stream
                .write_all(
                    &ResponseConstructor::new()
                        .set_status("405 Method Not Allowed")
                        .set_content_type("application/json")
                        .set_content(r#"{ "error": "Method Not Allowed" }"#)
                        .build(),
                )
                .unwrap();
            return;
        }

        stream
            .write_all(&self.routes[path].methods[method]().build())
            .unwrap();
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

struct Route {
    methods: HashMap<&'static str, MethodHandle>,
}
