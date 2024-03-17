use ploot::{response::ResponseConstructor, server};

fn home() -> ResponseConstructor {
    let mut response = ResponseConstructor::new();
    response.set_content("Hi");
    response
}

fn main() {
    let mut server = server::Listener::new();

    server.GET("/", home);

    match server.listen(":1234") {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
