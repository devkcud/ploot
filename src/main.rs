use ploot::{request::RequestConstructor, response::ResponseConstructor, server};

fn home(_: RequestConstructor) -> ResponseConstructor {
    let mut response = ResponseConstructor::new();
    response.set_content(&format!("Hi!",));

    response
}

fn get_user(request: RequestConstructor) -> ResponseConstructor {
    let params = request.url_params;

    let mut response = ResponseConstructor::new();
    response.set_content(&format!(
        "Hello, {}!",
        params.get("name").unwrap_or(&String::from("Unknown"))
    ));
    response
}

fn test(request: RequestConstructor) -> ResponseConstructor {
    let mut response = ResponseConstructor::new();
    response.set_content(&format!(
        "test: {}",
        request.url_params.get("type").unwrap()
    ));
    response
}

fn main() {
    let mut server = server::Listener::new();

    server.GET("/user", home);
    server.GET("/user/{name}", get_user);
    server.GET("/user/{name}/test/{type}", test);

    match server.listen(":1234") {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
