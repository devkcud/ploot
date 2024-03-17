use ploot::{request::RequestConstructor, response::ResponseConstructor, server};

fn get_user(request: RequestConstructor) -> ResponseConstructor {
    let mut response = ResponseConstructor::new();
    response.set_content(&format!(
        "{{ \"name\": \"{}\", \"group\": \"{}\" }}",
        request.url_params.get("name").unwrap(),
        request.group,
    ));
    response.set_content_type("application/json");
    response
}

fn main() {
    let mut server = server::Listener::new();

    server.set_group("/user");
    server.GET("/{name}", get_user);
    server.clear_group();

    match server.listen(":1234") {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
