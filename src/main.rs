use ploot::server;

fn main() {
    let server = server::Listener::new();

    server.listen(":8000").unwrap()
}
