use ploot::server;

fn main() {
    let server = server::Listener::new();

    match server.listen(":1234") {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
