use crate::{
    request::{RequestConstructor, URLParams},
    response::ResponseConstructor,
};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Error, ErrorKind, Result, Write},
    net::{TcpListener, TcpStream},
};

const DEFAULT_HOSTNAME: &str = "127.0.0.1";

type MethodHandle = fn(RequestConstructor) -> ResponseConstructor;

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

        if let Some((route, params)) = Self::parse_url_params(path, &self.routes) {
            if let Some(handler) = route.methods.get(method) {
                let mut request = RequestConstructor::new();

                for (key, value) in params {
                    request.add_url_param(&key, &value);
                }

                let response = handler(request);

                return stream.write_all(&response.build()).unwrap();
            }

            return stream
                .write_all(
                    &ResponseConstructor::new()
                        .set_status("405 Method Not Allowed")
                        .set_content_type("application/json")
                        .set_content(r#"{ "error": "Method Not Allowed" }"#)
                        .build(),
                )
                .unwrap();
        }

        stream
            .write_all(
                &ResponseConstructor::new()
                    .set_status("404 Not Found")
                    .set_content_type("application/json")
                    .set_content(r#"{ "error": "Not Found" }"#)
                    .build(),
            )
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

    fn parse_url_params(
        path: &str,
        routes: &HashMap<&'static str, Route>,
    ) -> Option<(Route, URLParams)> {
        let path_segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        for (route_path, route) in routes {
            let route_segments: Vec<&str> = route_path.trim_matches('/').split('/').collect();

            if path_segments.len() != route_segments.len() {
                continue;
            }

            let mut params = HashMap::new();
            let mut matched = true;

            for (route_part, path_part) in route_segments.iter().zip(path_segments.iter()) {
                if route_part.starts_with('{') && route_part.ends_with('}') {
                    let key = &route_part[1..route_part.len() - 1];
                    params.insert(key.to_string(), path_part.to_string());
                } else if route_part != path_part {
                    matched = false;
                    break;
                }
            }

            if matched {
                return Some((route.clone(), params));
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Route {
    methods: HashMap<&'static str, MethodHandle>,
}
